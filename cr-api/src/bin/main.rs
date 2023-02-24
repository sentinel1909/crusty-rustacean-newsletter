// main.rs

use cr_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let address = match TcpListener::bind("127.0.0.1:8000") {
        Ok(addr) => addr,
        Err(err) => panic!("Could not get a valid address. {:?}", err),
    };
    run(address)?.await
}
