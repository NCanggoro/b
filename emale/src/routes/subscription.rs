use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use actix_http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use sqlx::{PgPool, Transaction, Postgres};
use uuid::Uuid;
use anyhow::Context;

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
) -> Result<HttpResponse, SubscribeError> {
    let subs: NewSubscriber = req.0.try_into().map_err(SubscribeError::ValidationError)?;

    let mut transaction =pool.begin()
        .await
        .context("failed to get postgre connection from pool")?;

    let subs_id = insert_subscriber(&mut transaction, &subs)
        .await
        .context("failed to insert new subs to database")?;

    let subs_token = generate_subs_token();
    store_token(&mut transaction, subs_id, &subs_token)
        .await
        .context("failed to save subs token to database")?;

    transaction
        .commit()
        .await
        .context("failed to commit SQL transaction")?;

    send_confirmation_email(
        &email_client, 
        subs, 
        &base_url.0,
        &subs_token
        )
        .await
        .context("failed to send email confirmation")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(
    name = "Store subscription in database",
    skip(subs_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subs_id: Uuid,
    subs_token: &str
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"INSERT INTO subscriber_tokens (subscriber_token, subscriber_id)
        VALUES($1, $2)"#,
        subs_token,
        subs_id
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        StoreTokenError(e)
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
  skip(req, transaction)
)]

pub async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    req: &NewSubscriber, 
) -> Result<Uuid, sqlx::Error> {
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
    .execute(transaction)
    .await
    .map_err(|e| {
        e
    })?;

    Ok(subs_id)
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }  
    Ok(())
}

// it is forbidden to implement a foreign trait for a foreign
// type, where foreign stands for “from another crate”
// https://doc.rust-lang.org/std/error/trait.Error.html#method.source

#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    // #[error("Failed to store confirmation token for new subs")]
    // StoreTokenError(#[from] StoreTokenError),
    // #[error("Failed to send confirmation email")]
    // SendEmailError(#[from] reqwest::Error),
    // #[error("Failed to get postgre connection from pool")]
    // PoolError(#[from] sqlx::Error),
    // #[error("failed to insert new subs in database")]
    // InsertSubscriberError(#[source] sqlx::Error),
    // #[error("failed to commit SQL transaction")]
    // TransactionError(#[source] sqlx::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}


impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}


impl ResponseError for SubscribeError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            SubscribeError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SubscribeError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // SubscribeError::PoolError(_)
            // | SubscribeError::InsertSubscriberError(_)
            // | SubscribeError::TransactionError(_)
            // | SubscribeError::StoreTokenError(_)
            // | SubscribeError::SendEmailError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub struct StoreTokenError(sqlx::Error);

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database error was encountered when trying to store subs token"
        )
    }
}

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
