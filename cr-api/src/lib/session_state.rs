// src/lib/session_state.rs

use axum_macros::FromRequestParts;
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;
use uuid::Uuid;

#[derive(Debug, FromRequestParts)]
pub struct TypedSession(Session<SessionRedisPool>);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) {
        self.0.set(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Option<Uuid> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(self) {
        self.0.destroy()
    }
}
