// src/lib/routes/subscribe.rs

// dependencies
use anyhow::Context;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Result, Response}, 
};
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::AppState;
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

// create an AppError type to wrap an anyhow::Error
pub struct AppError(anyhow::Error);

// implement IntoResponse for AppError
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            tracing::error!("App Error: {}", self.0)
        )
            .into_response()
    }
}

// implement the From trait for AppError
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
// an enum to wrap errors related to the subscription function
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// implement IntoResponse for the StoreTokenError type, converts the sqlx::Error into a generic error
impl IntoResponse for SubscribeError {
    fn into_response(self) -> Response {
        match self {
            SubscribeError::ValidationError(e) => (StatusCode::BAD_REQUEST, tracing::error!(e)).into_response(),
            SubscribeError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }       
    }
}

// implement From trait to convert a StoreTokenError into a SubscribeError
impl From<StoreTokenError> for SubscribeError {
    fn from(error: StoreTokenError) -> Self {
        SubscribeError::UnexpectedError(error.0.into())
    } 
}

// a type to wrap a sqlx::Error
pub struct StoreTokenError(sqlx::Error);

// implement the TryFrom conversion trait for the incoming form data, to convert it into our domain data type
impl TryFrom<Form<SubscriptionData>> for NewSubscriber {
    type Error = String;

    fn try_from(value: Form<SubscriptionData>) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.0.name)?;            // check to confirm name exists
        let email = SubscriberEmail::parse(value.0.email)?;        // check to confirm email exists
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
) -> Result<(), SubscribeError> {
    sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(StoreTokenError)?;
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
) -> Result<StatusCode, AppError> {
    let new_subscriber = match subscription_data.try_into() {
        Ok(subscription_data) => subscription_data,
        Err(_) => return Ok(StatusCode::BAD_REQUEST),
    };

    let mut transaction = app_state.db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let subscriber_id = insert_subscriber(&mut transaction, &new_subscriber)
        .await
        .context("Failed to insert new subscriber in the database.")?;

    let subscription_token = generate_subscription_token();
    store_token(&mut transaction, subscriber_id, &subscription_token)
        .await
        .context("Failed to store the confirmation token for a new subscriber.")?;

    transaction.commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")?;

    send_confirmation_email(&app_state.em_client, new_subscriber, &app_state.bs_url.0, &subscription_token,)
        .await
        .context("Failed to send a confirmation email.")?;

    Ok(StatusCode::OK)
}
