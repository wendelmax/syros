# Syros Go SDK

Este SDK fornece uma interface Go para interagir com a Syros, incluindo as APIs REST e WebSocket.

## Instalação

```bash
go get github.com/syros/syros-sdk-go
```

## Uso

### Exemplo Básico

```go
package main

import (
    "fmt"
    "log"
    "github.com/syros/syros-sdk-go"
)

func main() {
    client := syros.NewSyrosClient()
    defer client.Close()
    
    // Health check
    health, err := client.HealthCheck()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Status: %v\n", health["status"])
    
    // Acquire lock
    ttlSeconds := int64(60)
    lock, err := client.AcquireLock("my_resource", "my_client", &ttlSeconds, nil)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Lock ID: %v\n", lock["lock_id"])
    
    // Start saga
    steps := []map[string]interface{}{
        {"name": "step1", "action": "do_something", "compensation": "undo_something"},
    }
    saga, err := client.StartSaga("my_saga", steps, nil)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Saga ID: %v\n", saga["saga_id"])
}
```

### WebSocket

```go
package main

import (
    "fmt"
    "log"
    "time"
    "github.com/syros/syros-sdk-go"
)

func main() {
    client := syros.NewSyrosClient()
    defer client.Close()
    
    err := client.ConnectWebSocket()
    if err != nil {
        log.Fatal(err)
    }
    
    // Listen for messages
    go func() {
        err := client.ListenWebSocket(func(message string) {
            fmt.Printf("Message: %s\n", message)
        })
        if err != nil {
            log.Printf("Listen error: %v", err)
        }
    }()
    
    // Send messages
    err = client.SendWebSocketMessage("ping", map[string]interface{}{
        "timestamp": time.Now().UnixMilli(),
    })
    if err != nil {
        log.Fatal(err)
    }
    
    time.Sleep(5 * time.Second)
    client.DisconnectWebSocket()
}
```

## API Reference

### REST API Methods

- `HealthCheck()` - Health check
- `AcquireLock(key, owner, ttlSeconds, metadata)` - Acquire lock
- `ReleaseLock(key, lockId, owner)` - Release lock
- `GetLockStatus(key)` - Get lock status
- `StartSaga(name, steps, metadata)` - Start saga
- `GetSagaStatus(sagaId)` - Get saga status
- `AppendEvent(streamId, eventType, data, metadata)` - Append event
- `GetEvents(streamId)` - Get events
- `SetCache(key, value, ttlSeconds, tags)` - Set cache
- `GetCache(key)` - Get cache
- `DeleteCache(key)` - Delete cache
- `GetMetrics()` - Get metrics

### WebSocket API Methods

- `ConnectWebSocket()` - Connect to WebSocket
- `SendWebSocketMessage(type, data)` - Send message
- `ListenWebSocket(onMessage)` - Listen for messages
- `DisconnectWebSocket()` - Disconnect

## Executando o Exemplo

```bash
cd sdks/go
go mod tidy
go run example.go syros_client.go
```

## Dependências

- Go 1.21+
- github.com/gorilla/websocket v1.5.1
