use crate::api::graphql::{mutations::MutationRoot, queries::QueryRoot};
use crate::api::rest::ApiState;
use async_graphql::{EmptySubscription, Schema};
use axum::{extract::State, response::Html, response::Json};
use serde_json::Value;

pub type SyrosSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> SyrosSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

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
