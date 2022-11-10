mod api;
mod auth;
mod db;
mod env;
mod rpc;

#[cfg(test)]
mod db_tests;
#[cfg(test)]
mod integration_tests;

use crate::rpc::ResultExt as _;
use anyhow::anyhow;
use axum::{routing::get, Router};
use collab::{Error, Result};
use db::DefaultDb as Db;
use serde::Deserialize;
use std::{
    env::args,
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::signal;
use tracing_log::LogTracer;
use tracing_subscriber::{filter::EnvFilter, fmt::format::JsonFields, Layer};
use util::ResultExt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    db: Arc<Db>,
    live_kit_client: Option<Arc<dyn live_kit_server::api::Client>>,
    config: Config,
}

impl AppState {
    async fn new(config: Config) -> Result<Arc<Self>> {
        let db = Db::new(&config.database_url, 5).await?;
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

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(error) = env::load_dotenv() {
        eprintln!(
            "error loading .env.toml (this is expected in production): {}",
            error
        );
    }

    match args().skip(1).next().as_deref() {
        Some("version") => {
            println!("collab v{VERSION}");
        }
        Some("migrate") => {
            let config = envy::from_env::<MigrateConfig>().expect("error loading config");
            let db = Db::new(&config.database_url, 5).await?;

            let migrations_path = config
                .migrations_path
                .as_deref()
                .unwrap_or_else(|| Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations")));

            let migrations = db.migrate(&migrations_path, false).await?;
            for (migration, duration) in migrations {
                println!(
                    "Ran {} {} {:?}",
                    migration.version, migration.description, duration
                );
            }

            return Ok(());
        }
        Some("serve") => {
            let config = envy::from_env::<Config>().expect("error loading config");
            init_tracing(&config);

            let state = AppState::new(config).await?;
            let listener = TcpListener::bind(&format!("0.0.0.0:{}", state.config.http_port))
                .expect("failed to bind TCP listener");

            let rpc_server = rpc::Server::new(state.clone());

            let app = api::routes(rpc_server.clone(), state.clone())
                .merge(rpc::routes(rpc_server.clone()))
                .merge(Router::new().route("/", get(handle_root)));

            axum::Server::from_tcp(listener)?
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(graceful_shutdown(rpc_server, state))
                .await?;
        }
        _ => {
            Err(anyhow!("usage: collab <version | migrate | serve>"))?;
        }
    }
    Ok(())
}

async fn handle_root() -> String {
    format!("collab v{VERSION}")
}

pub fn init_tracing(config: &Config) -> Option<()> {
    use std::str::FromStr;
    use tracing_subscriber::layer::SubscriberExt;
    let rust_log = config.rust_log.clone()?;

    LogTracer::init().log_err()?;

    let subscriber = tracing_subscriber::Registry::default()
        .with(if config.log_json.unwrap_or(false) {
            Box::new(
                tracing_subscriber::fmt::layer()
                    .fmt_fields(JsonFields::default())
                    .event_format(
                        tracing_subscriber::fmt::format()
                            .json()
                            .flatten_event(true)
                            .with_span_list(true),
                    ),
            ) as Box<dyn Layer<_> + Send + Sync>
        } else {
            Box::new(
                tracing_subscriber::fmt::layer()
                    .event_format(tracing_subscriber::fmt::format().pretty()),
            )
        })
        .with(EnvFilter::from_str(rust_log.as_str()).log_err()?);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    None
}

async fn graceful_shutdown(rpc_server: Arc<rpc::Server>, state: Arc<AppState>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    if let Some(live_kit) = state.live_kit_client.as_ref() {
        let deletions = rpc_server
            .store()
            .await
            .rooms()
            .values()
            .map(|room| {
                let name = room.live_kit_room.clone();
                async {
                    live_kit.delete_room(name).await.trace_err();
                }
            })
            .collect::<Vec<_>>();

        tracing::info!("deleting all live-kit rooms");
        if let Err(_) = tokio::time::timeout(
            Duration::from_secs(10),
            futures::future::join_all(deletions),
        )
        .await
        {
            tracing::error!("timed out waiting for live-kit room deletion");
        }
    }
}
