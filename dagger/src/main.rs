mod steps;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dagger_sdk::connect(|client| async move {
        steps::echo(&client, "What is your name?").await?;
        Ok(())
    })
    .await?;

    Ok(())
}

