#!/bin/bash

# Script master para publicar todos os SDKs

set -e

echo "🚀 Publicando todos os SDKs do Syros..."
echo "=================================================="

# Tornar scripts executáveis
chmod +x scripts/publish-*.sh

# Função para executar com tratamento de erro
run_script() {
    local script_name=$1
    local test_flag=$2
    
    echo ""
    echo "📦 Executando $script_name..."
    echo "----------------------------------------"
    
    if ./scripts/$script_name $test_flag; then
        echo "✅ $script_name concluído com sucesso!"
    else
        echo "❌ $script_name falhou!"
        return 1
    fi
}

# Verificar se é modo teste
TEST_MODE=""
if [ "$1" = "--test" ]; then
    TEST_MODE="--test"
    echo "🧪 Modo teste ativado - não será publicado de verdade"
fi

# Executar todos os scripts de publicação
echo "Iniciando publicação de todos os SDKs..."

# Python
run_script "publish-python.sh" $TEST_MODE

# Node.js
run_script "publish-nodejs.sh" $TEST_MODE

# Java
run_script "publish-java.sh" $TEST_MODE

# C#
run_script "publish-csharp.sh" $TEST_MODE

# Go
run_script "publish-go.sh" $TEST_MODE

echo ""
echo "🎉 Todos os SDKs foram processados com sucesso!"
echo "=================================================="

if [ "$TEST_MODE" = "--test" ]; then
    echo "🧪 Modo teste concluído - nenhum pacote foi publicado de verdade"
    echo "Para publicar de verdade, execute: ./scripts/publish-all.sh"
else
    echo "✅ Todos os SDKs foram publicados!"
    echo ""
    echo "📦 Instalação dos SDKs:"
    echo "  Python:  pip install syros-sdk"
    echo "  Node.js: npm install syros-sdk"
    echo "  Java:    <dependency><groupId>com.syros</groupId><artifactId>syros-sdk</artifactId><version>1.0.0</version></dependency>"
    echo "  C#:      dotnet add package SyrosSDK"
    echo "  Go:      go get github.com/syros/syros-sdk-go@v1.0.0"
fi
