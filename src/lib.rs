//! lib.rs for shuttle deployment

// dependencies
use axum::{routing::get, Router};
use shuttle_service::error::CustomError;
use sqlx::PgPool;
use sync_wrapper::SyncWrapper;

// pull in routes from the cr_api_local crate
use cr_api_local::routes::health_check::health_check;

// shuttle specific startup function
#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] _pool: PgPool) -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/health_check", get(health_check));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
