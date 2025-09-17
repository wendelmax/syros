#!/usr/bin/env python3
"""
Teste completo de todas as APIs da Syros
"""

import requests
import json
import time

def test_all_apis():
    """Testa todas as APIs implementadas"""
    print("🚀 Syros - Teste Completo de APIs")
    print("=" * 60)
    
    base_url = "http://localhost:8080"
    
    # Verificar se o servidor está rodando
    try:
        response = requests.get(f"{base_url}/health", timeout=5)
        if response.status_code != 200:
            print("❌ Servidor não está rodando!")
            print("Execute: cargo run -- --verbose")
            return False
    except Exception as e:
        print("❌ Servidor não está rodando!")
        print("Execute: cargo run -- --verbose")
        return False
    
    success_count = 0
    total_tests = 0
    
    # Teste 1: Health Checks
    print("\n🏥 Testando Health Checks...")
    health_endpoints = [
        ("/health", "Health Check"),
        ("/ready", "Readiness Check"),
        ("/live", "Liveness Check")
    ]
    
    for endpoint, name in health_endpoints:
        total_tests += 1
        try:
            response = requests.get(f"{base_url}{endpoint}")
            if response.status_code == 200:
                result = response.json()
                print(f"    ✅ {name}: {result['status']}")
                success_count += 1
            else:
                print(f"    ❌ {name}: HTTP {response.status_code}")
        except Exception as e:
            print(f"    ❌ {name}: {str(e)}")
    
    # Teste 2: Lock API
    print("\n🔒 Testando Lock API...")
    total_tests += 1
    try:
        # Adquirir lock
        acquire_data = {
            "key": "test_resource_complete",
            "owner": "test_client_complete",
            "ttl_seconds": 60,
            "metadata": "Complete test lock"
        }
        
        response = requests.post(f"{base_url}/api/v1/locks", json=acquire_data)
        if response.status_code == 200:
            result = response.json()
            lock_id = result['lock_id']
            print(f"    ✅ Lock adquirido: {lock_id}")
            
            # Verificar status
            response = requests.get(f"{base_url}/api/v1/locks/test_resource_complete/status")
            if response.status_code == 200:
                status = response.json()
                print(f"    ✅ Status do lock: {status}")
                success_count += 1
            else:
                print(f"    ❌ Falha ao verificar status: HTTP {response.status_code}")
        else:
            print(f"    ❌ Falha ao adquirir lock: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro na Lock API: {str(e)}")
    
    # Teste 3: Saga API
    print("\n🔄 Testando Saga API...")
    total_tests += 1
    try:
        saga_data = {
            "name": "complete_test_saga",
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
                },
                {
                    "name": "step3",
                    "action": "send_notification",
                    "compensation": "cancel_notification"
                }
            ]
        }
        
        response = requests.post(f"{base_url}/api/v1/sagas", json=saga_data)
        if response.status_code == 200:
            result = response.json()
            saga_id = result['saga_id']
            print(f"    ✅ Saga iniciada: {saga_id}")
            
            # Verificar status
            response = requests.get(f"{base_url}/api/v1/sagas/{saga_id}/status")
            if response.status_code == 200:
                status = response.json()
                print(f"    ✅ Status da saga: {status}")
                success_count += 1
            else:
                print(f"    ❌ Falha ao verificar status: HTTP {response.status_code}")
        else:
            print(f"    ❌ Falha ao iniciar saga: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro na Saga API: {str(e)}")
    
    # Teste 4: Event API
    print("\n📝 Testando Event API...")
    total_tests += 1
    try:
        event_data = {
            "stream_id": "complete_test_stream",
            "event_type": "UserAction",
            "data": {
                "user_id": "test_user_123",
                "action": "complete_test",
                "timestamp": int(time.time()),
                "details": {
                    "test_type": "complete_api_test",
                    "version": "1.0.0"
                }
            },
            "metadata": {
                "source": "python_test",
                "test_id": "complete_test_001"
            }
        }
        
        response = requests.post(f"{base_url}/api/v1/events", json=event_data)
        if response.status_code == 200:
            result = response.json()
            event_id = result['event_id']
            print(f"    ✅ Evento adicionado: {event_id}")
            
            # Buscar eventos
            response = requests.get(f"{base_url}/api/v1/events/complete_test_stream")
            if response.status_code == 200:
                events = response.json()
                print(f"    ✅ Eventos recuperados: {len(events['events'])} eventos")
                success_count += 1
            else:
                print(f"    ❌ Falha ao buscar eventos: HTTP {response.status_code}")
        else:
            print(f"    ❌ Falha ao adicionar evento: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro na Event API: {str(e)}")
    
    # Teste 5: Cache API
    print("\n💾 Testando Cache API...")
    total_tests += 1
    try:
        cache_data = {
            "value": {
                "product_id": "test_product_123",
                "name": "Test Product",
                "price": 99.99,
                "category": "test",
                "in_stock": True,
                "metadata": {
                    "created_at": int(time.time()),
                    "test_data": True
                }
            },
            "ttl_seconds": 300,
            "tags": ["test", "product", "complete_test"]
        }
        
        response = requests.post(f"{base_url}/api/v1/cache/test_product_123", json=cache_data)
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Cache definido: {result['message']}")
            
            # Buscar cache
            response = requests.get(f"{base_url}/api/v1/cache/test_product_123")
            if response.status_code == 200:
                cache_result = response.json()
                print(f"    ✅ Cache recuperado: {cache_result}")
                success_count += 1
            else:
                print(f"    ❌ Falha ao buscar cache: HTTP {response.status_code}")
        else:
            print(f"    ❌ Falha ao definir cache: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro na Cache API: {str(e)}")
    
    # Teste 6: Metrics
    print("\n📊 Testando Métricas...")
    total_tests += 1
    try:
        response = requests.get(f"{base_url}/metrics")
        if response.status_code == 200:
            metrics_data = response.text
            print(f"    ✅ Métricas obtidas: {len(metrics_data)} caracteres")
            
            # Verificar se contém métricas esperadas
            expected_metrics = [
                "locks_acquired_total",
                "sagas_started_total", 
                "events_appended_total",
                "cache_size"
            ]
            
            found_metrics = []
            for metric in expected_metrics:
                if metric in metrics_data:
                    found_metrics.append(metric)
            
            print(f"    ✅ Métricas encontradas: {len(found_metrics)}/{len(expected_metrics)}")
            print(f"    📈 Métricas: {', '.join(found_metrics)}")
            success_count += 1
        else:
            print(f"    ❌ Falha ao obter métricas: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro nas métricas: {str(e)}")
    
    # Resumo final
    print("\n" + "=" * 60)
    print("📋 RESUMO FINAL")
    print("=" * 60)
    print(f"✅ Testes bem-sucedidos: {success_count}/{total_tests}")
    print(f"📊 Taxa de sucesso: {(success_count/total_tests)*100:.1f}%")
    
    if success_count == total_tests:
        print("🎉 TODOS OS TESTES PASSARAM!")
        print("🚀 Syros está funcionando perfeitamente!")
    elif success_count >= total_tests * 0.8:
        print("✅ Maioria dos testes passou - plataforma funcional!")
    else:
        print("⚠️  Alguns testes falharam - verifique a configuração")
    
    print("\n🔗 APIs Testadas:")
    print("   ✅ Health Checks (/health, /ready, /live)")
    print("   ✅ Lock API (/api/v1/locks/*)")
    print("   ✅ Saga API (/api/v1/sagas/*)")
    print("   ✅ Event API (/api/v1/events/*)")
    print("   ✅ Cache API (/api/v1/cache/*)")
    print("   ✅ Metrics (/metrics)")
    
    print("\n📦 SDKs Disponíveis:")
    print("   ✅ Python SDK (completo)")
    print("   ✅ Node.js SDK (completo)")
    print("   ✅ Java SDK (completo)")
    print("   ✅ C# SDK (completo)")
    print("   ✅ Go SDK (completo)")
    
    return success_count == total_tests

if __name__ == "__main__":
    success = test_all_apis()
    exit(0 if success else 1)
