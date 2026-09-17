use anyhow::{Result, anyhow};
use redis::{AsyncCommands, RedisError};

pub mod events;
pub mod rooms;

#[derive(Clone)]
pub struct RedisAdapter {
    client: redis::Client,
    connection: redis::aio::MultiplexedConnection,
}

impl RedisAdapter {
    pub async fn new() -> Result<Self> {
        let Ok(redis_url) = std::env::var("REDIS_URL") else {
            return Err(anyhow!("missing REDIS_URL in environment variables"));
        };
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_multiplexed_async_connection().await?;
        let mut redis_adapter = Self { client, connection };
        redis_adapter.clear().await?;
        Ok(redis_adapter)
    }

    pub fn conn(&self) -> redis::aio::MultiplexedConnection {
        self.connection.clone()
    }

    pub async fn clear(&mut self) -> Result<()> {
        redis::cmd("FLUSHDB")
            .query_async::<()>(&mut self.connection)
            .await?;
        tracing::info!("redis cleared");
        Ok(())
    }

    pub async fn healthcheck(&self) -> Result<(), RedisAdapterError> {
        let res: Result<String, RedisError> = self.conn().ping().await;
        if let Err(err) = res {
            tracing::error!(%err, "redis not healthy");
            return Err(err.into());
        };
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RedisAdapterError {
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("serde_json error: {0}")]
    Serde(#[from] serde_json::Error),
}
