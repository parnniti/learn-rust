use shared::Client;
use shared::steps::utils::Utils;
mod args;
use clap::Parser;
use args::Args;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dagger_sdk::connect(|client| async move {
        let args = Args::parse();
        let client = Client::new(client);
        client.echo(&args.base_image, "Test").await?;
        Ok(())
    })
    .await?;

    Ok(())
}
