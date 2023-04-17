// health_check.rs

// dependencies
use axum::http::StatusCode;
use axum::response::IntoResponse;

// health_check handler, returns an OK response with no body
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
