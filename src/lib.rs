//! lib.rs for shuttle deployment

// dependencies
use axum::{
    routing::{get, post},
    Router,
};
use shuttle_service::error::CustomError;
use sqlx::PgPool;
use sync_wrapper::SyncWrapper;

// use routes from the cr-api-local version of the project
use cr_api_local::routes::*;

// shuttle specific startup function
#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleAxum {
    sqlx::migrate!("./cr-api-local/migrations")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;
    
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
