use crate::infra::enums::{Action, ResType};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::enums::ServerError;

pub trait ToJson {
    fn to_json(&self) -> Result<String, ServerError>
    where
        Self: Serialize,
    {
        serde_json::to_string(self).map_err(ServerError::Serialization)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct User {
    pub name: String,
    pub uuid: String,
}
impl ToJson for User {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Server {
    pub addres: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserMessage {
    pub user: Option<User>,
    pub message: String,
    pub datetime: String,
    pub room_code: String,
}

impl UserMessage {
    pub fn default() -> Self {
        Self {
            user: Some(User { name:"luis".to_string(), uuid: String::new()}.to_owned()),
            message: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry".to_string(),
            datetime: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            room_code: String::new(),
        }
    }
    pub fn new(user: &User, message: &String, room_code: &String) -> Self {
        Self {
            user: Some(User { name:"luis".to_string(), uuid: String::new()}.to_owned()),
            message: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry".to_string(),
            datetime: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            room_code: String::new(),
        }
    }
}

impl ToJson for UserMessage {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BaseRoomInfo {
    pub code: String,
    pub name: String,
    pub created_by: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateRoom {
    pub base_info: BaseRoomInfo,
    pub password: Option<String>,
    pub public: bool,
}
impl ToJson for CreateRoom {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoom {
    pub code: String,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRoom {
    pub room_code: String,
    pub password: Option<String>,
}

impl ToJson for AccessRoom {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRoom {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Room {
    pub info: CreateRoom,
    pub messages: Vec<UserMessage>,
    pub users: HashMap<String, User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AvaliableRoom {
    pub info: CreateRoom,
    pub users_count: u64,
    pub has_password: bool,
}

impl Room {
    pub fn public_info(&self) -> CreateRoom {
        CreateRoom {
            base_info: self.info.base_info.clone(),
            password: None,
            public: self.info.public,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomInfo {
    pub base_info: BaseRoomInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessage {
    pub for_action: Action,
    pub res_type: ResType,
    pub message: String,
}

impl ToJson for ServerMessage {}
