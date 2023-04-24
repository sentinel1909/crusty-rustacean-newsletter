// src/lib/errors.rs

// dependencies
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// enum to represent a SubscribeError, has two variants, ValidationError is user facing, UnexpectedError is operator facing
#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// implement the Debug trait for SubscribeError, calls the error_chain_fmt helper function for formatting
impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// implement the IntoResponse trait for SubscribeError
impl IntoResponse for SubscribeError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        let status = match self {
            SubscribeError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SubscribeError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}

// struct to represent a StoreTokenError, wraps a sqlx::Error
pub struct StoreTokenError(pub sqlx::Error);

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a subscription token."
        )
    }
}

// implement the Debug trait for StoreTokenError
impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// convert a StoreToken error into std::error::Error
impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

// helper function to nicely format the std::error::Error message chain
fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
