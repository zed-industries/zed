mod api;
mod auth;
mod db;
mod env;
mod rpc;

#[cfg(test)]
mod integration_tests;

use axum::{body::Body, Router};
use collab::{Error, Result};
use db::{Db, PostgresDb};
use serde::Deserialize;
use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};
use tracing_log::LogTracer;
use tracing_subscriber::{filter::EnvFilter, fmt::format::JsonFields, Layer};
use util::ResultExt;

#[derive(Default, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub database_url: String,
    pub api_token: String,
    pub invite_link_prefix: String,
    pub honeycomb_api_key: Option<String>,
    pub honeycomb_dataset: Option<String>,
    pub rust_log: Option<String>,
    pub log_json: Option<bool>,
}

pub struct AppState {
    db: Arc<dyn Db>,
    api_token: String,
    invite_link_prefix: String,
}

impl AppState {
    async fn new(config: &Config) -> Result<Arc<Self>> {
        let db = PostgresDb::new(&config.database_url, 5).await?;
        let this = Self {
            db: Arc::new(db),
            api_token: config.api_token.clone(),
            invite_link_prefix: config.invite_link_prefix.clone(),
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
    let rpc_server = rpc::Server::new(state.clone(), None);

    let app = Router::<Body>::new()
        .merge(api::routes(&rpc_server, state.clone()))
        .merge(rpc::routes(rpc_server));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
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
