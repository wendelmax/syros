# PowerShell script para configurar Syros

Write-Host "🚀 Configurando Syros..." -ForegroundColor Green

# Verificar se Docker está instalado
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker não encontrado. Instale o Docker primeiro." -ForegroundColor Red
    exit 1
}

# Verificar se Docker Compose está instalado
if (-not (Get-Command docker-compose -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker Compose não encontrado. Instale o Docker Compose primeiro." -ForegroundColor Red
    exit 1
}

Write-Host "📦 Construindo imagem Docker..." -ForegroundColor Yellow
docker-compose build

Write-Host "🗄️ Iniciando serviços de infraestrutura..." -ForegroundColor Yellow
docker-compose up -d redis etcd postgres

Write-Host "⏳ Aguardando serviços ficarem prontos..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

Write-Host "🔧 Executando migrações do banco de dados..." -ForegroundColor Yellow
docker-compose run --rm syros-platform /app/syros-platform migrate

Write-Host "🚀 Iniciando Syros..." -ForegroundColor Yellow
docker-compose up -d syros-platform

Write-Host "📊 Iniciando serviços de monitoramento..." -ForegroundColor Yellow
docker-compose up -d prometheus grafana

Write-Host "✅ Syros configurado com sucesso!" -ForegroundColor Green
Write-Host ""
Write-Host "🌐 Serviços disponíveis:" -ForegroundColor Cyan
Write-Host "  - Syros API: http://localhost:8080" -ForegroundColor White
Write-Host "  - Syros gRPC: localhost:9090" -ForegroundColor White
Write-Host "  - Syros WebSocket: ws://localhost:8081" -ForegroundColor White
Write-Host "  - Prometheus: http://localhost:9091" -ForegroundColor White
Write-Host "  - Grafana: http://localhost:3000 (admin/admin)" -ForegroundColor White
Write-Host ""
Write-Host "📚 Para ver logs: docker-compose logs -f syros-platform" -ForegroundColor Yellow
Write-Host "🛑 Para parar: docker-compose down" -ForegroundColor Yellow
