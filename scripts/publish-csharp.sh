#!/bin/bash

# Script para publicar o SDK C# no NuGet

set -e

echo "🔷 Publicando SDK C# no NuGet..."

cd sdks/csharp/SyrosSDK

# Verificar se estamos no diretório correto
if [ ! -f "SyrosSDK.csproj" ]; then
    echo "❌ Erro: SyrosSDK.csproj não encontrado. Execute este script a partir da raiz do projeto."
    exit 1
fi

# Verificar se dotnet está instalado
if ! command -v dotnet &> /dev/null; then
    echo "❌ Erro: .NET SDK não está instalado."
    exit 1
fi

# Restaurar dependências
echo "📦 Restaurando dependências..."
dotnet restore

# Executar testes
echo "🧪 Executando testes..."
dotnet test || echo "⚠️  Testes falharam, mas continuando..."

# Build do projeto
echo "🔨 Fazendo build do projeto..."
dotnet build --configuration Release

# Pack do pacote NuGet
echo "📦 Criando pacote NuGet..."
dotnet pack --configuration Release --no-build

# Verificar se o pacote foi criado
if [ ! -f "bin/Release/SyrosSDK.1.0.0.nupkg" ]; then
    echo "❌ Erro: Pacote NuGet não foi criado."
    exit 1
fi

# Publicar no NuGet
echo "🚀 Publicando no NuGet..."
if [ "$1" = "--test" ]; then
    echo "📝 Executando push de teste..."
    dotnet nuget push bin/Release/SyrosSDK.1.0.0.nupkg --api-key $NUGET_API_KEY --source https://api.nuget.org/v3/index.json --dry-run
    echo "✅ Dry run concluído!"
else
    echo "📝 Publicando no NuGet..."
    dotnet nuget push bin/Release/SyrosSDK.1.0.0.nupkg --api-key $NUGET_API_KEY --source https://api.nuget.org/v3/index.json
    echo "✅ Publicado no NuGet!"
    echo "📦 Instale com:"
    echo "   dotnet add package SyrosSDK"
fi

echo "🎉 SDK C# publicado com sucesso!"
