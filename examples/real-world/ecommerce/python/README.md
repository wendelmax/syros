# Exemplo Real: E-commerce com Saga Pattern

Este exemplo demonstra como usar o Syros para implementar o padrão Saga em um sistema de e-commerce real, orquestrando transações distribuídas para processamento de pedidos.

## Cenário

O sistema de e-commerce processa pedidos através de múltiplos microserviços:

1. **Order Service**: Validação e gerenciamento de pedidos
2. **Inventory Service**: Gerenciamento de estoque
3. **Payment Service**: Processamento de pagamentos
4. **Shipping Service**: Envio de produtos
5. **Notification Service**: Notificações ao usuário

## Saga Flow

```
Validar Pedido → Reservar Estoque → Processar Pagamento → Confirmar Estoque → Enviar Pedido → Finalizar Pedido
      ↓               ↓                    ↓                    ↓              ↓              ↓
   Cancelar        Liberar           Estornar           Restaurar        Cancelar      Marcar como
  Validação       Reserva           Pagamento           Estoque          Envio         Falhou
```

## Pré-requisitos

- Python 3.8+
- Syros rodando (padrão: http://localhost:8080)

## Instalação

```bash
pip install -r requirements.txt
```

## Execução

```bash
python main.py
```

## Exemplos Demonstrados

### 1. Pedido com Sucesso
- Demonstra um pedido que passa por todos os steps com sucesso
- Mostra o progresso da saga em tempo real

### 2. Pedido com Falha e Compensação
- Simula uma falha no processamento (ex: pagamento rejeitado)
- Demonstra como a compensação é executada automaticamente

### 3. Múltiplos Pedidos Concorrentes
- Processa vários pedidos simultaneamente
- Mostra como o Syros gerencia múltiplas sagas

## Funcionalidades Demonstradas

- ✅ Orquestração de transações distribuídas
- ✅ Compensação automática em caso de falha
- ✅ Monitoramento de progresso de sagas
- ✅ Processamento concorrente de múltiplas sagas
- ✅ Timeout e retry policies
- ✅ Metadados personalizados para auditoria

## Saída Esperada

```
🚀 Exemplo Real: E-commerce com Saga Pattern
============================================================
Este exemplo demonstra como usar o Syros para
orquestrar transações distribuídas em um sistema de e-commerce

🛒 Exemplo 1: Pedido processado com sucesso
==================================================

📦 Processando pedido pedido_a1b2c3d4
   Usuario: user_123
   Itens: 2
   Valor Total: R$ 1299.97
✅ Saga iniciada com ID: saga_xyz789...

👀 Monitorando saga saga_xyz789...
   Status: running
   Step atual: 1/6
   Status: running
   Step atual: 2/6
   Status: running
   Step atual: 3/6
   Status: running
   Step atual: 4/6
   Status: running
   Step atual: 5/6
   Status: running
   Step atual: 6/6
   Status: completed
✅ Pedido processado com sucesso!

💥 Exemplo 2: Pedido com falha e compensação
==================================================

📦 Processando pedido pedido_e5f6g7h8
   Usuario: user_456
   Itens: 2
   Valor Total: R$ 4799.96
✅ Saga iniciada com ID: saga_abc123...

👀 Monitorando saga saga_abc123...
   Status: running
   Step atual: 1/6
   Status: running
   Step atual: 2/6
   Status: compensating
   Status: compensated
🔄 Pedido falhou, compensação executada

🏃‍♂️ Exemplo 3: Múltiplos pedidos concorrentes
==================================================

📦 Processando pedido pedido_i9j0k1l2
   Usuario: user_0
   Itens: 1
   Valor Total: R$ 899.99
✅ Saga iniciada com ID: saga_def456...

📦 Processando pedido pedido_m3n4o5p6
   Usuario: user_1
   Itens: 1
   Valor Total: R$ 899.99
✅ Saga iniciada com ID: saga_ghi789...

📦 Processando pedido pedido_q7r8s9t0
   Usuario: user_2
   Itens: 1
   Valor Total: R$ 899.99
✅ Saga iniciada com ID: saga_jkl012...

👀 Monitorando saga saga_def456...
👀 Monitorando saga saga_ghi789...
👀 Monitorando saga saga_jkl012...
   Status: completed
✅ Pedido processado com sucesso!
   Status: completed
✅ Pedido processado com sucesso!
   Status: completed
✅ Pedido processado com sucesso!

✅ Todos os exemplos executados!
```

## Arquitetura

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   E-commerce    │    │  Syros │    │  Microservices  │
│     Client      │    │                 │    │                 │
│                 │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ • Criar Pedido  │───►│ │ Saga        │ │◄──►│ │ Order       │ │
│ • Monitorar     │    │ │ Orchestrator│ │    │ │ Service     │ │
│   Progresso     │    │ └─────────────┘ │    │ └─────────────┘ │
│                 │    │                 │    │                 │
│                 │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│                 │    │ │ Event       │ │    │ │ Payment     │ │
│                 │    │ │ Store       │ │    │ │ Service     │ │
│                 │    │ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Benefícios do Saga Pattern

1. **Consistência Eventual**: Garante que o sistema eventualmente chegue a um estado consistente
2. **Tolerância a Falhas**: Compensação automática em caso de falha
3. **Observabilidade**: Rastreamento completo do progresso das transações
4. **Escalabilidade**: Cada step pode ser executado por um serviço independente
5. **Flexibilidade**: Fácil adição/remoção de steps na transação
