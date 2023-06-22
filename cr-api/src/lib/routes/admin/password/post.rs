// src/routes/admin/password/post.rs

use crate::{errors::ResponseInternalServerError, session_state::TypedSession};
use axum::{
    extract::Form,
    response::{IntoResponse, Redirect},
};
use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PasswordData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    session: TypedSession,
    password_data: Form<PasswordData>,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    if session.get_user_id().is_none() {
        let response = Redirect::to("/login");
        return Ok(response.into_response());
    }
    todo!()
}
