use std::{error::Error, fmt, path::Path, rc::Rc};

use agent_client_protocol::{self as acp};
use anyhow::Result;
use collections::IndexMap;
use gpui::{AsyncApp, Entity, SharedString, Task};
use project::Project;
use ui::{App, IconName};

use crate::AcpThread;

pub trait AgentConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<AcpThread>>>;

    fn auth_methods(&self) -> &[acp::AuthMethod];

    fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>>;

    fn prompt(&self, params: acp::PromptRequest, cx: &mut App)
    -> Task<Result<acp::PromptResponse>>;

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);

    /// Returns this agent as an [Rc<dyn ModelSelector>] if the model selection capability is supported.
    ///
    /// If the agent does not support model selection, returns [None].
    /// This allows sharing the selector in UI components.
    fn model_selector(&self) -> Option<Rc<dyn AgentModelSelector>> {
        None
    }
}

#[derive(Debug)]
pub struct AuthRequired;

impl Error for AuthRequired {}
impl fmt::Display for AuthRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthRequired")
    }
}

/// Trait for agents that support listing, selecting, and querying language models.
///
/// This is an optional capability; agents indicate support via [AgentConnection::model_selector].
pub trait AgentModelSelector: 'static {
    /// Lists all available language models for this agent.
    ///
    /// # Parameters
    /// - `cx`: The GPUI app context for async operations and global access.
    ///
    /// # Returns
    /// A task resolving to the list of models or an error (e.g., if no models are configured).
    fn list_models(&self, cx: &mut App) -> Task<Result<AgentModelList>>;

    /// Selects a model for a specific session (thread).
    ///
    /// This sets the default model for future interactions in the session.
    /// If the session doesn't exist or the model is invalid, it returns an error.
    ///
    /// # Parameters
    /// - `session_id`: The ID of the session (thread) to apply the model to.
    /// - `model`: The model to select (should be one from [list_models]).
    /// - `cx`: The GPUI app context.
    ///
    /// # Returns
    /// A task resolving to `Ok(())` on success or an error.
    fn select_model(
        &self,
        session_id: acp::SessionId,
        model_id: AgentModelId,
        cx: &mut App,
    ) -> Task<Result<()>>;

    /// Retrieves the currently selected model for a specific session (thread).
    ///
    /// # Parameters
    /// - `session_id`: The ID of the session (thread) to query.
    /// - `cx`: The GPUI app context.
    ///
    /// # Returns
    /// A task resolving to the selected model (always set) or an error (e.g., session not found).
    fn selected_model(
        &self,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<Result<AgentModelInfo>>;

    /// Whenever the model list is updated the receiver will be notified.
    fn watch(&self, cx: &mut App) -> watch::Receiver<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentModelId(pub SharedString);

impl std::ops::Deref for AgentModelId {
    type Target = SharedString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for AgentModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentModelInfo {
    pub id: AgentModelId,
    pub name: SharedString,
    pub icon: Option<IconName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentModelGroupName(pub SharedString);

#[derive(Debug, Clone)]
pub enum AgentModelList {
    Flat(Vec<AgentModelInfo>),
    Grouped(IndexMap<AgentModelGroupName, Vec<AgentModelInfo>>),
}

impl AgentModelList {
    pub fn is_empty(&self) -> bool {
        match self {
            AgentModelList::Flat(models) => models.is_empty(),
            AgentModelList::Grouped(groups) => groups.is_empty(),
        }
    }
}
