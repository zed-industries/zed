mod api;
mod assets;
mod auth;
mod db;
mod env;
mod errors;
mod expiring;
mod github;
mod rpc;

use ::rpc::Peer;
use async_std::net::TcpListener;
use async_trait::async_trait;
use db::{Db, PostgresDb};
use handlebars::Handlebars;
use parking_lot::RwLock;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::sync::Arc;
use surf::http::cookies::SameSite;
use tide::sessions::SessionMiddleware;
use tide_compress::CompressMiddleware;

type Request = tide::Request<Arc<AppState>>;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Templates;

#[derive(Default, Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub database_url: String,
    pub session_secret: String,
    pub github_app_id: usize,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_private_key: String,
    pub api_token: String,
}

pub struct AppState {
    db: Arc<dyn Db>,
    handlebars: RwLock<Handlebars<'static>>,
    config: Config,
}

impl AppState {
    async fn new(config: Config) -> tide::Result<Arc<Self>> {
        let db = PostgresDb::new(&config.database_url, 5).await?;

        let this = Self {
            db: Arc::new(db),
            handlebars: Default::default(),
            config,
        };
        this.register_partials();
        Ok(Arc::new(this))
    }

    fn register_partials(&self) {
        for path in Templates::iter() {
            if let Some(partial_name) = path
                .strip_prefix("partials/")
                .and_then(|path| path.strip_suffix(".hbs"))
            {
                let partial = Templates::get(path.as_ref()).unwrap();
                self.handlebars
                    .write()
                    .register_partial(partial_name, std::str::from_utf8(&partial.data).unwrap())
                    .unwrap()
            }
        }
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
    web.with(
        SessionMiddleware::new(
            db::SessionStore::new_with_table_name(&state.config.database_url, "sessions")
                .await
                .unwrap(),
            state.config.session_secret.as_bytes(),
        )
        .with_same_site_policy(SameSite::Lax), // Required obtain our session in /auth_callback
    );
    api::add_routes(&mut web);

    let mut assets = tide::new();
    assets.with(CompressMiddleware::new());
    assets::add_routes(&mut assets);

    let mut app = tide::with_state(state.clone());
    rpc::add_routes(&mut app, &rpc);

    app.at("/").nest(web);
    app.at("/static").nest(assets);

    app.listen(listener).await?;

    Ok(())
}
