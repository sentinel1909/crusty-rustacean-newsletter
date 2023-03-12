// subscribe.rs

use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
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
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber...",
        %request_id,
        subscriber_email = %subscription_data.email,
        subscriber_name = %subscription_data.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "Saving new subscriber into the database..."
    );
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
    .instrument(query_span)
    .await
    {
        Ok(_) => { 
            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                request_id
            );
            StatusCode::OK
        },
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
