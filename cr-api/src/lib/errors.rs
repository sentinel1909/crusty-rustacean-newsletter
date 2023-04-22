// src/lib/errors.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct StoreTokenError(pub sqlx::Error);

// implement IntoResponse
impl IntoResponse for StoreTokenError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, tracing::error!("{}", self.0.to_string())).into_response()
    }
}
