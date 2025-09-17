# Syros Python SDK

SDK oficial para integração com a Syros em Python.

## Instalação

```bash
pip install -r requirements.txt
```

## Uso Básico

### Cliente REST

```python
import asyncio
from syros_sdk import SyrosClient, LockRequest, CacheRequest

async def exemplo():
    async with SyrosClient("http://localhost:8080") as client:
        # Verificar saúde da plataforma
        health = await client.health_check()
        print(f"Status: {health}")
        
        # Adquirir lock
        lock_request = LockRequest(
            key="meu-recurso",
            owner="cliente-python",
            ttl_seconds=60
        )
        
        lock_response = await client.acquire_lock(lock_request)
        print(f"Lock: {lock_response}")
        
        # Usar cache
        cache_request = CacheRequest(
            key="dados",
            value={"info": "importante"},
            ttl_seconds=300
        )
        
        cache_response = await client.set_cache(cache_request)
        print(f"Cache: {cache_response}")

asyncio.run(exemplo())
```

### Cliente WebSocket

```python
import asyncio
from syros_sdk import SyrosWebSocketClient

async def exemplo_websocket():
    ws_client = SyrosWebSocketClient("ws://localhost:8080/ws")
    await ws_client.connect()
    
    async def handle_event(event):
        print(f"Evento: {event}")
    
    await ws_client.subscribe()
    await ws_client.listen_for_events(handle_event)

asyncio.run(exemplo_websocket())
```

## Funcionalidades

- **Locks Distribuídos**: Adquirir e liberar locks
- **Sagas**: Orquestração de transações distribuídas
- **Event Store**: Armazenamento de eventos
- **Cache**: Cache distribuído com TTL
- **WebSocket**: Eventos em tempo real
- **Health Checks**: Verificação de saúde da plataforma

## Exemplos

Veja o arquivo `syros_sdk.py` para exemplos completos de uso.
