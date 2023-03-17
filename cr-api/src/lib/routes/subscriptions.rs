// subscribe.rs

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
    skip(subscription_data, pool)
)]
pub async fn insert_subscriber(
    State(pool): State<PgPool>,
    Form(subscription_data): Form<SubscriptionData>,
) -> hyper::Result<()> {
    let _ = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscription_data.email,
        subscription_data.name,
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
    match insert_subscriber(pool, subscription_data).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
