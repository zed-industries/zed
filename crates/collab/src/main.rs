mod api;
mod auth;
mod db;
mod env;
mod rpc;

#[cfg(test)]
mod db_tests;
#[cfg(test)]
mod integration_tests;

use anyhow::anyhow;
use axum::{routing::get, Router};
use collab::{Error, Result};
use db::{Db, PostgresDb};
use serde::Deserialize;
use std::{
    env::args,
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
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
    pub rust_log: Option<String>,
    pub log_json: Option<bool>,
}

#[derive(Default, Deserialize)]
pub struct MigrateConfig {
    pub database_url: String,
    pub migrations_path: Option<PathBuf>,
}

pub struct AppState {
    db: Arc<dyn Db>,
    config: Config,
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
            let db = PostgresDb::new(&config.database_url, 5).await?;

            let migrations_path = config
                .migrations_path
                .as_deref()
                .or(db::DEFAULT_MIGRATIONS_PATH.map(|s| s.as_ref()))
                .ok_or_else(|| anyhow!("missing MIGRATIONS_PATH environment variable"))?;

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
            let db = PostgresDb::new(&config.database_url, 5).await?;

            init_tracing(&config);
            let state = Arc::new(AppState {
                db: Arc::new(db),
                config,
            });

            let listener = TcpListener::bind(&format!("0.0.0.0:{}", state.config.http_port))
                .expect("failed to bind TCP listener");

            let rpc_server = rpc::Server::new(state.clone(), None);
            rpc_server
                .start_recording_project_activity(Duration::from_secs(5 * 60), rpc::RealExecutor);

            let app = api::routes(&rpc_server, state.clone())
                .merge(rpc::routes(rpc_server))
                .merge(Router::new().route("/", get(handle_root)));

            axum::Server::from_tcp(listener)?
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
