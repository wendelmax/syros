# Deployment Guide - Syros

This guide presents different deployment options for Syros, from local development to production at scale.

## 🚀 Quick Start

### Local Development

```bash
# Clone and build
git clone https://github.com/wendelmax/syros.git
cd syros
cargo build --release

# Run only REST API
cargo run -- start --servers rest --host 127.0.0.1 --port 8080

# Run all servers
cargo run -- start --servers all --host 0.0.0.0
```

### Docker

```bash
# Build image
docker build -t syros-platform .

# Run with Docker
docker run -p 8080:8080 -p 9090:9090 syros-platform

# Run with specific configuration
docker run -p 8080:8080 -p 9090:9090 \
  -e SYROS_HOST=0.0.0.0 \
  -e SYROS_PORT=8080 \
  -e SYROS_GRPC_PORT=9090 \
  syros-platform
```

## 🏗️ Deployment Configurations

### 1. Development

**Minimum configuration for development:**

```bash
# Only REST API
cargo run -- start --servers rest --host 127.0.0.1 --port 3000

# With gRPC for testing
cargo run -- start --servers rest,grpc --host 127.0.0.1 --port 3000 --grpc-port 9091
```

**Required resources:**
- CPU: 1 core
- RAM: 512MB
- Disk: 1GB

### 2. Staging

**Configuration for staging environment:**

```bash
# All servers with specific IP
cargo run -- start --servers all \
  --host 192.168.1.100 \
  --port 8080 \
  --grpc-port 9090 \
  --websocket-port 8081
```

**Required resources:**
- CPU: 2 cores
- RAM: 2GB
- Disk: 10GB

### 3. Production

**Configuration for production:**

```bash
# Production server with specific interface
cargo run -- start --servers all \
  --host 0.0.0.0 \
  --port 8080 \
  --grpc-port 9090 \
  --websocket-port 8081 \
  --interface eth0
```

**Required resources:**
- CPU: 4+ cores
- RAM: 8GB+
- Disk: 100GB+
- Network: 1Gbps+

## 🐳 Docker Deployment

### Optimized Dockerfile

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/syros-platform /usr/local/bin/

EXPOSE 8080 9090 8081

CMD ["syros-platform", "start", "--servers", "all", "--host", "0.0.0.0"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  syros-platform:
    build: .
    ports:
      - "8080:8080"  # REST API
      - "9090:9090"  # gRPC
      - "8081:8081"  # WebSocket
    environment:
      - SYROS_HOST=0.0.0.0
      - SYROS_PORT=8080
      - SYROS_GRPC_PORT=9090
      - SYROS_WEBSOCKET_PORT=8081
    depends_on:
      - redis
      - postgres
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    restart: unless-stopped

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=syros
      - POSTGRES_USER=syros
      - POSTGRES_PASSWORD=syros_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    restart: unless-stopped

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana
    restart: unless-stopped

volumes:
  redis_data:
  postgres_data:
  grafana_data:
```

## ☸️ Kubernetes Deployment

### Namespace

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: syros-platform
```

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: syros-config
  namespace: syros-platform
data:
  host: "0.0.0.0"
  port: "8080"
  grpc_port: "9090"
  websocket_port: "8081"
  servers: "all"
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: syros-platform
  namespace: syros-platform
spec:
  replicas: 3
  selector:
    matchLabels:
      app: syros-platform
  template:
    metadata:
      labels:
        app: syros-platform
    spec:
      containers:
      - name: syros-platform
        image: syros-platform:latest
        ports:
        - containerPort: 8080
          name: rest
        - containerPort: 9090
          name: grpc
        - containerPort: 8081
          name: websocket
        env:
        - name: SYROS_HOST
          valueFrom:
            configMapKeyRef:
              name: syros-config
              key: host
        - name: SYROS_PORT
          valueFrom:
            configMapKeyRef:
              name: syros-config
              key: port
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: syros-platform-service
  namespace: syros-platform
spec:
  selector:
    app: syros-platform
  ports:
  - name: rest
    port: 8080
    targetPort: 8080
  - name: grpc
    port: 9090
    targetPort: 9090
  - name: websocket
    port: 8081
    targetPort: 8081
  type: LoadBalancer
```

## 🔧 Advanced Configurations

### Environment Variables

```bash
# Server configuration
export SYROS_HOST=0.0.0.0
export SYROS_PORT=8080
export SYROS_GRPC_PORT=9090
export SYROS_WEBSOCKET_PORT=8081
export SYROS_SERVERS=all

# Database configuration
export SYROS_REDIS_URL=redis://localhost:6379
export SYROS_DATABASE_URL=postgresql://localhost/syros

# Security configuration
export SYROS_JWT_SECRET=your-secret-key
export SYROS_API_KEY_ENCRYPTION_KEY=your-api-key

# Logging configuration
export SYROS_LOG_LEVEL=info
export SYROS_LOG_FORMAT=json
```

### Configuration File

```toml
[server]
host = "0.0.0.0"
port = 8080
grpc_port = 9090
websocket_port = 8081

[storage]
redis.url = "redis://localhost:6379"
redis.pool_size = 10
redis.timeout_seconds = 30

database.url = "postgresql://localhost/syros"
database.pool_size = 10
database.timeout_seconds = 30

[security]
jwt_secret = "your-secret-key"
api_key_encryption_key = "your-api-key"
cors_origins = ["*"]

[logging]
level = "info"
format = "json"
output = "stdout"
```

## 📊 Monitoring

### Health Checks

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed health check
curl http://localhost:8080/ready

# Prometheus metrics
curl http://localhost:8080/metrics
```

### Logs

```bash
# Real-time logs
docker logs -f syros-platform

# Filtered logs
docker logs syros-platform | grep ERROR

# Structured logs
docker logs syros-platform | jq .
```

## 🔒 Security

### Firewall

```bash
# Allow only necessary ports
ufw allow 8080/tcp  # REST API
ufw allow 9090/tcp  # gRPC
ufw allow 8081/tcp  # WebSocket
ufw deny 6379/tcp   # Redis (internal only)
ufw deny 5432/tcp   # PostgreSQL (internal only)
```

### SSL/TLS

```bash
# SSL certificate
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Nginx reverse proxy
server {
    listen 443 ssl;
    server_name platform.syros.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 🚀 Scalability

### Load Balancer

```yaml
# Nginx upstream
upstream syros_backend {
    server syros-1:8080;
    server syros-2:8080;
    server syros-3:8080;
}

server {
    listen 80;
    location / {
        proxy_pass http://syros_backend;
    }
}
```

### Auto-scaling

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: syros-platform-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: syros-platform
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

## 📈 Performance

### Optimizations

```bash
# Optimized compilation
cargo build --release --target x86_64-unknown-linux-gnu

# Thread configuration
export RAYON_NUM_THREADS=4

# Memory configuration
export MALLOC_ARENA_MAX=2
```

### Benchmarks

```bash
# Load test
wrk -t12 -c400 -d30s http://localhost:8080/health

# gRPC test
ghz --insecure --call syros.v1.SyrosService/AcquireLock \
  --data '{"key":"test","owner":"test","ttl_seconds":60}' \
  localhost:9090
```

## 🔄 Backup and Restore

### Backup

```bash
# Redis backup
redis-cli --rdb /backup/redis-$(date +%Y%m%d).rdb

# PostgreSQL backup
pg_dump syros > /backup/postgres-$(date +%Y%m%d).sql
```

### Restore

```bash
# Redis restore
redis-cli --pipe < /backup/redis-20241219.rdb

# PostgreSQL restore
psql syros < /backup/postgres-20241219.sql
```

## 🆘 Troubleshooting

### Common Issues

1. **Port already in use**
   ```bash
   # Check ports in use
   netstat -tulpn | grep :8080
   
   # Kill process
   kill -9 $(lsof -t -i:8080)
   ```

2. **Redis connection error**
   ```bash
   # Check Redis
   redis-cli ping
   
   # Check logs
   docker logs redis
   ```

3. **PostgreSQL connection error**
   ```bash
   # Check PostgreSQL
   psql -h localhost -U syros -d syros -c "SELECT 1;"
   
   # Check logs
   docker logs postgres
   ```

### Debug Logs

```bash
# Run with debug
RUST_LOG=debug cargo run -- start --servers all

# Specific logs
RUST_LOG=syros_platform=debug cargo run -- start
```

## 📞 Support

For technical support:

- **Documentation**: [docs.syros.com](https://docs.syros.com)
- **Issues**: [GitHub Issues](https://github.com/wendelmax/syros/issues)
- **Discord**: [Syros Community](https://discord.gg/syros)
- **Email**: support@syros.com