use crate::authentication::middleware::reject_users;
use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::{
    admin_dashboard, confirm, health_check, home, login, 
    login_form, subscribe, change_password, change_password_form,
    logout, publish_newsletter, publish_newsletter_form
};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

#[derive(Serialize)]
struct Power {
    num: u32,
}

pub struct Application {
    port: u16,
    server: Server,
}

// define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.
// Retrieval from the context, in actix-web, is type-based: using
// a raw `String` would expose to conflicts.
pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&config.database);
        let email_client = config.email_client.client();
        let address = format!("{}:{}", config.application.host, config.application.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            config.application.base_url,
            config.application.hmac_secret,
            config.redis_uri,
        )
        .await?;

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
    let body: u32 = req.match_info().get("num").unwrap().parse::<u32>().unwrap();
    let res = Power {
        num: u32::pow(body, 2),
    };

    HttpResponse::Ok().json(&res)
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}

pub async fn run(
    address: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let server = HttpServer::new(move || {
        App::new()
            // middlewares
            .wrap(TracingLogger::default())
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            // routes
            .route("/", web::get().to(home))
            .route("/pow2/{num}", web::get().to(pow2))
            .route("/health_check", web::get().to(health_check))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .route("/subscribe", web::post().to(subscribe))
            .route("/subscribe/confirm", web::get().to(confirm))
			.service(
				web::scope("/admin")
					.wrap(from_fn(reject_users))
					.route("/dashboard", web::get().to(admin_dashboard))
          .route("/newsletters", web::get().to(publish_newsletter_form))
          .route("/newsletters", web::post().to(publish_newsletter))
					.route("/password", web::get().to(change_password_form))
					.route("/password", web::post().to(change_password))
					.route("/logout", web::post().to(logout)),
			)
            // app pool
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(address)?
    .run();
    Ok(server)
}
