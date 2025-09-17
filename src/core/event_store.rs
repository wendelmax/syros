//! Event store implementation for event sourcing.
//!
//! This module provides an event store that implements the event sourcing pattern,
//! allowing applications to store and replay events for state reconstruction.

use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub stream_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub version: u64,
}

#[derive(Debug, Clone)]
pub struct EventRequest {
    pub stream_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct EventResponse {
    pub event_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct GetEventsRequest {
    pub stream_id: String,
    pub from_version: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct GetEventsResponse {
    pub stream_id: String,
    pub events: Vec<Event>,
    pub success: bool,
    pub message: String,
}

#[derive(Clone)]
pub struct EventStore {
    events: Arc<RwLock<HashMap<String, Vec<Event>>>>,
    versions: Arc<RwLock<HashMap<String, u64>>>,
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
            versions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn append_event(&self, request: EventRequest) -> Result<EventResponse> {
        let event_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Incrementa versão
        let mut versions = self.versions.write().await;
        let version = *versions.entry(request.stream_id.clone()).or_insert(0) + 1;
        versions.insert(request.stream_id.clone(), version);
        drop(versions);

        let event = Event {
            id: event_id.clone(),
            stream_id: request.stream_id.clone(),
            event_type: request.event_type,
            data: request.data,
            metadata: request.metadata.unwrap_or_default(),
            timestamp: now,
            version,
        };

        let mut events = self.events.write().await;
        events
            .entry(request.stream_id)
            .or_insert_with(Vec::new)
            .push(event);

        Ok(EventResponse {
            event_id,
            success: true,
            message: "Event appended successfully".to_string(),
        })
    }

    pub async fn get_events(&self, request: GetEventsRequest) -> Result<GetEventsResponse> {
        let events = self.events.read().await;

        if let Some(stream_events) = events.get(&request.stream_id) {
            let mut filtered_events = stream_events.clone();

            // Filtra por versão se especificado
            if let Some(from_version) = request.from_version {
                filtered_events.retain(|event| event.version >= from_version);
            }

            // Aplica limite se especificado
            if let Some(limit) = request.limit {
                filtered_events.truncate(limit as usize);
            }

            Ok(GetEventsResponse {
                stream_id: request.stream_id,
                events: filtered_events,
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

    pub async fn get_stream_version(&self, stream_id: &str) -> Result<u64> {
        let versions = self.versions.read().await;
        Ok(versions.get(stream_id).copied().unwrap_or(0))
    }

    pub async fn get_stream_events_count(&self, stream_id: &str) -> Result<usize> {
        let events = self.events.read().await;
        Ok(events.get(stream_id).map(|v| v.len()).unwrap_or(0))
    }

    pub async fn cleanup_old_events(&self, stream_id: &str, keep_last: usize) -> Result<u64> {
        let mut events = self.events.write().await;

        if let Some(stream_events) = events.get_mut(stream_id) {
            let initial_count = stream_events.len();
            if initial_count > keep_last {
                let remove_count = initial_count - keep_last;
                stream_events.drain(0..remove_count);
                return Ok(remove_count as u64);
            }
        }

        Ok(0)
    }
}
