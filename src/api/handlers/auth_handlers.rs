use crate::api::rest::ApiState;
use crate::auth::api_keys::{ApiKeyResponse, ApiKeyStats, CreateApiKeyRequest};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
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
) -> impl IntoResponse {
    if request.username == "admin" && request.password == "admin123" {
        let expiration_hours = 24;
        let token = match state
            .auth_middleware
            .jwt_auth
            .generate_token("admin".to_string(), "admin".to_string(), expiration_hours)
        {
            Ok(t) => t,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        Json(LoginResponse {
            token,
            user_id: "admin".to_string(),
            role: "admin".to_string(),
            expires_in: expiration_hours * 3600,
        })
        .into_response()
    } else if request.username == "user" && request.password == "user123" {
        let expiration_hours = 8;
        let token = match state
            .auth_middleware
            .jwt_auth
            .generate_token("user".to_string(), "user".to_string(), expiration_hours)
        {
            Ok(t) => t,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        Json(LoginResponse {
            token,
            user_id: "user".to_string(),
            role: "user".to_string(),
            expires_in: expiration_hours * 3600,
        })
        .into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn create_token(
    State(state): State<ApiState>,
    Json(request): Json<CreateTokenRequest>,
) -> impl IntoResponse {
    let expiration_hours = request.expiration_hours.unwrap_or(24);
    let token = match state
        .auth_middleware
        .jwt_auth
        .generate_token(request.user_id, request.role, expiration_hours)
    {
        Ok(t) => t,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(TokenResponse {
        token,
        expires_in: expiration_hours * 3600,
    })
    .into_response()
}

pub async fn create_api_key(
    State(state): State<ApiState>,
    Json(request): Json<CreateApiKeyRequest>,
) -> impl IntoResponse {
    let api_key = match state
        .auth_middleware
        .api_key_manager
        .create_api_key(request)
        .await
    {
        Ok(key) => key,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(api_key).into_response()
}

pub async fn list_api_keys(
    State(state): State<ApiState>,
) -> impl IntoResponse {
    let api_keys = match state
        .auth_middleware
        .api_key_manager
        .list_api_keys()
        .await
    {
        Ok(keys) => keys,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(api_keys).into_response()
}

pub async fn revoke_api_key(
    State(state): State<ApiState>,
    axum::extract::Path(key_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let success = match state
        .auth_middleware
        .api_key_manager
        .revoke_api_key(&key_id)
        .await
    {
        Ok(s) => s,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if success {
        Json(serde_json::json!({
            "success": true,
            "message": "API key revoked successfully"
        }))
        .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn get_api_key_stats(
    State(state): State<ApiState>,
) -> impl IntoResponse {
    let stats = match state
        .auth_middleware
        .api_key_manager
        .get_api_key_stats()
        .await
    {
        Ok(s) => s,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(stats).into_response()
}
