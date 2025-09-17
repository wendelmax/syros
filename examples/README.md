# Exemplos do Syros

Esta pasta contém exemplos práticos demonstrando como usar o Syros em diferentes cenários.

## 📁 Estrutura

```
examples/
├── basic/                  # Exemplos básicos
│   └── simple-lock/       # Lock distribuído simples
│       └── python/        # Implementação em Python
└── real-world/            # Exemplos do mundo real
    └── ecommerce/         # Sistema de e-commerce
        └── python/        # Saga pattern para pedidos
```

## 🚀 Exemplos Básicos

### Simple Lock (Python)
- **Localização**: `basic/simple-lock/python/`
- **Funcionalidades**: Locks distribuídos básicos
- **Conceitos**: Aquisição, liberação, concorrência
- **Tempo**: ~5 minutos

```bash
cd basic/simple-lock/python
pip install -r requirements.txt
python main.py
```

## 🏢 Exemplos do Mundo Real

### E-commerce com Saga Pattern (Python)
- **Localização**: `real-world/ecommerce/python/`
- **Funcionalidades**: Transações distribuídas, compensação
- **Conceitos**: Saga pattern, microserviços, auditoria
- **Tempo**: ~10 minutos

```bash
cd real-world/ecommerce/python
pip install -r requirements.txt
python main.py
```

## 📋 Pré-requisitos

Todos os exemplos assumem que o Syros está rodando:

```bash
# Via Docker Compose (recomendado)
docker-compose up -d

# Ou via script de setup
./scripts/setup.sh  # Linux/macOS
.\scripts\setup.ps1 # Windows
```

## 🎯 Por onde começar?

1. **Iniciante**: Comece com `basic/simple-lock/python/`
2. **Intermediário**: Prossiga para `real-world/ecommerce/python/`
3. **Avançado**: Explore os SDKs em diferentes linguagens

## 🔧 Configuração

### Variáveis de Ambiente

```bash
# URL base do Syros (padrão)
export SYROS_BASE_URL=http://localhost:8080

# Para exemplos gRPC
export SYROS_GRPC_URL=localhost:9090

# Para exemplos WebSocket
export SYROS_WS_URL=ws://localhost:8081/ws
```

### Docker Compose para Desenvolvimento

```yaml
version: '3.8'
services:
  syros:
    build: ../..
    ports:
      - "8080:8080"  # REST API
      - "9090:9090"  # gRPC
      - "8081:8081"  # WebSocket
    environment:
      - LOG_LEVEL=debug
    depends_on:
      - redis
      - postgres
```

## 📊 Métricas e Observabilidade

Todos os exemplos geram métricas que podem ser visualizadas:

- **Grafana**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9091
- **Logs**: `docker-compose logs -f syros`

## 🧪 Testes

Cada exemplo inclui validações automáticas:

```bash
# Executar exemplo com validações
python main.py --validate

# Executar apenas testes
python -m pytest test_*.py
```

## 🤝 Contribuindo

Para adicionar novos exemplos:

1. Crie uma pasta na estrutura apropriada
2. Inclua `README.md` com instruções claras
3. Adicione `requirements.txt` ou equivalente
4. Documente os conceitos demonstrados
5. Teste em ambiente limpo

### Template de Exemplo

```
examples/
└── categoria/
    └── nome-exemplo/
        └── linguagem/
            ├── README.md           # Documentação
            ├── main.py            # Código principal
            ├── requirements.txt   # Dependências
            └── test_exemplo.py    # Testes (opcional)
```

## 📚 Recursos Adicionais

- [Documentação da API](../../docs/api-reference.md)
- [Guia de Arquitetura](../../docs/architecture.md)
- [Padrões de Uso](../../docs/patterns.md)
- [Troubleshooting](../../docs/troubleshooting.md)

## 🆘 Suporte

Se encontrar problemas:

1. Verifique se o Syros está rodando
2. Consulte os logs: `docker-compose logs syros`
3. Verifique as portas: `netstat -tlnp | grep 8080`
4. Abra uma issue no GitHub com detalhes do erro
