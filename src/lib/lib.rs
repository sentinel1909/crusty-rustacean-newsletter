// lib.rs

// dependcencies
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, IntoMakeService},
    Server,
    Router,
};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;

// health_check handler
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Html(""))
}

// run function
pub fn run() -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    // routes and their corresponding handlers
    let app = Router::new().route("/health_check", get(health_check));

    // spin up and listen on part 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    Ok(server)
}
