//! Configuration management for the Syros.
//!
//! This module handles loading and managing configuration settings
//! from TOML files and environment variables.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
    pub service_discovery: ServiceDiscoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    pub port: u16,
    pub grpc_port: u16,
    pub websocket_port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageConfig {
    pub redis: RedisConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub api_key_encryption_key: String,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub consul_url: String,
    pub service_name: String,
    pub service_id: String,
    pub health_check_interval: u64,
    pub tags: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, crate::errors::SyrosError> {
        let config_file_path =
            std::env::var("CONFIG_FILE").unwrap_or_else(|_| "config/default.toml".to_string());
        let config_str = std::fs::read_to_string(&config_file_path).map_err(|e| {
            crate::errors::SyrosError::ConfigError(format!(
                "Failed to read config file {}: {}",
                config_file_path, e
            ))
        })?;

        let config: Config = toml::from_str(&config_str).map_err(|e| {
            crate::errors::SyrosError::ConfigError(format!(
                "Failed to parse config file {}: {}",
                config_file_path, e
            ))
        })?;

        Ok(config)
    }
}
