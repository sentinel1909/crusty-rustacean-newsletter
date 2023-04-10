// subscribe.rs

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
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

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    State(pool): State<PgPool>,
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
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
    });
    Ok(())
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
    name = "Adding a new subscriber",
    skip(subscription_data, pool),
    fields(
        subscriber_email = %subscription_data.email,
        subscriber_name = %subscription_data.name
    )
)]
pub async fn subscribe(
    pool: State<PgPool>,
    subscription_data: Form<SubscriptionData>,
) -> impl IntoResponse {
    let new_subscriber = match subscription_data.try_into() {
        Ok(subscription_data) => subscription_data,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match insert_subscriber(pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
