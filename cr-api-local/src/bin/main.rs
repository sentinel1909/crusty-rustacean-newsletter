// main.rs

use cr_api_local::configuration::get_configuration;
use cr_api_local::startup::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    // Redirect logs to subscriber
    LogTracer::init().expect("Failed to set logger");
    
    // Initialize tracing
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "cr-api".into(),
        std::io::stdout
    );

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // setup the database connection pool
    let db_connection_str = configuration.database.connection_string().to_string();
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
    {
        Ok(pool) => pool,
        Err(err) => panic!("Could not get a database connection pool. {:?}", err),
    };

    // Construct an address and port, get the port from the configuration settings
    // pass the listener to the run function, which spins up the API
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = match TcpListener::bind(address) {
        Ok(listen) => listen,
        Err(err) => panic!("Could not get a valid address and port. {:?}", err),
    };
    run(listener, db_pool)?.await
}
