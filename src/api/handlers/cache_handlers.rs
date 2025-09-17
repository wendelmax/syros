use crate::core::cache_manager::{
    CacheManager, CacheRequest, CacheResponse, DeleteCacheRequest, DeleteCacheResponse,
    InvalidateByTagRequest, InvalidateByTagResponse,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SetCacheRequest {
    pub value: serde_json::Value,
    pub ttl_seconds: Option<u64>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct InvalidateByTagRequestPayload {
    pub tag: String,
}

#[derive(Debug, Serialize)]
pub struct CacheStatsResponse {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
}

pub async fn get_cache(
    State(cache_manager): State<CacheManager>,
    Path(key): Path<String>,
) -> Result<Json<CacheResponse>, StatusCode> {
    match cache_manager.get(&key).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Erro ao obter cache: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn set_cache(
    State(cache_manager): State<CacheManager>,
    Path(key): Path<String>,
    Json(request): Json<SetCacheRequest>,
) -> Result<Json<CacheResponse>, StatusCode> {
    let cache_request = CacheRequest {
        key,
        value: request.value,
        ttl: request.ttl_seconds.map(std::time::Duration::from_secs),
        tags: request.tags.unwrap_or_default(),
    };

    match cache_manager.set(cache_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Erro ao definir cache: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_cache(
    State(cache_manager): State<CacheManager>,
    Path(key): Path<String>,
) -> Result<Json<DeleteCacheResponse>, StatusCode> {
    let delete_request = DeleteCacheRequest { key };

    match cache_manager.delete(delete_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Erro ao deletar cache: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn invalidate_by_tag(
    State(cache_manager): State<CacheManager>,
    Path(tag): Path<String>,
) -> Result<Json<InvalidateByTagResponse>, StatusCode> {
    let invalidate_request = InvalidateByTagRequest { tag };

    match cache_manager.invalidate_by_tag(invalidate_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Erro ao invalidar cache por tag: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_cache_stats(
    State(cache_manager): State<CacheManager>,
) -> Result<Json<CacheStatsResponse>, StatusCode> {
    match cache_manager.get_stats().await {
        Ok(stats) => Ok(Json(CacheStatsResponse {
            total_entries: stats.total_entries,
            expired_entries: stats.expired_entries,
            active_entries: stats.active_entries,
        })),
        Err(e) => {
            eprintln!("Erro ao obter estatísticas do cache: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
