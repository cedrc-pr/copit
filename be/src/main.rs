use std::{collections::HashMap, sync::Arc};

use poem::{EndpointExt, Server, listener::TcpListener};
use tokio::sync::RwLock;

use crate::{
    adapters::{postgres::PostgresAdapter, redis::RedisAdapter},
    routes::routes,
    tools::ws_server_message::WsServerMessage,
    utils::{cors, load_env, periodic_cleanup, setup_tracing},
};

pub mod adapters;
pub mod app_error;
pub mod routes;
pub mod tools;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_env()?;
    setup_tracing();

    start_server().await?;
    Ok(())
}

async fn start_server() -> anyhow::Result<()> {
    let bind = "0.0.0.0:3000";
    let server = Server::new(TcpListener::bind(&bind));

    let conns: Arc<
        RwLock<HashMap<uuid::Uuid, tokio::sync::mpsc::UnboundedSender<WsServerMessage>>>,
    > = Arc::new(RwLock::new(HashMap::new()));

    let postgres_adapter = Arc::new(PostgresAdapter::init().await?);
    let redis_adapter = Arc::new(RedisAdapter::new().await?);
    redis_adapter.subscribe_ws_events(conns.clone());
    periodic_cleanup(postgres_adapter.clone());

    tracing::info!("server listening on: {}", bind);

    match server
        .run(
            routes()
                .data(postgres_adapter)
                .data(redis_adapter)
                .data(conns)
                .with(cors()),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow::anyhow!(err)),
    }
}
