use chrono::{DateTime, Utc};
use sqlx::{query_as, query_as_with};
use tracing::instrument;
use validator::Validate;

use crate::{
    adapters::postgres::{
        PostgresAdapter, PostgresAdapterError,
        req_builder::{CompOp, ReqBuilder},
    },
    routes::Normalize,
    utils::validate_slug,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow, Validate)]
pub struct RoomDto {
    #[validate(length(min = 1, max = 50), custom(function = "validate_slug"))]
    pub id: String,
}

impl Normalize for RoomDto {
    fn normalize(&mut self) -> Self {
        tracing::debug!("normalizing RoomDto");
        Self {
            id: ammonia::clean(self.id.trim()),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow, derive_more::Deref)]
pub struct Room {
    #[serde(flatten)]
    #[sqlx(flatten)]
    #[deref]
    pub room_dto: RoomDto,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl PostgresAdapter {
    #[instrument(skip(self))]
    pub async fn room_add(&self, room_dto: RoomDto) -> Result<Room, PostgresAdapterError> {
        let room: Room = query_as("INSERT INTO rooms (id) VALUES ($1) RETURNING *;")
            .bind(room_dto.id)
            .fetch_one(&self.pool)
            .await?;
        tracing::info!("room created");
        Ok(room)
    }

    #[instrument(skip(self))]
    pub async fn room_get(&self, room_id: &str) -> Result<Room, PostgresAdapterError> {
        let mut builder = ReqBuilder::default();
        builder.r#where("id", CompOp::Equal, room_id)?;
        let room = query_as_with(builder.build("SELECT * FROM rooms", ""), builder.args)
            .fetch_one(&self.pool)
            .await?;
        Ok(room)
    }

    #[instrument(skip(self))]
    pub async fn room_cleanup(&self) -> Result<Vec<Room>, PostgresAdapterError> {
        tracing::info!("room cleanup");
        let mut builder = ReqBuilder::default();
        builder.r#where("expires_at", CompOp::Lower, chrono::Utc::now())?;
        let rooms: Vec<Room> = query_as_with(
            builder.build("DELETE FROM rooms", "RETURNING *;"),
            builder.args,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rooms)
    }
}
