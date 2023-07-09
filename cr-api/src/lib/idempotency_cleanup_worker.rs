// src/lib/idempotency_cleanup_worker.rs

// dependencies
use crate::{configuration::Settings, startup::get_connection_pool};
use sqlx::PgPool;
use std::time::Duration;

// function to run the idempotency cleanup worker until stopped
pub async fn run_cleanup_until_stopped(configuration: Settings) -> Result<(), anyhow::Error> {
    let connection_pool = get_connection_pool(&configuration.database);
    worker_loop(connection_pool).await
}

// function to run the idempotency cleanup worker in a loop
async fn worker_loop(pool: PgPool) -> Result<(), anyhow::Error> {
    loop {
        remove_old_idempotency_keys(&pool).await?;
        tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;
    }
}

// function to remove old idempotency keys out of the associated database
#[tracing::instrument(skip_all)]
pub async fn remove_old_idempotency_keys(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM idempotency
        WHERE
            created_at < now()
             - interval '5 days'
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
