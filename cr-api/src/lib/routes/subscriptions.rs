// subscribe.rs

use axum::{
  http::StatusCode,
  Form
};

use serde::Deserialize;

// data structure to model the incoming form data from the subscribe route, will remove dead_code annotation in the future
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SubscriptionData {
    email: String,
    name: String
}

// subscriptions handler, for now the form paramater is not used and is marked as such
pub async fn subscribe(Form(_subcription_data): Form<SubscriptionData>) -> StatusCode {
    StatusCode::OK
}

