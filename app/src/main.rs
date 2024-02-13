use anyhow::Context;
use mimir::application::Application;
use mimir::configuration::Settings;
use mimir::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Redirect all `log` events to our subscriber
    telemetry::init_subscriber("mimir", "info", std::io::stdout)?;

    // Panic if we can't read configuration
    let config = Settings::read_from_file().context("Failed to read configuration.")?;
    tracing::info!("Starting app with settings: {:?}", config);

    let application = Application::start(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
