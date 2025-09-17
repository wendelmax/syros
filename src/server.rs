//! Server module for the Syros.
//!
//! This module contains the main server logic for starting and managing
//! the various server components (REST, gRPC, WebSocket) and their
//! associated services.

use crate::api::grpc::SyrosGrpcService;
use crate::api::rest::{create_rest_router, ApiState};
use crate::api::websocket::WebSocketService;
use crate::auth::AuthMiddleware;
use crate::cli::ServerType;
use crate::config::Config;
use crate::core::{
    CacheManager, EventStore, LockManager, SagaOrchestrator, ServiceCheck, ServiceDiscovery,
    ServiceRegistration,
};
use crate::metrics::Metrics;
use axum;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

/// Starts the Syros server with the specified configuration.
///
/// This function initializes all core components, sets up service discovery,
/// and starts the requested server types (REST, gRPC, WebSocket) concurrently.
///
/// # Arguments
///
/// * `verbose` - Enable verbose logging
/// * `quiet` - Suppress non-essential output
/// * `host` - Host address to bind to
/// * `port` - REST API port
/// * `grpc_port` - gRPC server port
/// * `websocket_port` - WebSocket server port
/// * `servers` - List of server types to start
/// * `interface` - Optional network interface to bind to
///
/// # Returns
///
/// Returns `Ok(())` on successful startup, or an error if something goes wrong.
pub async fn start_server(
    verbose: bool,
    quiet: bool,
    host: String,
    port: u16,
    grpc_port: u16,
    websocket_port: u16,
    servers: Vec<ServerType>,
    interface: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load().unwrap_or_else(|_| Config {
        server: crate::config::ServerConfig {
            port,
            grpc_port,
            websocket_port,
            host: host.clone(),
        },
        storage: crate::config::StorageConfig {
            redis: crate::config::RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
                timeout_seconds: 30,
            },
            database: crate::config::DatabaseConfig {
                url: "postgresql://localhost/syros".to_string(),
                pool_size: 10,
                timeout_seconds: 30,
            },
        },
        security: crate::config::SecurityConfig {
            jwt_secret: "your-secret-key".to_string(),
            api_key_encryption_key: "your-api-key".to_string(),
            cors_origins: vec!["*".to_string()],
        },
        logging: crate::config::LoggingConfig {
            level: "info".to_string(),
            format: "json".to_string(),
            output: "stdout".to_string(),
        },
        service_discovery: crate::config::ServiceDiscoveryConfig {
            enabled: false,
            consul_url: "http://localhost:8500".to_string(),
            service_name: "syros-platform".to_string(),
            service_id: "syros-platform-1".to_string(),
            health_check_interval: 10,
            tags: vec!["syros".to_string(), "platform".to_string()],
        },
    });

    let should_start_rest =
        servers.contains(&ServerType::Rest) || servers.contains(&ServerType::All);
    let should_start_grpc =
        servers.contains(&ServerType::Grpc) || servers.contains(&ServerType::All);
    let should_start_websocket =
        servers.contains(&ServerType::Websocket) || servers.contains(&ServerType::All);

    if verbose {
        println!("Starting Syros...");
        println!("Configuration loaded:");
        println!("   - Host: {}", config.server.host);
        if should_start_rest {
            println!("   - REST: {}:{}", config.server.host, config.server.port);
        }
        if should_start_grpc {
            println!(
                "   - gRPC: {}:{}",
                config.server.host, config.server.grpc_port
            );
        }
        if should_start_websocket {
            println!(
                "   - WebSocket: {}:{}",
                config.server.host, config.server.websocket_port
            );
        }
        if let Some(iface) = &interface {
            println!("   - Interface: {}", iface);
        }
    }

    let lock_manager = LockManager::new();
    let saga_orchestrator = SagaOrchestrator::new();
    let event_store = EventStore::new();
    let cache_manager = CacheManager::new();

    if verbose {
        println!("Core components initialized");
    }

    let mut service_discovery = if config.service_discovery.enabled {
        match ServiceDiscovery::new(&config.service_discovery.consul_url) {
            Ok(sd) => {
                if verbose {
                    println!(
                        "Service Discovery initialized with Consul at {}",
                        config.service_discovery.consul_url
                    );
                }
                Some(sd)
            }
            Err(e) => {
                eprintln!("Error initializing Service Discovery: {}", e);
                if verbose {
                    println!("Continuing without Service Discovery...");
                }
                None
            }
        }
    } else {
        if verbose {
            println!("Service Discovery disabled");
        }
        None
    };

    let metrics = Arc::new(
        Metrics::new()
            .map_err(|e| {
                eprintln!("Error initializing metrics: {}", e);
                std::process::exit(1);
            })
            .unwrap(),
    );

    let websocket_service = Arc::new(WebSocketService::new(
        lock_manager.clone(),
        saga_orchestrator.clone(),
        event_store.clone(),
        cache_manager.clone(),
    ));

    let auth_middleware = AuthMiddleware::new(&config.security.jwt_secret);
    let rbac_manager = Arc::new(tokio::sync::Mutex::new(crate::auth::RBACManager::new()));

    let api_state = ApiState {
        config: config.clone(),
        lock_manager,
        saga_orchestrator,
        event_store,
        cache_manager,
        websocket_service: websocket_service.clone(),
        metrics: metrics.clone(),
        auth_middleware,
        rbac_manager,
    };

    let app = create_rest_router(api_state.clone());

    let grpc_service = SyrosGrpcService::new(
        api_state.lock_manager.clone(),
        api_state.saga_orchestrator.clone(),
        api_state.event_store.clone(),
        api_state.cache_manager.clone(),
    );

    if let Some(ref mut sd) = service_discovery {
        let service_registration = ServiceRegistration {
            id: config.service_discovery.service_id.clone(),
            name: config.service_discovery.service_name.clone(),
            address: config.server.host.clone(),
            port: config.server.port,
            tags: config.service_discovery.tags.clone(),
            meta: std::collections::HashMap::new(),
            check: Some(ServiceCheck {
                http: Some(format!(
                    "http://{}:{}/health",
                    config.server.host, config.server.port
                )),
                tcp: None,
                interval: format!("{}s", config.service_discovery.health_check_interval),
                timeout: "5s".to_string(),
            }),
        };

        if let Err(e) = sd.register_service(service_registration).await {
            eprintln!("Error registering service in Service Discovery: {}", e);
        } else if verbose {
            println!(
                "Service registered in Service Discovery: {} ({})",
                config.service_discovery.service_name, config.service_discovery.service_id
            );
        }
    }

    let mut tasks = Vec::new();

    if should_start_rest {
        let rest_addr: SocketAddr =
            format!("{}:{}", config.server.host, config.server.port).parse()?;
        let rest_listener = TcpListener::bind(&rest_addr).await?;

        if !quiet {
            println!("REST server started at http://{}", rest_addr);
        }

        if verbose {
            println!("REST API documentation available at:");
            println!("   - Health: http://{}/health", rest_addr);
            println!("   - Ready: http://{}/ready", rest_addr);
            println!("   - Metrics: http://{}/metrics", rest_addr);
            println!("   - REST API: http://{}/api/v1/", rest_addr);
        }

        let rest_task = tokio::spawn(async move {
            let rest_server = axum::serve(rest_listener, app);
            if let Err(e) = rest_server.await {
                eprintln!("REST server error: {}", e);
            }
        });
        tasks.push(rest_task);
    }

    if should_start_grpc {
        let grpc_addr: SocketAddr =
            format!("{}:{}", config.server.host, config.server.grpc_port).parse()?;

        if !quiet {
            println!("gRPC server started at http://{}", grpc_addr);
        }

        if verbose {
            println!("gRPC API documentation available at:");
            println!("   - gRPC API: http://{}", grpc_addr);
        }

        if verbose {
            if let Err(e) = grpc_service.demo_grpc_operations().await {
                eprintln!("gRPC demo error: {}", e);
            }
        }

        let grpc_task = tokio::spawn(async move {
            if let Err(e) = grpc_service.start_grpc_server(grpc_addr).await {
                eprintln!("gRPC server error: {}", e);
            }
        });
        tasks.push(grpc_task);
    }

    if should_start_websocket {
        let websocket_addr: SocketAddr =
            format!("{}:{}", config.server.host, config.server.websocket_port).parse()?;

        if !quiet {
            println!("WebSocket server started at ws://{}", websocket_addr);
        }

        if verbose {
            println!("WebSocket documentation available at:");
            println!("   - WebSocket: ws://{}/ws", websocket_addr);
        }
    }

    if tasks.is_empty() {
        eprintln!("No servers selected to start!");
        return Ok(());
    }

    match tasks.len() {
        1 => {
            if let Some(task) = tasks.into_iter().next() {
                let _ = task.await;
            }
        }
        2 => {
            let mut tasks_iter = tasks.into_iter();
            let task1 = tasks_iter.next().unwrap();
            let task2 = tasks_iter.next().unwrap();

            tokio::select! {
                _ = task1 => {},
                _ = task2 => {},
            }
        }
        3 => {
            let mut tasks_iter = tasks.into_iter();
            let task1 = tasks_iter.next().unwrap();
            let task2 = tasks_iter.next().unwrap();
            let task3 = tasks_iter.next().unwrap();

            tokio::select! {
                _ = task1 => {},
                _ = task2 => {},
                _ = task3 => {},
            }
        }
        _ => {
            let mut tasks_iter = tasks.into_iter();
            let task1 = tasks_iter.next().unwrap();
            let task2 = tasks_iter.next().unwrap();
            let task3 = tasks_iter.next().unwrap();
            let remaining: Vec<_> = tasks_iter.collect();

            tokio::select! {
                _ = task1 => {},
                _ = task2 => {},
                _ = task3 => {},
                _ = async {
                    for task in remaining {
                        let _ = task.await;
                    }
                } => {},
            }
        }
    }

    Ok(())
}
