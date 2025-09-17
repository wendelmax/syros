package main

import (
	"fmt"
	"log"
	"time"
)

func main() {
	client := NewSyrosClient()
	defer client.Close()

	fmt.Println("🚀 Syros Go SDK Example")
	fmt.Println("==================================================")

	// Test REST API
	testRestAPI(client)

	// Test WebSocket API
	testWebSocketAPI(client)

	fmt.Println("\n==================================================")
	fmt.Println("✅ Example completed successfully!")
}

func testRestAPI(client *SyrosClient) {
	fmt.Println("\n📡 Testing REST API...")

	// Health check
	fmt.Println("  🏥 Health check...")
	health, err := client.HealthCheck()
	if err != nil {
		log.Printf("    ❌ Health check failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Health: %v\n", health["status"])

	// Lock operations
	fmt.Println("  🔒 Testing locks...")
	ttlSeconds := int64(60)
	metadata := "Go SDK test"
	lockResult, err := client.AcquireLock("go_test_lock", "go_client", &ttlSeconds, &metadata)
	if err != nil {
		log.Printf("    ❌ Acquire lock failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Lock acquired: %v\n", lockResult["lock_id"])

	lockStatus, err := client.GetLockStatus("go_test_lock")
	if err != nil {
		log.Printf("    ❌ Get lock status failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Lock status: %v\n", lockStatus["is_locked"])

	// Saga operations
	fmt.Println("  🔄 Testing sagas...")
	steps := []map[string]interface{}{
		{"name": "step1", "action": "create_order", "compensation": "cancel_order"},
		{"name": "step2", "action": "charge_payment", "compensation": "refund_payment"},
	}
	sagaResult, err := client.StartSaga("go_test_saga", steps, nil)
	if err != nil {
		log.Printf("    ❌ Start saga failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Saga started: %v\n", sagaResult["saga_id"])

	// Event operations
	fmt.Println("  📝 Testing events...")
	eventData := map[string]interface{}{
		"user_id": "go_user_123",
		"action":  "login",
	}
	eventResult, err := client.AppendEvent("go_stream_123", "UserLoggedIn", eventData, nil)
	if err != nil {
		log.Printf("    ❌ Append event failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Event appended: %v\n", eventResult["event_id"])

	// Cache operations
	fmt.Println("  💾 Testing cache...")
	cacheData := map[string]interface{}{
		"name":  "Go Widget",
		"price": 29.99,
	}
	cacheTTL := int64(300)
	tags := []string{"go", "widgets"}
	cacheResult, err := client.SetCache("go_cache_key", cacheData, &cacheTTL, tags)
	if err != nil {
		log.Printf("    ❌ Set cache failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Cache set: %v\n", cacheResult["message"])

	// Metrics
	fmt.Println("  📊 Testing metrics...")
	metrics, err := client.GetMetrics()
	if err != nil {
		log.Printf("    ❌ Get metrics failed: %v", err)
		return
	}
	fmt.Printf("    ✅ Metrics retrieved: %d characters\n", len(metrics))
}

func testWebSocketAPI(client *SyrosClient) {
	fmt.Println("\n🔌 Testing WebSocket API...")

	err := client.ConnectWebSocket()
	if err != nil {
		log.Printf("    ❌ WebSocket connection failed: %v", err)
		return
	}

	// Start listening for messages in background
	go func() {
		err := client.ListenWebSocket(func(message string) {
			fmt.Printf("    📨 WebSocket message: %s\n", message)
		})
		if err != nil {
			log.Printf("    ❌ WebSocket listen error: %v", err)
		}
	}()

	// Send some messages
	time.Sleep(1 * time.Second) // Wait for connection
	err = client.SendWebSocketMessage("ping", map[string]interface{}{
		"timestamp": time.Now().UnixMilli(),
	})
	if err != nil {
		log.Printf("    ❌ Send ping failed: %v", err)
		return
	}

	time.Sleep(1 * time.Second)
	err = client.SendWebSocketMessage("subscribe", map[string]interface{}{
		"topic": "locks",
	})
	if err != nil {
		log.Printf("    ❌ Send subscribe failed: %v", err)
		return
	}

	time.Sleep(2 * time.Second)
	err = client.DisconnectWebSocket()
	if err != nil {
		log.Printf("    ❌ WebSocket disconnect failed: %v", err)
	}
}
