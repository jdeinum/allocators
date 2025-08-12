use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // run our app to completion
    allocators::run::run()
        .await
        .context("run server to completion")?;

    Ok(())
}
