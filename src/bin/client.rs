use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub server_port: u16,
}

async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
}
