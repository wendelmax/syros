//! Error types and handling for the Syros.
//!
//! This module defines custom error types used throughout the platform
//! for consistent error handling and reporting.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SyrosError>;

#[derive(Error, Debug)]
pub enum SyrosError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Saga error: {0}")]
    SagaError(String),

    #[error("Event store error: {0}")]
    EventStoreError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Service discovery error: {0}")]
    ServiceDiscoveryError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
