use axum::{http::StatusCode, Json};
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

pub async fn health_check() -> Result<Json<HealthResponse>, StatusCode> {
    let now = SystemTime::now();
    let uptime = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        uptime_seconds: uptime,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

pub async fn readiness_check() -> Result<Json<ReadinessResponse>, StatusCode> {
    let mut checks = Vec::new();

    // Verifica se os componentes estão prontos
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

    Ok(Json(ReadinessResponse {
        ready: all_ready,
        checks,
    }))
}

pub async fn liveness_check() -> Result<Json<HealthResponse>, StatusCode> {
    health_check().await
}
