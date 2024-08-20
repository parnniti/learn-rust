use dagger_sdk::{Query};
use eyre::{Result, Report};

pub async fn echo(client: &Query, message: &str) -> Result<(), Report> {
    let output = client
        .container()
        .from("alpine")
        .with_exec(vec!["echo", message])
        .stdout()
        .await?;

    println!("{}", output);
    Ok(())
}