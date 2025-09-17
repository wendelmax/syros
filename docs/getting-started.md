# Guia de Início Rápido

Este guia te ajudará a começar rapidamente com a Syros Platform.

## Pré-requisitos

- **Rust 1.70+** - [Instalar Rust](https://rustup.rs/)
- **Docker** (opcional) - [Instalar Docker](https://docs.docker.com/get-docker/)
- **Python 3.8+** (para testes) - [Instalar Python](https://www.python.org/downloads/)

## Instalação Rápida

### 1. Clone o Repositório

```bash
git clone https://github.com/syros/platform.git
cd platform
```

### 2. Compile o Projeto

```bash
cargo build --release
```

### 3. Configure (Opcional)

```bash
# Configure variáveis de ambiente
export SYROS_REDIS_URL="redis://localhost:6379"
export SYROS_POSTGRES_URL="postgres://localhost:5432/syros"
export SYROS_JWT_SECRET="your-secret-key"
```

### 4. Inicie o Servidor

```bash
# Iniciar todos os servidores
cargo run

# Ou iniciar apenas REST API
cargo run -- start --servers rest --host 127.0.0.1 --port 8080
```

## Primeiros Passos

### Teste Básico

```bash
# 1. Verificar se o servidor está rodando
curl http://localhost:8080/health

# Resposta esperada:
# {"status":"healthy","timestamp":"2025-09-19T10:00:00Z"}
```

### Exemplo com Lock

```bash
# 1. Adquirir um lock
curl -X POST http://localhost:8080/api/v1/locks \
  -H "Content-Type: application/json" \
  -d '{
    "key": "meu-recurso",
    "ttl": 300,
    "owner": "meu-servico"
  }'

# 2. Verificar status
curl http://localhost:8080/api/v1/locks/meu-recurso/status

# 3. Liberar lock
curl -X DELETE http://localhost:8080/api/v1/locks/meu-recurso \
  -H "Content-Type: application/json" \
  -d '{"lock_id": "lock-uuid-retornado"}'
```

### Exemplo com Cache

```bash
# 1. Armazenar no cache
curl -X POST http://localhost:8080/api/v1/cache \
  -H "Content-Type: application/json" \
  -d '{
    "key": "usuario-123",
    "value": {"nome": "João", "email": "joao@example.com"},
    "ttl": 3600
  }'

# 2. Recuperar do cache
curl http://localhost:8080/api/v1/cache/usuario-123
```

## Configuração Básica

### Arquivo de Configuração

Crie `config/default.toml`:

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
jwt_secret = "sua-chave-secreta-jwt"
api_key_encryption_key = "sua-chave-criptografia-api"
cors_origins = ["*"]

[logging]
level = "info"
format = "json"
output = "stdout"
```

## Modos de Execução

### Servidor Completo

```bash
cargo run -- start --servers all
```

### Apenas REST API

```bash
cargo run -- start --servers rest --host 127.0.0.1 --port 8080
```

### REST + gRPC

```bash
cargo run -- start --servers rest,grpc --host 0.0.0.0 --port 8080 --grpc-port 9090
```

### Modo Verbose

```bash
cargo run -- --verbose start --servers all
```

### Modo Quiet

```bash
cargo run -- --quiet start --servers rest
```

## Verificar Status

### Health Checks

```bash
# Health básico
curl http://localhost:8080/health

# Health detalhado
curl http://localhost:8080/ready

# Health para Kubernetes
curl http://localhost:8080/live
```

### Métricas

```bash
# Ver métricas Prometheus
curl http://localhost:8080/metrics
```

## Testes

### Executar Todos os Testes

```bash
cargo test
```

### Executar Testes de Integração

```bash
cargo test --test integration_test
```

### Executar com Cobertura

```bash
cargo test -- --nocapture
```

## Docker (Opcional)

### Usando Docker Compose

```bash
# Iniciar com Docker Compose
docker-compose up -d

# Verificar logs
docker-compose logs -f syros-platform

# Parar
docker-compose down
```

### Usando Docker

```bash
# Build da imagem
docker build -t syros-platform .

# Executar container
docker run -p 8080:8080 -p 9090:9090 syros-platform
```

## Próximos Passos

Agora que você tem a Syros Platform rodando:

1. **Explore as APIs**: Veja [REST API](rest-api.md), [gRPC API](grpc-api.md), [WebSocket API](websocket-api.md)
2. **Use os SDKs**: Consulte [SDKs](sdks.md) para sua linguagem preferida
3. **Configure Observabilidade**: Veja [Observabilidade](observability.md)
4. **Deploy em Produção**: Consulte [Deployment](deployment.md)

## Problemas Comuns

### Porta já em uso

```bash
# Verificar processos usando a porta
netstat -tulpn | grep :8080

# Matar processo (Linux/Mac)
sudo kill -9 $(lsof -t -i:8080)

# Matar processo (Windows)
taskkill /F /IM syros-platform.exe
```

### Erro de conexão Redis/PostgreSQL

```bash
# Verificar se os serviços estão rodando
docker ps | grep redis
docker ps | grep postgres

# Iniciar com Docker Compose
docker-compose up -d redis postgres
```

### Erro de compilação

```bash
# Atualizar Rust
rustup update

# Limpar cache
cargo clean

# Recompilar
cargo build --release
```

## Recursos Adicionais

- [Arquitetura da Plataforma](architecture.md)
- [Configuração Avançada](configuration.md)
- [FAQ](faq.md)
- [Changelog](../CHANGELOG.md)

---

**Precisa de ajuda?** Abra uma [issue](https://github.com/syros/platform/issues) ou consulte a [FAQ](faq.md)!
