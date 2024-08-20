use dagger_sdk::Query;
use eyre::{Result, Report};

pub async fn echo(client: &Query, image: &String, message: &str) -> Result<(), Report> {
    let output = client
        .container()
        .from(image)
        .with_exec(vec!["echo", message])
        .stdout()
        .await?;

    println!("{}", output);
    Ok(())
}
