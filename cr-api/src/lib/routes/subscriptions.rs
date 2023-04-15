// subscribe.rs

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::AppState;
use axum::debug_handler;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

// data structure to model the incoming form data from the subscribe route, will remove dead_code annotation in the future
#[derive(Deserialize)]
pub struct SubscriptionData {
    email: String,
    name: String,
}

impl TryFrom<Form<SubscriptionData>> for NewSubscriber {
    type Error = String;

    fn try_from(value: Form<SubscriptionData>) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.0.name)?;
        let email = SubscriberEmail::parse(value.0.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Sending a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
) -> Result<(), reqwest::Error> {
    let confirmation_link = "https://there-is-no-such-domain.com/subscriptions/confirm";

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

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscription_data, app_state),
    fields(
        subscriber_email = %subscription_data.email,
        subscriber_name = %subscription_data.name
    )
)]
#[debug_handler(state = AppState)]
pub async fn subscribe(
    State(app_state): State<AppState>,
    subscription_data: Form<SubscriptionData>,
) -> impl IntoResponse {
    let new_subscriber = match subscription_data.try_into() {
        Ok(subscription_data) => subscription_data,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    if insert_subscriber(&app_state.db_pool, &new_subscriber)
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    if send_confirmation_email(&app_state.em_client, new_subscriber)
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}
