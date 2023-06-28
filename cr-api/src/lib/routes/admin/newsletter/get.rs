// src/routes/admin/newsletter/get.rs

// dependencies
use crate::domain::PublishNewsletterTemplate;
use crate::errors::ResponseInternalServerError;
use crate::session_state::TypedSession;
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use std::fmt::Write;

#[debug_handler(state = axum_flash::Config)]
#[tracing::instrument(name = "Publish newsletter form", skip(flashes))]
// home page route, renders the main newsletter homepage
pub async fn publish_newsletter_form(
    flashes: IncomingFlashes,
    session: TypedSession,
) -> Result<(IncomingFlashes, PublishNewsletterTemplate), ResponseInternalServerError<anyhow::Error>> {
    // process any incoming flash messages and convert them to a string for rendering
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the change password form, given that there is a valid user session, display any error message
    let publish_newsletter_template = PublishNewsletterTemplate { flash_msg };

    Ok((flashes, publish_newsletter_template))
}