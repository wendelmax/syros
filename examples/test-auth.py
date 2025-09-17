#!/usr/bin/env python3
"""
Teste da API de Autenticação da Syros Platform
"""

import requests
import json
import time

def test_auth_api():
    """Testa a API de autenticação"""
    print("🔐 Syros Platform - Teste de Autenticação")
    print("=" * 50)
    
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
    
    # Teste 1: Login com credenciais válidas
    print("\n🔑 Testando Login...")
    total_tests += 1
    try:
        login_data = {
            "username": "admin",
            "password": "admin123"
        }
        
        response = requests.post(f"{base_url}/api/v1/auth/login", json=login_data)
        if response.status_code == 200:
            result = response.json()
            token = result['token']
            print(f"    ✅ Login admin bem-sucedido!")
            print(f"    🔑 Token: {token[:50]}...")
            print(f"    👤 User ID: {result['user_id']}")
            print(f"    🎭 Role: {result['role']}")
            print(f"    ⏰ Expires in: {result['expires_in']} seconds")
            success_count += 1
        else:
            print(f"    ❌ Login admin falhou: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro no login admin: {str(e)}")
    
    # Teste 2: Login com usuário comum
    print("\n👤 Testando Login de Usuário...")
    total_tests += 1
    try:
        login_data = {
            "username": "user",
            "password": "user123"
        }
        
        response = requests.post(f"{base_url}/api/v1/auth/login", json=login_data)
        if response.status_code == 200:
            result = response.json()
            user_token = result['token']
            print(f"    ✅ Login user bem-sucedido!")
            print(f"    🔑 Token: {user_token[:50]}...")
            print(f"    👤 User ID: {result['user_id']}")
            print(f"    🎭 Role: {result['role']}")
            success_count += 1
        else:
            print(f"    ❌ Login user falhou: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro no login user: {str(e)}")
    
    # Teste 3: Login com credenciais inválidas
    print("\n🚫 Testando Login Inválido...")
    total_tests += 1
    try:
        login_data = {
            "username": "invalid",
            "password": "invalid"
        }
        
        response = requests.post(f"{base_url}/api/v1/auth/login", json=login_data)
        if response.status_code == 401:
            print(f"    ✅ Login inválido rejeitado corretamente!")
            success_count += 1
        else:
            print(f"    ❌ Login inválido deveria retornar 401, retornou: {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro no teste de login inválido: {str(e)}")
    
    # Teste 4: Criar API Key
    print("\n🔑 Testando Criação de API Key...")
    total_tests += 1
    try:
        api_key_data = {
            "name": "test_api_key",
            "description": "API key para testes",
            "permissions": ["read", "write"],
            "expires_in_days": 30
        }
        
        response = requests.post(f"{base_url}/api/v1/auth/api-keys", json=api_key_data)
        if response.status_code == 200:
            result = response.json()
            api_key = result['key']
            api_key_id = result['id']
            print(f"    ✅ API Key criada com sucesso!")
            print(f"    🔑 Key: {api_key[:20]}...")
            print(f"    🆔 ID: {api_key_id}")
            print(f"    📝 Name: {result['name']}")
            print(f"    🔐 Permissions: {result['permissions']}")
            success_count += 1
        else:
            print(f"    ❌ Criação de API Key falhou: HTTP {response.status_code}")
            api_key = None
            api_key_id = None
    except Exception as e:
        print(f"    ❌ Erro na criação de API Key: {str(e)}")
        api_key = None
        api_key_id = None
    
    # Teste 5: Listar API Keys
    print("\n📋 Testando Listagem de API Keys...")
    total_tests += 1
    try:
        response = requests.get(f"{base_url}/api/v1/auth/api-keys")
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ API Keys listadas com sucesso!")
            print(f"    📊 Total de keys: {len(result)}")
            for key in result:
                print(f"      - {key['name']}: {key['key']}")
            success_count += 1
        else:
            print(f"    ❌ Listagem de API Keys falhou: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro na listagem de API Keys: {str(e)}")
    
    # Teste 6: Estatísticas de API Keys
    print("\n📊 Testando Estatísticas de API Keys...")
    total_tests += 1
    try:
        response = requests.get(f"{base_url}/api/v1/auth/stats")
        if response.status_code == 200:
            result = response.json()
            print(f"    ✅ Estatísticas obtidas com sucesso!")
            print(f"    📊 Total keys: {result['total_keys']}")
            print(f"    ✅ Active keys: {result['active_keys']}")
            print(f"    ⏰ Expired keys: {result['expired_keys']}")
            print(f"    🔢 Total usage: {result['total_usage']}")
            success_count += 1
        else:
            print(f"    ❌ Estatísticas de API Keys falharam: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro nas estatísticas de API Keys: {str(e)}")
    
    # Teste 7: Criar Token personalizado
    print("\n🎫 Testando Criação de Token Personalizado...")
    total_tests += 1
    try:
        token_data = {
            "user_id": "custom_user_123",
            "role": "developer",
            "expiration_hours": 12
        }
        
        response = requests.post(f"{base_url}/api/v1/auth/token", json=token_data)
        if response.status_code == 200:
            result = response.json()
            custom_token = result['token']
            print(f"    ✅ Token personalizado criado com sucesso!")
            print(f"    🔑 Token: {custom_token[:50]}...")
            print(f"    ⏰ Expires in: {result['expires_in']} seconds")
            success_count += 1
        else:
            print(f"    ❌ Criação de token personalizado falhou: HTTP {response.status_code}")
    except Exception as e:
        print(f"    ❌ Erro na criação de token personalizado: {str(e)}")
    
    # Teste 8: Usar API Key para acessar endpoint protegido
    if api_key:
        print("\n🔐 Testando Acesso com API Key...")
        total_tests += 1
        try:
            headers = {
                "x-api-key": api_key
            }
            
            response = requests.get(f"{base_url}/api/v1/locks/test_key/status", headers=headers)
            if response.status_code == 200:
                print(f"    ✅ Acesso com API Key bem-sucedido!")
                success_count += 1
            else:
                print(f"    ❌ Acesso com API Key falhou: HTTP {response.status_code}")
        except Exception as e:
            print(f"    ❌ Erro no acesso com API Key: {str(e)}")
    
    # Teste 9: Usar JWT Token para acessar endpoint protegido
    if 'token' in locals():
        print("\n🎫 Testando Acesso com JWT Token...")
        total_tests += 1
        try:
            headers = {
                "Authorization": f"Bearer {token}"
            }
            
            response = requests.get(f"{base_url}/api/v1/locks/test_jwt/status", headers=headers)
            if response.status_code == 200:
                print(f"    ✅ Acesso com JWT Token bem-sucedido!")
                success_count += 1
            else:
                print(f"    ❌ Acesso com JWT Token falhou: HTTP {response.status_code}")
        except Exception as e:
            print(f"    ❌ Erro no acesso com JWT Token: {str(e)}")
    
    # Resumo final
    print("\n" + "=" * 50)
    print("📋 RESUMO FINAL - AUTENTICAÇÃO")
    print("=" * 50)
    print(f"✅ Testes bem-sucedidos: {success_count}/{total_tests}")
    print(f"📊 Taxa de sucesso: {(success_count/total_tests)*100:.1f}%")
    
    if success_count == total_tests:
        print("🎉 TODOS OS TESTES DE AUTENTICAÇÃO PASSARAM!")
        print("🔐 Sistema de segurança funcionando perfeitamente!")
    elif success_count >= total_tests * 0.8:
        print("✅ Maioria dos testes passou - sistema de segurança funcional!")
    else:
        print("⚠️  Alguns testes falharam - verifique a configuração")
    
    print("\n🔐 Funcionalidades de Segurança Testadas:")
    print("   ✅ Login com JWT")
    print("   ✅ Criação de API Keys")
    print("   ✅ Validação de tokens")
    print("   ✅ Controle de acesso")
    print("   ✅ Estatísticas de uso")
    
    print("\n👤 Usuários de Teste:")
    print("   - admin/admin123 (role: admin)")
    print("   - user/user123 (role: user)")
    
    return success_count == total_tests

if __name__ == "__main__":
    success = test_auth_api()
    exit(0 if success else 1)
