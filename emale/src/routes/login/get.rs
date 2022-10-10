use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!("<p><i>{error_message}</i></p>"),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
			<!DOCTYPE html>
			<html lang="en">
			<head>
			<meta charset="UTF-8">
			<meta http-equiv="X-UA-Compatible" content="IE=edge">
			<meta name="viewport" content="width=device-width, initial-scale=1.0">
			<title>Login</title>
			</head>
			<body>
			{error_html}
			<form action="/login" method="post">
				<label>Username</label>
				<input
					type="text"
					placeholder="Username"
					name="username"
				>
				<label>Password</label>
				<input
					type="password"
					placeholder="Password"
					name="password"
				>
				<button type="submit">Login</button>
			</form>
			</body>
			</html>
		"#
        ))
}
