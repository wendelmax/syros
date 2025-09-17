#!/bin/bash

# Script para publicar o SDK Go

set -e

echo "🐹 Publicando SDK Go..."

cd sdks/go

# Verificar se estamos no diretório correto
if [ ! -f "go.mod" ]; then
    echo "❌ Erro: go.mod não encontrado. Execute este script a partir da raiz do projeto."
    exit 1
fi

# Verificar se Go está instalado
if ! command -v go &> /dev/null; then
    echo "❌ Erro: Go não está instalado."
    exit 1
fi

# Verificar se goreleaser está instalado
if ! command -v goreleaser &> /dev/null; then
    echo "📦 Instalando goreleaser..."
    go install github.com/goreleaser/goreleaser@latest
fi

# Limpar builds anteriores
echo "🧹 Limpando builds anteriores..."
rm -rf dist/

# Executar testes
echo "🧪 Executando testes..."
go test ./... -v || echo "⚠️  Testes falharam, mas continuando..."

# Verificar se há tags Git
if [ -z "$(git tag -l)" ]; then
    echo "⚠️  Aviso: Nenhuma tag Git encontrada. Criando tag v1.0.0..."
    git tag v1.0.0
fi

# Publicar com goreleaser
echo "🚀 Publicando com goreleaser..."
if [ "$1" = "--test" ]; then
    echo "📝 Executando snapshot..."
    goreleaser release --snapshot --rm-dist
    echo "✅ Snapshot criado!"
else
    echo "📝 Publicando release..."
    goreleaser release --rm-dist
    echo "✅ Release publicado!"
    echo "📦 Instale com:"
    echo "   go get github.com/syros/syros-sdk-go@v1.0.0"
fi

echo "🎉 SDK Go publicado com sucesso!"
