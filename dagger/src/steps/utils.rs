use eyre::{Result, Error};

#[trait_variant::make(Send)]
pub trait Utils {
    async fn echo(&self, image: &str, message: &str) -> Result<(), Error>;
    async fn sh(&self, image: &str, script: &str) -> Result<(), Error>;
}
