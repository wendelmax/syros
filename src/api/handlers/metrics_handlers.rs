//! Metrics handlers for the Syros API.
//!
//! This module provides HTTP handlers for metrics and monitoring operations,
//! including Prometheus metrics collection and health checks.

use crate::api::rest::ApiState;
use axum::{http::StatusCode, response::Response};

/// Handles metrics collection requests.
///
/// This handler returns Prometheus-formatted metrics data for monitoring
/// the Syros service performance and health.
///
/// # Arguments
///
/// * `state` - API state containing the metrics collector
///
/// # Returns
///
/// Returns a response with Prometheus metrics data or an error status.
pub async fn metrics_handler(
    axum::extract::State(state): axum::extract::State<ApiState>,
) -> Result<Response<String>, StatusCode> {
    match state.metrics.get_metrics() {
        Ok(metrics_data) => {
            let response = Response::builder()
                .status(200)
                .header("Content-Type", "text/plain; version=0.0.4; charset=utf-8")
                .body(metrics_data)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(response)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
