// main.rs

use cr_api_local::configuration::get_configuration;
use cr_api_local::startup::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
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
