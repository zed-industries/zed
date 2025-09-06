mod acp;
mod claude;
mod custom;
mod gemini;
mod settings;

#[cfg(any(test, feature = "test-support"))]
pub mod e2e_tests;

use anyhow::Context as _;
pub use claude::*;
pub use custom::*;
use fs::Fs;
use fs::RemoveOptions;
use fs::RenameOptions;
use futures::StreamExt as _;
pub use gemini::*;
use gpui::AppContext;
use node_runtime::NodeRuntime;
use project::agent_server_store::AgentServerStore;
pub use settings::*;

use acp_thread::AgentConnection;
use acp_thread::LoadError;
use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use gpui::{App, AsyncApp, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr as _;
use std::{
    any::Any,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use util::ResultExt as _;

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

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>>;

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl dyn AgentServer {
    pub fn downcast<T: 'static + AgentServer + Sized>(self: Rc<Self>) -> Option<Rc<T>> {
        self.into_any().downcast().ok()
    }
}
