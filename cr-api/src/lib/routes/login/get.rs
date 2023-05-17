// src/lib/routes/login/get.rs

use axum::{http::StatusCode, response::Html};

pub async fn login_form() -> (StatusCode, Html<&'static str>) {
    let body = Html(include_str!("login.html"));
    (StatusCode::OK, body)
}
