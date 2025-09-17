"""
Syros Platform Python SDK
SDK oficial para integração com a Syros Platform
"""

import asyncio
import json
import time
from typing import Optional, Dict, Any, List
import aiohttp
import websockets
from dataclasses import dataclass
from datetime import datetime


@dataclass
class LockRequest:
    key: str
    owner: str
    ttl_seconds: int = 300
    metadata: Optional[str] = None
    wait_timeout_seconds: Optional[int] = None


@dataclass
class LockResponse:
    lock_id: str
    success: bool
    message: str


@dataclass
class SagaStep:
    name: str
    action: str
    compensation: str
    timeout_seconds: Optional[int] = None
    retry_policy: Optional[Dict[str, Any]] = None
    payload: Optional[Dict[str, Any]] = None


@dataclass
class SagaRequest:
    name: str
    steps: List[SagaStep]
    metadata: Optional[Dict[str, str]] = None


@dataclass
class SagaResponse:
    saga_id: str
    status: str
    message: str


@dataclass
class EventRequest:
    stream_id: str
    event_type: str
    data: Dict[str, Any]
    metadata: Optional[Dict[str, str]] = None


@dataclass
class EventResponse:
    event_id: str
    version: int
    success: bool
    message: str


@dataclass
class CacheRequest:
    key: str
    value: Dict[str, Any]
    ttl_seconds: Optional[int] = None
    tags: Optional[List[str]] = None


@dataclass
class CacheResponse:
    key: str
    value: Dict[str, Any]
    expires_at: Optional[str] = None
    tags: List[str]
    success: bool
    message: str


class SyrosClient:
    """Cliente principal para a Syros Platform"""
    
    def __init__(self, endpoint: str = "http://localhost:8080", api_key: Optional[str] = None):
        self.endpoint = endpoint.rstrip('/')
        self.api_key = api_key
        self.session: Optional[aiohttp.ClientSession] = None
        
    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self
        
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
    
    def _get_headers(self) -> Dict[str, str]:
        headers = {"Content-Type": "application/json"}
        if self.api_key:
            headers["Authorization"] = f"Bearer {self.api_key}"
        return headers
    
    async def health_check(self) -> Dict[str, Any]:
        """Verifica a saúde da plataforma"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        async with self.session.get(f"{self.endpoint}/health") as response:
            return await response.json()
    
    async def acquire_lock(self, request: LockRequest) -> LockResponse:
        """Adquire um lock distribuído"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        data = {
            "key": request.key,
            "owner": request.owner,
            "ttl_seconds": request.ttl_seconds,
            "metadata": request.metadata,
            "wait_timeout_seconds": request.wait_timeout_seconds,
        }
        
        async with self.session.post(
            f"{self.endpoint}/api/v1/locks",
            json=data,
            headers=self._get_headers()
        ) as response:
            result = await response.json()
            return LockResponse(
                lock_id=result.get("lock_id", ""),
                success=result.get("success", False),
                message=result.get("message", "")
            )
    
    async def release_lock(self, key: str) -> Dict[str, Any]:
        """Libera um lock distribuído"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        async with self.session.delete(
            f"{self.endpoint}/api/v1/locks/{key}",
            headers=self._get_headers()
        ) as response:
            return await response.json()
    
    async def get_lock_status(self, key: str) -> Dict[str, Any]:
        """Obtém o status de um lock"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        async with self.session.get(
            f"{self.endpoint}/api/v1/locks/{key}/status",
            headers=self._get_headers()
        ) as response:
            return await response.json()
    
    async def start_saga(self, request: SagaRequest) -> SagaResponse:
        """Inicia uma saga"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        data = {
            "name": request.name,
            "steps": [
                {
                    "name": step.name,
                    "action": step.action,
                    "compensation": step.compensation,
                    "timeout_seconds": step.timeout_seconds,
                    "retry_policy": step.retry_policy,
                    "payload": step.payload,
                }
                for step in request.steps
            ],
            "metadata": request.metadata,
        }
        
        async with self.session.post(
            f"{self.endpoint}/api/v1/sagas",
            json=data,
            headers=self._get_headers()
        ) as response:
            result = await response.json()
            return SagaResponse(
                saga_id=result.get("saga_id", ""),
                status=result.get("status", ""),
                message=result.get("message", "")
            )
    
    async def get_saga_status(self, saga_id: str) -> Dict[str, Any]:
        """Obtém o status de uma saga"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        async with self.session.get(
            f"{self.endpoint}/api/v1/sagas/{saga_id}/status",
            headers=self._get_headers()
        ) as response:
            return await response.json()
    
    async def append_event(self, request: EventRequest) -> EventResponse:
        """Adiciona um evento ao event store"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        data = {
            "stream_id": request.stream_id,
            "event_type": request.event_type,
            "data": request.data,
            "metadata": request.metadata,
        }
        
        async with self.session.post(
            f"{self.endpoint}/api/v1/events",
            json=data,
            headers=self._get_headers()
        ) as response:
            result = await response.json()
            return EventResponse(
                event_id=result.get("event_id", ""),
                version=result.get("version", 0),
                success=result.get("success", False),
                message=result.get("message", "")
            )
    
    async def get_events(self, stream_id: str, from_version: Optional[int] = None) -> Dict[str, Any]:
        """Obtém eventos de um stream"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        url = f"{self.endpoint}/api/v1/events/{stream_id}"
        if from_version is not None:
            url += f"?from_version={from_version}"
            
        async with self.session.get(url, headers=self._get_headers()) as response:
            return await response.json()
    
    async def set_cache(self, request: CacheRequest) -> CacheResponse:
        """Define um valor no cache"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        data = {
            "value": request.value,
            "ttl_seconds": request.ttl_seconds,
            "tags": request.tags or [],
        }
        
        async with self.session.post(
            f"{self.endpoint}/api/v1/cache/{request.key}",
            json=data,
            headers=self._get_headers()
        ) as response:
            result = await response.json()
            return CacheResponse(
                key=result.get("key", request.key),
                value=result.get("value", {}),
                expires_at=result.get("expires_at"),
                tags=result.get("tags", []),
                success=result.get("success", False),
                message=result.get("message", "")
            )
    
    async def get_cache(self, key: str) -> Dict[str, Any]:
        """Obtém um valor do cache"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        async with self.session.get(
            f"{self.endpoint}/api/v1/cache/{key}",
            headers=self._get_headers()
        ) as response:
            return await response.json()
    
    async def delete_cache(self, key: str) -> Dict[str, Any]:
        """Remove um valor do cache"""
        if not self.session:
            raise RuntimeError("Cliente não inicializado. Use 'async with SyrosClient()'")
            
        async with self.session.delete(
            f"{self.endpoint}/api/v1/cache/{key}",
            headers=self._get_headers()
        ) as response:
            return await response.json()


class SyrosWebSocketClient:
    """Cliente WebSocket para eventos em tempo real"""
    
    def __init__(self, endpoint: str = "ws://localhost:8080/ws"):
        self.endpoint = endpoint
        self.websocket = None
        
    async def connect(self):
        """Conecta ao WebSocket"""
        self.websocket = await websockets.connect(self.endpoint)
        
    async def disconnect(self):
        """Desconecta do WebSocket"""
        if self.websocket:
            await self.websocket.close()
            
    async def send_ping(self):
        """Envia um ping"""
        if not self.websocket:
            raise RuntimeError("WebSocket não conectado")
            
        await self.websocket.send(json.dumps({"type": "ping"}))
        
    async def subscribe(self):
        """Inscreve-se para receber eventos"""
        if not self.websocket:
            raise RuntimeError("WebSocket não conectado")
            
        await self.websocket.send(json.dumps({"type": "subscribe"}))
        
    async def listen_for_events(self, callback):
        """Escuta eventos e chama callback para cada evento"""
        if not self.websocket:
            raise RuntimeError("WebSocket não conectado")
            
        async for message in self.websocket:
            try:
                data = json.loads(message)
                await callback(data)
            except json.JSONDecodeError:
                print(f"Erro ao decodificar mensagem: {message}")


# Exemplo de uso
async def exemplo_uso():
    """Exemplo de como usar o SDK"""
    
    # Cliente REST
    async with SyrosClient("http://localhost:8080") as client:
        # Verificar saúde
        health = await client.health_check()
        print(f"Status da plataforma: {health}")
        
        # Adquirir lock
        lock_request = LockRequest(
            key="meu-recurso",
            owner="cliente-python",
            ttl_seconds=60
        )
        
        lock_response = await client.acquire_lock(lock_request)
        print(f"Lock adquirido: {lock_response}")
        
        # Liberar lock
        release_result = await client.release_lock("meu-recurso")
        print(f"Lock liberado: {release_result}")
        
        # Usar cache
        cache_request = CacheRequest(
            key="meu-cache",
            value={"dados": "importantes"},
            ttl_seconds=300
        )
        
        cache_response = await client.set_cache(cache_request)
        print(f"Cache definido: {cache_response}")
        
        # Obter cache
        cache_data = await client.get_cache("meu-cache")
        print(f"Cache obtido: {cache_data}")
    
    # Cliente WebSocket
    ws_client = SyrosWebSocketClient("ws://localhost:8080/ws")
    await ws_client.connect()
    
    async def handle_event(event):
        print(f"Evento recebido: {event}")
    
    await ws_client.subscribe()
    await ws_client.listen_for_events(handle_event)


if __name__ == "__main__":
    asyncio.run(exemplo_uso())
