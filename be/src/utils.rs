use std::sync::Arc;

use anyhow::anyhow;
use poem::{
    http::{Method, header},
    middleware::Cors,
};
use tracing_subscriber::{EnvFilter, fmt};

use crate::adapters::postgres::PostgresAdapter;

pub fn setup_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        if std::env::var("ENV")
            .map_err(|_| anyhow!("missing ENV in .env"))
            .unwrap_or("dev".to_owned())
            == "dev"
        {
            EnvFilter::new("be=trace")
        } else {
            EnvFilter::new("be=info")
        }
    });
    fmt().with_env_filter(env_filter).init();
}

pub fn load_env() -> anyhow::Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(_) => {
            if !std::env::var("ENV").is_ok() {
                return Err(anyhow::anyhow!("missing .env file"));
            } else {
                Ok(())
            }
        }
    }
}

pub fn periodic_cleanup(postgres_adapter: Arc<PostgresAdapter>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15 * 60));
        loop {
            interval.tick().await;
            if let Err(e) = postgres_adapter.room_cleanup().await {
                eprintln!("Erreur lors du nettoyage : {e}");
            }
        }
    });
}

pub fn validate_slug(slug: &str) -> Result<(), validator::ValidationError> {
    let re = match regex::Regex::new(r"[A-Za-z0-9-]{1,50}$") {
        Ok(re) => re,
        Err(_) => return Err(validator::ValidationError::new("app error")),
    };

    if re.is_match(slug) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid slug"))
    }
}

pub fn cors() -> Cors {
    match std::env::var("ENV") {
        Ok(env) => {
            if env == "dev" {
                Cors::new()
                    .allow_origin("http://localhost:5173")
                    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            } else {
                Cors::new()
            }
        }
        Err(_) => Cors::new(),
    }
}
