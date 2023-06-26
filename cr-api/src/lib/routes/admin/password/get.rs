// src/routes/home/get.rs

// dependencies
use crate::session_state::TypedSession;
use crate::{domain::ChangePasswordTemplate, errors::ResponseInternalServerError};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::response::Html;
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use std::fmt::Write;

#[allow(clippy::let_with_type_underscore)]
#[debug_handler(state = axum_flash::Config)]
#[tracing::instrument(name = "Change password form", skip(flashes))]
// home page route, renders the main newsletter homepage
pub async fn change_password_form(
    flashes: IncomingFlashes,
    session: TypedSession,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    // check for a valid user session, if there isn't one, redirect to the login page
    if session.get_user_id().is_none() {
        let response = Redirect::to("/login");
        return Ok(response.into_response());
    }

    // process any incoming flash messages and convert them to a string for rendering
    let mut error_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(error_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the change password form, given that there is a valid user session, display any error message
    let template = ChangePasswordTemplate {
        flash_msg: error_msg,
    };
    match template.render() {
        Ok(html) => Ok(Html(html).into_response()),
        Err(_) => Ok((StatusCode::NOT_FOUND, "page not found").into_response()),
    }
}
