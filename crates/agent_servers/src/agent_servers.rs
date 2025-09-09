mod acp;
mod claude;
mod custom;
mod gemini;

#[cfg(any(test, feature = "test-support"))]
pub mod e2e_tests;

pub use claude::*;
pub use custom::*;
use fs::Fs;
pub use gemini::*;
use project::agent_server_store::AgentServerStore;

use acp_thread::AgentConnection;
use anyhow::Result;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
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
