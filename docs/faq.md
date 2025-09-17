# FAQ - Frequently Asked Questions

## Installation and Setup

### How to install Syros?

```bash
# 1. Clone the repository
git clone https://github.com/wendelmax/syros.git
cd syros

# 2. Build the project
cargo build --release

# 3. Start the server
cargo run
```

### What are the prerequisites?

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Docker** (optional) - [Install Docker](https://docs.docker.com/get-docker/)
- **Python 3.8+** (for testing) - [Install Python](https://www.python.org/downloads/)

### How to configure the server?

Create a `config/default.toml` file:

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
```

## Usage and Features

### How to use distributed locks?

```bash
# 1. Acquire lock
curl -X POST http://localhost:8080/api/v1/locks \
  -H "Content-Type: application/json" \
  -d '{
    "key": "resource-123",
    "ttl": 300,
    "owner": "service-a"
  }'

# 2. Check status
curl http://localhost:8080/api/v1/locks/resource-123/status

# 3. Release lock
curl -X DELETE http://localhost:8080/api/v1/locks/resource-123 \
  -H "Content-Type: application/json" \
  -d '{"lock_id": "lock-uuid-123"}'
```

### How to use sagas for distributed transactions?

```bash
# 1. Start saga
curl -X POST http://localhost:8080/api/v1/sagas \
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
      }
    ]
  }'

# 2. Execute step
curl -X POST http://localhost:8080/api/v1/sagas/saga-uuid-456/execute \
  -H "Content-Type: application/json" \
  -d '{"step_id": "validate-order", "data": {"order_id": "123"}}'
```

### How to use Event Store?

```bash
# 1. Add event
curl -X POST http://localhost:8080/api/v1/events \
  -H "Content-Type: application/json" \
  -d '{
    "stream_id": "user-123",
    "event_type": "user_created",
    "data": {
      "user_id": "123",
      "email": "user@example.com"
    }
  }'

# 2. Search events
curl "http://localhost:8080/api/v1/events/user-123?limit=10"
```

### How to use distributed cache?

```bash
# 1. Store in cache
curl -X POST http://localhost:8080/api/v1/cache \
  -H "Content-Type: application/json" \
  -d '{
    "key": "user-profile-123",
    "value": {"name": "John", "email": "john@example.com"},
    "ttl": 3600
  }'

# 2. Retrieve from cache
curl http://localhost:8080/api/v1/cache/user-profile-123
```

## Authentication and Security

### How to authenticate with the API?

```bash
# 1. Get JWT token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'

# 2. Use token in requests
export TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
curl -H "Authorization: Bearer $TOKEN" http://localhost:8080/api/v1/locks
```

### How to use API Keys?

```bash
# Use API Key in requests
curl -H "X-API-Key: your-api-key" http://localhost:8080/api/v1/locks
```

### How to configure RBAC?

```bash
# Create user with roles
curl -X POST http://localhost:8080/api/v1/auth/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "developer",
    "email": "dev@example.com",
    "roles": ["developer", "read-only"]
  }'
```

## Docker and Deployment

### How to use Docker?

```bash
# 1. Build image
docker build -t syros-platform .

# 2. Run container
docker run -p 8080:8080 -p 9090:9090 syros-platform

# 3. Use Docker Compose
docker-compose up -d
```

### How to deploy to Kubernetes?

```bash
# 1. Apply manifests
kubectl apply -f k8s/

# 2. Check pods
kubectl get pods -l app=syros-platform

# 3. Check logs
kubectl logs -f deployment/syros-platform
```

### How to use Helm?

```bash
# 1. Install chart
helm install syros-platform ./helm/syros-platform

# 2. Update
helm upgrade syros-platform ./helm/syros-platform

# 3. Uninstall
helm uninstall syros-platform
```

## Monitoring and Observability

### How to access metrics?

```bash
# Prometheus metrics
curl http://localhost:8080/metrics

# Health checks
curl http://localhost:8080/health
curl http://localhost:8080/ready
curl http://localhost:8080/live
```

### How to configure Grafana?

1. Install Grafana
2. Configure Prometheus as data source
3. Import dashboards from `grafana/` folder
4. Visualize real-time metrics

### How to configure structured logs?

```toml
[logging]
level = "info"
format = "json"
output = "stdout"
```

## Testing

### How to run tests?

```bash
# All tests
cargo test

# Integration tests
cargo test --test integration_test

# Tests with output
cargo test -- --nocapture
```

### How to use mocks for testing?

```rust
use syros_platform::mock_server::{MockServer, with_mock_server};

#[tokio::test]
async fn test_with_mock() {
    with_mock_server(|server| async move {
        // Your tests here
        let response = server.rest_url();
        assert!(response.contains("http://"));
    }).await.unwrap();
}
```

## Troubleshooting

### Error: "Port already in use"

```bash
# Check processes using the port
netstat -tulpn | grep :8080

# Kill process (Linux/Mac)
sudo kill -9 $(lsof -t -i:8080)

# Kill process (Windows)
taskkill /F /IM syros-platform.exe
```

### Error: "Connection refused Redis/PostgreSQL"

```bash
# Check if services are running
docker ps | grep redis
docker ps | grep postgres

# Start with Docker Compose
docker-compose up -d redis postgres
```

### Error: "Compilation failed"

```bash
# Update Rust
rustup update

# Clean cache
cargo clean

# Rebuild
cargo build --release
```

### Error: "Invalid token"

```bash
# Check if token is correct
echo $TOKEN

# Get new token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'
```

## SDKs

### How to use Python SDK?

```python
from syros_platform import SyrosClient

client = SyrosClient(
    base_url="http://localhost:8080",
    api_key="your-api-key"
)

# Acquire lock
lock = client.locks.acquire(
    key="resource-123",
    ttl=300,
    owner="python-service"
)
```

### How to use Node.js SDK?

```javascript
const { SyrosClient } = require('@syros/platform-sdk');

const client = new SyrosClient({
  baseUrl: 'http://localhost:8080',
  apiKey: 'your-api-key'
});

// Acquire lock
const lock = await client.locks.acquire({
  key: 'resource-123',
  ttl: 300,
  owner: 'node-service'
});
```

### How to use Java SDK?

```java
import com.syros.platform.SyrosClient;

SyrosClient client = new SyrosClient.Builder()
    .baseUrl("http://localhost:8080")
    .apiKey("your-api-key")
    .build();

// Acquire lock
AcquireLockRequest request = AcquireLockRequest.builder()
    .key("resource-123")
    .ttl(300)
    .owner("java-service")
    .build();

LockResponse lock = client.locks().acquire(request);
```

## Performance

### What is the platform's performance?

- **Locks**: ~10,000 operations/second
- **Cache**: ~50,000 operations/second
- **Event Store**: ~5,000 events/second
- **gRPC**: ~20,000 requests/second
- **REST API**: ~5,000 requests/second

### How to optimize performance?

1. **Use gRPC** for high performance
2. **Configure connection pooling** properly
3. **Use cache** for frequently accessed data
4. **Monitor metrics** to identify bottlenecks
5. **Configure retry policies** properly

## Security

### How to configure HTTPS?

```toml
[server]
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
```

### How to configure CORS?

```toml
[security]
cors_origins = ["https://example.com", "https://app.example.com"]
```

### How to configure rate limiting?

```toml
[security]
rate_limit_requests = 1000
rate_limit_window = 3600
```

## Scalability

### How to scale horizontally?

1. **Use load balancer** (nginx, HAProxy)
2. **Configure service discovery** (Consul)
3. **Use Redis Cluster** for locks and cache
4. **Configure PostgreSQL replication** for Event Store
5. **Use Kubernetes** for orchestration

### How to monitor scalability?

1. **CPU and memory metrics**
2. **Request latency**
3. **Throughput per second**
4. **Error rate**
5. **Resource utilization**

## Support

### How to get help?

1. **Check documentation** in [`docs/`](docs/)
2. **Open an issue** on [GitHub](https://github.com/wendelmax/syros/issues)
3. **Join the community** on Discord
4. **Check examples** in `examples/` folder

### How to report bugs?

1. **Check if a similar issue already exists**
2. **Create a new issue** with:
   - Problem description
   - Steps to reproduce
   - Relevant logs
   - Platform version
   - Operating system

### How to contribute?

1. **Fork the project**
2. **Create a branch** for your feature
3. **Implement and test** your change
4. **Open a Pull Request** with detailed description

---

**Need more help?** Check the [complete documentation](README.md) or open an [issue](https://github.com/wendelmax/syros/issues)!