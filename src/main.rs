// main.rs for Shuttle deployment

// dependencies
use anyhow::anyhow;
use axum::{
    routing::get,
    Router,
};
use cr_api_docker::routes::health_check;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

// shuttle runtime main function
#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool, #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    // load secrets from Secrets.toml (production)
    let _secret = if let Some(secret) = secret_store.get("BASE_URL") {
        secret
    } else {
        return Err(anyhow!("Secret was not found.").into());
    };

    // set up the database pool and perform migrations
    pool.execute(include_str!(
        "../cr-api/migrations/20230225210742_create_subscriptions_table.sql"
    ))
    .await
    .map_err(CustomError::new)?;

    // spin up the router
    let router = Router::new()
        .route("/health_check", get(health_check));

    // return the router
    Ok(router.into())
}
