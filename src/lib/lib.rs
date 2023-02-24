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
use std::net::TcpListener;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

// health_check handler
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Html(""))
}

// run function
pub fn run(listener: TcpListener) -> hyper::Result<App> {
    // routes and their corresponding handlers
    let app = Router::new().route("/health_check", get(health_check));

    // receive an address and port, spin up the server, panics with a message if an invalid address and port is received)
    // let addr = match address.parse() {
    //    Ok(addr) => addr,
    //    Err(err) => panic!("{:?}", err)
    // };
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
