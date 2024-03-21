use anyhow::anyhow;
use axum::{
    extract::MatchedPath,
    http::{Request, Response},
    routing::get,
    Extension, Router,
};
use collab::{
    api::fetch_extensions_from_blob_store_periodically, db, env, executor::Executor, AppState,
    Config, RateLimiter, Result,
};
use db::Database;
use std::{
    env::args,
    net::{SocketAddr, TcpListener},
    path::Path,
    sync::Arc,
    time::Duration,
};
#[cfg(unix)]
use tokio::signal::unix::SignalKind;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    filter::EnvFilter, fmt::format::JsonFields, util::SubscriberInitExt, Layer,
};
use util::ResultExt;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REVISION: Option<&'static str> = option_env!("GITHUB_SHA");

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(error) = env::load_dotenv() {
        eprintln!(
            "error loading .env.toml (this is expected in production): {}",
            error
        );
    }

    let mut args = args().skip(1);
    match args.next().as_deref() {
        Some("version") => {
            println!("collab v{} ({})", VERSION, REVISION.unwrap_or("unknown"));
        }
        Some("migrate") => {
            let config = envy::from_env::<Config>().expect("error loading config");
            run_migrations(&config).await?;
        }
        Some("seed") => {
            let config = envy::from_env::<Config>().expect("error loading config");
            let db_options = db::ConnectOptions::new(config.database_url.clone());
            let mut db = Database::new(db_options, Executor::Production).await?;
            db.initialize_notification_kinds().await?;

            collab::seed::seed(&config, &db, true).await?;
        }
        Some("serve") => {
            let (is_api, is_collab) = if let Some(next) = args.next() {
                (next == "api", next == "collab")
            } else {
                (true, true)
            };
            if !is_api && !is_collab {
                Err(anyhow!(
                    "usage: collab <version | migrate | seed | serve [api|collab]>"
                ))?;
            }

            let config = envy::from_env::<Config>().expect("error loading config");
            init_tracing(&config);

            run_migrations(&config).await?;

            let state = AppState::new(config, Executor::Production).await?;

            let listener = TcpListener::bind(&format!("0.0.0.0:{}", state.config.http_port))
                .expect("failed to bind TCP listener");

            let rpc_server = if is_collab {
                let epoch = state
                    .db
                    .create_server(&state.config.zed_environment)
                    .await?;
                let rpc_server = collab::rpc::Server::new(epoch, state.clone());
                rpc_server.start().await?;

                Some(rpc_server)
            } else {
                None
            };

            if is_collab {
                RateLimiter::save_periodically(state.rate_limiter.clone(), state.executor.clone());
            }

            if is_api {
                fetch_extensions_from_blob_store_periodically(state.clone());
            }

            let mut app = collab::api::routes(rpc_server.clone(), state.clone());
            if let Some(rpc_server) = rpc_server.clone() {
                app = app.merge(collab::rpc::routes(rpc_server))
            }
            app = app
                .merge(
                    Router::new()
                        .route("/", get(handle_root))
                        .route("/healthz", get(handle_liveness_probe))
                        .merge(collab::api::extensions::router())
                        .merge(collab::api::events::router())
                        .layer(Extension(state.clone())),
                )
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            tracing::info_span!(
                                "http_request",
                                method = ?request.method(),
                                matched_path,
                            )
                        })
                        .on_response(
                            |response: &Response<_>, latency: Duration, _: &tracing::Span| {
                                let duration_ms = latency.as_micros() as f64 / 1000.;
                                tracing::info!(
                                    duration_ms,
                                    status = response.status().as_u16(),
                                    "finished processing request"
                                );
                            },
                        ),
                );

            #[cfg(unix)]
            axum::Server::from_tcp(listener)
                .map_err(|e| anyhow!(e))?
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

                    if let Some(rpc_server) = rpc_server {
                        rpc_server.teardown();
                    }
                })
                .await
                .map_err(|e| anyhow!(e))?;

            // todo("windows")
            #[cfg(windows)]
            unimplemented!();
        }
        _ => {
            Err(anyhow!(
                "usage: collab <version | migrate | seed | serve [api|collab]>"
            ))?;
        }
    }
    Ok(())
}

async fn run_migrations(config: &Config) -> Result<()> {
    let db_options = db::ConnectOptions::new(config.database_url.clone());
    let mut db = Database::new(db_options, Executor::Production).await?;

    let migrations_path = config.migrations_path.as_deref().unwrap_or_else(|| {
        #[cfg(feature = "sqlite")]
        let default_migrations = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations.sqlite");
        #[cfg(not(feature = "sqlite"))]
        let default_migrations = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");

        Path::new(default_migrations)
    });

    let migrations = db.migrate(&migrations_path, false).await?;
    for (migration, duration) in migrations {
        log::info!(
            "Migrated {} {} {:?}",
            migration.version,
            migration.description,
            duration
        );
    }

    db.initialize_notification_kinds().await?;

    if config.seed_path.is_some() {
        collab::seed::seed(&config, &db, false).await?;
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

    let filter = EnvFilter::from_str(config.rust_log.as_deref()?).log_err()?;

    tracing_subscriber::registry()
        .with(if config.log_json.unwrap_or(false) {
            Box::new(
                tracing_subscriber::fmt::layer()
                    .fmt_fields(JsonFields::default())
                    .event_format(
                        tracing_subscriber::fmt::format()
                            .json()
                            .flatten_event(true)
                            .with_span_list(false),
                    )
                    .with_filter(filter),
            ) as Box<dyn Layer<_> + Send + Sync>
        } else {
            Box::new(
                tracing_subscriber::fmt::layer()
                    .event_format(tracing_subscriber::fmt::format().pretty())
                    .with_filter(filter),
            )
        })
        .init();

    None
}
