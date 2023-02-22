// main.rs

// dependcencies
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use color_eyre::eyre::Result;
use std::net::SocketAddr;

// health_check handler
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Html("<h1>Alive, 200 OK</h1>"))
}

// main function
#[tokio::main]
async fn main() -> Result<()> {
    // initialize color_eyre for nice looking error messages
    color_eyre::install()?;

    // routes and their corresponding handlers
    let app = Router::new().route("/health_check", get(health_check));

    // spin up and listen on part 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
