#!/bin/bash

set -e

echo "🚀 Configurando Syros..."

if ! command -v docker &> /dev/null; then
    echo "❌ Docker não encontrado. Instale o Docker primeiro."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose não encontrado. Instale o Docker Compose primeiro."
    exit 1
fi

echo "📦 Construindo imagem Docker..."
docker-compose build

echo "🗄️ Iniciando serviços de infraestrutura..."
docker-compose up -d redis etcd postgres

echo "⏳ Aguardando serviços ficarem prontos..."
sleep 10

echo "🔧 Executando migrações do banco de dados..."
docker-compose run --rm syros-platform /app/syros-platform migrate

echo "🚀 Iniciando Syros..."
docker-compose up -d syros-platform

echo "📊 Iniciando serviços de monitoramento..."
docker-compose up -d prometheus grafana

echo "✅ Syros configurado com sucesso!"
echo ""
echo "🌐 Serviços disponíveis:"
echo "  - Syros API: http://localhost:8080"
echo "  - Syros gRPC: localhost:9090"
echo "  - Syros WebSocket: ws://localhost:8081"
echo "  - Prometheus: http://localhost:9091"
echo "  - Grafana: http://localhost:3000 (admin/admin)"
echo ""
echo "📚 Para ver logs: docker-compose logs -f syros-platform"
echo "🛑 Para parar: docker-compose down"
