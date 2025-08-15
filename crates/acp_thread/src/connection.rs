use crate::AcpThread;
use agent_client_protocol::{self as acp};
use anyhow::Result;
use collections::IndexMap;
use gpui::{Entity, SharedString, Task};
use project::Project;
use std::{any::Any, error::Error, fmt, path::Path, rc::Rc, sync::Arc};
use ui::{App, IconName};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserMessageId(Arc<str>);

impl UserMessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

pub trait AgentConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>>;

    fn auth_methods(&self) -> &[acp::AuthMethod];

    fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>>;

    fn prompt(
        &self,
        user_message_id: Option<UserMessageId>,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>>;

    fn resume(
        &self,
        _session_id: &acp::SessionId,
        _cx: &mut App,
    ) -> Option<Rc<dyn AgentSessionResume>> {
        None
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);

    fn session_editor(
        &self,
        _session_id: &acp::SessionId,
        _cx: &mut App,
    ) -> Option<Rc<dyn AgentSessionEditor>> {
        None
    }

    /// Returns this agent as an [Rc<dyn ModelSelector>] if the model selection capability is supported.
    ///
    /// If the agent does not support model selection, returns [None].
    /// This allows sharing the selector in UI components.
    fn model_selector(&self) -> Option<Rc<dyn AgentModelSelector>> {
        None
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl dyn AgentConnection {
    pub fn downcast<T: 'static + AgentConnection + Sized>(self: Rc<Self>) -> Option<Rc<T>> {
        self.into_any().downcast().ok()
    }
}

pub trait AgentSessionEditor {
    fn truncate(&self, message_id: UserMessageId, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentSessionResume {
    fn run(&self, cx: &mut App) -> Task<Result<acp::PromptResponse>>;
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

#[cfg(feature = "test-support")]
mod test_support {
    use std::sync::Arc;

    use collections::HashMap;
    use futures::future::try_join_all;
    use gpui::{AppContext as _, WeakEntity};
    use parking_lot::Mutex;

    use super::*;

    #[derive(Clone, Default)]
    pub struct StubAgentConnection {
        sessions: Arc<Mutex<HashMap<acp::SessionId, WeakEntity<AcpThread>>>>,
        permission_requests: HashMap<acp::ToolCallId, Vec<acp::PermissionOption>>,
        next_prompt_updates: Arc<Mutex<Vec<acp::SessionUpdate>>>,
    }

    impl StubAgentConnection {
        pub fn new() -> Self {
            Self {
                next_prompt_updates: Default::default(),
                permission_requests: HashMap::default(),
                sessions: Arc::default(),
            }
        }

        pub fn set_next_prompt_updates(&self, updates: Vec<acp::SessionUpdate>) {
            *self.next_prompt_updates.lock() = updates;
        }

        pub fn with_permission_requests(
            mut self,
            permission_requests: HashMap<acp::ToolCallId, Vec<acp::PermissionOption>>,
        ) -> Self {
            self.permission_requests = permission_requests;
            self
        }

        pub fn send_update(
            &self,
            session_id: acp::SessionId,
            update: acp::SessionUpdate,
            cx: &mut App,
        ) {
            self.sessions
                .lock()
                .get(&session_id)
                .unwrap()
                .update(cx, |thread, cx| {
                    thread.handle_session_update(update.clone(), cx).unwrap();
                })
                .unwrap();
        }
    }

    impl AgentConnection for StubAgentConnection {
        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = acp::SessionId(self.sessions.lock().len().to_string().into());
            let thread =
                cx.new(|cx| AcpThread::new("Test", self.clone(), project, session_id.clone(), cx));
            self.sessions.lock().insert(session_id, thread.downgrade());
            Task::ready(Ok(thread))
        }

        fn authenticate(
            &self,
            _method_id: acp::AuthMethodId,
            _cx: &mut App,
        ) -> Task<gpui::Result<()>> {
            unimplemented!()
        }

        fn prompt(
            &self,
            _id: Option<UserMessageId>,
            params: acp::PromptRequest,
            cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            let sessions = self.sessions.lock();
            let thread = sessions.get(&params.session_id).unwrap();
            let mut tasks = vec![];
            for update in self.next_prompt_updates.lock().drain(..) {
                let thread = thread.clone();
                let update = update.clone();
                let permission_request = if let acp::SessionUpdate::ToolCall(tool_call) = &update
                    && let Some(options) = self.permission_requests.get(&tool_call.id)
                {
                    Some((tool_call.clone(), options.clone()))
                } else {
                    None
                };
                let task = cx.spawn(async move |cx| {
                    if let Some((tool_call, options)) = permission_request {
                        let permission = thread.update(cx, |thread, cx| {
                            thread.request_tool_call_authorization(
                                tool_call.clone(),
                                options.clone(),
                                cx,
                            )
                        })?;
                        permission.await?;
                    }
                    thread.update(cx, |thread, cx| {
                        thread.handle_session_update(update.clone(), cx).unwrap();
                    })?;
                    anyhow::Ok(())
                });
                tasks.push(task);
            }
            cx.spawn(async move |_| {
                try_join_all(tasks).await?;
                Ok(acp::PromptResponse {
                    stop_reason: acp::StopReason::EndTurn,
                })
            })
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {
            unimplemented!()
        }

        fn session_editor(
            &self,
            _session_id: &agent_client_protocol::SessionId,
            _cx: &mut App,
        ) -> Option<Rc<dyn AgentSessionEditor>> {
            Some(Rc::new(StubAgentSessionEditor))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    struct StubAgentSessionEditor;

    impl AgentSessionEditor for StubAgentSessionEditor {
        fn truncate(&self, _: UserMessageId, _: &mut App) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }
    }
}

#[cfg(feature = "test-support")]
pub use test_support::*;
