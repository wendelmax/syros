# Guia de Deployment - Syros Platform

Este guia apresenta diferentes opções de deployment da Syros Platform, desde desenvolvimento local até produção em escala.

## 🚀 Início Rápido

### Desenvolvimento Local

```bash
# Clone e compile
git clone https://github.com/syros/platform.git
cd platform
cargo build --release

# Execute apenas REST API
cargo run -- start --servers rest --host 127.0.0.1 --port 8080

# Execute todos os servidores
cargo run -- start --servers all --host 0.0.0.0
```

### Docker

```bash
# Build da imagem
docker build -t syros-platform .

# Execute com Docker
docker run -p 8080:8080 -p 9090:9090 syros-platform

# Execute com configuração específica
docker run -p 8080:8080 -p 9090:9090 \
  -e SYROS_HOST=0.0.0.0 \
  -e SYROS_PORT=8080 \
  -e SYROS_GRPC_PORT=9090 \
  syros-platform
```

## 🏗️ Configurações de Deployment

### 1. Desenvolvimento

**Configuração mínima para desenvolvimento:**

```bash
# Apenas REST API
cargo run -- start --servers rest --host 127.0.0.1 --port 3000

# Com gRPC para testes
cargo run -- start --servers rest,grpc --host 127.0.0.1 --port 3000 --grpc-port 9091
```

**Recursos necessários:**
- CPU: 1 core
- RAM: 512MB
- Disco: 1GB

### 2. Staging

**Configuração para ambiente de staging:**

```bash
# Todos os servidores com IP específico
cargo run -- start --servers all \
  --host 192.168.1.100 \
  --port 8080 \
  --grpc-port 9090 \
  --websocket-port 8081
```

**Recursos necessários:**
- CPU: 2 cores
- RAM: 2GB
- Disco: 10GB

### 3. Produção

**Configuração para produção:**

```bash
# Servidor de produção com interface específica
cargo run -- start --servers all \
  --host 0.0.0.0 \
  --port 8080 \
  --grpc-port 9090 \
  --websocket-port 8081 \
  --interface eth0
```

**Recursos necessários:**
- CPU: 4+ cores
- RAM: 8GB+
- Disco: 100GB+
- Rede: 1Gbps+

## 🐳 Docker Deployment

### Dockerfile Otimizado

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

## 🔧 Configurações Avançadas

### Variáveis de Ambiente

```bash
# Configuração do servidor
export SYROS_HOST=0.0.0.0
export SYROS_PORT=8080
export SYROS_GRPC_PORT=9090
export SYROS_WEBSOCKET_PORT=8081
export SYROS_SERVERS=all

# Configuração de banco
export SYROS_REDIS_URL=redis://localhost:6379
export SYROS_DATABASE_URL=postgresql://localhost/syros

# Configuração de segurança
export SYROS_JWT_SECRET=your-secret-key
export SYROS_API_KEY_ENCRYPTION_KEY=your-api-key

# Configuração de logging
export SYROS_LOG_LEVEL=info
export SYROS_LOG_FORMAT=json
```

### Arquivo de Configuração

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

## 📊 Monitoramento

### Health Checks

```bash
# Health check básico
curl http://localhost:8080/health

# Health check detalhado
curl http://localhost:8080/ready

# Métricas Prometheus
curl http://localhost:8080/metrics
```

### Logs

```bash
# Logs em tempo real
docker logs -f syros-platform

# Logs com filtro
docker logs syros-platform | grep ERROR

# Logs estruturados
docker logs syros-platform | jq .
```

## 🔒 Segurança

### Firewall

```bash
# Permitir apenas portas necessárias
ufw allow 8080/tcp  # REST API
ufw allow 9090/tcp  # gRPC
ufw allow 8081/tcp  # WebSocket
ufw deny 6379/tcp   # Redis (apenas interno)
ufw deny 5432/tcp   # PostgreSQL (apenas interno)
```

### SSL/TLS

```bash
# Certificado SSL
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

## 🚀 Escalabilidade

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

### Otimizações

```bash
# Compilação otimizada
cargo build --release --target x86_64-unknown-linux-gnu

# Configuração de threads
export RAYON_NUM_THREADS=4

# Configuração de memória
export MALLOC_ARENA_MAX=2
```

### Benchmarks

```bash
# Teste de carga
wrk -t12 -c400 -d30s http://localhost:8080/health

# Teste gRPC
ghz --insecure --call syros.v1.SyrosService/AcquireLock \
  --data '{"key":"test","owner":"test","ttl_seconds":60}' \
  localhost:9090
```

## 🔄 Backup e Restore

### Backup

```bash
# Backup do Redis
redis-cli --rdb /backup/redis-$(date +%Y%m%d).rdb

# Backup do PostgreSQL
pg_dump syros > /backup/postgres-$(date +%Y%m%d).sql
```

### Restore

```bash
# Restore do Redis
redis-cli --pipe < /backup/redis-20241219.rdb

# Restore do PostgreSQL
psql syros < /backup/postgres-20241219.sql
```

## 🆘 Troubleshooting

### Problemas Comuns

1. **Porta já em uso**
   ```bash
   # Verificar portas em uso
   netstat -tulpn | grep :8080
   
   # Matar processo
   kill -9 $(lsof -t -i:8080)
   ```

2. **Erro de conexão com Redis**
   ```bash
   # Verificar Redis
   redis-cli ping
   
   # Verificar logs
   docker logs redis
   ```

3. **Erro de conexão com PostgreSQL**
   ```bash
   # Verificar PostgreSQL
   psql -h localhost -U syros -d syros -c "SELECT 1;"
   
   # Verificar logs
   docker logs postgres
   ```

### Logs de Debug

```bash
# Executar com debug
RUST_LOG=debug cargo run -- start --servers all

# Logs específicos
RUST_LOG=syros_platform=debug cargo run -- start
```

## 📞 Suporte

Para suporte técnico:

- **Documentação**: [docs.syros.com](https://docs.syros.com)
- **Issues**: [GitHub Issues](https://github.com/syros/platform/issues)
- **Discord**: [Syros Community](https://discord.gg/syros)
- **Email**: support@syros.com
