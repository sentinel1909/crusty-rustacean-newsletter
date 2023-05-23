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

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    let response = match validate_credentials(credentials, &app_state.db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Redirect::to("/").into_response()
        }

        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };

            tracing::error!("{:?}", &e);

            let response = Redirect::to("/login").into_response();

            response.into_response()
        }
    };

    Ok(response)
}
