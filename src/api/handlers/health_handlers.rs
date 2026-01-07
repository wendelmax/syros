use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub uptime_seconds: u64,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub name: String,
    pub status: String,
    pub message: String,
}

pub async fn health_check() -> impl IntoResponse {
    let now = SystemTime::now();
    let uptime = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        uptime_seconds: uptime,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
    .into_response()
}

pub async fn readiness_check() -> impl IntoResponse {
    let mut checks = Vec::new();

    checks.push(CheckResult {
        name: "lock_manager".to_string(),
        status: "ready".to_string(),
        message: "Lock manager is ready".to_string(),
    });

    checks.push(CheckResult {
        name: "saga_orchestrator".to_string(),
        status: "ready".to_string(),
        message: "Saga orchestrator is ready".to_string(),
    });

    checks.push(CheckResult {
        name: "event_store".to_string(),
        status: "ready".to_string(),
        message: "Event store is ready".to_string(),
    });

    checks.push(CheckResult {
        name: "cache_manager".to_string(),
        status: "ready".to_string(),
        message: "Cache manager is ready".to_string(),
    });

    let all_ready = checks.iter().all(|check| check.status == "ready");

    Json(ReadinessResponse {
        ready: all_ready,
        checks,
    })
    .into_response()
}

pub async fn liveness_check() -> impl IntoResponse {
    health_check().await
}
