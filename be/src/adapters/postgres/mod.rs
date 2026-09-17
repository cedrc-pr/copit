use std::time::Duration;

use sqlx::{Pool, Postgres};

pub mod messages;
pub mod req_builder;
pub mod rooms;
pub mod shared;

#[derive(Debug)]
pub struct PostgresAdapter {
    pub pool: Pool<Postgres>,
}

impl PostgresAdapter {
    pub async fn init() -> anyhow::Result<Self> {
        let url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return Err(anyhow::anyhow!("missing DATABASE_URL environment variable")),
        };
        let pool = match sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(3))
            .connect(&url)
            .await
        {
            Ok(pool) => pool,
            Err(err) => {
                return Err(anyhow::anyhow!("could not to connect to database: {}", err));
            }
        };
        Ok(Self { pool })
    }

    pub async fn healthcheck(&self) -> Result<(), PostgresAdapterError> {
        if let Err(err) = sqlx::query("SELECT 1").execute(&self.pool).await {
            tracing::error!(%err, "postgres not healthy");
            return Err(err.into());
        };
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PostgresAdapterError {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("internal server error")]
    Unexpected,
}

impl From<sqlx::Error> for PostgresAdapterError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            sqlx::Error::Database(err) => {
                if err.is_unique_violation() {
                    Self::Conflict
                } else if err.is_foreign_key_violation() {
                    Self::NotFound
                } else {
                    tracing::error!(err = %err, "unexpected error");
                    Self::Unexpected
                }
            }
            _ => {
                tracing::error!(err = %value, "unexpected error");
                Self::Unexpected
            }
        }
    }
}

impl From<Box<dyn std::error::Error + 'static + Send + Sync>> for PostgresAdapterError {
    fn from(value: Box<dyn std::error::Error + 'static + Send + Sync>) -> Self {
        tracing::error!(err = %value, "req builder error");
        Self::Unexpected
    }
}
