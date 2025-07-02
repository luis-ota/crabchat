use crate::infra::models::{
    AccessRoom, AvailableRoom, CreateRoom, DeleteRoom, ServerMessage, ToJson, User, UserMessage,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::ParseError;

use super::models::LeaveRoom;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("room not found: {0}")]
    RoomNotFound(String),

    #[error("user not found: {0}")]
    UserNotFound(String),

    #[error("invalid uuid: {0}")]
    InvalidUuid(String),

    #[error("you need to pass the User struct fist")]
    Unauthorized,

    #[error("the reciver was not initialized")]
    MissingReceiver,

    #[error("the sender was not initialized")]
    MissingSender,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JsonMessage {
    User(User),
    CreateRoom(CreateRoom),
    DeleteRoom(DeleteRoom),
    AccessRoom(AccessRoom),
    LeaveRoom(LeaveRoom),
    UserMessage(UserMessage),
    AvailableRooms(Vec<AvailableRoom>),
    ServerMessage(ServerMessage),
}
impl ToJson for JsonMessage {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum BroadCastMessage {
    UserMessage(UserMessage),
    ServerMessage(ServerMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum ResType {
    Success,
    ServerError,
    InvalidRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum Action {
    Connect,
    Request,
    CreateRoom,
    DeleteRoom,
    AcessRoom,
    LeaveRoom,
}

pub enum Current {
    Connect,
    Request,
    CreateRoom,
    DeleteRoom,
    AcessRoom,
    LeaveRoom,
}
