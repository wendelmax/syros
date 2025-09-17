#!/usr/bin/env python3
"""
Cliente de teste para Syros
Demonstra o uso básico da API REST
"""

import requests
import json
import time

BASE_URL = "http://localhost:8080"

def test_health():
    """Testa o endpoint de health"""
    print("🔍 Testando health check...")
    response = requests.get(f"{BASE_URL}/health")
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Status: {data['status']}")
        print(f"📅 Timestamp: {data['timestamp']}")
        print(f"⏱️  Uptime: {data['uptime_seconds']} segundos")
        print(f"📦 Versão: {data['version']}")
    else:
        print(f"❌ Erro: {response.status_code}")
    print()

def test_readiness():
    """Testa o endpoint de readiness"""
    print("🔍 Testando readiness check...")
    response = requests.get(f"{BASE_URL}/ready")
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Ready: {data['ready']}")
        print("📋 Checks:")
        for check in data['checks']:
            status_emoji = "✅" if check['status'] == 'ready' else "❌"
            print(f"   {status_emoji} {check['name']}: {check['message']}")
    else:
        print(f"❌ Erro: {response.status_code}")
    print()

def test_liveness():
    """Testa o endpoint de liveness"""
    print("🔍 Testando liveness check...")
    response = requests.get(f"{BASE_URL}/live")
    if response.status_code == 200:
        data = response.json()
        print(f"✅ Liveness: {data['status']}")
    else:
        print(f"❌ Erro: {response.status_code}")
    print()

def main():
    """Função principal"""
    print("🚀 Syros - Cliente de Teste")
    print("=" * 50)
    
    try:
        test_health()
        test_readiness()
        test_liveness()
        
        print("🎉 Todos os testes passaram!")
        print("\n📖 Próximos passos:")
        print("   - Implementar APIs completas (locks, sagas, events, cache)")
        print("   - Adicionar gRPC e WebSocket")
        print("   - Criar SDKs para outras linguagens")
        print("   - Adicionar testes automatizados")
        
    except requests.exceptions.ConnectionError:
        print("❌ Erro: Não foi possível conectar ao servidor")
        print("   Certifique-se de que o servidor está rodando em http://localhost:8080")
    except Exception as e:
        print(f"❌ Erro inesperado: {e}")

if __name__ == "__main__":
    main()
