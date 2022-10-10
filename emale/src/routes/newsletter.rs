use actix_web::{web, HttpResponse, ResponseError, HttpRequest};
use actix_web::http::{StatusCode, header};
use actix_web::http::header::{HeaderMap, HeaderValue};
use secrecy::{Secret};
use sqlx::PgPool;
use anyhow::Context;
use crate::email_client::EmailClient;
use crate::domain::SubscriberEmail;
use crate::authentication::{AuthError, Credentials, validate_credentials};

use super::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct Body {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

struct ConfirmedSubs {
    email: SubscriberEmail,
}
 
fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
	let header_value = headers
		.get("Authorization")
		.context("The 'Authorization' header was missing")?
		.to_str()
		.context("The 'Authorization' header was not a valid UTF8 string")?;
	let base64encoded_segment = header_value
		.strip_prefix("Basic ")
		.context("the authorization scheme was not 'Basic'")?;
	let decoded_bytes = base64::decode_config(base64encoded_segment, base64::STANDARD)
		.context("Failed to decode credentials")?;
	let decoded_credetials = String::from_utf8(decoded_bytes)
		.context("Decoded credentials string is not valid UTF8")?;
	
	let mut credentials = decoded_credetials.splitn(2, ":");
	let username = credentials
		.next()
		.ok_or_else(|| anyhow::anyhow!("Username must be provided in 'Basic' Auth"))?
		.to_string();
	let password = credentials
		.next()
		.ok_or_else(|| anyhow::anyhow!("Password must be provided in 'Basic' Auth"))?
		.to_string();

	Ok(Credentials { 
		username,
		password: Secret::new(password) 
	})

}

#[tracing::instrument(name = "Get confirmed subs", skip(pool))]
async fn get_confirmed_subs(
	pool: &PgPool
) -> Result<Vec<Result<ConfirmedSubs, anyhow::Error>>, anyhow::Error> {
    let confirmed_subs = sqlx::query!(
        r#"
		SELECT email
		FROM subscriber
		WHERE status = 'confirmed'
		"#
    )
    .fetch_all(pool)
    .await?
	.into_iter()
		.map(|r| match SubscriberEmail::parse(r.email) {
			Ok(email) => Ok(ConfirmedSubs{ email }),
			Err(error) => Err(anyhow::anyhow!(error))
		})
	.collect();

	Ok(confirmed_subs)
}

#[tracing::instrument(
	name = "Publish a newsletter",
	skip(request, pool, body, email_client),
	fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]

pub async fn publish_newsletter(
	request: HttpRequest,
	body: web::Json<Body>,
	pool: web::Data<PgPool>,
	email_client: web::Data<EmailClient>
) -> Result<HttpResponse, PublishError> {
	let credentials = basic_authentication(request.headers())
		.map_err(PublishError::AuthError)?;
	tracing::Span::current()
		.record("username", &tracing::field::display(&credentials.username));
	let user_id = validate_credentials(credentials, &pool)
		.await
		// match on `AuthError`'s variants, pass the errors into the 
		// constructors for `PublishError` variants. This ensures that
		// the context of the top-level wrapper is preserved when the 
		// error is logged by middleware
		.map_err(|e| match e {
			AuthError::InvalidCredential(_) => PublishError::AuthError(e.into()),
			AuthError::UnexpectedError(_) => PublishError::UnexpectedError(e.into())
		})?;
	tracing::Span::current()
		.record("username", &tracing::field::display(&user_id));
	let subs = get_confirmed_subs(&pool).await?;
	for subscriber in subs {
		match subscriber {
			Ok(subscriber) => {
				email_client
					.send_email(
						&subscriber.email, 
						&body.title, 
						&body.content.html, 
						&body.content.text
					)
					.await
					.with_context(|| {
						format!(
							"failed to send newsletter {}",
							subscriber.email
						)
					})?;
			},
			Err(error) => {
				tracing::warn!(
					error.cause_chain = ?error,
					"skipping a subs. \
					email is invalid"
				)
			}
		}
	}
    Ok(HttpResponse::Ok().finish())
}

#[derive(thiserror::Error)]
pub enum PublishError {
	#[error("Authentication failed")]
	AuthError(#[source] anyhow::Error),
	#[error(transparent)]
	UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for PublishError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		error_chain_fmt(self, f)
	}
}

impl ResponseError for PublishError {
	fn error_response(&self) -> HttpResponse<actix_http::body::BoxBody> {
		match self {
			PublishError::UnexpectedError(_) => {
				HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
			}
			PublishError::AuthError(_) => {
				let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
				let header_value = HeaderValue::from_str(r#"Basic realm="publish""#)
					.unwrap();
				response
					.headers_mut()
					.insert(header::WWW_AUTHENTICATE, header_value);

				response
				
			}
		}		
	}
}