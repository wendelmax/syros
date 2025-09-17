# REST API

A REST API da Syros Platform oferece uma interface HTTP simples e intuitiva para todas as funcionalidades da plataforma.

## 🔗 Endpoints Base

- **Base URL**: `http://localhost:8080`
- **Content-Type**: `application/json`
- **Autenticação**: Bearer Token (JWT) ou API Key

## Autenticação

### Obter Token JWT

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password"
  }'
```

**Resposta:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "token_type": "Bearer"
}
```

### Usar Token

```bash
export TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/api/v1/locks
```

### API Key

```bash
curl -H "X-API-Key: your-api-key" \
  http://localhost:8080/api/v1/locks
```

## Gerenciamento de Locks

### Adquirir Lock

```bash
curl -X POST http://localhost:8080/api/v1/locks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "key": "resource-123",
    "ttl": 300,
    "owner": "service-a"
  }'
```

**Resposta:**
```json
{
  "lock_id": "lock-uuid-123",
  "key": "resource-123",
  "acquired": true,
  "expires_at": "2025-09-19T15:30:00Z",
  "owner": "service-a"
}
```

### Verificar Status do Lock

```bash
curl -X GET http://localhost:8080/api/v1/locks/resource-123/status \
  -H "Authorization: Bearer $TOKEN"
```

**Resposta:**
```json
{
  "lock_id": "lock-uuid-123",
  "key": "resource-123",
  "acquired": true,
  "expires_at": "2025-09-19T15:30:00Z",
  "owner": "service-a",
  "created_at": "2025-09-19T15:25:00Z"
}
```

### Liberar Lock

```bash
curl -X DELETE http://localhost:8080/api/v1/locks/resource-123 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"lock_id": "lock-uuid-123"}'
```

**Resposta:**
```json
{
  "lock_id": "lock-uuid-123",
  "key": "resource-123",
  "released": true
}
```

### Estender Lock

```bash
curl -X PUT http://localhost:8080/api/v1/locks/resource-123/extend \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "lock_id": "lock-uuid-123",
    "ttl": 600
  }'
```

### Listar Locks

```bash
curl -X GET "http://localhost:8080/api/v1/locks?owner=service-a&limit=10&offset=0" \
  -H "Authorization: Bearer $TOKEN"
```

## Orquestração de Sagas

### Iniciar Saga

```bash
curl -X POST http://localhost:8080/api/v1/sagas \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "order-processing",
    "steps": [
      {
        "id": "validate-order",
        "action": "validate",
        "timeout": 30,
        "retry_policy": {
          "max_retries": 3,
          "backoff_strategy": "exponential"
        }
      },
      {
        "id": "process-payment",
        "action": "payment",
        "timeout": 60,
        "retry_policy": {
          "max_retries": 2,
          "backoff_strategy": "linear"
        }
      }
    ]
  }'
```

**Resposta:**
```json
{
  "saga_id": "saga-uuid-456",
  "name": "order-processing",
  "status": "started",
  "current_step": "validate-order",
  "created_at": "2025-09-19T10:00:00Z"
}
```

### Verificar Status da Saga

```bash
curl -X GET http://localhost:8080/api/v1/sagas/saga-uuid-456/status \
  -H "Authorization: Bearer $TOKEN"
```

**Resposta:**
```json
{
  "saga_id": "saga-uuid-456",
  "name": "order-processing",
  "status": "running",
  "current_step": "process-payment",
  "steps": [
    {
      "id": "validate-order",
      "action": "validate",
      "status": "completed",
      "completed_at": "2025-09-19T10:01:00Z"
    },
    {
      "id": "process-payment",
      "action": "payment",
      "status": "running",
      "started_at": "2025-09-19T10:01:00Z"
    }
  ]
}
```

### Executar Próximo Passo

```bash
curl -X POST http://localhost:8080/api/v1/sagas/saga-uuid-456/execute \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "step_id": "validate-order",
    "data": {"order_id": "123"}
  }'
```

### Cancelar Saga

```bash
curl -X POST http://localhost:8080/api/v1/sagas/saga-uuid-456/cancel \
  -H "Authorization: Bearer $TOKEN"
```

### Listar Sagas

```bash
curl -X GET "http://localhost:8080/api/v1/sagas?status=running&limit=10" \
  -H "Authorization: Bearer $TOKEN"
```

## Event Store

### Adicionar Evento

```bash
curl -X POST http://localhost:8080/api/v1/events \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "stream_id": "user-123",
    "event_type": "user_created",
    "data": {
      "user_id": "123",
      "email": "user@example.com",
      "created_at": "2025-09-19T10:00:00Z"
    },
    "metadata": {
      "source": "user-service",
      "version": "1.0"
    }
  }'
```

**Resposta:**
```json
{
  "event_id": "event-uuid-789",
  "stream_id": "user-123",
  "event_type": "user_created",
  "sequence_number": 1,
  "created_at": "2025-09-19T10:00:00Z"
}
```

### Buscar Eventos

```bash
# Buscar todos os eventos de um stream
curl -X GET "http://localhost:8080/api/v1/events/user-123?limit=10&offset=0" \
  -H "Authorization: Bearer $TOKEN"

# Buscar eventos por tipo
curl -X GET "http://localhost:8080/api/v1/events/user-123?event_type=user_created" \
  -H "Authorization: Bearer $TOKEN"

# Buscar eventos por data
curl -X GET "http://localhost:8080/api/v1/events/user-123?from=2025-09-19T00:00:00Z&to=2025-09-19T23:59:59Z" \
  -H "Authorization: Bearer $TOKEN"
```

**Resposta:**
```json
{
  "events": [
    {
      "event_id": "event-uuid-789",
      "stream_id": "user-123",
      "event_type": "user_created",
      "data": {
        "user_id": "123",
        "email": "user@example.com",
        "created_at": "2025-09-19T10:00:00Z"
      },
      "metadata": {
        "source": "user-service",
        "version": "1.0"
      },
      "sequence_number": 1,
      "created_at": "2025-09-19T10:00:00Z"
    }
  ],
  "total": 1,
  "has_more": false
}
```

### Informações do Stream

```bash
curl -X GET http://localhost:8080/api/v1/events/user-123/info \
  -H "Authorization: Bearer $TOKEN"
```

## Cache Distribuído

### Armazenar no Cache

```bash
curl -X POST http://localhost:8080/api/v1/cache \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "key": "user-profile-123",
    "value": {
      "name": "João Silva",
      "email": "joao@example.com",
      "preferences": {"theme": "dark"}
    },
    "ttl": 3600,
    "tags": ["user", "profile"]
  }'
```

**Resposta:**
```json
{
  "key": "user-profile-123",
  "stored": true,
  "expires_at": "2025-09-19T11:00:00Z"
}
```

### Recuperar do Cache

```bash
curl -X GET http://localhost:8080/api/v1/cache/user-profile-123 \
  -H "Authorization: Bearer $TOKEN"
```

**Resposta:**
```json
{
  "key": "user-profile-123",
  "value": {
    "name": "João Silva",
    "email": "joao@example.com",
    "preferences": {"theme": "dark"}
  },
  "expires_at": "2025-09-19T11:00:00Z",
  "created_at": "2025-09-19T10:00:00Z"
}
```

### Deletar do Cache

```bash
curl -X DELETE http://localhost:8080/api/v1/cache/user-profile-123 \
  -H "Authorization: Bearer $TOKEN"
```

**Resposta:**
```json
{
  "key": "user-profile-123",
  "deleted": true
}
```

### Listar Cache por Tags

```bash
curl -X GET "http://localhost:8080/api/v1/cache?tags=user&limit=10" \
  -H "Authorization: Bearer $TOKEN"
```

## Health Checks

### Health Básico

```bash
curl http://localhost:8080/health
```

**Resposta:**
```json
{
  "status": "healthy",
  "timestamp": "2025-09-19T10:00:00Z",
  "version": "1.0.0"
}
```

### Health Detalhado

```bash
curl http://localhost:8080/ready
```

**Resposta:**
```json
{
  "status": "ready",
  "timestamp": "2025-09-19T10:00:00Z",
  "services": {
    "redis": "healthy",
    "postgres": "healthy",
    "grpc": "healthy",
    "websocket": "healthy"
  }
}
```

### Health para Kubernetes

```bash
curl http://localhost:8080/live
```

## Métricas

### Endpoint de Métricas

```bash
curl http://localhost:8080/metrics
```

**Exemplo de métricas:**
```
# HELP syros_http_requests_total Total HTTP requests
# TYPE syros_http_requests_total counter
syros_http_requests_total{method="POST",endpoint="/api/v1/locks",status="200"} 42

# HELP syros_locks_acquired_total Total locks acquired
# TYPE syros_locks_acquired_total counter
syros_locks_acquired_total{owner="service-a"} 15

# HELP syros_cache_hits_total Total cache hits
# TYPE syros_cache_hits_total counter
syros_cache_hits_total{key="user-profile-123"} 25
```

## Códigos de Erro

### 400 Bad Request
```json
{
  "error": "Bad Request",
  "message": "Invalid request body",
  "code": "INVALID_REQUEST"
}
```

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing token",
  "code": "UNAUTHORIZED"
}
```

### 403 Forbidden
```json
{
  "error": "Forbidden",
  "message": "Insufficient permissions",
  "code": "FORBIDDEN"
}
```

### 404 Not Found
```json
{
  "error": "Not Found",
  "message": "Resource not found",
  "code": "NOT_FOUND"
}
```

### 409 Conflict
```json
{
  "error": "Conflict",
  "message": "Lock already acquired by another owner",
  "code": "LOCK_CONFLICT"
}
```

### 429 Too Many Requests
```json
{
  "error": "Too Many Requests",
  "message": "Rate limit exceeded",
  "code": "RATE_LIMIT_EXCEEDED"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal Server Error",
  "message": "An unexpected error occurred",
  "code": "INTERNAL_ERROR"
}
```

## Headers Personalizados

### Rate Limiting

```bash
# Headers retornados para rate limiting
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1634567890
```

### Paginação

```bash
# Headers para paginação
X-Total-Count: 150
X-Page-Size: 10
X-Page-Number: 1
X-Has-More: true
```

### CORS

```bash
# Headers CORS
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization, X-API-Key
```

## Exemplos Práticos

### Exemplo Completo: Processamento de Pedido

```bash
#!/bin/bash

# 1. Autenticar
TOKEN=$(curl -s -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}' | \
  jq -r '.token')

# 2. Adquirir lock para o pedido
LOCK_RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/locks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "key": "order-123",
    "ttl": 300,
    "owner": "order-service"
  }')

LOCK_ID=$(echo $LOCK_RESPONSE | jq -r '.lock_id')

if [ "$(echo $LOCK_RESPONSE | jq -r '.acquired')" = "true" ]; then
  echo "Lock adquirido: $LOCK_ID"
  
  # 3. Processar pedido
  echo "Processando pedido..."
  sleep 2
  
  # 4. Liberar lock
  curl -s -X DELETE http://localhost:8080/api/v1/locks/order-123 \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"lock_id\": \"$LOCK_ID\"}"
  
  echo "Lock liberado"
else
  echo "Falha ao adquirir lock"
fi
```

---

**Próximo**: [gRPC API](grpc-api.md) | [WebSocket API](websocket-api.md) | [SDKs](sdks.md)
