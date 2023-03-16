// main.rs

use cr_api_local::configuration::get_configuration;
use cr_api_local::startup::run;
use cr_api_local::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    
    // initialize tracing
    let subscriber = get_subscriber("cr-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // setup the database connection pool;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(configuration.database.connection_string().expose_secret())
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
