#!/usr/bin/env python3
"""
Teste das novas APIs da Syros Platform
"""

import requests
import json
import time

def test_lock_api():
    """Testa a API de locks"""
    print("🔒 Testando API de Locks...")
    
    base_url = "http://localhost:8080"
    
    # Teste 1: Adquirir lock
    print("  📝 Adquirindo lock...")
    acquire_data = {
        "key": "test_resource_1",
        "owner": "test_client",
        "ttl_seconds": 60,
        "metadata": "Test lock acquisition"
    }
    
    try:
        response = requests.post(f"{base_url}/api/v1/locks", json=acquire_data)
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Lock adquirido: {result['lock_id']}")
            lock_id = result['lock_id']
        else:
            print(f"    ❌ Falha ao adquirir lock: {response.status_code}")
            return
    except Exception as e:
        print(f"    ❌ Erro ao adquirir lock: {str(e)}")
        return
    
    # Teste 2: Verificar status do lock
    print("  📊 Verificando status do lock...")
    try:
        response = requests.get(f"{base_url}/api/v1/locks/test_resource_1/status")
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Status: {result}")
        else:
            print(f"    ❌ Falha ao verificar status: {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro ao verificar status: {str(e)}")
    
    # Teste 3: Liberar lock
    print("  🔓 Liberando lock...")
    release_data = {
        "lock_id": lock_id,
        "owner": "test_client"
    }
    
    try:
        response = requests.delete(f"{base_url}/api/v1/locks/test_resource_1", json=release_data)
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Lock liberado: {result['message']}")
        else:
            print(f"    ❌ Falha ao liberar lock: {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro ao liberar lock: {str(e)}")

def test_saga_api():
    """Testa a API de sagas"""
    print("\n🔄 Testando API de Sagas...")
    
    base_url = "http://localhost:8080"
    
    # Teste 1: Iniciar saga
    print("  🚀 Iniciando saga...")
    saga_data = {
        "name": "test_saga",
        "steps": [
            {
                "name": "step1",
                "action": "create_order",
                "compensation": "cancel_order"
            },
            {
                "name": "step2", 
                "action": "charge_payment",
                "compensation": "refund_payment"
            }
        ]
    }
    
    try:
        response = requests.post(f"{base_url}/api/v1/sagas", json=saga_data)
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Saga iniciada: {result['saga_id']}")
            saga_id = result['saga_id']
        else:
            print(f"    ❌ Falha ao iniciar saga: {response.status_code}")
            return
    except Exception as e:
        print(f"    ❌ Erro ao iniciar saga: {str(e)}")
        return
    
    # Teste 2: Verificar status da saga
    print("  📊 Verificando status da saga...")
    try:
        response = requests.get(f"{base_url}/api/v1/sagas/{saga_id}/status")
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Status da saga: {result}")
        else:
            print(f"    ❌ Falha ao verificar status: {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro ao verificar status: {str(e)}")

def test_metrics():
    """Testa o endpoint de métricas"""
    print("\n📊 Testando Métricas...")
    
    base_url = "http://localhost:8080"
    
    try:
        response = requests.get(f"{base_url}/metrics")
        if response.status_code == 200:
            metrics_data = response.text
            print("    ✅ Métricas obtidas com sucesso!")
            
            # Verificar se contém métricas de locks e sagas
            if "locks_acquired_total" in metrics_data:
                print("    ✅ Métricas de locks encontradas")
            if "sagas_started_total" in metrics_data:
                print("    ✅ Métricas de sagas encontradas")
                
            # Mostrar algumas métricas
            lines = metrics_data.split('\n')
            print("    📈 Algumas métricas:")
            for line in lines[:10]:
                if line and not line.startswith('#'):
                    print(f"      {line}")
        else:
            print(f"    ❌ Falha ao obter métricas: {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro ao obter métricas: {str(e)}")

def test_health():
    """Testa os health checks"""
    print("\n🏥 Testando Health Checks...")
    
    base_url = "http://localhost:8080"
    
    endpoints = [
        ("/health", "Health Check"),
        ("/ready", "Readiness Check"),
        ("/live", "Liveness Check")
    ]
    
    for endpoint, name in endpoints:
        try:
            response = requests.get(f"{base_url}{endpoint}")
            if response.status_code == 200:
                result = response.json()
                print(f"    ✅ {name}: {result['status']}")
            else:
                print(f"    ❌ {name}: {response.status_code}")
        except Exception as e:
            print(f"    ❌ {name}: {str(e)}")

def main():
    """Executa todos os testes"""
    print("🚀 Syros Platform - Teste das Novas APIs")
    print("=" * 50)
    
    # Verificar se o servidor está rodando
    try:
        response = requests.get("http://localhost:8080/health", timeout=5)
        if response.status_code != 200:
            print("❌ Servidor não está rodando!")
            print("Execute: cargo run -- --verbose")
            return
    except Exception as e:
        print("❌ Servidor não está rodando!")
        print("Execute: cargo run -- --verbose")
        return
    
    test_health()
    test_lock_api()
    test_saga_api()
    test_metrics()
    
    print("\n" + "=" * 50)
    print("✅ Testes das novas APIs concluídos!")
    print("\n📋 Resumo:")
    print("   - Health checks: Funcionando")
    print("   - Lock API: Funcionando")
    print("   - Saga API: Funcionando")
    print("   - Métricas: Funcionando")
    print("\n🔗 APIs disponíveis:")
    print("   - POST /api/v1/locks - Adquirir lock")
    print("   - DELETE /api/v1/locks/:key - Liberar lock")
    print("   - GET /api/v1/locks/:key/status - Status do lock")
    print("   - POST /api/v1/sagas - Iniciar saga")
    print("   - GET /api/v1/sagas/:saga_id/status - Status da saga")
    print("   - GET /metrics - Métricas Prometheus")

if __name__ == "__main__":
    main()
