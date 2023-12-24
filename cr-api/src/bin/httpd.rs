// cr-api/src/main.rs

// dependencies
use anyhow::{Context, Result};
use cr_api::configuration::get_configuration;
use cr_api::idempotency_cleanup_worker::run_cleanup_until_stopped;
use cr_api::issue_delivery_worker::run_delivery_until_stopped;
use cr_api::startup::Application;
use cr_api::telemetry::{get_subscriber, init_subscriber};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

// report exit function
fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            "{} failed",
            task_name
            )
        }
        Err(e) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            "{}' task failed to complete",
            task_name
            )
        }
    }
}

// main function
#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    let subscriber = get_subscriber("cr-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // read configuration file
    let configuration =
        get_configuration().context("Failed to get the application configuration settings...")?;

    // return an instance of the application
    let application = Application::build(configuration.clone())
        .await
        .context("Failed to build the application...")?;
    let application_task = tokio::spawn(application.run_until_stopped());

    // define the delivery processing service worker
    let email_delivery_task = tokio::spawn(run_delivery_until_stopped(configuration.clone()));

    // define the idempotency cleanup service worker
    let idempotency_cleanup_task = tokio::spawn(run_cleanup_until_stopped(configuration));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = email_delivery_task => report_exit("Email delivery worker", o),
        o = idempotency_cleanup_task => report_exit("Idempotency cleanup worker", o),
    };
    Ok(())
}
