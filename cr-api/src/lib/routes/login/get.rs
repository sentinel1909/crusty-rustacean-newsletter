// src/lib/routes/login/get.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::response::Html;
use axum_flash::IncomingFlashes;
use axum_macros::debug_handler;
use std::fmt::Write;

// login_form handler
#[allow(clippy::let_with_type_underscore)]
#[debug_handler(state = axum_flash::Config)]
#[tracing::instrument(name = "Login form", skip(flashes))]
pub async fn login_form(flashes: IncomingFlashes) -> impl IntoResponse {
    let mut error_html = String::new();

    for (level, text) in flashes.iter() {
        writeln!(
            error_html,
            "<p><strong>{:?}</strong>: <i>{}</i></p>\n",
            level, text
        )
        .unwrap();
    }

    let response = Html((
        StatusCode::OK,
        format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            
            <body>
                {error_html}
                <form action="/login" method="post">
                    <label>Username
                        <input type="text" placeholder="Enter Username" name="username">
                    </label>
                    <label>Password
                        <input type="password" placeholder="Enter Password" name="password">
                    </label>
                    <button type="submit">Login</button>
                </form>
            </body>
            
            </html>
            "#
        ),
    ));

    (flashes, response)
}
