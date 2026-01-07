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
    response::IntoResponse,
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
    pub from_version: Option<i64>,
    /// Maximum number of events to return (optional)
    pub limit: Option<i64>,
}

/// Response structure for event data.
#[derive(Debug, Serialize, Deserialize)]
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
    pub version: i64,
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
) -> impl IntoResponse {
    let event_request = EventRequest {
        stream_id,
        event_type: request.event_type,
        data: request.data,
        metadata: request.metadata,
    };

    match event_store.append_event(event_request).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Error appending event: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
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
) -> impl IntoResponse {
    let get_events_request = GetEventsRequest {
        stream_id,
        from_version: params.from_version,
        limit: params.limit,
    };

    match event_store.get_events(get_events_request).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Error getting events: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
