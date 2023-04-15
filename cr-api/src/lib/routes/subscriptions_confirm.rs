// src/routes/subscriptions_confirm.rs

// dependencies
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

// struct to represent the query parameters
#[derive(Debug, Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

// confirm function
#[tracing::instrument(name = "Confirm a pending subscriber")]
pub async fn confirm(_parameters: Query<Parameters>) -> impl IntoResponse {
    StatusCode::OK
}
