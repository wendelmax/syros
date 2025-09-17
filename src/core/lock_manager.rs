//! Distributed lock manager implementation.
//!
//! This module provides a distributed lock manager that allows multiple processes
//! to coordinate access to shared resources by acquiring and releasing locks.

use crate::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Represents the state of a distributed lock.
#[derive(Debug, Clone)]
pub struct LockState {
    /// Unique identifier for the lock
    pub id: String,
    /// Lock key/name
    pub key: String,
    /// Owner of the lock
    pub owner: String,
    /// When the lock was acquired
    pub acquired_at: DateTime<Utc>,
    /// When the lock expires
    pub expires_at: DateTime<Utc>,
    /// Optional metadata associated with the lock
    pub metadata: Option<String>,
}

/// Request to acquire a distributed lock.
#[derive(Debug, Clone)]
pub struct LockRequest {
    /// Lock key/name
    pub key: String,
    /// Time-to-live for the lock
    pub ttl: Duration,
    /// Optional metadata
    pub metadata: Option<String>,
    /// Owner identifier
    pub owner: String,
    /// Maximum time to wait for lock acquisition
    pub wait_timeout: Option<Duration>,
}

/// Response from a lock acquisition attempt.
#[derive(Debug, Clone)]
pub struct LockResponse {
    /// Unique identifier for the acquired lock
    pub lock_id: String,
    /// Whether the lock was successfully acquired
    pub success: bool,
    /// Status message
    pub message: String,
}

/// Request to release a distributed lock.
#[derive(Debug, Clone)]
pub struct ReleaseLockRequest {
    /// Lock key/name
    pub key: String,
    /// Lock identifier to release
    pub lock_id: String,
    /// Owner identifier
    pub owner: String,
}

/// Response from a lock release attempt.
#[derive(Debug, Clone)]
pub struct ReleaseLockResponse {
    /// Whether the lock was successfully released
    pub success: bool,
    /// Status message
    pub message: String,
}

/// Distributed lock manager for coordinating access to shared resources.
#[derive(Clone)]
pub struct LockManager {
    locks: Arc<RwLock<HashMap<String, LockState>>>,
}

impl LockManager {
    /// Creates a new lock manager instance.
    ///
    /// # Returns
    ///
    /// Returns a new `LockManager` with an empty lock registry.
    pub fn new() -> Self {
        Self {
            locks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Attempts to acquire a distributed lock.
    ///
    /// This method tries to acquire a lock with the specified key. If a lock
    /// already exists and hasn't expired, the acquisition will fail.
    ///
    /// # Arguments
    ///
    /// * `request` - Lock acquisition request containing key, TTL, owner, etc.
    ///
    /// # Returns
    ///
    /// Returns a `LockResponse` indicating success or failure of the acquisition.
    pub async fn acquire_lock(&self, request: LockRequest) -> Result<LockResponse> {
        let mut locks = self.locks.write().await;

        if let Some(existing_lock) = locks.get(&request.key) {
            if existing_lock.expires_at > Utc::now() {
                return Ok(LockResponse {
                    lock_id: String::new(),
                    success: false,
                    message: "Lock already exists".to_string(),
                });
            }
        }

        let lock_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::from_std(request.ttl).unwrap();

        let lock_state = LockState {
            id: lock_id.clone(),
            key: request.key.clone(),
            owner: request.owner.clone(),
            acquired_at: now,
            expires_at,
            metadata: request.metadata,
        };

        locks.insert(request.key, lock_state);

        Ok(LockResponse {
            lock_id,
            success: true,
            message: "Lock acquired successfully".to_string(),
        })
    }

    /// Releases a distributed lock.
    ///
    /// This method releases a lock if the requester is the owner of the lock.
    ///
    /// # Arguments
    ///
    /// * `request` - Lock release request containing key, lock ID, and owner
    ///
    /// # Returns
    ///
    /// Returns a `ReleaseLockResponse` indicating success or failure of the release.
    pub async fn release_lock(&self, request: ReleaseLockRequest) -> Result<ReleaseLockResponse> {
        let mut locks = self.locks.write().await;

        if let Some(lock_state) = locks.get(&request.key) {
            if lock_state.id == request.lock_id && lock_state.owner == request.owner {
                locks.remove(&request.key);
                return Ok(ReleaseLockResponse {
                    success: true,
                    message: "Lock released successfully".to_string(),
                });
            }
        }

        Ok(ReleaseLockResponse {
            success: false,
            message: "Lock not found or not owned by requester".to_string(),
        })
    }

    /// Gets the current status of a lock.
    ///
    /// This method returns the current state of a lock if it exists and hasn't expired.
    /// Expired locks are automatically removed.
    ///
    /// # Arguments
    ///
    /// * `key` - Lock key to check
    ///
    /// # Returns
    ///
    /// Returns `Some(LockState)` if the lock exists and is active, `None` otherwise.
    pub async fn get_lock_status(&self, key: &str) -> Result<Option<LockState>> {
        let locks = self.locks.read().await;

        if let Some(lock_state) = locks.get(key) {
            if lock_state.expires_at > Utc::now() {
                return Ok(Some(lock_state.clone()));
            } else {
                drop(locks);
                let mut locks = self.locks.write().await;
                locks.remove(key);
                return Ok(None);
            }
        }

        Ok(None)
    }

    /// Cleans up expired locks from the registry.
    ///
    /// This method removes all locks that have expired from the internal registry.
    ///
    /// # Returns
    ///
    /// Returns the number of expired locks that were removed.
    pub async fn cleanup_expired_locks(&self) -> Result<u64> {
        let mut locks = self.locks.write().await;
        let now = Utc::now();
        let initial_count = locks.len();

        locks.retain(|_, lock_state| lock_state.expires_at > now);

        Ok((initial_count - locks.len()) as u64)
    }
}
