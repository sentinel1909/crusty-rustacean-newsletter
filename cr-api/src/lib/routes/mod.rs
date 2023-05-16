//! src/lib/routes/mod.rs

pub mod health_check;
mod home;
pub mod newsletters;
pub mod subscriptions;
pub mod subscriptions_confirm;

pub use health_check::*;
pub use home::*;
pub use newsletters::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
