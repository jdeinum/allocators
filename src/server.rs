use anyhow::Result;
use tokio::net::TcpListener;

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    pub async fn build() -> Result<Self> {
        todo!()
    }

    pub async fn run_to_completion(self) -> Result<()> {
        todo!()
    }
}
