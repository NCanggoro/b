use actix_web::cookie::Cookie;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, HttpRequest};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use std::fmt::Write;

pub async fn login_form(
	flash_messages: IncomingFlashMessages
) -> HttpResponse {
	let mut error_html = String::new();
	for m in flash_messages
		.iter()
		.filter(|m| m.level() == Level::Error) {
			writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
		}

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
