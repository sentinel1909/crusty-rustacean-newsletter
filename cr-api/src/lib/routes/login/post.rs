// src/lib/routes/login/post.rs

use axum::response::{IntoResponse, Redirect};

pub async fn login() -> impl IntoResponse {
    Redirect::to("/").into_response()
}
