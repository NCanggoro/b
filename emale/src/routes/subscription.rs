use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
use serde::Deserialize;
use chrono::Utc;

#[derive(Deserialize)]
pub struct SubscribeFormData {
  name: String,
  email: String
}

pub async fn subscribe(
  req: web::Form<SubscribeFormData>,
  pool: web::Data<PgPool>
) -> HttpResponse {
  let id = Uuid::new_v4();
  let req_span = tracing::info_span!(
    "Add new subscriber",
    %id,
    email = %req.email,
    name = %req.name
  );

  let _req_span_guard = req_span.enter();
  let query_span = tracing::info_span!("Saving new subscriber in database");

  match sqlx::query!(
    r#"
    INSERT INTO subscriber (id, email, name, subscribed_at)
    VALUES($1, $2, $3, $4)
    "#,
    id,
    req.email,
    req.name,
    Utc::now()
  )
  .execute(pool.as_ref())
  .instrument(query_span)
  .await
  {
    Ok(_) =>  {
      HttpResponse::Ok().finish()
    },
    Err(e) => {
      tracing::error!("Failed to execute query: {:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}