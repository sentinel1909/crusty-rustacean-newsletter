// src/routes/home/get.rs

// dependencies
use crate::session_state::TypedSession;
use crate::{domain::ChangePasswordTemplate, errors::ResponseInternalServerError};
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::response::Html;
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use std::fmt::Write;

#[debug_handler(state = axum_flash::Config)]
#[tracing::instrument(name = "Change password form", skip(flashes))]
// home page route, renders the main newsletter homepage
pub async fn change_password_form(
    flashes: IncomingFlashes,
    session: TypedSession,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    // process any incoming flash messages and convert them to a string for rendering
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the change password form, given that there is a valid user session, display any error message
    let template = ChangePasswordTemplate { flash_msg };
    match template.render() {
        Ok(body) => Ok(Html((StatusCode::OK, body)).into_response()),
        Err(_) => Ok((StatusCode::NOT_FOUND, "page not found").into_response()),
    }
}
