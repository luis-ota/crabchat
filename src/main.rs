mod client;

use client::Client;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("0.0.0.0", 8080, "luis", "123").await?;

    client.start_receiver_task().await;

    loop {
        let stdin = BufReader::new(io::stdin());
        let mut lines = stdin.lines();
        if let Ok(Some(line)) = lines.next_line().await {
            client.send(line).await.expect("TODO: panic message");
        }
    }
}
