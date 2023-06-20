//! src/lib/routes/mod.rs

mod admin;
pub mod health_check;
mod home;
mod login;
pub mod newsletters;
pub mod subscriptions;
pub mod subscriptions_confirm;

pub use admin::*;
pub use health_check::*;
pub use home::*;
pub use login::*;
pub use newsletters::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
