// lib/src/errors.rs

// borrowed concept from actix_web::Error, we're going to build our own version
// we need a trait called IntoResponseError, we can build our errors by implementing this trait

// dependencies
use axum::{
  http::StatusCode,
};
use std::{
  fmt,
};

pub trait IntoResponseError: fmt::Debug + fmt::Display {

  fn status_code(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }
}