# Demonstração dos Modos de Execução

## 🎯 Resposta à Pergunta

**As informações detalhadas agora aparecem apenas quando você usa flags específicas!**

### 📋 Modos Disponíveis

#### 1. **Modo Normal** (padrão)
```bash
cargo run
```
**Saída:**
```
🚀 Syros - Distributed Coordination Service
📦 Versão: 1.0.0
🌐 Servidor iniciado em http://0.0.0.0:8080
```

#### 2. **Modo Verbose** (`-v` ou `--verbose`)
```bash
cargo run -- --verbose
```
**Saída:**
```
🚀 Syros - Distributed Coordination Service
📦 Versão: 1.0.0
🔧 Ambiente: Desenvolvimento
🔧 Modo: Verbose
🔧 Configuração: config/default.toml
🚀 Iniciando Syros...
📋 Configuração carregada:
   - Servidor: 0.0.0.0:8080
   - gRPC: 0.0.0.0:9090
   - WebSocket: 0.0.0.0:8081
✅ Componentes core inicializados
🌐 Servidor iniciado em http://0.0.0.0:8080
📖 Documentação disponível em:
   - Health: http://0.0.0.0:8080/health
   - Ready: http://0.0.0.0:8080/ready
   - API: http://0.0.0.0:8080/api/v1/
```

#### 3. **Modo Quiet** (`-q` ou `--quiet`)
```bash
cargo run -- --quiet
```
**Saída:**
```
🌐 Servidor iniciado em http://0.0.0.0:8080
```

#### 4. **Comando Info**
```bash
cargo run -- info
```
**Saída:**
```
🚀 Syros - Distributed Coordination Service
📦 Versão: 1.0.0
🔧 Ambiente: Desenvolvimento
🔧 Rust: Unknown
🔧 Target: Unknown
```

#### 5. **Comando Config**
```bash
cargo run -- config --validate
```
**Saída:**
```
🔧 Verificando configuração...
✅ Configuração válida!
```

### 🎛️ Opções Disponíveis

```bash
# Ajuda completa
cargo run -- --help

# Versão
cargo run -- --version

# Arquivo de configuração personalizado
cargo run -- --config my-config.toml

# Modo verbose com configuração personalizada
cargo run -- --verbose --config production.toml
```

### 🔧 Comandos Disponíveis

- `start` - Inicia o servidor (padrão)
- `config` - Verifica a configuração
- `info` - Mostra informações do sistema
- `help` - Mostra ajuda

### 📊 Comparação dos Modos

| Modo | Informações Básicas | Informações Detalhadas | Logs de Inicialização |
|------|-------------------|----------------------|---------------------|
| **Normal** | ✅ | ❌ | ❌ |
| **Verbose** | ✅ | ✅ | ✅ |
| **Quiet** | ❌ | ❌ | ❌ |

### 🎯 Resumo

**Antes:** Todas as informações apareciam sempre
**Agora:** 
- **Modo Normal**: Apenas informações essenciais
- **Modo Verbose**: Todas as informações detalhadas
- **Modo Quiet**: Apenas mensagens críticas
- **Comandos**: Funcionalidades específicas sem iniciar servidor

Isso torna o Syros muito mais flexível para diferentes ambientes e casos de uso!
