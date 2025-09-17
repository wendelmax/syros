//! RBAC handlers for the Syros API.
//!
//! This module provides HTTP handlers for Role-Based Access Control (RBAC)
//! operations, including user management, role assignment, and permission checking.

use crate::api::rest::ApiState;
use crate::auth::{Permission, Role};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

pub async fn create_user(
    State(state): State<ApiState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac
        .create_user(payload.username, payload.email, payload.roles)
        .await
    {
        Ok(user) => Ok(Json(json!({
            "success": true,
            "data": user
        }))),
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let rbac = state.rbac_manager.lock().await;

    match rbac.get_user(&user_id).await {
        Ok(Some(user)) => Ok(Json(json!({
            "success": true,
            "data": user
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_by_username(
    State(state): State<ApiState>,
    Path(username): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let rbac = state.rbac_manager.lock().await;

    match rbac.get_user_by_username(&username).await {
        Ok(Some(user)) => Ok(Json(json!({
            "success": true,
            "data": user
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get user by username: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_user_roles(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
    Json(payload): Json<UpdateUserRolesRequest>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac.update_user_roles(&user_id, payload.roles).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "User roles updated successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to update user roles: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_user_permission(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
    Json(payload): Json<AddPermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac.add_user_permission(&user_id, payload.permission).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "Permission added successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to add user permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn remove_user_permission(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
    Json(payload): Json<RemovePermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac
        .remove_user_permission(&user_id, payload.permission)
        .await
    {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "Permission removed successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to remove user permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn check_permission(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
    Json(payload): Json<CheckPermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let rbac = state.rbac_manager.lock().await;

    match rbac.check_permission(&user_id, &payload.permission).await {
        Ok(has_permission) => Ok(Json(json!({
            "success": true,
            "has_permission": has_permission
        }))),
        Err(e) => {
            tracing::error!("Failed to check permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn check_resource_permission(
    State(state): State<ApiState>,
    Path((user_id, resource_id)): Path<(String, String)>,
    Json(payload): Json<CheckResourcePermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let rbac = state.rbac_manager.lock().await;

    match rbac
        .check_resource_permission(&user_id, &resource_id, &payload.permission)
        .await
    {
        Ok(has_permission) => Ok(Json(json!({
            "success": true,
            "has_permission": has_permission
        }))),
        Err(e) => {
            tracing::error!("Failed to check resource permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_custom_role(
    State(state): State<ApiState>,
    Json(payload): Json<CreateCustomRoleRequest>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac
        .create_custom_role(payload.name, payload.description, payload.permissions)
        .await
    {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "Custom role created successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to create custom role: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_users(State(state): State<ApiState>) -> Result<Json<Value>, StatusCode> {
    let rbac = state.rbac_manager.lock().await;

    match rbac.get_all_users().await {
        Ok(users) => Ok(Json(json!({
            "success": true,
            "data": users
        }))),
        Err(e) => {
            tracing::error!("Failed to get all users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_roles(State(state): State<ApiState>) -> Result<Json<Value>, StatusCode> {
    let rbac = state.rbac_manager.lock().await;

    match rbac.get_all_roles().await {
        Ok(roles) => Ok(Json(json!({
            "success": true,
            "data": roles
        }))),
        Err(e) => {
            tracing::error!("Failed to get all roles: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn deactivate_user(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac.deactivate_user(&user_id).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "User deactivated successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to deactivate user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn activate_user(
    State(state): State<ApiState>,
    Path(user_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let mut rbac = state.rbac_manager.lock().await;

    match rbac.activate_user(&user_id).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "User activated successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to activate user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


/// Request structure for creating a new user.
#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    /// Username for the new user
    pub username: String,
    /// Email address for the new user
    pub email: String,
    /// List of roles to assign to the user
    pub roles: Vec<Role>,
}

/// Request structure for updating user roles.
#[derive(serde::Deserialize)]
pub struct UpdateUserRolesRequest {
    /// New list of roles for the user
    pub roles: Vec<Role>,
}

/// Request structure for adding a permission to a user.
#[derive(serde::Deserialize)]
pub struct AddPermissionRequest {
    /// Permission to add
    pub permission: Permission,
}

/// Request structure for removing a permission from a user.
#[derive(serde::Deserialize)]
pub struct RemovePermissionRequest {
    /// Permission to remove
    pub permission: Permission,
}

/// Request structure for checking user permissions.
#[derive(serde::Deserialize)]
pub struct CheckPermissionRequest {
    /// Permission to check
    pub permission: Permission,
}

/// Request structure for checking resource-specific permissions.
#[derive(serde::Deserialize)]
pub struct CheckResourcePermissionRequest {
    /// Permission to check
    pub permission: Permission,
}

/// Request structure for creating a custom role.
#[derive(serde::Deserialize)]
pub struct CreateCustomRoleRequest {
    /// Name of the custom role
    pub name: String,
    /// Description of the role
    pub description: String,
    /// List of permissions for the role
    pub permissions: Vec<Permission>,
}
