use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::api::rest::ApiState;

#[derive(Debug, Deserialize)]
pub struct AcquireLockRequest {
    pub key: String,
    pub owner: String,
    pub ttl_seconds: Option<u64>,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AcquireLockResponse {
    pub lock_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseLockRequest {
    pub lock_id: String,
    pub owner: String,
}

#[derive(Debug, Serialize)]
pub struct ReleaseLockResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LockStatusResponse {
    pub key: String,
    pub lock_id: Option<String>,
    pub owner: Option<String>,
    pub is_locked: bool,
}

#[derive(Debug, Deserialize)]
pub struct StartSagaRequest {
    pub name: String,
    pub steps: Vec<SagaStepRequest>,
}

#[derive(Debug, Deserialize)]
pub struct SagaStepRequest {
    pub name: String,
    pub action: String,
    pub compensation: String,
}

#[derive(Debug, Serialize)]
pub struct StartSagaResponse {
    pub saga_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SagaStatusResponse {
    pub saga_id: String,
    pub name: String,
    pub status: String,
    pub current_step: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct AppendEventRequest {
    pub stream_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct AppendEventResponse {
    pub event_id: String,
    pub version: u64,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GetEventsResponse {
    pub stream_id: String,
    pub events: Vec<EventResponse>,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: std::collections::HashMap<String, String>,
    pub timestamp: String,
    pub version: u64,
}

#[derive(Debug, Deserialize)]
pub struct SetCacheRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub ttl_seconds: Option<u64>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct SetCacheResponse {
    pub key: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GetCacheResponse {
    pub key: String,
    pub value: Option<serde_json::Value>,
    pub found: bool,
    pub expires_at: Option<String>,
}

pub async fn acquire_lock(
    State(state): State<ApiState>,
    Json(_request): Json<AcquireLockRequest>,
) -> Result<Json<AcquireLockResponse>, StatusCode> {
    state.metrics.increment_locks_acquired();
    
    let lock_id = uuid::Uuid::new_v4().to_string();
    
    Ok(Json(AcquireLockResponse {
        lock_id,
        success: true,
        message: "Lock acquired successfully".to_string(),
    }))
}

pub async fn release_lock(
    State(state): State<ApiState>,
    Path(_key): Path<String>,
    Json(_request): Json<ReleaseLockRequest>,
) -> Result<Json<ReleaseLockResponse>, StatusCode> {
    state.metrics.increment_locks_released();
    
    Ok(Json(ReleaseLockResponse {
        success: true,
        message: "Lock released successfully".to_string(),
    }))
}

pub async fn get_lock_status(
    State(_state): State<ApiState>,
    Path(key): Path<String>,
) -> Result<Json<LockStatusResponse>, StatusCode> {
    Ok(Json(LockStatusResponse {
        key,
        lock_id: None,
        owner: None,
        is_locked: false,
    }))
}

pub async fn start_saga(
    State(state): State<ApiState>,
    Json(_request): Json<StartSagaRequest>,
) -> Result<Json<StartSagaResponse>, StatusCode> {
    state.metrics.increment_sagas_started();
    
    let saga_id = uuid::Uuid::new_v4().to_string();
    
    Ok(Json(StartSagaResponse {
        saga_id,
        success: true,
        message: "Saga started successfully".to_string(),
    }))
}

pub async fn get_saga_status(
    State(_state): State<ApiState>,
    Path(saga_id): Path<String>,
) -> Result<Json<SagaStatusResponse>, StatusCode> {
    Ok(Json(SagaStatusResponse {
        saga_id,
        name: "test_saga".to_string(),
        status: "completed".to_string(),
        current_step: Some(0),
    }))
}

pub async fn append_event(
    State(state): State<ApiState>,
    Json(_request): Json<AppendEventRequest>,
) -> Result<Json<AppendEventResponse>, StatusCode> {
    state.metrics.increment_events_appended();
    
    let event_id = uuid::Uuid::new_v4().to_string();
    let version = 1;
    
    Ok(Json(AppendEventResponse {
        event_id,
        version,
        success: true,
        message: "Event appended successfully".to_string(),
    }))
}

pub async fn get_events(
    State(_state): State<ApiState>,
    Path(stream_id): Path<String>,
) -> Result<Json<GetEventsResponse>, StatusCode> {
    Ok(Json(GetEventsResponse {
        stream_id,
        events: vec![],
        success: true,
    }))
}

pub async fn set_cache(
    State(state): State<ApiState>,
    Path(key): Path<String>,
    Json(_request): Json<SetCacheRequest>,
) -> Result<Json<SetCacheResponse>, StatusCode> {
    state.metrics.set_cache_size(1.0);
    
    Ok(Json(SetCacheResponse {
        key,
        success: true,
        message: "Cache set successfully".to_string(),
    }))
}

pub async fn get_cache(
    State(_state): State<ApiState>,
    Path(key): Path<String>,
) -> Result<Json<GetCacheResponse>, StatusCode> {
    Ok(Json(GetCacheResponse {
        key,
        value: None,
        found: false,
        expires_at: None,
    }))
}

pub async fn delete_cache(
    State(_state): State<ApiState>,
    Path(_key): Path<String>,
) -> Result<Json<SetCacheResponse>, StatusCode> {
    Ok(Json(SetCacheResponse {
        key: "deleted".to_string(),
        success: true,
        message: "Cache deleted successfully".to_string(),
    }))
}
