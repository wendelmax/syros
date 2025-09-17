# Syros C# SDK

Este SDK fornece uma interface C# para interagir com a Syros, incluindo as APIs REST e WebSocket.

## Instalação

### NuGet Package Manager

```bash
Install-Package SyrosSDK
```

### .NET CLI

```bash
dotnet add package SyrosSDK
```

### PackageReference

```xml
<PackageReference Include="SyrosSDK" Version="1.0.0" />
```

## Uso

### Exemplo Básico

```csharp
using SyrosSDK;
using Newtonsoft.Json.Linq;

class Program
{
    static async Task Main(string[] args)
    {
        using var client = new SyrosClient();
        
        // Health check
        var health = await client.HealthCheckAsync();
        Console.WriteLine($"Status: {health["status"]}");
        
        // Acquire lock
        var lock = await client.AcquireLockAsync("my_resource", "my_client", 60);
        Console.WriteLine($"Lock ID: {lock["lock_id"]}");
        
        // Start saga
        var steps = new object[]
        {
            new { name = "step1", action = "do_something", compensation = "undo_something" }
        };
        var saga = await client.StartSagaAsync("my_saga", steps, null);
        Console.WriteLine($"Saga ID: {saga["saga_id"]}");
    }
}
```

### WebSocket

```csharp
using var client = new SyrosClient();

await client.ConnectWebSocketAsync();

// Listen for messages
var listenTask = Task.Run(async () =>
{
    await client.ListenWebSocketAsync(async message =>
    {
        Console.WriteLine($"Message: {message}");
    });
});

// Send messages
await client.SendWebSocketMessageAsync("ping", new { timestamp = DateTimeOffset.Now.ToUnixTimeMilliseconds() });
await client.SendWebSocketMessageAsync("subscribe", new { topic = "locks" });

await client.DisconnectWebSocketAsync();
```

## API Reference

### REST API Methods

- `HealthCheckAsync()` - Health check
- `AcquireLockAsync(key, owner, ttlSeconds, metadata)` - Acquire lock
- `ReleaseLockAsync(key, lockId, owner)` - Release lock
- `GetLockStatusAsync(key)` - Get lock status
- `StartSagaAsync(name, steps, metadata)` - Start saga
- `GetSagaStatusAsync(sagaId)` - Get saga status
- `AppendEventAsync(streamId, eventType, data, metadata)` - Append event
- `GetEventsAsync(streamId)` - Get events
- `SetCacheAsync(key, value, ttlSeconds, tags)` - Set cache
- `GetCacheAsync(key)` - Get cache
- `DeleteCacheAsync(key)` - Delete cache
- `GetMetricsAsync()` - Get metrics

### WebSocket API Methods

- `ConnectWebSocketAsync()` - Connect to WebSocket
- `SendWebSocketMessageAsync(type, data)` - Send message
- `ListenWebSocketAsync(onMessage)` - Listen for messages
- `DisconnectWebSocketAsync()` - Disconnect

## Executando o Exemplo

```bash
cd sdks/csharp/SyrosSDK
dotnet run
```

## Dependências

- .NET 6.0+
- Newtonsoft.Json 13.0.3
- System.Net.Http 4.3.4
- System.Net.WebSockets.Client 4.3.2
- Microsoft.Extensions.Logging 7.0.0
