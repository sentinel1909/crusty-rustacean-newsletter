// main.rs for Shuttle deployment

// dependencies, internal and external
use axum::{routing::get, Router};
use cr_api_docker::routes::health_check;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

// main function, annoted with the Shuttle runtime macro
#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // shuttle sets up a global tracing subscriber for us, so no need to set up tracing

    // read in configuration
    // struggling here...

    // run the migration schema for the database
    pool.execute(include_str!(
        "../cr-api/migrations/20230225210742_create_subscriptions_table.sql"
    ))
    .await
    .map_err(CustomError::new)?;

    // create the application, with a /health_check endpoint
    let application = Router::new().route("/health_check", get(health_check));

    // return the application
    Ok(application.into())
}
