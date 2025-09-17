using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Newtonsoft.Json.Linq;

namespace SyrosSDK
{
    /// <summary>
    /// Example usage of Syros Platform C# SDK
    /// </summary>
    class Example
    {
        static async Task Main(string[] args)
        {
            using var client = new SyrosClient();

            try
            {
                Console.WriteLine("🚀 Syros Platform C# SDK Example");
                Console.WriteLine(new string('=', 50));

                // Test REST API
                await TestRestApiAsync(client);

                // Test WebSocket API
                await TestWebSocketApiAsync(client);

                Console.WriteLine("\n" + new string('=', 50));
                Console.WriteLine("✅ Example completed successfully!");

            }
            catch (Exception e)
            {
                Console.WriteLine($"❌ Error: {e.Message}");
                Console.WriteLine(e.StackTrace);
            }
        }

        private static async Task TestRestApiAsync(SyrosClient client)
        {
            Console.WriteLine("\n📡 Testing REST API...");

            // Health check
            Console.WriteLine("  🏥 Health check...");
            var health = await client.HealthCheckAsync();
            Console.WriteLine($"    ✅ Health: {health["status"]}");

            // Lock operations
            Console.WriteLine("  🔒 Testing locks...");
            var lockResult = await client.AcquireLockAsync("csharp_test_lock", "csharp_client", 60, "C# SDK test");
            Console.WriteLine($"    ✅ Lock acquired: {lockResult["lock_id"]}");

            var lockStatus = await client.GetLockStatusAsync("csharp_test_lock");
            Console.WriteLine($"    ✅ Lock status: {lockStatus["is_locked"]}");

            // Saga operations
            Console.WriteLine("  🔄 Testing sagas...");
            var steps = new object[]
            {
                new { name = "step1", action = "create_order", compensation = "cancel_order" },
                new { name = "step2", action = "charge_payment", compensation = "refund_payment" }
            };
            var sagaResult = await client.StartSagaAsync("csharp_test_saga", steps, null);
            Console.WriteLine($"    ✅ Saga started: {sagaResult["saga_id"]}");

            // Event operations
            Console.WriteLine("  📝 Testing events...");
            var eventData = new { user_id = "csharp_user_123", action = "login" };
            var eventResult = await client.AppendEventAsync("csharp_stream_123", "UserLoggedIn", eventData, null);
            Console.WriteLine($"    ✅ Event appended: {eventResult["event_id"]}");

            // Cache operations
            Console.WriteLine("  💾 Testing cache...");
            var cacheData = new { name = "C# Widget", price = 29.99 };
            var cacheResult = await client.SetCacheAsync("csharp_cache_key", cacheData, 300, new[] { "csharp", "widgets" });
            Console.WriteLine($"    ✅ Cache set: {cacheResult["message"]}");

            // Metrics
            Console.WriteLine("  📊 Testing metrics...");
            var metrics = await client.GetMetricsAsync();
            Console.WriteLine($"    ✅ Metrics retrieved: {metrics.Length} characters");
        }

        private static async Task TestWebSocketApiAsync(SyrosClient client)
        {
            Console.WriteLine("\n🔌 Testing WebSocket API...");

            await client.ConnectWebSocketAsync();

            // Start listening for messages in background
            var listenTask = Task.Run(async () =>
            {
                await client.ListenWebSocketAsync(async message =>
                {
                    Console.WriteLine($"    📨 WebSocket message: {message}");
                });
            });

            // Send some messages
            await Task.Delay(1000); // Wait for connection
            await client.SendWebSocketMessageAsync("ping", new { timestamp = DateTimeOffset.Now.ToUnixTimeMilliseconds() });
            await Task.Delay(1000);
            await client.SendWebSocketMessageAsync("subscribe", new { topic = "locks" });
            await Task.Delay(2000);

            await client.DisconnectWebSocketAsync();
            await listenTask;
        }
    }
}
