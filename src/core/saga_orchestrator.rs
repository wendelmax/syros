//! Saga orchestrator implementation for distributed transactions.
//!
//! This module provides a saga orchestrator that manages distributed transactions
//! using the saga pattern, including compensation logic for rollback scenarios.

use crate::storage::postgres::PostgresManager;
use crate::{Result, SyrosError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Represents a single step in a saga transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaStep {
    /// Name of the step
    pub name: String,
    /// Service that will execute this step
    pub service: String,
    /// Action to perform
    pub action: String,
    /// Compensation action for rollback
    pub compensation: String,
    /// Timeout for this step
    pub timeout: Duration,
    /// Retry policy for this step
    pub retry_policy: Option<RetryPolicy>,
}

/// Retry policy configuration for saga steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Backoff strategy for retries
    pub backoff_strategy: BackoffStrategy,
    /// Initial delay before first retry
    pub initial_delay: Duration,
}

/// Backoff strategies for retry policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    /// Linear backoff - constant delay between retries
    Linear,
    /// Exponential backoff - exponentially increasing delay
    Exponential,
    /// Fixed backoff - same delay for all retries
    Fixed,
}

/// Status of a saga transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SagaStatus {
    /// Saga is pending execution
    Pending,
    /// Saga is currently running
    Running,
    /// Saga completed successfully
    Completed,
    /// Saga failed and needs compensation
    Failed,
    /// Saga is currently compensating (rolling back)
    Compensating,
    /// Saga compensation completed
    Compensated,
}

use std::fmt;

impl fmt::Display for SagaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SagaStatus::Pending => "Pending",
            SagaStatus::Running => "Running",
            SagaStatus::Completed => "Completed",
            SagaStatus::Failed => "Failed",
            SagaStatus::Compensating => "Compensating",
            SagaStatus::Compensated => "Compensated",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for SagaStatus {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(SagaStatus::Pending),
            "Running" => Ok(SagaStatus::Running),
            "Completed" => Ok(SagaStatus::Completed),
            "Failed" => Ok(SagaStatus::Failed),
            "Compensating" => Ok(SagaStatus::Compensating),
            "Compensated" => Ok(SagaStatus::Compensated),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Saga {
    pub id: String,
    pub name: String,
    pub status: String,
    #[sqlx(json)]
    pub steps: serde_json::Value,
    pub current_step: Option<i32>, // SQL uses signed integers
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(json)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaRequest {
    pub name: String,
    pub steps: Vec<SagaStep>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaResponse {
    pub saga_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct SagaOrchestrator {
    pg: PostgresManager,
}

impl SagaOrchestrator {
    pub fn new(pg: PostgresManager) -> Self {
        Self { pg }
    }

    pub async fn start_saga(&self, request: SagaRequest) -> Result<SagaResponse> {
        let saga_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let pool = self.pg.get_pool();

        let metadata = request.metadata.unwrap_or_default();

        sqlx::query(
            "INSERT INTO sagas (id, name, status, steps, created_at, updated_at, metadata) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(Uuid::parse_str(&saga_id).unwrap_or_default())
        .bind(&request.name)
        .bind("Pending")
        .bind(sqlx::types::Json(
            serde_json::to_value(&request.steps).unwrap_or_default(),
        ))
        .bind(now)
        .bind(now)
        .bind(sqlx::types::Json(
            serde_json::to_value(&metadata).unwrap_or_default(),
        ))
        .execute(pool)
        .await
        .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        let orchestrator_clone = Arc::new(self.clone());
        let saga_id_clone = saga_id.clone();
        tokio::spawn(async move {
            if let Err(e) = orchestrator_clone.execute_saga(&saga_id_clone).await {
                eprintln!("Error executing saga {}: {}", saga_id_clone, e);
            }
        });

        Ok(SagaResponse {
            saga_id,
            success: true,
            message: "Saga started successfully".to_string(),
        })
    }

    pub async fn execute_saga(&self, saga_id: &str) -> Result<()> {
        let pool = self.pg.get_pool();

        sqlx::query("UPDATE sagas SET status = 'Running', updated_at = NOW() WHERE id = $1")
            .bind(Uuid::parse_str(saga_id).unwrap_or_default())
            .execute(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        let steps_count = self.get_saga_steps_count(saga_id).await?;

        for step_index in 0..steps_count {
            if let Err(e) = self.execute_step(saga_id, step_index).await {
                // If it fails, start compensation
                self.compensate_saga(saga_id).await?;
                return Err(e);
            }
        }

        sqlx::query("UPDATE sagas SET status = 'Completed', updated_at = NOW() WHERE id = $1")
            .bind(Uuid::parse_str(saga_id).unwrap_or_default())
            .execute(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn execute_step(&self, saga_id: &str, step_index: usize) -> Result<()> {
        let pool = self.pg.get_pool();

        sqlx::query("UPDATE sagas SET current_step = $1, updated_at = NOW() WHERE id = $2")
            .bind(step_index as i32)
            .bind(Uuid::parse_str(saga_id).unwrap_or_default())
            .execute(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        tokio::time::sleep(Duration::from_millis(100)).await;

        // Artificial failure probability
        if fastrand::f32() < 0.1 {
            // We could log failure here
            // For improvement, let's remove it or make it very rare to keep system stable
            // return Err(crate::SyrosError::SagaError("Step execution failed (simulated)".to_string()));
        }

        Ok(())
    }

    async fn compensate_saga(&self, saga_id: &str) -> Result<()> {
        let pool = self.pg.get_pool();

        sqlx::query("UPDATE sagas SET status = 'Compensating', updated_at = NOW() WHERE id = $1")
            .bind(Uuid::parse_str(saga_id).unwrap_or_default())
            .execute(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        let steps_count = self.get_saga_steps_count(saga_id).await?;
        for step_index in (0..steps_count).rev() {
            self.compensate_step(saga_id, step_index).await?;
        }

        sqlx::query("UPDATE sagas SET status = 'Compensated', updated_at = NOW() WHERE id = $1")
            .bind(Uuid::parse_str(saga_id).unwrap_or_default())
            .execute(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn compensate_step(&self, _saga_id: &str, _step_index: usize) -> Result<()> {
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    async fn get_saga_steps_count(&self, saga_id: &str) -> Result<usize> {
        let pool = self.pg.get_pool();

        // Retrieve steps JSONB to count it
        let steps: sqlx::types::Json<serde_json::Value> =
            sqlx::query_scalar("SELECT steps FROM sagas WHERE id = $1")
                .bind(Uuid::parse_str(saga_id).unwrap_or_default())
                .fetch_one(pool)
                .await
                .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(steps.0.as_array().map(|a| a.len()).unwrap_or(0))
    }

    pub async fn get_saga_status(&self, saga_id: &str) -> Result<Option<Saga>> {
        let pool = self.pg.get_pool();

        let saga: Option<Saga> = sqlx::query_as("SELECT id, name, status, steps, current_step, created_at, updated_at, metadata FROM sagas WHERE id = $1")
            .bind(Uuid::parse_str(saga_id).unwrap_or_default())
            .fetch_optional(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(saga)
    }
}
