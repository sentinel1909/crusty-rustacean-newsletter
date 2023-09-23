// src/lib/routes/subscribe.rs

// dependencies
use crate::domain::PendingConfirmationTemplate;
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::errors::{StoreTokenError, SubscribeError};
use crate::state::AppState;
use anyhow::Context;
use axum::{
    extract::{Form, State},
    response::Result,
};
use axum_flash::IncomingFlashes;
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use sqlx::{Executor, Postgres, Transaction};
use std::fmt::Write;
use uuid::Uuid;

// data structure to model the incoming form data from the subscribe handler
#[derive(Deserialize)]
pub struct SubscriptionData {
    email: String,
    name: String,
}

// implement the TryFrom conversion trait for the incoming form data, to convert it into our domain data type
impl TryFrom<Form<SubscriptionData>> for NewSubscriber {
    type Error = String;

    fn try_from(value: Form<SubscriptionData>) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.0.name)?; // check to confirm name exists
        let email = SubscriberEmail::parse(value.0.email)?; // check to confirm email exists
        Ok(NewSubscriber { email, name })
    }
}

// function to generate a random 25-characters-long case-sensitive subscription token.
fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

// function which stores a subscription token within the subscription_tokens database
#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), StoreTokenError> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    );
    transaction.execute(query).await.map_err(StoreTokenError)?;
    Ok(())
}

// function to insert a new subscriber into the subscriptions database
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    );
    transaction.execute(query).await?;
    Ok(subscriber_id)
}

// function which sends out a confirmation email
#[tracing::instrument(
    name = "Sending a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );

    let plain_body = format!(
        "Welcome to the Crusty Rustacean newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );

    let html_body = &format!(
        "<h1>Crusty Rustacean - The Newsletter</h1>
        <h2>A source for all things Rust</h2>
        <h3>Mostly once a month...mostly...</h3>
        <p>Welcome!</p>
        <p>Click <a href=\"{}\">here</a> to confirm your subscription.</p>
        <p>After clicking, a confirmation page will come up to confirm.",
        confirmation_link
    );

    email_client
        .send_email(&new_subscriber.email, "Welcome!", html_body, &plain_body)
        .await
}

// subscribe handler function
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscription_data, app_state),
    fields(
        subscriber_email = %subscription_data.email,
        subscriber_name = %subscription_data.name
    )
)]
pub async fn subscribe(
    flashes: IncomingFlashes,
    State(app_state): State<AppState>,
    subscription_data: Form<SubscriptionData>,
) -> Result<(IncomingFlashes, PendingConfirmationTemplate), SubscribeError> {
    let new_subscriber = subscription_data
        .try_into()
        .map_err(SubscribeError::ValidationError)?;

    let mut transaction = app_state
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool.")?;

    let subscriber_id = insert_subscriber(&mut transaction, &new_subscriber)
        .await
        .context("Failed to insert a new subscriber in the database.")?;

    let subscription_token = generate_subscription_token();
    store_token(&mut transaction, subscriber_id, &subscription_token)
        .await
        .context("Failed to store the confirmation token for a new subscriber.")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;

    send_confirmation_email(
        &app_state.em_client,
        new_subscriber,
        &app_state.bs_url.0,
        &subscription_token,
    )
    .await
    .context("Failed to send a confirmation email.")?;

    // process any incoming flash messages and convert them to a string for rendering
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the change password form, given that there is a valid user session, display any error message
    // TODO: make sure errors are rendered properly, as it stands now, this page will render regardless of any errors
    let pending_confirmation_template = PendingConfirmationTemplate { flash_msg };

    Ok((flashes, pending_confirmation_template))
}
