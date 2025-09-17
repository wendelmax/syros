# Configuration

This guide details all available configuration options in Syros.

## Configuration Structure

### Main File

The main configuration file is in `config/default.toml`:

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

## Server Configuration

### Basic Settings

```toml
[server]
# REST API server port
port = 8080

# gRPC server port
grpc_port = 9090

# WebSocket server port
websocket_port = 8081

# Host/IP for binding
host = "127.0.0.1"

# Specific network interface (optional)
interface = "eth0"

# Request timeout (seconds)
request_timeout = 30

# Maximum body size (bytes)
max_body_size = 1048576

# Number of workers (0 = automatic)
workers = 0
```

### Advanced Settings

```toml
[server.advanced]
# Keep-alive timeout
keep_alive_timeout = 75

# TCP keep-alive
tcp_keep_alive = true

# TCP no delay
tcp_nodelay = true

# Read buffer size
read_buffer_size = 8192

# Write buffer size
write_buffer_size = 8192

# Maximum concurrent connections
max_connections = 1000

# Graceful shutdown timeout
shutdown_timeout = 30
```

## Storage Configuration

### Redis

```toml
[storage.redis]
# Connection URL
url = "redis://localhost:6379"

# Connection pool size
pool_size = 10

# Operation timeout (seconds)
timeout_seconds = 30

# Retry settings
max_retries = 3
retry_delay = 100

# Cluster settings
cluster_nodes = ["redis://node1:6379", "redis://node2:6379", "redis://node3:6379"]

# Sentinel settings
sentinel_masters = ["mymaster"]
sentinel_nodes = ["redis://sentinel1:26379", "redis://sentinel2:26379"]

# Authentication settings
password = "your-redis-password"
username = "your-redis-username"

# TLS settings
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
tls_ca = "/path/to/ca.pem"
```

### PostgreSQL

```toml
[storage.database]
# Connection URL
url = "postgres://localhost:5432/syros"

# Connection pool size
pool_size = 10

# Operation timeout (seconds)
timeout_seconds = 30

# Retry settings
max_retries = 3
retry_delay = 100

# SSL settings
ssl_mode = "prefer"
ssl_cert = "/path/to/cert.pem"
ssl_key = "/path/to/key.pem"
ssl_ca = "/path/to/ca.pem"

# Pool settings
min_connections = 1
max_connections = 20
connection_timeout = 30
idle_timeout = 600

# Migration settings
migrate_on_startup = true
migration_path = "migrations/"
```

### etcd

```toml
[storage.etcd]
# etcd node URLs
endpoints = ["http://localhost:2379", "http://localhost:2380"]

# Operation timeout (seconds)
timeout_seconds = 5

# Retry settings
max_retries = 3
retry_delay = 100

# Authentication settings
username = "your-etcd-username"
password = "your-etcd-password"

# TLS settings
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
tls_ca = "/path/to/ca.pem"
```

## Security Configuration

### JWT Authentication

```toml
[security.jwt]
# Secret key for signing tokens
secret = "your-jwt-secret-key"

# Signature algorithm
algorithm = "HS256"

# Token expiration time (seconds)
expiration = 3600

# Refresh token expiration time (seconds)
refresh_expiration = 86400

# Token issuer
issuer = "syros-platform"

# Token audience
audience = "syros-clients"

# Clock skew settings
clock_skew = 60
```

### API Keys

```toml
[security.api_keys]
# Key for encrypting API keys
encryption_key = "your-api-key-encryption-key"

# Encryption algorithm
encryption_algorithm = "AES-256-GCM"

# API key prefix
prefix = "sk_"

# API key length
key_length = 32

# Expiration time (seconds, 0 = never expires)
expiration = 0
```

### CORS

```toml
[security.cors]
# Allowed origins
origins = ["*"]

# Allowed methods
methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]

# Allowed headers
headers = ["Content-Type", "Authorization", "X-API-Key"]

# Exposed headers
expose_headers = ["X-Total-Count", "X-Page-Size"]

# Allow credentials
allow_credentials = true

# Preflight cache time (seconds)
max_age = 86400
```

### Rate Limiting

```toml
[security.rate_limiting]
# Enable rate limiting
enabled = true

# Number of requests per window
requests_per_window = 1000

# Window size (seconds)
window_size = 3600

# Rate limiting strategy
strategy = "sliding_window"  # sliding_window, fixed_window, token_bucket

# Response headers
include_headers = true

# Per-IP limits
per_ip_limits = true

# Per-user limits
per_user_limits = true
```

### RBAC

```toml
[security.rbac]
# Enable RBAC
enabled = true

# Permission cache
cache_permissions = true

# Permission cache TTL (seconds)
cache_ttl = 300

# Default roles
default_roles = ["guest", "user", "admin"]

# Default permissions
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

## Logging Configuration

### Basic Settings

```toml
[logging]
# Log level
level = "info"  # trace, debug, info, warn, error

# Log format
format = "json"  # json, text, pretty

# Log output
output = "stdout"  # stdout, stderr, file

# Log file (if output = "file")
file_path = "/var/log/syros-platform.log"

# Log rotation
rotation = "daily"  # daily, hourly, never

# Maximum log files
max_files = 7

# Maximum file size (bytes)
max_file_size = 10485760
```

### Advanced Settings

```toml
[logging.advanced]
# Include timestamp
include_timestamp = true

# Timestamp format
timestamp_format = "%Y-%m-%dT%H:%M:%S%.3fZ"

# Include thread ID
include_thread_id = false

# Include span ID
include_span_id = true

# Include trace ID
include_trace_id = true

# Filter settings
filter_modules = ["syros_platform"]
exclude_modules = ["tokio", "hyper"]

# Color settings
color_output = true
color_stderr = false
```

### Component Settings

```toml
[logging.components]
# Log level per component
lock_manager = "debug"
saga_orchestrator = "info"
event_store = "info"
cache_manager = "info"
rest_api = "info"
grpc_api = "info"
websocket_api = "info"
graphql_api = "info"
```

## Observability Configuration

### Metrics

```toml
[observability.metrics]
# Enable metrics
enabled = true

# Metrics server port
port = 9090

# Metrics endpoint path
path = "/metrics"

# Collection settings
collect_interval = 15

# Retention settings
retention_days = 30

# Aggregation settings
aggregation_interval = 60

# Label settings
include_labels = true
label_whitelist = ["method", "endpoint", "status"]
```

### Tracing

```toml
[observability.tracing]
# Enable tracing
enabled = true

# Jaeger endpoint
jaeger_endpoint = "http://localhost:14268/api/traces"

# Service name
service_name = "syros-platform"

# Sampling settings
sampling_rate = 0.1

# Batch settings
batch_size = 100
batch_timeout = 5

# Tag settings
tags = {
    "environment" = "production"
    "version" = "1.0.0"
}
```

### Health Checks

```toml
[observability.health]
# Enable health checks
enabled = true

# Readiness settings
readiness_check = true
readiness_timeout = 30

# Liveness settings
liveness_check = true
liveness_timeout = 30

# Startup settings
startup_check = true
startup_timeout = 60

# Dependencies settings
check_dependencies = true
dependency_timeout = 10
```

## Service Discovery Configuration

### Consul

```toml
[service_discovery.consul]
# Enable service discovery
enabled = true

# Consul URL
url = "http://localhost:8500"

# Service name
service_name = "syros-platform"

# Service ID
service_id = "syros-platform-1"

# Service address
service_address = "127.0.0.1"

# Service port
service_port = 8080

# Service tags
tags = ["api", "grpc", "websocket"]

# Health check settings
health_check_interval = 30
health_check_timeout = 10
health_check_path = "/health"

# Retry settings
max_retries = 3
retry_delay = 1000

# TLS settings
tls_enabled = false
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
tls_ca = "/path/to/ca.pem"
```

### etcd

```toml
[service_discovery.etcd]
# Enable service discovery
enabled = false

# etcd node URLs
endpoints = ["http://localhost:2379"]

# Key prefix
key_prefix = "/syros/services/"

# Registration TTL (seconds)
ttl = 30

# Retry settings
max_retries = 3
retry_delay = 1000
```

## Development Configuration

### Debug Settings

```toml
[development]
# Debug mode
debug = false

# Verbose logs
verbose = false

# Hot reload settings
hot_reload = false
watch_files = ["src/", "config/"]

# Profiling settings
profiling = false
profile_output = "profile.prof"

# Testing settings
test_mode = false
mock_external_services = false
```

### Test Settings

```toml
[testing]
# Test database settings
test_database_url = "postgres://localhost:5432/syros_test"

# Test Redis settings
test_redis_url = "redis://localhost:6379/1"

# Timeout settings
test_timeout = 30

# Cleanup settings
cleanup_after_tests = true
cleanup_interval = 60
```

## Environment Variables

### Variable Mapping

```bash
# Server
export SYROS_SERVER_PORT=8080
export SYROS_SERVER_GRPC_PORT=9090
export SYROS_SERVER_WEBSOCKET_PORT=8081
export SYROS_SERVER_HOST=127.0.0.1

# Storage
export SYROS_REDIS_URL=redis://localhost:6379
export SYROS_POSTGRES_URL=postgres://localhost:5432/syros
export SYROS_ETCD_ENDPOINTS=http://localhost:2379

# Security
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

### Environment-specific Configuration

```bash
# Development
export SYROS_ENV=development
export SYROS_LOG_LEVEL=debug
export SYROS_DEBUG=true

# Production
export SYROS_ENV=production
export SYROS_LOG_LEVEL=info
export SYROS_DEBUG=false
export SYROS_TLS_ENABLED=true
```

## Configuration Examples

### Local Development

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

### Production

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

**Next**: [Deployment](deployment.md) | [Observability](observability.md) | [FAQ](faq.md)