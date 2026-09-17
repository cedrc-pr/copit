use std::sync::Arc;

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Route, get};

use crate::{
    adapters::{postgres::PostgresAdapter, redis::RedisAdapter},
    routes::{rooms::rooms_routes, ws::ws_routes},
};
pub mod rooms;
pub mod ws;

pub fn routes() -> Route {
    Route::new()
        .nest("/rooms", rooms_routes())
        .nest("/ws", ws_routes())
        .at("/health", get(health))
}

pub trait Normalize {
    fn normalize(&mut self) -> Self;
}

#[poem::handler]
async fn health(
    Data(postgres_adapter): Data<&Arc<PostgresAdapter>>,
    Data(redis_adapter): Data<&Arc<RedisAdapter>>,
) -> (StatusCode, String) {
    match (
        postgres_adapter.healthcheck().await,
        redis_adapter.healthcheck().await,
    ) {
        (Ok(_), Ok(_)) => (StatusCode::OK, "healthy".to_owned()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "not healthy".to_owned()),
    }
}
