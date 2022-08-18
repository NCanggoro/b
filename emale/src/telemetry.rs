use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_tracing_subscriber<Sink>(
    name: String,
    env_fliter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
// a higher-ranked trait bound (HRTB)
// It basically means that Sink implements the `MakeWriter`
// trait for all choices of the lifetime parameter `'a`
// https://doc.rust-lang.org/nomicon/hrtb.html
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_fliter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_tracing_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to subcsribe log");
}
