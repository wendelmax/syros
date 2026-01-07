//! GraphQL types and schemas for the Syros API.
//!
//! This module defines all GraphQL types, enums, input objects, and response
//! structures used in the GraphQL API for distributed coordination operations.

use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a distributed lock in the system.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Lock {
    /// Unique key identifier for the lock
    pub key: String,
    /// Owner of the lock
    pub owner: String,
    /// Timestamp when the lock was acquired
    pub acquired_at: DateTime<Utc>,
    /// Timestamp when the lock expires (optional)
    pub expires_at: Option<DateTime<Utc>>,
    /// Current status of the lock
    pub status: LockStatus,
}

/// Represents a saga orchestration instance.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Saga {
    /// Unique identifier of the saga
    pub id: String,
    /// Name of the saga
    pub name: String,
    /// Current status of the saga
    pub status: SagaStatus,
    /// List of steps in the saga
    pub steps: Vec<SagaStep>,
    /// Timestamp when the saga was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the saga was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents a single step in a saga.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SagaStep {
    /// Unique identifier of the step
    pub id: String,
    /// Name of the step
    pub name: String,
    /// Current status of the step
    pub status: StepStatus,
    /// Compensation action for this step (optional)
    pub compensation: Option<String>,
    /// Timestamp when the step was executed (optional)
    pub executed_at: Option<DateTime<Utc>>,
}

/// Represents an event in the event store.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier of the event
    pub id: String,
    /// Stream identifier this event belongs to
    pub stream_id: String,
    /// Type of the event
    pub event_type: String,
    /// Event data (JSON string)
    pub data: String,
    /// Event metadata (JSON string)
    pub metadata: String,
    /// Version number in the stream
    pub version: i32,
    /// Timestamp when the event was created
    pub created_at: DateTime<Utc>,
}

/// Represents a cache entry.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cache key
    pub key: String,
    /// Cached value (JSON string)
    pub value: String,
    /// Time-to-live in seconds (optional)
    pub ttl: Option<i32>,
    /// Timestamp when the entry was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the entry expires (optional)
    pub expires_at: Option<DateTime<Utc>>,
}

/// Represents a user in the system.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier of the user
    pub id: String,
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// List of roles assigned to the user
    pub roles: Vec<String>,
    /// Whether the user is active
    pub is_active: bool,
    /// Timestamp when the user was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents a role in the RBAC system.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    /// Name of the role
    pub name: String,
    /// Description of the role
    pub description: String,
    /// List of permissions for this role
    pub permissions: Vec<String>,
    /// Whether this is a system role
    pub is_system: bool,
}

/// Represents a permission in the RBAC system.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// Name of the permission
    pub name: String,
    /// Type of resource this permission applies to
    pub resource_type: String,
    /// Action this permission allows
    pub action: String,
}

/// Status of a distributed lock.
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LockStatus {
    /// Lock is currently held
    Locked,
    /// Lock is not held
    Unlocked,
    /// Lock has expired
    Expired,
}

/// Status of a saga orchestration.
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum SagaStatus {
    /// Saga is waiting to start
    Pending,
    /// Saga is currently executing
    Running,
    /// Saga completed successfully
    Completed,
    /// Saga failed during execution
    Failed,
    /// Saga was compensated after failure
    Compensated,
}

/// Status of a saga step.
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum StepStatus {
    /// Step is waiting to execute
    Pending,
    /// Step is currently executing
    Running,
    /// Step completed successfully
    Completed,
    /// Step failed during execution
    Failed,
    /// Step was compensated after failure
    Compensated,
}

/// Input for acquiring a distributed lock.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct AcquireLockInput {
    /// Lock key identifier
    pub key: String,
    /// Time-to-live in seconds (optional)
    pub ttl: Option<i32>,
}

/// Input for starting a new saga.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct StartSagaInput {
    /// Name of the saga
    pub name: String,
    /// List of steps in the saga
    pub steps: Vec<SagaStepInput>,
}

/// Input for defining a saga step.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SagaStepInput {
    /// Name of the step
    pub name: String,
    /// Compensation action (optional)
    pub compensation: Option<String>,
}

/// Input for appending an event to a stream.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct AppendEventInput {
    /// Stream identifier
    pub stream_id: String,
    /// Type of the event
    pub event_type: String,
    /// Event data (JSON string)
    pub data: String,
    /// Event metadata (JSON string, optional)
    pub metadata: Option<String>,
}

/// Input for setting a cache entry.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct SetCacheInput {
    /// Cache key
    pub key: String,
    /// Value to cache (JSON string)
    pub value: String,
    /// Time-to-live in seconds (optional)
    pub ttl: Option<i32>,
}

/// Input for creating a new user.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserInput {
    /// Username
    pub username: String,
    /// Email address
    pub email: String,
    /// List of roles to assign
    pub roles: Vec<String>,
}

/// Input for updating user roles.
#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserRolesInput {
    /// User identifier
    pub user_id: String,
    /// New list of roles
    pub roles: Vec<String>,
}

/// Response for lock operations.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct LockResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Lock information (if successful)
    pub lock: Option<Lock>,
}

/// Response for saga operations.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct SagaResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Saga information (if successful)
    pub saga: Option<Saga>,
}

/// Response for event operations.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct EventResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Event information (if successful)
    pub event: Option<Event>,
}

/// Response for cache operations.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct CacheResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Cache entry information (if successful)
    pub entry: Option<CacheEntry>,
}

/// Response for user operations.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// User information (if successful)
    pub user: Option<User>,
}

/// Response for permission check operations.
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    /// Whether the user has the permission
    pub has_permission: bool,
    /// User identifier
    pub user_id: String,
    /// Permission that was checked
    pub permission: String,
    /// Resource identifier (if applicable)
    pub resource_id: Option<String>,
}
