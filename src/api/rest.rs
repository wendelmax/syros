//! REST API module for the Syros.
//!
//! This module defines the REST API routes, handlers, and state management
//! for the Syros. It provides endpoints for distributed locks,
//! saga orchestration, event sourcing, caching, authentication, and RBAC.

use crate::api::graphql::{graphql_handler, graphql_playground};
use crate::api::handlers::{
    auth_handlers, cache_handlers, event_handlers, health_handlers, lock_handlers, metrics_handlers,
    rbac_handlers, saga_handlers,
};
use crate::api::websocket::WebSocketService;
use crate::auth::{AuthMiddleware, RBACManager};
use crate::config::Config;
use crate::core::{CacheManager, EventStore, LockManager, SagaOrchestrator};
use crate::metrics::Metrics;
use axum::{
    extract::WebSocketUpgrade,
    response::Response,
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/// API state structure containing all shared components.
///
/// This struct holds all the core components and services that need to be
/// shared across different API handlers, including managers, middleware,
/// and configuration.
#[derive(Clone)]
pub struct ApiState {
    /// Application configuration
    pub config: Config,
    /// Distributed lock manager
    pub lock_manager: LockManager,
    /// Saga orchestration service
    pub saga_orchestrator: SagaOrchestrator,
    /// Event store for event sourcing
    pub event_store: EventStore,
    /// Cache manager for distributed caching
    pub cache_manager: CacheManager,
    /// WebSocket service for real-time communication
    pub websocket_service: Arc<WebSocketService>,
    /// Metrics collection service
    pub metrics: Arc<Metrics>,
    /// Authentication middleware
    pub auth_middleware: AuthMiddleware,
    /// Role-based access control manager
    pub rbac_manager: Arc<tokio::sync::Mutex<RBACManager>>,
}

impl axum::extract::FromRef<ApiState> for Config {
    fn from_ref(state: &ApiState) -> Self {
        state.config.clone()
    }
}

impl axum::extract::FromRef<ApiState> for LockManager {
    fn from_ref(state: &ApiState) -> Self {
        state.lock_manager.clone()
    }
}

impl axum::extract::FromRef<ApiState> for SagaOrchestrator {
    fn from_ref(state: &ApiState) -> Self {
        state.saga_orchestrator.clone()
    }
}

impl axum::extract::FromRef<ApiState> for EventStore {
    fn from_ref(state: &ApiState) -> Self {
        state.event_store.clone()
    }
}

impl axum::extract::FromRef<ApiState> for CacheManager {
    fn from_ref(state: &ApiState) -> Self {
        state.cache_manager.clone()
    }
}

impl axum::extract::FromRef<ApiState> for Arc<Metrics> {
    fn from_ref(state: &ApiState) -> Self {
        state.metrics.clone()
    }
}

impl axum::extract::FromRef<ApiState> for AuthMiddleware {
    fn from_ref(state: &ApiState) -> Self {
        state.auth_middleware.clone()
    }
}

/// WebSocket connection handler.
///
/// This function handles WebSocket upgrade requests and delegates
/// the connection to the WebSocket service.
///
/// # Arguments
///
/// * `ws` - WebSocket upgrade request
/// * `state` - API state containing the WebSocket service
///
/// # Returns
///
/// Returns a WebSocket response for the connection.
async fn websocket_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<ApiState>,
) -> Response {
    WebSocketService::handle_websocket(ws, axum::extract::State(state.websocket_service)).await
}

/// Creates the main REST API router with all endpoints.
///
/// This function sets up all the REST API routes for the Syros,
/// including health checks, core functionality (locks, sagas, events, cache),
/// authentication, RBAC, GraphQL, and WebSocket endpoints.
///
/// # Arguments
///
/// * `state` - API state containing all shared components
///
/// # Returns
///
/// Returns an Axum router configured with all API endpoints and middleware.
pub fn create_rest_router(state: ApiState) -> Router {
    let cors_layer = CorsLayer::permissive();

    Router::new()
        .route("/health", get(health_handlers::health_check))
        .route("/ready", get(health_handlers::readiness_check))
        .route("/live", get(health_handlers::liveness_check))
        .route("/metrics", get(metrics_handlers::metrics_handler))
        .route("/api/v1/locks", post(lock_handlers::acquire_lock))
        .route("/api/v1/locks/:key", delete(lock_handlers::release_lock))
        .route(
            "/api/v1/locks/:key/status",
            get(lock_handlers::get_lock_status),
        )
        .route("/api/v1/sagas", post(saga_handlers::start_saga))
        .route(
            "/api/v1/sagas/:saga_id/status",
            get(saga_handlers::get_saga_status),
        )
        .route("/api/v1/events", post(event_handlers::append_event))
        .route("/api/v1/events/:stream_id", get(event_handlers::get_events))
        .route("/api/v1/cache/:key", post(cache_handlers::set_cache))
        .route("/api/v1/cache/:key", get(cache_handlers::get_cache))
        .route("/api/v1/cache/:key", delete(cache_handlers::delete_cache))
        .route("/api/v1/auth/login", post(auth_handlers::login))
        .route("/api/v1/auth/token", post(auth_handlers::create_token))
        .route("/api/v1/auth/api-keys", post(auth_handlers::create_api_key))
        .route("/api/v1/auth/api-keys", get(auth_handlers::list_api_keys))
        .route(
            "/api/v1/auth/api-keys/:key_id/revoke",
            delete(auth_handlers::revoke_api_key),
        )
        .route("/api/v1/auth/stats", get(auth_handlers::get_api_key_stats))
        .route("/api/v1/rbac/users", post(rbac_handlers::create_user))
        .route("/api/v1/rbac/users", get(rbac_handlers::get_all_users))
        .route("/api/v1/rbac/users/:user_id", get(rbac_handlers::get_user))
        .route(
            "/api/v1/rbac/users/username/:username",
            get(rbac_handlers::get_user_by_username),
        )
        .route(
            "/api/v1/rbac/users/:user_id/roles",
            post(rbac_handlers::update_user_roles),
        )
        .route(
            "/api/v1/rbac/users/:user_id/permissions",
            post(rbac_handlers::add_user_permission),
        )
        .route(
            "/api/v1/rbac/users/:user_id/permissions",
            delete(rbac_handlers::remove_user_permission),
        )
        .route(
            "/api/v1/rbac/users/:user_id/activate",
            post(rbac_handlers::activate_user),
        )
        .route(
            "/api/v1/rbac/users/:user_id/deactivate",
            post(rbac_handlers::deactivate_user),
        )
        .route("/api/v1/rbac/roles", get(rbac_handlers::get_all_roles))
        .route(
            "/api/v1/rbac/roles/custom",
            post(rbac_handlers::create_custom_role),
        )
        .route(
            "/api/v1/rbac/permissions/check/:user_id",
            post(rbac_handlers::check_permission),
        )
        .route(
            "/api/v1/rbac/permissions/check/:user_id/:resource_id",
            post(rbac_handlers::check_resource_permission),
        )
        .route("/graphql", post(graphql_handler))
        .route("/graphql-playground", get(graphql_playground))
        .route("/ws", get(websocket_handler))
        .layer(cors_layer)
        .with_state(state)
}
