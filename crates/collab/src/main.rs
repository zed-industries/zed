use anyhow::anyhow;
use axum::headers::HeaderMapExt;
use axum::{
    extract::MatchedPath,
    http::{Request, Response},
    routing::get,
    Extension, Router,
};
use collab::api::CloudflareIpCountryHeader;
use collab::llm::{db::LlmDatabase, log_usage_periodically};
use collab::migrations::run_database_migrations;
use collab::user_backfiller::spawn_user_backfiller;
use collab::{api::billing::poll_stripe_events_periodically, llm::LlmState, ServiceMode};
use collab::{
    api::fetch_extensions_from_blob_store_periodically, db, env, executor::Executor,
    rpc::ResultExt, AppState, Config, RateLimiter, Result,
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
use util::ResultExt as _;

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
            setup_app_database(&config).await?;
        }
        Some("seed") => {
            let config = envy::from_env::<Config>().expect("error loading config");
            let db_options = db::ConnectOptions::new(config.database_url.clone());

            let mut db = Database::new(db_options, Executor::Production).await?;
            db.initialize_notification_kinds().await?;

            collab::seed::seed(&config, &db, false).await?;

            if let Some(llm_database_url) = config.llm_database_url.clone() {
                let db_options = db::ConnectOptions::new(llm_database_url);
                let mut db = LlmDatabase::new(db_options.clone(), Executor::Production).await?;
                db.initialize().await?;
                collab::llm::db::seed_database(&config, &mut db, true).await?;
            }
        }
        Some("serve") => {
            let mode = match args.next().as_deref() {
                Some("collab") => ServiceMode::Collab,
                Some("api") => ServiceMode::Api,
                Some("llm") => ServiceMode::Llm,
                Some("all") => ServiceMode::All,
                _ => {
                    return Err(anyhow!(
                        "usage: collab <version | migrate | seed | serve <api|collab|llm|all>>"
                    ))?;
                }
            };

            let config = envy::from_env::<Config>().expect("error loading config");
            init_tracing(&config);
            let mut app = Router::new()
                .route("/", get(handle_root))
                .route("/healthz", get(handle_liveness_probe))
                .layer(Extension(mode));

            let listener = TcpListener::bind(&format!("0.0.0.0:{}", config.http_port))
                .expect("failed to bind TCP listener");

            let mut on_shutdown = None;

            if mode.is_llm() {
                setup_llm_database(&config).await?;

                let state = LlmState::new(config.clone(), Executor::Production).await?;

                log_usage_periodically(state.clone());

                app = app
                    .merge(collab::llm::routes())
                    .layer(Extension(state.clone()));
            }

            if mode.is_collab() || mode.is_api() {
                setup_app_database(&config).await?;

                let state = AppState::new(config, Executor::Production).await?;

                if mode.is_collab() {
                    state.db.purge_old_embeddings().await.trace_err();
                    RateLimiter::save_periodically(
                        state.rate_limiter.clone(),
                        state.executor.clone(),
                    );

                    let epoch = state
                        .db
                        .create_server(&state.config.zed_environment)
                        .await?;
                    let rpc_server = collab::rpc::Server::new(epoch, state.clone());
                    rpc_server.start().await?;

                    app = app
                        .merge(collab::api::routes(rpc_server.clone()))
                        .merge(collab::rpc::routes(rpc_server.clone()));

                    on_shutdown = Some(Box::new(move || rpc_server.teardown()));
                }

                if mode.is_api() {
                    poll_stripe_events_periodically(state.clone());
                    fetch_extensions_from_blob_store_periodically(state.clone());
                    spawn_user_backfiller(state.clone());

                    app = app
                        .merge(collab::api::events::router())
                        .merge(collab::api::extensions::router())
                }

                app = app.layer(Extension(state.clone()));
            }

            app = app.layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        let geoip_country_code = request
                            .headers()
                            .typed_get::<CloudflareIpCountryHeader>()
                            .map(|header| header.to_string());

                        tracing::info_span!(
                            "http_request",
                            method = ?request.method(),
                            matched_path,
                            geoip_country_code,
                            user_id = tracing::field::Empty,
                            login = tracing::field::Empty,
                            authn.jti = tracing::field::Empty,
                            is_staff = tracing::field::Empty
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
            let signal = async move {
                let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())
                    .expect("failed to listen for interrupt signal");
                let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())
                    .expect("failed to listen for interrupt signal");
                let sigterm = sigterm.recv();
                let sigint = sigint.recv();
                futures::pin_mut!(sigterm, sigint);
                futures::future::select(sigterm, sigint).await;
            };

            #[cfg(windows)]
            let signal = async move {
                // todo(windows):
                // `ctrl_close` does not work well, because tokio's signal handler always returns soon,
                // but system terminates the application soon after returning CTRL+CLOSE handler.
                // So we should implement blocking handler to treat CTRL+CLOSE signal.
                let mut ctrl_break = tokio::signal::windows::ctrl_break()
                    .expect("failed to listen for interrupt signal");
                let mut ctrl_c = tokio::signal::windows::ctrl_c()
                    .expect("failed to listen for interrupt signal");
                let ctrl_break = ctrl_break.recv();
                let ctrl_c = ctrl_c.recv();
                futures::pin_mut!(ctrl_break, ctrl_c);
                futures::future::select(ctrl_break, ctrl_c).await;
            };

            axum::Server::from_tcp(listener)
                .map_err(|e| anyhow!(e))?
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(async move {
                    signal.await;
                    tracing::info!("Received interrupt signal");

                    if let Some(on_shutdown) = on_shutdown {
                        on_shutdown();
                    }
                })
                .await
                .map_err(|e| anyhow!(e))?;
        }
        _ => {
            Err(anyhow!(
                "usage: collab <version | migrate | seed | serve <api|collab|llm|all>>"
            ))?;
        }
    }
    Ok(())
}

async fn setup_app_database(config: &Config) -> Result<()> {
    let db_options = db::ConnectOptions::new(config.database_url.clone());
    let mut db = Database::new(db_options, Executor::Production).await?;

    let migrations_path = config.migrations_path.as_deref().unwrap_or_else(|| {
        #[cfg(feature = "sqlite")]
        let default_migrations = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations.sqlite");
        #[cfg(not(feature = "sqlite"))]
        let default_migrations = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");

        Path::new(default_migrations)
    });

    let migrations = run_database_migrations(db.options(), migrations_path).await?;
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

    Ok(())
}

async fn setup_llm_database(config: &Config) -> Result<()> {
    let database_url = config
        .llm_database_url
        .as_ref()
        .ok_or_else(|| anyhow!("missing LLM_DATABASE_URL"))?;

    let db_options = db::ConnectOptions::new(database_url.clone());
    let db = LlmDatabase::new(db_options, Executor::Production).await?;

    let migrations_path = config
        .llm_database_migrations_path
        .as_deref()
        .unwrap_or_else(|| {
            #[cfg(feature = "sqlite")]
            let default_migrations = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations_llm.sqlite");
            #[cfg(not(feature = "sqlite"))]
            let default_migrations = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations_llm");

            Path::new(default_migrations)
        });

    let migrations = run_database_migrations(db.options(), migrations_path).await?;
    for (migration, duration) in migrations {
        log::info!(
            "Migrated {} {} {:?}",
            migration.version,
            migration.description,
            duration
        );
    }

    Ok(())
}

async fn handle_root(Extension(mode): Extension<ServiceMode>) -> String {
    format!("zed:{mode} v{VERSION} ({})", REVISION.unwrap_or("unknown"))
}

async fn handle_liveness_probe(
    app_state: Option<Extension<Arc<AppState>>>,
    llm_state: Option<Extension<Arc<LlmState>>>,
) -> Result<String> {
    if let Some(state) = app_state {
        state.db.get_all_users(0, 1).await?;
    }

    if let Some(llm_state) = llm_state {
        llm_state.db.list_providers().await?;
    }

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
