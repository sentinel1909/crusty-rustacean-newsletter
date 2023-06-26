// src/lib/admin/logout.rs

use crate::session_state::TypedSession;
use axum::response::{IntoResponse, Redirect};
use axum_flash::Flash;

pub async fn log_out(flash: Flash, session: TypedSession) -> impl IntoResponse {
    if session.get_user_id().is_none() {
        let response = Redirect::to("/login");
        response.into_response()
    } else {
        session.log_out();
        let flash = flash.info("You have successfully logged out.");
        let response = Redirect::to("/login");
        (flash, response).into_response()
    }
}
