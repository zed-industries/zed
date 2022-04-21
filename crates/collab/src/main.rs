mod api;
mod auth;
mod db;
mod env;
mod errors;
mod github;
mod rpc;

use ::rpc::Peer;
use async_std::net::TcpListener;
use async_trait::async_trait;
use db::{Db, PostgresDb};
use serde::Deserialize;
use std::sync::Arc;
use tide_compress::CompressMiddleware;

type Request = tide::Request<Arc<AppState>>;

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
    async fn new(config: Config) -> tide::Result<Arc<Self>> {
        let db = PostgresDb::new(&config.database_url, 5).await?;

        let this = Self {
            db: Arc::new(db),
            config,
        };
        Ok(Arc::new(this))
    }
}

#[async_trait]
trait RequestExt {
    fn db(&self) -> &Arc<dyn Db>;
}

#[async_trait]
impl RequestExt for Request {
    fn db(&self) -> &Arc<dyn Db> {
        &self.state().db
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    if std::env::var("LOG_JSON").is_ok() {
        json_env_logger::init();
    } else {
        tide::log::start();
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
        TcpListener::bind(&format!("0.0.0.0:{}", state.config.http_port)).await?,
    )
    .await?;
    Ok(())
}

pub async fn run_server(
    state: Arc<AppState>,
    rpc: Arc<Peer>,
    listener: TcpListener,
) -> tide::Result<()> {
    let mut web = tide::with_state(state.clone());
    web.with(CompressMiddleware::new());
    api::add_routes(&mut web);

    let mut app = tide::with_state(state.clone());
    rpc::add_routes(&mut app, &rpc);

    app.at("/").nest(web);

    app.listen(listener).await?;

    Ok(())
}
