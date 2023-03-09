// health_check.rs

use axum::http::StatusCode;

// health_check handler
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
