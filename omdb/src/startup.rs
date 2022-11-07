use std::net::TcpListener;
use crate::configuration::{DatabaseSettings, Settings};
// use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{HttpRequest, Responder, HttpResponse};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::routes::{login, register_user};

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(String);

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&config.database);
        let addr = format!("{}:{}", config.application.host, config.application.port);

        let listener = TcpListener::bind(addr)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool, 
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}

#[derive(serde::Serialize)]
struct Power {
    num: u32
}

async fn pow2(req: HttpRequest) -> impl Responder {

    let body: u32 = req
        .match_info()
        .get("num")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let res = Power {
        num: u32::pow(body, 2),
    };

    HttpResponse::Ok().json(&res)
}

pub fn run (
    addr: TcpListener,
    db_pool: PgPool
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    
    let server = HttpServer::new(move || {
        App::new()
          .route("/pow2/{num}", web::get().to(pow2))
          .route("/", web::get().to(login))
          .route("/register", web::post().to(register_user))
          .route("/login", web::post().to(login))
          .app_data(db_pool.clone())
      })
    .listen(addr)?
    .run();
  Ok(server)
}
