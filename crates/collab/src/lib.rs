pub mod api;
pub mod auth;
pub mod db;
pub mod env;
mod executor;
#[cfg(test)]
mod integration_tests;
pub mod rpc;

use axum::{http::StatusCode, response::IntoResponse};
use db::Database;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Http(StatusCode, String),
    Database(sea_orm::error::DbErr),
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal(error)
    }
}

impl From<sea_orm::error::DbErr> for Error {
    fn from(error: sea_orm::error::DbErr) -> Self {
        Self::Database(error)
    }
}

impl From<axum::Error> for Error {
    fn from(error: axum::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Http(code, message) => (code, message).into_response(),
            Error::Database(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &error)).into_response()
            }
            Error::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &error)).into_response()
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message) => (code, message).fmt(f),
            Error::Database(error) => error.fmt(f),
            Error::Internal(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message) => write!(f, "{code}: {message}"),
            Error::Database(error) => error.fmt(f),
            Error::Internal(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Default, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub database_url: String,
    pub api_token: String,
    pub invite_link_prefix: String,
    pub live_kit_server: Option<String>,
    pub live_kit_key: Option<String>,
    pub live_kit_secret: Option<String>,
    pub rust_log: Option<String>,
    pub log_json: Option<bool>,
}

#[derive(Default, Deserialize)]
pub struct MigrateConfig {
    pub database_url: String,
    pub migrations_path: Option<PathBuf>,
}

pub struct AppState {
    pub db: Arc<Database>,
    pub live_kit_client: Option<Arc<dyn live_kit_server::api::Client>>,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Arc<Self>> {
        let mut db_options = db::ConnectOptions::new(config.database_url.clone());
        db_options.max_connections(5);
        let db = Database::new(db_options).await?;
        let live_kit_client = if let Some(((server, key), secret)) = config
            .live_kit_server
            .as_ref()
            .zip(config.live_kit_key.as_ref())
            .zip(config.live_kit_secret.as_ref())
        {
            Some(Arc::new(live_kit_server::api::LiveKitClient::new(
                server.clone(),
                key.clone(),
                secret.clone(),
            )) as Arc<dyn live_kit_server::api::Client>)
        } else {
            None
        };

        let this = Self {
            db: Arc::new(db),
            live_kit_client,
            config,
        };
        Ok(Arc::new(this))
    }
}
