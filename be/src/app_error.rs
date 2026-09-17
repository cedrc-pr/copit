use poem::{IntoResponse, http::StatusCode, web::Json};

use crate::adapters::postgres::PostgresAdapterError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("postgres error: {0}")]
    Postgres(#[from] PostgresAdapterError),
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("unauthorized: {0}")]
    BadRequest(String),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::BadRequest(value.to_string())
    }
}

impl poem::error::ResponseError for AppError {
    fn status(&self) -> poem::http::StatusCode {
        match self {
            Self::Postgres(err) => match err {
                PostgresAdapterError::NotFound => StatusCode::NOT_FOUND,
                PostgresAdapterError::Conflict => StatusCode::CONFLICT,
                PostgresAdapterError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> poem::Response
    where
        Self: std::error::Error + Send + Sync + 'static,
    {
        (self.status(), Json(self.to_string())).into_response()
    }
}
