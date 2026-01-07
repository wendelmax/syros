//! Benchmarks for event store operations.
//!
//! This module contains performance benchmarks for the Syros event store system.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use syros::core::event_store::EventStore;
use syros::core::event_store::{EventRequest, GetEventsRequest};
use tokio::runtime::Runtime;

fn bench_event_append(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let event_store = EventStore::new();

    c.bench_function("event_append", |b| {
        b.to_async(&rt).iter(|| async {
            let request = EventRequest {
                stream_id: black_box("benchmark-stream".to_string()),
                event_type: black_box("benchmark-event".to_string()),
                data: black_box(serde_json::json!({
                    "message": "benchmark data",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
                metadata: black_box(HashMap::new()),
            };

            let _ = event_store.append_event(request).await;
        })
    });
}

fn bench_event_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let event_store = EventStore::new();

    c.bench_function("event_get", |b| {
        b.to_async(&rt).iter(|| async {
            let request = GetEventsRequest {
                stream_id: black_box("benchmark-stream".to_string()),
                from_version: black_box(Some(1)),
                limit: black_box(Some(100)),
            };

            let _ = event_store.get_events(request).await;
        })
    });
}

fn bench_event_append_and_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let event_store = EventStore::new();

    c.bench_function("event_append_and_get", |b| {
        b.to_async(&rt).iter(|| async {
            let append_request = EventRequest {
                stream_id: black_box("benchmark-stream".to_string()),
                event_type: black_box("benchmark-event".to_string()),
                data: black_box(serde_json::json!({
                    "message": "benchmark data",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
                metadata: black_box(HashMap::new()),
            };

            let _ = event_store.append_event(append_request).await;

            let get_request = GetEventsRequest {
                stream_id: black_box("benchmark-stream".to_string()),
                from_version: black_box(Some(1)),
                limit: black_box(Some(100)),
            };

            let _ = event_store.get_events(get_request).await;
        })
    });
}

criterion_group!(
    event_benches,
    bench_event_append,
    bench_event_get,
    bench_event_append_and_get
);
criterion_main!(event_benches);
