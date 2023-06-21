// src/routes/home/mod.rs

// dependencies
use crate::domain::HomeTemplate;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::response::Html;

// home page route, renders the main newsletter homepage
pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate;
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "page not found").into_response(),
    }
}
