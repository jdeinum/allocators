use crate::server::Server;
use anyhow::{Context, Result};

pub async fn run() -> Result<()> {
    // set our allocator

    // create the server
    let server = Server::build().await.context("build server")?;

    // run the server to completion
    server
        .run_to_completion()
        .await
        .context("run server to completion")?;

    // idk, analyze stuff?

    Ok(())
}
