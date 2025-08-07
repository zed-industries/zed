use std::{error::Error, fmt, path::Path, rc::Rc, sync::Arc};

use agent_client_protocol::{self as acp};
use anyhow::Result;
use gpui::{AsyncApp, Entity, Task};
use language_model::LanguageModel;
use project::Project;
use ui::App;

use crate::AcpThread;

/// Trait for agents that support listing, selecting, and querying language models.
///
/// This is an optional capability; agents indicate support via [AgentConnection::model_selector].
pub trait ModelSelector: 'static {
    /// Lists all available language models for this agent.
    ///
    /// # Parameters
    /// - `cx`: The GPUI app context for async operations and global access.
    ///
    /// # Returns
    /// A task resolving to the list of models or an error (e.g., if no models are configured).
    fn list_models(&self, cx: &mut AsyncApp) -> Task<Result<Vec<Arc<dyn LanguageModel>>>>;

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
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
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
        cx: &mut AsyncApp,
    ) -> Task<Result<Arc<dyn LanguageModel>>>;
}

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
    fn model_selector(&self) -> Option<Rc<dyn ModelSelector>> {
        None // Default impl for agents that don't support it
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
