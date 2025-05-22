use tokio::net::{TcpStream};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream};
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Utf8Bytes;
use url::Url;

pub struct Client {
    pub ws_stream_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub ws_stream_reciver: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub user: String,
    pub room: String,
    pub history: Vec<UserMessage>,
}

pub struct UserMessage{
    pub user: String,
    pub datetime: String,
    pub message: Message,  
}

impl Client {
    pub async fn new(server: &str, port: i32, user: &str, room: &str) -> Result<Self> {
        let uri = Url::parse(&format!("ws://{server}:{port}"));
        let (ws_stream, _) = connect_async(uri.unwrap().to_string()).await?;
        let (ws_stream_sender, ws_stream_reciver) = ws_stream.split();
        Ok(Self {
            ws_stream_sender,
            ws_stream_reciver: Some(ws_stream_reciver),
            user: String::from(user),
            room: String::from(room),
            history: Vec::new(),       
        })
    }

    pub async fn send(&mut self, message: String) -> Result<()> {
        self.ws_stream_sender.send(Message::Text(Utf8Bytes::from(message))).await?;
        Ok(())
    }

    pub async fn start_receiver_task(&mut self){
        let mut recv = self.ws_stream_reciver.take().expect("Already Taken");
        
        tokio::spawn(async move {
            while let Some(Ok(message)) = recv.next().await {
                match message {
                    Message::Text(text) => println!("Received: {}", text),
                    Message::Close(_) => {
                        println!("Connection closed by server");
                        break;
                    }
                    _ => {} // Ignore other message types
                }
            }
        });
        
    }
}
