// src/routes/home/mod.rs

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn home() -> impl IntoResponse {
    let body = Html(include_str!("home.html"));
    (StatusCode::OK, body)
}
