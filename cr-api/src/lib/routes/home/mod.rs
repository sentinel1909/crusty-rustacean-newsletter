// src/routes/home/mod.rs

use axum::{http::StatusCode, response::Html};

pub async fn home() -> (StatusCode, Html<&'static str>) {
    let body = Html(include_str!("home.html"));
    (StatusCode::OK, body)
}
