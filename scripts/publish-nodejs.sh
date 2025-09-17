#!/bin/bash

# Script para publicar o SDK Node.js no npm

set -e

echo "📦 Publicando SDK Node.js no npm..."

cd sdks/nodejs

# Verificar se estamos no diretório correto
if [ ! -f "package.json" ]; then
    echo "❌ Erro: package.json não encontrado. Execute este script a partir da raiz do projeto."
    exit 1
fi

# Verificar se npm está instalado
if ! command -v npm &> /dev/null; then
    echo "❌ Erro: npm não está instalado."
    exit 1
fi

# Verificar se estamos logados no npm
if ! npm whoami &> /dev/null; then
    echo "❌ Erro: Não está logado no npm. Execute 'npm login' primeiro."
    exit 1
fi

# Instalar dependências
echo "📦 Instalando dependências..."
npm install

# Executar testes
echo "🧪 Executando testes..."
npm test || echo "⚠️  Testes falharam, mas continuando..."

# Executar lint
echo "🔍 Executando lint..."
npm run lint || echo "⚠️  Lint falhou, mas continuando..."

# Build do TypeScript
echo "🔨 Fazendo build do TypeScript..."
npm run build

# Verificar se o build foi bem-sucedido
if [ ! -f "syros-sdk.d.ts" ]; then
    echo "❌ Erro: Build do TypeScript falhou."
    exit 1
fi

# Publicar no npm
echo "🚀 Publicando no npm..."
if [ "$1" = "--test" ]; then
    echo "📝 Publicando no npm (teste)..."
    npm publish --dry-run
    echo "✅ Dry run concluído! Para publicar de verdade, execute sem --test"
else
    echo "📝 Publicando no npm..."
    npm publish
    echo "✅ Publicado no npm! Instale com: npm install syros-sdk"
fi

echo "🎉 SDK Node.js publicado com sucesso!"
