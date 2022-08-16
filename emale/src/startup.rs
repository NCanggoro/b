use crate::routes::{health_check, subscribe};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::{Server};
use serde::{Serialize};
use std::net::TcpListener;
use sqlx::PgPool;

#[derive(Serialize)]
struct Power {
  num: u32
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

pub fn run(
  address: TcpListener,
  db_pool: PgPool
) -> Result<Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
        App::new()
          // middlewares
          .wrap(Logger::default())
          // routes
          .route("/pow2/{num}", web::get().to(pow2))
          .route("/health_check", web::get().to(health_check))
          .route("/subscribe", web::post().to(subscribe))
          // database
          .app_data(db_pool.clone())
      })
    .listen(address)?
    .run();
  Ok(server)
}