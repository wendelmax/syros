# Platform Architecture

## Big Picture - Architecture Overview

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

## Data Flow and Interactions

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

## Implemented Architecture Patterns

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

## Main Components

### **API Gateway Layer**
- **REST API**: HTTP interface for web integration
- **gRPC API**: High-performance interface for microservices
- **WebSocket API**: Real-time communication
- **GraphQL API**: Flexible interface for complex queries

### **Core Services Layer**
- **Lock Manager**: Distributed lock management
- **Saga Orchestrator**: Distributed transaction orchestration
- **Event Store**: Event storage and replay
- **Cache Manager**: Distributed cache with TTL
- **Service Discovery**: Automatic service discovery
- **RBAC Manager**: Role-based access control

### **Middleware Layer**
- **Auth Middleware**: JWT and API Key authentication
- **Rate Limiting**: Request rate control
- **CORS Handler**: Cross-origin request support
- **Metrics Collector**: Metrics collection for observability

### **Storage Layer**
- **Redis**: Lock and cache storage
- **PostgreSQL**: Event and metadata persistence
- **etcd**: Distributed configuration

### **Observability Layer**
- **Prometheus**: Metrics collection and storage
- **Grafana**: Dashboards and visualizations
- **Jaeger**: Distributed tracing
- **Structured Logging**: JSON structured logs

### **Infrastructure Layer**
- **Docker**: Application containerization
- **Kubernetes**: Container orchestration
- **Helm Charts**: Deployment management
- **GitHub Actions**: CI/CD pipeline

### **SDK Layer**
- **Python SDK**: For Python applications
- **Node.js SDK**: For JavaScript/TypeScript applications
- **Java SDK**: For Java applications
- **C# SDK**: For .NET applications
- **Go SDK**: For Go applications

## Architectural Decisions

### Why Rust?
- **Performance**: Zero-cost abstractions and memory safety
- **Concurrency**: Native async/await with tokio
- **Reliability**: Compile-time bug prevention
- **Ecosystem**: Mature crate ecosystem for distributed systems

### Why Multiple APIs?
- **REST**: Simplicity and universal compatibility
- **gRPC**: Performance and type safety for microservices
- **WebSocket**: Real-time communication
- **GraphQL**: Flexibility for complex frontends

### Why Event Sourcing?
- **Audit**: Complete history of changes
- **Replay**: Ability to rebuild state
- **Debugging**: Problem traceability
- **Integration**: Events as source of truth

### Why Saga Pattern?
- **Distributed Transactions**: Coordination between services
- **Compensation**: Automatic rollback on failure
- **Resilience**: Retry policies and circuit breakers
- **Observability**: Transaction state visibility

## Scalability

### Horizontal Scaling
- **Stateless Services**: Services without internal state
- **Load Balancing**: Automatic load distribution
- **Service Discovery**: Automatic instance discovery
- **Data Partitioning**: Data partitioning by key

### Vertical Scaling
- **Async Processing**: Non-blocking async processing
- **Connection Pooling**: Optimized connection pool
- **Memory Management**: Efficient memory management
- **CPU Optimization**: CPU-specific optimizations

## Security

### Authentication
- **JWT**: Stateless and secure tokens
- **API Keys**: Service-to-service authentication
- **OAuth2**: Integration with external providers
- **mTLS**: Mutual authentication for internal communication

### Authorization
- **RBAC**: Role-based control
- **Permissions**: Granular permissions
- **Resource-based**: Per-resource authorization
- **Time-based**: Automatic permission expiration

### Encryption
- **TLS**: Encrypted communication
- **Data at Rest**: Stored data encryption
- **Key Management**: Secure key management
- **Hashing**: Secure password and token hashing

## Monitoring

### Metrics
- **Business Metrics**: Business metrics
- **Technical Metrics**: Technical metrics
- **Custom Metrics**: Custom metrics
- **Alerting**: Metrics-based alerts

### Logs
- **Structured Logging**: JSON format logs
- **Correlation IDs**: Request tracking
- **Log Levels**: Configurable log levels
- **Log Aggregation**: Centralized aggregation

### Tracing
- **Distributed Tracing**: Distributed tracking
- **Span Correlation**: Span correlation
- **Performance Analysis**: Performance analysis
- **Error Tracking**: Error tracking

## Resilience

### Fault Tolerance
- **Circuit Breakers**: Protection against cascade failures
- **Retry Policies**: Configurable retry policies
- **Timeout Handling**: Timeout handling
- **Graceful Degradation**: Graceful degradation

### Disaster Recovery
- **Backup Strategy**: Backup strategy
- **Data Replication**: Data replication
- **Failover**: Automatic failover
- **Recovery Procedures**: Recovery procedures

---

**Next**: [Configuration](configuration.md) | [Deployment](deployment.md) | [Observability](observability.md)