// src/routes/newsletters.rs

// dependencies
use axum::{
  http::StatusCode,
  response::IntoResponse,
};

// publish newsletter handler
pub async fn publish_newsletter() -> impl IntoResponse {
  StatusCode::OK
}

