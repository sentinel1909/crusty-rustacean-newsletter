// src/lib/routes/login/post.rs

// dependencies
use crate::authentication::{validate_credentials, Credentials};
use crate::errors::{AuthError, LoginError};
use crate::state::AppState;
use axum::{
    extract::{Form, State},
    response::{ErrorResponse, IntoResponse, Redirect},
};
use axum_flash::Flash;
use axum_session::{Session, SessionRedisPool};
use secrecy::Secret;

// struct to represent the login data, including username and password
#[derive(serde::Deserialize)]
pub struct LoginData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(login_data, app_state, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    State(app_state): State<AppState>,
    flash: Flash,
    session: Session<SessionRedisPool>,
    login_data: Form<LoginData>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let credentials = Credentials {
        username: login_data.0.username,
        password: login_data.0.password,
    };

    match validate_credentials(credentials, &app_state.db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session.set("user_id", user_id);
            Ok(Redirect::to("/admin/dashboard"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            tracing::error!("{:?}", &e);

            let flash = flash.error(e.to_string());

            let response = Redirect::to("/login").into_response();

            Err(ErrorResponse::from((flash, response).into_response()))
        }
    }
}
