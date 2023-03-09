//! lib.rs for shuttle deployment

// dependencies
use axum::{routing::get, Router};
use sqlx::PgPool;
use sync_wrapper::SyncWrapper;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

// shuttle specific startup function
#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
