//! Metrics collection and monitoring.
//!
//! This module provides metrics collection using Prometheus for monitoring
//! the Syros's performance and health.

use prometheus::{
    Counter, CounterVec, Encoder, Gauge, Histogram, HistogramOpts, HistogramVec, Opts, Registry,
    TextEncoder,
};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct Metrics {
    pub http_requests_total: CounterVec,
    pub grpc_requests_total: CounterVec,
    pub websocket_connections_total: Counter,

    pub locks_acquired_total: Counter,
    pub locks_released_total: Counter,
    pub sagas_started_total: Counter,
    pub sagas_completed_total: Counter,
    pub sagas_failed_total: Counter,
    pub events_appended_total: Counter,
    pub cache_hits_total: Counter,
    pub cache_misses_total: Counter,

    pub http_request_duration: HistogramVec,
    pub grpc_request_duration: HistogramVec,
    pub lock_operation_duration: HistogramVec,
    pub saga_execution_duration: Histogram,
    pub cache_operation_duration: HistogramVec,

    pub active_locks: Gauge,
    pub active_sagas: Gauge,
    pub cache_size: Gauge,
    pub websocket_connections: Gauge,

    pub registry: Arc<Registry>,
}

impl Metrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Arc::new(Registry::new());

        let http_requests_total = CounterVec::new(
            Opts::new("http_requests_total", "Total HTTP requests"),
            &["method", "endpoint", "status"],
        )?;

        let grpc_requests_total = CounterVec::new(
            Opts::new("grpc_requests_total", "Total gRPC requests"),
            &["service", "method", "status"],
        )?;

        let websocket_connections_total =
            Counter::new("websocket_connections_total", "Total WebSocket connections")?;

        let locks_acquired_total = Counter::new("locks_acquired_total", "Total locks acquired")?;

        let locks_released_total = Counter::new("locks_released_total", "Total locks released")?;

        let sagas_started_total = Counter::new("sagas_started_total", "Total sagas started")?;

        let sagas_completed_total = Counter::new("sagas_completed_total", "Total sagas completed")?;

        let sagas_failed_total = Counter::new("sagas_failed_total", "Total sagas failed")?;

        let events_appended_total = Counter::new("events_appended_total", "Total events appended")?;

        let cache_hits_total = Counter::new("cache_hits_total", "Total cache hits")?;

        let cache_misses_total = Counter::new("cache_misses_total", "Total cache misses")?;
        let http_request_duration = HistogramVec::new(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request duration").buckets(
                vec![
                    0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                ],
            ),
            &["method", "endpoint"],
        )?;

        let grpc_request_duration = HistogramVec::new(
            HistogramOpts::new("grpc_request_duration_seconds", "gRPC request duration").buckets(
                vec![
                    0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                ],
            ),
            &["service", "method"],
        )?;

        let lock_operation_duration = HistogramVec::new(
            HistogramOpts::new("lock_operation_duration_seconds", "Lock operation duration")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]),
            &["operation"],
        )?;

        let saga_execution_duration = Histogram::with_opts(
            HistogramOpts::new("saga_execution_duration_seconds", "Saga execution duration")
                .buckets(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0]),
        )?;

        let cache_operation_duration = HistogramVec::new(
            HistogramOpts::new(
                "cache_operation_duration_seconds",
                "Cache operation duration",
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5]),
            &["operation"],
        )?;

        let active_locks = Gauge::new("active_locks", "Number of active locks")?;
        let active_sagas = Gauge::new("active_sagas", "Number of active sagas")?;
        let cache_size = Gauge::new("cache_size", "Number of items in cache")?;
        let websocket_connections = Gauge::new(
            "websocket_connections",
            "Number of active WebSocket connections",
        )?;
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(grpc_requests_total.clone()))?;
        registry.register(Box::new(websocket_connections_total.clone()))?;
        registry.register(Box::new(locks_acquired_total.clone()))?;
        registry.register(Box::new(locks_released_total.clone()))?;
        registry.register(Box::new(sagas_started_total.clone()))?;
        registry.register(Box::new(sagas_completed_total.clone()))?;
        registry.register(Box::new(sagas_failed_total.clone()))?;
        registry.register(Box::new(events_appended_total.clone()))?;
        registry.register(Box::new(cache_hits_total.clone()))?;
        registry.register(Box::new(cache_misses_total.clone()))?;
        registry.register(Box::new(http_request_duration.clone()))?;
        registry.register(Box::new(grpc_request_duration.clone()))?;
        registry.register(Box::new(lock_operation_duration.clone()))?;
        registry.register(Box::new(saga_execution_duration.clone()))?;
        registry.register(Box::new(cache_operation_duration.clone()))?;
        registry.register(Box::new(active_locks.clone()))?;
        registry.register(Box::new(active_sagas.clone()))?;
        registry.register(Box::new(cache_size.clone()))?;
        registry.register(Box::new(websocket_connections.clone()))?;

        Ok(Metrics {
            http_requests_total,
            grpc_requests_total,
            websocket_connections_total,
            locks_acquired_total,
            locks_released_total,
            sagas_started_total,
            sagas_completed_total,
            sagas_failed_total,
            events_appended_total,
            cache_hits_total,
            cache_misses_total,
            http_request_duration,
            grpc_request_duration,
            lock_operation_duration,
            saga_execution_duration,
            cache_operation_duration,
            active_locks,
            active_sagas,
            cache_size,
            websocket_connections,
            registry,
        })
    }

    pub fn record_http_request(&self, method: &str, endpoint: &str, status: &str, duration: f64) {
        self.http_requests_total
            .with_label_values(&[method, endpoint, status])
            .inc();
        self.http_request_duration
            .with_label_values(&[method, endpoint])
            .observe(duration);
    }

    pub fn record_grpc_request(&self, service: &str, method: &str, status: &str, duration: f64) {
        self.grpc_requests_total
            .with_label_values(&[service, method, status])
            .inc();
        self.grpc_request_duration
            .with_label_values(&[service, method])
            .observe(duration);
    }

    pub fn record_lock_operation(&self, operation: &str, duration: f64) {
        self.lock_operation_duration
            .with_label_values(&[operation])
            .observe(duration);
    }

    pub fn record_cache_operation(&self, operation: &str, duration: f64) {
        self.cache_operation_duration
            .with_label_values(&[operation])
            .observe(duration);
    }

    pub fn record_saga_execution(&self, duration: f64) {
        self.saga_execution_duration.observe(duration);
    }

    pub fn increment_locks_acquired(&self) {
        self.locks_acquired_total.inc();
        self.active_locks.inc();
    }

    pub fn increment_locks_released(&self) {
        self.locks_released_total.inc();
        self.active_locks.dec();
    }

    pub fn increment_sagas_started(&self) {
        self.sagas_started_total.inc();
        self.active_sagas.inc();
    }

    pub fn increment_sagas_completed(&self) {
        self.sagas_completed_total.inc();
        self.active_sagas.dec();
    }

    pub fn increment_sagas_failed(&self) {
        self.sagas_failed_total.inc();
        self.active_sagas.dec();
    }

    pub fn increment_events_appended(&self) {
        self.events_appended_total.inc();
    }

    pub fn increment_cache_hits(&self) {
        self.cache_hits_total.inc();
    }

    pub fn increment_cache_misses(&self) {
        self.cache_misses_total.inc();
    }

    pub fn increment_websocket_connections(&self) {
        self.websocket_connections_total.inc();
        self.websocket_connections.inc();
    }

    pub fn decrement_websocket_connections(&self) {
        self.websocket_connections.dec();
    }

    pub fn set_cache_size(&self, size: f64) {
        self.cache_size.set(size);
    }

    pub fn get_metrics(&self) -> Result<String, prometheus::Error> {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}

pub struct MetricsTimer {
    start: Instant,
    metrics: Arc<Metrics>,
    _operation: String,
    operation_type: OperationType,
}

#[derive(Clone)]
pub enum OperationType {
    Http { method: String, endpoint: String },
    Grpc { service: String, method: String },
    Lock { operation: String },
    Cache { operation: String },
    Saga,
}

impl MetricsTimer {
    pub fn new(metrics: Arc<Metrics>, operation: String, operation_type: OperationType) -> Self {
        Self {
            start: Instant::now(),
            metrics,
            _operation: operation,
            operation_type,
        }
    }

    pub fn finish(self, status: &str) {
        let duration = self.start.elapsed().as_secs_f64();

        match self.operation_type {
            OperationType::Http { method, endpoint } => {
                self.metrics
                    .record_http_request(&method, &endpoint, status, duration);
            }
            OperationType::Grpc { service, method } => {
                self.metrics
                    .record_grpc_request(&service, &method, status, duration);
            }
            OperationType::Lock { operation } => {
                self.metrics.record_lock_operation(&operation, duration);
            }
            OperationType::Cache { operation } => {
                self.metrics.record_cache_operation(&operation, duration);
            }
            OperationType::Saga => {
                self.metrics.record_saga_execution(duration);
            }
        }
    }
}
