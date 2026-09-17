use chrono::{DateTime, Utc};
use sqlx::query_as;
use validator::Validate;

use crate::{
    adapters::postgres::{PostgresAdapter, PostgresAdapterError},
    routes::Normalize,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow, Validate, Clone)]
pub struct MessageDto {
    #[validate(length(min = 1, max = 2048))]
    pub content: String,
    #[validate(length(min = 1, max = 50))]
    pub author: String,
}

impl Normalize for MessageDto {
    fn normalize(&mut self) -> Self {
        tracing::debug!("normalizing MessageDto");
        Self {
            content: ammonia::clean(self.content.trim()),
            author: ammonia::clean(self.author.trim()),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow, derive_more::Deref, Clone)]
pub struct Message {
    pub id: uuid::Uuid,
    #[serde(flatten)]
    #[sqlx(flatten)]
    #[deref]
    pub message_dto: MessageDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PostgresAdapter {
    pub async fn message_add(
        &self,
        message_dto: MessageDto,
        room_id: String,
    ) -> Result<Message, PostgresAdapterError> {
        let message: Message = query_as(
            "INSERT INTO messages (room_id, content, author) VALUES ($1, $2, $3) RETURNING *;",
        )
        .bind(room_id)
        .bind(message_dto.content)
        .bind(message_dto.author)
        .fetch_one(&self.pool)
        .await?;

        tracing::info!("message created");
        Ok(message)
    }
}
