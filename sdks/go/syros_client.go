package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"

	"github.com/gorilla/websocket"
)

// SyrosClient provides a Go interface to interact with Syros
type SyrosClient struct {
	httpClient *http.Client
	restURL    string
	wsURL      string
	wsConn     *websocket.Conn
}

// NewSyrosClient creates a new Syros client with default settings
func NewSyrosClient() *SyrosClient {
	return NewSyrosClientWithURLs("http://localhost:8080", "ws://localhost:8080/ws")
}

// NewSyrosClientWithURLs creates a new Syros client with custom URLs
func NewSyrosClientWithURLs(restURL, wsURL string) *SyrosClient {
	return &SyrosClient{
		httpClient: &http.Client{
			Timeout: 30 * time.Second,
		},
		restURL: restURL,
		wsURL:   wsURL,
	}
}

// HealthCheck performs a health check
func (c *SyrosClient) HealthCheck() (map[string]interface{}, error) {
	return c.sendRestRequest("GET", "/health", nil)
}

// AcquireLock acquires a distributed lock
func (c *SyrosClient) AcquireLock(key, owner string, ttlSeconds *int64, metadata *string) (map[string]interface{}, error) {
	payload := map[string]interface{}{
		"key":   key,
		"owner": owner,
	}
	if ttlSeconds != nil {
		payload["ttl_seconds"] = *ttlSeconds
	}
	if metadata != nil {
		payload["metadata"] = *metadata
	}

	return c.sendRestRequest("POST", "/api/v1/locks", payload)
}

// ReleaseLock releases a distributed lock
func (c *SyrosClient) ReleaseLock(key, lockID, owner string) (map[string]interface{}, error) {
	payload := map[string]interface{}{
		"lock_id": lockID,
		"owner":   owner,
	}

	return c.sendRestRequest("DELETE", "/api/v1/locks/"+key, payload)
}

// GetLockStatus gets the status of a lock
func (c *SyrosClient) GetLockStatus(key string) (map[string]interface{}, error) {
	return c.sendRestRequest("GET", "/api/v1/locks/"+key+"/status", nil)
}

// StartSaga starts a new saga
func (c *SyrosClient) StartSaga(name string, steps []map[string]interface{}, metadata map[string]string) (map[string]interface{}, error) {
	payload := map[string]interface{}{
		"name":  name,
		"steps": steps,
	}
	if metadata != nil {
		payload["metadata"] = metadata
	}

	return c.sendRestRequest("POST", "/api/v1/sagas", payload)
}

// GetSagaStatus gets the status of a saga
func (c *SyrosClient) GetSagaStatus(sagaID string) (map[string]interface{}, error) {
	return c.sendRestRequest("GET", "/api/v1/sagas/"+sagaID+"/status", nil)
}

// AppendEvent appends an event to the event store
func (c *SyrosClient) AppendEvent(streamID, eventType string, data interface{}, metadata map[string]string) (map[string]interface{}, error) {
	payload := map[string]interface{}{
		"stream_id":  streamID,
		"event_type": eventType,
		"data":       data,
	}
	if metadata != nil {
		payload["metadata"] = metadata
	}

	return c.sendRestRequest("POST", "/api/v1/events", payload)
}

// GetEvents gets events from the event store
func (c *SyrosClient) GetEvents(streamID string) (map[string]interface{}, error) {
	return c.sendRestRequest("GET", "/api/v1/events/"+streamID, nil)
}

// SetCache sets a value in the cache
func (c *SyrosClient) SetCache(key string, value interface{}, ttlSeconds *int64, tags []string) (map[string]interface{}, error) {
	payload := map[string]interface{}{
		"value": value,
	}
	if ttlSeconds != nil {
		payload["ttl_seconds"] = *ttlSeconds
	}
	if tags != nil {
		payload["tags"] = tags
	}

	return c.sendRestRequest("POST", "/api/v1/cache/"+key, payload)
}

// GetCache gets a value from the cache
func (c *SyrosClient) GetCache(key string) (map[string]interface{}, error) {
	return c.sendRestRequest("GET", "/api/v1/cache/"+key, nil)
}

// DeleteCache deletes a value from the cache
func (c *SyrosClient) DeleteCache(key string) (map[string]interface{}, error) {
	return c.sendRestRequest("DELETE", "/api/v1/cache/"+key, nil)
}

// GetMetrics gets Prometheus metrics
func (c *SyrosClient) GetMetrics() (string, error) {
	resp, err := c.sendRestRequestRaw("GET", "/metrics", nil)
	if err != nil {
		return "", err
	}
	return resp, nil
}

// ConnectWebSocket connects to the WebSocket
func (c *SyrosClient) ConnectWebSocket() error {
	u, err := url.Parse(c.wsURL)
	if err != nil {
		return err
	}

	c.wsConn, _, err = websocket.DefaultDialer.Dial(u.String(), nil)
	if err != nil {
		return err
	}

	fmt.Println("WebSocket connected to", c.wsURL)
	return nil
}

// SendWebSocketMessage sends a message through WebSocket
func (c *SyrosClient) SendWebSocketMessage(messageType string, data interface{}) error {
	if c.wsConn == nil {
		return fmt.Errorf("WebSocket not connected")
	}

	message := map[string]interface{}{
		"type": messageType,
		"data": data,
	}

	return c.wsConn.WriteJSON(message)
}

// ListenWebSocket listens for WebSocket messages
func (c *SyrosClient) ListenWebSocket(onMessage func(string)) error {
	if c.wsConn == nil {
		return fmt.Errorf("WebSocket not connected")
	}

	for {
		_, message, err := c.wsConn.ReadMessage()
		if err != nil {
			return err
		}
		onMessage(string(message))
	}
}

// DisconnectWebSocket disconnects from WebSocket
func (c *SyrosClient) DisconnectWebSocket() error {
	if c.wsConn != nil {
		err := c.wsConn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""))
		if err != nil {
			return err
		}
		c.wsConn.Close()
		c.wsConn = nil
		fmt.Println("WebSocket disconnected")
	}
	return nil
}

// Close closes the client and releases resources
func (c *SyrosClient) Close() error {
	c.DisconnectWebSocket()
	return nil
}

// Private helper methods

func (c *SyrosClient) sendRestRequest(method, path string, data interface{}) (map[string]interface{}, error) {
	resp, err := c.sendRestRequestRaw(method, path, data)
	if err != nil {
		return nil, err
	}

	var result map[string]interface{}
	if err := json.Unmarshal([]byte(resp), &result); err != nil {
		return nil, err
	}

	return result, nil
}

func (c *SyrosClient) sendRestRequestRaw(method, path string, data interface{}) (string, error) {
	url := c.restURL + path
	var body io.Reader

	if data != nil {
		jsonData, err := json.Marshal(data)
		if err != nil {
			return "", err
		}
		body = bytes.NewBuffer(jsonData)
	}

	req, err := http.NewRequestWithContext(context.Background(), method, url, body)
	if err != nil {
		return "", err
	}

	if data != nil {
		req.Header.Set("Content-Type", "application/json")
	}

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return "", fmt.Errorf("HTTP %d: %s", resp.StatusCode, resp.Status)
	}

	responseBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	return string(responseBody), nil
}
