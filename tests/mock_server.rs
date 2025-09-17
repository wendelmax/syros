//! Mock server implementation for integration tests.
//!
//! This module provides mock implementations of the Syros Platform servers
//! to enable testing without requiring actual server instances.

use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use uuid::Uuid;

use syros_platform::{
    api::rest::ApiState,
    api::websocket::WebSocketService,
    auth::{AuthMiddleware, RBACManager},
    config::Config,
    core::{
        cache_manager::CacheManager, event_store::EventStore, lock_manager::LockManager,
        saga_orchestrator::SagaOrchestrator,
    },
    metrics::Metrics,
};

/// Mock server configuration
pub struct MockServerConfig {
    pub rest_port: u16,
    pub grpc_port: u16,
    pub websocket_port: u16,
}

impl Default for MockServerConfig {
    fn default() -> Self {
        Self {
            rest_port: 18080,
            grpc_port: 18081,
            websocket_port: 18082,
        }
    }
}

/// Mock server instance
pub struct MockServer {
    config: MockServerConfig,
    rest_handle: Option<JoinHandle<()>>,
    grpc_handle: Option<JoinHandle<()>>,
    websocket_handle: Option<JoinHandle<()>>,
    rest_port: Option<u16>,
    websocket_port: Option<u16>,
}

impl MockServer {
    /// Create a new mock server
    pub fn new(config: MockServerConfig) -> Self {
        Self {
            config,
            rest_handle: None,
            grpc_handle: None,
            websocket_handle: None,
            rest_port: None,
            websocket_port: None,
        }
    }

    /// Start all mock servers
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start REST API mock
        self.start_rest_mock().await?;

        // Start gRPC mock
        self.start_grpc_mock().await?;

        // Start WebSocket mock
        self.start_websocket_mock().await?;

        Ok(())
    }

    /// Stop all mock servers
    pub async fn stop(&mut self) {
        if let Some(handle) = self.rest_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.grpc_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.websocket_handle.take() {
            handle.abort();
        }
    }

    /// Start REST API mock server
    async fn start_rest_mock(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let lock_manager = LockManager::new();
        let saga_orchestrator = SagaOrchestrator::new();
        let event_store = EventStore::new();
        let cache_manager = CacheManager::new();

        let app_state = Arc::new(ApiState {
            config: Config::load().unwrap_or_else(|_| Config {
                server: syros_platform::config::ServerConfig {
                    port: 8080,
                    grpc_port: 8081,
                    websocket_port: 8082,
                    host: "127.0.0.1".to_string(),
                },
                storage: syros_platform::config::StorageConfig {
                    redis: syros_platform::config::RedisConfig {
                        url: "redis://localhost:6379".to_string(),
                        pool_size: 10,
                        timeout_seconds: 30,
                    },
                    database: syros_platform::config::DatabaseConfig {
                        url: "postgres://localhost:5432/syros".to_string(),
                        pool_size: 10,
                        timeout_seconds: 30,
                    },
                },
                security: syros_platform::config::SecurityConfig {
                    jwt_secret: "test_secret".to_string(),
                    api_key_encryption_key: "test_encryption_key".to_string(),
                    cors_origins: vec!["*".to_string()],
                },
                logging: syros_platform::config::LoggingConfig {
                    level: "info".to_string(),
                    format: "json".to_string(),
                    output: "stdout".to_string(),
                },
                service_discovery: syros_platform::config::ServiceDiscoveryConfig {
                    enabled: true,
                    consul_url: "http://localhost:8500".to_string(),
                    service_name: "syros-platform".to_string(),
                    service_id: "syros-platform-1".to_string(),
                    health_check_interval: 30,
                    tags: vec!["api".to_string(), "grpc".to_string()],
                },
            }),
            lock_manager: lock_manager.clone(),
            saga_orchestrator: saga_orchestrator.clone(),
            event_store: event_store.clone(),
            cache_manager: cache_manager.clone(),
            websocket_service: Arc::new(WebSocketService::new(
                lock_manager,
                saga_orchestrator,
                event_store,
                cache_manager,
            )),
            metrics: Arc::new(Metrics::new()?),
            auth_middleware: AuthMiddleware::new("test_secret"),
            rbac_manager: Arc::new(tokio::sync::Mutex::new(RBACManager::new())),
        });

        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/metrics", get(metrics_handler))
            .route("/api/v1/locks", post(acquire_lock_handler))
            .route("/graphql", post(graphql_handler))
            .with_state(app_state);

        // Use port 0 to get a random available port
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        self.rest_handle = Some(handle);
        self.rest_port = Some(addr.port());
        println!("Mock REST server started on {}", addr);
        Ok(())
    }

    /// Start gRPC mock server
    async fn start_grpc_mock(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // For now, just create a placeholder task
        let handle = tokio::spawn(async {
            // Mock gRPC server would go here
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        self.grpc_handle = Some(handle);
        println!("Mock gRPC server started on port {}", self.config.grpc_port);
        Ok(())
    }

    /// Start WebSocket mock server
    async fn start_websocket_mock(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        let handle = tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                // Mock WebSocket connection handling
                tokio::spawn(async move {
                    let _ws_stream = tokio_tungstenite::accept_async(stream).await;
                    // Mock WebSocket logic would go here
                });
            }
        });

        self.websocket_handle = Some(handle);
        self.websocket_port = Some(addr.port());
        println!("Mock WebSocket server started on {}", addr);
        Ok(())
    }

    /// Get the base URL for REST API
    pub fn rest_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.rest_port.unwrap_or(8080))
    }

    /// Get the WebSocket URL
    pub fn websocket_url(&self) -> String {
        format!("ws://127.0.0.1:{}", self.websocket_port.unwrap_or(8082))
    }
}

/// Health check handler
async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0"
    }))
}

/// Metrics handler
async fn metrics_handler(State(state): State<Arc<ApiState>>) -> String {
    state
        .metrics
        .get_metrics()
        .unwrap_or_else(|_| "metrics_error".to_string())
}

/// Lock acquisition handler
async fn acquire_lock_handler(
    State(_state): State<Arc<ApiState>>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let key = payload
        .get("key")
        .and_then(|v| v.as_str())
        .unwrap_or("test_key");
    let owner = payload
        .get("owner")
        .and_then(|v| v.as_str())
        .unwrap_or("test_owner");

    Json(json!({
        "success": true,
        "lock_id": Uuid::new_v4().to_string(),
        "key": key,
        "owner": owner,
        "acquired_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// GraphQL handler
async fn graphql_handler(
    State(_state): State<Arc<ApiState>>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let query = payload.get("query").and_then(|v| v.as_str()).unwrap_or("");

    if query.contains("health") {
        Json(json!({
            "data": {
                "health": "healthy"
            }
        }))
    } else {
        Json(json!({
            "data": null,
            "errors": [{"message": "Mock GraphQL response"}]
        }))
    }
}

/// Test helper to run tests with mock server
pub async fn with_mock_server<F, Fut>(
    test_fn: F,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: FnOnce(MockServer) -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>,
{
    let config = MockServerConfig::default();
    let mut server = MockServer::new(config);

    // Start the mock server
    server.start().await?;

    // Wait a bit for servers to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Run the test
    let result = test_fn(server).await;

    // Cleanup is handled by Drop trait
    Ok(result?)
}

impl Drop for MockServer {
    fn drop(&mut self) {
        if let Some(handle) = self.rest_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.grpc_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.websocket_handle.take() {
            handle.abort();
        }
    }
}
