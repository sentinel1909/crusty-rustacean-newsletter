// subscribe.rs

use crate::domain::{NewSubscriber, SubscriberName};
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
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
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
    let name = match SubscriberName::parse(subscription_data.0.name) {
        Ok(name) => name,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    let new_subscriber = NewSubscriber {
        email: subscription_data.0.email,
        name,
    };
    match insert_subscriber(pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
