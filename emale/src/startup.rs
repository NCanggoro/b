use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::{
    confirm, 
	health_check, 
	home, 
	login, 
	login_form, 
	publish_newsletter, 
	subscribe,
};
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{
	web, 
	App, 
	HttpRequest, 
	HttpResponse, 
	HttpServer, 
	Responder
};
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::CookieMessageStore;
use secrecy::{Secret, ExposeSecret};
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
            timeout,
        );

        let address = format!("{}:{}", config.application.host, config.application.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            config.application.base_url,
            config.application.hmac_secret
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

pub fn run(
    address: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let message_store = CookieMessageStore::builder(
        Key::from(hmac_secret.expose_secret().as_bytes())
    ).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let server = HttpServer::new(move || {
        App::new()
            // middlewares
            .wrap(TracingLogger::default())
            .wrap(message_framework.clone())
            // routes
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .route("/pow2/{num}", web::get().to(pow2))
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .route("/subscribe/confirm", web::get().to(confirm))
            .route("/newsletters", web::post().to(publish_newsletter))
            // app pool
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(address)?
    .run();
    Ok(server)
}