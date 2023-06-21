// src/lib/domain/new_subscriber.rs

// domain new subscriber type

// dependencies
use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;

// a struct to represent a new subscriber type
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
