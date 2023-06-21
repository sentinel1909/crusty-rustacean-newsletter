// src/lib/domain/templates.rs

// domain template types

// dependencies
use askama::Template;

// struct to represent the home page template
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;
