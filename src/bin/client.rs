use allocators::client::Client;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// The port the server is listening on
    #[arg(short = 'p', long, value_parser=clap::value_parser!(u16).range(1024..),)]
    pub server_port: u16,

    /// The number of messages the client sends before expecting a response back
    pub num_messages: usize,

    /// Number of seconds between messages
    /// Needs to be <5 because the server is hardcoded to wait a max of 5 seconds :D
    #[arg(short, long, value_parser=clap::value_parser!(u16).range(1..5),)]
    pub secs_between_messages: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    let client = Client::build(cli.server_port)
        .await
        .context("build client")?;

    client
        .run_to_completion(cli.num_messages, cli.secs_between_messages)
        .await
        .context("run client")
}
