mod api;
mod auth;
mod db;
mod env;
mod rpc;

use ::rpc::Peer;
use anyhow::Result;
use axum::{body::Body, http::StatusCode, Router};
use db::{Db, PostgresDb};

use serde::Deserialize;
use std::{net::TcpListener, sync::Arc};

// type Request = tide::Request<Arc<AppState>>;

#[derive(Default, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub database_url: String,
    pub api_token: String,
}

pub struct AppState {
    db: Arc<dyn Db>,
    config: Config,
}

impl AppState {
    async fn new(config: Config) -> Result<Arc<Self>> {
        let db = PostgresDb::new(&config.database_url, 5).await?;

        let this = Self {
            db: Arc::new(db),
            config,
        };
        Ok(Arc::new(this))
    }
}

// trait RequestExt {
//     fn db(&self) -> &Arc<dyn Db>;
// }

// impl RequestExt for Request<Body> {
//     fn db(&self) -> &Arc<dyn Db> {
//         &self.data::<Arc<AppState>>().unwrap().db
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("LOG_JSON").is_ok() {
        json_env_logger::init();
    } else {
        env_logger::init();
    }

    if let Err(error) = env::load_dotenv() {
        log::error!(
            "error loading .env.toml (this is expected in production): {}",
            error
        );
    }

    let config = envy::from_env::<Config>().expect("error loading config");
    let state = AppState::new(config).await?;
    let rpc = Peer::new();
    run_server(
        state.clone(),
        rpc,
        TcpListener::bind(&format!("0.0.0.0:{}", state.config.http_port))
            .expect("failed to bind TCP listener"),
    )
    .await?;
    Ok(())
}

async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}

pub async fn run_server(
    state: Arc<AppState>,
    peer: Arc<Peer>,
    listener: TcpListener,
) -> Result<()> {
    let app = Router::<Body>::new();
    // TODO: Assign app state to request somehow
    // TODO: Compression on API routes?
    // TODO: Authenticate API routes.

    let app = api::add_routes(app);
    // TODO: Add rpc routes

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
