#[derive(clap::Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(env, short, long, default_value = "3000")]
    pub port: u16,
    #[arg(env, short, long, default_value = "alpine")]
    pub base_image: String,
}
