//! Cache handlers for the Syros API.
//!
//! This module provides HTTP handlers for distributed caching operations,
//! including setting, getting, deleting cache entries and managing cache by tags.

use crate::core::cache_manager::{
    CacheManager, CacheRequest, CacheResponse, DeleteCacheRequest, DeleteCacheResponse,
    InvalidateByTagRequest, InvalidateByTagResponse,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

/// Request structure for setting a cache entry.
#[derive(Debug, Deserialize)]
pub struct SetCacheRequest {
    /// Value to cache (JSON)
    pub value: serde_json::Value,
    /// Time-to-live in seconds (optional)
    pub ttl_seconds: Option<u64>,
    /// Tags for cache invalidation (optional)
    pub tags: Option<Vec<String>>,
}

/// Request structure for invalidating cache by tag.
#[derive(Debug, Deserialize)]
pub struct InvalidateByTagRequestPayload {
    /// Tag to invalidate
    pub tag: String,
}

/// Response structure for cache statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStatsResponse {
    /// Total number of cache entries
    pub total_entries: usize,
    /// Number of expired entries
    pub expired_entries: usize,
    /// Number of active entries
    pub active_entries: usize,
}

/// Retrieves a cache entry by its key.
///
/// This handler fetches a cached value using the provided key.
/// Returns the cached value if found and not expired, otherwise returns not found.
///
/// # Arguments
///
/// * `cache_manager` - Cache manager instance
/// * `key` - Cache key to retrieve
///
/// # Returns
///
/// Returns a JSON response with the cached value or an error status.
pub async fn get_cache(
    State(cache_manager): State<CacheManager>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    match cache_manager.get(&key).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Error getting cache: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Sets a cache entry with the specified key and value.
///
/// This handler stores a value in the cache with optional TTL and tags.
/// The value will be automatically expired if TTL is specified.
///
/// # Arguments
///
/// * `cache_manager` - Cache manager instance
/// * `key` - Cache key to set
/// * `request` - Cache value and configuration
///
/// # Returns
///
/// Returns a JSON response indicating success or failure.
pub async fn set_cache(
    State(cache_manager): State<CacheManager>,
    Path(key): Path<String>,
    Json(request): Json<SetCacheRequest>,
) -> impl IntoResponse {
    let cache_request = CacheRequest {
        key,
        value: request.value,
        ttl: request.ttl_seconds.map(std::time::Duration::from_secs),
        tags: request.tags.unwrap_or_default(),
    };

    match cache_manager.set(cache_request).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Error setting cache: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Deletes a cache entry by its key.
///
/// This handler removes a cached value using the provided key.
///
/// # Arguments
///
/// * `cache_manager` - Cache manager instance
/// * `key` - Cache key to delete
///
/// # Returns
///
/// Returns a JSON response indicating success or failure.
pub async fn delete_cache(
    State(cache_manager): State<CacheManager>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let delete_request = DeleteCacheRequest { key };

    match cache_manager.delete(delete_request).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Error deleting cache: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Invalidates all cache entries with the specified tag.
///
/// This handler removes all cached values that have the specified tag.
///
/// # Arguments
///
/// * `cache_manager` - Cache manager instance
/// * `tag` - Tag to invalidate
///
/// # Returns
///
/// Returns a JSON response with the number of invalidated entries.
pub async fn invalidate_by_tag(
    State(cache_manager): State<CacheManager>,
    Path(tag): Path<String>,
) -> impl IntoResponse {
    let invalidate_request = InvalidateByTagRequest { tag };

    match cache_manager.invalidate_by_tag(invalidate_request).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Error invalidating cache by tag: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Retrieves cache statistics and metrics.
///
/// This handler returns information about the cache including total entries,
/// expired entries, and active entries.
///
/// # Arguments
///
/// * `cache_manager` - Cache manager instance
///
/// # Returns
///
/// Returns a JSON response with cache statistics.
pub async fn get_cache_stats(
    State(cache_manager): State<CacheManager>,
) -> impl IntoResponse {
    match cache_manager.get_stats().await {
        Ok(stats) => Json(CacheStatsResponse {
            total_entries: stats.total_entries,
            expired_entries: stats.expired_entries,
            active_entries: stats.active_entries,
        })
        .into_response(),
        Err(e) => {
            eprintln!("Error getting cache statistics: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
