// src/lib/routes/admin/dashboard.rs

// dependencies
use crate::authentication::UserId;
use crate::domain::AdminDashboard;
use crate::errors::{e500, ResponseInternalServerError};
use crate::state::AppState;
use anyhow::Context;
use axum::{extract::State, response::Extension};
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use sqlx::PgPool;
use std::fmt::Write;
use uuid::Uuid;

// function which retrives the username from the users database and return it
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

// handler to render the admin dashboard, displaying the current logged in user's name
#[debug_handler]
pub async fn admin_dashboard(
    Extension(user_id): Extension<UserId>,
    flashes: IncomingFlashes,
    State(app_state): State<AppState>,
) -> Result<AdminDashboard, ResponseInternalServerError<anyhow::Error>> {
    // process any incoming flash messages
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }
    
    // get the logged in user's username
    let username = get_username(*user_id, &app_state.db_pool)
        .await
        .map_err(e500)?;

    // render the admin dashboard page
    let admin_dashboard_template = AdminDashboard { flash_msg, username };

    Ok(admin_dashboard_template)
}
