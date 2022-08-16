use emale::configuration::get_config;
use emale::startup::run;
use emale::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logger
    let subscriber = get_tracing_subscriber("emale".into(), "info".into());
    init_tracing_subscriber(subscriber);
    
    // App
    let config = get_config().expect("Failed to get configuration");
    let addr = format!("{}:{}", config.application_host, config.application_port);
    let connection_pool = PgPool::connect(&config.database.connection_strings())
        .await
        .expect("Failed connect to database");

    let listener = TcpListener::bind(addr)?;
    println!(
        "APPLICATION RUNNING ON {}:{}",
        config.application_host, config.application_port
    );
    run(listener, connection_pool)?.await
}
