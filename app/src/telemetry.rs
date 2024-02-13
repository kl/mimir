use anyhow::Context;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init_subscriber<Sink>(name: &str, env_filter: &str, sink: Sink) -> anyhow::Result<()>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    LogTracer::init().context("Failed to set logger")?;

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.to_string(), sink);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).context("Failed to set global default subscriber")
}
