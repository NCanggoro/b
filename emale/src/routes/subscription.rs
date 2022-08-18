use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use chrono::Utc;

#[derive(Deserialize, Debug)]
pub struct SubscribeFormData {
  name: String,
  email: String
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
  match insert_subscriber(&req, &pool).await {
    Ok(_)   => HttpResponse::Ok().finish(),
    Err(_)  => HttpResponse::InternalServerError().finish()
    
  }
}

#[tracing::instrument(
  name = "Saving subscriber in database"
  skip(req, pool)
)]

pub async fn insert_subscriber(
  req: &SubscribeFormData,
  pool: &PgPool
) -> Result<(),  sqlx::Error> {
  sqlx::query!(
    r#"
    INSERT INTO subscriber (id, email, name, subscribed_at)
    VALUES($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    req.email,
    req.name,
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