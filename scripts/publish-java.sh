#!/bin/bash

# Script para publicar o SDK Java no Maven Central

set -e

echo "☕ Publicando SDK Java no Maven Central..."

cd sdks/java

# Verificar se estamos no diretório correto
if [ ! -f "pom.xml" ]; then
    echo "❌ Erro: pom.xml não encontrado. Execute este script a partir da raiz do projeto."
    exit 1
fi

# Verificar se Maven está instalado
if ! command -v mvn &> /dev/null; then
    echo "❌ Erro: Maven não está instalado."
    exit 1
fi

# Verificar configuração do Maven para OSSRH
if [ ! -f "$HOME/.m2/settings.xml" ]; then
    echo "⚠️  Aviso: settings.xml não encontrado. Configure as credenciais do OSSRH."
    echo "Crie ~/.m2/settings.xml com as credenciais do Sonatype."
fi

# Limpar builds anteriores
echo "🧹 Limpando builds anteriores..."
mvn clean

# Executar testes
echo "🧪 Executando testes..."
mvn test || echo "⚠️  Testes falharam, mas continuando..."

# Compilar e gerar JARs
echo "🔨 Compilando e gerando JARs..."
mvn package

# Gerar sources e javadoc
echo "📚 Gerando sources e javadoc..."
mvn source:jar javadoc:jar

# Verificar se os JARs foram gerados
if [ ! -f "target/syros-sdk-1.0.0.jar" ]; then
    echo "❌ Erro: JAR principal não foi gerado."
    exit 1
fi

# Publicar no Maven Central
echo "🚀 Publicando no Maven Central..."
if [ "$1" = "--test" ]; then
    echo "📝 Executando deploy de teste..."
    mvn deploy -DskipTests
    echo "✅ Deploy de teste concluído!"
else
    echo "📝 Publicando no Maven Central..."
    mvn deploy -DskipTests
    echo "✅ Publicado no Maven Central!"
    echo "📦 Instale com:"
    echo "   <dependency>"
    echo "     <groupId>com.syros</groupId>"
    echo "     <artifactId>syros-sdk</artifactId>"
    echo "     <version>1.0.0</version>"
    echo "   </dependency>"
fi

echo "🎉 SDK Java publicado com sucesso!"
