use poll::{app::Application, conf::ConfigurationBuilder, telemetry};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = ConfigurationBuilder::default().build()?;
    telemetry::setup_tracing(&configuration)?;
    let app = Application::new(&configuration)?;
    app.run().await?;
    Ok(())
}
