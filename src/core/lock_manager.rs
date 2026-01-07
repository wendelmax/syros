//! Distributed lock manager implementation.
//!
//! This module provides a distributed lock manager that allows multiple processes
//! to coordinate access to shared resources by acquiring and releasing locks.

use crate::storage::redis::RedisManager;
use crate::Result;
use chrono::{DateTime, Utc};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Represents the state of a distributed lock.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockResponse {
    /// Unique identifier for the acquired lock
    pub lock_id: String,
    /// Whether the lock was successfully acquired
    pub success: bool,
    /// Status message
    pub message: String,
}

/// Request to release a distributed lock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseLockRequest {
    /// Lock key/name
    pub key: String,
    /// Lock identifier to release
    pub lock_id: String,
    /// Owner identifier
    pub owner: String,
}

/// Response from a lock release attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseLockResponse {
    /// Whether the lock was successfully released
    pub success: bool,
    /// Status message
    pub message: String,
}

/// Distributed lock manager for coordinating access to shared resources.
#[derive(Clone)]
pub struct LockManager {
    redis: RedisManager,
}

impl LockManager {
    /// Creates a new lock manager instance.
    pub fn new(redis: RedisManager) -> Self {
        Self { redis }
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
        let mut conn = self.redis.get_connection().await?;
        let lock_key = format!("syros:locks:{}", request.key);
        let lock_id = Uuid::new_v4().to_string();
        let ttl_ms = request.ttl.as_millis() as u64;

        // Try to acquire lock using SET NX PX
        let result: Option<String> = redis::cmd("SET")
            .arg(&lock_key)
            .arg(&lock_id)
            .arg("NX")
            .arg("PX")
            .arg(ttl_ms)
            .query_async(&mut conn)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        if result.is_some() {
            // Lock acquired
            Ok(LockResponse {
                lock_id,
                success: true,
                message: "Lock acquired successfully".to_string(),
            })
        } else {
            // Lock already exists
            Ok(LockResponse {
                lock_id: String::new(),
                success: false,
                message: "Lock already exists".to_string(),
            })
        }
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
        let mut conn = self.redis.get_connection().await?;
        let lock_key = format!("syros:locks:{}", request.key);

        // Lua script to safely release lock only if ID matches
        let script = redis::Script::new(
            r"
            if redis.call('get', KEYS[1]) == ARGV[1] then
                return redis.call('del', KEYS[1])
            else
                return 0
            end
            ",
        );

        let result: i32 = script
            .key(&lock_key)
            .arg(&request.lock_id)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        if result == 1 {
            Ok(ReleaseLockResponse {
                success: true,
                message: "Lock released successfully".to_string(),
            })
        } else {
            Ok(ReleaseLockResponse {
                success: false,
                message: "Lock not found or ID mismatch".to_string(),
            })
        }
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
    /// Gets the current status of a lock.
    pub async fn get_lock_status(&self, key: &str) -> Result<Option<LockState>> {
        let mut conn = self.redis.get_connection().await?;
        let lock_key = format!("syros:locks:{}", key);

        // In this simple implementation, we just check existence and return a dummy state
        // To do this properly, we should store the state as JSON, as per design.
        // For now, we return minimal info if it exists.
        let lock_id: Option<String> = conn
            .get(&lock_key)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        if let Some(id) = lock_id {
            // Calculate TTL remaining
            let ttl: i64 = conn
                .ttl(&lock_key)
                .await
                .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

            let now = Utc::now();
            let expires_at = now + chrono::Duration::seconds(ttl);

            Ok(Some(LockState {
                id,
                key: key.to_string(),
                owner: "unknown".to_string(), // We didn't store owner in main key
                acquired_at: now,             // Approximate
                expires_at,
                metadata: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Cleans up expired locks from the registry.
    ///
    /// Redis handles expiration automatically, so this is a no-op.
    pub async fn cleanup_expired_locks(&self) -> Result<u64> {
        Ok(0)
    }
}
