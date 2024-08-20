pub mod steps;
pub struct Client {
    client: dagger_sdk::Query,
}

impl Client {
    pub fn new(client: dagger_sdk::Query) -> Self {
        Self { client }
    }
}

impl steps::utils::Utils for Client {
    async fn echo(&self, image: &str, message: &str) -> eyre::Result<(), eyre::Error> {
        let output = self.client
            .container()
            .from(image)
            .with_exec(vec!["echo", message])
            .stdout()
            .await?;

        println!("{}", output);
        Ok(())
    }

    async fn sh(&self, image: &str, script: &str) -> eyre::Result<(), eyre::Error> {
        let output = self.client
            .container()
            .from(image)
            .with_exec(vec!["/bin/sh", "-c", script])
            .stdout()
            .await?;

        println!("{}", output);
        Ok(())
    }
}