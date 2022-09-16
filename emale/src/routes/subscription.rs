use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

impl TryFrom<SubscribeFormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: SubscribeFormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
  name = "Add new subscriber",
  skip(req, pool, email_client, base_url),
  fields(
    email   = %req.email,
    name    = %req.name
  )
)]
pub async fn subscribe(
    req: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
	base_url: web::Data<ApplicationBaseUrl>
) -> HttpResponse {
    let subs: NewSubscriber = match req.0.try_into() {
        Ok(subs) => subs,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_subscriber(&subs, &pool).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(&email_client, subs, &base_url).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

pub async fn send_confirmation_email(
    email_client: &EmailClient,
    subs: NewSubscriber,
	base_url: &ApplicationBaseUrl
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!("{}/subscribe/confirm?subscription_token=token", base_url.0);
    let plain_body = format!(
        "Hello<br />\
		Click <a href=\"{}\"> here</a> to confirm",
        confirmation_link
    );
    let html_body = format!(
        "Hello<br />\
  		Click <a href=\"{}\"> here</a> to confirm",
        confirmation_link
    );
    let subject = "hello";

    email_client
        .send_email(
			&subs.email, 
			&subject, 
			&html_body, 
			&plain_body
		)
        .await
}

#[tracing::instrument(
  name = "Saving subscriber in database"
  skip(req, pool)
)]

pub async fn insert_subscriber(req: &NewSubscriber, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriber (id, email, name, subscribed_at, status)
    VALUES($1, $2, $3, $4, 'pending')
    "#,
        Uuid::new_v4(),
        req.email.as_ref(),
        req.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed save subscriber {}", e);
        e
    })?;

    Ok(())
}
