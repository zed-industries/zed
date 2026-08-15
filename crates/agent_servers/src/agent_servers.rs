mod acp;
mod custom;

#[cfg(any(test, feature = "test-support"))]
pub mod e2e_tests;

use client::ProxySettings;
use collections::{HashMap, HashSet};
pub use custom::*;
use fs::Fs;
use http_client::read_no_proxy_from_env;
use project::{AgentId, Project, agent_server_store::AgentServerStore};

use acp_thread::AgentConnection;
use agent_client_protocol::schema::v1 as acp_schema;
use anyhow::Result;
use gpui::{App, AppContext, Entity, Task};
use settings::{AgentConfigOptionValue, SettingsStore};
use std::{any::Any, rc::Rc, sync::Arc};

#[cfg(any(test, feature = "test-support"))]
pub use acp::test_support::{
    FakeAcpAgentServer, FakeAcpConnectionHarness, connect_fake_acp_connection,
};
pub use acp::{
    AcpConnection, AcpDebugMessage, AcpDebugMessageContent, AcpDebugMessageDirection,
    GEMINI_TERMINAL_AUTH_METHOD_ID,
};

pub struct AgentServerDelegate {
    store: Entity<AgentServerStore>,
    new_version_available: Option<watch::Sender<Option<String>>>,
    loading_status: Option<watch::Sender<Option<String>>>,
}

impl AgentServerDelegate {
    pub fn new(
        store: Entity<AgentServerStore>,
        new_version_tx: Option<watch::Sender<Option<String>>>,
        loading_status_tx: Option<watch::Sender<Option<String>>>,
    ) -> Self {
        Self {
            store,
            new_version_available: new_version_tx,
            loading_status: loading_status_tx,
        }
    }
}

pub trait AgentServer: Send {
    fn logo(&self) -> ui::IconName;
    fn agent_id(&self) -> AgentId;
    fn connect(
        &self,
        delegate: AgentServerDelegate,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>>;

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;

    fn default_mode(&self, _cx: &App) -> Option<acp_schema::SessionModeId> {
        None
    }

    fn set_default_mode(
        &self,
        _mode_id: Option<acp_schema::SessionModeId>,
        _fs: Arc<dyn Fs>,
        _cx: &mut App,
    ) {
    }

    fn default_config_option(&self, _config_id: &str, _cx: &App) -> Option<AgentConfigOptionValue> {
        None
    }

    fn set_default_config_option(
        &self,
        _config_id: &str,
        _value: Option<AgentConfigOptionValue>,
        _fs: Arc<dyn Fs>,
        _cx: &mut App,
    ) {
    }

    fn favorite_config_option_value_ids(
        &self,
        _config_id: &acp_schema::SessionConfigId,
        _cx: &mut App,
    ) -> HashSet<acp_schema::SessionConfigValueId> {
        HashSet::default()
    }

    fn toggle_favorite_config_option_value(
        &self,
        _config_id: acp_schema::SessionConfigId,
        _value_id: acp_schema::SessionConfigValueId,
        _should_be_favorite: bool,
        _fs: Arc<dyn Fs>,
        _cx: &App,
    ) {
    }
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
        env.insert("NO_PROXY".to_owned(), "localhost,127.0.0.1".to_owned());
    }

    env
}

/// Restart policy for actor supervised child processes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupervisorRestartPolicy {
    OneForOne,
    OneForAll,
    ExponentialBackoff { max_retries: u32 },
}

/// Status of a managed actor process
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorProcessStatus {
    Starting,
    Running { pid: u32 },
    Failed { reason: String },
    Terminated,
}

/// Erlang-style process supervisor for LSP, MCP, and ACP agent subprocesses
pub struct ActorSupervisor {
    name: String,
    policy: SupervisorRestartPolicy,
    process_status: HashMap<String, ActorProcessStatus>,
}

impl ActorSupervisor {
    pub fn new(name: impl Into<String>, policy: SupervisorRestartPolicy) -> Self {
        Self {
            name: name.into(),
            policy,
            process_status: HashMap::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn policy(&self) -> SupervisorRestartPolicy {
        self.policy
    }

    pub fn register_process(&mut self, process_id: impl Into<String>, status: ActorProcessStatus) {
        self.process_status.insert(process_id.into(), status);
    }

    pub fn get_status(&self, process_id: &str) -> Option<&ActorProcessStatus> {
        self.process_status.get(process_id)
    }
}
