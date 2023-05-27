// src/lib/routes/login/post.rs

// dependencies
use crate::authentication::{validate_credentials, Credentials};
use crate::errors::{AuthError, LoginError};
use crate::state::AppState;
use axum::debug_handler;
use axum::{
    extract::{Form, State},
    response::{ErrorResponse, IntoResponse, Redirect},
};
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct LoginData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(login_data, app_state),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[debug_handler]
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
            let query_string = format!("error={}", urlencoding::Encoded::new(e.to_string()));
            let hmac_tag = {
                let mut mac = Hmac::<sha2::Sha256>::new_from_slice(
                    app_state.hm_secret.0.expose_secret().as_bytes(),
                )
                .unwrap();
                mac.update(query_string.as_bytes());
                mac.finalize().into_bytes()
            };
            let response =
                Redirect::to((format!("/login?{}&tag={:x}", query_string, hmac_tag)).as_str());
            Err(ErrorResponse::from(response.into_response()))
        }
    }
}
