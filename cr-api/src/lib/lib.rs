// lib.rs

// dependcencies
use axum::{
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;

use crate::routes::health_check::health_check;

pub mod routes;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

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
