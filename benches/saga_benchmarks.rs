//! Benchmarks for saga orchestration operations.
//!
//! This module contains performance benchmarks for the Syros saga orchestration system.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use syros::core::saga_orchestrator::{
    BackoffStrategy, RetryPolicy, SagaOrchestrator, SagaRequest, SagaStep,
};
use tokio::runtime::Runtime;

fn bench_saga_start(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let saga_orchestrator = SagaOrchestrator::new();

    c.bench_function("saga_start", |b| {
        b.to_async(&rt).iter(|| async {
            let steps = vec![
                SagaStep {
                    name: black_box("step1".to_string()),
                    service: black_box("service1".to_string()),
                    action: black_box("action1".to_string()),
                    compensation: black_box("compensation1".to_string()),
                    timeout: black_box(Duration::from_secs(30)),
                    retry_policy: Some(RetryPolicy {
                        max_retries: black_box(3),
                        backoff_strategy: black_box(BackoffStrategy::Exponential),
                        initial_delay: black_box(Duration::from_millis(100)),
                    }),
                },
                SagaStep {
                    name: black_box("step2".to_string()),
                    service: black_box("service2".to_string()),
                    action: black_box("action2".to_string()),
                    compensation: black_box("compensation2".to_string()),
                    timeout: black_box(Duration::from_secs(30)),
                    retry_policy: Some(RetryPolicy {
                        max_retries: black_box(3),
                        backoff_strategy: black_box(BackoffStrategy::Exponential),
                        initial_delay: black_box(Duration::from_millis(100)),
                    }),
                },
            ];

            let request = SagaRequest {
                name: black_box("benchmark-saga".to_string()),
                steps: black_box(steps),
                metadata: black_box(std::collections::HashMap::new()),
            };

            let _ = saga_orchestrator.start_saga(request).await;
        })
    });
}

fn bench_saga_get_status(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let saga_orchestrator = SagaOrchestrator::new();

    c.bench_function("saga_get_status", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = saga_orchestrator
                .get_saga_status(black_box("benchmark-saga-id"))
                .await;
        })
    });
}

fn bench_saga_compensate(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let saga_orchestrator = SagaOrchestrator::new();

    c.bench_function("saga_compensate", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = saga_orchestrator
                .compensate_saga(black_box("benchmark-saga-id"))
                .await;
        })
    });
}

criterion_group!(
    saga_benches,
    bench_saga_start,
    bench_saga_get_status,
    bench_saga_compensate
);
criterion_main!(saga_benches);
