use crate::configuration::{DatabaseSettings, Settings};
use actix_web_lab::middleware::from_fn;
use std::net::TcpListener;
use crate::routes::{login, register_user, get_movies_search, get_movie_by_name, get_movie_by_id};
use crate::middleware::check_token;
use actix_web::{dev::Server, web, App, HttpServer};
use actix_web::{HttpRequest, HttpResponse, Responder};
use sqlx::{postgres::PgPoolOptions, PgPool};

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
        let server = run(listener, connection_pool, config.redis_uri).await?;

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
    num: u32,
}

async fn pow2(req: HttpRequest) -> impl Responder {
    let body: u32 = req.match_info().get("num").unwrap().parse::<u32>().unwrap();

    let res = Power {
        num: u32::pow(body, 2),
    };

    HttpResponse::Ok().json(&res)
}

#[derive(serde::Deserialize)]
struct KeyStruct {
    key: String,
}

pub async fn run(
    addr: TcpListener,
    db_pool: PgPool,
    redis_uri: String,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let redis_client = web::Data::new(redis::Client::open(redis_uri)?);

    let server = HttpServer::new(move || {
        App::new()
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login))
            .service(
                web::scope("")
                    .wrap(from_fn(check_token))
                    //search by 
                    // general-search
                    .route("/movies", web::get().to(get_movies_search))
                    // title-name
                    .route("/movies/title", web::get().to(get_movie_by_name))
                    // id
                    .route("/movies/id", web::get().to(get_movie_by_id))
                    .route("/pow2/{num}", web::get().to(pow2))
            )
            .app_data(db_pool.clone())
            .app_data(redis_client.clone())
    })
    .listen(addr)?
    .run();
    Ok(server)
}
