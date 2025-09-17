# Documentação Syros Platform

Bem-vindo à documentação completa da Syros Platform! Esta pasta contém toda a documentação detalhada para usar a plataforma.

## Índice da Documentação

### [Guia de Início Rápido](getting-started.md)
- Instalação e configuração
- Primeiros passos
- Exemplos básicos

### [REST API](rest-api.md)
- Autenticação e autorização
- Gerenciamento de locks
- Orquestração de sagas
- Event Store
- Cache distribuído
- Exemplos práticos com curl

### [gRPC API](grpc-api.md)
- Configuração de clientes
- Exemplos em Python, Node.js, Java, C#, Go
- Protocol Buffers
- Streaming e performance

### [WebSocket API](websocket-api.md)
- Conexão e autenticação
- Eventos em tempo real
- Subscrições
- Exemplos JavaScript

### [GraphQL API](graphql-api.md)
- Schema e tipos
- Queries e mutations
- Apollo Client
- Exemplos práticos

### [SDKs](sdks.md)
- Python SDK
- Node.js SDK
- Java SDK
- C# SDK
- Go SDK
- Exemplos de uso

### [Observabilidade](observability.md)
- Métricas Prometheus
- Health checks
- Logs estruturados
- Dashboards Grafana
- Tracing Jaeger

### [Arquitetura](architecture.md)
- Visão geral da arquitetura
- Padrões implementados
- Fluxo de dados
- Diagramas Mermaid

### [Configuração](configuration.md)
- Arquivos de configuração
- Variáveis de ambiente
- Configuração por ambiente
- Exemplos de configuração

### [Deployment](deployment.md)
- Docker
- Kubernetes
- Helm Charts
- CI/CD
- Produção

### [Testes](testing.md)
- Testes unitários
- Testes de integração
- Mock servers
- Exemplos de teste

### [FAQ](faq.md)
- Perguntas frequentes
- Troubleshooting
- Problemas comuns
- Soluções

## Como Usar Esta Documentação

1. **Comece com o [Guia de Início Rápido](getting-started.md)** se você é novo na plataforma
2. **Escolha a API que melhor se adequa ao seu caso de uso**:
   - REST para integrações web simples
   - gRPC para alta performance
   - WebSocket para tempo real
   - GraphQL para consultas flexíveis
3. **Use os SDKs** para integração mais fácil em sua linguagem preferida
4. **Configure a observabilidade** para monitorar sua aplicação
5. **Consulte a FAQ** se tiver dúvidas

## Links Úteis

- [Repositório GitHub](https://github.com/syros/platform)
- [Changelog](../CHANGELOG.md)
- [Deployment Guide](../DEPLOYMENT.md)
- [Configuração Padrão](../config/default.toml)

## Contribuindo

Encontrou um erro na documentação? Tem uma sugestão de melhoria?

1. Abra uma [issue](https://github.com/syros/platform/issues)
2. Faça um [pull request](https://github.com/syros/platform/pulls)
3. Entre em contato conosco

---

**Última atualização**: 19 de setembro de 2025
