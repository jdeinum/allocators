use crate::server::Server;
use anyhow::{Context, Result};

pub async fn run() -> Result<()> {
    // set our allocator

    // create the server
    println!("Building server...");
    let server = Server::build(crate::server::ServerSettings { port: 8080 })
        .await
        .context("build server")?;

    // run the server to completion
    println!("Running to completion...");
    server
        .run_to_completion()
        .await
        .context("run server to completion")?;

    // idk, analyze stuff?

    Ok(())
}
