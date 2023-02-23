// main.rs

use crusty_rustacean_api::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    run()?.await
}
