use crate::infra::enums::ServerError;
use crate::infra::models::{AccessRoom, BaseRoomInfo, CreateRoom, Room, ToJson, User, UserMessage};

use anyhow::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{MaybeTlsStream, connect_async, tungstenite::protocol::Message};
use tungstenite::Utf8Bytes;
use url::Url;

pub struct Client {
    pub ws_stream_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub ws_stream_reciver: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub user: User,
}

impl Client {
    pub async fn new(server: &str, port: i32, user: &User) -> Result<Self, ServerError> {
        let uri = Url::parse(&format!("ws://{server}:{port}"));
        let (ws_stream, _) = connect_async(uri.unwrap().to_string())
            .await
            .map_err(ServerError::WebSocket)?;
        let (ws_stream_sender, ws_stream_reciver) = ws_stream.split();
        println!("Connected to server succefully!");
        Ok(Self {
            ws_stream_sender,
            ws_stream_reciver: Some(ws_stream_reciver),
            user: user.to_owned(),
        })
    }

    async fn send_message(
        &mut self,
        message: &String,
        room_code: &String,
    ) -> Result<(), ServerError> {
        let user_message = UserMessage::new(&self.user, message, &room_code.to_string())?;
        self.ws_stream_sender
            .send(Message::Text(Utf8Bytes::from(user_message.to_json()?)))
            .map_err(ServerError::WebSocket)
            .await?;
        Ok(())
    }

    async fn send_method(&mut self, json: &String) -> Result<(), ServerError> {
        self.ws_stream_sender
            .send(Message::Text(Utf8Bytes::from(json)))
            .await
            .map_err(ServerError::WebSocket)?;
        Ok(())
    }

    async fn acess_room(
        &mut self,
        room_code: String,
        password: Option<String>,
    ) -> Result<(), ServerError> {
        let access_room = AccessRoom {
            room_code: room_code.to_string(),
            password,
        };
        self.send_method(&access_room.to_json()?).await?;

        Ok(())
    }

    async fn create_room(
        &mut self,
        base_info: &BaseRoomInfo,
        password: &String,
        public: &bool,
    ) -> Result<(), ServerError> {
        let create_room = CreateRoom {
            base_info: base_info.to_owned(),
            password: Some(password.to_string()),
            public: *public,
        };

        self.send_method(&create_room.to_json()?).await?;

        Ok(())
    }
}
