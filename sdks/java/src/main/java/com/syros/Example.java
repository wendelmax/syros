package com.syros;

import com.fasterxml.jackson.databind.JsonNode;
import okhttp3.ws.WebSocketListener;
import okio.ByteString;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

/**
 * Example usage of Syros Platform Java SDK
 */
public class Example {
    public static void main(String[] args) {
        SyrosClient client = new SyrosClient();

        try {
            System.out.println("🚀 Syros Platform Java SDK Example");
            System.out.println("=" * 50);

            // Test REST API
            testRestApi(client);

            // Test WebSocket API
            testWebSocketApi(client);

            System.out.println("\n" + "=" * 50);
            System.out.println("✅ Example completed successfully!");

        } catch (Exception e) {
            System.err.println("❌ Error: " + e.getMessage());
            e.printStackTrace();
        } finally {
            try {
                client.close();
            } catch (IOException e) {
                System.err.println("Error closing client: " + e.getMessage());
            }
        }
    }

    private static void testRestApi(SyrosClient client) throws IOException {
        System.out.println("\n📡 Testing REST API...");

        // Health check
        System.out.println("  🏥 Health check...");
        JsonNode health = client.healthCheck();
        System.out.println("    ✅ Health: " + health.get("status").asText());

        // Lock operations
        System.out.println("  🔒 Testing locks...");
        JsonNode lockResult = client.acquireLock("java_test_lock", "java_client", 60L, "Java SDK test");
        System.out.println("    ✅ Lock acquired: " + lockResult.get("lock_id").asText());
        
        JsonNode lockStatus = client.getLockStatus("java_test_lock");
        System.out.println("    ✅ Lock status: " + lockStatus.get("is_locked").asBoolean());

        // Saga operations
        System.out.println("  🔄 Testing sagas...");
        Object[] steps = {
            Map.of("name", "step1", "action", "create_order", "compensation", "cancel_order"),
            Map.of("name", "step2", "action", "charge_payment", "compensation", "refund_payment")
        };
        JsonNode sagaResult = client.startSaga("java_test_saga", steps, null);
        System.out.println("    ✅ Saga started: " + sagaResult.get("saga_id").asText());

        // Event operations
        System.out.println("  📝 Testing events...");
        Map<String, Object> eventData = new HashMap<>();
        eventData.put("user_id", "java_user_123");
        eventData.put("action", "login");
        
        JsonNode eventResult = client.appendEvent("java_stream_123", "UserLoggedIn", eventData, null);
        System.out.println("    ✅ Event appended: " + eventResult.get("event_id").asText());

        // Cache operations
        System.out.println("  💾 Testing cache...");
        Map<String, Object> cacheData = new HashMap<>();
        cacheData.put("name", "Java Widget");
        cacheData.put("price", 29.99);
        
        JsonNode cacheResult = client.setCache("java_cache_key", cacheData, 300L, new String[]{"java", "widgets"});
        System.out.println("    ✅ Cache set: " + cacheResult.get("message").asText());

        // Metrics
        System.out.println("  📊 Testing metrics...");
        JsonNode metrics = client.getMetrics();
        System.out.println("    ✅ Metrics retrieved: " + metrics.toString().length() + " characters");
    }

    private static void testWebSocketApi(SyrosClient client) throws IOException {
        System.out.println("\n🔌 Testing WebSocket API...");

        WebSocketListener listener = new WebSocketListener() {
            @Override
            public void onOpen(okhttp3.ws.WebSocket webSocket, okhttp3.Response response) {
                System.out.println("    ✅ WebSocket connected");
            }

            @Override
            public void onMessage(okhttp3.ResponseBody message) throws IOException {
                System.out.println("    📨 WebSocket message: " + message.string());
            }

            @Override
            public void onPong(ByteString payload) {
                System.out.println("    🏓 WebSocket pong received");
            }

            @Override
            public void onClose(int code, String reason) {
                System.out.println("    🔌 WebSocket closed: " + code + " - " + reason);
            }

            @Override
            public void onFailure(IOException e, okhttp3.Response response) {
                System.err.println("    ❌ WebSocket error: " + e.getMessage());
            }
        };

        client.connectWebSocket(listener);

        // Send some messages
        Thread.sleep(1000); // Wait for connection
        client.sendWebSocketMessage("ping", Map.of("timestamp", System.currentTimeMillis()));
        Thread.sleep(1000);
        client.sendWebSocketMessage("subscribe", Map.of("topic", "locks"));
        Thread.sleep(2000);

        client.disconnectWebSocket();
    }
}
