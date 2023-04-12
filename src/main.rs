// main.rs for Shuttle deployment

// dependencies
use axum::{
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use tracing::info;

// use routes from the cr-api-docker version of the project
use cr_api_docker::routes::*;

// start up the app using the shuttle runtime
#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    info!("Running database migration...");
    sqlx::migrate!("./cr-api/migrations")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    info!("Database migrated...spinning up the application...");
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);

    info!("API ready.  Current routes are: /health_check and /subscriptions");
    Ok(router.into())
}
