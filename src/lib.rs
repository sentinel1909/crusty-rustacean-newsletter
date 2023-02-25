// lib.rs for shuttle deployment

use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

use cr_api::routes::health_check::health_check;

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
  let router = Router::new().route("/health_check", get(health_check));
  let sync_wrapper = SyncWrapper::new(router);

  Ok(sync_wrapper)
}