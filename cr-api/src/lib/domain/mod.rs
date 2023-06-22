// src/lib/domain/mod.rs

// domain module definitions

mod new_subscriber;
mod subscriber_email;
mod subscriber_name;
mod templates;

pub use new_subscriber::NewSubscriber;
pub use subscriber_email::SubscriberEmail;
pub use subscriber_name::SubscriberName;
pub use templates::*;
