use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use volo::FastStr;
use volo_grpc::body::BoxBody;
use volo_grpc::{Request, Response, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockRequest {
    pub key: FastStr,
    pub owner: FastStr,
    pub ttl_seconds: u64,
    pub metadata: Option<FastStr>,
    pub wait_timeout_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockResponse {
    pub lock_id: FastStr,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseLockRequest {
    pub key: FastStr,
    pub lock_id: FastStr,
    pub owner: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseLockResponse {
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendLockRequest {
    pub key: FastStr,
    pub lock_id: FastStr,
    pub owner: FastStr,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendLockResponse {
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLocksRequest {
    pub owner: Option<FastStr>,
    pub pattern: Option<FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLocksResponse {
    pub locks: Vec<LockInfo>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockInfo {
    pub key: FastStr,
    pub lock_id: FastStr,
    pub owner: FastStr,
    pub expires_at: u64,
    pub metadata: Option<FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaRequest {
    pub name: FastStr,
    pub steps: Vec<SagaStep>,
    pub metadata: HashMap<FastStr, FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaStep {
    pub name: FastStr,
    pub service: FastStr,
    pub action: FastStr,
    pub compensation: FastStr,
    pub timeout_seconds: Option<u64>,
    pub retry_policy: Option<RetryPolicy>,
    pub payload: Option<FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_strategy: FastStr,
    pub initial_delay_seconds: Option<u64>,
    pub max_delay_seconds: Option<u64>,
    pub factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaResponse {
    pub saga_id: FastStr,
    pub status: FastStr,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSagaStatusRequest {
    pub saga_id: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSagaStatusResponse {
    pub saga_id: FastStr,
    pub status: FastStr,
    pub current_step: u32,
    pub step_results: Vec<StepResult>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_name: FastStr,
    pub status: FastStr,
    pub error: Option<FastStr>,
    pub started_at: u64,
    pub completed_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSagaRequest {
    pub saga_id: FastStr,
    pub reason: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSagaResponse {
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSagasRequest {
    pub status: Option<FastStr>,
    pub owner: Option<FastStr>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSagasResponse {
    pub sagas: Vec<SagaInfo>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaInfo {
    pub saga_id: FastStr,
    pub name: FastStr,
    pub status: FastStr,
    pub current_step: u32,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRequest {
    pub stream_id: FastStr,
    pub event_type: FastStr,
    pub data: FastStr,
    pub metadata: HashMap<FastStr, FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResponse {
    pub event_id: FastStr,
    pub version: u64,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventsRequest {
    pub stream_id: FastStr,
    pub from_version: Option<u64>,
    pub to_version: Option<u64>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventsResponse {
    pub events: Vec<Event>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: FastStr,
    pub stream_id: FastStr,
    pub event_type: FastStr,
    pub data: FastStr,
    pub version: u64,
    pub timestamp: u64,
    pub metadata: HashMap<FastStr, FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStreamInfoRequest {
    pub stream_id: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStreamInfoResponse {
    pub stream_id: FastStr,
    pub version: u64,
    pub event_count: u64,
    pub created_at: u64,
    pub last_updated: u64,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCacheRequest {
    pub key: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCacheResponse {
    pub key: FastStr,
    pub value: FastStr,
    pub expires_at: Option<FastStr>,
    pub tags: Vec<FastStr>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheRequest {
    pub key: FastStr,
    pub value: FastStr,
    pub ttl_seconds: Option<u64>,
    pub tags: Vec<FastStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCacheResponse {
    pub key: FastStr,
    pub value: FastStr,
    pub expires_at: Option<FastStr>,
    pub tags: Vec<FastStr>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCacheRequest {
    pub key: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCacheResponse {
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCacheRequest {
    pub pattern: Option<FastStr>,
    pub tags: Vec<FastStr>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCacheResponse {
    pub items: Vec<CacheItem>,
    pub success: bool,
    pub message: FastStr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub key: FastStr,
    pub value: FastStr,
    pub expires_at: Option<FastStr>,
    pub tags: Vec<FastStr>,
}

#[async_trait::async_trait]
pub trait SyrosService {
    async fn acquire_lock(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<LockResponse>, Status>;
    async fn release_lock(
        &self,
        request: Request<ReleaseLockRequest>,
    ) -> Result<Response<ReleaseLockResponse>, Status>;
    async fn extend_lock(
        &self,
        request: Request<ExtendLockRequest>,
    ) -> Result<Response<ExtendLockResponse>, Status>;
    async fn list_locks(
        &self,
        request: Request<ListLocksRequest>,
    ) -> Result<Response<ListLocksResponse>, Status>;
    async fn start_saga(
        &self,
        request: Request<SagaRequest>,
    ) -> Result<Response<SagaResponse>, Status>;
    async fn get_saga_status(
        &self,
        request: Request<GetSagaStatusRequest>,
    ) -> Result<Response<GetSagaStatusResponse>, Status>;
    async fn cancel_saga(
        &self,
        request: Request<CancelSagaRequest>,
    ) -> Result<Response<CancelSagaResponse>, Status>;
    async fn list_sagas(
        &self,
        request: Request<ListSagasRequest>,
    ) -> Result<Response<ListSagasResponse>, Status>;
    async fn append_event(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventResponse>, Status>;
    async fn get_events(
        &self,
        request: Request<GetEventsRequest>,
    ) -> Result<Response<GetEventsResponse>, Status>;
    async fn get_stream_info(
        &self,
        request: Request<GetStreamInfoRequest>,
    ) -> Result<Response<GetStreamInfoResponse>, Status>;
    async fn get_cache(
        &self,
        request: Request<GetCacheRequest>,
    ) -> Result<Response<GetCacheResponse>, Status>;
    async fn set_cache(
        &self,
        request: Request<SetCacheRequest>,
    ) -> Result<Response<SetCacheResponse>, Status>;
    async fn delete_cache(
        &self,
        request: Request<DeleteCacheRequest>,
    ) -> Result<Response<DeleteCacheResponse>, Status>;
    async fn list_cache(
        &self,
        request: Request<ListCacheRequest>,
    ) -> Result<Response<ListCacheResponse>, Status>;
}

#[derive(Clone)]
pub struct SyrosServiceServer<T> {
    _inner: T,
}

impl<T> SyrosServiceServer<T> {
    pub fn new(service: T) -> Self {
        Self { _inner: service }
    }
}

impl<T: SyrosService + Clone + Send + Sync + 'static>
    volo::Service<volo_grpc::context::ServerContext, volo_grpc::Request<BoxBody>>
    for SyrosServiceServer<T>
{
    type Response = volo_grpc::Response<BoxBody>;
    type Error = volo_grpc::Status;

    async fn call(
        &self,
        _cx: &mut volo_grpc::context::ServerContext,
        _req: volo_grpc::Request<BoxBody>,
    ) -> Result<Self::Response, Self::Error> {
        Err(volo_grpc::Status::unimplemented("Método não implementado"))
    }
}

impl<T> volo_grpc::server::NamedService for SyrosServiceServer<T> {
    const NAME: &'static str = "syros.v1.SyrosService";
}
