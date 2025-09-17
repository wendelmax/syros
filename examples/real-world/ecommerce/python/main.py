#!/usr/bin/env python3
"""
Exemplo real: Sistema de E-commerce usando Saga Pattern
Demonstra como orquestrar uma transação distribuída para processamento de pedidos
"""

import asyncio
import aiohttp
import json
import uuid
from typing import Dict, List, Optional
from dataclasses import dataclass, asdict

@dataclass
class Produto:
    id: str
    nome: str
    preco: float
    estoque: int

@dataclass
class ItemPedido:
    produto_id: str
    quantidade: int
    preco_unitario: float

@dataclass
class Pedido:
    id: str
    usuario_id: str
    itens: List[ItemPedido]
    valor_total: float
    status: str = "pendente"

class SyrosSagaClient:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url
        self.session = None
    
    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
    
    async def start_saga(
        self,
        name: str,
        steps: List[Dict],
        metadata: Optional[Dict] = None
    ) -> Optional[str]:
        """Inicia uma saga"""
        payload = {
            "name": name,
            "steps": steps,
            "metadata": metadata
        }
        
        async with self.session.post(
            f"{self.base_url}/api/v1/sagas",
            json=payload
        ) as response:
            if response.status == 200:
                data = await response.json()
                return data["saga_id"]
            else:
                error_text = await response.text()
                print(f"Erro ao iniciar saga: {response.status} - {error_text}")
                return None
    
    async def get_saga_status(self, saga_id: str) -> Dict:
        """Obtém o status de uma saga"""
        async with self.session.get(
            f"{self.base_url}/api/v1/sagas/{saga_id}/status"
        ) as response:
            if response.status == 200:
                return await response.json()
            else:
                error_text = await response.text()
                print(f"Erro ao obter status da saga: {response.status} - {error_text}")
                return {}

class EcommerceService:
    """Simula um serviço de e-commerce usando Saga Pattern"""
    
    def __init__(self):
        self.produtos = {
            "prod_001": Produto("prod_001", "Smartphone", 899.99, 10),
            "prod_002": Produto("prod_002", "Notebook", 1299.99, 5),
            "prod_003": Produto("prod_003", "Fone de Ouvido", 199.99, 20),
        }
        self.pedidos = {}
        self.estoque_reservado = {}
        self.pagamentos = {}
    
    def criar_saga_processamento_pedido(self, pedido: Pedido) -> List[Dict]:
        """Cria os steps da saga para processamento de pedido"""
        return [
            {
                "name": "validar_pedido",
                "service": "order-service",
                "action": "validate",
                "compensation": "cancel_validation",
                "timeout_seconds": 30,
                "payload": asdict(pedido)
            },
            {
                "name": "reservar_estoque",
                "service": "inventory-service", 
                "action": "reserve",
                "compensation": "release_reservation",
                "timeout_seconds": 45,
                "payload": {
                    "pedido_id": pedido.id,
                    "itens": [asdict(item) for item in pedido.itens]
                }
            },
            {
                "name": "processar_pagamento",
                "service": "payment-service",
                "action": "charge",
                "compensation": "refund",
                "timeout_seconds": 60,
                "payload": {
                    "pedido_id": pedido.id,
                    "usuario_id": pedido.usuario_id,
                    "valor": pedido.valor_total,
                    "metodo": "cartao_credito"
                }
            },
            {
                "name": "confirmar_estoque",
                "service": "inventory-service",
                "action": "confirm",
                "compensation": "restore_stock",
                "timeout_seconds": 30,
                "payload": {
                    "pedido_id": pedido.id,
                    "itens": [asdict(item) for item in pedido.itens]
                }
            },
            {
                "name": "enviar_pedido",
                "service": "shipping-service",
                "action": "ship",
                "compensation": "cancel_shipment",
                "timeout_seconds": 120,
                "payload": {
                    "pedido_id": pedido.id,
                    "endereco": "Rua Exemplo, 123",
                    "itens": [asdict(item) for item in pedido.itens]
                }
            },
            {
                "name": "finalizar_pedido",
                "service": "order-service",
                "action": "complete",
                "compensation": "mark_as_failed",
                "timeout_seconds": 30,
                "payload": {
                    "pedido_id": pedido.id,
                    "status": "enviado"
                }
            }
        ]
    
    async def processar_pedido_com_saga(self, pedido: Pedido) -> Optional[str]:
        """Processa um pedido usando o padrão Saga"""
        print(f"\n📦 Processando pedido {pedido.id}")
        print(f"   Usuario: {pedido.usuario_id}")
        print(f"   Itens: {len(pedido.itens)}")
        print(f"   Valor Total: R$ {pedido.valor_total:.2f}")
        
        async with SyrosSagaClient() as client:
            steps = self.criar_saga_processamento_pedido(pedido)
            
            saga_id = await client.start_saga(
                name=f"processar_pedido_{pedido.id}",
                steps=steps,
                metadata={
                    "pedido_id": pedido.id,
                    "usuario_id": pedido.usuario_id,
                    "valor_total": pedido.valor_total,
                    "tipo": "ecommerce_order"
                }
            )
            
            if saga_id:
                print(f"✅ Saga iniciada com ID: {saga_id}")
                return saga_id
            else:
                print("❌ Falha ao iniciar saga")
                return None
    
    async def monitorar_saga(self, saga_id: str, timeout: int = 300):
        """Monitora o progresso de uma saga"""
        print(f"\n👀 Monitorando saga {saga_id}...")
        
        async with SyrosSagaClient() as client:
            start_time = asyncio.get_event_loop().time()
            
            while True:
                current_time = asyncio.get_event_loop().time()
                if current_time - start_time > timeout:
                    print(f"⏰ Timeout ao monitorar saga {saga_id}")
                    break
                
                status = await client.get_saga_status(saga_id)
                
                if status:
                    saga_status = status.get("status", "unknown")
                    current_step = status.get("current_step_index")
                    
                    print(f"   Status: {saga_status}")
                    if current_step is not None:
                        print(f"   Step atual: {current_step + 1}/6")
                    
                    if saga_status in ["completed", "failed", "compensated"]:
                        if saga_status == "completed":
                            print("✅ Pedido processado com sucesso!")
                        elif saga_status == "compensated":
                            print("🔄 Pedido falhou, compensação executada")
                        else:
                            print("❌ Pedido falhou")
                        break
                
                await asyncio.sleep(2)

async def exemplo_pedido_sucesso():
    """Exemplo de pedido processado com sucesso"""
    print("\n🛒 Exemplo 1: Pedido processado com sucesso")
    print("=" * 50)
    
    service = EcommerceService()
    
    # Criar pedido válido
    pedido = Pedido(
        id=f"pedido_{uuid.uuid4().hex[:8]}",
        usuario_id="user_123",
        itens=[
            ItemPedido("prod_001", 1, 899.99),
            ItemPedido("prod_003", 2, 199.99)
        ],
        valor_total=1299.97
    )
    
    saga_id = await service.processar_pedido_com_saga(pedido)
    
    if saga_id:
        await service.monitorar_saga(saga_id, timeout=60)

async def exemplo_pedido_com_falha():
    """Exemplo de pedido que falha e aciona compensação"""
    print("\n💥 Exemplo 2: Pedido com falha e compensação")
    print("=" * 50)
    
    service = EcommerceService()
    
    # Criar pedido que pode falhar no pagamento
    pedido = Pedido(
        id=f"pedido_{uuid.uuid4().hex[:8]}",
        usuario_id="user_456",
        itens=[
            ItemPedido("prod_002", 3, 1299.99),  # Quantidade alta, pode falhar
            ItemPedido("prod_001", 1, 899.99)
        ],
        valor_total=4799.96
    )
    
    saga_id = await service.processar_pedido_com_saga(pedido)
    
    if saga_id:
        await service.monitorar_saga(saga_id, timeout=60)

async def exemplo_multiplos_pedidos():
    """Exemplo de múltiplos pedidos concorrentes"""
    print("\n🏃‍♂️ Exemplo 3: Múltiplos pedidos concorrentes")
    print("=" * 50)
    
    service = EcommerceService()
    
    # Criar múltiplos pedidos
    pedidos = [
        Pedido(
            id=f"pedido_{uuid.uuid4().hex[:8]}",
            usuario_id=f"user_{i}",
            itens=[ItemPedido("prod_001", 1, 899.99)],
            valor_total=899.99
        )
        for i in range(3)
    ]
    
    # Processar pedidos concorrentemente
    tasks = []
    for pedido in pedidos:
        task = asyncio.create_task(service.processar_pedido_com_saga(pedido))
        tasks.append(task)
    
    saga_ids = await asyncio.gather(*tasks)
    
    # Monitorar todas as sagas
    monitor_tasks = []
    for saga_id in saga_ids:
        if saga_id:
            task = asyncio.create_task(service.monitorar_saga(saga_id, timeout=120))
            monitor_tasks.append(task)
    
    await asyncio.gather(*monitor_tasks)

async def main():
    """Função principal"""
    print("🚀 Exemplo Real: E-commerce com Saga Pattern")
    print("=" * 60)
    print("Este exemplo demonstra como usar o Syros para")
    print("orquestrar transações distribuídas em um sistema de e-commerce")
    
    try:
        await exemplo_pedido_sucesso()
        await asyncio.sleep(2)
        
        await exemplo_pedido_com_falha()
        await asyncio.sleep(2)
        
        await exemplo_multiplos_pedidos()
        
        print("\n✅ Todos os exemplos executados!")
        
    except Exception as e:
        print(f"\n❌ Erro durante execução: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(main())
