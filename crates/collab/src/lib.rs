pub mod api;
pub mod auth;
mod cents;
pub mod db;
pub mod env;
pub mod executor;
pub mod llm;
pub mod migrations;
pub mod rpc;
pub mod seed;
pub mod stripe_billing;
pub mod user_backfiller;

#[cfg(test)]
mod tests;

use anyhow::anyhow;
use aws_config::{BehaviorVersion, Region};
use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
pub use cents::*;
use db::{ChannelId, Database};
use executor::Executor;
use llm::db::LlmDatabase;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use util::ResultExt;

use crate::stripe_billing::StripeBilling;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Http(StatusCode, String, HeaderMap),
    Database(sea_orm::error::DbErr),
    Internal(anyhow::Error),
    Stripe(stripe::StripeError),
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

impl From<stripe::StripeError> for Error {
    fn from(error: stripe::StripeError) -> Self {
        Self::Stripe(error)
    }
}

impl From<axum::Error> for Error {
    fn from(error: axum::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<axum::http::Error> for Error {
    fn from(error: axum::http::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl Error {
    fn http(code: StatusCode, message: String) -> Self {
        Self::Http(code, message, HeaderMap::default())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Http(code, message, headers) => {
                log::error!("HTTP error {}: {}", code, &message);
                (code, headers, message).into_response()
            }
            Error::Database(error) => {
                log::error!(
                    "HTTP error {}: {:?}",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error
                );
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &error)).into_response()
            }
            Error::Internal(error) => {
                log::error!(
                    "HTTP error {}: {:?}",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error
                );
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &error)).into_response()
            }
            Error::Stripe(error) => {
                log::error!(
                    "HTTP error {}: {:?}",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error
                );
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &error)).into_response()
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message, _headers) => (code, message).fmt(f),
            Error::Database(error) => error.fmt(f),
            Error::Internal(error) => error.fmt(f),
            Error::Stripe(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message, _) => write!(f, "{code}: {message}"),
            Error::Database(error) => error.fmt(f),
            Error::Internal(error) => error.fmt(f),
            Error::Stripe(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub database_url: String,
    pub migrations_path: Option<PathBuf>,
    pub seed_path: Option<PathBuf>,
    pub database_max_connections: u32,
    pub api_token: String,
    pub invite_link_prefix: String,
    pub livekit_server: Option<String>,
    pub livekit_key: Option<String>,
    pub livekit_secret: Option<String>,
    pub llm_database_url: Option<String>,
    pub llm_database_max_connections: Option<u32>,
    pub llm_database_migrations_path: Option<PathBuf>,
    pub llm_api_secret: Option<String>,
    pub rust_log: Option<String>,
    pub log_json: Option<bool>,
    pub blob_store_url: Option<String>,
    pub blob_store_region: Option<String>,
    pub blob_store_access_key: Option<String>,
    pub blob_store_secret_key: Option<String>,
    pub blob_store_bucket: Option<String>,
    pub kinesis_region: Option<String>,
    pub kinesis_stream: Option<String>,
    pub kinesis_access_key: Option<String>,
    pub kinesis_secret_key: Option<String>,
    pub zed_environment: Arc<str>,
    pub openai_api_key: Option<Arc<str>>,
    pub google_ai_api_key: Option<Arc<str>>,
    pub anthropic_api_key: Option<Arc<str>>,
    pub anthropic_staff_api_key: Option<Arc<str>>,
    pub llm_closed_beta_model_name: Option<Arc<str>>,
    pub prediction_api_url: Option<Arc<str>>,
    pub prediction_api_key: Option<Arc<str>>,
    pub prediction_model: Option<Arc<str>>,
    pub zed_client_checksum_seed: Option<String>,
    pub slack_panics_webhook: Option<String>,
    pub auto_join_channel_id: Option<ChannelId>,
    pub stripe_api_key: Option<String>,
    pub supermaven_admin_api_key: Option<Arc<str>>,
    pub user_backfiller_github_access_token: Option<Arc<str>>,
}

impl Config {
    pub fn is_development(&self) -> bool {
        self.zed_environment == "development".into()
    }

    /// Returns the base `zed.dev` URL.
    pub fn zed_dot_dev_url(&self) -> &str {
        match self.zed_environment.as_ref() {
            "development" => "http://localhost:3000",
            "staging" => "https://staging.zed.dev",
            _ => "https://zed.dev",
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            http_port: 0,
            database_url: "".into(),
            database_max_connections: 0,
            api_token: "".into(),
            invite_link_prefix: "".into(),
            livekit_server: None,
            livekit_key: None,
            livekit_secret: None,
            llm_database_url: None,
            llm_database_max_connections: None,
            llm_database_migrations_path: None,
            llm_api_secret: None,
            rust_log: None,
            log_json: None,
            zed_environment: "test".into(),
            blob_store_url: None,
            blob_store_region: None,
            blob_store_access_key: None,
            blob_store_secret_key: None,
            blob_store_bucket: None,
            openai_api_key: None,
            google_ai_api_key: None,
            anthropic_api_key: None,
            anthropic_staff_api_key: None,
            llm_closed_beta_model_name: None,
            prediction_api_url: None,
            prediction_api_key: None,
            prediction_model: None,
            zed_client_checksum_seed: None,
            slack_panics_webhook: None,
            auto_join_channel_id: None,
            migrations_path: None,
            seed_path: None,
            stripe_api_key: None,
            supermaven_admin_api_key: None,
            user_backfiller_github_access_token: None,
            kinesis_region: None,
            kinesis_access_key: None,
            kinesis_secret_key: None,
            kinesis_stream: None,
        }
    }
}

/// The service mode that collab should run in.
#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum ServiceMode {
    Api,
    Collab,
    All,
}

impl ServiceMode {
    pub fn is_collab(&self) -> bool {
        matches!(self, Self::Collab | Self::All)
    }

    pub fn is_api(&self) -> bool {
        matches!(self, Self::Api | Self::All)
    }
}

pub struct AppState {
    pub db: Arc<Database>,
    pub llm_db: Option<Arc<LlmDatabase>>,
    pub livekit_client: Option<Arc<dyn livekit_api::Client>>,
    pub blob_store_client: Option<aws_sdk_s3::Client>,
    pub stripe_client: Option<Arc<stripe::Client>>,
    pub stripe_billing: Option<Arc<StripeBilling>>,
    pub executor: Executor,
    pub kinesis_client: Option<::aws_sdk_kinesis::Client>,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config, executor: Executor) -> Result<Arc<Self>> {
        let mut db_options = db::ConnectOptions::new(config.database_url.clone());
        db_options.max_connections(config.database_max_connections);
        let mut db = Database::new(db_options, Executor::Production).await?;
        db.initialize_notification_kinds().await?;

        let llm_db = if let Some((llm_database_url, llm_database_max_connections)) = config
            .llm_database_url
            .clone()
            .zip(config.llm_database_max_connections)
        {
            let mut llm_db_options = db::ConnectOptions::new(llm_database_url);
            llm_db_options.max_connections(llm_database_max_connections);
            let mut llm_db = LlmDatabase::new(llm_db_options, executor.clone()).await?;
            llm_db.initialize().await?;
            Some(Arc::new(llm_db))
        } else {
            None
        };

        let livekit_client = if let Some(((server, key), secret)) = config
            .livekit_server
            .as_ref()
            .zip(config.livekit_key.as_ref())
            .zip(config.livekit_secret.as_ref())
        {
            Some(Arc::new(livekit_api::LiveKitClient::new(
                server.clone(),
                key.clone(),
                secret.clone(),
            )) as Arc<dyn livekit_api::Client>)
        } else {
            None
        };

        let db = Arc::new(db);
        let stripe_client = build_stripe_client(&config).map(Arc::new).log_err();
        let this = Self {
            db: db.clone(),
            llm_db,
            livekit_client,
            blob_store_client: build_blob_store_client(&config).await.log_err(),
            stripe_billing: stripe_client
                .clone()
                .map(|stripe_client| Arc::new(StripeBilling::new(stripe_client))),
            stripe_client,
            executor,
            kinesis_client: if config.kinesis_access_key.is_some() {
                build_kinesis_client(&config).await.log_err()
            } else {
                None
            },
            config,
        };
        Ok(Arc::new(this))
    }
}

fn build_stripe_client(config: &Config) -> anyhow::Result<stripe::Client> {
    let api_key = config
        .stripe_api_key
        .as_ref()
        .ok_or_else(|| anyhow!("missing stripe_api_key"))?;
    Ok(stripe::Client::new(api_key))
}

async fn build_blob_store_client(config: &Config) -> anyhow::Result<aws_sdk_s3::Client> {
    let keys = aws_sdk_s3::config::Credentials::new(
        config
            .blob_store_access_key
            .clone()
            .ok_or_else(|| anyhow!("missing blob_store_access_key"))?,
        config
            .blob_store_secret_key
            .clone()
            .ok_or_else(|| anyhow!("missing blob_store_secret_key"))?,
        None,
        None,
        "env",
    );

    let s3_config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(
            config
                .blob_store_url
                .as_ref()
                .ok_or_else(|| anyhow!("missing blob_store_url"))?,
        )
        .region(Region::new(
            config
                .blob_store_region
                .clone()
                .ok_or_else(|| anyhow!("missing blob_store_region"))?,
        ))
        .credentials_provider(keys)
        .load()
        .await;

    Ok(aws_sdk_s3::Client::new(&s3_config))
}

async fn build_kinesis_client(config: &Config) -> anyhow::Result<aws_sdk_kinesis::Client> {
    let keys = aws_sdk_s3::config::Credentials::new(
        config
            .kinesis_access_key
            .clone()
            .ok_or_else(|| anyhow!("missing kinesis_access_key"))?,
        config
            .kinesis_secret_key
            .clone()
            .ok_or_else(|| anyhow!("missing kinesis_secret_key"))?,
        None,
        None,
        "env",
    );

    let kinesis_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(
            config
                .kinesis_region
                .clone()
                .ok_or_else(|| anyhow!("missing kinesis_region"))?,
        ))
        .credentials_provider(keys)
        .load()
        .await;

    Ok(aws_sdk_kinesis::Client::new(&kinesis_config))
}
