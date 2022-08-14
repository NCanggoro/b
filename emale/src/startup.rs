use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::{Server};
use serde::{Serialize, Deserialize};
use std::net::TcpListener;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
struct Power {
  num: u32
}

#[derive(Deserialize)]
struct SubscribeFormData {
  name: String,
  email: String
}

async fn pow2(req: HttpRequest) -> impl Responder {
  let body: u32 = req.match_info()
               .get("num")
               .unwrap()
               .parse::<u32>()
               .unwrap();
  let res = Power{num: u32::pow(2, body)};

  HttpResponse::Ok()
    .json(&res)
}


async fn subscribe(
  req: web::Form<SubscribeFormData>,
  pool: web::Data<PgPool>
) -> HttpResponse {
  match sqlx::query!(
    r#"
    INSERT INTO subscriber (id, email, name, subscribed_at)
    VALUES($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    req.email,
    req.name,
    Utc::now()
  )
  .execute(pool.get_ref())
  .await
  {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(e) => {
      println!("Failed to insert data for subscribe, error: {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

async fn health_check(_: HttpRequest) -> impl Responder {
	HttpResponse::Ok()
}
 
pub fn run(
  address: TcpListener,
  db_pool: PgPool
) -> Result<Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
        App::new()
          .route("/pow2/{num}", web::get().to(pow2))
          .route("/health_check", web::get().to(health_check))
          .route("/subscribe", web::post().to(subscribe))
          .app_data(db_pool.clone())
      })
    .listen(address)?
    .run();
  Ok(server)
}