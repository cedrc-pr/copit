use crate::{
    adapters::redis::RedisAdapter,
    tools::ws_server_message::{WsServerMessage, WsServerMessageErrorCode},
};
use redis::AsyncCommands;
use tokio::sync::mpsc::UnboundedSender;

impl RedisAdapter {
    pub async fn join_room(&self, room: &str, conn_id: uuid::Uuid) -> redis::RedisResult<()> {
        let _ = self.remove_from_room(conn_id).await;
        let _: redis::RedisResult<()> = self
            .conn()
            .sadd(format!("room:{room}"), conn_id.to_string())
            .await;
        self.conn().set(format!("connection:{conn_id}"), room).await
    }

    pub async fn leave_room(&self, conn_id: uuid::Uuid) -> redis::RedisResult<()> {
        let _ = self.remove_from_room(conn_id).await;
        self.conn().del(format!("connection:{conn_id}")).await
    }

    pub async fn room_members(&self, room: &str) -> redis::RedisResult<Vec<uuid::Uuid>> {
        let members: Vec<String> = self.conn().smembers(format!("room:{room}")).await?;

        let uuids = members
            .into_iter()
            .filter_map(|s| uuid::Uuid::parse_str(&s).ok())
            .collect();

        Ok(uuids)
    }

    pub async fn remove_from_room(&self, conn_id: uuid::Uuid) -> redis::RedisResult<()> {
        if let Ok(Some(old_room)) = self.room_of_conn(conn_id).await {
            let _: redis::RedisResult<()> = self
                .conn()
                .srem(format!("room:{old_room}"), conn_id.to_string())
                .await;
        }
        Ok(())
    }

    pub async fn room_of_conn(&self, conn_id: uuid::Uuid) -> redis::RedisResult<Option<String>> {
        self.conn().get(format!("connection:{conn_id}")).await
    }

    pub async fn get_room_id(
        &self,
        conn_tx: &UnboundedSender<WsServerMessage>,
        conn_id: uuid::Uuid,
    ) -> Option<String> {
        let current_room = match self.room_of_conn(conn_id).await {
            Ok(Some(r)) => r,
            Ok(None) => {
                tracing::warn!("not in a room");
                let _ = conn_tx.send(WsServerMessage::Error {
                    code: WsServerMessageErrorCode::NotInARoom,
                });
                return None;
            }
            Err(e) => {
                tracing::error!(%e, "redis error");
                let _ = conn_tx.send(WsServerMessage::Error {
                    code: WsServerMessageErrorCode::Unexpected,
                });
                return None;
            }
        };
        Some(current_room)
    }
}
