// src/routes/admin/newsletter/post.rs

// dependencies
use crate::authentication::UserId;
use crate::domain::SubscriberEmail;
use crate::errors::{e500, ResponseInternalServerError};
use crate::state::AppState;
use anyhow::Context;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_flash::Flash;
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

// a struct to represent the form data received from the newsletter publish form
#[derive(Debug, Deserialize, Validate)]
pub struct NewsletterData {
    #[validate(length(min = 5))]
    title: String,
    #[validate(length(min = 5))]
    text_content: String,
    #[validate(length(min = 5))]
    html_content: String,
}

// a struct to represent a confirmed subscriber
struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

// function which gets all confirmed subscribers from the database
#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
    )
    .fetch_all(pool)
    .await?;

    let confirmed_subscribers = rows
        .into_iter()
        .map(|r| match SubscriberEmail::parse(r.email) {
            Ok(email) => Ok(ConfirmedSubscriber { email }),
            Err(error) => Err(anyhow::anyhow!(error)),
        })
        .collect();

    Ok(confirmed_subscribers)
}

// publish newsletter handler
#[tracing::instrument(
name = "Publish a newsletter issue",
skip(newsletter_data, app_state, user_id),
fields(user_id=%*user_id)
)]
pub async fn publish_newsletter(
    Extension(user_id): Extension<UserId>,
    flash: Flash,
    State(app_state): State<AppState>,
    newsletter_data: Form<NewsletterData>,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    // validate the form data
    let validated_data = match newsletter_data.validate() {
        Ok(_) => {
            tracing::trace!("Successfully extracted form body.");
            &newsletter_data
        }
        Err(e) => {
            tracing::trace!("Unable to extract form body: {:?}", e);
            let flash = flash.error("Part of the form body has less than 5 characters");
            return Ok((flash, Redirect::to("/admin/newsletter")).into_response());
        }
    };

    // send out a newsletter issue to all the confirmed subscribers, using the validated form data
    let subscribers = get_confirmed_subscribers(&app_state.db_pool)
        .await
        .map_err(e500)?;
    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                app_state
                    .em_client
                    .send_email(
                        &subscriber.email,
                        &validated_data.title,
                        &validated_data.html_content,
                        &validated_data.text_content,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {}", subscriber.email)
                    })
                    .map_err(e500)?;
            }
            Err(error) => {
                tracing::warn!(
                error.cause_chain = ?error,
                "Skipping a confirmed subscriber. \
                Their stored contact details are invalid",
                );
            }
        }
    }

    // build and send the success response message
    let flash = flash.info("The newsletter issue has been published.");
    let response = Redirect::to("/admin/newsletter");
    Ok((flash, response).into_response())
}
