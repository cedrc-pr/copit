use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use poem::{
    IntoResponse, Route, get,
    web::{
        Data,
        websocket::{Message as WsMessage, WebSocket},
    },
};
use tokio::sync::RwLock;

use crate::{
    adapters::{
        postgres::PostgresAdapter,
        redis::{RedisAdapter, events::WsRedisEvent},
    },
    app_error::AppError,
    tools::{
        ws_client_message::WsClientMessage,
        ws_server_message::{WsServerMessage, WsServerMessageErrorCode},
    },
};

pub fn ws_routes() -> Route {
    Route::new().at("/chat", get(chat))
}

#[poem::handler]
pub async fn chat(
    ws: WebSocket,
    Data(redis_adapter): Data<&Arc<RedisAdapter>>,
    Data(postgres_adapter): Data<&Arc<PostgresAdapter>>,
    Data(conns): Data<
        &Arc<RwLock<HashMap<uuid::Uuid, tokio::sync::mpsc::UnboundedSender<WsServerMessage>>>>,
    >,
) -> Result<impl IntoResponse, AppError> {
    let connection_id = uuid::Uuid::new_v4();

    Ok(ws.on_upgrade({
        let redis = redis_adapter.clone();
        let postgres = postgres_adapter.clone();
        let conns_map = conns.clone();
        let conn_id = connection_id.clone();

        move |socket| async move {
            let (mut ws_sink, mut ws_stream) = socket.split();
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<WsServerMessage>();
            let local_tx = tx.clone();

            conns_map.write().await.insert(conn_id.clone(), tx);

            let send_task = tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if let Ok(json) = serde_json::to_string(&msg) {
                        if ws_sink.send(WsMessage::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }
            });

            while let Some(Ok(msg)) = ws_stream.next().await {
                if let WsMessage::Text(text) = msg {
                    let res_message: Result<WsClientMessage, _> = serde_json::from_str(&text);
                    match res_message {
                        Ok(message) => {
                            message.process(conn_id, &local_tx, &redis, &postgres).await;
                        }
                        Err(e) => {
                            let _ = local_tx.send(WsServerMessage::Error {
                                code: WsServerMessageErrorCode::InvalidMessage,
                            });
                            tracing::warn!(%e, "invalid message");
                        }
                    }
                }
            }

            send_task.abort();
            conns_map.write().await.remove(&conn_id);
            if let Ok(Some(room_id)) = redis.room_of_conn(conn_id).await {
                if let Err(e) = redis
                    .publish_ws_event(&WsRedisEvent::Disconnected {
                        room_id: room_id.clone(),
                    })
                    .await
                {
                    tracing::error!(%e, "could not send disconnected")
                }
                if let Err(e) = redis
                    .publish_ws_event(&WsRedisEvent::Typing {
                        room_id,
                        status: false,
                    })
                    .await
                {
                    tracing::error!(%e, "could not send disconnected")
                }
            }
            let _ = redis.leave_room(conn_id).await;
            tracing::info!(%conn_id, "client disconnected");
        }
    }))
}
