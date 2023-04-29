// main.rs for Shuttle deployment

// dependencies
use axum::{routing::get, Router};

// hello handler, returns a simple message
async fn hello_world() -> &'static str {
    "Hello, world!"
}

// main function, annoted with the Shuttle runtime macro
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // create a router, with a /hello endpoint
    let router = Router::new().route("/hello", get(hello_world));

    // return the router
    Ok(router.into())
}
