// src/lib/routes/login/post.rs

// dependencies
use crate::authentication::{validate_credentials, Credentials};
use crate::errors::{AuthError, LoginError};
use crate::state::AppState;
use axum::{
    extract::{Form, State},
    http::{header, HeaderMap},
    response::{ErrorResponse, IntoResponse, Redirect},
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
) -> Result<impl IntoResponse, ErrorResponse> {
    let credentials = Credentials {
        username: login_data.0.username,
        password: login_data.0.password,
    };

    match validate_credentials(credentials, &app_state.db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Ok(Redirect::to("/"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };

            let response = Redirect::to("/login").into_response();
            let mut headers = HeaderMap::new();
            headers.insert(header::SET_COOKIE, format!("_flash={e}").parse().unwrap());
            Err(ErrorResponse::from((headers, response).into_response()))
        }
    }
}
