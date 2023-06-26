// src/lib/routes/admin/dashboard.rs

// dependencies
use crate::domain::AdminDashboard;
use crate::errors::{e500, ResponseInternalServerError};
use crate::session_state::TypedSession;
use crate::state::AppState;
use anyhow::Context;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
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
    session: TypedSession,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    // check the logged in user has a matching session, redirect to the login page if they don't
    let username = if let Some(user_id) = session.get_user_id() {
        get_username(user_id, &app_state.db_pool)
            .await
            .map_err(e500)?
    } else {
        let response = Redirect::to("/login");
        return Ok(response.into_response());
    };

    // render the admin dashboard page
    let template = AdminDashboard { username };
    match template.render() {
        Ok(html) => Ok(Html(html).into_response()),
        Err(_) => Ok((StatusCode::NOT_FOUND, "page not found").into_response()),
    }
}
