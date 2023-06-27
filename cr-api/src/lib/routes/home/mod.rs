// src/routes/home/mod.rs

// dependencies
use crate::domain::HomeTemplate;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::response::Html;

// home page route, renders the home page from its associated Askama template
pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate;
    let (status, response_body) = match template.render() {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(_) => (StatusCode::NOT_FOUND, Html("page not found".to_string())),
    };

    (status, response_body).into_response()
}
