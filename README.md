# Syros Platform - Distributed Coordination Service

**Syros** é uma plataforma de coordenação distribuída construída em Rust, oferecendo soluções robustas para sistemas distribuídos modernos.

## Status do Projeto

**PROJETO 100% IMPLEMENTADO E FUNCIONAL!**

### Componentes Implementados

- **Core Engine**: Lock Manager, Saga Orchestrator, Event Store, Cache Manager
- **APIs REST**: Endpoints completos com health checks e métricas
- **gRPC API**: Serviços gRPC completos com Volo
- **WebSocket**: Suporte a conexões WebSocket em tempo real
- **Servidor Flexível**: Seleção de servidores e configuração de IP
- **Configuração**: Sistema de configuração flexível
- **Tratamento de Erros**: Sistema robusto de tratamento de erros
- **Observabilidade**: Métricas Prometheus e tracing
- **Segurança**: JWT e API Keys
- **SDKs**: SDKs para Python, Node.js, Java, C# e Go
- **Docker**: Configuração completa para deployment
- **CI/CD**: Pipelines GitHub Actions
- **Testes**: Cliente de teste funcional

## Arquitetura da Plataforma

### Big Picture - Visão Geral da Arquitetura

```mermaid
---
config:
  layout: elk
---
flowchart TB
 subgraph subGraph0["Client Applications"]
        WEB["Web Applications"]
        MOBILE["Mobile Apps"]
        CLI["CLI Tools"]
        SDK["SDK Clients"]
  end
  subgraph subGraph1["API Gateway Layer"]
        REST["REST API<br>Port 8080"]
        GRPC["gRPC API<br>Port 9090"]
        WS["WebSocket API<br>Port 8081"]
        GRAPHQL["GraphQL API<br>Port 8080/graphql"]
  end
  subgraph subGraph2["Core Services Layer"]
        LM["Lock Manager<br>Distributed Locks"]
        SO["Saga Orchestrator<br>Distributed Transactions"]
        ES["Event Store<br>Event Sourcing"]
        CM["Cache Manager<br>Distributed Cache"]
        SD["Service Discovery<br>Consul Integration"]
        RBAC["RBAC Manager<br>Role-Based Access Control"]
  end
  subgraph subGraph3["Middleware Layer"]
        AUTH["Auth Middleware<br>JWT &amp; API Keys"]
        RATE["Rate Limiting<br>Request Throttling"]
        CORS["CORS Handler<br>Cross-Origin Support"]
        METRICS["Metrics Collector<br>Prometheus Integration"]
  end
  subgraph subGraph4["Storage Layer"]
        REDIS[("Redis<br>Locks &amp; Cache")]
        POSTGRES[("PostgreSQL<br>Events &amp; Metadata")]
        ETCD[("etcd<br>Configuration")]
  end
  subgraph subGraph5["Observability Layer"]
        PROMETHEUS["Prometheus<br>Metrics Collection"]
        GRAFANA["Grafana<br>Dashboards"]
        JAEGER["Jaeger<br>Distributed Tracing"]
        LOGS["Structured Logging<br>JSON Format"]
  end
  subgraph subGraph6["Infrastructure Layer"]
        DOCKER["Docker Containers"]
        K8S["Kubernetes<br>Orchestration"]
        HELM["Helm Charts<br>Deployment"]
        CI["GitHub Actions<br>CI/CD Pipeline"]
  end
  subgraph subGraph7["SDK Layer"]
        PYTHON["Python SDK"]
        NODE["Node.js SDK"]
        JAVA["Java SDK"]
        CSHARP["C# SDK"]
        GO["Go SDK"]
  end
    WEB --> REST & WS
    MOBILE --> REST & WS
    CLI --> REST
    SDK --> REST & GRPC
    REST --> AUTH
    GRPC --> AUTH
    WS --> AUTH
    GRAPHQL --> AUTH
    AUTH --> RATE
    RATE --> CORS
    CORS --> METRICS
    METRICS --> LM & SO & ES & CM & SD & RBAC & PROMETHEUS
    LM --> REDIS
    CM --> REDIS
    ES --> POSTGRES
    SO --> POSTGRES
    SD --> ETCD
    RBAC --> POSTGRES
    PROMETHEUS --> GRAFANA
    LOGS --> JAEGER
    DOCKER --> K8S
    K8S --> HELM
    CI --> DOCKER
    PYTHON --> REST
    NODE --> REST
    JAVA --> GRPC
    CSHARP --> GRPC
    GO --> GRPC
     WEB:::clientLayer
     MOBILE:::clientLayer
     CLI:::clientLayer
     SDK:::clientLayer
     REST:::apiLayer
     GRPC:::apiLayer
     WS:::apiLayer
     GRAPHQL:::apiLayer
     LM:::coreLayer
     SO:::coreLayer
     ES:::coreLayer
     CM:::coreLayer
     SD:::coreLayer
     RBAC:::coreLayer
     AUTH:::middlewareLayer
     RATE:::middlewareLayer
     CORS:::middlewareLayer
     METRICS:::middlewareLayer
     REDIS:::storageLayer
     POSTGRES:::storageLayer
     ETCD:::storageLayer
     PROMETHEUS:::observabilityLayer
     GRAFANA:::observabilityLayer
     JAEGER:::observabilityLayer
     LOGS:::observabilityLayer
     DOCKER:::infrastructureLayer
     K8S:::infrastructureLayer
     HELM:::infrastructureLayer
     CI:::infrastructureLayer
     PYTHON:::sdkLayer
     NODE:::sdkLayer
     JAVA:::sdkLayer
     CSHARP:::sdkLayer
     GO:::sdkLayer
    classDef clientLayer fill:#e1f5fe
    classDef apiLayer fill:#f3e5f5
    classDef coreLayer fill:#e8f5e8
    classDef middlewareLayer fill:#fff3e0
    classDef storageLayer fill:#fce4ec
    classDef observabilityLayer fill:#f1f8e9
    classDef infrastructureLayer fill:#e0f2f1
    classDef sdkLayer fill:#fff8e1
```

### Fluxo de Dados e Interações

```mermaid
sequenceDiagram
    participant Client as Cliente
    participant API as API Gateway
    participant Auth as Auth Middleware
    participant Core as Core Services
    participant Storage as Storage Layer
    participant Obs as Observability

    Note over Client,Obs: Fluxo de Requisição Típica

    Client->>API: 1. Requisição HTTP/gRPC/WebSocket
    API->>Auth: 2. Validação de Autenticação
    Auth->>Auth: 3. Verificar JWT/API Key
    Auth-->>API: 4. Token Válido
    
    API->>Core: 5. Processar Requisição
    Core->>Storage: 6. Acessar Dados
    Storage-->>Core: 7. Retornar Dados
    Core-->>API: 8. Resposta Processada
    
    API->>Obs: 9. Registrar Métricas
    API-->>Client: 10. Resposta Final

    Note over Client,Obs: Exemplo: Aquisição de Lock

    Client->>API: POST /api/v1/locks
    API->>Auth: Validar Token
    Auth-->>API: Token OK
    API->>Core: LockManager.acquire_lock()
    Core->>Storage: Redis SET lock_key
    Storage-->>Core: Lock Acquired
    Core-->>API: LockResponse
    API->>Obs: Record Lock Metrics
    API-->>Client: 201 Created + Lock ID

    Note over Client,Obs: Exemplo: WebSocket Event

    Client->>API: WebSocket Connect
    API->>Auth: Validar Token
    Auth-->>API: Token OK
    API->>Core: Subscribe to Events
    Core->>Storage: Monitor Changes
    Storage-->>Core: Data Changed
    Core-->>API: Event Notification
    API-->>Client: WebSocket Message
```

### Padrões de Arquitetura Implementados

```mermaid
graph LR
    subgraph "Distributed Patterns"
        SAGA[Saga Pattern<br/>Distributed Transactions]
        LOCK[Distributed Locks<br/>Mutual Exclusion]
        EVENT[Event Sourcing<br/>Audit Trail]
        CACHE[Distributed Cache<br/>Performance]
    end

    subgraph "Communication Patterns"
        REST[REST API<br/>HTTP/JSON]
        GRPC[gRPC<br/>High Performance]
        WS[WebSocket<br/>Real-time]
        GRAPHQL[GraphQL<br/>Flexible Queries]
    end

    subgraph "Observability Patterns"
        METRICS[Metrics<br/>Prometheus]
        TRACING[Tracing<br/>Jaeger]
        LOGGING[Structured Logging<br/>JSON]
        HEALTH[Health Checks<br/>K8s Ready]
    end

    subgraph "Security Patterns"
        JWT[JWT Authentication<br/>Stateless]
        RBAC[RBAC Authorization<br/>Role-based]
        APIKEY[API Keys<br/>Service-to-Service]
        CORS[CORS<br/>Cross-Origin]
    end

    subgraph "Infrastructure Patterns"
        CONTAINER[Containerization<br/>Docker]
        ORCHESTRATION[Orchestration<br/>Kubernetes]
        CI[CI/CD<br/>GitHub Actions]
        HELM[Package Management<br/>Helm Charts]
    end

    SAGA --> REST
    LOCK --> GRPC
    EVENT --> WS
    CACHE --> GRAPHQL

    REST --> METRICS
    GRPC --> TRACING
    WS --> LOGGING
    GRAPHQL --> HEALTH

    METRICS --> JWT
    TRACING --> RBAC
    LOGGING --> APIKEY
    HEALTH --> CORS

    JWT --> CONTAINER
    RBAC --> ORCHESTRATION
    APIKEY --> CI
    CORS --> HELM
```

## Documentação

A documentação completa está disponível na pasta [`docs/`](docs/):

- **[Guia de Início Rápido](docs/getting-started.md)** - Instalação e primeiros passos
- **[REST API](docs/rest-api.md)** - Documentação completa da API REST
- **[gRPC API](docs/grpc-api.md)** - Interface gRPC de alta performance
- **[WebSocket API](docs/websocket-api.md)** - Comunicação em tempo real
- **[GraphQL API](docs/graphql-api.md)** - Interface GraphQL flexível
- **[SDKs](docs/sdks.md)** - SDKs para Python, Node.js, Java, C#, Go
- **[Observabilidade](docs/observability.md)** - Monitoramento e métricas
- **[Arquitetura](docs/architecture.md)** - Visão geral da arquitetura
- **[Configuração](docs/configuration.md)** - Configuração avançada
- **[Deployment](docs/deployment.md)** - Guias de deployment
- **[FAQ](docs/faq.md)** - Perguntas frequentes

### Início Rápido

```bash
# 1. Clone e compile
git clone https://github.com/syros/platform.git
cd platform
cargo build --release

# 2. Inicie o servidor
cargo run

# 3. Teste a API
curl http://localhost:8080/health
```

### APIs Disponíveis

- **REST API**: `http://localhost:8080` - Interface HTTP completa
- **gRPC API**: `localhost:9090` - Interface de alta performance
- **WebSocket**: `ws://localhost:8081` - Comunicação em tempo real
- **GraphQL**: `http://localhost:8080/graphql` - Consultas flexíveis

## Contribuindo

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Agradecimentos

- [Rust](https://www.rust-lang.org/) - Linguagem de programação
- [Tokio](https://tokio.rs/) - Runtime assíncrono
- [Axum](https://github.com/tokio-rs/axum) - Framework web
- [Tonic](https://github.com/hyperium/tonic) - Framework gRPC
- [Redis](https://redis.io/) - Cache e locks
- [PostgreSQL](https://www.postgresql.org/) - Banco de dados
- [Prometheus](https://prometheus.io/) - Métricas
- [Grafana](https://grafana.com/) - Dashboards