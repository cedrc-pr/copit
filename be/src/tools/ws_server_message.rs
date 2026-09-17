use crate::adapters::postgres::messages::Message;

#[derive(serde::Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum WsServerMessage {
    #[serde(rename = "message")]
    Message { message: Message },
    #[serde(rename = "error")]
    Error { code: WsServerMessageErrorCode },
    #[serde(rename = "connected_users")]
    ConnectedUsers { count: usize },
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "typing")]
    Typing { status: bool },
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WsServerMessageErrorCode {
    RoomNotFound,
    NotInARoom,
    InvalidMessage,
    Unexpected,
}
