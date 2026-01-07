//! GraphQL queries for the Syros API.
//!
//! This module defines all GraphQL query operations for retrieving data
//! from the Syros distributed coordination service.

use crate::api::graphql::types::*;
use crate::api::rest::ApiState;
use async_graphql::{Context, Object, Result};

/// Root query type for GraphQL operations.
///
/// This struct contains all the query resolvers for the GraphQL API.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn lock_status(&self, ctx: &Context<'_>, key: String) -> Result<Lock> {
        let _state = ctx.data::<ApiState>()?;
        Ok(Lock {
            key: key.clone(),
            owner: "system".to_string(),
            acquired_at: chrono::Utc::now(),
            expires_at: None,
            status: LockStatus::Unlocked,
        })
    }

    async fn locks(&self, ctx: &Context<'_>) -> Result<Vec<Lock>> {
        Ok(vec![])
    }

    async fn saga(&self, ctx: &Context<'_>, id: String) -> Result<Option<Saga>> {
        Ok(Some(Saga {
            id: id.clone(),
            name: "test-saga".to_string(),
            status: SagaStatus::Pending,
            steps: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }))
    }

    async fn sagas(&self, ctx: &Context<'_>) -> Result<Vec<Saga>> {
        Ok(vec![])
    }

    async fn events(&self, ctx: &Context<'_>, stream_id: String) -> Result<Vec<Event>> {
        Ok(vec![])
    }

    async fn event(&self, ctx: &Context<'_>, id: String) -> Result<Option<Event>> {
        Ok(Some(Event {
            id: id.clone(),
            stream_id: "test-stream".to_string(),
            event_type: "test-event".to_string(),
            data: "{}".to_string(),
            metadata: "{}".to_string(),
            version: 1,
            created_at: chrono::Utc::now(),
        }))
    }

    async fn cache_entry(&self, ctx: &Context<'_>, key: String) -> Result<Option<CacheEntry>> {
        Ok(Some(CacheEntry {
            key: key.clone(),
            value: "cached-value".to_string(),
            ttl: Some(3600),
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        }))
    }

    async fn user(&self, ctx: &Context<'_>, id: String) -> Result<Option<User>> {
        let state = ctx.data::<ApiState>()?;
        let rbac = state.rbac_manager.lock().await;

        match rbac.get_user(&id).await {
            Ok(Some(user)) => Ok(Some(User {
                id: user.id.clone(),
                username: user.username.clone(),
                email: user.email.clone(),
                roles: user.roles.iter().map(|r| format!("{:?}", r)).collect(),
                is_active: user.is_active,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })),
            Ok(None) => Ok(None),
            Err(_) => Ok(None),
        }
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let state = ctx.data::<ApiState>()?;
        let rbac = state.rbac_manager.lock().await;

        match rbac.get_all_users().await {
            Ok(users) => Ok(users
                .into_iter()
                .map(|user| User {
                    id: user.id.clone(),
                    username: user.username.clone(),
                    email: user.email.clone(),
                    roles: user.roles.iter().map(|r| format!("{:?}", r)).collect(),
                    is_active: user.is_active,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
                .collect()),
            Err(_) => Ok(vec![]),
        }
    }

    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<Role>> {
        let state = ctx.data::<ApiState>()?;
        let rbac = state.rbac_manager.lock().await;

        match rbac.get_all_roles().await {
            Ok(roles) => Ok(roles
                .into_iter()
                .map(|role| Role {
                    name: format!("{:?}", role.name),
                    description: role.description.clone(),
                    permissions: role
                        .permissions
                        .iter()
                        .map(|p| format!("{:?}", p))
                        .collect(),
                    is_system: role.is_system,
                })
                .collect()),
            Err(e) => Err(async_graphql::Error::new(format!("Failed to get all roles: {}", e))),
        }
    }

    async fn check_permission(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        permission: String,
    ) -> Result<PermissionCheckResponse> {
        let state = ctx.data::<ApiState>()?;
        let rbac = state.rbac_manager.lock().await;

        let perm = match permission.as_str() {
            "LockCreate" => crate::auth::Permission::LockCreate,
            "LockRead" => crate::auth::Permission::LockRead,
            "LockUpdate" => crate::auth::Permission::LockUpdate,
            "LockDelete" => crate::auth::Permission::LockDelete,
            "LockAcquire" => crate::auth::Permission::LockAcquire,
            "LockRelease" => crate::auth::Permission::LockRelease,
            "SagaCreate" => crate::auth::Permission::SagaCreate,
            "SagaRead" => crate::auth::Permission::SagaRead,
            "SagaUpdate" => crate::auth::Permission::SagaUpdate,
            "SagaDelete" => crate::auth::Permission::SagaDelete,
            "SagaExecute" => crate::auth::Permission::SagaExecute,
            "SagaCompensate" => crate::auth::Permission::SagaCompensate,
            "EventCreate" => crate::auth::Permission::EventCreate,
            "EventRead" => crate::auth::Permission::EventRead,
            "EventUpdate" => crate::auth::Permission::EventUpdate,
            "EventDelete" => crate::auth::Permission::EventDelete,
            "EventQuery" => crate::auth::Permission::EventQuery,
            "CacheCreate" => crate::auth::Permission::CacheCreate,
            "CacheRead" => crate::auth::Permission::CacheRead,
            "CacheUpdate" => crate::auth::Permission::CacheUpdate,
            "CacheDelete" => crate::auth::Permission::CacheDelete,
            "CacheClear" => crate::auth::Permission::CacheClear,
            "AdminUsers" => crate::auth::Permission::AdminUsers,
            "AdminRoles" => crate::auth::Permission::AdminRoles,
            "AdminPermissions" => crate::auth::Permission::AdminPermissions,
            "AdminSystem" => crate::auth::Permission::AdminSystem,
            "ApiRest" => crate::auth::Permission::ApiRest,
            "ApiGrpc" => crate::auth::Permission::ApiGrpc,
            "ApiWebSocket" => crate::auth::Permission::ApiWebSocket,
            "ApiGraphQL" => crate::auth::Permission::ApiGraphQL,
            _ => {
                return Ok(PermissionCheckResponse {
                    has_permission: false,
                    user_id: user_id.clone(),
                    permission: permission.clone(),
                    resource_id: None,
                })
            }
        };

        match rbac.check_permission(&user_id, &perm).await {
            Ok(has_permission) => Ok(PermissionCheckResponse {
                has_permission,
                user_id: user_id.clone(),
                permission: permission.clone(),
                resource_id: None,
            }),
            Err(_) => Ok(PermissionCheckResponse {
                has_permission: false,
                user_id: user_id.clone(),
                permission: permission.clone(),
                resource_id: None,
            }),
        }
    }

    async fn health(&self, ctx: &Context<'_>) -> Result<String> {
        Ok("OK".to_string())
    }

    async fn version(&self, ctx: &Context<'_>) -> Result<String> {
        Ok("1.0.0".to_string())
    }
}
