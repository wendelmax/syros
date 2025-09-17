//! Cache manager implementation for distributed caching.
//!
//! This module provides a cache manager that implements distributed caching
//! with TTL support and tagging capabilities.

use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub value: serde_json::Value,
    pub expires_at: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CacheRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub ttl: Option<Duration>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CacheResponse {
    pub key: String,
    pub value: Option<serde_json::Value>,
    pub found: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct DeleteCacheRequest {
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct DeleteCacheResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct InvalidateByTagRequest {
    pub tag: String,
}

#[derive(Debug, Clone)]
pub struct InvalidateByTagResponse {
    pub invalidated_count: u64,
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct CacheManager {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, request: CacheRequest) -> Result<CacheResponse> {
        let now = Utc::now();
        let expires_at = request
            .ttl
            .map(|ttl| now + chrono::Duration::from_std(ttl).unwrap());

        let entry = CacheEntry {
            key: request.key.clone(),
            value: request.value.clone(),
            expires_at,
            tags: request.tags,
            created_at: now,
        };

        let mut cache = self.cache.write().await;
        cache.insert(request.key.clone(), entry);

        Ok(CacheResponse {
            key: request.key,
            value: Some(request.value),
            found: true,
            message: "Cache set successfully".to_string(),
        })
    }

    pub async fn get(&self, key: &str) -> Result<CacheResponse> {
        let mut cache = self.cache.write().await;
        let now = Utc::now();

        if let Some(entry) = cache.get(key) {
            if let Some(expires_at) = entry.expires_at {
                if expires_at <= now {
                    cache.remove(key);
                    return Ok(CacheResponse {
                        key: key.to_string(),
                        value: None,
                        found: false,
                        message: "Cache expired".to_string(),
                    });
                }
            }

            Ok(CacheResponse {
                key: key.to_string(),
                value: Some(entry.value.clone()),
                found: true,
                message: "Cache retrieved successfully".to_string(),
            })
        } else {
            Ok(CacheResponse {
                key: key.to_string(),
                value: None,
                found: false,
                message: "Cache key not found".to_string(),
            })
        }
    }

    pub async fn delete(&self, request: DeleteCacheRequest) -> Result<DeleteCacheResponse> {
        let mut cache = self.cache.write().await;

        if cache.remove(&request.key).is_some() {
            Ok(DeleteCacheResponse {
                success: true,
                message: "Cache deleted successfully".to_string(),
            })
        } else {
            Ok(DeleteCacheResponse {
                success: false,
                message: "Cache key not found".to_string(),
            })
        }
    }

    pub async fn invalidate_by_tag(
        &self,
        request: InvalidateByTagRequest,
    ) -> Result<InvalidateByTagResponse> {
        let mut cache = self.cache.write().await;
        let initial_count = cache.len();

        cache.retain(|_, entry| !entry.tags.contains(&request.tag));

        let invalidated_count = (initial_count - cache.len()) as u64;

        Ok(InvalidateByTagResponse {
            invalidated_count,
            success: true,
            message: format!("Invalidated {} cache entries", invalidated_count),
        })
    }

    pub async fn cleanup_expired(&self) -> Result<u64> {
        let mut cache = self.cache.write().await;
        let now = Utc::now();
        let initial_count = cache.len();

        cache.retain(|_, entry| {
            if let Some(expires_at) = entry.expires_at {
                expires_at > now
            } else {
                true // No expiration
            }
        });

        Ok((initial_count - cache.len()) as u64)
    }

    pub async fn get_stats(&self) -> Result<CacheStats> {
        let cache = self.cache.read().await;
        let now = Utc::now();

        let total_entries = cache.len();
        let expired_entries = cache
            .values()
            .filter(|entry| {
                if let Some(expires_at) = entry.expires_at {
                    expires_at <= now
                } else {
                    false
                }
            })
            .count();

        Ok(CacheStats {
            total_entries,
            expired_entries,
            active_entries: total_entries - expired_entries,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
}
