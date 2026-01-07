# Syros - Distributed Coordination Service

**Syros** is a distributed coordination platform built in Rust, offering robust solutions for modern distributed systems.

## Project Status

### Implemented Components

- **Core Engine**: Lock Manager, Saga Orchestrator, Event Store, Cache Manager
- **REST APIs**: Complete endpoints with health checks and metrics
- **gRPC API**: Complete gRPC services with Volo
- **WebSocket**: Real-time WebSocket connection support
- **Flexible Server**: Server selection and IP configuration
- **Configuration**: Flexible configuration system using environment variables or TOML
- **Error Handling**: Robust error handling with descriptive codes
- **Observability**: Built-in Prometheus metrics and structured logging
- **Security**: JWT-based authentication and API Key authorization
- **SDKs**: Native SDKs for Python, Node.js, Java, C#, and Go
- **Infrastructure**: Ready-to-use Docker and Kubernetes (Helm) configurations

## Key Features

### Distributed Lock Manager
Syros provides a high-performance, distributed lock manager using Redis as the backend. It supports lock acquisition with TTL, wait timeouts, and ownership verification, ensuring mutual exclusion across a cluster of services.

### Saga Orchestrator
Implement complex distributed transactions using the Saga pattern. Syros manages step execution, retries with customizable backoff strategies, and automatic compensation logic (rollback) if a step fails, maintaining eventual consistency in your microservices.

### Event Store
A persistent event sourcing engine powered by PostgreSQL. Append events to streams, retrieve event history with versioning, and build reactive systems by replaying events. Includes support for metadata and automatic versioning.

### Distributed Cache
A fast, tag-based distributed cache. Optimize your system by caching expensive computations, with support for TTL-based expiration and bulk invalidation by tags.

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

### Prerequisites

-   **Rust**: Stable 1.70+
-   **Docker & Docker Compose**: For backing services
-   **Services**: Redis (Locks/Cache) and PostgreSQL (Saga/Events)

### Quick Start

The fastest way to get Syros running with all its dependencies is via Docker Compose:

```bash
# 1. Clone the repository
git clone https://github.com/wendelmax/syros.git
cd syros

# 2. Build and start all services
docker-compose up -d --build
```

Access the health check to verify everything is working:
```bash
curl http://localhost:8080/health
```

### Usage Examples

#### Acquire a Distributed Lock
```bash
curl -X POST http://localhost:8080/api/v1/locks \
  -H "Content-Type: application/json" \
  -d '{
    "key": "inventory_process_123",
    "owner": "worker_service_01",
    "ttl_seconds": 30
  }'
```

#### Start a Distributed Saga
```bash
curl -X POST http://localhost:8080/api/v1/sagas \
  -H "Content-Type: application/json" \
  -d '{
    "name": "order_fulfillment_saga",
    "steps": [
      {
        "name": "reserve_inventory",
        "service": "inventory_service",
        "action": "reserve",
        "compensation": "release_inventory",
        "timeout_seconds": 10
      },
      {
        "name": "charge_payment",
        "service": "payment_service",
        "action": "charge",
        "compensation": "refund_payment",
        "timeout_seconds": 15
      }
    ]
  }'
```

#### Append an Event to a Stream
```bash
curl -X POST http://localhost:8080/api/v1/events/order_stream_99 \
  -H "Content-Type: application/json" \
  -d '{
    "event_type": "OrderCreated",
    "data": { "customer_id": "user_456", "total": 129.90 },
    "metadata": { "source": "mobile_app" }
  }'
```

## Configuration

Syros can be configured via environment variables or a `config.toml` file.

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://syros:syros@localhost:5432/syros` |
| `REDIS_URL` | Redis connection string | `redis://localhost:6379` |
| `SERVER_HOST` | Host interface to bind to | `0.0.0.0` |
| `REST_PORT` | Port for the REST API | `8080` |
| `GRPC_PORT` | Port for the gRPC API | `9090` |
| `WS_PORT` | Port for the WebSocket API | `8081` |
| `JWT_SECRET` | Secret key for JWT generation | `syros-secret-key-change-me` |
| `LOG_LEVEL` | Logging level (info, debug, error) | `info` |

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
- [Volo](https://github.com/cloudwego/volo) - gRPC framework
- [Redis](https://redis.io/) - Cache and locks
- [PostgreSQL](https://www.postgresql.org/) - Database
- [Prometheus](https://prometheus.io/) - Metrics
- [Grafana](https://grafana.com/) - Dashboards