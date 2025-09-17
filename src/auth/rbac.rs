use crate::{Result, SyrosError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    // Lock permissions
    LockCreate,
    LockRead,
    LockUpdate,
    LockDelete,
    LockAcquire,
    LockRelease,

    // Saga permissions
    SagaCreate,
    SagaRead,
    SagaUpdate,
    SagaDelete,
    SagaExecute,
    SagaCompensate,

    // Event permissions
    EventCreate,
    EventRead,
    EventUpdate,
    EventDelete,
    EventQuery,

    // Cache permissions
    CacheCreate,
    CacheRead,
    CacheUpdate,
    CacheDelete,
    CacheClear,

    // Admin permissions
    AdminUsers,
    AdminRoles,
    AdminPermissions,
    AdminSystem,

    // API permissions
    ApiRest,
    ApiGrpc,
    ApiWebSocket,
    ApiGraphQL,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Manager,
    Developer,
    Viewer,
    Custom(String),
}

impl Role {
    pub fn get_permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::LockCreate,
                Permission::LockRead,
                Permission::LockUpdate,
                Permission::LockDelete,
                Permission::LockAcquire,
                Permission::LockRelease,
                Permission::SagaCreate,
                Permission::SagaRead,
                Permission::SagaUpdate,
                Permission::SagaDelete,
                Permission::SagaExecute,
                Permission::SagaCompensate,
                Permission::EventCreate,
                Permission::EventRead,
                Permission::EventUpdate,
                Permission::EventDelete,
                Permission::EventQuery,
                Permission::CacheCreate,
                Permission::CacheRead,
                Permission::CacheUpdate,
                Permission::CacheDelete,
                Permission::CacheClear,
                Permission::AdminUsers,
                Permission::AdminRoles,
                Permission::AdminPermissions,
                Permission::AdminSystem,
                Permission::ApiRest,
                Permission::ApiGrpc,
                Permission::ApiWebSocket,
                Permission::ApiGraphQL,
            ],
            Role::Manager => vec![
                Permission::LockCreate,
                Permission::LockRead,
                Permission::LockUpdate,
                Permission::LockDelete,
                Permission::LockAcquire,
                Permission::LockRelease,
                Permission::SagaCreate,
                Permission::SagaRead,
                Permission::SagaUpdate,
                Permission::SagaDelete,
                Permission::SagaExecute,
                Permission::SagaCompensate,
                Permission::EventCreate,
                Permission::EventRead,
                Permission::EventUpdate,
                Permission::EventDelete,
                Permission::EventQuery,
                Permission::CacheCreate,
                Permission::CacheRead,
                Permission::CacheUpdate,
                Permission::CacheDelete,
                Permission::CacheClear,
                Permission::ApiRest,
                Permission::ApiGrpc,
                Permission::ApiWebSocket,
                Permission::ApiGraphQL,
            ],
            Role::Developer => vec![
                Permission::LockCreate,
                Permission::LockRead,
                Permission::LockAcquire,
                Permission::LockRelease,
                Permission::SagaCreate,
                Permission::SagaRead,
                Permission::SagaExecute,
                Permission::SagaCompensate,
                Permission::EventCreate,
                Permission::EventRead,
                Permission::EventQuery,
                Permission::CacheCreate,
                Permission::CacheRead,
                Permission::CacheUpdate,
                Permission::CacheDelete,
                Permission::ApiRest,
                Permission::ApiGrpc,
                Permission::ApiWebSocket,
            ],
            Role::Viewer => vec![
                Permission::LockRead,
                Permission::SagaRead,
                Permission::EventRead,
                Permission::EventQuery,
                Permission::CacheRead,
                Permission::ApiRest,
            ],
            Role::Custom(_) => vec![], // Custom roles have no default permissions
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    pub name: Role,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub is_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub resource_type: ResourceType,
    pub owner_id: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    Lock,
    Saga,
    Event,
    Cache,
    User,
    Role,
    System,
}

pub struct RBACManager {
    users: HashMap<String, User>,
    roles: HashMap<Role, RoleDefinition>,
    resources: HashMap<String, Resource>,
}

impl RBACManager {
    pub fn new() -> Self {
        let mut rbac = Self {
            users: HashMap::new(),
            roles: HashMap::new(),
            resources: HashMap::new(),
        };

        // Initialize default roles
        rbac.initialize_default_roles();
        rbac
    }

    fn initialize_default_roles(&mut self) {
        let default_roles = vec![
            RoleDefinition {
                name: Role::Admin,
                description: "Full system access".to_string(),
                permissions: Role::Admin.get_permissions(),
                is_system: true,
            },
            RoleDefinition {
                name: Role::Manager,
                description: "Management access to all resources".to_string(),
                permissions: Role::Manager.get_permissions(),
                is_system: true,
            },
            RoleDefinition {
                name: Role::Developer,
                description: "Developer access to create and use resources".to_string(),
                permissions: Role::Developer.get_permissions(),
                is_system: true,
            },
            RoleDefinition {
                name: Role::Viewer,
                description: "Read-only access to resources".to_string(),
                permissions: Role::Viewer.get_permissions(),
                is_system: true,
            },
        ];

        for role_def in default_roles {
            self.roles.insert(role_def.name.clone(), role_def);
        }
    }

    pub async fn create_user(
        &mut self,
        username: String,
        email: String,
        roles: Vec<Role>,
    ) -> Result<User> {
        let user_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        let mut permissions = Vec::new();
        for role in &roles {
            permissions.extend(role.get_permissions());
        }

        let user = User {
            id: user_id.clone(),
            username: username.clone(),
            email: email.clone(),
            roles: roles.clone(),
            permissions,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.users.insert(user_id.clone(), user.clone());
        Ok(user)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<&User>> {
        Ok(self.users.get(user_id))
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<&User>> {
        Ok(self.users.values().find(|u| u.username == username))
    }

    pub async fn update_user_roles(&mut self, user_id: &str, roles: Vec<Role>) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            user.roles = roles.clone();
            user.updated_at = chrono::Utc::now();

            // Update permissions based on roles
            let mut permissions = Vec::new();
            for role in &roles {
                permissions.extend(role.get_permissions());
            }
            user.permissions = permissions;

            Ok(())
        } else {
            Err(SyrosError::ApiError(format!("User {} not found", user_id)))
        }
    }

    pub async fn add_user_permission(
        &mut self,
        user_id: &str,
        permission: Permission,
    ) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            if !user.permissions.contains(&permission) {
                user.permissions.push(permission);
                user.updated_at = chrono::Utc::now();
            }
            Ok(())
        } else {
            Err(SyrosError::ApiError(format!("User {} not found", user_id)))
        }
    }

    pub async fn remove_user_permission(
        &mut self,
        user_id: &str,
        permission: Permission,
    ) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            user.permissions.retain(|p| p != &permission);
            user.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(SyrosError::ApiError(format!("User {} not found", user_id)))
        }
    }

    pub async fn check_permission(&self, user_id: &str, permission: &Permission) -> Result<bool> {
        if let Some(user) = self.users.get(user_id) {
            if !user.is_active {
                return Ok(false);
            }

            // Check direct permissions
            if user.permissions.contains(permission) {
                return Ok(true);
            }

            // Check role permissions
            for role in &user.roles {
                if role.get_permissions().contains(permission) {
                    return Ok(true);
                }
            }

            Ok(false)
        } else {
            Ok(false)
        }
    }

    pub async fn check_resource_permission(
        &self,
        user_id: &str,
        resource_id: &str,
        permission: &Permission,
    ) -> Result<bool> {
        // First check general permission
        if !self.check_permission(user_id, permission).await? {
            return Ok(false);
        }

        // Check resource-specific permissions
        if let Some(resource) = self.resources.get(resource_id) {
            // Check if user owns the resource
            if resource.owner_id == user_id {
                return Ok(true);
            }

            // Check if user has permission for this resource type
            if let Some(user) = self.users.get(user_id) {
                for role in &user.roles {
                    if role.get_permissions().contains(permission) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    pub async fn create_custom_role(
        &mut self,
        name: String,
        description: String,
        permissions: Vec<Permission>,
    ) -> Result<()> {
        let role = Role::Custom(name.clone());
        let role_def = RoleDefinition {
            name: role.clone(),
            description,
            permissions,
            is_system: false,
        };

        self.roles.insert(role, role_def);
        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<&User>> {
        Ok(self.users.values().collect())
    }

    pub async fn get_all_roles(&self) -> Result<Vec<&RoleDefinition>> {
        Ok(self.roles.values().collect())
    }

    pub async fn deactivate_user(&mut self, user_id: &str) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            user.is_active = false;
            user.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(SyrosError::ApiError(format!("User {} not found", user_id)))
        }
    }

    pub async fn activate_user(&mut self, user_id: &str) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            user.is_active = true;
            user.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(SyrosError::ApiError(format!("User {} not found", user_id)))
        }
    }
}

impl Default for RBACManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rbac_manager_creation() {
        let rbac = RBACManager::new();
        assert!(!rbac.roles.is_empty());
    }

    #[tokio::test]
    async fn test_user_creation() {
        let mut rbac = RBACManager::new();
        let user = rbac
            .create_user(
                "testuser".to_string(),
                "test@example.com".to_string(),
                vec![Role::Developer],
            )
            .await
            .unwrap();

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.roles.contains(&Role::Developer));
    }

    #[tokio::test]
    async fn test_permission_check() {
        let mut rbac = RBACManager::new();
        let user = rbac
            .create_user(
                "testuser".to_string(),
                "test@example.com".to_string(),
                vec![Role::Developer],
            )
            .await
            .unwrap();

        assert!(rbac
            .check_permission(&user.id, &Permission::LockCreate)
            .await
            .unwrap());
        assert!(!rbac
            .check_permission(&user.id, &Permission::AdminUsers)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_role_permissions() {
        let admin_permissions = Role::Admin.get_permissions();
        let developer_permissions = Role::Developer.get_permissions();

        assert!(admin_permissions.contains(&Permission::AdminUsers));
        assert!(!developer_permissions.contains(&Permission::AdminUsers));
        assert!(developer_permissions.contains(&Permission::LockCreate));
    }
}
