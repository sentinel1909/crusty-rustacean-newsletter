// src/routes/admin/password/post.rs

use crate::authentication::{validate_credentials, Credentials};
use crate::errors::{e500, AuthError};
use crate::routes::admin::dashboard::get_username;
use crate::session_state::TypedSession;
use crate::state::AppState;
use axum::{
    extract::{Form, State},
    response::{ErrorResponse, IntoResponse, Redirect},
};
use axum_flash::Flash;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PasswordData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    State(app_state): State<AppState>,
    flash: Flash,
    session: TypedSession,
    password_data: Form<PasswordData>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Ensure the new password is the correct length
    if password_data.new_password.expose_secret().len() < 12
        || password_data.new_password.expose_secret().len() > 128
    {
        let flash = flash.error("The new password should be between 12 and 128 characters long.");
        return Ok((flash, Redirect::to("/admin/password")).into_response());
    }

    //
    let user_id = session.get_user_id();
    if user_id.is_none() {
        let response = Redirect::to("/login");
        return Ok(response.into_response());
    }
    let user_id = user_id.unwrap();

    if password_data.new_password.expose_secret()
        != password_data.new_password_check.expose_secret()
    {
        let flash =
            flash.error("You entered two different new passwords - the field values must match.");
        let response = Redirect::to("/admin/password");
        return Ok((flash, response).into_response());
    }

    let username = get_username(user_id, &app_state.db_pool)
        .await
        .map_err(e500)?;

    let credentials = Credentials {
        username,
        password: password_data.0.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &app_state.db_pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                let flash = flash.error("The current password is incorrect.");
                let response = Redirect::to("/admin/password");
                Ok((flash, response).into_response())
            }
            AuthError::UnexpectedError(_) => Err(e500(e).into()),
        };
    }

    crate::authentication::change_password(
        user_id,
        password_data.0.new_password,
        &app_state.db_pool,
    )
    .await
    .map_err(e500)?;
    let flash = flash.info("Your password has been changed.");
    let response = Redirect::to("/admin/password");
    Ok((flash, response).into_response())
}
