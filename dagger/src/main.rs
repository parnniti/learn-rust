mod steps;
mod configuration;
use clap::Parser;
use configuration::Configuration;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dagger_sdk::connect(|client| async move {
        let Configuration { port } = Configuration::parse();
        steps::echo(&client, &port.to_string()).await?;
        Ok(())
    })
    .await?;

    Ok(())
}

