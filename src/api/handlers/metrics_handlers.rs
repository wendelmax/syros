use crate::api::rest::ApiState;
use axum::{http::StatusCode, response::Response};

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
