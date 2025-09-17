# FAQ - Perguntas Frequentes

## Instalação e Configuração

### Como instalar a Syros Platform?

```bash
# 1. Clone o repositório
git clone https://github.com/syros/platform.git
cd platform

# 2. Compile o projeto
cargo build --release

# 3. Inicie o servidor
cargo run
```

### Quais são os pré-requisitos?

- **Rust 1.70+** - [Instalar Rust](https://rustup.rs/)
- **Docker** (opcional) - [Instalar Docker](https://docs.docker.com/get-docker/)
- **Python 3.8+** (para testes) - [Instalar Python](https://www.python.org/downloads/)

### Como configurar o servidor?

Crie um arquivo `config/default.toml`:

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

## Uso e Funcionalidades

### Como usar locks distribuídos?

```bash
# 1. Adquirir lock
curl -X POST http://localhost:8080/api/v1/locks \
  -H "Content-Type: application/json" \
  -d '{
    "key": "resource-123",
    "ttl": 300,
    "owner": "service-a"
  }'

# 2. Verificar status
curl http://localhost:8080/api/v1/locks/resource-123/status

# 3. Liberar lock
curl -X DELETE http://localhost:8080/api/v1/locks/resource-123 \
  -H "Content-Type: application/json" \
  -d '{"lock_id": "lock-uuid-123"}'
```

### Como usar sagas para transações distribuídas?

```bash
# 1. Iniciar saga
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

# 2. Executar passo
curl -X POST http://localhost:8080/api/v1/sagas/saga-uuid-456/execute \
  -H "Content-Type: application/json" \
  -d '{"step_id": "validate-order", "data": {"order_id": "123"}}'
```

### Como usar o Event Store?

```bash
# 1. Adicionar evento
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

# 2. Buscar eventos
curl "http://localhost:8080/api/v1/events/user-123?limit=10"
```

### Como usar o cache distribuído?

```bash
# 1. Armazenar no cache
curl -X POST http://localhost:8080/api/v1/cache \
  -H "Content-Type: application/json" \
  -d '{
    "key": "user-profile-123",
    "value": {"name": "João", "email": "joao@example.com"},
    "ttl": 3600
  }'

# 2. Recuperar do cache
curl http://localhost:8080/api/v1/cache/user-profile-123
```

## Autenticação e Segurança

### Como autenticar na API?

```bash
# 1. Obter token JWT
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'

# 2. Usar token nas requisições
export TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
curl -H "Authorization: Bearer $TOKEN" http://localhost:8080/api/v1/locks
```

### Como usar API Keys?

```bash
# Usar API Key nas requisições
curl -H "X-API-Key: your-api-key" http://localhost:8080/api/v1/locks
```

### Como configurar RBAC?

```bash
# Criar usuário com roles
curl -X POST http://localhost:8080/api/v1/auth/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "developer",
    "email": "dev@example.com",
    "roles": ["developer", "read-only"]
  }'
```

## Docker e Deployment

### Como usar Docker?

```bash
# 1. Build da imagem
docker build -t syros-platform .

# 2. Executar container
docker run -p 8080:8080 -p 9090:9090 syros-platform

# 3. Usar Docker Compose
docker-compose up -d
```

### Como fazer deploy no Kubernetes?

```bash
# 1. Aplicar manifests
kubectl apply -f k8s/

# 2. Verificar pods
kubectl get pods -l app=syros-platform

# 3. Ver logs
kubectl logs -f deployment/syros-platform
```

### Como usar Helm?

```bash
# 1. Instalar chart
helm install syros-platform ./helm/syros-platform

# 2. Atualizar
helm upgrade syros-platform ./helm/syros-platform

# 3. Desinstalar
helm uninstall syros-platform
```

## Monitoramento e Observabilidade

### Como acessar métricas?

```bash
# Métricas Prometheus
curl http://localhost:8080/metrics

# Health checks
curl http://localhost:8080/health
curl http://localhost:8080/ready
curl http://localhost:8080/live
```

### Como configurar Grafana?

1. Instale Grafana
2. Configure Prometheus como fonte de dados
3. Importe os dashboards da pasta `grafana/`
4. Visualize métricas em tempo real

### Como configurar logs estruturados?

```toml
[logging]
level = "info"
format = "json"
output = "stdout"
```

## Testes

### Como executar testes?

```bash
# Todos os testes
cargo test

# Testes de integração
cargo test --test integration_test

# Testes com output
cargo test -- --nocapture
```

### Como usar mocks para testes?

```rust
use syros_platform::mock_server::{MockServer, with_mock_server};

#[tokio::test]
async fn test_with_mock() {
    with_mock_server(|server| async move {
        // Seus testes aqui
        let response = server.rest_url();
        assert!(response.contains("http://"));
    }).await.unwrap();
}
```

## Troubleshooting

### Erro: "Porta já em uso"

```bash
# Verificar processos usando a porta
netstat -tulpn | grep :8080

# Matar processo (Linux/Mac)
sudo kill -9 $(lsof -t -i:8080)

# Matar processo (Windows)
taskkill /F /IM syros-platform.exe
```

### Erro: "Conexão recusada Redis/PostgreSQL"

```bash
# Verificar se os serviços estão rodando
docker ps | grep redis
docker ps | grep postgres

# Iniciar com Docker Compose
docker-compose up -d redis postgres
```

### Erro: "Falha na compilação"

```bash
# Atualizar Rust
rustup update

# Limpar cache
cargo clean

# Recompilar
cargo build --release
```

### Erro: "Token inválido"

```bash
# Verificar se o token está correto
echo $TOKEN

# Obter novo token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'
```

## SDKs

### Como usar o Python SDK?

```python
from syros_platform import SyrosClient

client = SyrosClient(
    base_url="http://localhost:8080",
    api_key="your-api-key"
)

# Adquirir lock
lock = client.locks.acquire(
    key="resource-123",
    ttl=300,
    owner="python-service"
)
```

### Como usar o Node.js SDK?

```javascript
const { SyrosClient } = require('@syros/platform-sdk');

const client = new SyrosClient({
  baseUrl: 'http://localhost:8080',
  apiKey: 'your-api-key'
});

// Adquirir lock
const lock = await client.locks.acquire({
  key: 'resource-123',
  ttl: 300,
  owner: 'node-service'
});
```

### Como usar o Java SDK?

```java
import com.syros.platform.SyrosClient;

SyrosClient client = new SyrosClient.Builder()
    .baseUrl("http://localhost:8080")
    .apiKey("your-api-key")
    .build();

// Adquirir lock
AcquireLockRequest request = AcquireLockRequest.builder()
    .key("resource-123")
    .ttl(300)
    .owner("java-service")
    .build();

LockResponse lock = client.locks().acquire(request);
```

## Performance

### Qual é a performance da plataforma?

- **Locks**: ~10,000 operações/segundo
- **Cache**: ~50,000 operações/segundo
- **Event Store**: ~5,000 eventos/segundo
- **gRPC**: ~20,000 requisições/segundo
- **REST API**: ~5,000 requisições/segundo

### Como otimizar performance?

1. **Use gRPC** para alta performance
2. **Configure connection pooling** adequadamente
3. **Use cache** para dados frequentemente acessados
4. **Monitore métricas** para identificar gargalos
5. **Configure retry policies** adequadamente

## Segurança

### Como configurar HTTPS?

```toml
[server]
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
```

### Como configurar CORS?

```toml
[security]
cors_origins = ["https://example.com", "https://app.example.com"]
```

### Como configurar rate limiting?

```toml
[security]
rate_limit_requests = 1000
rate_limit_window = 3600
```

## Escalabilidade

### Como escalar horizontalmente?

1. **Use load balancer** (nginx, HAProxy)
2. **Configure service discovery** (Consul)
3. **Use Redis Cluster** para locks e cache
4. **Configure PostgreSQL replication** para Event Store
5. **Use Kubernetes** para orquestração

### Como monitorar escalabilidade?

1. **Métricas de CPU e memória**
2. **Latência de requisições**
3. **Throughput por segundo**
4. **Taxa de erro**
5. **Utilização de recursos**

## Suporte

### Como obter ajuda?

1. **Consulte a documentação** em [`docs/`](docs/)
2. **Abra uma issue** no [GitHub](https://github.com/syros/platform/issues)
3. **Participe da comunidade** no Discord
4. **Consulte os exemplos** na pasta `examples/`

### Como reportar bugs?

1. **Verifique se já existe** uma issue similar
2. **Crie uma nova issue** com:
   - Descrição do problema
   - Passos para reproduzir
   - Logs relevantes
   - Versão da plataforma
   - Sistema operacional

### Como contribuir?

1. **Fork o projeto**
2. **Crie uma branch** para sua feature
3. **Implemente e teste** sua mudança
4. **Abra um Pull Request** com descrição detalhada

---

**Precisa de mais ajuda?** Consulte a [documentação completa](README.md) ou abra uma [issue](https://github.com/syros/platform/issues)!
