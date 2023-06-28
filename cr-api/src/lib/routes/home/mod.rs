// src/routes/home/mod.rs

// dependencies
use crate::domain::HomeTemplate;

// home page route, renders the home page from its associated Askama template
pub async fn home() -> HomeTemplate {
    HomeTemplate
}
