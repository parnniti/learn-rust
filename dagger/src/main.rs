use shared::steps;
mod args;
use clap::Parser;
use args::Args;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dagger_sdk::connect(|client| async move {
        let args = Args::parse();
        steps::utils::echo(&client, &args.base_image, &args.port.to_string()).await?;
        steps::utils::echo(&client, &args.base_image, "Fuck you bro").await?;
        Ok(())
    })
    .await?;

    Ok(())
}
