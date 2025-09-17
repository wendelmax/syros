//! GraphQL schema definition and handlers.
//!
//! This module provides the GraphQL schema definition and HTTP handlers
//! for GraphQL operations in the Syros API.

use crate::api::graphql::{mutations::MutationRoot, queries::QueryRoot};
use crate::api::rest::ApiState;
use async_graphql::{EmptySubscription, Schema};
use axum::{extract::State, response::Html, response::Json};
use serde_json::Value;

/// Type alias for the Syros GraphQL schema.
pub type SyrosSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Creates a new GraphQL schema instance.
///
/// This function builds the GraphQL schema with the defined queries and mutations.
///
/// # Returns
///
/// Returns a configured GraphQL schema.
pub fn create_schema() -> SyrosSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

/// Handles GraphQL requests.
///
/// This handler processes GraphQL queries and mutations, executing them
/// against the schema and returning the results.
///
/// # Arguments
///
/// * `state` - API state containing service dependencies
/// * `payload` - GraphQL request payload
///
/// # Returns
///
/// Returns a JSON response with the GraphQL result.
pub async fn graphql_handler(
    State(state): State<ApiState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let schema = create_schema();
    let query = payload.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let variables = payload
        .get("variables")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    let result = schema.execute(query).await;
    Json(serde_json::to_value(result).unwrap_or(serde_json::Value::Null))
}

pub async fn graphql_playground() -> Html<&'static str> {
    Html(include_str!("playground.html"))
}
