use anyhow::{Context as _, Result, anyhow, bail};
use collections::BTreeMap;
use context_server::registry::ServerListResponse;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

use crate::DisableAiSettings;
use fs::Fs;
use futures::AsyncReadExt;
use gpui::{
    App, AppContext as _, BackgroundExecutor, Context, Entity, FutureExt as _, Global,
    SharedString, Task, TaskExt as _,
};
use http_client::{AsyncBody, HttpClient, StatusCode};

const REGISTRY_URL: &str = "https://registry.modelcontextprotocol.io/v0.1/servers";
const MAX_PAGES: u32 = 50;
const INDEX_VERSION: u32 = 1;
const REGISTRY_FETCH_TIMEOUT: Duration = Duration::from_secs(30);

fn registry_cache_dir() -> PathBuf {
    paths::mcp_registry_dir().clone()
}

fn registry_cache_path() -> PathBuf {
    registry_cache_dir().join("index.json")
}

#[derive(Serialize, Deserialize)]
pub struct McpRegistryIndex {
    pub version: u32,
    pub servers: BTreeMap<String, CachedRegistryServer>,
}

#[derive(Serialize, Deserialize)]
pub struct CachedRegistryServer {
    pub server: context_server::registry::RegistryServer,
    pub official: Option<context_server::registry::RegistryOfficial>,
}

struct GlobalMcpRegistryStore(Entity<McpRegistryStore>);

impl Global for GlobalMcpRegistryStore {}

pub struct McpRegistryStore {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    is_fetching: bool,
    fetch_error: Option<SharedString>,
    pending_refresh: Option<Task<()>>,
    last_refresh: Option<Instant>,
    index: McpRegistryIndex,
}

impl McpRegistryStore {
    /// Initialize the global McpRegistryStore.
    ///
    /// This loads the cached registry from disk. If the cache is empty, it
    /// will trigger a network fetch. Otherwise, call `refresh()` explicitly
    /// when you need fresh data (e.g., when opening the MCP Registry page).
    pub fn init_global(
        cx: &mut App,
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
    ) -> Entity<Self> {
        if let Some(store) = Self::try_global(cx) {
            return store;
        }

        let store = cx.new(|cx| Self::new(fs, http_client, cx));
        cx.set_global(GlobalMcpRegistryStore(store.clone()));

        store.update(cx, |store, cx| {
            if store.index.servers.is_empty() {
                store.refresh(cx);
            }
        });

        store
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalMcpRegistryStore>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalMcpRegistryStore>()
            .map(|store| store.0.clone())
    }

    fn new(fs: Arc<dyn Fs>, http_client: Arc<dyn HttpClient>, cx: &mut Context<Self>) -> Self {
        let mut store = Self {
            fs: fs.clone(),
            http_client,
            index: McpRegistryIndex {
                version: INDEX_VERSION,
                servers: BTreeMap::new(),
            },
            is_fetching: false,
            fetch_error: None,
            pending_refresh: None,
            last_refresh: None,
        };

        store.load_cached_registry(fs, cx);

        store
    }

    fn load_cached_registry(&mut self, fs: Arc<dyn Fs>, cx: &mut Context<Self>) {
        if DisableAiSettings::get_global(cx).disable_ai {
            return;
        }

        cx.spawn(async move |this, cx| -> Result<()> {
            let cache_path = registry_cache_path();
            if !fs.is_file(&cache_path).await {
                return Ok(());
            }

            let bytes = fs
                .load_bytes(&cache_path)
                .await
                .context("reading cached registry")?;
            let index: McpRegistryIndex =
                serde_json::from_slice(&bytes).context("parsing cached registry")?;

            if index.version != INDEX_VERSION {
                log::info!(
                    "discarding MCP registry cache with version {} (expected {INDEX_VERSION})",
                    index.version
                );
                return Ok(());
            }

            this.update(cx, |this, cx| {
                this.index = index;
                cx.notify();
            })?;

            Ok(())
        })
        .detach_and_log_err(cx);
    }

    /// Refresh the registry from the network.
    ///
    /// This will fetch the latest registry data and update the cache.
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        if self.pending_refresh.is_some() {
            return;
        }

        if DisableAiSettings::get_global(cx).disable_ai {
            return;
        }

        self.is_fetching = true;
        self.fetch_error = None;
        self.last_refresh = Some(Instant::now());
        cx.notify();

        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        let executor = cx.background_executor().clone();

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            let result = match fetch_registry_index(http_client.clone(), &executor).await {
                Ok(index) => cache_registry_index(fs, &index).await.map(|_| index),
                Err(error) => {
                    log::error!("McpRegistryStore::refresh: fetch failed: {error:#}");
                    Err(error)
                }
            };

            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                this.is_fetching = false;
                match result {
                    Ok(index) => {
                        this.index = index;
                        this.fetch_error = None;
                    }
                    Err(error) => {
                        this.fetch_error = Some(SharedString::from(format!("{error:#}")));
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }
}

async fn fetch_registry_index(
    http_client: Arc<dyn HttpClient>,
    executor: &BackgroundExecutor,
) -> Result<McpRegistryIndex> {
    let mut servers = BTreeMap::new();
    let mut cursor: Option<String> = None;

    for _ in 0..MAX_PAGES {
        let mut url = Url::parse(REGISTRY_URL)?;

        url.query_pairs_mut()
            .append_pair("limit", "100")
            .append_pair("version", "latest");

        if let Some(cursor) = &cursor {
            url.query_pairs_mut().append_pair("cursor", cursor);
        }

        let (status, body) = fetch_url_body(
            http_client.clone(),
            url.as_str(),
            REGISTRY_FETCH_TIMEOUT,
            executor,
        )
        .await
        .context("fetch MCP Registry")?;

        if status.is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "registry status error {}, response: {text:?}",
                status.as_u16()
            );
        }

        let page: ServerListResponse =
            serde_json::from_slice(&body).context("parsing MCP registry")?;

        for entry in page.servers {
            servers.insert(
                entry.server.name.clone(),
                CachedRegistryServer {
                    server: entry.server,
                    official: entry.meta.and_then(|m| m.official),
                },
            );
        }

        cursor = page.metadata.next_cursor.filter(|c| !c.is_empty());
        if cursor.is_none() {
            return Ok(McpRegistryIndex {
                version: INDEX_VERSION,
                servers,
            });
        }
    }

    bail!("registry pagination did not terminate after {MAX_PAGES} pages");
}

/// Persists a fetched index to the on-disk cache.
///
/// Unlike the ACP registry, which downloads a single JSON document, this index
/// is assembled from many paginated responses, so the parsed form is cached
/// rather than a raw response body.
async fn cache_registry_index(fs: Arc<dyn Fs>, index: &McpRegistryIndex) -> Result<()> {
    fs.create_dir(&registry_cache_dir()).await?;

    let bytes = serde_json::to_vec_pretty(index).context("serializing MCP registry index")?;
    fs.write(&registry_cache_path(), bytes.as_slice())
        .await
        .context("writing MCP registry index")?;

    Ok(())
}

async fn fetch_url_body(
    http_client: Arc<dyn HttpClient>,
    url: &str,
    timeout: Duration,
    executor: &BackgroundExecutor,
) -> Result<(StatusCode, Vec<u8>)> {
    async {
        let mut response = http_client
            .get(url, AsyncBody::default(), true)
            .await
            .with_context(|| format!("requesting {url}"))?;

        let status = response.status();
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("reading response from {url}"))?;

        Ok((status, body))
    }
    .with_timeout(timeout, executor)
    .await
    .map_err(|_| {
        anyhow!(
            "timed out after {}s while fetching {url}",
            timeout.as_secs()
        )
    })?
}
