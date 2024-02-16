use anyhow::anyhow;
use axum::{routing::get, Extension, Router};
use collab::{
    api::fetch_extensions_from_blob_store_periodically, db, env, executor::Executor, AppState,
    Config, MigrateConfig, Result,
};
use db::Database;
use std::{
    env::args,
    net::{SocketAddr, TcpListener},
    path::Path,
    sync::Arc,
};
use tokio::signal::unix::SignalKind;
use tracing_log::LogTracer;
use tracing_subscriber::{filter::EnvFilter, fmt::format::JsonFields, Layer};
use util::ResultExt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const REVISION: Option<&'static str> = option_env!("GITHUB_SHA");

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
            println!("collab v{} ({})", VERSION, REVISION.unwrap_or("unknown"));
        }
        Some("migrate") => {
            run_migrations().await?;
        }
        Some("serve") => {
            let config = envy::from_env::<Config>().expect("error loading config");
            init_tracing(&config);

            run_migrations().await?;

            let state = AppState::new(config).await?;

            let listener = TcpListener::bind(&format!("0.0.0.0:{}", state.config.http_port))
                .expect("failed to bind TCP listener");

            let epoch = state
                .db
                .create_server(&state.config.zed_environment)
                .await?;
            let rpc_server = collab::rpc::Server::new(epoch, state.clone(), Executor::Production);
            rpc_server.start().await?;

            fetch_extensions_from_blob_store_periodically(state.clone(), Executor::Production);

            let app = collab::api::routes(rpc_server.clone(), state.clone())
                .merge(collab::rpc::routes(rpc_server.clone()))
                .merge(
                    Router::new()
                        .route("/", get(handle_root))
                        .route("/healthz", get(handle_liveness_probe))
                        .layer(Extension(state.clone())),
                );

            axum::Server::from_tcp(listener)?
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(async move {
                    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())
                        .expect("failed to listen for interrupt signal");
                    let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())
                        .expect("failed to listen for interrupt signal");
                    let sigterm = sigterm.recv();
                    let sigint = sigint.recv();
                    futures::pin_mut!(sigterm, sigint);
                    futures::future::select(sigterm, sigint).await;
                    tracing::info!("Received interrupt signal");
                    rpc_server.teardown();
                })
                .await?;
        }
        _ => {
            Err(anyhow!("usage: collab <version | migrate | serve>"))?;
        }
    }
    Ok(())
}

async fn run_migrations() -> Result<()> {
    let config = envy::from_env::<MigrateConfig>().expect("error loading config");
    let db_options = db::ConnectOptions::new(config.database_url.clone());
    let db = Database::new(db_options, Executor::Production).await?;

    let migrations_path = config
        .migrations_path
        .as_deref()
        .unwrap_or_else(|| Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations")));

    let migrations = db.migrate(&migrations_path, false).await?;
    for (migration, duration) in migrations {
        log::info!(
            "Migrated {} {} {:?}",
            migration.version,
            migration.description,
            duration
        );
    }

    return Ok(());
}

async fn handle_root() -> String {
    format!("collab v{} ({})", VERSION, REVISION.unwrap_or("unknown"))
}

async fn handle_liveness_probe(Extension(state): Extension<Arc<AppState>>) -> Result<String> {
    state.db.get_all_users(0, 1).await?;
    Ok("ok".to_string())
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
