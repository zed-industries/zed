use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, bail};
use collections::HashMap;
use fs::Fs;
use futures::AsyncReadExt;
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Task};
use http_client::{AsyncBody, HttpClient};
use serde::Deserialize;

const REGISTRY_URL: &str =
    "https://github.com/agentclientprotocol/registry/releases/latest/download/registry.json";
const REGISTRY_REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Clone, Debug)]
pub struct RegistryAgent {
    pub id: SharedString,
    pub name: SharedString,
    pub description: SharedString,
    pub version: SharedString,
    pub repository: Option<SharedString>,
    pub icon_path: Option<SharedString>,
    pub targets: HashMap<String, RegistryTargetConfig>,
    pub supports_current_platform: bool,
}

#[derive(Clone, Debug)]
pub struct RegistryTargetConfig {
    pub archive: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub sha256: Option<String>,
    pub env: HashMap<String, String>,
}

struct GlobalAgentRegistryStore(Entity<AgentRegistryStore>);

impl Global for GlobalAgentRegistryStore {}

pub struct AgentRegistryStore {
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    agents: Vec<RegistryAgent>,
    is_fetching: bool,
    fetch_error: Option<SharedString>,
    pending_refresh: Option<Task<()>>,
    _poll_task: Task<Result<()>>,
}

impl AgentRegistryStore {
    pub fn init_global(cx: &mut App) -> Entity<Self> {
        if let Some(store) = Self::try_global(cx) {
            return store;
        }

        let fs = <dyn Fs>::global(cx);
        let http_client: Arc<dyn HttpClient> = cx.http_client();

        let store = cx.new(|cx| Self::new(fs, http_client, cx));
        store.update(cx, |store, cx| {
            store.refresh(cx);
            store.start_polling(cx);
        });
        cx.set_global(GlobalAgentRegistryStore(store.clone()));
        store
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalAgentRegistryStore>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalAgentRegistryStore>()
            .map(|store| store.0.clone())
    }

    pub fn agents(&self) -> &[RegistryAgent] {
        &self.agents
    }

    pub fn agent(&self, id: &str) -> Option<&RegistryAgent> {
        self.agents.iter().find(|agent| agent.id == id)
    }

    pub fn is_fetching(&self) -> bool {
        self.is_fetching
    }

    pub fn fetch_error(&self) -> Option<SharedString> {
        self.fetch_error.clone()
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        if self.pending_refresh.is_some() {
            return;
        }

        self.is_fetching = true;
        self.fetch_error = None;
        cx.notify();

        let fs = self.fs.clone();
        let http_client = self.http_client.clone();

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            let result = match fetch_registry_index(http_client.clone()).await {
                Ok(data) => {
                    build_registry_agents(fs.clone(), http_client, data.index, data.raw_body, true)
                        .await
                }
                Err(error) => Err(error),
            };

            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                this.is_fetching = false;
                match result {
                    Ok(agents) => {
                        this.agents = agents;
                        this.fetch_error = None;
                    }
                    Err(error) => {
                        this.fetch_error = Some(SharedString::from(error.to_string()));
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn new(fs: Arc<dyn Fs>, http_client: Arc<dyn HttpClient>, cx: &mut Context<Self>) -> Self {
        let mut store = Self {
            fs: fs.clone(),
            http_client,
            agents: Vec::new(),
            is_fetching: false,
            fetch_error: None,
            pending_refresh: None,
            _poll_task: Task::ready(Ok(())),
        };

        store.load_cached_registry(fs, store.http_client.clone(), cx);

        store
    }

    fn load_cached_registry(
        &mut self,
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut Context<Self>,
    ) {
        cx.spawn(async move |this, cx| -> Result<()> {
            let cache_path = registry_cache_path();
            if !fs.is_file(&cache_path).await {
                return Ok(());
            }

            let bytes = fs
                .load_bytes(&cache_path)
                .await
                .context("reading cached registry")?;
            let index: RegistryIndex =
                serde_json::from_slice(&bytes).context("parsing cached registry")?;

            let agents = build_registry_agents(fs, http_client, index, bytes, false).await?;

            this.update(cx, |this, cx| {
                this.agents = agents;
                cx.notify();
            })?;

            Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn start_polling(&mut self, cx: &mut Context<Self>) {
        self._poll_task = cx.spawn(async move |this, cx| -> Result<()> {
            loop {
                this.update(cx, |this, cx| this.refresh(cx))?;
                cx.background_executor()
                    .timer(REGISTRY_REFRESH_INTERVAL)
                    .await;
            }
        });
    }
}

struct RegistryFetchResult {
    index: RegistryIndex,
    raw_body: Vec<u8>,
}

async fn fetch_registry_index(http_client: Arc<dyn HttpClient>) -> Result<RegistryFetchResult> {
    let mut response = http_client
        .get(REGISTRY_URL, AsyncBody::default(), true)
        .await
        .context("requesting ACP registry")?;

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("reading ACP registry response")?;

    if response.status().is_client_error() {
        let text = String::from_utf8_lossy(body.as_slice());
        bail!(
            "registry status error {}, response: {text:?}",
            response.status().as_u16()
        );
    }

    let index: RegistryIndex = serde_json::from_slice(&body).context("parsing ACP registry")?;
    Ok(RegistryFetchResult {
        index,
        raw_body: body,
    })
}

async fn build_registry_agents(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    index: RegistryIndex,
    raw_body: Vec<u8>,
    update_cache: bool,
) -> Result<Vec<RegistryAgent>> {
    let cache_dir = registry_cache_dir();
    fs.create_dir(&cache_dir).await?;

    let cache_path = cache_dir.join("registry.json");
    if update_cache {
        fs.write(&cache_path, &raw_body).await?;
    }

    let icons_dir = cache_dir.join("icons");
    if update_cache {
        fs.create_dir(&icons_dir).await?;
    }

    let current_platform = current_platform_key();

    let mut agents = Vec::new();
    for entry in index.agents {
        let Some(binary) = entry.distribution.binary.as_ref() else {
            continue;
        };

        if binary.is_empty() {
            continue;
        }

        let mut targets = HashMap::default();
        for (platform, target) in binary.iter() {
            targets.insert(
                platform.clone(),
                RegistryTargetConfig {
                    archive: target.archive.clone(),
                    cmd: target.cmd.clone(),
                    args: target.args.clone(),
                    sha256: None,
                    env: target.env.clone(),
                },
            );
        }

        let supports_current_platform = current_platform
            .as_ref()
            .is_some_and(|platform| targets.contains_key(*platform));

        let icon_path = resolve_icon_path(
            &entry,
            &icons_dir,
            update_cache,
            fs.clone(),
            http_client.clone(),
        )
        .await?;

        agents.push(RegistryAgent {
            id: entry.id.into(),
            name: entry.name.into(),
            description: entry.description.into(),
            version: entry.version.into(),
            repository: entry.repository.map(Into::into),
            icon_path,
            targets,
            supports_current_platform,
        });
    }

    Ok(agents)
}

async fn resolve_icon_path(
    entry: &RegistryEntry,
    icons_dir: &Path,
    update_cache: bool,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
) -> Result<Option<SharedString>> {
    let icon_url = resolve_icon_url(entry);
    let Some(icon_url) = icon_url else {
        return Ok(None);
    };

    let icon_path = icons_dir.join(format!("{}.svg", entry.id));
    if update_cache && !fs.is_file(&icon_path).await {
        if let Err(error) = download_icon(fs.clone(), http_client, &icon_url, entry).await {
            log::warn!(
                "Failed to download ACP registry icon for {}: {error:#}",
                entry.id
            );
        }
    }

    if fs.is_file(&icon_path).await {
        Ok(Some(SharedString::from(
            icon_path.to_string_lossy().into_owned(),
        )))
    } else {
        Ok(None)
    }
}

async fn download_icon(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    icon_url: &str,
    entry: &RegistryEntry,
) -> Result<()> {
    let mut response = http_client
        .get(icon_url, AsyncBody::default(), true)
        .await
        .with_context(|| format!("requesting icon for {}", entry.id))?;

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .with_context(|| format!("reading icon for {}", entry.id))?;

    if response.status().is_client_error() {
        let text = String::from_utf8_lossy(body.as_slice());
        bail!(
            "icon status error {}, response: {text:?}",
            response.status().as_u16()
        );
    }

    let icon_path = registry_cache_dir()
        .join("icons")
        .join(format!("{}.svg", entry.id));
    fs.write(&icon_path, &body).await?;
    Ok(())
}

fn resolve_icon_url(entry: &RegistryEntry) -> Option<String> {
    let icon = entry.icon.as_ref()?;
    if icon.starts_with("https://") || icon.starts_with("http://") {
        return Some(icon.to_string());
    }

    let relative_icon = icon.trim_start_matches("./");
    Some(format!(
        "https://raw.githubusercontent.com/agentclientprotocol/registry/main/{}/{relative_icon}",
        entry.id
    ))
}

fn current_platform_key() -> Option<&'static str> {
    let os = if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        return None;
    };

    let arch = if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        return None;
    };

    Some(match os {
        "darwin" => match arch {
            "aarch64" => "darwin-aarch64",
            "x86_64" => "darwin-x86_64",
            _ => return None,
        },
        "linux" => match arch {
            "aarch64" => "linux-aarch64",
            "x86_64" => "linux-x86_64",
            _ => return None,
        },
        "windows" => match arch {
            "aarch64" => "windows-aarch64",
            "x86_64" => "windows-x86_64",
            _ => return None,
        },
        _ => return None,
    })
}

fn registry_cache_dir() -> PathBuf {
    paths::external_agents_dir().join("registry")
}

fn registry_cache_path() -> PathBuf {
    registry_cache_dir().join("registry.json")
}

#[derive(Deserialize)]
struct RegistryIndex {
    #[serde(rename = "version")]
    _version: String,
    agents: Vec<RegistryEntry>,
    #[serde(rename = "extensions")]
    _extensions: Vec<RegistryEntry>,
}

#[derive(Deserialize)]
struct RegistryEntry {
    id: String,
    name: String,
    version: String,
    description: String,
    #[serde(default)]
    repository: Option<String>,
    #[serde(default)]
    icon: Option<String>,
    distribution: RegistryDistribution,
}

#[derive(Deserialize)]
struct RegistryDistribution {
    #[serde(default)]
    binary: Option<HashMap<String, RegistryBinaryTarget>>,
}

#[derive(Deserialize)]
struct RegistryBinaryTarget {
    archive: String,
    cmd: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}
