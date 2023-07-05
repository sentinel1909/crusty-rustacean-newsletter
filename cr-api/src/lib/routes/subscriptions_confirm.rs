// src/routes/subscriptions_confirm.rs

// dependencies

use crate::state::AppState;
use crate::{domain::SubscriptionConfirmationTemplate, errors::ConfirmationError};
use anyhow::Context;
use axum::extract::{Query, State};
use axum_flash::IncomingFlashes;
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt::Write;
use uuid::Uuid;

// struct to represent the query parameters, which includes a subscription token
#[derive(Debug, Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

// function which confirms the subscriber in the database
#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

// function which retrieves a subscriber id from an incoming subscription token
#[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.map(|r| r.subscriber_id))
}

// confirm handler
#[tracing::instrument(name = "Confirm a pending subscriber")]
pub async fn confirm(
    State(app_state): State<AppState>,
    flashes: IncomingFlashes,
    parameters: Query<Parameters>,
) -> Result<(IncomingFlashes, SubscriptionConfirmationTemplate), ConfirmationError> {
    let subscriber_id =
        get_subscriber_id_from_token(&app_state.db_pool, &parameters.subscription_token)
            .await
            .context("Failed to retrieve the subscriber id associated with the provided token.")?
            .ok_or(ConfirmationError::UnknownToken)?;
    confirm_subscriber(&app_state.db_pool, subscriber_id)
        .await
        .context("Failed to update the subscriber status to 'confirmed'.")?;

    // process any incoming flash messages
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the subscription confirmation page form from its associated Askama template
    // TODO: make sure errors are rendered properly, as it stands now, this page will render regardless of any errors
    let subscription_confirmation_template = SubscriptionConfirmationTemplate { flash_msg };

    Ok((flashes, subscription_confirmation_template))
}
