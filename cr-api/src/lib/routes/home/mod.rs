// src/routes/home/mod.rs

// dependencies
use crate::domain::HomeTemplate;
use axum_flash::IncomingFlashes;
use std::fmt::Write;

// home page route, renders the home page from its associated Askama template
pub async fn home(flashes: IncomingFlashes) -> (IncomingFlashes, HomeTemplate) {
    // process any incoming flash messages
    let mut flash_msg = String::new();
    for (level, text) in flashes.iter() {
        writeln!(flash_msg, "{:?}: {}\n", level, text).unwrap();
    }

    // render the login form from its associated Askama template
    let login_template = HomeTemplate { flash_msg };

    (flashes, login_template)
}
