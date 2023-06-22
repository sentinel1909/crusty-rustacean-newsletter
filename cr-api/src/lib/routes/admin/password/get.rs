// src/routes/home/get.rs

// dependencies
use crate::session_state::TypedSession;
use crate::{domain::ChangePasswordTemplate, errors::ResponseInternalServerError};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::response::Html;

// home page route, renders the main newsletter homepage
pub async fn change_password_form(
    session: TypedSession,
) -> Result<impl IntoResponse, ResponseInternalServerError<anyhow::Error>> {
    if session.get_user_id().is_none() {
        let response = Redirect::to("/login");
        return Ok(response.into_response());
    }

    let template = ChangePasswordTemplate;
    match template.render() {
        Ok(html) => Ok(Html(html).into_response()),
        Err(_) => Ok((StatusCode::NOT_FOUND, "page not found").into_response()),
    }
}
