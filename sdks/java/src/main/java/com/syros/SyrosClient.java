package com.syros;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.JsonNode;
import okhttp3.*;
import okhttp3.ws.WebSocket;
import okhttp3.ws.WebSocketListener;
import okio.ByteString;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeUnit;

/**
 * Syros Java SDK
 * 
 * Provides a Java interface to interact with Syros,
 * including REST APIs and WebSocket communication.
 */
public class SyrosClient {
    private final OkHttpClient httpClient;
    private final ObjectMapper objectMapper;
    private final String restUrl;
    private final String wsUrl;
    private WebSocket webSocket;

    public SyrosClient() {
        this("http://localhost:8080", "ws://localhost:8080/ws");
    }

    public SyrosClient(String restUrl, String wsUrl) {
        this.restUrl = restUrl;
        this.wsUrl = wsUrl;
        this.objectMapper = new ObjectMapper();
        this.httpClient = new OkHttpClient.Builder()
                .connectTimeout(30, TimeUnit.SECONDS)
                .readTimeout(30, TimeUnit.SECONDS)
                .writeTimeout(30, TimeUnit.SECONDS)
                .build();
    }

    // --- REST API Methods ---

    /**
     * Health check endpoint
     */
    public JsonNode healthCheck() throws IOException {
        return sendRestRequest("GET", "/health", null);
    }

    /**
     * Acquire a distributed lock
     */
    public JsonNode acquireLock(String key, String owner, Long ttlSeconds, String metadata) throws IOException {
        Map<String, Object> payload = new HashMap<>();
        payload.put("key", key);
        payload.put("owner", owner);
        if (ttlSeconds != null) payload.put("ttl_seconds", ttlSeconds);
        if (metadata != null) payload.put("metadata", metadata);

        return sendRestRequest("POST", "/api/v1/locks", payload);
    }

    /**
     * Release a distributed lock
     */
    public JsonNode releaseLock(String key, String lockId, String owner) throws IOException {
        Map<String, Object> payload = new HashMap<>();
        payload.put("lock_id", lockId);
        payload.put("owner", owner);

        return sendRestRequest("DELETE", "/api/v1/locks/" + key, payload);
    }

    /**
     * Get lock status
     */
    public JsonNode getLockStatus(String key) throws IOException {
        return sendRestRequest("GET", "/api/v1/locks/" + key + "/status", null);
    }

    /**
     * Start a saga
     */
    public JsonNode startSaga(String name, Object[] steps, Map<String, String> metadata) throws IOException {
        Map<String, Object> payload = new HashMap<>();
        payload.put("name", name);
        payload.put("steps", steps);
        if (metadata != null) payload.put("metadata", metadata);

        return sendRestRequest("POST", "/api/v1/sagas", payload);
    }

    /**
     * Get saga status
     */
    public JsonNode getSagaStatus(String sagaId) throws IOException {
        return sendRestRequest("GET", "/api/v1/sagas/" + sagaId + "/status", null);
    }

    /**
     * Append event to event store
     */
    public JsonNode appendEvent(String streamId, String eventType, Object data, Map<String, String> metadata) throws IOException {
        Map<String, Object> payload = new HashMap<>();
        payload.put("stream_id", streamId);
        payload.put("event_type", eventType);
        payload.put("data", data);
        if (metadata != null) payload.put("metadata", metadata);

        return sendRestRequest("POST", "/api/v1/events", payload);
    }

    /**
     * Get events from event store
     */
    public JsonNode getEvents(String streamId) throws IOException {
        return sendRestRequest("GET", "/api/v1/events/" + streamId, null);
    }

    /**
     * Set cache value
     */
    public JsonNode setCache(String key, Object value, Long ttlSeconds, String[] tags) throws IOException {
        Map<String, Object> payload = new HashMap<>();
        payload.put("value", value);
        if (ttlSeconds != null) payload.put("ttl_seconds", ttlSeconds);
        if (tags != null) payload.put("tags", tags);

        return sendRestRequest("POST", "/api/v1/cache/" + key, payload);
    }

    /**
     * Get cache value
     */
    public JsonNode getCache(String key) throws IOException {
        return sendRestRequest("GET", "/api/v1/cache/" + key, null);
    }

    /**
     * Delete cache value
     */
    public JsonNode deleteCache(String key) throws IOException {
        return sendRestRequest("DELETE", "/api/v1/cache/" + key, null);
    }

    /**
     * Get metrics
     */
    public JsonNode getMetrics() throws IOException {
        return sendRestRequest("GET", "/metrics", null);
    }

    // --- WebSocket API Methods ---

    /**
     * Connect to WebSocket
     */
    public void connectWebSocket(WebSocketListener listener) {
        Request request = new Request.Builder()
                .url(wsUrl)
                .build();

        webSocket = httpClient.newWebSocket(request, listener);
    }

    /**
     * Send WebSocket message
     */
    public void sendWebSocketMessage(String type, Object data) throws IOException {
        if (webSocket != null) {
            Map<String, Object> message = new HashMap<>();
            message.put("type", type);
            message.put("data", data);
            
            String jsonMessage = objectMapper.writeValueAsString(message);
            webSocket.send(ByteString.encodeUtf8(jsonMessage));
        }
    }

    /**
     * Disconnect WebSocket
     */
    public void disconnectWebSocket() throws IOException {
        if (webSocket != null) {
            webSocket.close(1000, "Client disconnect");
            webSocket = null;
        }
    }

    // --- Private Helper Methods ---

    private JsonNode sendRestRequest(String method, String path, Object data) throws IOException {
        String url = restUrl + path;
        Request.Builder requestBuilder = new Request.Builder().url(url);

        if (data != null) {
            String jsonData = objectMapper.writeValueAsString(data);
            RequestBody body = RequestBody.create(jsonData, MediaType.get("application/json; charset=utf-8"));
            
            if ("POST".equals(method)) {
                requestBuilder.post(body);
            } else if ("DELETE".equals(method)) {
                requestBuilder.delete(body);
            } else if ("PUT".equals(method)) {
                requestBuilder.put(body);
            }
        } else {
            if ("DELETE".equals(method)) {
                requestBuilder.delete();
            }
        }

        Request request = requestBuilder.build();
        
        try (Response response = httpClient.newCall(request).execute()) {
            if (!response.isSuccessful()) {
                throw new IOException("HTTP " + response.code() + ": " + response.message());
            }
            
            String responseBody = response.body().string();
            return objectMapper.readTree(responseBody);
        }
    }

    /**
     * Close the client and release resources
     */
    public void close() throws IOException {
        disconnectWebSocket();
        httpClient.dispatcher().executorService().shutdown();
    }
}
