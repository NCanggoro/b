use emale::configuration::get_config;
use emale::startup::Application;
use emale::telemetry::{get_tracing_subscriber, init_tracing_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logger
    let subscriber = get_tracing_subscriber("emale".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(subscriber);

    // App
    let config = get_config().expect("Failed to get configuration");
    let app = Application::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
