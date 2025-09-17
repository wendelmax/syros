# Arquitetura da Plataforma

## Big Picture - Visão Geral da Arquitetura

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

## Fluxo de Dados e Interações

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

## Padrões de Arquitetura Implementados

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

## Componentes Principais

### **API Gateway Layer**
- **REST API**: Interface HTTP para integração web
- **gRPC API**: Interface de alta performance para microserviços
- **WebSocket API**: Comunicação em tempo real
- **GraphQL API**: Interface flexível para consultas complexas

### **Core Services Layer**
- **Lock Manager**: Gerenciamento de locks distribuídos
- **Saga Orchestrator**: Orquestração de transações distribuídas
- **Event Store**: Armazenamento e replay de eventos
- **Cache Manager**: Cache distribuído com TTL
- **Service Discovery**: Descoberta automática de serviços
- **RBAC Manager**: Controle de acesso baseado em roles

### **Middleware Layer**
- **Auth Middleware**: Autenticação JWT e API Keys
- **Rate Limiting**: Controle de taxa de requisições
- **CORS Handler**: Suporte a requisições cross-origin
- **Metrics Collector**: Coleta de métricas para observabilidade

### **Storage Layer**
- **Redis**: Armazenamento de locks e cache
- **PostgreSQL**: Persistência de eventos e metadados
- **etcd**: Configuração distribuída

### **Observability Layer**
- **Prometheus**: Coleta e armazenamento de métricas
- **Grafana**: Dashboards e visualizações
- **Jaeger**: Rastreamento distribuído
- **Structured Logging**: Logs estruturados em JSON

### **Infrastructure Layer**
- **Docker**: Containerização da aplicação
- **Kubernetes**: Orquestração de containers
- **Helm Charts**: Gerenciamento de deployments
- **GitHub Actions**: Pipeline de CI/CD

### **SDK Layer**
- **Python SDK**: Para aplicações Python
- **Node.js SDK**: Para aplicações JavaScript/TypeScript
- **Java SDK**: Para aplicações Java
- **C# SDK**: Para aplicações .NET
- **Go SDK**: Para aplicações Go

## Decisões Arquiteturais

### Por que Rust?
- **Performance**: Zero-cost abstractions e memory safety
- **Concorrência**: Async/await nativo com tokio
- **Confiabilidade**: Prevenção de bugs em tempo de compilação
- **Ecosistema**: Crate ecosystem maduro para sistemas distribuídos

### Por que Múltiplas APIs?
- **REST**: Simplicidade e compatibilidade universal
- **gRPC**: Performance e type safety para microserviços
- **WebSocket**: Comunicação em tempo real
- **GraphQL**: Flexibilidade para frontends complexos

### Por que Event Sourcing?
- **Auditoria**: Histórico completo de mudanças
- **Replay**: Capacidade de reconstruir estado
- **Debugging**: Rastreabilidade de problemas
- **Integração**: Eventos como fonte de verdade

### Por que Saga Pattern?
- **Transações Distribuídas**: Coordenação entre serviços
- **Compensação**: Rollback automático em caso de falha
- **Resiliência**: Retry policies e circuit breakers
- **Observabilidade**: Visibilidade do estado das transações

## Escalabilidade

### Horizontal Scaling
- **Stateless Services**: Serviços sem estado interno
- **Load Balancing**: Distribuição de carga automática
- **Service Discovery**: Descoberta automática de instâncias
- **Data Partitioning**: Particionamento de dados por chave

### Vertical Scaling
- **Async Processing**: Processamento assíncrono não-bloqueante
- **Connection Pooling**: Pool de conexões otimizado
- **Memory Management**: Gerenciamento eficiente de memória
- **CPU Optimization**: Otimizações específicas para CPU

## Segurança

### Autenticação
- **JWT**: Tokens stateless e seguros
- **API Keys**: Autenticação service-to-service
- **OAuth2**: Integração com provedores externos
- **mTLS**: Autenticação mútua para comunicação interna

### Autorização
- **RBAC**: Controle baseado em roles
- **Permissions**: Permissões granulares
- **Resource-based**: Autorização por recurso
- **Time-based**: Expiração automática de permissões

### Criptografia
- **TLS**: Comunicação criptografada
- **Data at Rest**: Criptografia de dados armazenados
- **Key Management**: Gerenciamento seguro de chaves
- **Hashing**: Hash seguro de senhas e tokens

## Monitoramento

### Métricas
- **Business Metrics**: Métricas de negócio
- **Technical Metrics**: Métricas técnicas
- **Custom Metrics**: Métricas personalizadas
- **Alerting**: Alertas baseados em métricas

### Logs
- **Structured Logging**: Logs em formato JSON
- **Correlation IDs**: Rastreamento de requisições
- **Log Levels**: Níveis de log configuráveis
- **Log Aggregation**: Agregação centralizada

### Tracing
- **Distributed Tracing**: Rastreamento distribuído
- **Span Correlation**: Correlação de spans
- **Performance Analysis**: Análise de performance
- **Error Tracking**: Rastreamento de erros

## Resiliência

### Fault Tolerance
- **Circuit Breakers**: Proteção contra falhas em cascata
- **Retry Policies**: Políticas de retry configuráveis
- **Timeout Handling**: Tratamento de timeouts
- **Graceful Degradation**: Degradação graciosa

### Disaster Recovery
- **Backup Strategy**: Estratégia de backup
- **Data Replication**: Replicação de dados
- **Failover**: Failover automático
- **Recovery Procedures**: Procedimentos de recuperação

---

**Próximo**: [Configuração](configuration.md) | [Deployment](deployment.md) | [Observabilidade](observability.md)
