// subscribe.rs

use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use axum_macros::debug_handler;
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

// subscriptions handler, for now the form paramater is not used and is marked as such
#[debug_handler]
pub async fn subscribe(
    State(pool): State<PgPool>,
    Form(subscription_data): Form<SubscriptionData>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscription_data.email,
        subscription_data.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
