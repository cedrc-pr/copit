use std::{collections::HashMap, sync::Arc};

use redis::AsyncTypedCommands;
use tokio::sync::{RwLock, mpsc::UnboundedSender};

use crate::{
    adapters::{
        postgres::messages::Message,
        redis::{RedisAdapter, RedisAdapterError},
    },
    tools::ws_server_message::WsServerMessage,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WsRedisEvent {
    Message { room_id: String, message: Message },
    Connected { room_id: String, except: uuid::Uuid },
    Disconnected { room_id: String },
    Typing { room_id: String, status: bool },
}

impl RedisAdapter {
    pub fn subscribe_ws_events(
        &self,
        conns_map: Arc<RwLock<HashMap<uuid::Uuid, UnboundedSender<WsServerMessage>>>>,
    ) {
        let redis_adapter = self.clone();
        tokio::spawn(async move {
            use futures_util::StreamExt;
            let mut pubsub = match redis_adapter.client.get_async_pubsub().await {
                Ok(pubsub) => pubsub,
                Err(e) => {
                    tracing::error!(%e, "could not get pubsub");
                    return;
                }
            };
            if let Err(e) = pubsub.subscribe("ws:events").await {
                tracing::error!(%e, "could not subscribe");
            };
            let mut stream = pubsub.on_message();
            tracing::info!("subscribe on ws:events");
            while let Some(msg) = stream.next().await {
                let payload: String = match msg.get_payload() {
                    Ok(payload) => payload,
                    Err(e) => {
                        tracing::error!(%e, "could not get ws:events payload");
                        return;
                    }
                };
                let event: WsRedisEvent = match serde_json::from_str(&payload) {
                    Ok(event) => event,
                    Err(e) => {
                        tracing::error!(%e, "could not parse ws:events payload");
                        return;
                    }
                };
                tracing::debug!("received redis ws:events");
                match event {
                    WsRedisEvent::Message { room_id, message } => {
                        let members = match redis_adapter.room_members(&room_id).await {
                            Ok(members) => members,
                            Err(e) => {
                                tracing::error!(%e, "could not get members of room");
                                return;
                            }
                        };
                        let to_send = WsServerMessage::Message { message };
                        for id in members {
                            if let Some(sender) = conns_map.write().await.get(&id) {
                                if let Err(e) = sender.send(to_send.clone()) {
                                    tracing::error!(%e, "could not send message");
                                }
                                tracing::trace!(%id, "message send");
                            }
                        }
                    }
                    WsRedisEvent::Connected { room_id, except } => {
                        let members = match redis_adapter.room_members(&room_id).await {
                            Ok(members) => members,
                            Err(e) => {
                                tracing::error!(%e, "could not get members of room");
                                return;
                            }
                        };
                        for id in members {
                            if id != except
                                && let Some(sender) = conns_map.write().await.get(&id)
                            {
                                if let Err(e) = sender.send(WsServerMessage::Connected) {
                                    tracing::error!(%e, "could not send connected");
                                }
                                tracing::trace!(%id, "connected send");
                            }
                        }
                    }
                    WsRedisEvent::Disconnected { room_id } => {
                        let members = match redis_adapter.room_members(&room_id).await {
                            Ok(members) => members,
                            Err(e) => {
                                tracing::error!(%e, "could not get members of room");
                                return;
                            }
                        };
                        for id in members {
                            if let Some(sender) = conns_map.write().await.get(&id) {
                                if let Err(e) = sender.send(WsServerMessage::Connected) {
                                    tracing::error!(%e, "could not send disconnected");
                                }
                                tracing::trace!(%id, "disconnected send");
                            }
                        }
                    }
                    WsRedisEvent::Typing { room_id, status } => {
                        let members = match redis_adapter.room_members(&room_id).await {
                            Ok(members) => members,
                            Err(e) => {
                                tracing::error!(%e, "could not get members of room");
                                return;
                            }
                        };
                        for id in members {
                            if let Some(sender) = conns_map.write().await.get(&id) {
                                if let Err(e) = sender.send(WsServerMessage::Typing { status }) {
                                    tracing::error!(%e, "could not send typing");
                                }
                                tracing::trace!(%id, "typing send");
                            }
                        }
                    }
                };
            }
        });
    }

    pub async fn publish_ws_event(&self, event: &WsRedisEvent) -> Result<(), RedisAdapterError> {
        let payload = serde_json::to_string(event)?;
        self.conn().publish("ws:events", payload).await?;
        Ok(())
    }
}
