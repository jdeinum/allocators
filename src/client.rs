use crate::messages::Message;
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct ClientSettings {
    pub server_port: u16,
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

    pub async fn run_to_completion(
        mut self,
        num_messages: usize,
        secs_between_messages: u64,
    ) -> Result<()> {
        // send the messages
        for i in 0..num_messages {
            println!("sending message {i}");

            // get message
            let message = Message::default();

            // serialize the message
            let s_message = serde_json::to_string(&message).context("convert struct to json")?;

            // send the message, not expecting any response
            self.listener
                .write_all(&s_message.as_bytes())
                .await
                .context("send message")?;

            // wait for a little
            tokio::time::sleep(Duration::from_secs(secs_between_messages)).await;
        }

        // after we've sent all of our messages, we expect to receive our results
        let mut buf: Vec<u8> = Vec::new();
        self.listener
            .read_to_end(&mut buf)
            .await
            .context("receive final count")?;

        // convert back to a struct
        let final_count: Message =
            serde_json::from_slice(&buf).context("deserialize final message from string")?;

        println!("Final count for {}: {}", final_count.id, final_count.value);
        Ok(())
    }
}
