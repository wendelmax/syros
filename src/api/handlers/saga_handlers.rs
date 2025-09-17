//! Saga handlers for the Syros API.
//!
//! This module provides HTTP handlers for saga orchestration operations,
//! including starting sagas, checking status, and managing saga execution.

use crate::api::rest::ApiState;
use crate::core::saga_orchestrator::{
    BackoffStrategy, RetryPolicy, SagaRequest, SagaResponse, SagaStep,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

/// Request structure for starting a new saga.
#[derive(Debug, Deserialize)]
pub struct StartSagaRequest {
    /// Name of the saga
    pub name: String,
    /// List of steps to execute in the saga
    pub steps: Vec<SagaStepRequest>,
    /// Optional metadata for the saga
    pub metadata: Option<serde_json::Value>,
}

/// Request structure for defining a saga step.
#[derive(Debug, Deserialize)]
pub struct SagaStepRequest {
    /// Name of the step
    pub name: String,
    /// Service that will execute this step
    pub service: String,
    /// Action to perform in this step
    pub action: String,
    /// Compensation action if this step fails
    pub compensation: String,
    /// Timeout for this step in seconds
    pub timeout_seconds: u64,
    /// Optional retry policy for this step
    pub retry_policy: Option<RetryPolicyRequest>,
}

/// Request structure for defining retry policy.
#[derive(Debug, Deserialize)]
pub struct RetryPolicyRequest {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Backoff strategy: "exponential", "linear", or "fixed"
    pub backoff_strategy: String,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
}

/// Response structure for saga status information.
#[derive(Debug, Serialize)]
pub struct SagaStatusResponse {
    /// Unique identifier of the saga
    pub saga_id: String,
    /// Name of the saga
    pub name: String,
    /// Current status of the saga
    pub status: String,
    /// Index of the current step being executed
    pub current_step_index: Option<usize>,
    /// Timestamp when the saga was created
    pub created_at: String,
    /// Timestamp when the saga was last updated
    pub updated_at: String,
    /// Optional metadata associated with the saga
    pub metadata: Option<serde_json::Value>,
}

/// Starts a new saga with the provided steps and configuration.
///
/// This handler creates a new saga orchestration instance and begins
/// executing the steps according to the specified retry policies and timeouts.
///
/// # Arguments
///
/// * `state` - API state containing the saga orchestrator
/// * `request` - Saga configuration including steps and metadata
///
/// # Returns
///
/// Returns a JSON response with saga information or an error status.
pub async fn start_saga(
    State(state): State<ApiState>,
    Json(request): Json<StartSagaRequest>,
) -> Result<Json<SagaResponse>, StatusCode> {
    let steps: Vec<SagaStep> = request
        .steps
        .into_iter()
        .map(|step| SagaStep {
            name: step.name,
            service: step.service,
            action: step.action,
            compensation: step.compensation,
            timeout: std::time::Duration::from_secs(step.timeout_seconds),
            retry_policy: step.retry_policy.map(|rp| RetryPolicy {
                max_retries: rp.max_retries,
                backoff_strategy: match rp.backoff_strategy.as_str() {
                    "exponential" => BackoffStrategy::Exponential,
                    "linear" => BackoffStrategy::Linear,
                    _ => BackoffStrategy::Fixed,
                },
                initial_delay: std::time::Duration::from_millis(rp.initial_delay_ms),
            }),
        })
        .collect();

    let metadata = request
        .metadata
        .and_then(|m| serde_json::from_value::<std::collections::HashMap<String, String>>(m).ok());

    let saga_request = SagaRequest {
        name: request.name,
        steps,
        metadata,
    };

    state.metrics.increment_sagas_started();

    match state.saga_orchestrator.start_saga(saga_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Error starting saga: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves the current status of a saga by its ID.
///
/// This handler returns detailed information about a saga's current state,
/// including its status, current step, and metadata.
///
/// # Arguments
///
/// * `state` - API state containing the saga orchestrator
/// * `saga_id` - Unique identifier of the saga to check
///
/// # Returns
///
/// Returns a JSON response with saga status information or an error status.
pub async fn get_saga_status(
    State(state): State<ApiState>,
    Path(saga_id): Path<String>,
) -> Result<Json<SagaStatusResponse>, StatusCode> {
    match state.saga_orchestrator.get_saga_status(&saga_id).await {
        Ok(Some(saga)) => {
            let status = match saga.status {
                crate::core::saga_orchestrator::SagaStatus::Pending => "pending",
                crate::core::saga_orchestrator::SagaStatus::Running => "running",
                crate::core::saga_orchestrator::SagaStatus::Completed => "completed",
                crate::core::saga_orchestrator::SagaStatus::Failed => "failed",
                crate::core::saga_orchestrator::SagaStatus::Compensating => "compensating",
                crate::core::saga_orchestrator::SagaStatus::Compensated => "compensated",
            }
            .to_string();

            let metadata = if saga.metadata.is_empty() {
                None
            } else {
                Some(serde_json::to_value(saga.metadata).unwrap_or(serde_json::Value::Null))
            };

            Ok(Json(SagaStatusResponse {
                saga_id: saga.id,
                name: saga.name,
                status,
                current_step_index: saga.current_step,
                created_at: saga.created_at.to_rfc3339(),
                updated_at: saga.updated_at.to_rfc3339(),
                metadata,
            }))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting saga status: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
