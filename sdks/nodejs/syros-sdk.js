/**
 * Syros Node.js SDK
 * SDK oficial para integração com a Syros
 */

const axios = require('axios');
const WebSocket = require('ws');

class SyrosClient {
    constructor(endpoint = 'http://localhost:8080', apiKey = null) {
        this.endpoint = endpoint.replace(/\/$/, '');
        this.apiKey = apiKey;
        this.client = axios.create({
            baseURL: this.endpoint,
            headers: {
                'Content-Type': 'application/json',
                ...(apiKey && { 'Authorization': `Bearer ${apiKey}` })
            }
        });
    }

    /**
     * Verifica a saúde da plataforma
     */
    async healthCheck() {
        const response = await this.client.get('/health');
        return response.data;
    }

    /**
     * Adquire um lock distribuído
     */
    async acquireLock(lockRequest) {
        const response = await this.client.post('/api/v1/locks', {
            key: lockRequest.key,
            owner: lockRequest.owner,
            ttl_seconds: lockRequest.ttlSeconds || 300,
            metadata: lockRequest.metadata,
            wait_timeout_seconds: lockRequest.waitTimeoutSeconds
        });
        return response.data;
    }

    /**
     * Libera um lock distribuído
     */
    async releaseLock(key) {
        const response = await this.client.delete(`/api/v1/locks/${key}`);
        return response.data;
    }

    /**
     * Obtém o status de um lock
     */
    async getLockStatus(key) {
        const response = await this.client.get(`/api/v1/locks/${key}/status`);
        return response.data;
    }

    /**
     * Inicia uma saga
     */
    async startSaga(sagaRequest) {
        const response = await this.client.post('/api/v1/sagas', {
            name: sagaRequest.name,
            steps: sagaRequest.steps.map(step => ({
                name: step.name,
                action: step.action,
                compensation: step.compensation,
                timeout_seconds: step.timeoutSeconds,
                retry_policy: step.retryPolicy,
                payload: step.payload
            })),
            metadata: sagaRequest.metadata
        });
        return response.data;
    }

    /**
     * Obtém o status de uma saga
     */
    async getSagaStatus(sagaId) {
        const response = await this.client.get(`/api/v1/sagas/${sagaId}/status`);
        return response.data;
    }

    /**
     * Adiciona um evento ao event store
     */
    async appendEvent(eventRequest) {
        const response = await this.client.post('/api/v1/events', {
            stream_id: eventRequest.streamId,
            event_type: eventRequest.eventType,
            data: eventRequest.data,
            metadata: eventRequest.metadata
        });
        return response.data;
    }

    /**
     * Obtém eventos de um stream
     */
    async getEvents(streamId, fromVersion = null) {
        let url = `/api/v1/events/${streamId}`;
        if (fromVersion !== null) {
            url += `?from_version=${fromVersion}`;
        }
        const response = await this.client.get(url);
        return response.data;
    }

    /**
     * Define um valor no cache
     */
    async setCache(cacheRequest) {
        const response = await this.client.post(`/api/v1/cache/${cacheRequest.key}`, {
            value: cacheRequest.value,
            ttl_seconds: cacheRequest.ttlSeconds,
            tags: cacheRequest.tags || []
        });
        return response.data;
    }

    /**
     * Obtém um valor do cache
     */
    async getCache(key) {
        const response = await this.client.get(`/api/v1/cache/${key}`);
        return response.data;
    }

    /**
     * Remove um valor do cache
     */
    async deleteCache(key) {
        const response = await this.client.delete(`/api/v1/cache/${key}`);
        return response.data;
    }

    /**
     * Invalida cache por tag
     */
    async invalidateCacheByTag(tag) {
        const response = await this.client.post(`/api/v1/cache/invalidate/${tag}`);
        return response.data;
    }

    /**
     * Obtém estatísticas do cache
     */
    async getCacheStats() {
        const response = await this.client.get('/api/v1/cache/stats');
        return response.data;
    }
}

class SyrosWebSocketClient {
    constructor(endpoint = 'ws://localhost:8080/ws') {
        this.endpoint = endpoint;
        this.ws = null;
        this.eventHandlers = new Map();
    }

    /**
     * Conecta ao WebSocket
     */
    async connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.endpoint);
            
            this.ws.on('open', () => {
                console.log('Conectado ao WebSocket da Syros');
                resolve();
            });
            
            this.ws.on('message', (data) => {
                try {
                    const message = JSON.parse(data.toString());
                    this.handleMessage(message);
                } catch (error) {
                    console.error('Erro ao processar mensagem:', error);
                }
            });
            
            this.ws.on('error', (error) => {
                console.error('Erro no WebSocket:', error);
                reject(error);
            });
            
            this.ws.on('close', () => {
                console.log('WebSocket desconectado');
            });
        });
    }

    /**
     * Desconecta do WebSocket
     */
    disconnect() {
        if (this.ws) {
            this.ws.close();
        }
    }

    /**
     * Envia um ping
     */
    sendPing() {
        if (!this.ws) {
            throw new Error('WebSocket não conectado');
        }
        this.ws.send(JSON.stringify({ type: 'ping' }));
    }

    /**
     * Inscreve-se para receber eventos
     */
    subscribe() {
        if (!this.ws) {
            throw new Error('WebSocket não conectado');
        }
        this.ws.send(JSON.stringify({ type: 'subscribe' }));
    }

    /**
     * Registra um handler para eventos
     */
    on(eventType, handler) {
        this.eventHandlers.set(eventType, handler);
    }

    /**
     * Processa mensagens recebidas
     */
    handleMessage(message) {
        const { type, data, timestamp } = message;
        
        if (this.eventHandlers.has(type)) {
            const handler = this.eventHandlers.get(type);
            handler(data, timestamp);
        } else {
            console.log('Evento não tratado:', message);
        }
    }
}

// Exemplo de uso
async function exemploUso() {
    // Cliente REST
    const client = new SyrosClient('http://localhost:8080');
    
    try {
        // Verificar saúde
        const health = await client.healthCheck();
        console.log('Status da plataforma:', health);
        
        // Adquirir lock
        const lockResponse = await client.acquireLock({
            key: 'meu-recurso',
            owner: 'cliente-nodejs',
            ttlSeconds: 60
        });
        console.log('Lock adquirido:', lockResponse);
        
        // Liberar lock
        const releaseResult = await client.releaseLock('meu-recurso');
        console.log('Lock liberado:', releaseResult);
        
        // Usar cache
        const cacheResponse = await client.setCache({
            key: 'meu-cache',
            value: { dados: 'importantes' },
            ttlSeconds: 300
        });
        console.log('Cache definido:', cacheResponse);
        
        // Obter cache
        const cacheData = await client.getCache('meu-cache');
        console.log('Cache obtido:', cacheData);
        
    } catch (error) {
        console.error('Erro:', error.message);
    }
    
    // Cliente WebSocket
    const wsClient = new SyrosWebSocketClient('ws://localhost:8080/ws');
    
    try {
        await wsClient.connect();
        
        wsClient.on('welcome', (data) => {
            console.log('Bem-vindo:', data);
        });
        
        wsClient.on('pong', (data) => {
            console.log('Pong recebido:', data);
        });
        
        wsClient.on('subscribed', (data) => {
            console.log('Inscrito:', data);
        });
        
        wsClient.subscribe();
        wsClient.sendPing();
        
        // Manter conexão por 10 segundos
        setTimeout(() => {
            wsClient.disconnect();
        }, 10000);
        
    } catch (error) {
        console.error('Erro WebSocket:', error.message);
    }
}

// Executar exemplo se chamado diretamente
if (require.main === module) {
    exemploUso().catch(console.error);
}

module.exports = {
    SyrosClient,
    SyrosWebSocketClient
};
