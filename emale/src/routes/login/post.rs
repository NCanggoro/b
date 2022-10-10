use actix_http::{header::LOCATION, StatusCode};
use actix_web::{HttpResponse, web, ResponseError};
use secrecy::Secret;
use sqlx::PgPool;

use crate::{authentication::{Credentials, validate_credentials, AuthError}, routes::error_chain_fmt};

#[derive(serde::Deserialize)]
pub struct FormData {
	username: String,
	password: Secret<String>
}

#[tracing::instrument(
	skip(form, pool),
	fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
	form: web::Form<FormData>, 
	pool: web::Data<PgPool>
) -> Result<HttpResponse, LoginError> {
	let credentials = Credentials {
		username: form.0.username,
		password: form.0.password
	};

	tracing::Span::current()
		.record("username", &tracing::field::display(&credentials.username));

	let user_id = validate_credentials(credentials, &pool)
		.await
		.map_err(|e| match e {
			AuthError::InvalidCredential(_) => LoginError::AuthError(e.into()),
			AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into())
		})?;
	tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

	Ok(HttpResponse::SeeOther()
		.insert_header((LOCATION, "/"))
		.finish())
}

#[derive(thiserror::Error)]
pub enum LoginError {
	#[error("Auth failed")]
	AuthError(#[source] anyhow::Error),
	#[error("Something went wrong")]
	UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for LoginError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		error_chain_fmt(self, f)
	}
}

impl ResponseError for LoginError {
	fn error_response(&self) -> HttpResponse<actix_http::body::BoxBody> {
		let encoded_error = urlencoding::Encoded::new(self.to_string());
		HttpResponse::build(self.status_code())
			.insert_header((LOCATION, format!("/login?error={}", encoded_error)))
			.finish()
	}
	fn status_code(&self) -> StatusCode {
		StatusCode::SEE_OTHER
	}
}
