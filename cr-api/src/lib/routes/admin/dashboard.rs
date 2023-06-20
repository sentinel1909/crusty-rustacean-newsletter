// src/lib/routes/admin/dashboard.rs

use crate::errors::{e500, ResponseInternalServerError};
use crate::state::AppState;
use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_macros::debug_handler;
use axum_session::Session;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
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
    session: Session,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    let username = if let Some(user_id) = session.get::<Uuid>("user_id").map_err(e500)?
    {
        get_username(user_id, &app_state.db_pool).await.map_err(e500)?
    } else {
        return Ok(Redirect::to("/login"));
    };
    let response = Html((
        StatusCode::OK,
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Admin dashboard</title>
</head>
<body>
<p>Welcome {username}!</p>
</body>
</html>"#
        ),
    ));
    Ok(response.into_response())
}
