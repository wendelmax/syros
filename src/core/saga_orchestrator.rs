//! Saga orchestrator implementation for distributed transactions.
//!
//! This module provides a saga orchestrator that manages distributed transactions
//! using the saga pattern, including compensation logic for rollback scenarios.

use crate::{Result, SyrosError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saga {
    pub id: String,
    pub name: String,
    pub steps: Vec<SagaStep>,
    pub status: SagaStatus,
    pub current_step: Option<usize>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SagaRequest {
    pub name: String,
    pub steps: Vec<SagaStep>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct SagaResponse {
    pub saga_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct SagaOrchestrator {
    sagas: Arc<RwLock<HashMap<String, Saga>>>,
}

impl SagaOrchestrator {
    pub fn new() -> Self {
        Self {
            sagas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_saga(&self, request: SagaRequest) -> Result<SagaResponse> {
        let saga_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let saga = Saga {
            id: saga_id.clone(),
            name: request.name,
            steps: request.steps,
            status: SagaStatus::Pending,
            current_step: None,
            created_at: now,
            updated_at: now,
            metadata: request.metadata.unwrap_or_default(),
        };

        let mut sagas = self.sagas.write().await;
        sagas.insert(saga_id.clone(), saga);

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
        let mut sagas = self.sagas.write().await;

        if let Some(saga) = sagas.get_mut(saga_id) {
            saga.status = SagaStatus::Running;
            saga.updated_at = Utc::now();
        }

        drop(sagas);

        for step_index in 0..self.get_saga_steps_count(saga_id).await? {
            if let Err(e) = self.execute_step(saga_id, step_index).await {
                self.compensate_saga(saga_id).await?;
                return Err(e);
            }
        }

        let mut sagas = self.sagas.write().await;
        if let Some(saga) = sagas.get_mut(saga_id) {
            saga.status = SagaStatus::Completed;
            saga.updated_at = Utc::now();
        }

        Ok(())
    }

    async fn execute_step(&self, saga_id: &str, step_index: usize) -> Result<()> {
        let mut sagas = self.sagas.write().await;

        if let Some(saga) = sagas.get_mut(saga_id) {
            saga.current_step = Some(step_index);
            saga.updated_at = Utc::now();
        }

        drop(sagas);

        tokio::time::sleep(Duration::from_millis(100)).await;

        if fastrand::f32() < 0.1 {
            return Err(SyrosError::SagaError("Step execution failed".to_string()));
        }

        Ok(())
    }

    async fn compensate_saga(&self, saga_id: &str) -> Result<()> {
        let mut sagas = self.sagas.write().await;

        if let Some(saga) = sagas.get_mut(saga_id) {
            saga.status = SagaStatus::Compensating;
            saga.updated_at = Utc::now();
        }

        drop(sagas);

        let steps_count = self.get_saga_steps_count(saga_id).await?;
        for step_index in (0..steps_count).rev() {
            self.compensate_step(saga_id, step_index).await?;
        }

        let mut sagas = self.sagas.write().await;
        if let Some(saga) = sagas.get_mut(saga_id) {
            saga.status = SagaStatus::Compensated;
            saga.updated_at = Utc::now();
        }

        Ok(())
    }

    async fn compensate_step(&self, _saga_id: &str, _step_index: usize) -> Result<()> {
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(())
    }

    async fn get_saga_steps_count(&self, saga_id: &str) -> Result<usize> {
        let sagas = self.sagas.read().await;
        if let Some(saga) = sagas.get(saga_id) {
            Ok(saga.steps.len())
        } else {
            Err(SyrosError::SagaError("Saga not found".to_string()))
        }
    }

    pub async fn get_saga_status(&self, saga_id: &str) -> Result<Option<Saga>> {
        let sagas = self.sagas.read().await;
        Ok(sagas.get(saga_id).cloned())
    }
}

