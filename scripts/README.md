# Scripts de Publicação dos SDKs

Este diretório contém scripts para publicar todos os SDKs do Syros nos respectivos package managers.

## Scripts Disponíveis

### Scripts Individuais

- `publish-python.sh` - Publica o SDK Python no PyPI
- `publish-nodejs.sh` - Publica o SDK Node.js no npm
- `publish-java.sh` - Publica o SDK Java no Maven Central
- `publish-csharp.sh` - Publica o SDK C# no NuGet
- `publish-go.sh` - Publica o SDK Go (usando goreleaser)

### Script Master

- `publish-all.sh` - Executa todos os scripts de publicação

## Uso

### Publicação Individual

```bash
# Python
./scripts/publish-python.sh

# Node.js
./scripts/publish-nodejs.sh

# Java
./scripts/publish-java.sh

# C#
./scripts/publish-csharp.sh

# Go
./scripts/publish-go.sh
```

### Publicação de Todos os SDKs

```bash
# Publicar todos
./scripts/publish-all.sh

# Modo teste (não publica de verdade)
./scripts/publish-all.sh --test
```

## Pré-requisitos

### Python (PyPI)
- Python 3.8+
- pip
- build
- twine
- Conta no PyPI

### Node.js (npm)
- Node.js 14+
- npm
- Conta no npm

### Java (Maven Central)
- Maven 3.6+
- GPG
- Conta no Sonatype OSSRH
- Configuração do `~/.m2/settings.xml`

### C# (NuGet)
- .NET 6.0+
- API Key do NuGet (variável `NUGET_API_KEY`)

### Go
- Go 1.21+
- goreleaser
- Conta no GitHub
- Tags Git configuradas

## Configuração

### 1. Credenciais

Configure as credenciais para cada package manager:

```bash
# PyPI
pip install twine
twine login

# npm
npm login

# Maven Central
# Configure ~/.m2/settings.xml com credenciais do OSSRH

# NuGet
export NUGET_API_KEY="sua-api-key"

# Go
# Configure GitHub token para goreleaser
```

### 2. Configuração do Maven Central

Crie `~/.m2/settings.xml`:

```xml
<settings>
  <servers>
    <server>
      <id>ossrh</id>
      <username>seu-usuario</username>
      <password>sua-senha</password>
    </server>
  </servers>
</settings>
```

## Modo Teste

Todos os scripts suportam modo teste:

```bash
./scripts/publish-python.sh --test
./scripts/publish-nodejs.sh --test
./scripts/publish-java.sh --test
./scripts/publish-csharp.sh --test
./scripts/publish-go.sh --test
./scripts/publish-all.sh --test
```

## Troubleshooting

### Erro de Credenciais
- Verifique se está logado nos package managers
- Confirme as credenciais estão corretas

### Erro de Build
- Execute `cargo build` na raiz do projeto
- Verifique se todas as dependências estão instaladas

### Erro de Testes
- Os scripts continuam mesmo se os testes falharem
- Para parar em caso de falha, remova o `|| echo "⚠️  Testes falharam, mas continuando..."`

### Erro de Permissões
- No Windows, execute no PowerShell como administrador
- No Linux/Mac, use `chmod +x scripts/*.sh`

## Instalação dos SDKs Publicados

Após a publicação, os usuários podem instalar com:

```bash
# Python
pip install syros-sdk

# Node.js
npm install syros-sdk

# Java (Maven)
<dependency>
  <groupId>com.syros</groupId>
  <artifactId>syros-sdk</artifactId>
  <version>1.0.0</version>
</dependency>

# C#
dotnet add package SyrosSDK

# Go
go get github.com/syros/syros-sdk-go@v1.0.0
```
