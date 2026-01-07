//! Metrics handlers for the Syros API.
//!
//! This module provides HTTP handlers for metrics and monitoring operations,
//! including Prometheus metrics collection and health checks.

use crate::api::rest::ApiState;
use axum::{http::StatusCode, response::{IntoResponse, Response}};

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
) -> impl IntoResponse {
    match state.metrics.get_metrics() {
        Ok(metrics_data) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain; version=0.0.4")
                .body(metrics_data)
                .unwrap();
            response.into_response()
        }
        Err(e) => {
            eprintln!("Error collecting metrics: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
