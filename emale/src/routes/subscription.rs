use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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

fn generate_subs_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(20)
        .collect()
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
    base_url: web::Data<ApplicationBaseUrl>,
) -> HttpResponse {
    let subs: NewSubscriber = match req.0.try_into() {
        Ok(subs) => subs,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let subs_id = match insert_subscriber(&subs, &pool).await {
        Ok(subs_id) => subs_id,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let subs_token = generate_subs_token();
    if store_token(&pool, subs_id, &subs_token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(
        &email_client, 
        subs, 
        &base_url.0,
        &subs_token
        )
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Store subscription in database",
    skip(subs_token, pool)
)]
pub async fn store_token(
    pool: &PgPool,
    subs_id: Uuid,
    subs_token: &str
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriber_tokens (subscriber_token, subscriber_id)
        VALUES($1, $2)"#,
        subs_token,
        subs_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

pub async fn send_confirmation_email(
    email_client: &EmailClient,
    subs: NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscribe/confirm?subscription_token={}",
        base_url, subscription_token
    );
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
        .send_email(&subs.email, &subject, &html_body, &plain_body)
        .await
}

#[tracing::instrument(
  name = "Saving subscriber in database"
  skip(req, pool)
)]

pub async fn insert_subscriber(req: &NewSubscriber, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    let subs_id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO subscriber (id, email, name, subscribed_at, status)
    VALUES($1, $2, $3, $4, 'pending')
    "#,
        subs_id,
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

    Ok(subs_id)
}
