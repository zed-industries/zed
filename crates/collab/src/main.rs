mod api;
mod auth;
mod db;
mod env;
mod rpc;

use axum::{body::Body, http::StatusCode, response::IntoResponse, Router};
use db::{Db, PostgresDb};
use serde::Deserialize;
use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};
use tracing_log::LogTracer;
use tracing_subscriber::filter::EnvFilter;
use util::ResultExt;

#[derive(Default, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub database_url: String,
    pub api_token: String,
    pub honeycomb_api_key: Option<String>,
    pub honeycomb_dataset: Option<String>,
    pub rust_log: Option<String>,
}

pub struct AppState {
    db: Arc<dyn Db>,
    api_token: String,
}

impl AppState {
    async fn new(config: &Config) -> Result<Arc<Self>> {
        let db = PostgresDb::new(&config.database_url, 5).await?;
        let this = Self {
            db: Arc::new(db),
            api_token: config.api_token.clone(),
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

    let config = envy::from_env::<Config>().expect("error loading config");
    init_tracing(&config);
    let state = AppState::new(&config).await?;

    let listener = TcpListener::bind(&format!("0.0.0.0:{}", config.http_port))
        .expect("failed to bind TCP listener");

    let app = Router::<Body>::new()
        .merge(api::routes(state.clone()))
        .merge(rpc::routes(state));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Http(StatusCode, String),
    Internal(anyhow::Error),
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self::Internal(error.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Http(code, message) => (code, message).into_response(),
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
            Error::Internal(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message) => write!(f, "{code}: {message}"),
            Error::Internal(error) => error.fmt(f),
        }
    }
}

pub fn init_tracing(config: &Config) -> Option<()> {
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::WithExportConfig;
    use std::str::FromStr;
    use tracing_opentelemetry::OpenTelemetryLayer;
    use tracing_subscriber::layer::SubscriberExt;
    let rust_log = config.rust_log.clone()?;

    LogTracer::init().log_err()?;

    let open_telemetry_layer = config
        .honeycomb_api_key
        .clone()
        .zip(config.honeycomb_dataset.clone())
        .map(|(honeycomb_api_key, honeycomb_dataset)| {
            let mut metadata = tonic::metadata::MetadataMap::new();
            metadata.insert("x-honeycomb-team", honeycomb_api_key.parse().unwrap());
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint("https://api.honeycomb.io")
                        .with_metadata(metadata),
                )
                .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
                    opentelemetry::sdk::Resource::new(vec![KeyValue::new(
                        "service.name",
                        honeycomb_dataset,
                    )]),
                ))
                .install_batch(opentelemetry::runtime::Tokio)
                .expect("failed to initialize tracing");

            OpenTelemetryLayer::new(tracer)
        });

    let subscriber = tracing_subscriber::Registry::default()
        .with(open_telemetry_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(tracing_subscriber::fmt::format().pretty()),
        )
        .with(EnvFilter::from_str(rust_log.as_str()).log_err()?);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    None
}
