// main.rs

use cr_api_docker::configuration::get_configuration;
use cr_api_docker::startup::Application;
use cr_api_docker::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    // initialize tracing
    let subscriber = get_subscriber("cr-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // read configuration file, panic if we can't
    let configuration = get_configuration().expect("Failed to read configuration.");

    // return an instance of the application
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
