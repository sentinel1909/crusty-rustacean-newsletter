// src/lib/routes/login/get.rs

// dependencies
use crate::domain::LoginTemplate;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::response::Html;
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use std::fmt::Write;

#[allow(clippy::let_with_type_underscore)]
#[debug_handler(state = axum_flash::Config)]
#[tracing::instrument(name = "Login form", skip(flashes))]
// login_form handler
pub async fn login_form(flashes: IncomingFlashes) -> impl IntoResponse {
    // process any incoming flash messages
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the login page, errors regarding login failure are posted as a flash message
    let template = LoginTemplate {
        flash_msg: flash_msg,
    };
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "page not found").into_response(),
    }
}
