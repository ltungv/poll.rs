use clap::Parser;
use poll::{
    app::{Application, Cli},
    conf::ConfigurationBuilder,
    telemetry,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut config_builder = ConfigurationBuilder::default();

    let cli = Cli::parse();
    if let Some(config) = cli.config() {
        config_builder.config_directory(config);
    }

    let configuration = config_builder.build()?;
    match cli.migrate() {
        Some(migrate) => {
            poll::app::migrate(migrate, &configuration).await?;
        }
        None => {
            telemetry::setup_tracing(&configuration)?;
            let app = Application::new(&configuration)?;
            app.run().await?;
        }
    }
    Ok(())
}
