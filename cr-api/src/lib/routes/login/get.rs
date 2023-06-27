// src/lib/routes/login/get.rs

// dependencies
use crate::domain::LoginTemplate;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::response::Html;
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use std::fmt::Write;

// login_form handler
#[allow(clippy::let_with_type_underscore)]
#[debug_handler(state = axum_flash::Config)]
#[tracing::instrument(name = "Login form", skip(flashes))]
pub async fn login_form(flashes: IncomingFlashes) -> impl IntoResponse {
    // process any incoming flash messages
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the login form from its associated Askama template
    let template = LoginTemplate { flash_msg };
    let response = match template.render() {
        Ok(body) => Html((StatusCode::OK, body)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "page not found").into_response(),
    };

    (flashes, response)
}
