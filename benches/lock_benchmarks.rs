//! Benchmarks for distributed lock operations.
//!
//! This module contains performance benchmarks for the Syros distributed lock system.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use syros::core::lock_manager::LockManager;
use syros::core::lock_manager::{AcquireLockRequest, ReleaseLockRequest};
use std::time::Duration;
use tokio::runtime::Runtime;

fn bench_lock_acquire(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let lock_manager = LockManager::new();

    c.bench_function("lock_acquire", |b| {
        b.to_async(&rt).iter(|| async {
            let request = AcquireLockRequest {
                key: black_box("benchmark-key".to_string()),
                owner: black_box("benchmark-owner".to_string()),
                ttl: Some(black_box(60)),
                metadata: Some(black_box("benchmark-metadata".to_string())),
                wait_timeout: Some(Duration::from_secs(1)),
            };
            
            let _ = lock_manager.acquire_lock(request).await;
        })
    });
}

fn bench_lock_release(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let lock_manager = LockManager::new();

    c.bench_function("lock_release", |b| {
        b.to_async(&rt).iter(|| async {
            let request = ReleaseLockRequest {
                key: black_box("benchmark-key".to_string()),
                lock_id: black_box("benchmark-lock-id".to_string()),
                owner: black_box("benchmark-owner".to_string()),
            };
            
            let _ = lock_manager.release_lock(request).await;
        })
    });
}

fn bench_lock_acquire_and_release(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let lock_manager = LockManager::new();

    c.bench_function("lock_acquire_and_release", |b| {
        b.to_async(&rt).iter(|| async {
            let acquire_request = AcquireLockRequest {
                key: black_box("benchmark-key".to_string()),
                owner: black_box("benchmark-owner".to_string()),
                ttl: Some(black_box(60)),
                metadata: Some(black_box("benchmark-metadata".to_string())),
                wait_timeout: Some(Duration::from_secs(1)),
            };
            
            if let Ok(acquire_response) = lock_manager.acquire_lock(acquire_request).await {
                let release_request = ReleaseLockRequest {
                    key: black_box("benchmark-key".to_string()),
                    lock_id: black_box(acquire_response.lock_id),
                    owner: black_box("benchmark-owner".to_string()),
                };
                
                let _ = lock_manager.release_lock(release_request).await;
            }
        })
    });
}

criterion_group!(lock_benches, bench_lock_acquire, bench_lock_release, bench_lock_acquire_and_release);
criterion_main!(lock_benches);
