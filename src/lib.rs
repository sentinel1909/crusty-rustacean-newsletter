//! lib.rs for shuttle deployment

// dependencies
use axum::{
    routing::{get, post},
    Router,
};
use shuttle_service::error::CustomError;
use sqlx::PgPool;
use sync_wrapper::SyncWrapper;
use tracing::info;

// use routes from the cr-api-local version of the project
use cr_api_docker::routes::*;

// start up the app using the shuttle service
#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleAxum {
    info!("Running database migration...");
    sqlx::migrate!("./cr-api/migrations")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    info!("Database migrated...spinning up routes...");
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);

    let sync_wrapper = SyncWrapper::new(router);

    info!("API ready.  Current routes are: /health_check and /subscriptions");
    Ok(sync_wrapper)
}
