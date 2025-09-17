//! Service discovery implementation.
//!
//! This module provides service discovery functionality for registering
//! and discovering services in a distributed system.

use crate::{Result, SyrosError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::interval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
    pub health: ServiceHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceHealth {
    Passing,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
    pub check: Option<ServiceCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCheck {
    pub http: Option<String>,
    pub tcp: Option<String>,
    pub interval: String,
    pub timeout: String,
}

pub struct ServiceDiscovery {
    consul_url: String,
    registered_services: HashMap<String, ServiceRegistration>,
}

impl ServiceDiscovery {
    pub fn new(consul_url: &str) -> Result<Self> {
        Ok(Self {
            consul_url: consul_url.to_string(),
            registered_services: HashMap::new(),
        })
    }

    pub async fn register_service(&mut self, service: ServiceRegistration) -> Result<()> {
        // Por enquanto, apenas armazenamos localmente
        // Em uma implementação completa, faríamos uma chamada HTTP para o Consul
        let service_name = service.name.clone();
        let service_id = service.id.clone();
        self.registered_services.insert(service_id.clone(), service);

        tracing::info!(
            "Serviço registrado localmente: {} ({})",
            service_name,
            service_id
        );
        Ok(())
    }

    pub async fn deregister_service(&mut self, service_id: &str) -> Result<()> {
        self.registered_services.remove(service_id);
        tracing::info!("Serviço desregistrado: {}", service_id);
        Ok(())
    }

    pub async fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInfo>> {
        // Por enquanto, retornamos apenas os serviços registrados localmente
        let mut service_infos = Vec::new();

        for (_, service) in &self.registered_services {
            if service.name == service_name {
                service_infos.push(ServiceInfo {
                    id: service.id.clone(),
                    name: service.name.clone(),
                    address: service.address.clone(),
                    port: service.port,
                    tags: service.tags.clone(),
                    meta: service.meta.clone(),
                    health: ServiceHealth::Passing, // Assumimos que está saudável por enquanto
                });
            }
        }

        Ok(service_infos)
    }

    pub async fn get_healthy_services(&self, service_name: &str) -> Result<Vec<ServiceInfo>> {
        self.discover_services(service_name).await
    }

    pub async fn get_service_health(
        &self,
        _service_name: &str,
        _service_id: &str,
    ) -> Result<ServiceHealth> {
        // Por enquanto, sempre retornamos Passing
        // Em uma implementação completa, faríamos uma verificação real
        Ok(ServiceHealth::Passing)
    }

    pub async fn start_health_checker(
        &self,
        service_id: &str,
        check_url: &str,
        interval_secs: u64,
    ) -> Result<()> {
        let service_id = service_id.to_string();
        let check_url = check_url.to_string();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs));

            loop {
                interval.tick().await;

                if let Err(e) = Self::perform_health_check(&check_url).await {
                    tracing::warn!("Health check failed for service {}: {}", service_id, e);
                }
            }
        });

        Ok(())
    }

    async fn perform_health_check(check_url: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .get(check_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| {
                SyrosError::ServiceDiscoveryError(format!("Health check request failed: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(SyrosError::ServiceDiscoveryError(format!(
                "Health check failed with status: {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn list_all_services(&self) -> Result<Vec<String>> {
        let mut service_names = Vec::new();
        for (_, service) in &self.registered_services {
            if !service_names.contains(&service.name) {
                service_names.push(service.name.clone());
            }
        }
        Ok(service_names)
    }

    pub async fn get_service_instances(&self, service_name: &str) -> Result<Vec<ServiceInfo>> {
        self.discover_services(service_name).await
    }

    pub fn get_registered_services(&self) -> &HashMap<String, ServiceRegistration> {
        &self.registered_services
    }
}

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self::new("http://localhost:8500").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_discovery_creation() {
        let discovery = ServiceDiscovery::new("http://localhost:8500");
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_service_registration() {
        let mut discovery = ServiceDiscovery::new("http://localhost:8500").unwrap();

        let service = ServiceRegistration {
            id: "test-service-1".to_string(),
            name: "test-service".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            tags: vec!["test".to_string()],
            meta: HashMap::new(),
            check: Some(ServiceCheck {
                http: Some("http://127.0.0.1:8080/health".to_string()),
                tcp: None,
                interval: "10s".to_string(),
                timeout: "5s".to_string(),
            }),
        };

        let result = discovery.register_service(service).await;
        assert!(result.is_ok());
    }
}
