use shared::steps;
mod configuration;
use clap::Parser;
use configuration::Configuration;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dagger_sdk::connect(|client| async move {
        let Configuration { port, base_image } = Configuration::parse();
        steps::utils::echo(&client, &base_image, &port.to_string()).await?;
        steps::utils::echo(&client, &base_image, "Fuck you bro").await?;
        Ok(())
    })
    .await?;

    Ok(())
}
