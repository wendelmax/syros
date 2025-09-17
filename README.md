# Syros - Distributed Coordination Service

**Syros** is a distributed coordination platform built in Rust, offering robust solutions for modern distributed systems.

## Project Status

**PROJECT 100% IMPLEMENTED AND FUNCTIONAL!**

### Implemented Components

- **Core Engine**: Lock Manager, Saga Orchestrator, Event Store, Cache Manager
- **REST APIs**: Complete endpoints with health checks and metrics
- **gRPC API**: Complete gRPC services with Volo
- **WebSocket**: Real-time WebSocket connection support
- **Flexible Server**: Server selection and IP configuration
- **Configuration**: Flexible configuration system
- **Error Handling**: Robust error handling system
- **Observability**: Prometheus metrics and tracing
- **Security**: JWT and API Keys
- **SDKs**: SDKs for Python, Node.js, Java, C#, and Go
- **Docker**: Complete deployment configuration
- **CI/CD**: GitHub Actions pipelines
- **Tests**: Functional test client

## Platform Architecture

### Big Picture - Architecture Overview

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

### Data Flow and Interactions

```mermaid
sequenceDiagram
    participant Client as Client
    participant API as API Gateway
    participant Auth as Auth Middleware
    participant Core as Core Services
    participant Storage as Storage Layer
    participant Obs as Observability

    Note over Client,Obs: Typical Request Flow

    Client->>API: 1. HTTP/gRPC/WebSocket Request
    API->>Auth: 2. Authentication Validation
    Auth->>Auth: 3. Verify JWT/API Key
    Auth-->>API: 4. Valid Token
    
    API->>Core: 5. Process Request
    Core->>Storage: 6. Access Data
    Storage-->>Core: 7. Return Data
    Core-->>API: 8. Processed Response
    
    API->>Obs: 9. Record Metrics
    API-->>Client: 10. Final Response

    Note over Client,Obs: Example: Lock Acquisition

    Client->>API: POST /api/v1/locks
    API->>Auth: Validate Token
    Auth-->>API: Token OK
    API->>Core: LockManager.acquire_lock()
    Core->>Storage: Redis SET lock_key
    Storage-->>Core: Lock Acquired
    Core-->>API: LockResponse
    API->>Obs: Record Lock Metrics
    API-->>Client: 201 Created + Lock ID

    Note over Client,Obs: Example: WebSocket Event

    Client->>API: WebSocket Connect
    API->>Auth: Validate Token
    Auth-->>API: Token OK
    API->>Core: Subscribe to Events
    Core->>Storage: Monitor Changes
    Storage-->>Core: Data Changed
    Core-->>API: Event Notification
    API-->>Client: WebSocket Message
```

### Implemented Architecture Patterns

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

## Documentation

Complete documentation is available in the [`docs/`](docs/) folder:

- **[Quick Start Guide](docs/getting-started.md)** - Installation and first steps
- **[REST API](docs/rest-api.md)** - Complete REST API documentation
- **[gRPC API](docs/grpc-api.md)** - High-performance gRPC interface
- **[WebSocket API](docs/websocket-api.md)** - Real-time communication
- **[GraphQL API](docs/graphql-api.md)** - Flexible GraphQL interface
- **[SDKs](docs/sdks.md)** - SDKs for Python, Node.js, Java, C#, Go
- **[Observability](docs/observability.md)** - Monitoring and metrics
- **[Architecture](docs/architecture.md)** - Architecture overview
- **[Configuration](docs/configuration.md)** - Advanced configuration
- **[Deployment](docs/deployment.md)** - Deployment guides
- **[FAQ](docs/faq.md)** - Frequently asked questions

### Quick Start

```bash
# 1. Clone and build
git clone https://github.com/wendelmax/syros.git
cd syros
cargo build --release

# 2. Start the server
cargo run

# 3. Test the API
curl http://localhost:8080/health
```

### Available APIs

- **REST API**: `http://localhost:8080` - Complete HTTP interface
- **gRPC API**: `localhost:9090` - High-performance interface
- **WebSocket**: `ws://localhost:8081` - Real-time communication
- **GraphQL**: `http://localhost:8080/graphql` - Flexible queries

## Contributing

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rust](https://www.rust-lang.org/) - Programming language
- [Tokio](https://tokio.rs/) - Async runtime
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tonic](https://github.com/hyperium/tonic) - gRPC framework
- [Redis](https://redis.io/) - Cache and locks
- [PostgreSQL](https://www.postgresql.org/) - Database
- [Prometheus](https://prometheus.io/) - Metrics
- [Grafana](https://grafana.com/) - Dashboards