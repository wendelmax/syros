# Syros Java SDK

Este SDK fornece uma interface Java para interagir com a Syros, incluindo as APIs REST e WebSocket.

## Instalação

### Maven

Adicione a dependência ao seu `pom.xml`:

```xml
<dependency>
    <groupId>com.syros</groupId>
    <artifactId>syros-sdk</artifactId>
    <version>1.0.0</version>
</dependency>
```

### Gradle

```gradle
implementation 'com.syros:syros-sdk:1.0.0'
```

## Uso

### Exemplo Básico

```java
import com.syros.SyrosClient;
import com.fasterxml.jackson.databind.JsonNode;

public class MyApp {
    public static void main(String[] args) throws IOException {
        SyrosClient client = new SyrosClient();
        
        // Health check
        JsonNode health = client.healthCheck();
        System.out.println("Status: " + health.get("status"));
        
        // Acquire lock
        JsonNode lock = client.acquireLock("my_resource", "my_client", 60L, null);
        System.out.println("Lock ID: " + lock.get("lock_id"));
        
        // Start saga
        Object[] steps = {
            Map.of("name", "step1", "action", "do_something", "compensation", "undo_something")
        };
        JsonNode saga = client.startSaga("my_saga", steps, null);
        System.out.println("Saga ID: " + saga.get("saga_id"));
        
        client.close();
    }
}
```

### WebSocket

```java
import okhttp3.ws.WebSocketListener;
import okio.ByteString;

WebSocketListener listener = new WebSocketListener() {
    @Override
    public void onOpen(okhttp3.ws.WebSocket webSocket, okhttp3.Response response) {
        System.out.println("Connected!");
    }
    
    @Override
    public void onMessage(okhttp3.ResponseBody message) throws IOException {
        System.out.println("Message: " + message.string());
    }
    
    @Override
    public void onClose(int code, String reason) {
        System.out.println("Disconnected: " + reason);
    }
    
    @Override
    public void onFailure(IOException e, okhttp3.Response response) {
        System.err.println("Error: " + e.getMessage());
    }
    
    @Override
    public void onPong(ByteString payload) {
        // Handle pong
    }
};

client.connectWebSocket(listener);
client.sendWebSocketMessage("ping", Map.of("timestamp", System.currentTimeMillis()));
```

## API Reference

### REST API Methods

- `healthCheck()` - Health check
- `acquireLock(key, owner, ttlSeconds, metadata)` - Acquire lock
- `releaseLock(key, lockId, owner)` - Release lock
- `getLockStatus(key)` - Get lock status
- `startSaga(name, steps, metadata)` - Start saga
- `getSagaStatus(sagaId)` - Get saga status
- `appendEvent(streamId, eventType, data, metadata)` - Append event
- `getEvents(streamId)` - Get events
- `setCache(key, value, ttlSeconds, tags)` - Set cache
- `getCache(key)` - Get cache
- `deleteCache(key)` - Delete cache
- `getMetrics()` - Get metrics

### WebSocket API Methods

- `connectWebSocket(listener)` - Connect to WebSocket
- `sendWebSocketMessage(type, data)` - Send message
- `disconnectWebSocket()` - Disconnect

## Executando o Exemplo

```bash
cd sdks/java
mvn compile exec:java -Dexec.mainClass="com.syros.Example"
```

## Dependências

- Java 11+
- OkHttp 4.12.0
- Jackson 2.16.1
- SLF4J 2.0.9
