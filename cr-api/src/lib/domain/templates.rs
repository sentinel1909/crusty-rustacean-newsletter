// src/lib/domain/templates.rs

// domain template types

// dependencies
use askama::Template;

// struct to represent the home page template
#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

// struct to represent the change password form template
#[derive(Template)]
#[template(path = "change_password_form.html")]
pub struct ChangePasswordTemplate;
