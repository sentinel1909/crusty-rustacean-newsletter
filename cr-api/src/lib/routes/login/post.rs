// src/lib/routes/login/post.rs

use crate::authentication::{validate_credentials, Credentials};
use crate::errors::{AuthError, LoginError};
use crate::state::AppState;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct LoginData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(login_data, app_state),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    State(app_state): State<AppState>,
    login_data: Form<LoginData>,
) -> Result<impl IntoResponse, LoginError> {
    let credentials = Credentials {
        username: login_data.0.username,
        password: login_data.0.password,
    };

    let user_id = validate_credentials(credentials, &app_state.db_pool)
        .await
        .map_err(|e| match e {
            AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
            AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
        })?;

    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

    Ok(Redirect::to("/"))
}
