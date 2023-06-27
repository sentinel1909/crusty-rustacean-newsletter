// src/lib/routes/admin/dashboard.rs

// dependencies
use crate::authentication::UserId;
use crate::domain::AdminDashboard;
use crate::errors::{e500, ResponseInternalServerError};
use crate::state::AppState;
use anyhow::Context;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Extension, IntoResponse},
};
use axum_extra::response::Html;
use axum_macros::debug_handler;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}

#[debug_handler]
pub async fn admin_dashboard(
    Extension(user_id): Extension<UserId>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    // get the logged in user's username
    let username = get_username(*user_id, &app_state.db_pool)
        .await
        .map_err(e500)?;

    // render the admin dashboard page
    let template = AdminDashboard { username };
    match template.render() {
        Ok(body) => Ok(Html((StatusCode::OK, body)).into_response()),
        Err(_) => Ok((StatusCode::NOT_FOUND, "page not found").into_response()),
    }
}
