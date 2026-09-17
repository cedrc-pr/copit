use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;
use validator::Validate;

use crate::{
    adapters::{
        postgres::{PostgresAdapter, messages::MessageDto},
        redis::{RedisAdapter, events::WsRedisEvent},
    },
    routes::Normalize,
    tools::ws_server_message::{WsServerMessage, WsServerMessageErrorCode},
};

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "action", content = "data")]
pub enum WsClientMessage {
    #[serde(rename = "join-room")]
    JoinRoom { id: String },
    #[serde(rename = "send-message")]
    SendMessage(MessageDto),
    #[serde(rename = "typing")]
    Typing { status: bool },
}

impl WsClientMessage {
    pub async fn process(
        self,
        conn_id: uuid::Uuid,
        conn_tx: &UnboundedSender<WsServerMessage>,
        redis_adapter: &Arc<RedisAdapter>,
        postgres_adapter: &Arc<PostgresAdapter>,
    ) {
        match self {
            Self::JoinRoom { id } => {
                if postgres_adapter.room_get(&id).await.is_err() {
                    tracing::warn!(%id, "no room with this id");
                    let _ = conn_tx.send(WsServerMessage::Error {
                        code: WsServerMessageErrorCode::RoomNotFound,
                    });
                    return;
                }
                let _ = redis_adapter.join_room(&id, conn_id).await;
                tracing::info!(%conn_id, %id, "client join room");
                if let Ok(members) = redis_adapter.room_members(&id).await {
                    let _ = conn_tx.send(WsServerMessage::ConnectedUsers {
                        count: members.len(),
                    });
                    let _ = redis_adapter
                        .publish_ws_event(&WsRedisEvent::Connected {
                            room_id: id,
                            except: conn_id,
                        })
                        .await;
                }
            }

            Self::SendMessage(mut dto) => {
                let current_room = match redis_adapter.get_room_id(conn_tx, conn_id).await {
                    Some(current_room) => current_room,
                    None => return,
                };

                if let Err(e) = dto.normalize().validate() {
                    let _ = conn_tx.send(WsServerMessage::Error {
                        code: WsServerMessageErrorCode::InvalidMessage,
                    });
                    tracing::warn!(%e, "invalid message dto");
                    return;
                }

                match postgres_adapter
                    .message_add(dto, current_room.clone())
                    .await
                {
                    Ok(saved_message) => {
                        let event = WsRedisEvent::Message {
                            room_id: current_room,
                            message: saved_message,
                        };
                        if let Err(e) = redis_adapter.publish_ws_event(&event).await {
                            tracing::error!(%e, "failed to publish ws:events");
                        }
                        tracing::debug!("published redis ws:events");
                    }
                    Err(e) => {
                        tracing::error!(%e, "failed to save message");
                    }
                }
            }

            Self::Typing { status } => {
                if let Some(current_room) = redis_adapter.get_room_id(conn_tx, conn_id).await
                    && let Err(e) = redis_adapter
                        .publish_ws_event(&WsRedisEvent::Typing {
                            room_id: current_room,
                            status,
                        })
                        .await
                {
                    tracing::error!(%e, "redis error");
                    let _ = conn_tx.send(WsServerMessage::Error {
                        code: WsServerMessageErrorCode::Unexpected,
                    });
                }
            }
        }
    }
}
