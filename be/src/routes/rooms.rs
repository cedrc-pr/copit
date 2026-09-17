use std::sync::Arc;

use poem::{
    Route, get,
    http::StatusCode,
    post,
    web::{Data, Json, Path, Query},
};
use validator::Validate;

use crate::{
    adapters::postgres::{
        PostgresAdapter,
        messages::Message,
        rooms::{Room, RoomDto},
        shared::MessagesQuery,
    },
    app_error::AppError,
    routes::Normalize,
};

pub fn rooms_routes() -> Route {
    Route::new()
        .at("/", post(room_add))
        .at("/:id/messages", get(get_room_messages))
        .at("/:id", get(get_room))
}

#[poem::handler]
async fn room_add(
    Data(postgres_adapter): Data<&Arc<PostgresAdapter>>,
    Json(mut room_dto): Json<RoomDto>,
) -> Result<(StatusCode, Json<Room>), AppError> {
    room_dto.normalize().validate()?;
    Ok((
        StatusCode::CREATED,
        Json(postgres_adapter.room_add(room_dto).await?),
    ))
}

#[poem::handler]
async fn get_room_messages(
    Data(postgres_adapter): Data<&Arc<PostgresAdapter>>,
    Query(messages_query): Query<MessagesQuery>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Message>>, AppError> {
    Ok(Json(
        postgres_adapter
            .get_room_messages(messages_query, id)
            .await?,
    ))
}

#[poem::handler]
async fn get_room(
    Data(postgres_adapter): Data<&Arc<PostgresAdapter>>,
    Path(id): Path<String>,
) -> Result<Json<Room>, AppError> {
    Ok(Json(postgres_adapter.room_get(&id).await?))
}
