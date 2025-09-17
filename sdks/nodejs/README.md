# Syros Node.js SDK

SDK oficial para integração com a Syros em Node.js.

## Instalação

```bash
npm install
```

## Uso Básico

### Cliente REST

```javascript
const { SyrosClient } = require('./syros-sdk');

async function exemplo() {
    const client = new SyrosClient('http://localhost:8080');
    
    // Verificar saúde da plataforma
    const health = await client.healthCheck();
    console.log('Status:', health);
    
    // Adquirir lock
    const lockResponse = await client.acquireLock({
        key: 'meu-recurso',
        owner: 'cliente-nodejs',
        ttlSeconds: 60
    });
    console.log('Lock:', lockResponse);
    
    // Usar cache
    const cacheResponse = await client.setCache({
        key: 'dados',
        value: { info: 'importante' },
        ttlSeconds: 300
    });
    console.log('Cache:', cacheResponse);
}

exemplo().catch(console.error);
```

### Cliente WebSocket

```javascript
const { SyrosWebSocketClient } = require('./syros-sdk');

async function exemploWebSocket() {
    const wsClient = new SyrosWebSocketClient('ws://localhost:8080/ws');
    
    await wsClient.connect();
    
    wsClient.on('welcome', (data) => {
        console.log('Bem-vindo:', data);
    });
    
    wsClient.on('lock_event', (data) => {
        console.log('Evento de lock:', data);
    });
    
    wsClient.subscribe();
}

exemploWebSocket().catch(console.error);
```

## Funcionalidades

- **Locks Distribuídos**: Adquirir e liberar locks
- **Sagas**: Orquestração de transações distribuídas
- **Event Store**: Armazenamento de eventos
- **Cache**: Cache distribuído com TTL
- **WebSocket**: Eventos em tempo real
- **Health Checks**: Verificação de saúde da plataforma

## Exemplos

Execute o exemplo:

```bash
npm run example
```

Veja o arquivo `syros-sdk.js` para exemplos completos de uso.
