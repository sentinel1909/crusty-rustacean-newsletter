// subscribe.rs

use axum::{
  http::StatusCode,
  Form
};

use serde::Deserialize;

// data structure to model the incoming form data from the subscribe route
#[derive(Deserialize)]
pub struct SubscriptionData {
    email: String,
    name: String
}

pub async fn subscribe(Form(subcription_data): Form<SubscriptionData>) -> StatusCode {
    StatusCode::OK
}

