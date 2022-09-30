use actix_http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use anyhow::Context;

use crate::{email_client::EmailClient, domain::SubscriberEmail};

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

pub async fn publish_newsletter(
	body: web::Json<Body>,
	pool: web::Data<PgPool>,
	email_client: web::Data<EmailClient>
) -> Result<HttpResponse, PublishError> {
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
	#[error(transparent)]
	UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for PublishError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		error_chain_fmt(self, f)
	}
}

impl ResponseError for PublishError {
	fn status_code(&self) -> actix_http::StatusCode {
		match self {
			PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}