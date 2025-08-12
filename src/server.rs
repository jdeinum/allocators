use crate::messages::Message;
use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

pub struct ServerSettings {
    pub port: u16,
}

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    pub async fn build(settings: ServerSettings) -> Result<Self> {
        // create a socket on the port
        let addr = format!("localhost:{}", settings.port);
        let listener = TcpListener::bind(&addr).await.context("create listener")?;
        Ok(Self { listener })
    }

    pub async fn run_to_completion(self) -> Result<()> {
        loop {
            // accept the connection
            println!("Waiting for connection");
            let (socket, addr) = self.listener.accept().await?;
            println!("Accepted connection from {addr:?}");
            let (reader, mut writer) = tokio::io::split(socket);
            let reader = BufReader::new(reader);
            let mut lines = reader.lines();
            let mut i = 0;
            let mut sum = 0;

            while let Ok(Some(line)) = lines.next_line().await {
                let o: Message = serde_json::de::from_str(&line).context("parse message")?;
                println!("Got message {i} from {} with value {}", o.id, o.value);

                // add to the sum
                sum += o.value;

                // received 10 messages, send back sum
                i += 1;
                if i == 9 {
                    println!("Last message, sending sum to {}", o.id);
                    let id = o.id;
                    let msg = Message { id, value: sum };
                    let json_msg = serde_json::to_vec(&msg).context("sum to bytes")?;
                    writer.write_all(&json_msg).await.context("send sum")?;
                    break;
                }
            }

            println!("Connection closed from {addr:?}");
        }
    }
}
