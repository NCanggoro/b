use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::{Server};
use serde::Serialize;
use std::net::TcpListener;

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

async fn health_check(_: HttpRequest) -> impl Responder {
	HttpResponse::Ok()
}
 
pub fn run(address: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
        App::new()
          .route("/pow2/{num}", web::get().to(pow2))
          .route("/health_check", web::get().to(health_check))
      })
    .listen(address)?
    .run();
  Ok(server)
}