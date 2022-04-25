mod api;
mod auth;
mod db;
mod env;
mod rpc;

use ::rpc::Peer;
use axum::{body::Body, http::StatusCode, response::IntoResponse, Router};
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

pub async fn run_server(
    state: Arc<AppState>,
    peer: Arc<Peer>,
    listener: TcpListener,
) -> Result<()> {
    let app = Router::<Body>::new();
    // TODO: Compression on API routes?
    // TODO: Authenticate API routes.

    let app = api::add_routes(app, state);
    // TODO: Add rpc routes

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub type Result<T> = std::result::Result<T, Error>;

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
