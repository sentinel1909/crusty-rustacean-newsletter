// health_check.rs

use axum::{
  http::StatusCode,
  response::{Html, IntoResponse},
};

// health_check handler
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Html(""))
}