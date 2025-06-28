use crate::infra::enums::{JsonMessage, ResType, ServerError};
use crate::infra::models::{
    AccessRoom, AvaliableRoom, BaseRoomInfo, CreateRoom, Room, ToJson, User, UserMessage,
};
use dioxus_toast::{ToastInfo, ToastManager};

use anyhow::Result;
use dioxus::prelude::*;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{MaybeTlsStream, connect_async};
use tracing::{error, info, warn};
use tungstenite::{Message, Utf8Bytes};

use url::Url;

#[derive(Default, Debug)]
pub struct Client {
    pub ws_stream_sender: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    pub ws_stream_reciver: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub user: User,
}

impl Client {
    pub async fn new(server: &str, user: &User) -> Result<Self, ServerError> {
        let uri = Url::parse(&format!("ws://{server}"));
        let (ws_stream, _) = connect_async(uri.unwrap().to_string())
            .await
            .map_err(ServerError::WebSocket)?;
        let (ws_stream_sender, ws_stream_reciver) = ws_stream.split();
        println!("Connected to server succefully!");
        Ok(Self {
            ws_stream_sender: Some(ws_stream_sender),
            ws_stream_reciver: Some(ws_stream_reciver),
            user: user.to_owned(),
        })
    }

    pub async fn start_recive_task(
        &mut self,
        mut avaliable_rooms: Signal<Vec<AvaliableRoom>>,
        mut current_room: Signal<Room>,
        mut toast: Signal<ToastManager>,
    ) -> Result<Task, ServerError> {
        let mut ws_reciver = self
            .ws_stream_reciver
            .take()
            .ok_or(ServerError::MissingReceiver)?;

        let handle = spawn(async move {
            while let Some(msg) = ws_reciver.next().await {
                if let Ok(Message::Text(text)) = msg {
                    info!("recived {:#?} from server", &text);

                    match serde_json::from_str::<JsonMessage>(&text) {
                        Ok(incoming_message) => match incoming_message {
                            JsonMessage::AvailableRooms(a_rooms) => {
                                avaliable_rooms.set(a_rooms);
                            }
                            JsonMessage::UserMessage(u_message) => {
                                current_room.write().messages.push(u_message);
                            }
                            JsonMessage::ServerMessage(s_message) => {
                                toast.write().popup(ToastInfo {
                                    heading: Some(format!(
                                        "{:#?} {:#?}",
                                        s_message.for_action, s_message.res_type
                                    )),
                                    context: s_message.message,
                                    allow_toast_close: true,
                                    position: dioxus_toast::Position::TopRight,
                                    icon: None,
                                    hide_after: Some(7),
                                });
                            }
                            _ => {
                                warn!("Received unexpected message from server {:#?}", text);
                            }
                        },
                        Err(e) => {
                            error!("erro ao desserializar a mensagem: {}. texto: {}", e, text);
                        }
                    }
                }
            }
        });

        Ok(handle)
    }

    async fn send_message(
        &mut self,
        message: &String,
        room_code: &String,
    ) -> Result<(), ServerError> {
        let user_message = UserMessage::new(&self.user, message, &room_code.to_string());
        let ws_sender = self.ws_stream_sender.as_mut().unwrap();
        ws_sender
            .send(Message::Text(Utf8Bytes::from(user_message.to_json()?)))
            .map_err(ServerError::WebSocket)
            .await?;
        Ok(())
    }

    pub async fn send_method(&mut self, json: &String) -> Result<(), ServerError> {
        let ws_sender = self.ws_stream_sender.as_mut().unwrap();

        ws_sender
            .send(Message::Text(Utf8Bytes::from(json)))
            .await
            .map_err(ServerError::WebSocket)?;

        info!("{:#?} was sent to the server", json);
        Ok(())
    }

    pub async fn login(&mut self, user: &User) -> Result<(), ServerError> {
        let user_json = JsonMessage::User(user.clone());
        self.send_method(&user_json.to_json()?).await?;
        Ok(())
    }

    pub async fn access_room(
        &mut self,
        room_code: String,
        password: Option<String>,
    ) -> Result<(), ServerError> {
        let access_room = AccessRoom {
            room_code: room_code.to_string(),
            password,
        };
        // info!("{:#?}", access_room);
        self.send_method(&JsonMessage::AcessRoom(access_room).to_json()?)
            .await?;

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

    // async fn leave_room(&mut self) -> Result<(), ServerError> {
    //     let leave_room = LeaveRoom {
    //         room_code: self.room_code.clone(),
    //     };
    //     self.send_method(&leave_room.to_json()?).await?;

    //     Ok(())
    // }
}
