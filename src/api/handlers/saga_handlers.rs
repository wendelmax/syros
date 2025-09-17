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

#[derive(Debug, Deserialize)]
pub struct StartSagaRequest {
    pub name: String,
    pub steps: Vec<SagaStepRequest>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SagaStepRequest {
    pub name: String,
    pub service: String,
    pub action: String,
    pub compensation: String,
    pub timeout_seconds: u64,
    pub retry_policy: Option<RetryPolicyRequest>,
}

#[derive(Debug, Deserialize)]
pub struct RetryPolicyRequest {
    pub max_retries: u32,
    pub backoff_strategy: String,
    pub initial_delay_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct SagaStatusResponse {
    pub saga_id: String,
    pub name: String,
    pub status: String,
    pub current_step_index: Option<usize>,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: Option<serde_json::Value>,
}

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

    // Registrar métricas
    state.metrics.increment_sagas_started();

    match state.saga_orchestrator.start_saga(saga_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Erro ao iniciar saga: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

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
            eprintln!("Erro ao obter status da saga: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
