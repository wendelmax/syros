//! Benchmarks for distributed cache operations.
//!
//! This module contains performance benchmarks for the Syros distributed cache system.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use syros::core::cache_manager::CacheManager;
use syros::core::cache_manager::{CacheRequest, DeleteCacheRequest};
use tokio::runtime::Runtime;

fn bench_cache_set(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let cache_manager = CacheManager::new();

    c.bench_function("cache_set", |b| {
        b.to_async(&rt).iter(|| async {
            let request = CacheRequest {
                key: black_box("benchmark-key".to_string()),
                value: black_box(serde_json::json!({
                    "data": "benchmark-value",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
                ttl: Some(black_box(std::time::Duration::from_secs(60))),
                tags: black_box(vec!["benchmark".to_string()]),
            };
            
            let _ = cache_manager.set(request).await;
        })
    });
}

fn bench_cache_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let cache_manager = CacheManager::new();

    c.bench_function("cache_get", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = cache_manager.get(black_box("benchmark-key")).await;
        })
    });
}

fn bench_cache_delete(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let cache_manager = CacheManager::new();

    c.bench_function("cache_delete", |b| {
        b.to_async(&rt).iter(|| async {
            let request = DeleteCacheRequest {
                key: black_box("benchmark-key".to_string()),
            };
            
            let _ = cache_manager.delete(request).await;
        })
    });
}

fn bench_cache_set_and_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let cache_manager = CacheManager::new();

    c.bench_function("cache_set_and_get", |b| {
        b.to_async(&rt).iter(|| async {
            let set_request = CacheRequest {
                key: black_box("benchmark-key".to_string()),
                value: black_box(serde_json::json!({
                    "data": "benchmark-value",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
                ttl: Some(black_box(std::time::Duration::from_secs(60))),
                tags: black_box(vec!["benchmark".to_string()]),
            };
            
            let _ = cache_manager.set(set_request).await;
            let _ = cache_manager.get(black_box("benchmark-key")).await;
        })
    });
}

criterion_group!(cache_benches, bench_cache_set, bench_cache_get, bench_cache_delete, bench_cache_set_and_get);
criterion_main!(cache_benches);
