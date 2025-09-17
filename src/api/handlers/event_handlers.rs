//! Event handlers for the Syros API.
//!
//! This module provides HTTP handlers for event sourcing operations,
//! including appending events to streams and retrieving event history.

use crate::core::event_store::{
    EventRequest, EventResponse, EventStore, GetEventsRequest, GetEventsResponse,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

/// Request structure for appending an event to a stream.
#[derive(Debug, Deserialize)]
pub struct AppendEventRequest {
    /// Type of the event
    pub event_type: String,
    /// Event data (JSON)
    pub data: serde_json::Value,
    /// Optional metadata for the event
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Query parameters for retrieving events from a stream.
#[derive(Debug, Deserialize)]
pub struct GetEventsQuery {
    /// Start from this version (optional)
    pub from_version: Option<u64>,
    /// Maximum number of events to return (optional)
    pub limit: Option<u64>,
}

/// Response structure for event data.
#[derive(Debug, Serialize)]
pub struct EventResponseData {
    /// Unique identifier of the event
    pub id: String,
    /// Stream identifier
    pub stream_id: String,
    /// Type of the event
    pub event_type: String,
    /// Event data (JSON)
    pub data: serde_json::Value,
    /// Event metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Timestamp when the event was created
    pub timestamp: String,
    /// Version number of the event in the stream
    pub version: u64,
}

/// Appends an event to the specified stream.
///
/// This handler adds a new event to the event store with the provided
/// stream ID, event type, data, and optional metadata.
///
/// # Arguments
///
/// * `event_store` - Event store instance
/// * `stream_id` - Stream identifier
/// * `request` - Event data and metadata
///
/// # Returns
///
/// Returns a JSON response with event information or an error status.
pub async fn append_event(
    State(event_store): State<EventStore>,
    Path(stream_id): Path<String>,
    Json(request): Json<AppendEventRequest>,
) -> Result<Json<EventResponse>, StatusCode> {
    let event_request = EventRequest {
        stream_id,
        event_type: request.event_type,
        data: request.data,
        metadata: request.metadata,
    };

    match event_store.append_event(event_request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("Error appending event: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves events from the specified stream.
///
/// This handler fetches events from a stream with optional filtering
/// by version and limiting the number of results.
///
/// # Arguments
///
/// * `event_store` - Event store instance
/// * `stream_id` - Stream identifier
/// * `params` - Query parameters for filtering
///
/// # Returns
///
/// Returns a JSON response with the list of events or an error status.
pub async fn get_events(
    State(event_store): State<EventStore>,
    Path(stream_id): Path<String>,
    Query(params): Query<GetEventsQuery>,
) -> Result<Json<GetEventsResponse>, StatusCode> {
    let get_events_request = GetEventsRequest {
        stream_id,
        from_version: params.from_version,
        limit: params.limit,
    };

    match event_store.get_events(get_events_request).await {
        Ok(response) => {
            let events_data: Vec<EventResponseData> = response
                .events
                .into_iter()
                .map(|event| EventResponseData {
                    id: event.id,
                    stream_id: event.stream_id,
                    event_type: event.event_type,
                    data: event.data,
                    metadata: event.metadata,
                    timestamp: event.timestamp.to_rfc3339(),
                    version: event.version,
                })
                .collect();

            Ok(Json(GetEventsResponse {
                stream_id: response.stream_id,
                events: events_data
                    .into_iter()
                    .map(|e| crate::core::event_store::Event {
                        id: e.id,
                        stream_id: e.stream_id,
                        event_type: e.event_type,
                        data: e.data,
                        metadata: e.metadata,
                        timestamp: chrono::DateTime::parse_from_rfc3339(&e.timestamp)
                            .unwrap()
                            .with_timezone(&chrono::Utc),
                        version: e.version,
                    })
                    .collect(),
                success: response.success,
                message: response.message,
            }))
        }
        Err(e) => {
            eprintln!("Error getting events: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
