// src/lib/errors.rs

// dependencies
use axum::{
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use hyper::header;

// enum to represent a subscribe error, has two variants, validation error is user is user facing, unexpected error is operator facing
#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// implement the Debug trait for subscribe error, calls the error_chain_fmt helper function for formatting
impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// implement the IntoResponse trait for subscribe error
impl IntoResponse for SubscribeError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        let (status, msg) = match self {
            SubscribeError::ValidationError(_) => (StatusCode::BAD_REQUEST, "bad request"),
            SubscribeError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error")
            }
        };

        (status, msg).into_response()
    }
}

// struct to represent a store token error, wraps a sqlx::Error
pub struct StoreTokenError(pub sqlx::Error);

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a subscription token."
        )
    }
}

// implement the Debug trait for the store token error type
impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// convert a store token error into std::error::Error
impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

// enum to represent a confirmation error, has two variants,Unexpected Error and UnknownToken
#[derive(thiserror::Error)]
pub enum ConfirmationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("There is no subscriber associated with the provided token.")]
    UnknownToken,
}

// implement the Debug trait for the confirmation error type
impl std::fmt::Debug for ConfirmationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// implement the IntoResponse trait for the confirmation error type
impl IntoResponse for ConfirmationError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        let (status, msg) = match self {
            Self::UnknownToken => (StatusCode::UNAUTHORIZED, "unauthorized"),
            Self::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };

        (status, msg).into_response()
    }
}

// enum to represent a publish error
#[derive(thiserror::Error)]
pub enum PublishError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// implement the Debug trait for the publish error type
impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// implement the IntoResponse trait for the publish error type
impl IntoResponse for PublishError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            PublishError::AuthError(_) => {
                let mut response = StatusCode::UNAUTHORIZED.into_response();
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
        }
    }
}

// enum to represent an authentication error
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// enum to represent a login error
#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

// implement the IntoResponse trait for the login error type
impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        let (status, msg) = match self {
            LoginError::AuthError(_) => (StatusCode::UNAUTHORIZED, "unauthorized"),
            LoginError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };

        (status, msg).into_response()
    }
}

// implement the Debug trait for LoginError
impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ResponseInternalServerError<T>(#[from] T);

impl<T: std::fmt::Debug> IntoResponse for ResponseInternalServerError<T> {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

// return an opaque 500 while preserving the error's root cause for logging
pub fn e500<T>(e: T) -> ResponseInternalServerError<T>
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    ResponseInternalServerError::from(e)
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
