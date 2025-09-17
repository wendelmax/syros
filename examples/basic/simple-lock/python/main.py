#!/usr/bin/env python3
"""
Exemplo básico de uso do Syros para locks distribuídos
"""

import asyncio
import aiohttp
import json
import time
from typing import Optional

class SyrosLockClient:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url
        self.session = None
    
    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
    
    async def acquire_lock(
        self, 
        key: str, 
        ttl_seconds: int = 30, 
        owner: str = "python-client",
        metadata: Optional[str] = None,
        wait_timeout_seconds: Optional[int] = None
    ) -> Optional[str]:
        """Adquire um lock distribuído"""
        payload = {
            "key": key,
            "ttl_seconds": ttl_seconds,
            "owner": owner,
            "metadata": metadata,
            "wait_timeout_seconds": wait_timeout_seconds
        }
        
        async with self.session.post(
            f"{self.base_url}/api/v1/locks", 
            json=payload
        ) as response:
            if response.status == 200:
                data = await response.json()
                return data["lock_id"]
            else:
                error_text = await response.text()
                print(f"Erro ao adquirir lock: {response.status} - {error_text}")
                return None
    
    async def release_lock(self, key: str, lock_id: str, owner: str = "python-client") -> bool:
        """Libera um lock distribuído"""
        payload = {
            "lock_id": lock_id,
            "owner": owner
        }
        
        async with self.session.delete(
            f"{self.base_url}/api/v1/locks/{key}",
            json=payload
        ) as response:
            if response.status == 204:
                return True
            else:
                error_text = await response.text()
                print(f"Erro ao liberar lock: {response.status} - {error_text}")
                return False
    
    async def get_lock_status(self, key: str) -> dict:
        """Obtém o status de um lock"""
        async with self.session.get(f"{self.base_url}/api/v1/locks/{key}/status") as response:
            if response.status == 200:
                return await response.json()
            else:
                error_text = await response.text()
                print(f"Erro ao obter status do lock: {response.status} - {error_text}")
                return {}

async def exemplo_basico():
    """Exemplo básico de uso de locks"""
    print("🔒 Exemplo básico de locks distribuídos com Syros")
    
    async with SyrosLockClient() as client:
        resource_key = "recurso_critico_001"
        
        print(f"\n1. Verificando status inicial do lock '{resource_key}'...")
        status = await client.get_lock_status(resource_key)
        print(f"Status: {json.dumps(status, indent=2)}")
        
        print(f"\n2. Tentando adquirir lock para '{resource_key}'...")
        lock_id = await client.acquire_lock(
            key=resource_key,
            ttl_seconds=30,
            metadata="Exemplo Python - operação crítica",
            owner="worker-001"
        )
        
        if lock_id:
            print(f"✅ Lock adquirido com sucesso! ID: {lock_id}")
            
            print(f"\n3. Verificando status após aquisição...")
            status = await client.get_lock_status(resource_key)
            print(f"Status: {json.dumps(status, indent=2)}")
            
            print(f"\n4. Simulando trabalho crítico por 5 segundos...")
            await asyncio.sleep(5)
            
            print(f"\n5. Liberando lock...")
            success = await client.release_lock(resource_key, lock_id, "worker-001")
            
            if success:
                print("✅ Lock liberado com sucesso!")
            else:
                print("❌ Falha ao liberar lock")
            
            print(f"\n6. Verificando status após liberação...")
            status = await client.get_lock_status(resource_key)
            print(f"Status: {json.dumps(status, indent=2)}")
        else:
            print("❌ Falha ao adquirir lock")

async def exemplo_concorrencia():
    """Exemplo demonstrando concorrência entre workers"""
    print("\n🏃‍♂️ Exemplo de concorrência entre workers")
    
    async def worker(worker_id: str, delay: float):
        await asyncio.sleep(delay)
        
        async with SyrosLockClient() as client:
            resource_key = "recurso_compartilhado"
            
            print(f"Worker {worker_id}: Tentando adquirir lock...")
            start_time = time.time()
            
            lock_id = await client.acquire_lock(
                key=resource_key,
                ttl_seconds=10,
                owner=f"worker-{worker_id}",
                wait_timeout_seconds=15
            )
            
            if lock_id:
                elapsed = time.time() - start_time
                print(f"Worker {worker_id}: ✅ Lock adquirido após {elapsed:.2f}s (ID: {lock_id})")
                
                # Simular trabalho
                work_time = 3
                print(f"Worker {worker_id}: Executando trabalho por {work_time}s...")
                await asyncio.sleep(work_time)
                
                # Liberar lock
                await client.release_lock(resource_key, lock_id, f"worker-{worker_id}")
                print(f"Worker {worker_id}: ✅ Lock liberado")
            else:
                print(f"Worker {worker_id}: ❌ Timeout ao aguardar lock")
    
    # Iniciar 3 workers com delays diferentes
    tasks = [
        asyncio.create_task(worker("A", 0)),
        asyncio.create_task(worker("B", 1)),
        asyncio.create_task(worker("C", 2)),
    ]
    
    await asyncio.gather(*tasks)

async def main():
    """Função principal"""
    print("🚀 Exemplos de uso do Syros - Locks Distribuídos")
    print("=" * 60)
    
    try:
        await exemplo_basico()
        await exemplo_concorrencia()
        
        print("\n✅ Exemplos executados com sucesso!")
    except Exception as e:
        print(f"\n❌ Erro durante execução: {e}")

if __name__ == "__main__":
    asyncio.run(main())
