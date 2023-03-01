//! src/lib/startup.rs

// dependcencies, external and internal
use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

// run function
pub fn run(listener: TcpListener) -> hyper::Result<App> {
    // routes and their corresponding handlers
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
