// main.rs for Shuttle deployment
// Note: this is the boilerplate starter for an Axum API
// see https://docs.shuttle.rs/examples/axum

use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world));

    Ok(router.into())
}

