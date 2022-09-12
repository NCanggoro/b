use crate::{routes::{health_check, subscribe}, configuration::Settings};
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use crate::email_client::EmailClient;
use crate::configuration::DatabaseSettings;
use actix_web::dev::{Server};
use serde::{Serialize};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Serialize)]
struct Power {
  num: u32
}

pub struct Application {
  port: u16,
  server: Server
}

impl Application {
  pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
    let connection_pool = get_connection_pool(&config.database);
    
    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address");
    
    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
      config.email_client.base_url, 
      sender_email, 
      config.email_client.authorization_token,
      timeout
    );
  
    let address = format!(
      "{}:{}",
      config.application.host, config.application.port
    );
  
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection_pool, email_client)?;

    Ok(Self { port, server })
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
    self.server.await
  }
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

pub fn get_connection_pool(
  config: &DatabaseSettings
) -> PgPool {
  PgPoolOptions::new()
      .connect_timeout(std::time::Duration::from_secs(2))
      .connect_lazy_with(config.with_db())
}

pub fn run(
  address: TcpListener,
  db_pool: PgPool,
  email_client: EmailClient
) -> Result<Server, std::io::Error> {
  let db_pool = web::Data::new(db_pool);
  let server = HttpServer::new(move || {
        App::new()
          // middlewares
          .wrap(TracingLogger::default())
          // routes
          .route("/pow2/{num}", web::get().to(pow2))
          .route("/health_check", web::get().to(health_check))
          .route("/subscribe", web::post().to(subscribe))
          // database
          .app_data(db_pool.clone())
          .app_data(email_client.clone())
      })
    .listen(address)?
    .run();
  Ok(server)
}