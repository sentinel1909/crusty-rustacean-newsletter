// src/lib/routes/login/post.rs

// dependencies
use crate::authentication::{validate_credentials, Credentials};
use crate::errors::{AuthError, LoginError};
use crate::session_state::TypedSession;
use crate::state::AppState;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use axum_flash::Flash;
use axum_macros::debug_handler;
use secrecy::Secret;

// struct to represent the login data, including username and password
#[derive(serde::Deserialize)]
pub struct LoginData {
    username: String,
    password: Secret<String>,
}

// handler to process results received from the login form
#[debug_handler(state = crate::state::AppState)]
#[tracing::instrument(
    skip(login_data, app_state, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    State(app_state): State<AppState>,
    flash: Flash,
    session: TypedSession,
    login_data: Form<LoginData>,
) -> Result<impl IntoResponse, LoginError> {
    // build an instance of the credentials struct using the received form data (username and password)
    let credentials = Credentials {
        username: login_data.0.username,
        password: login_data.0.password,
    };

    // check the users credentials, allow them through into the admin dashboard if they're validated, create a session for this user
    let response = match validate_credentials(credentials, &app_state.db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", tracing::field::display(&user_id));
            session.renew();
            session.insert_user_id(user_id);
            Redirect::to("/admin/dashboard").into_response()
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            tracing::error!("{:?}", &e);

            let flash = flash.error(e.to_string());

            let response = Redirect::to("/login").into_response();

            (flash, response).into_response()
        }
    };

    Ok(response)
}
