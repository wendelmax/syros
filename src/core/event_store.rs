//! Event store implementation for event sourcing.
//!
//! This module provides an event store that implements the event sourcing pattern,
//! allowing applications to store and replay events for state reconstruction.

use crate::storage::postgres::PostgresManager;
use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: String,
    pub stream_id: String,
    pub event_type: String,
    #[sqlx(json)]
    pub data: serde_json::Value,
    #[sqlx(json)]
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRequest {
    pub stream_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResponse {
    pub event_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventsRequest {
    pub stream_id: String,
    pub from_version: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventsResponse {
    pub stream_id: String,
    pub events: Vec<Event>,
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct EventStore {
    pg: PostgresManager,
}

impl EventStore {
    pub fn new(pg: PostgresManager) -> Self {
        Self { pg }
    }

    pub async fn append_event(&self, request: EventRequest) -> Result<EventResponse> {
        let event_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let pool = self.pg.get_pool();

        // Transaction to ensure version consistency
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        // Get expected version (optimistic concurrency can be added here)
        let version: i64 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(version), 0) + 1 FROM events WHERE stream_id = $1",
        )
        .bind(&request.stream_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        sqlx::query(
            "INSERT INTO events (id, stream_id, event_type, data, metadata, version, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(Uuid::parse_str(&event_id).unwrap_or_default())
        .bind(&request.stream_id)
        .bind(&request.event_type)
        .bind(sqlx::types::Json(request.data))
        .bind(sqlx::types::Json(request.metadata))
        .bind(version)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(EventResponse {
            event_id,
            success: true,
            message: "Event appended successfully".to_string(),
        })
    }

    pub async fn get_events(&self, request: GetEventsRequest) -> Result<GetEventsResponse> {
        let pool = self.pg.get_pool();
        let mut query = "SELECT id::text, stream_id, event_type, data, metadata, created_at as timestamp, version::bigint FROM events WHERE stream_id = $1".to_string();

        if let Some(from_version) = request.from_version {
            query.push_str(&format!(" AND version >= {}", from_version));
        }

        query.push_str(" ORDER BY version ASC");

        if let Some(limit) = request.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let events: Vec<Event> = sqlx::query_as(&query)
            .bind(&request.stream_id)
            .fetch_all(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        if !events.is_empty() {
            Ok(GetEventsResponse {
                stream_id: request.stream_id,
                events,
                success: true,
                message: "Events retrieved successfully".to_string(),
            })
        } else {
            Ok(GetEventsResponse {
                stream_id: request.stream_id,
                events: Vec::new(),
                success: true,
                message: "No events found".to_string(),
            })
        }
    }

    pub async fn get_stream_version(&self, stream_id: &str) -> Result<i64> {
        let pool = self.pg.get_pool();
        let version: i64 =
            sqlx::query_scalar("SELECT COALESCE(MAX(version), 0) FROM events WHERE stream_id = $1")
                .bind(stream_id)
                .fetch_one(pool)
                .await
                .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(version)
    }

    pub async fn get_stream_events_count(&self, stream_id: &str) -> Result<usize> {
        let pool = self.pg.get_pool();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM events WHERE stream_id = $1")
            .bind(stream_id)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::SyrosError::StorageError(e.to_string()))?;

        Ok(count as usize)
    }

    pub async fn cleanup_old_events(&self, _stream_id: &str, _keep_last: usize) -> Result<u64> {
        // This is complex in SQL without subqueries or window functions, but doable.
        // Simplified approach for now (no-op):
        Ok(0)
    }
}
