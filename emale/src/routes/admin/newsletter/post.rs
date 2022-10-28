use crate::authentication::UserId;
use crate::domain::SubscriberEmail;
use crate::idempotency::{
    save_response, try_proccesing, IdempotencyKey, NextAction,
};
use crate::utils::{error_400, error_500, see_other};
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::{PgPool, Transaction, Postgres};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    text_content: String,
    html_content: String,
    idempotency_key: String,
}

#[tracing::instrument(
	name = "Publish a newsletter",
	skip(user_id, pool, form),
	fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]

pub async fn publish_newsletter(
    form: web::Form<FormData>,
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // destructure form to avoid upsetting borrow checker
    let user_id = user_id.into_inner();
    let FormData {
        title,
        text_content,
        html_content,
        idempotency_key,
    } = form.0;
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(error_400)?;
    let mut transaction = match try_proccesing(&pool, &idempotency_key, *user_id)
        .await
        .map_err(error_500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            success_message().send();
            return Ok(saved_response);
        }
    };
	let issue_id = insert_newsletter_issue(&mut transaction, &title, &text_content, &html_content)
		.await
		.context("Failed to store newsletter isuue details")
		.map_err(error_500)?;

	enqueue_delivery_tasks(&mut transaction, issue_id)
		.await
		.context("Failed to enqueue delivery tasks")
		.map_err(error_500)?;
	
    let response = see_other("/admin/newsletters");
    let response = save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(error_500)?;

    Ok(response)
}

#[tracing::instrument(skip_all)]
async fn insert_newsletter_issue(
	transaction: &mut Transaction<'_, Postgres>,
	title: &str,
	text_content: &str,
	html_content: &str,
) -> Result <Uuid, sqlx::Error> {
	let newsletter_issue_id = Uuid::new_v4();
	sqlx::query!(
		r#"
			INSERT INTO newsletter_issues (
				newsletter_issue_id,
				title,
				text_content,
				html_content,
				published_at
			)
			VALUES ($1, $2, $3, $4, now())
		"#,
		newsletter_issue_id,
		title,
		text_content,
		html_content
	)
	.execute(transaction)
	.await?;
	Ok(newsletter_issue_id)
}

#[tracing::instrument(skip_all)]
async fn enqueue_delivery_tasks (
	transaction: &mut Transaction<'_, Postgres>,
	newsletter_issue_id: Uuid
) -> Result<(), sqlx::Error> {
	sqlx::query!(
		r#"
			INSERT INTO issue_delivery_queue (
				newsletter_issue_id,
				subscriber_email
			)
			SELECT $1, email
			FROM subscriber
			WHERE status = 'confirmed'
		"#,
		newsletter_issue_id
	)
	.execute(transaction)
	.await?;
	Ok(())
}

fn success_message() -> FlashMessage {
    FlashMessage::info("Newsletter has been published")
}

struct ConfirmedSubs {
    email: SubscriberEmail,
}

#[tracing::instrument(name = "Get confirmed subs", skip(pool))]
async fn get_confirmed_subs(
    pool: &PgPool,
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
        Ok(email) => Ok(ConfirmedSubs { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();

    Ok(confirmed_subs)
}
