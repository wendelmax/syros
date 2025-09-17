using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;
using System.Net.WebSockets;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Microsoft.Extensions.Logging;

namespace SyrosSDK
{
    /// <summary>
    /// Syros Platform C# SDK
    /// 
    /// Provides a C# interface to interact with Syros Platform,
    /// including REST APIs and WebSocket communication.
    /// </summary>
    public class SyrosClient : IDisposable
    {
        private readonly HttpClient _httpClient;
        private readonly ILogger<SyrosClient> _logger;
        private readonly string _restUrl;
        private readonly string _wsUrl;
        private ClientWebSocket? _webSocket;

        public SyrosClient() : this("http://localhost:8080", "ws://localhost:8080/ws")
        {
        }

        public SyrosClient(string restUrl, string wsUrl)
        {
            _restUrl = restUrl;
            _wsUrl = wsUrl;
            _httpClient = new HttpClient();
            _httpClient.Timeout = TimeSpan.FromSeconds(30);
            
            // Setup logging
            var loggerFactory = LoggerFactory.Create(builder => builder.AddConsole());
            _logger = loggerFactory.CreateLogger<SyrosClient>();
        }

        // --- REST API Methods ---

        /// <summary>
        /// Health check endpoint
        /// </summary>
        public async Task<JObject> HealthCheckAsync()
        {
            return await SendRestRequestAsync<JObject>("GET", "/health", null);
        }

        /// <summary>
        /// Acquire a distributed lock
        /// </summary>
        public async Task<JObject> AcquireLockAsync(string key, string owner, long? ttlSeconds = null, string? metadata = null)
        {
            var payload = new
            {
                key,
                owner,
                ttl_seconds = ttlSeconds,
                metadata
            };

            return await SendRestRequestAsync<JObject>("POST", "/api/v1/locks", payload);
        }

        /// <summary>
        /// Release a distributed lock
        /// </summary>
        public async Task<JObject> ReleaseLockAsync(string key, string lockId, string owner)
        {
            var payload = new
            {
                lock_id = lockId,
                owner
            };

            return await SendRestRequestAsync<JObject>("DELETE", $"/api/v1/locks/{key}", payload);
        }

        /// <summary>
        /// Get lock status
        /// </summary>
        public async Task<JObject> GetLockStatusAsync(string key)
        {
            return await SendRestRequestAsync<JObject>("GET", $"/api/v1/locks/{key}/status", null);
        }

        /// <summary>
        /// Start a saga
        /// </summary>
        public async Task<JObject> StartSagaAsync(string name, object[] steps, Dictionary<string, string>? metadata = null)
        {
            var payload = new
            {
                name,
                steps,
                metadata
            };

            return await SendRestRequestAsync<JObject>("POST", "/api/v1/sagas", payload);
        }

        /// <summary>
        /// Get saga status
        /// </summary>
        public async Task<JObject> GetSagaStatusAsync(string sagaId)
        {
            return await SendRestRequestAsync<JObject>("GET", $"/api/v1/sagas/{sagaId}/status", null);
        }

        /// <summary>
        /// Append event to event store
        /// </summary>
        public async Task<JObject> AppendEventAsync(string streamId, string eventType, object data, Dictionary<string, string>? metadata = null)
        {
            var payload = new
            {
                stream_id = streamId,
                event_type = eventType,
                data,
                metadata
            };

            return await SendRestRequestAsync<JObject>("POST", "/api/v1/events", payload);
        }

        /// <summary>
        /// Get events from event store
        /// </summary>
        public async Task<JObject> GetEventsAsync(string streamId)
        {
            return await SendRestRequestAsync<JObject>("GET", $"/api/v1/events/{streamId}", null);
        }

        /// <summary>
        /// Set cache value
        /// </summary>
        public async Task<JObject> SetCacheAsync(string key, object value, long? ttlSeconds = null, string[]? tags = null)
        {
            var payload = new
            {
                value,
                ttl_seconds = ttlSeconds,
                tags
            };

            return await SendRestRequestAsync<JObject>("POST", $"/api/v1/cache/{key}", payload);
        }

        /// <summary>
        /// Get cache value
        /// </summary>
        public async Task<JObject> GetCacheAsync(string key)
        {
            return await SendRestRequestAsync<JObject>("GET", $"/api/v1/cache/{key}", null);
        }

        /// <summary>
        /// Delete cache value
        /// </summary>
        public async Task<JObject> DeleteCacheAsync(string key)
        {
            return await SendRestRequestAsync<JObject>("DELETE", $"/api/v1/cache/{key}", null);
        }

        /// <summary>
        /// Get metrics
        /// </summary>
        public async Task<string> GetMetricsAsync()
        {
            return await SendRestRequestAsync<string>("GET", "/metrics", null);
        }

        // --- WebSocket API Methods ---

        /// <summary>
        /// Connect to WebSocket
        /// </summary>
        public async Task ConnectWebSocketAsync()
        {
            _webSocket = new ClientWebSocket();
            await _webSocket.ConnectAsync(new Uri(_wsUrl), CancellationToken.None);
            _logger.LogInformation("WebSocket connected to {Url}", _wsUrl);
        }

        /// <summary>
        /// Send WebSocket message
        /// </summary>
        public async Task SendWebSocketMessageAsync(string type, object data)
        {
            if (_webSocket?.State == WebSocketState.Open)
            {
                var message = new
                {
                    type,
                    data
                };

                var jsonMessage = JsonConvert.SerializeObject(message);
                var bytes = Encoding.UTF8.GetBytes(jsonMessage);
                var buffer = new ArraySegment<byte>(bytes);

                await _webSocket.SendAsync(buffer, WebSocketMessageType.Text, true, CancellationToken.None);
            }
        }

        /// <summary>
        /// Listen for WebSocket messages
        /// </summary>
        public async Task ListenWebSocketAsync(Func<string, Task> onMessage)
        {
            if (_webSocket?.State == WebSocketState.Open)
            {
                var buffer = new byte[4096];
                while (_webSocket.State == WebSocketState.Open)
                {
                    var result = await _webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None);
                    if (result.MessageType == WebSocketMessageType.Text)
                    {
                        var message = Encoding.UTF8.GetString(buffer, 0, result.Count);
                        await onMessage(message);
                    }
                }
            }
        }

        /// <summary>
        /// Disconnect WebSocket
        /// </summary>
        public async Task DisconnectWebSocketAsync()
        {
            if (_webSocket?.State == WebSocketState.Open)
            {
                await _webSocket.CloseAsync(WebSocketCloseStatus.NormalClosure, "Client disconnect", CancellationToken.None);
                _webSocket = null;
                _logger.LogInformation("WebSocket disconnected");
            }
        }

        // --- Private Helper Methods ---

        private async Task<T> SendRestRequestAsync<T>(string method, string path, object? data)
        {
            var url = _restUrl + path;
            HttpRequestMessage request;

            if (data != null)
            {
                var jsonData = JsonConvert.SerializeObject(data);
                var content = new StringContent(jsonData, Encoding.UTF8, "application/json");

                request = method.ToUpper() switch
                {
                    "POST" => new HttpRequestMessage(HttpMethod.Post, url) { Content = content },
                    "PUT" => new HttpRequestMessage(HttpMethod.Put, url) { Content = content },
                    "DELETE" => new HttpRequestMessage(HttpMethod.Delete, url) { Content = content },
                    _ => new HttpRequestMessage(HttpMethod.Get, url)
                };
            }
            else
            {
                request = method.ToUpper() switch
                {
                    "DELETE" => new HttpRequestMessage(HttpMethod.Delete, url),
                    _ => new HttpRequestMessage(HttpMethod.Get, url)
                };
            }

            request.RequestUri = new Uri(url);

            try
            {
                var response = await _httpClient.SendAsync(request);
                response.EnsureSuccessStatusCode();

                var responseContent = await response.Content.ReadAsStringAsync();

                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseContent;
                }

                return JsonConvert.DeserializeObject<T>(responseContent) ?? throw new InvalidOperationException("Failed to deserialize response");
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error sending REST request {Method} {Url}", method, url);
                throw;
            }
        }

        /// <summary>
        /// Dispose resources
        /// </summary>
        public void Dispose()
        {
            _httpClient?.Dispose();
            _webSocket?.Dispose();
        }
    }
}
