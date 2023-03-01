// main.rs

use cr_api::configuration::get_configuration;
use cr_api::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Construct an address and port, get the port from the configuration settings
    // pass the listener to the run function, which spins up the API
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = match TcpListener::bind(address) {
        Ok(listen) => listen,
        Err(err) => panic!("Could not get a valid address and port. {:?}", err),
    };
    run(listener)?.await
}
