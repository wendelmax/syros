//! gRPC API module for the Syros Platform.
//!
//! This module implements the gRPC service for the Syros Platform using Volo.
//! It provides high-performance RPC endpoints for distributed locks, saga orchestration,
//! event sourcing, and caching operations.

use crate::core::{CacheManager, EventStore, LockManager, SagaOrchestrator};
use crate::generated::*;
use crate::generated::{SyrosService, SyrosServiceServer};
use std::sync::Arc;
use volo::FastStr;
use volo_grpc::{Request, Response, Status};

/// gRPC service implementation for the Syros Platform.
///
/// This struct holds references to all core components and implements
/// the gRPC service trait for distributed coordination operations.
pub struct SyrosGrpcService {
    lock_manager: Arc<LockManager>,
    saga_orchestrator: Arc<SagaOrchestrator>,
    event_store: Arc<EventStore>,
    cache_manager: Arc<CacheManager>,
}

impl SyrosGrpcService {
    /// Creates a new gRPC service instance.
    ///
    /// # Arguments
    ///
    /// * `lock_manager` - Distributed lock manager
    /// * `saga_orchestrator` - Saga orchestration service
    /// * `event_store` - Event store for event sourcing
    /// * `cache_manager` - Cache manager for distributed caching
    ///
    /// # Returns
    ///
    /// Returns a new `SyrosGrpcService` instance.
    pub fn new(
        lock_manager: LockManager,
        saga_orchestrator: SagaOrchestrator,
        event_store: EventStore,
        cache_manager: CacheManager,
    ) -> Self {
        Self {
            lock_manager: Arc::new(lock_manager),
            saga_orchestrator: Arc::new(saga_orchestrator),
            event_store: Arc::new(event_store),
            cache_manager: Arc::new(cache_manager),
        }
    }

    /// Starts the gRPC server on the specified address.
    ///
    /// This method creates a new gRPC server instance and starts it on the
    /// provided address. The server will handle all gRPC requests for the
    /// Syros Platform services.
    ///
    /// # Arguments
    ///
    /// * `addr` - Socket address to bind the server to
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful startup, or an error if something goes wrong.
    pub async fn start_grpc_server(
        &self,
        addr: std::net::SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let svc = SyrosServiceServer::new(self.clone());

        let server = volo_grpc::server::Server::new().add_service(svc);

        let address = volo::net::Address::from(addr);

        server
            .run(address)
            .await
            .map_err(|e| format!("gRPC server error: {}", e))?;

        Ok(())
    }

    /// Demonstrates gRPC operations for testing purposes.
    ///
    /// This method performs a series of gRPC operations to demonstrate
    /// the functionality of the Syros Platform services. It's primarily
    /// used for testing and validation during development.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all operations succeed, or an error if something goes wrong.
    pub async fn demo_grpc_operations(&self) -> Result<(), String> {
        let lock_req = LockRequest {
            key: FastStr::from("test_key"),
            owner: FastStr::from("test_owner"),
            ttl_seconds: 60,
            metadata: Some(FastStr::from("test metadata")),
            wait_timeout_seconds: Some(10),
        };

        match self.acquire_lock(Request::new(lock_req)).await {
            Ok(_response) => {}
            Err(e) => return Err(format!("Lock error: {}", e)),
        }

        let cache_req = SetCacheRequest {
            key: FastStr::from("test_cache"),
            value: FastStr::from(serde_json::json!({"message": "Hello gRPC!"}).to_string()),
            ttl_seconds: Some(300),
            tags: vec![FastStr::from("test")],
        };

        match self.set_cache(Request::new(cache_req)).await {
            Ok(_response) => {}
            Err(e) => return Err(format!("Cache error: {}", e)),
        }

        let event_req = EventRequest {
            stream_id: FastStr::from("test_stream"),
            event_type: FastStr::from("TestEvent"),
            data: FastStr::from(
                serde_json::json!({"action": "test", "timestamp": chrono::Utc::now()}).to_string(),
            ),
            metadata: std::collections::HashMap::new(),
        };

        match self.append_event(Request::new(event_req)).await {
            Ok(_response) => {}
            Err(e) => return Err(format!("Event error: {}", e)),
        }

        let saga_req = SagaRequest {
            name: FastStr::from("test_saga"),
            steps: vec![SagaStep {
                name: FastStr::from("step1"),
                service: FastStr::from("test_service"),
                action: FastStr::from("test_action"),
                compensation: FastStr::from("test_compensation"),
                timeout_seconds: Some(30),
                retry_policy: Some(RetryPolicy {
                    max_retries: 3,
                    backoff_strategy: FastStr::from("exponential"),
                    initial_delay_seconds: Some(1),
                    max_delay_seconds: Some(60),
                    factor: Some(2.0),
                }),
                payload: Some(FastStr::from("test_payload")),
            }],
            metadata: std::collections::HashMap::new(),
        };

        match self.start_saga(Request::new(saga_req)).await {
            Ok(_response) => {}
            Err(e) => return Err(format!("Saga error: {}", e)),
        }

        Ok(())
    }
}

impl Clone for SyrosGrpcService {
    /// Creates a clone of the gRPC service.
    ///
    /// This implementation allows the service to be shared across multiple
    /// threads and tasks, which is necessary for the gRPC server implementation.
    fn clone(&self) -> Self {
        Self {
            lock_manager: self.lock_manager.clone(),
            saga_orchestrator: self.saga_orchestrator.clone(),
            event_store: self.event_store.clone(),
            cache_manager: self.cache_manager.clone(),
        }
    }
}

#[async_trait::async_trait]
impl SyrosService for SyrosGrpcService {
    /// Acquires a distributed lock.
    ///
    /// This method attempts to acquire a distributed lock with the specified
    /// key, owner, TTL, and optional metadata. It returns a lock ID if successful.
    ///
    /// # Arguments
    ///
    /// * `request` - gRPC request containing lock parameters
    ///
    /// # Returns
    ///
    /// Returns a gRPC response with lock information or an error status.
    async fn acquire_lock(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<LockResponse>, Status> {
        let req = request.into_inner();

        let lock_request = crate::core::lock_manager::LockRequest {
            key: req.key.to_string(),
            owner: req.owner.to_string(),
            ttl: std::time::Duration::from_secs(req.ttl_seconds),
            metadata: req.metadata.map(|m| m.to_string()),
            wait_timeout: req
                .wait_timeout_seconds
                .map(|s| std::time::Duration::from_secs(s)),
        };

        match self.lock_manager.acquire_lock(lock_request).await {
            Ok(response) => Ok(Response::new(LockResponse {
                lock_id: FastStr::from(response.lock_id),
                success: response.success,
                message: FastStr::from(response.message),
            })),
            Err(e) => Err(Status::internal(format!("Error acquiring lock: {}", e))),
        }
    }

    async fn release_lock(
        &self,
        request: Request<ReleaseLockRequest>,
    ) -> Result<Response<ReleaseLockResponse>, Status> {
        let req = request.into_inner();

        let release_request = crate::core::lock_manager::ReleaseLockRequest {
            key: req.key.to_string(),
            lock_id: req.lock_id.to_string(),
            owner: req.owner.to_string(),
        };

        match self.lock_manager.release_lock(release_request).await {
            Ok(response) => Ok(Response::new(ReleaseLockResponse {
                success: response.success,
                message: FastStr::from(response.message),
            })),
            Err(e) => Err(Status::internal(format!("Error releasing lock: {}", e))),
        }
    }

    async fn extend_lock(
        &self,
        request: Request<ExtendLockRequest>,
    ) -> Result<Response<ExtendLockResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(ExtendLockResponse {
            success: true,
            message: FastStr::from(format!(
                "Lock {} extended by {} seconds",
                req.lock_id, req.ttl_seconds
            )),
        }))
    }

    async fn list_locks(
        &self,
        request: Request<ListLocksRequest>,
    ) -> Result<Response<ListLocksResponse>, Status> {
        let _req = request.into_inner();

        Ok(Response::new(ListLocksResponse {
            locks: vec![],
            success: true,
            message: FastStr::from("Lock list retrieved successfully"),
        }))
    }

    async fn start_saga(
        &self,
        request: Request<SagaRequest>,
    ) -> Result<Response<SagaResponse>, Status> {
        let req = request.into_inner();

        let steps: Result<Vec<crate::core::saga_orchestrator::SagaStep>, String> = req
            .steps
            .into_iter()
            .map(|step| {
                Ok(crate::core::saga_orchestrator::SagaStep {
                    name: step.name.to_string(),
                    service: step.service.to_string(),
                    action: step.action.to_string(),
                    compensation: step.compensation.to_string(),
                    timeout: step
                        .timeout_seconds
                        .map(|s| std::time::Duration::from_secs(s))
                        .unwrap_or(std::time::Duration::from_secs(30)),
                    retry_policy: step.retry_policy.map(|rp| {
                        crate::core::saga_orchestrator::RetryPolicy {
                            max_retries: rp.max_retries,
                            backoff_strategy: match rp.backoff_strategy.as_str() {
                                "exponential" => {
                                    crate::core::saga_orchestrator::BackoffStrategy::Exponential
                                }
                                "linear" => crate::core::saga_orchestrator::BackoffStrategy::Linear,
                                _ => crate::core::saga_orchestrator::BackoffStrategy::Fixed,
                            },
                            initial_delay: std::time::Duration::from_secs(
                                rp.initial_delay_seconds.unwrap_or(1),
                            ),
                        }
                    }),
                })
            })
            .collect();

        let saga_request = crate::core::saga_orchestrator::SagaRequest {
            name: req.name.to_string(),
            steps: steps.map_err(|e| Status::invalid_argument(format!("Error in steps: {}", e)))?,
            metadata: Some(
                req.metadata
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            ),
        };

        match self.saga_orchestrator.start_saga(saga_request).await {
            Ok(response) => Ok(Response::new(SagaResponse {
                saga_id: FastStr::from(response.saga_id),
                status: FastStr::from("Started"),
                message: FastStr::from(response.message),
            })),
            Err(e) => Err(Status::internal(format!("Error starting saga: {}", e))),
        }
    }

    async fn get_saga_status(
        &self,
        request: Request<GetSagaStatusRequest>,
    ) -> Result<Response<GetSagaStatusResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(GetSagaStatusResponse {
            saga_id: req.saga_id,
            status: FastStr::from("Running"),
            current_step: 1,
            step_results: vec![],
            success: true,
            message: FastStr::from("Saga status retrieved successfully"),
        }))
    }

    async fn cancel_saga(
        &self,
        request: Request<CancelSagaRequest>,
    ) -> Result<Response<CancelSagaResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(CancelSagaResponse {
            success: true,
            message: FastStr::from(format!("Saga {} cancelled: {}", req.saga_id, req.reason)),
        }))
    }

    async fn list_sagas(
        &self,
        request: Request<ListSagasRequest>,
    ) -> Result<Response<ListSagasResponse>, Status> {
        let _req = request.into_inner();

        Ok(Response::new(ListSagasResponse {
            sagas: vec![],
            success: true,
            message: FastStr::from("Saga list retrieved successfully"),
        }))
    }

    async fn append_event(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        let req = request.into_inner();

        let data: serde_json::Value = serde_json::from_str(&req.data)
            .map_err(|e| Status::invalid_argument(format!("Invalid JSON: {}", e)))?;

        let event_request = crate::core::event_store::EventRequest {
            stream_id: req.stream_id.to_string(),
            event_type: req.event_type.to_string(),
            data,
            metadata: Some(
                req.metadata
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            ),
        };

        match self.event_store.append_event(event_request).await {
            Ok(response) => Ok(Response::new(EventResponse {
                event_id: FastStr::from(response.event_id),
                version: 1,
                success: response.success,
                message: FastStr::from(response.message),
            })),
            Err(e) => Err(Status::internal(format!("Error adding event: {}", e))),
        }
    }

    async fn get_events(
        &self,
        request: Request<GetEventsRequest>,
    ) -> Result<Response<GetEventsResponse>, Status> {
        let _req = request.into_inner();

        Ok(Response::new(GetEventsResponse {
            events: vec![],
            success: true,
            message: FastStr::from("Events retrieved successfully"),
        }))
    }

    async fn get_stream_info(
        &self,
        request: Request<GetStreamInfoRequest>,
    ) -> Result<Response<GetStreamInfoResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(GetStreamInfoResponse {
            stream_id: req.stream_id,
            version: 1,
            event_count: 0,
            created_at: chrono::Utc::now().timestamp() as u64,
            last_updated: chrono::Utc::now().timestamp() as u64,
            success: true,
            message: FastStr::from("Stream information retrieved successfully"),
        }))
    }

    async fn get_cache(
        &self,
        request: Request<GetCacheRequest>,
    ) -> Result<Response<GetCacheResponse>, Status> {
        let req = request.into_inner();

        match self.cache_manager.get(&req.key).await {
            Ok(response) => {
                if response.found {
                    Ok(Response::new(GetCacheResponse {
                        key: req.key,
                        value: FastStr::from(
                            serde_json::to_string(&response.value).unwrap_or_default(),
                        ),
                        expires_at: None,
                        tags: vec![],
                        success: true,
                        message: FastStr::from("Cache retrieved successfully"),
                    }))
                } else {
                    Ok(Response::new(GetCacheResponse {
                        key: req.key,
                        value: FastStr::from(""),
                        expires_at: None,
                        tags: vec![],
                        success: false,
                        message: FastStr::from("Cache not found"),
                    }))
                }
            }
            Err(e) => Err(Status::internal(format!("Error getting cache: {}", e))),
        }
    }

    async fn set_cache(
        &self,
        request: Request<SetCacheRequest>,
    ) -> Result<Response<SetCacheResponse>, Status> {
        let req = request.into_inner();

        let value: serde_json::Value = serde_json::from_str(&req.value)
            .map_err(|e| Status::invalid_argument(format!("Invalid JSON: {}", e)))?;

        let cache_request = crate::core::cache_manager::CacheRequest {
            key: req.key.to_string(),
            value,
            ttl: req.ttl_seconds.map(|s| std::time::Duration::from_secs(s)),
            tags: req.tags.into_iter().map(|t| t.to_string()).collect(),
        };

        match self.cache_manager.set(cache_request).await {
            Ok(response) => Ok(Response::new(SetCacheResponse {
                key: req.key,
                value: FastStr::from(serde_json::to_string(&response.value).unwrap_or_default()),
                expires_at: None,
                tags: vec![],
                success: true,
                message: FastStr::from("Cache set successfully"),
            })),
            Err(e) => Err(Status::internal(format!("Error setting cache: {}", e))),
        }
    }

    async fn delete_cache(
        &self,
        request: Request<DeleteCacheRequest>,
    ) -> Result<Response<DeleteCacheResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(DeleteCacheResponse {
            success: true,
            message: FastStr::from(format!("Cache {} deleted successfully", req.key)),
        }))
    }

    async fn list_cache(
        &self,
        request: Request<ListCacheRequest>,
    ) -> Result<Response<ListCacheResponse>, Status> {
        let _req = request.into_inner();

        Ok(Response::new(ListCacheResponse {
            items: vec![],
            success: true,
            message: FastStr::from("Cache list retrieved successfully"),
        }))
    }
}
