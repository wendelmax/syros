use crate::api::rest::ApiState;
use crate::auth::api_keys::{ApiKeyResponse, ApiKeyStats, CreateApiKeyRequest};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub user_id: String,
    pub role: String,
    pub expiration_hours: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub role: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires_in: u64,
}

pub async fn login(
    State(state): State<ApiState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if request.username == "admin" && request.password == "admin123" {
        let expiration_hours = 24;
        let token = state
            .auth_middleware
            .jwt_auth
            .generate_token("admin".to_string(), "admin".to_string(), expiration_hours)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse {
            token,
            user_id: "admin".to_string(),
            role: "admin".to_string(),
            expires_in: expiration_hours * 3600,
        }))
    } else if request.username == "user" && request.password == "user123" {
        let expiration_hours = 8;
        let token = state
            .auth_middleware
            .jwt_auth
            .generate_token("user".to_string(), "user".to_string(), expiration_hours)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse {
            token,
            user_id: "user".to_string(),
            role: "user".to_string(),
            expires_in: expiration_hours * 3600,
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn create_token(
    State(state): State<ApiState>,
    Json(request): Json<CreateTokenRequest>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let expiration_hours = request.expiration_hours.unwrap_or(24);
    let token = state
        .auth_middleware
        .jwt_auth
        .generate_token(request.user_id, request.role, expiration_hours)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TokenResponse {
        token,
        expires_in: expiration_hours * 3600,
    }))
}

pub async fn create_api_key(
    State(state): State<ApiState>,
    Json(request): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>, StatusCode> {
    let api_key = state
        .auth_middleware
        .api_key_manager
        .create_api_key(request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(api_key))
}

pub async fn list_api_keys(
    State(state): State<ApiState>,
) -> Result<Json<Vec<ApiKeyResponse>>, StatusCode> {
    let api_keys = state
        .auth_middleware
        .api_key_manager
        .list_api_keys()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(api_keys))
}

pub async fn revoke_api_key(
    State(state): State<ApiState>,
    axum::extract::Path(key_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let success = state
        .auth_middleware
        .api_key_manager
        .revoke_api_key(&key_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if success {
        Ok(Json(serde_json::json!({
            "success": true,
            "message": "API key revoked successfully"
        })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_api_key_stats(
    State(state): State<ApiState>,
) -> Result<Json<ApiKeyStats>, StatusCode> {
    let stats = state
        .auth_middleware
        .api_key_manager
        .get_api_key_stats()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stats))
}
