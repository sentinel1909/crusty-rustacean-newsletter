// src/lib/routes/login/get.rs

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: Query<QueryParams>) -> impl IntoResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!(
            "<p><i>{}</i></p>",
            htmlescape::encode_minimal(&error_message)
        ),
    };

    let body = Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Login</title>
</head>
<body>
{error_html}
<form action="/login" method="post">
<label>Username
<input
type="text"
placeholder="Enter Username"
name="username"
>
</label>
<label>Password
<input
type="password"
placeholder="Enter Password"
name="password"
>
</label>
<button type="submit">Login</button>
</form>
</body>
</html>"#,
    ));
    (StatusCode::OK, body)
}
