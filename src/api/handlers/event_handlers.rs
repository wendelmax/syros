use crate::core::event_store::{
    EventRequest, EventResponse, EventStore, GetEventsRequest, GetEventsResponse,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AppendEventRequest {
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct GetEventsQuery {
    pub from_version: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct EventResponseData {
    pub id: String,
    pub stream_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: std::collections::HashMap<String, String>,
    pub timestamp: String,
    pub version: u64,
}

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
            eprintln!("Erro ao anexar evento: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

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
            // Converte os eventos para o formato de resposta
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
            eprintln!("Erro ao obter eventos: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
