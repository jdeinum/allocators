use anyhow::{Context, Result};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

pub struct ClientSettings {
    pub port: u16,
    pub num_messages: usize,
}

pub struct Client {
    pub listener: TcpStream,
}

impl Client {
    pub async fn build(server_port: u16) -> Result<Self> {
        // connect to the endpoint, hardcoded to localhost just for simplicity
        let endpoint = format!("localhost:{server_port}");
        let listener = TcpStream::connect(&endpoint)
            .await
            .context("connect to server")?;

        Ok(Self { listener })
    }

    pub async fn run(mut self, num_messages: usize) -> Result<()> {
        // send the messages
        for i in 0..num_messages {}

        // after we've sent all of our messages, we expect to receive our results
        // self.listener.read_exact()
    }
}
