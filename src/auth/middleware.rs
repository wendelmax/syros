use crate::api::rest::ApiState;
use crate::auth::{ApiKeyManager, JwtAuth};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct AuthMiddleware {
    pub jwt_auth: JwtAuth,
    pub api_key_manager: ApiKeyManager,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_auth: JwtAuth::new(jwt_secret),
            api_key_manager: ApiKeyManager::new(),
        }
    }

    pub async fn authenticate_request(
        State(state): State<ApiState>,
        headers: HeaderMap,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Skip authentication for health checks and metrics
        let path = request.uri().path();
        if path.starts_with("/health")
            || path.starts_with("/ready")
            || path.starts_with("/live")
            || path.starts_with("/metrics")
        {
            return Ok(next.run(request).await);
        }

        // Check for API key first
        if let Some(api_key) = headers.get("x-api-key") {
            if let Ok(api_key_str) = api_key.to_str() {
                if let Ok(Some(_)) = state
                    .auth_middleware
                    .api_key_manager
                    .validate_api_key(api_key_str)
                    .await
                {
                    return Ok(next.run(request).await);
                }
            }
        }

        // Check for JWT token
        if let Some(auth_header) = headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = JwtAuth::extract_token_from_header(auth_str) {
                    if state
                        .auth_middleware
                        .jwt_auth
                        .validate_token(&token)
                        .is_ok()
                    {
                        return Ok(next.run(request).await);
                    }
                }
            }
        }

        // No valid authentication found
        Err(StatusCode::UNAUTHORIZED)
    }

    pub async fn require_role(
        State(state): State<ApiState>,
        headers: HeaderMap,
        request: Request,
        next: Next,
        required_role: &str,
    ) -> Result<Response, StatusCode> {
        // Check for JWT token and role
        if let Some(auth_header) = headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = JwtAuth::extract_token_from_header(auth_str) {
                    if let Ok(claims) = state.auth_middleware.jwt_auth.validate_token(&token) {
                        if claims.role == required_role || claims.role == "admin" {
                            return Ok(next.run(request).await);
                        }
                    }
                }
            }
        }

        Err(StatusCode::FORBIDDEN)
    }
}

// Helper function to create auth middleware
pub fn create_auth_middleware(jwt_secret: &str) -> AuthMiddleware {
    AuthMiddleware::new(jwt_secret)
}
