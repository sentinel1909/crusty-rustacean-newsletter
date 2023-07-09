// src/lib/domain/templates.rs

// domain template types

// dependencies
pub use askama::*;
pub use axum::response::{IntoResponse, Response};
use http::StatusCode;
use uuid::Uuid;

// struct to represent the home page template
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub flash_msg: String,
}

// struct to represent the login page template
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub flash_msg: String,
}

// struct to represent teh admin dashboard template
#[derive(Template)]
#[template(path = "admin_dashboard.html")]
pub struct AdminDashboard {
    pub flash_msg: String,
    pub username: String,
}

// struct to represent the change password form template
#[derive(Template)]
#[template(path = "change_password_form.html")]
pub struct ChangePasswordTemplate {
    pub flash_msg: String,
}

// struct to represent the publish newsletter form template
#[derive(Template)]
#[template(path = "publish_newsletter_form.html")]
pub struct PublishNewsletterTemplate {
    pub flash_msg: String,
    pub idempotency_key: Uuid,
}

// struct to represent the subscription conformation template
#[derive(Template)]
#[template(path = "subscription_confirmed.html")]
pub struct SubscriptionConfirmationTemplate {
    pub flash_msg: String,
}

// implement IntoResponse for the Askama templates
pub fn into_response<T: Template>(t: &T) -> Response {
    match t.render() {
        Ok(body) => {
            let headers = [(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(T::MIME_TYPE),
            )];

            (headers, body).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
