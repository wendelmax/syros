//! Syros - Distributed Coordination Service
//!
//! A high-performance, distributed coordination platform built in Rust that provides
//! essential distributed system patterns including distributed locks, saga orchestration,
//! event sourcing, and caching for microservices architectures.
//!
//! # Features
//!
//! - **Distributed Locks**: Prevent race conditions in distributed systems
//! - **Saga Orchestration**: Manage distributed transactions with compensation
//! - **Event Sourcing**: Store and replay application state changes
//! - **Caching**: High-performance distributed caching
//! - **Service Discovery**: Automatic service registration and discovery
//! - **Authentication & Authorization**: JWT and RBAC support
//! - **Multiple APIs**: REST, gRPC, WebSocket, and GraphQL
//! - **Observability**: Metrics, logging, and tracing
//!
//! # Quick Start
//!
//! ```rust
//! use syros_platform::{server, cli};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let cli = cli::parse_args();
//!     // Start the server with your configuration
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod auth;
pub mod cli;
pub mod config;
pub mod core;
pub mod errors;
pub mod generated;
pub mod metrics;
pub mod server;
pub mod storage;

pub use errors::{Result, SyrosError};
