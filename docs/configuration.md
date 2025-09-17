# Configuração

Este guia detalha todas as opções de configuração disponíveis na Syros Platform.

## Estrutura de Configuração

### Arquivo Principal

O arquivo de configuração principal está em `config/default.toml`:

```toml
[server]
port = 8080
grpc_port = 9090
websocket_port = 8081
host = "127.0.0.1"

[storage.redis]
url = "redis://localhost:6379"
pool_size = 10
timeout_seconds = 30

[storage.database]
url = "postgres://localhost:5432/syros"
pool_size = 10
timeout_seconds = 30

[security]
jwt_secret = "your-jwt-secret"
api_key_encryption_key = "your-api-key-encryption-key"
cors_origins = ["*"]

[logging]
level = "info"
format = "json"
output = "stdout"

[service_discovery]
enabled = true
consul_url = "http://localhost:8500"
service_name = "syros-platform"
service_id = "syros-platform-1"
health_check_interval = 30
tags = ["api", "grpc"]
```

## Configuração do Servidor

### Configurações Básicas

```toml
[server]
# Porta do servidor REST API
port = 8080

# Porta do servidor gRPC
grpc_port = 9090

# Porta do servidor WebSocket
websocket_port = 8081

# Host/IP para binding
host = "127.0.0.1"

# Interface de rede específica (opcional)
interface = "eth0"

# Timeout para requisições (segundos)
request_timeout = 30

# Tamanho máximo do body (bytes)
max_body_size = 1048576

# Número de workers (0 = automático)
workers = 0
```

### Configurações Avançadas

```toml
[server.advanced]
# Keep-alive timeout
keep_alive_timeout = 75

# TCP keep-alive
tcp_keep_alive = true

# TCP no delay
tcp_nodelay = true

# Buffer size para leitura
read_buffer_size = 8192

# Buffer size para escrita
write_buffer_size = 8192

# Máximo de conexões simultâneas
max_connections = 1000

# Timeout para shutdown graceful
shutdown_timeout = 30
```

## Configuração de Storage

### Redis

```toml
[storage.redis]
# URL de conexão
url = "redis://localhost:6379"

# Tamanho do pool de conexões
pool_size = 10

# Timeout para operações (segundos)
timeout_seconds = 30

# Configurações de retry
max_retries = 3
retry_delay = 100

# Configurações de cluster
cluster_nodes = ["redis://node1:6379", "redis://node2:6379", "redis://node3:6379"]

# Configurações de sentinel
sentinel_masters = ["mymaster"]
sentinel_nodes = ["redis://sentinel1:26379", "redis://sentinel2:26379"]

# Configurações de autenticação
password = "your-redis-password"
username = "your-redis-username"

# Configurações de TLS
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
tls_ca = "/path/to/ca.pem"
```

### PostgreSQL

```toml
[storage.database]
# URL de conexão
url = "postgres://localhost:5432/syros"

# Tamanho do pool de conexões
pool_size = 10

# Timeout para operações (segundos)
timeout_seconds = 30

# Configurações de retry
max_retries = 3
retry_delay = 100

# Configurações de SSL
ssl_mode = "prefer"
ssl_cert = "/path/to/cert.pem"
ssl_key = "/path/to/key.pem"
ssl_ca = "/path/to/ca.pem"

# Configurações de pool
min_connections = 1
max_connections = 20
connection_timeout = 30
idle_timeout = 600

# Configurações de migração
migrate_on_startup = true
migration_path = "migrations/"
```

### etcd

```toml
[storage.etcd]
# URLs dos nós etcd
endpoints = ["http://localhost:2379", "http://localhost:2380"]

# Timeout para operações (segundos)
timeout_seconds = 5

# Configurações de retry
max_retries = 3
retry_delay = 100

# Configurações de autenticação
username = "your-etcd-username"
password = "your-etcd-password"

# Configurações de TLS
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
tls_ca = "/path/to/ca.pem"
```

## Configuração de Segurança

### Autenticação JWT

```toml
[security.jwt]
# Chave secreta para assinar tokens
secret = "your-jwt-secret-key"

# Algoritmo de assinatura
algorithm = "HS256"

# Tempo de expiração do token (segundos)
expiration = 3600

# Tempo de expiração do refresh token (segundos)
refresh_expiration = 86400

# Issuer do token
issuer = "syros-platform"

# Audience do token
audience = "syros-clients"

# Configurações de clock skew
clock_skew = 60
```

### API Keys

```toml
[security.api_keys]
# Chave para criptografar API keys
encryption_key = "your-api-key-encryption-key"

# Algoritmo de criptografia
encryption_algorithm = "AES-256-GCM"

# Prefixo para API keys
prefix = "sk_"

# Tamanho da API key
key_length = 32

# Tempo de expiração (segundos, 0 = nunca expira)
expiration = 0
```

### CORS

```toml
[security.cors]
# Origins permitidos
origins = ["*"]

# Métodos permitidos
methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]

# Headers permitidos
headers = ["Content-Type", "Authorization", "X-API-Key"]

# Headers expostos
expose_headers = ["X-Total-Count", "X-Page-Size"]

# Credenciais permitidas
allow_credentials = true

# Tempo de cache do preflight (segundos)
max_age = 86400
```

### Rate Limiting

```toml
[security.rate_limiting]
# Habilitar rate limiting
enabled = true

# Número de requisições por janela
requests_per_window = 1000

# Tamanho da janela (segundos)
window_size = 3600

# Estratégia de rate limiting
strategy = "sliding_window"  # sliding_window, fixed_window, token_bucket

# Headers de resposta
include_headers = true

# Configurações por IP
per_ip_limits = true

# Configurações por usuário
per_user_limits = true
```

### RBAC

```toml
[security.rbac]
# Habilitar RBAC
enabled = true

# Cache de permissões
cache_permissions = true

# TTL do cache de permissões (segundos)
cache_ttl = 300

# Configurações de roles padrão
default_roles = ["guest", "user", "admin"]

# Configurações de permissões padrão
default_permissions = [
    "locks:read",
    "locks:write",
    "sagas:read",
    "sagas:write",
    "events:read",
    "events:write",
    "cache:read",
    "cache:write"
]
```

## Configuração de Logging

### Configurações Básicas

```toml
[logging]
# Nível de log
level = "info"  # trace, debug, info, warn, error

# Formato de log
format = "json"  # json, text, pretty

# Output do log
output = "stdout"  # stdout, stderr, file

# Arquivo de log (se output = "file")
file_path = "/var/log/syros-platform.log"

# Rotação de logs
rotation = "daily"  # daily, hourly, never

# Máximo de arquivos de log
max_files = 7

# Tamanho máximo do arquivo (bytes)
max_file_size = 10485760
```

### Configurações Avançadas

```toml
[logging.advanced]
# Incluir timestamp
include_timestamp = true

# Formato do timestamp
timestamp_format = "%Y-%m-%dT%H:%M:%S%.3fZ"

# Incluir thread ID
include_thread_id = false

# Incluir span ID
include_span_id = true

# Incluir trace ID
include_trace_id = true

# Configurações de filtro
filter_modules = ["syros_platform"]
exclude_modules = ["tokio", "hyper"]

# Configurações de cor
color_output = true
color_stderr = false
```

### Configurações por Componente

```toml
[logging.components]
# Nível de log por componente
lock_manager = "debug"
saga_orchestrator = "info"
event_store = "info"
cache_manager = "info"
rest_api = "info"
grpc_api = "info"
websocket_api = "info"
graphql_api = "info"
```

## Configuração de Observabilidade

### Métricas

```toml
[observability.metrics]
# Habilitar métricas
enabled = true

# Porta do servidor de métricas
port = 9090

# Path do endpoint de métricas
path = "/metrics"

# Configurações de coleta
collect_interval = 15

# Configurações de retenção
retention_days = 30

# Configurações de agregação
aggregation_interval = 60

# Configurações de labels
include_labels = true
label_whitelist = ["method", "endpoint", "status"]
```

### Tracing

```toml
[observability.tracing]
# Habilitar tracing
enabled = true

# Endpoint do Jaeger
jaeger_endpoint = "http://localhost:14268/api/traces"

# Nome do serviço
service_name = "syros-platform"

# Configurações de sampling
sampling_rate = 0.1

# Configurações de batch
batch_size = 100
batch_timeout = 5

# Configurações de tags
tags = {
    "environment" = "production"
    "version" = "1.0.0"
}
```

### Health Checks

```toml
[observability.health]
# Habilitar health checks
enabled = true

# Configurações de readiness
readiness_check = true
readiness_timeout = 30

# Configurações de liveness
liveness_check = true
liveness_timeout = 30

# Configurações de startup
startup_check = true
startup_timeout = 60

# Configurações de dependências
check_dependencies = true
dependency_timeout = 10
```

## Configuração de Service Discovery

### Consul

```toml
[service_discovery.consul]
# Habilitar service discovery
enabled = true

# URL do Consul
url = "http://localhost:8500"

# Nome do serviço
service_name = "syros-platform"

# ID do serviço
service_id = "syros-platform-1"

# Endereço do serviço
service_address = "127.0.0.1"

# Porta do serviço
service_port = 8080

# Tags do serviço
tags = ["api", "grpc", "websocket"]

# Configurações de health check
health_check_interval = 30
health_check_timeout = 10
health_check_path = "/health"

# Configurações de retry
max_retries = 3
retry_delay = 1000

# Configurações de TLS
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
tls_ca = "/path/to/ca.pem"
```

### etcd

```toml
[service_discovery.etcd]
# Habilitar service discovery
enabled = false

# URLs dos nós etcd
endpoints = ["http://localhost:2379"]

# Prefixo para chaves
key_prefix = "/syros/services/"

# TTL do registro (segundos)
ttl = 30

# Configurações de retry
max_retries = 3
retry_delay = 1000
```

## Configuração de Desenvolvimento

### Configurações de Debug

```toml
[development]
# Modo debug
debug = false

# Logs verbosos
verbose = false

# Configurações de hot reload
hot_reload = false
watch_files = ["src/", "config/"]

# Configurações de profiling
profiling = false
profile_output = "profile.prof"

# Configurações de testing
test_mode = false
mock_external_services = false
```

### Configurações de Teste

```toml
[testing]
# Configurações de banco de dados de teste
test_database_url = "postgres://localhost:5432/syros_test"

# Configurações de Redis de teste
test_redis_url = "redis://localhost:6379/1"

# Configurações de timeout
test_timeout = 30

# Configurações de cleanup
cleanup_after_tests = true
cleanup_interval = 60
```

## Variáveis de Ambiente

### Mapeamento de Variáveis

```bash
# Servidor
export SYROS_SERVER_PORT=8080
export SYROS_SERVER_GRPC_PORT=9090
export SYROS_SERVER_WEBSOCKET_PORT=8081
export SYROS_SERVER_HOST=127.0.0.1

# Storage
export SYROS_REDIS_URL=redis://localhost:6379
export SYROS_POSTGRES_URL=postgres://localhost:5432/syros
export SYROS_ETCD_ENDPOINTS=http://localhost:2379

# Segurança
export SYROS_JWT_SECRET=your-jwt-secret
export SYROS_API_KEY_ENCRYPTION_KEY=your-api-key-encryption-key

# Logging
export SYROS_LOG_LEVEL=info
export SYROS_LOG_FORMAT=json
export SYROS_LOG_OUTPUT=stdout

# Service Discovery
export SYROS_CONSUL_URL=http://localhost:8500
export SYROS_SERVICE_NAME=syros-platform
export SYROS_SERVICE_ID=syros-platform-1
```

### Configuração por Ambiente

```bash
# Desenvolvimento
export SYROS_ENV=development
export SYROS_LOG_LEVEL=debug
export SYROS_DEBUG=true

# Produção
export SYROS_ENV=production
export SYROS_LOG_LEVEL=info
export SYROS_DEBUG=false
export SYROS_TLS_ENABLED=true
```

## Exemplos de Configuração

### Desenvolvimento Local

```toml
[server]
port = 3000
host = "127.0.0.1"

[storage.redis]
url = "redis://localhost:6379"
pool_size = 5

[storage.database]
url = "postgres://localhost:5432/syros_dev"
pool_size = 5

[logging]
level = "debug"
format = "pretty"
output = "stdout"

[development]
debug = true
verbose = true
```

### Produção

```toml
[server]
port = 8080
host = "0.0.0.0"
workers = 4

[storage.redis]
url = "redis://redis-cluster:6379"
pool_size = 20

[storage.database]
url = "postgres://postgres-cluster:5432/syros"
pool_size = 20

[security]
jwt_secret = "${JWT_SECRET}"
api_key_encryption_key = "${API_KEY_ENCRYPTION_KEY}"
cors_origins = ["https://app.example.com"]

[logging]
level = "info"
format = "json"
output = "file"
file_path = "/var/log/syros-platform.log"

[observability.metrics]
enabled = true
port = 9090

[service_discovery.consul]
enabled = true
url = "http://consul:8500"
```

### Kubernetes

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: syros-platform-config
data:
  config.toml: |
    [server]
    port = 8080
    host = "0.0.0.0"
    
    [storage.redis]
    url = "redis://redis-service:6379"
    
    [storage.database]
    url = "postgres://postgres-service:5432/syros"
    
    [security]
    jwt_secret = "${JWT_SECRET}"
    
    [logging]
    level = "info"
    format = "json"
```

---

**Próximo**: [Deployment](deployment.md) | [Observabilidade](observability.md) | [FAQ](faq.md)
