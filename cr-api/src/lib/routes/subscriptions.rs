// src/lib/routes/subscribe.rs

// dependencies
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::errors::IntoResponseError;
use crate::startup::AppState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Response, Result},
};
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

// data structure to model the incoming form data from the subscribe handler
#[derive(Deserialize)]
pub struct SubscriptionData {
    email: String,
    name: String,
}

#[derive(Debug)]
pub struct StoreTokenError(sqlx::Error);

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a subscription token."
        )
    }
}

impl IntoResponseError for StoreTokenError {}

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
    sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        StoreTokenError(e)
    })?;
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
    let _ = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(subscriber_id)
}

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
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );

    let html_body = &format!(
        "Welcome to our newsletter!<br />
        Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_email(new_subscriber.email, "Welcome!", html_body, &plain_body)
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
    State(app_state): State<AppState>,
    subscription_data: Form<SubscriptionData>,
) -> Result<Response, axum::Error> {
    let new_subscriber = match subscription_data.try_into() {
        Ok(subscription_data) => subscription_data,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let mut transaction = match app_state.db_pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let subscription_token = generate_subscription_token();
    store_token(&mut transaction, subscriber_id, &subscription_token).await?;

    if transaction.commit().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    if send_confirmation_email(
        &app_state.em_client,
        new_subscriber,
        &app_state.bs_url.0,
        &subscription_token,
    )
    .await
    .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
