use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub expires_in_days: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub is_active: bool,
}

#[derive(Clone)]
pub struct ApiKeyManager {
    keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    key_to_id: Arc<RwLock<HashMap<String, String>>>, // Maps API key to ID
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            key_to_id: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_api_key(&self, request: CreateApiKeyRequest) -> Result<ApiKeyResponse> {
        let id = Uuid::new_v4().to_string();
        let key = format!("sk_{}", Uuid::new_v4().to_string().replace('-', ""));
        let now = Utc::now();

        let expires_at = request
            .expires_in_days
            .map(|days| now + chrono::Duration::days(days as i64));

        let api_key = ApiKey {
            id: id.clone(),
            key: key.clone(),
            name: request.name.clone(),
            description: request.description.clone(),
            permissions: request.permissions.clone(),
            created_at: now,
            expires_at,
            is_active: true,
            last_used_at: None,
            usage_count: 0,
        };

        // Store the API key
        {
            let mut keys = self.keys.write().await;
            let mut key_to_id = self.key_to_id.write().await;
            keys.insert(id.clone(), api_key.clone());
            key_to_id.insert(key.clone(), id.clone());
        }

        Ok(ApiKeyResponse {
            id: api_key.id,
            key: api_key.key,
            name: api_key.name,
            description: api_key.description,
            permissions: api_key.permissions,
            created_at: api_key.created_at.to_rfc3339(),
            expires_at: api_key.expires_at.map(|dt| dt.to_rfc3339()),
            is_active: api_key.is_active,
        })
    }

    pub async fn validate_api_key(&self, key: &str) -> Result<Option<ApiKey>> {
        let key_to_id = self.key_to_id.read().await;
        let keys = self.keys.read().await;

        if let Some(id) = key_to_id.get(key) {
            if let Some(api_key) = keys.get(id) {
                // Check if key is active and not expired
                if api_key.is_active {
                    if let Some(expires_at) = api_key.expires_at {
                        if Utc::now() > expires_at {
                            return Ok(None); // Expired
                        }
                    }

                    // Update usage statistics
                    let mut keys = self.keys.write().await;
                    if let Some(api_key) = keys.get_mut(id) {
                        api_key.last_used_at = Some(Utc::now());
                        api_key.usage_count += 1;
                    }

                    return Ok(Some(api_key.clone()));
                }
            }
        }

        Ok(None)
    }

    pub async fn list_api_keys(&self) -> Result<Vec<ApiKeyResponse>> {
        let keys = self.keys.read().await;
        let mut result = Vec::new();

        for api_key in keys.values() {
            result.push(ApiKeyResponse {
                id: api_key.id.clone(),
                key: format!(
                    "{}...{}",
                    &api_key.key[..8],
                    &api_key.key[api_key.key.len() - 4..]
                ), // Masked key
                name: api_key.name.clone(),
                description: api_key.description.clone(),
                permissions: api_key.permissions.clone(),
                created_at: api_key.created_at.to_rfc3339(),
                expires_at: api_key.expires_at.map(|dt| dt.to_rfc3339()),
                is_active: api_key.is_active,
            });
        }

        Ok(result)
    }

    pub async fn revoke_api_key(&self, id: &str) -> Result<bool> {
        let mut keys = self.keys.write().await;
        if let Some(api_key) = keys.get_mut(id) {
            api_key.is_active = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_api_key_stats(&self) -> Result<ApiKeyStats> {
        let keys = self.keys.read().await;
        let mut total_keys = 0;
        let mut active_keys = 0;
        let mut expired_keys = 0;
        let mut total_usage = 0;

        let now = Utc::now();

        for api_key in keys.values() {
            total_keys += 1;
            if api_key.is_active {
                active_keys += 1;

                if let Some(expires_at) = api_key.expires_at {
                    if now > expires_at {
                        expired_keys += 1;
                    }
                }
            }
            total_usage += api_key.usage_count;
        }

        Ok(ApiKeyStats {
            total_keys,
            active_keys,
            expired_keys,
            total_usage,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyStats {
    pub total_keys: u64,
    pub active_keys: u64,
    pub expired_keys: u64,
    pub total_usage: u64,
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}
