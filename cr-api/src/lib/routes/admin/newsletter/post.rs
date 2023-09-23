// src/routes/admin/newsletter/post.rs

// dependencies
use crate::authentication::UserId;
use crate::errors::{e400, e500, ResponseError};
use crate::idempotency::IdempotencyKey;
use crate::idempotency::{save_response, try_processing, NextAction};
use crate::state::AppState;
use anyhow::Context;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_flash::Flash;
use serde::Deserialize;
use sqlx::{Executor, Postgres, Transaction};
use uuid::Uuid;
use validator::Validate;

// a struct to represent the form data received from the newsletter publish form
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct NewsletterData {
    #[validate(length(min = 5))]
    title: String,
    #[validate(length(min = 5))]
    text_content: String,
    #[validate(length(min = 5))]
    html_content: String,
    idempotency_key: String,
}

pub static PUBLISH_SUCCESS_INFO_MESSAGE: &str =
    "The newsletter issue has been accepted - emails will go out shortly.";

// a function which publishes a newsletter issue
#[tracing::instrument(skip_all)]
async fn insert_newsletter_issue(
    transaction: &mut Transaction<'_, Postgres>,
    title: &str,
    text_content: &str,
    html_content: &str,
) -> Result<Uuid, sqlx::Error> {
    let newsletter_issue_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO newsletter_issues (
            newsletter_issue_id,
            title,
            text_content,
            html_content,
            published_at
        )
        VALUES ($1, $2, $3, $4, now())
        "#,
        newsletter_issue_id,
        title,
        text_content,
        html_content
    );
    transaction.execute(query).await?;
    Ok(newsletter_issue_id)
}

// a function to queue delivery tasks
#[tracing::instrument(skip_all)]
async fn enqueue_delivery_tasks(
    transaction: &mut Transaction<'_, Postgres>,
    newsletter_issue_id: Uuid,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO issue_delivery_queue (
            newsletter_issue_id,
            subscriber_email
        )
        SELECT $1, email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
        newsletter_issue_id,
    );
    transaction.execute(query).await?;
    Ok(())
}

// publish newsletter handler
#[tracing::instrument(
name = "Publish a newsletter issue",
skip(flash, newsletter_data, app_state, user_id),
fields(user_id=%*user_id)
)]
pub async fn publish_newsletter(
    Extension(user_id): Extension<UserId>,
    flash: Flash,
    State(app_state): State<AppState>,
    newsletter_data: Form<NewsletterData>,
) -> Result<impl IntoResponse, ResponseError> {
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

    // destructure the validated data and idempotency key (to make the borrow checker happy)
    let NewsletterData {
        title,
        html_content,
        text_content,
        idempotency_key,
    } = validated_data.0.clone();

    // convert the incoming idempotency key received from the newsletter form data into our IdempotencyKey type
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;

    // call the try_processing function to deal with concurrent requests
    let mut transaction = match try_processing(&app_state.db_pool, &idempotency_key, *user_id)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            let flash = flash.info(PUBLISH_SUCCESS_INFO_MESSAGE);
            return Ok((flash, saved_response).into_response());
        }
    };

    let issue_id = insert_newsletter_issue(&mut transaction, &title, &text_content, &html_content)
        .await
        .context("Failed to store newsletter issue details")
        .map_err(e500)?;

    enqueue_delivery_tasks(&mut transaction, issue_id)
        .await
        .context("Failed to enqueue delivery tasks")
        .map_err(e500)?;

    // build and send the success response message after the newsletter issue has been published
    let flash = flash.info(PUBLISH_SUCCESS_INFO_MESSAGE);
    let response = (flash, Redirect::to("/admin/newsletter")).into_response();
    let response = save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(e500)?;
    Ok((response).into_response())
}
