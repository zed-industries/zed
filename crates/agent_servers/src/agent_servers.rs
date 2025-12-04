mod acp;
mod claude;
mod codex;
mod custom;
mod gemini;

#[cfg(any(test, feature = "test-support"))]
pub mod e2e_tests;

pub use claude::*;
use client::ProxySettings;
pub use codex::*;
use collections::HashMap;
pub use custom::*;
use fs::Fs;
pub use gemini::*;
use http_client::read_no_proxy_from_env;
use project::agent_server_store::AgentServerStore;

use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::Project;
use settings::SettingsStore;
use std::{any::Any, path::Path, rc::Rc, sync::Arc};

pub use acp::AcpConnection;

pub struct AgentServerDelegate {
    store: Entity<AgentServerStore>,
    project: Entity<Project>,
    status_tx: Option<watch::Sender<SharedString>>,
    new_version_available: Option<watch::Sender<Option<String>>>,
}

impl AgentServerDelegate {
    pub fn new(
        store: Entity<AgentServerStore>,
        project: Entity<Project>,
        status_tx: Option<watch::Sender<SharedString>>,
        new_version_tx: Option<watch::Sender<Option<String>>>,
    ) -> Self {
        Self {
            store,
            project,
            status_tx,
            new_version_available: new_version_tx,
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }
}

pub trait AgentServer: Send {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> SharedString;
    fn telemetry_id(&self) -> &'static str;
    fn default_mode(&self, _cx: &mut App) -> Option<agent_client_protocol::SessionModeId> {
        None
    }
    fn set_default_mode(
        &self,
        _mode_id: Option<agent_client_protocol::SessionModeId>,
        _fs: Arc<dyn Fs>,
        _cx: &mut App,
    ) {
    }

    fn default_model(&self, _cx: &mut App) -> Option<agent_client_protocol::ModelId> {
        None
    }

    fn set_default_model(
        &self,
        _model_id: Option<agent_client_protocol::ModelId>,
        _fs: Arc<dyn Fs>,
        _cx: &mut App,
    ) {
    }

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>>;

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl dyn AgentServer {
    pub fn downcast<T: 'static + AgentServer + Sized>(self: Rc<Self>) -> Option<Rc<T>> {
        self.into_any().downcast().ok()
    }
}

/// Load the default proxy environment variables to pass through to the agent
pub fn load_proxy_env(cx: &mut App) -> HashMap<String, String> {
    let proxy_url = cx
        .read_global(|settings: &SettingsStore, _| settings.get::<ProxySettings>(None).proxy_url());
    let mut env = HashMap::default();

    if let Some(proxy_url) = &proxy_url {
        let env_var = if proxy_url.scheme() == "https" {
            "HTTPS_PROXY"
        } else {
            "HTTP_PROXY"
        };
        env.insert(env_var.to_owned(), proxy_url.to_string());
    }

    if let Some(no_proxy) = read_no_proxy_from_env() {
        env.insert("NO_PROXY".to_owned(), no_proxy);
    } else if proxy_url.is_some() {
        // We sometimes need local MCP servers that we don't want to proxy
        env.insert("NO_PROXY".to_owned(), "localhost,127.0.0.1".to_owned());
    }

    env
}
