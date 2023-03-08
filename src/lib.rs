//! lib.rs for shuttle deployment

// dependencies
use axum::{
    routing::{get, post},
    Router,
};
use shuttle_service::error::CustomError;
use sqlx::{Executor, PgPool};
use sync_wrapper::SyncWrapper;

// pull in the routes from the non-shuttle side of the app
use cr_api::routes::health_check::health_check;
use cr_api::routes::subscriptions::subscribe;

// shuttle specific startup function
#[shuttle_service::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_service::ShuttleAxum {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);
    
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
