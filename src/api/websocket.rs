//! WebSocket service for real-time communication.
//!
//! This module provides WebSocket functionality for real-time updates
//! and communication with the Syros distributed coordination service.

use crate::core::{CacheManager, EventStore, LockManager, SagaOrchestrator};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// WebSocket message structure for real-time communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    /// Type of the message
    pub r#type: String,
    /// Message data (JSON)
    pub data: serde_json::Value,
    /// Timestamp when the message was created
    pub timestamp: String,
}

/// WebSocket service for handling real-time connections.
///
/// This service manages WebSocket connections and provides real-time
/// updates for distributed coordination operations.
pub struct WebSocketService {
    _lock_manager: Arc<LockManager>,
    _saga_orchestrator: Arc<SagaOrchestrator>,
    _event_store: Arc<EventStore>,
    _cache_manager: Arc<CacheManager>,
    event_sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketService {
    /// Creates a new WebSocket service instance.
    ///
    /// # Arguments
    ///
    /// * `lock_manager` - Distributed lock manager
    /// * `saga_orchestrator` - Saga orchestration service
    /// * `event_store` - Event store for event sourcing
    /// * `cache_manager` - Cache manager for distributed caching
    ///
    /// # Returns
    ///
    /// Returns a new `WebSocketService` instance.
    pub fn new(
        lock_manager: LockManager,
        saga_orchestrator: SagaOrchestrator,
        event_store: EventStore,
        cache_manager: CacheManager,
    ) -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            _lock_manager: Arc::new(lock_manager),
            _saga_orchestrator: Arc::new(saga_orchestrator),
            _event_store: Arc::new(event_store),
            _cache_manager: Arc::new(cache_manager),
            event_sender,
        }
    }

    /// Handles WebSocket upgrade requests.
    ///
    /// This method upgrades HTTP connections to WebSocket and starts
    /// the WebSocket handler for real-time communication.
    ///
    /// # Arguments
    ///
    /// * `ws` - WebSocket upgrade request
    /// * `state` - WebSocket service state
    ///
    /// # Returns
    ///
    /// Returns a WebSocket response.
    pub async fn handle_websocket(
        ws: WebSocketUpgrade,
        State(state): State<Arc<Self>>,
    ) -> Response {
        ws.on_upgrade(|socket| handle_socket(socket, state))
    }

    /// Gets the event sender for broadcasting messages.
    ///
    /// This method returns a clone of the event sender that can be used
    /// to broadcast messages to all connected WebSocket clients.
    ///
    /// # Returns
    ///
    /// Returns a broadcast sender for WebSocket messages.
    pub fn get_event_sender(&self) -> broadcast::Sender<WebSocketMessage> {
        self.event_sender.clone()
    }
}

async fn handle_socket(socket: WebSocket, state: Arc<WebSocketService>) {
    let mut rx = state.event_sender.subscribe();

    let (mut sender, mut receiver) = socket.split();

    let welcome_msg = WebSocketMessage {
        r#type: "welcome".to_string(),
        data: serde_json::json!({
            "message": "Connected to Syros WebSocket",
            "version": env!("CARGO_PKG_VERSION")
        }),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    if let Ok(msg) = serde_json::to_string(&welcome_msg) {
        let _ = sender.send(Message::Text(msg)).await;
    }

    loop {
        tokio::select! {
            msg = receiver.next() => {
                if let Some(Ok(msg)) = msg {
                    match msg {
                        Message::Text(text) => {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Some(msg_type) = parsed.get("type").and_then(|v| v.as_str()) {
                                    match msg_type {
                                        "ping" => {
                                            let pong = WebSocketMessage {
                                                r#type: "pong".to_string(),
                                                data: serde_json::json!({"timestamp": chrono::Utc::now().to_rfc3339()}),
                                                timestamp: chrono::Utc::now().to_rfc3339(),
                                            };
                                            if let Ok(pong_msg) = serde_json::to_string(&pong) {
                                                let _ = sender.send(Message::Text(pong_msg)).await;
                                            }
                                        }
                                        "subscribe" => {
                                            let response = WebSocketMessage {
                                                r#type: "subscribed".to_string(),
                                                data: serde_json::json!({"message": "Inscrito para receber eventos"}),
                                                timestamp: chrono::Utc::now().to_rfc3339(),
                                            };
                                            if let Ok(response_msg) = serde_json::to_string(&response) {
                                                let _ = sender.send(Message::Text(response_msg)).await;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        Message::Close(_) => break,
                        _ => {}
                    }
                } else {
                    break;
                }
            }
            event_msg = rx.recv() => {
                if let Ok(msg) = event_msg {
                    if let Ok(msg_str) = serde_json::to_string(&msg) {
                        let _ = sender.send(Message::Text(msg_str)).await;
                    }
                }
            }
        }
    }
}
