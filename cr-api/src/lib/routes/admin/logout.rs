// src/lib/admin/logout.rs

// dependencies
use crate::session_state::TypedSession;
use axum::response::{ErrorResponse, IntoResponse, Redirect};
use axum_flash::Flash;

// log out handler
pub async fn log_out(
    flash: Flash,
    session: TypedSession,
) -> Result<impl IntoResponse, ErrorResponse> {
    session.log_out();
    let flash = flash.info("You have successfully logged out.");
    Ok((flash, Redirect::to("/login")).into_response())
}
