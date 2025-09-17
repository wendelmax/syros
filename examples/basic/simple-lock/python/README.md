# Exemplo Básico: Locks Distribuídos com Python

Este exemplo demonstra como usar o Syros para implementar locks distribuídos em Python.

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

## O que o exemplo demonstra

1. **Exemplo Básico**: Como adquirir, usar e liberar um lock distribuído
2. **Exemplo de Concorrência**: Como múltiplos workers competem por um recurso compartilhado

## Funcionalidades demonstradas

- ✅ Aquisição de locks com TTL
- ✅ Liberação manual de locks
- ✅ Verificação de status de locks
- ✅ Timeout ao aguardar locks
- ✅ Metadados personalizados
- ✅ Múltiplos workers concorrentes

## Saída esperada

```
🚀 Exemplos de uso do Syros - Locks Distribuídos
============================================================

🔒 Exemplo básico de locks distribuídos com Syros

1. Verificando status inicial do lock 'recurso_critico_001'...
Status: {
  "key": "recurso_critico_001",
  "lock_id": null,
  "owner": null,
  "acquired_at": null,
  "expires_at": null,
  "metadata": null,
  "is_locked": false
}

2. Tentando adquirir lock para 'recurso_critico_001'...
✅ Lock adquirido com sucesso! ID: abc123...

3. Verificando status após aquisição...
Status: {
  "key": "recurso_critico_001",
  "lock_id": "abc123...",
  "owner": "worker-001",
  "acquired_at": "2024-01-01T12:00:00Z",
  "expires_at": "2024-01-01T12:00:30Z",
  "metadata": "Exemplo Python - operação crítica",
  "is_locked": true
}

4. Simulando trabalho crítico por 5 segundos...

5. Liberando lock...
✅ Lock liberado com sucesso!

6. Verificando status após liberação...
Status: {
  "key": "recurso_critico_001",
  "lock_id": null,
  "owner": null,
  "acquired_at": null,
  "expires_at": null,
  "metadata": null,
  "is_locked": false
}

🏃‍♂️ Exemplo de concorrência entre workers
Worker A: Tentando adquirir lock...
Worker A: ✅ Lock adquirido após 0.05s (ID: def456...)
Worker B: Tentando adquirir lock...
Worker C: Tentando adquirir lock...
Worker A: Executando trabalho por 3s...
Worker A: ✅ Lock liberado
Worker B: ✅ Lock adquirido após 3.12s (ID: ghi789...)
Worker B: Executando trabalho por 3s...
Worker B: ✅ Lock liberado
Worker C: ✅ Lock adquirido após 6.18s (ID: jkl012...)
Worker C: Executando trabalho por 3s...
Worker C: ✅ Lock liberado

✅ Exemplos executados com sucesso!
```
