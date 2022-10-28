use tokio::task::JoinError;
use emale::configuration::get_config;
use emale::issue_delivery_worker::run_worker_until_stopped;
use emale::startup::Application;
use emale::telemetry::{get_tracing_subscriber, init_tracing_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logger
    let subscriber = get_tracing_subscriber("emale".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(subscriber);

    // App
    let config = get_config().expect("Failed to get configuration");
    let app = tokio::spawn(Application::build(config.clone())
		.await?
		.run_until_stopped());
	// background worker
	let worker = tokio::spawn(run_worker_until_stopped(config));

	// build the Future for each of the two long-running tasks - Rust Futures are lazy
	// so nothing happens until they are actually awaited.
	tokio::select! {
		o = app => report_exit("API", o),
		o = worker => report_exit("Background worker", o)
	};
	
    Ok(())
}

fn report_exit(
	task_name: &str,
	outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, JoinError>
) {
	match outcome {
		Ok(Ok(())) => {
			tracing::info!("{}, has exited", task_name)
		}
		Ok(Err(e)) => {
			tracing::error!(
				error.cause_chain = ?e,
				error.message = %e,
				"{} failed",
				task_name
			)
		}
		Err(e) => {
			tracing::error!(
				error.cause_chain = ?e,
				error.message = %e,
				"{} task failed to complete",
				task_name
			)
		}
	}
}
