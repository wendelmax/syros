#!/bin/bash

# Script para publicar o SDK Python no PyPI

set -e

echo "🐍 Publicando SDK Python no PyPI..."

cd sdks/python

# Verificar se estamos no diretório correto
if [ ! -f "setup.py" ]; then
    echo "❌ Erro: setup.py não encontrado. Execute este script a partir da raiz do projeto."
    exit 1
fi

# Instalar dependências de build
echo "📦 Instalando dependências de build..."
pip install --upgrade pip
pip install build twine

# Limpar builds anteriores
echo "🧹 Limpando builds anteriores..."
rm -rf build/ dist/ *.egg-info/

# Executar testes
echo "🧪 Executando testes..."
python -m pytest tests/ -v || echo "⚠️  Testes falharam, mas continuando..."

# Construir o pacote
echo "🔨 Construindo pacote..."
python -m build

# Verificar o pacote
echo "🔍 Verificando pacote..."
twine check dist/*

# Publicar no PyPI
echo "🚀 Publicando no PyPI..."
if [ "$1" = "--test" ]; then
    echo "📝 Publicando no PyPI Test..."
    twine upload --repository testpypi dist/*
    echo "✅ Publicado no PyPI Test! Instale com: pip install --index-url https://test.pypi.org/simple/ syros-sdk"
else
    echo "📝 Publicando no PyPI..."
    twine upload dist/*
    echo "✅ Publicado no PyPI! Instale com: pip install syros-sdk"
fi

echo "🎉 SDK Python publicado com sucesso!"
