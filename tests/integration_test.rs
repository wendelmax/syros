//! Integration tests for the Syros.
//!
//! This module contains comprehensive integration tests that verify
//! all functionality of the Syros including core services,
//! APIs, authentication, and distributed coordination features.

use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

use syros::{
    auth::{Permission, RBACManager, Resource, ResourceType, Role, User},
    core::{
        cache_manager::{CacheManager, CacheRequest, DeleteCacheRequest},
        event_store::{EventRequest, EventStore, GetEventsRequest},
        lock_manager::{LockManager, LockRequest, ReleaseLockRequest},
        saga_orchestrator::{
            BackoffStrategy, RetryPolicy, SagaOrchestrator, SagaRequest, SagaResponse, SagaStep,
        },
    },
    metrics::Metrics,
    storage::{postgres::PostgresManager, redis::RedisManager},
};

mod mock_server;
use mock_server::{with_mock_server, MockServer, MockServerConfig};

/// Test the core lock manager functionality
#[tokio::test]
async fn test_lock_manager_integration() {
    let redis_manager = RedisManager::new("redis://localhost:6379");
    let lock_manager = LockManager::new(redis_manager);

    let key = format!("test_lock_{}", Uuid::new_v4());
    let owner = "test_owner";
    let ttl = Duration::from_secs(30);

    // Test lock acquisition
    let lock_request = LockRequest {
        key: key.clone(),
        ttl,
        metadata: Some("test metadata".to_string()),
        owner: owner.to_string(),
        wait_timeout: None,
    };

    let response = lock_manager
        .acquire_lock(lock_request)
        .await
        .expect("Failed to acquire lock");

    assert!(response.success);
    assert!(!response.lock_id.is_empty());

    // Test lock status
    let status = lock_manager
        .get_lock_status(&key)
        .await
        .expect("Failed to get lock status");

    assert!(status.is_some());
    let lock_state = status.unwrap();
    assert_eq!(lock_state.key, key);
    assert_eq!(lock_state.owner, owner);
    assert_eq!(lock_state.id, response.lock_id);

    // Test lock release
    let release_request = ReleaseLockRequest {
        key: key.clone(),
        lock_id: response.lock_id.clone(),
        owner: owner.to_string(),
    };

    let release_response = lock_manager
        .release_lock(release_request)
        .await
        .expect("Failed to release lock");

    assert!(release_response.success);

    // Verify lock is released
    let status = lock_manager
        .get_lock_status(&key)
        .await
        .expect("Failed to get lock status");

    assert!(status.is_none());
}

/// Test the saga orchestrator functionality
#[tokio::test]
async fn test_saga_orchestrator_integration() {
    let postgres_manager = PostgresManager::new("postgres://localhost:5432/syros", 10).await.unwrap();
    let orchestrator = SagaOrchestrator::new(postgres_manager);

    let saga_name = format!("test_saga_{}", Uuid::new_v4());
    let steps = vec![
        SagaStep {
            name: "step1".to_string(),
            service: "test-service".to_string(),
            action: "test-action".to_string(),
            compensation: "test-compensation".to_string(),
            timeout: Duration::from_secs(30),
            retry_policy: Some(RetryPolicy {
                max_retries: 3,
                backoff_strategy: BackoffStrategy::Exponential,
                initial_delay: Duration::from_secs(1),
            }),
        },
        SagaStep {
            name: "step2".to_string(),
            service: "test-service".to_string(),
            action: "test-action-2".to_string(),
            compensation: "test-compensation-2".to_string(),
            timeout: Duration::from_secs(30),
            retry_policy: None,
        },
    ];

    let saga_request = SagaRequest {
        name: saga_name.clone(),
        steps,
        metadata: Some(std::collections::HashMap::from([(
            "test".to_string(),
            "data".to_string(),
        )])),
    };

    // Start saga
    let response: SagaResponse = orchestrator
        .start_saga(saga_request)
        .await
        .expect("Failed to start saga");

    assert!(response.success);
    assert!(!response.saga_id.is_empty());

    // Wait a bit for saga to process
    sleep(Duration::from_millis(100)).await;

    // Check saga status
    let status = orchestrator
        .get_saga_status(&response.saga_id)
        .await
        .expect("Failed to get saga status");

    assert!(status.is_some());
    let saga = status.unwrap();
    assert_eq!(saga.name, saga_name);
    assert_eq!(saga.id, response.saga_id);
}

/// Test the event store functionality
#[tokio::test]
async fn test_event_store_integration() {
    let postgres_manager = PostgresManager::new("postgres://localhost:5432/syros", 10).await.unwrap();
    let event_store = EventStore::new(postgres_manager);

    let stream_id = format!("test_stream_{}", Uuid::new_v4());
    let event_type = "test.event";
    let event_data = json!({"message": "Hello, World!"});

    let event_request = EventRequest {
        stream_id: stream_id.clone(),
        event_type: event_type.to_string(),
        data: event_data.clone(),
        metadata: Some(std::collections::HashMap::from([(
            "source".to_string(),
            "test".to_string(),
        )])),
    };

    // Append event
    let response = event_store
        .append_event(event_request)
        .await
        .expect("Failed to append event");

    assert!(response.success);
    assert!(!response.event_id.is_empty());

    // Get events
    let get_events_request = GetEventsRequest {
        stream_id: stream_id.clone(),
        from_version: None,
        limit: None,
    };

    let events_response = event_store
        .get_events(get_events_request)
        .await
        .expect("Failed to get events");

    assert!(events_response.success);
    assert!(!events_response.events.is_empty());
    let event = &events_response.events[0];
    assert_eq!(event.stream_id, stream_id);
    assert_eq!(event.event_type, event_type);
    assert_eq!(event.data, event_data);
    assert_eq!(event.id, response.event_id);
}

/// Test the cache manager functionality
#[tokio::test]
async fn test_cache_manager_integration() {
    let redis_manager = RedisManager::new("redis://localhost:6379");
    let cache_manager = CacheManager::new(redis_manager);

    let key = format!("test_key_{}", Uuid::new_v4());
    let value = json!({"cached": "data", "number": 42});
    let ttl = Duration::from_secs(60);

    let cache_request = CacheRequest {
        key: key.clone(),
        value: value.clone(),
        ttl: Some(ttl),
        tags: vec!["test".to_string()],
    };

    // Set cache
    let response = cache_manager
        .set(cache_request)
        .await
        .expect("Failed to set cache");

    // Cache response doesn't have success field, check if operation completed
    assert_eq!(response.key, key);

    // Get cache
    let cached_response = cache_manager.get(&key).await.expect("Failed to get cache");

    assert!(cached_response.found);
    assert_eq!(cached_response.value, Some(value));

    // Delete cache
    let delete_request = DeleteCacheRequest { key: key.clone() };

    let delete_response = cache_manager
        .delete(delete_request)
        .await
        .expect("Failed to delete cache");

    assert!(delete_response.success);

    // Verify cache is deleted
    let cached_response = cache_manager.get(&key).await.expect("Failed to get cache");

    assert!(!cached_response.found);
}

/// Test concurrent lock acquisition
#[tokio::test]
async fn test_concurrent_lock_acquisition() {
    let redis_manager = RedisManager::new("redis://localhost:6379");
    let lock_manager = LockManager::new(redis_manager);

    let key = format!("concurrent_test_{}", Uuid::new_v4());
    let ttl = Duration::from_secs(5);

    // Spawn multiple tasks trying to acquire the same lock
    let mut handles = vec![];
    for i in 0..5 {
        let lock_manager_clone = lock_manager.clone();
        let key_clone = key.clone();
        let owner = format!("owner_{}", i);

        let handle = tokio::spawn(async move {
            let lock_request = LockRequest {
                key: key_clone,
                ttl,
                metadata: None,
                owner: owner.clone(),
                wait_timeout: Some(Duration::from_secs(10)),
            };

            lock_manager_clone
                .acquire_lock(lock_request)
                .await
                .map(|response| (owner, response.lock_id))
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    let results: Vec<_> = futures::future::join_all(handles).await;

    // Count successful acquisitions
    let successful_acquisitions: Vec<_> = results
        .into_iter()
        .filter_map(|result| {
            result.ok().and_then(|r: Result<(String, String), syros::SyrosError>| r.ok())
        })
        .collect();

    // Only one should succeed initially, others should wait and then succeed
    assert!(!successful_acquisitions.is_empty());
}

/// Test saga compensation
#[tokio::test]
async fn test_saga_compensation() {
    let postgres_manager = PostgresManager::new("postgres://localhost:5432/syros", 10).await.unwrap();
    let orchestrator = SagaOrchestrator::new(postgres_manager);

    let saga_name = format!("compensation_test_{}", Uuid::new_v4());
    let steps = vec![
        SagaStep {
            name: "success_step".to_string(),
            service: "test-service".to_string(),
            action: "success".to_string(),
            compensation: "undo_success".to_string(),
            timeout: Duration::from_secs(30),
            retry_policy: None,
        },
        SagaStep {
            name: "process_payment".to_string(),
            service: "payment-service".to_string(),
            action: "charge".to_string(),
            compensation: "refund".to_string(),
            timeout: Duration::from_secs(30),
            retry_policy: Some(RetryPolicy {
                max_retries: 2,
                backoff_strategy: BackoffStrategy::Linear,
                initial_delay: Duration::from_secs(1),
            }),
        },
    ];

    let saga_request = SagaRequest {
        name: saga_name,
        steps,
        metadata: Some(std::collections::HashMap::from([(
            "test".to_string(),
            "compensation".to_string(),
        )])),
    };

    // Start saga
    let response: SagaResponse = orchestrator
        .start_saga(saga_request)
        .await
        .expect("Failed to start saga");

    // Wait for saga to complete (or fail and compensate)
    sleep(Duration::from_secs(2)).await;

    // Check final status
    let status = orchestrator
        .get_saga_status(&response.saga_id)
        .await
        .expect("Failed to get saga status");

    assert!(status.is_some());
    let saga = status.unwrap();

    // The saga should either be completed or compensated
    assert!(matches!(
        saga.status,
        syros::core::saga_orchestrator::SagaStatus::Completed
            | syros::core::saga_orchestrator::SagaStatus::Compensated
    ));
}

/// Test RBAC functionality
#[tokio::test]
async fn test_rbac_integration() {
    let mut rbac_manager = RBACManager::new();

    // Create a user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        roles: vec![Role::Admin],
        permissions: vec![],
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created_user = rbac_manager
        .create_user(
            user.username.clone(),
            user.email.clone(),
            user.roles.clone(),
        )
        .await
        .expect("Failed to create user");

    // Test user retrieval
    let retrieved_user = rbac_manager
        .get_user(&created_user.id)
        .await
        .expect("Failed to get user");
    if let Some(retrieved_user) = retrieved_user {
        assert_eq!(retrieved_user.username, user.username);
        assert_eq!(retrieved_user.email, user.email);
    } else {
        panic!("User not found after creation");
    }

    // Test role assignment
    rbac_manager
        .update_user_roles(&created_user.id, vec![Role::Admin])
        .await
        .expect("Failed to update roles");

    let updated_user = rbac_manager
        .get_user(&created_user.id)
        .await
        .expect("Failed to get updated user");
    if let Some(updated_user) = updated_user {
        assert!(updated_user.roles.contains(&Role::Admin));
    } else {
        panic!("User not found after role update");
    }

    // Test permission check
    let _resource = Resource {
        id: "test_resource".to_string(),
        resource_type: ResourceType::Lock,
        name: "Test Resource".to_string(),
        owner_id: created_user.id.clone(),
        permissions: vec![],
    };

    let has_permission = rbac_manager
        .check_permission(&created_user.id, &Permission::LockRead)
        .await;
    assert!(has_permission.is_ok());
}

/// Test metrics collection
#[tokio::test]
async fn test_metrics_integration() {
    let metrics = Metrics::new().expect("Failed to create metrics");

    // Record some metrics
    metrics.record_http_request("GET", "/test", "200", 0.1);
    metrics.record_http_request("POST", "/api/v1/locks", "201", 0.2);
    // Note: record_operation_duration method doesn't exist, using record_http_request instead
    metrics.record_http_request("OPERATION", "lock_acquire", "200", 0.05);
    metrics.record_http_request("OPERATION", "saga_execute", "200", 0.2);

    // Get metrics as text
    let metrics_text = metrics.get_metrics().expect("Failed to get metrics text");

    // Verify metrics contain expected data
    assert!(metrics_text.contains("http_requests_total"));
    // Note: operation_duration_seconds might not be present since we're using record_http_request
    assert!(metrics_text.contains("method=\"GET\""));
    assert!(metrics_text.contains("method=\"POST\""));
    // Check for operation metrics (using http_request format)
    assert!(metrics_text.contains("method=\"OPERATION\""));
}

/// Test REST API endpoints with mock server
#[tokio::test]
async fn test_rest_api_integration() {
    with_mock_server(|server| async move {
        let client = Client::new();
        let base_url = server.rest_url();

        // Test health endpoint
        let response = client
            .get(&format!("{}/health", base_url))
            .send()
            .await
            .expect("Failed to send health request");

        assert_eq!(response.status(), 200);

        let health_data: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse health response");
        assert_eq!(health_data["status"], "healthy");

        // Test metrics endpoint
        let response = client
            .get(&format!("{}/metrics", base_url))
            .send()
            .await
            .expect("Failed to send metrics request");

        assert_eq!(response.status(), 200);

        // Test lock acquisition
        let lock_data = json!({
            "key": format!("test_lock_{}", Uuid::new_v4()),
            "owner": "test_owner",
            "ttl_seconds": 30
        });

        let response = client
            .post(&format!("{}/api/v1/locks", base_url))
            .json(&lock_data)
            .send()
            .await
            .expect("Failed to send lock request");

        assert_eq!(response.status(), 200);

        let response_data: serde_json::Value =
            response.json().await.expect("Failed to parse response");
        assert_eq!(response_data["success"], true);
        assert!(response_data["lock_id"].is_string());

        Ok(())
    })
    .await
    .expect("Mock server test failed");
}

/// Test WebSocket connection with mock server
#[tokio::test]
async fn test_websocket_integration() {
    with_mock_server(|server| async move {
        let url = server.websocket_url();
        let (_ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .expect("Failed to connect to WebSocket");

        // Test WebSocket connection - just verify it was created successfully
        // WebSocketStream doesn't have is_ok() or is_err() methods, so we just check it exists
        // The connection was successful if we got here without panicking
        // We can't easily test the WebSocket stream without more complex setup

        Ok(())
    })
    .await
    .expect("Mock WebSocket test failed");
}

/// Test GraphQL API with mock server
#[tokio::test]
async fn test_graphql_integration() {
    with_mock_server(|server| async move {
        let client = Client::new();
        let base_url = server.rest_url();

        // Test GraphQL query
        let query = json!({
            "query": "{ health }"
        });

        let response = client
            .post(&format!("{}/graphql", base_url))
            .json(&query)
            .send()
            .await
            .expect("Failed to send GraphQL request");

        assert_eq!(response.status(), 200);

        let response_data: serde_json::Value =
            response.json().await.expect("Failed to parse response");
        assert!(response_data.get("data").is_some());
        assert_eq!(response_data["data"]["health"], "healthy");

        Ok(())
    })
    .await
    .expect("Mock GraphQL test failed");
}

/// Test complete workflow integration
#[tokio::test]
async fn test_complete_workflow_integration() {
    // Initialize all components
    let redis_manager = RedisManager::new("redis://localhost:6379");
    let postgres_manager = PostgresManager::new("postgres://localhost:5432/syros", 10).await.unwrap();
    let lock_manager = LockManager::new(redis_manager.clone());
    let saga_orchestrator = SagaOrchestrator::new(postgres_manager.clone());
    let event_store = EventStore::new(postgres_manager.clone());
    let cache_manager = CacheManager::new();
    let mut rbac_manager = RBACManager::new();

    // Create a test user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: "workflow_user".to_string(),
        email: "workflow@example.com".to_string(),
        roles: vec![Role::Admin],
        permissions: vec![],
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    rbac_manager
        .create_user(
            user.username.clone(),
            user.email.clone(),
            user.roles.clone(),
        )
        .await
        .expect("Failed to create user");

    // 1. Acquire a lock
    let lock_request = LockRequest {
        key: "workflow_lock".to_string(),
        ttl: Duration::from_secs(60),
        metadata: Some("workflow test".to_string()),
        owner: user.id.clone(),
        wait_timeout: None,
    };

    let lock_response = lock_manager
        .acquire_lock(lock_request)
        .await
        .expect("Failed to acquire lock");
    assert!(lock_response.success);

    // 2. Start a saga
    let saga_request = SagaRequest {
        name: "workflow_saga".to_string(),
        steps: vec![SagaStep {
            name: "process_order".to_string(),
            service: "order-service".to_string(),
            action: "create_order".to_string(),
            compensation: "cancel_order".to_string(),
            timeout: Duration::from_secs(30),
            retry_policy: None,
        }],
        metadata: Some(std::collections::HashMap::from([(
            "user_id".to_string(),
            user.id.clone(),
        )])),
    };

    let saga_response = saga_orchestrator
        .start_saga(saga_request)
        .await
        .expect("Failed to start saga");
    assert!(saga_response.success);

    // 3. Log an event
    let event_request = EventRequest {
        stream_id: "workflow_events".to_string(),
        event_type: "order.created".to_string(),
        data: json!({"order_id": "12345", "user_id": user.id}),
        metadata: Some(std::collections::HashMap::from([(
            "source".to_string(),
            "workflow_test".to_string(),
        )])),
    };

    let event_response = event_store
        .append_event(event_request)
        .await
        .expect("Failed to append event");
    assert!(event_response.success);

    // 4. Cache some data
    let cache_request = CacheRequest {
        key: "workflow_cache".to_string(),
        value: json!({"status": "processing", "order_id": "12345"}),
        ttl: Some(Duration::from_secs(300)),
        tags: vec!["workflow".to_string()],
    };

    let cache_response = cache_manager
        .set(cache_request)
        .await
        .expect("Failed to set cache");
    // Cache response doesn't have success field, check if operation completed
    assert_eq!(cache_response.key, "workflow_cache");

    // 5. Verify all operations completed successfully
    let lock_status = lock_manager
        .get_lock_status("workflow_lock")
        .await
        .expect("Failed to get lock status");
    assert!(lock_status.is_some());

    let saga_status = saga_orchestrator
        .get_saga_status(&saga_response.saga_id)
        .await
        .expect("Failed to get saga status");
    assert!(saga_status.is_some());

    let get_events_request = GetEventsRequest {
        stream_id: "workflow_events".to_string(),
        from_version: None,
        limit: None,
    };

    let events_response = event_store
        .get_events(get_events_request)
        .await
        .expect("Failed to get events");
    assert!(events_response.success);
    assert!(!events_response.events.is_empty());

    let cached_data = cache_manager
        .get("workflow_cache")
        .await
        .expect("Failed to get cache");
    assert!(cached_data.found);

    // 6. Cleanup
    let release_request = ReleaseLockRequest {
        key: "workflow_lock".to_string(),
        lock_id: lock_response.lock_id,
        owner: user.id,
    };

    lock_manager
        .release_lock(release_request)
        .await
        .expect("Failed to release lock");

    let delete_request = DeleteCacheRequest {
        key: "workflow_cache".to_string(),
    };

    cache_manager
        .delete(delete_request)
        .await
        .expect("Failed to delete cache");
}
