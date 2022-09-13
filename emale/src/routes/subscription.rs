use crate::domain::{NewSubscriber, SubscriberName, SubscriberEmail};
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
  type Error= String;

  fn try_from(value: SubscribeFormData) -> Result<Self, Self::Error> {
      let name = SubscriberName::parse(value.name)?;
      let email = SubscriberEmail::parse(value.email)?;
      Ok(Self { email, name })
  }
}

#[tracing::instrument(
  name = "Add new subscriber",
  skip(req, pool),
  fields(
    email   = %req.email,
    name    = %req.name
  )
)]
pub async fn subscribe(
  req: web::Form<SubscribeFormData>, 
  pool: web::Data<PgPool>
) -> HttpResponse {
    let subs = match req.0.try_into() {
      Ok(subs) => subs,
      Err(_) => return HttpResponse::BadRequest().finish()
    };

    match insert_subscriber(&subs, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
  name = "Saving subscriber in database"
  skip(req, pool)
)]

pub async fn insert_subscriber(
  req: &NewSubscriber, 
  pool: &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriber (id, email, name, subscribed_at, status)
    VALUES($1, $2, $3, $4, 'ok')
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
