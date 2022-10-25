use crate::authentication::{UserId};
use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::utils::{error_500, see_other};
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse,};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    text_content: String,
    html_content: String,
}

#[tracing::instrument(
	name = "Publish a newsletter",
	skip(user_id, pool, form, email_client),
	fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]

pub async fn publish_newsletter(
    form: web::Form<FormData>,
    user_id: ReqData<UserId>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, actix_web::Error> {
    let subs = get_confirmed_subs(&pool).await.map_err(error_500)?;
    for subscriber in subs {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        &subscriber.email,
                        &form.title,
                        &form.html_content,
                        &form.text_content,
                    )
                    .await
                    .with_context(|| 
						format!("failed to send newsletter {}", subscriber.email
					))
                    .map_err(error_500)?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    error.message = %error,
                    "Skipping confrirmed subs, invalid email"
                )
            }
        }
    }
    FlashMessage::info("Newsletter has been published").send();

    Ok(see_other("/admin/newsletters"))
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
