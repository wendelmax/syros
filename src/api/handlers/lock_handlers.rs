use crate::api::rest::ApiState;
use crate::core::lock_manager::{
    LockRequest, LockResponse, ReleaseLockRequest, ReleaseLockResponse,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AcquireLockRequest {
    pub key: String,
    pub ttl_seconds: u64,
    pub metadata: Option<String>,
    pub owner: String,
    pub wait_timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseLockRequestPayload {
    pub lock_id: String,
    pub owner: String,
}

#[derive(Debug, Serialize)]
pub struct LockStatusResponse {
    pub key: String,
    pub lock_id: Option<String>,
    pub owner: Option<String>,
    pub acquired_at: Option<String>,
    pub expires_at: Option<String>,
    pub metadata: Option<String>,
    pub is_locked: bool,
}

pub async fn acquire_lock(
    State(state): State<ApiState>,
    Json(request): Json<AcquireLockRequest>,
) -> Result<Json<LockResponse>, StatusCode> {
    let lock_request = LockRequest {
        key: request.key,
        ttl: std::time::Duration::from_secs(request.ttl_seconds),
        metadata: request.metadata,
        owner: request.owner,
        wait_timeout: request
            .wait_timeout_seconds
            .map(std::time::Duration::from_secs),
    };

    state.metrics.increment_locks_acquired();

    match state.lock_manager.acquire_lock(lock_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Error acquiring lock: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn release_lock(
    State(state): State<ApiState>,
    Path(key): Path<String>,
    Json(request): Json<ReleaseLockRequestPayload>,
) -> Result<Json<ReleaseLockResponse>, StatusCode> {
    let release_request = ReleaseLockRequest {
        key,
        lock_id: request.lock_id,
        owner: request.owner,
    };

    state.metrics.increment_locks_released();

    match state.lock_manager.release_lock(release_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Error releasing lock: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_lock_status(
    State(state): State<ApiState>,
    Path(key): Path<String>,
) -> Result<Json<LockStatusResponse>, StatusCode> {
    match state.lock_manager.get_lock_status(&key).await {
        Ok(Some(lock_state)) => Ok(Json(LockStatusResponse {
            key: lock_state.key,
            lock_id: Some(lock_state.id),
            owner: Some(lock_state.owner),
            acquired_at: Some(lock_state.acquired_at.to_rfc3339()),
            expires_at: Some(lock_state.expires_at.to_rfc3339()),
            metadata: lock_state.metadata,
            is_locked: true,
        })),
        Ok(None) => Ok(Json(LockStatusResponse {
            key,
            lock_id: None,
            owner: None,
            acquired_at: None,
            expires_at: None,
            metadata: None,
            is_locked: false,
        })),
        Err(e) => {
            eprintln!("Error getting lock status: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
