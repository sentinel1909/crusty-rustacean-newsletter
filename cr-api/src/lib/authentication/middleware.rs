// src/lib/authentication/middleware.rs

// dependencies
use crate::session_state::TypedSession;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use std::ops::Deref;
use uuid::Uuid;

// a struct to represent a user id type
#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

// implement the Display trait for the user id struct
impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

//
impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// reject anonymous users function
pub async fn reject_anonymous_users(
    session: TypedSession,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    match session.get_user_id() {
        Some(uid) => {
            request.extensions_mut().insert(UserId(uid));
            Ok(next.run(request).await)
        }
        None => {
            tracing::error!("User has not logged in.");
            Err(Redirect::to("/login"))
        }
    }
}
