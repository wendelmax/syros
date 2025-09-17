# Quick Start Guide

This guide will help you get started quickly with Syros Platform.

## Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Docker** (optional) - [Install Docker](https://docs.docker.com/get-docker/)
- **Python 3.8+** (for testing) - [Install Python](https://www.python.org/downloads/)

## Quick Installation

### 1. Clone the Repository

```bash
git clone https://github.com/wendelmax/syros.git
cd syros
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Configure (Optional)

```bash
# Set environment variables
export SYROS_REDIS_URL="redis://localhost:6379"
export SYROS_POSTGRES_URL="postgres://localhost:5432/syros"
export SYROS_JWT_SECRET="your-secret-key"
```

### 4. Start the Server

```bash
# Start all servers
cargo run

# Or start only REST API
cargo run -- start --servers rest --host 127.0.0.1 --port 8080
```

## First Steps

### Basic Test

```bash
# 1. Check if server is running
curl http://localhost:8080/health

# Expected response:
# {"status":"healthy","timestamp":"2025-09-19T10:00:00Z"}
```

### Lock Example

```bash
# 1. Acquire a lock
curl -X POST http://localhost:8080/api/v1/locks \
  -H "Content-Type: application/json" \
  -d '{
    "key": "my-resource",
    "ttl": 300,
    "owner": "my-service"
  }'

# 2. Check status
curl http://localhost:8080/api/v1/locks/my-resource/status

# 3. Release lock
curl -X DELETE http://localhost:8080/api/v1/locks/my-resource \
  -H "Content-Type: application/json" \
  -d '{"lock_id": "returned-lock-uuid"}'
```

### Cache Example

```bash
# 1. Store in cache
curl -X POST http://localhost:8080/api/v1/cache \
  -H "Content-Type: application/json" \
  -d '{
    "key": "user-123",
    "value": {"name": "John", "email": "john@example.com"},
    "ttl": 3600
  }'

# 2. Retrieve from cache
curl http://localhost:8080/api/v1/cache/user-123
```

## Basic Configuration

### Configuration File

Create `config/default.toml`:

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
jwt_secret = "your-jwt-secret-key"
api_key_encryption_key = "your-api-encryption-key"
cors_origins = ["*"]

[logging]
level = "info"
format = "json"
output = "stdout"
```

## Execution Modes

### Complete Server

```bash
cargo run -- start --servers all
```

### REST API Only

```bash
cargo run -- start --servers rest --host 127.0.0.1 --port 8080
```

### REST + gRPC

```bash
cargo run -- start --servers rest,grpc --host 0.0.0.0 --port 8080 --grpc-port 9090
```

### Verbose Mode

```bash
cargo run -- --verbose start --servers all
```

### Quiet Mode

```bash
cargo run -- --quiet start --servers rest
```

## Check Status

### Health Checks

```bash
# Basic health
curl http://localhost:8080/health

# Detailed health
curl http://localhost:8080/ready

# Health for Kubernetes
curl http://localhost:8080/live
```

### Metrics

```bash
# View Prometheus metrics
curl http://localhost:8080/metrics
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Integration Tests

```bash
cargo test --test integration_test
```

### Run with Coverage

```bash
cargo test -- --nocapture
```

## Docker (Optional)

### Using Docker Compose

```bash
# Start with Docker Compose
docker-compose up -d

# Check logs
docker-compose logs -f syros

# Stop
docker-compose down
```

### Using Docker

```bash
# Build image
docker build -t syros .

# Run container
docker run -p 8080:8080 -p 9090:9090 syros
```

## Next Steps

Now that you have Syros Platform running:

1. **Explore the APIs**: See [REST API](rest-api.md), [gRPC API](grpc-api.md), [WebSocket API](websocket-api.md)
2. **Use the SDKs**: Check [SDKs](sdks.md) for your preferred language
3. **Set up Observability**: See [Observability](observability.md)
4. **Deploy to Production**: Check [Deployment](deployment.md)

## Common Issues

### Port already in use

```bash
# Check processes using the port
netstat -tulpn | grep :8080

# Kill process (Linux/Mac)
sudo kill -9 $(lsof -t -i:8080)

# Kill process (Windows)
taskkill /F /IM syros.exe
```

### Redis/PostgreSQL connection error

```bash
# Check if services are running
docker ps | grep redis
docker ps | grep postgres

# Start with Docker Compose
docker-compose up -d redis postgres
```

### Compilation error

```bash
# Update Rust
rustup update

# Clean cache
cargo clean

# Rebuild
cargo build --release
```

## Additional Resources

- [Platform Architecture](architecture.md)
- [Advanced Configuration](configuration.md)
- [FAQ](faq.md)
- [Changelog](../CHANGELOG.md)

---

**Need help?** Open an [issue](https://github.com/wendelmax/syros/issues) or check the [FAQ](faq.md)!