// src/lib/routes/login/get.rs

use axum::{
    headers::Cookie,
    http::{StatusCode},
    response::{Html, IntoResponse},
    TypedHeader,
};

pub async fn login_form(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
    let error_html = match cookie.get("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie)
        }
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
