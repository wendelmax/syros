use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Lock {
    pub key: String,
    pub owner: String,
    pub acquired_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: LockStatus,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Saga {
    pub id: String,
    pub name: String,
    pub status: SagaStatus,
    pub steps: Vec<SagaStep>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SagaStep {
    pub id: String,
    pub name: String,
    pub status: StepStatus,
    pub compensation: Option<String>,
    pub executed_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub stream_id: String,
    pub event_type: String,
    pub data: String,
    pub metadata: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub value: String,
    pub ttl: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub is_system: bool,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    pub name: String,
    pub resource_type: String,
    pub action: String,
}

// Enums
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LockStatus {
    Locked,
    Unlocked,
    Expired,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum SagaStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Compensated,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Compensated,
}

// Input types
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct AcquireLockInput {
    pub key: String,
    pub ttl: Option<i32>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct StartSagaInput {
    pub name: String,
    pub steps: Vec<SagaStepInput>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SagaStepInput {
    pub name: String,
    pub compensation: Option<String>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct AppendEventInput {
    pub stream_id: String,
    pub event_type: String,
    pub data: String,
    pub metadata: Option<String>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SetCacheInput {
    pub key: String,
    pub value: String,
    pub ttl: Option<i32>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserRolesInput {
    pub user_id: String,
    pub roles: Vec<String>,
}

// Response types
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct LockResponse {
    pub success: bool,
    pub message: String,
    pub lock: Option<Lock>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SagaResponse {
    pub success: bool,
    pub message: String,
    pub saga: Option<Saga>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub success: bool,
    pub message: String,
    pub event: Option<Event>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct CacheResponse {
    pub success: bool,
    pub message: String,
    pub entry: Option<CacheEntry>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<User>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    pub has_permission: bool,
    pub user_id: String,
    pub permission: String,
    pub resource_id: Option<String>,
}
