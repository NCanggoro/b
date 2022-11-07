use omdb::startup::Application;
use omdb::configuration::get_config;
use omdb::telemetry::{get_tracing_subscriber, init_tracing_subscriber};

#[tokio::main]
async fn main () -> anyhow::Result<()> {
    let tracing_subscriber = get_tracing_subscriber("ombd".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(tracing_subscriber);
    
    let config = get_config().expect("Failed to get configuration");
    
    let app = Application::build(config.clone()).await?;
    
    println!("APPLICATION RUNNING ON {}:{}", config.application.host, config.application.port);

    app.run_until_stopped().await?;

    Ok(())

}