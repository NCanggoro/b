use std::net::TcpListener;

use actix_web::{get, HttpServer, web, App};
use omdb::{configuration::get_config, startup::Application, telemetry::{get_tracing_subscriber, init_tracing_subscriber}};
use sqlx::{PgPool, postgres::PgPoolOptions};
use omdb::startup::run;
// use omdb::startup::Application;

#[tokio::main]
async fn main () -> std::io::Result<()> {
    let tracing_subscriber = get_tracing_subscriber("ombd".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(tracing_subscriber);
    
    let config = get_config().expect("Failed to get configuration");
    
    let app = Application::build(config.clone()).await?;
    
    println!("APPLICATION RUNNING ON {}:{}", config.application.host, config.application.port);

    app.run_until_stopped().await?;

    Ok(())

}