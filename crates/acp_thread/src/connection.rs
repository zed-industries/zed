use crate::AcpThread;
use agent_client_protocol::{self as acp};
use anyhow::Result;
use chrono::{DateTime, Utc};
use collections::IndexMap;
use gpui::{Entity, SharedString, Task};
use language_model::LanguageModelProviderId;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    error::Error,
    fmt,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use ui::{App, IconName};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct UserMessageId(Arc<str>);

impl UserMessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

pub trait AgentConnection {
    fn telemetry_id(&self) -> SharedString;

    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>>;

    /// Whether this agent supports loading existing sessions.
    fn supports_load_session(&self, _cx: &App) -> bool {
        false
    }

    /// Load an existing session by ID.
    fn load_session(
        self: Rc<Self>,
        _session: AgentSessionInfo,
        _project: Entity<Project>,
        _cwd: &Path,
        _cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        Task::ready(Err(anyhow::Error::msg("Loading sessions is not supported")))
    }

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
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionResume>> {
        None
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);

    fn truncate(
        &self,
        _session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionTruncate>> {
        None
    }

    fn set_title(
        &self,
        _session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionSetTitle>> {
        None
    }

    /// Returns this agent as an [Rc<dyn ModelSelector>] if the model selection capability is supported.
    ///
    /// If the agent does not support model selection, returns [None].
    /// This allows sharing the selector in UI components.
    fn model_selector(&self, _session_id: &acp::SessionId) -> Option<Rc<dyn AgentModelSelector>> {
        None
    }

    fn telemetry(&self) -> Option<Rc<dyn AgentTelemetry>> {
        None
    }

    fn session_modes(
        &self,
        _session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionModes>> {
        None
    }

    fn session_config_options(
        &self,
        _session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn AgentSessionConfigOptions>> {
        None
    }

    fn session_list(&self, _cx: &mut App) -> Option<Rc<dyn AgentSessionList>> {
        None
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl dyn AgentConnection {
    pub fn downcast<T: 'static + AgentConnection + Sized>(self: Rc<Self>) -> Option<Rc<T>> {
        self.into_any().downcast().ok()
    }
}

pub trait AgentSessionTruncate {
    fn run(&self, message_id: UserMessageId, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentSessionResume {
    fn run(&self, cx: &mut App) -> Task<Result<acp::PromptResponse>>;
}

pub trait AgentSessionSetTitle {
    fn run(&self, title: SharedString, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentTelemetry {
    /// A representation of the current thread state that can be serialized for
    /// storage with telemetry events.
    fn thread_data(
        &self,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<Result<serde_json::Value>>;
}

pub trait AgentSessionModes {
    fn current_mode(&self) -> acp::SessionModeId;

    fn all_modes(&self) -> Vec<acp::SessionMode>;

    fn set_mode(&self, mode: acp::SessionModeId, cx: &mut App) -> Task<Result<()>>;
}

pub trait AgentSessionConfigOptions {
    /// Get all current config options with their state
    fn config_options(&self) -> Vec<acp::SessionConfigOption>;

    /// Set a config option value
    /// Returns the full updated list of config options
    fn set_config_option(
        &self,
        config_id: acp::SessionConfigId,
        value: acp::SessionConfigValueId,
        cx: &mut App,
    ) -> Task<Result<Vec<acp::SessionConfigOption>>>;

    /// Whenever the config options are updated the receiver will be notified.
    /// Optional for agents that don't update their config options dynamically.
    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct AgentSessionListRequest {
    pub cwd: Option<PathBuf>,
    pub cursor: Option<String>,
    pub meta: Option<acp::Meta>,
}

#[derive(Debug, Clone)]
pub struct AgentSessionListResponse {
    pub sessions: Vec<AgentSessionInfo>,
    pub next_cursor: Option<String>,
    pub meta: Option<acp::Meta>,
}

impl AgentSessionListResponse {
    pub fn new(sessions: Vec<AgentSessionInfo>) -> Self {
        Self {
            sessions,
            next_cursor: None,
            meta: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentSessionInfo {
    pub session_id: acp::SessionId,
    pub cwd: Option<PathBuf>,
    pub title: Option<SharedString>,
    pub updated_at: Option<DateTime<Utc>>,
    pub meta: Option<acp::Meta>,
}

impl AgentSessionInfo {
    pub fn new(session_id: impl Into<acp::SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
            cwd: None,
            title: None,
            updated_at: None,
            meta: None,
        }
    }
}

pub trait AgentSessionList {
    fn list_sessions(
        &self,
        request: AgentSessionListRequest,
        cx: &mut App,
    ) -> Task<Result<AgentSessionListResponse>>;

    fn supports_delete(&self) -> bool {
        false
    }

    fn delete_session(&self, _session_id: &acp::SessionId, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!("delete_session not supported")))
    }

    fn delete_sessions(&self, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!("delete_sessions not supported")))
    }

    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        None
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl dyn AgentSessionList {
    pub fn downcast<T: 'static + AgentSessionList + Sized>(self: Rc<Self>) -> Option<Rc<T>> {
        self.into_any().downcast().ok()
    }
}

#[derive(Debug)]
pub struct AuthRequired {
    pub description: Option<String>,
    pub provider_id: Option<LanguageModelProviderId>,
}

impl AuthRequired {
    pub fn new() -> Self {
        Self {
            description: None,
            provider_id: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_language_model_provider(mut self, provider_id: LanguageModelProviderId) -> Self {
        self.provider_id = Some(provider_id);
        self
    }
}

impl Error for AuthRequired {}
impl fmt::Display for AuthRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Authentication required")
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
    /// - `model`: The model to select (should be one from [list_models]).
    /// - `cx`: The GPUI app context.
    ///
    /// # Returns
    /// A task resolving to `Ok(())` on success or an error.
    fn select_model(&self, model_id: acp::ModelId, cx: &mut App) -> Task<Result<()>>;

    /// Retrieves the currently selected model for a specific session (thread).
    ///
    /// # Parameters
    /// - `cx`: The GPUI app context.
    ///
    /// # Returns
    /// A task resolving to the selected model (always set) or an error (e.g., session not found).
    fn selected_model(&self, cx: &mut App) -> Task<Result<AgentModelInfo>>;

    /// Whenever the model list is updated the receiver will be notified.
    /// Optional for agents that don't update their model list.
    fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
        None
    }

    /// Returns whether the model picker should render a footer.
    fn should_render_footer(&self) -> bool {
        false
    }
}

/// Icon for a model in the model selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentModelIcon {
    /// A built-in icon from Zed's icon set.
    Named(IconName),
    /// Path to a custom SVG icon file.
    Path(SharedString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentModelInfo {
    pub id: acp::ModelId,
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<AgentModelIcon>,
}

impl From<acp::ModelInfo> for AgentModelInfo {
    fn from(info: acp::ModelInfo) -> Self {
        Self {
            id: info.model_id,
            name: info.name.into(),
            description: info.description.map(|desc| desc.into()),
            icon: None,
        }
    }
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

    pub fn is_flat(&self) -> bool {
        matches!(self, AgentModelList::Flat(_))
    }
}

#[derive(Debug, Clone)]
pub struct PermissionOptionChoice {
    pub allow: acp::PermissionOption,
    pub deny: acp::PermissionOption,
}

impl PermissionOptionChoice {
    pub fn label(&self) -> SharedString {
        self.allow.name.clone().into()
    }
}

#[derive(Debug, Clone)]
pub enum PermissionOptions {
    Flat(Vec<acp::PermissionOption>),
    Dropdown(Vec<PermissionOptionChoice>),
}

impl PermissionOptions {
    pub fn is_empty(&self) -> bool {
        match self {
            PermissionOptions::Flat(options) => options.is_empty(),
            PermissionOptions::Dropdown(options) => options.is_empty(),
        }
    }

    pub fn first_option_of_kind(
        &self,
        kind: acp::PermissionOptionKind,
    ) -> Option<&acp::PermissionOption> {
        match self {
            PermissionOptions::Flat(options) => options.iter().find(|option| option.kind == kind),
            PermissionOptions::Dropdown(options) => options.iter().find_map(|choice| {
                if choice.allow.kind == kind {
                    Some(&choice.allow)
                } else if choice.deny.kind == kind {
                    Some(&choice.deny)
                } else {
                    None
                }
            }),
        }
    }

    pub fn allow_once_option_id(&self) -> Option<acp::PermissionOptionId> {
        self.first_option_of_kind(acp::PermissionOptionKind::AllowOnce)
            .map(|option| option.option_id.clone())
    }
}

#[cfg(feature = "test-support")]
mod test_support {
    //! Test-only stubs and helpers for acp_thread.
    //!
    //! This module is gated by the `test-support` feature and is not included
    //! in production builds. It provides:
    //! - `StubAgentConnection` for mocking agent connections in tests
    //! - `create_test_png_base64` for generating test images

    use std::sync::Arc;

    use action_log::ActionLog;
    use collections::HashMap;
    use futures::{channel::oneshot, future::try_join_all};
    use gpui::{AppContext as _, WeakEntity};
    use parking_lot::Mutex;

    use super::*;

    /// Creates a PNG image encoded as base64 for testing.
    ///
    /// Generates a solid-color PNG of the specified dimensions and returns
    /// it as a base64-encoded string suitable for use in `ImageContent`.
    pub fn create_test_png_base64(width: u32, height: u32, color: [u8; 4]) -> String {
        use image::ImageEncoder as _;

        let mut png_data = Vec::new();
        {
            let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
            let mut pixels = Vec::with_capacity((width * height * 4) as usize);
            for _ in 0..(width * height) {
                pixels.extend_from_slice(&color);
            }
            encoder
                .write_image(&pixels, width, height, image::ExtendedColorType::Rgba8)
                .expect("Failed to encode PNG");
        }

        use image::EncodableLayout as _;
        base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            png_data.as_bytes(),
        )
    }

    #[derive(Clone, Default)]
    pub struct StubAgentConnection {
        sessions: Arc<Mutex<HashMap<acp::SessionId, Session>>>,
        permission_requests: HashMap<acp::ToolCallId, PermissionOptions>,
        next_prompt_updates: Arc<Mutex<Vec<acp::SessionUpdate>>>,
    }

    struct Session {
        thread: WeakEntity<AcpThread>,
        response_tx: Option<oneshot::Sender<acp::StopReason>>,
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
            permission_requests: HashMap<acp::ToolCallId, PermissionOptions>,
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
            assert!(
                self.next_prompt_updates.lock().is_empty(),
                "Use either send_update or set_next_prompt_updates"
            );

            self.sessions
                .lock()
                .get(&session_id)
                .unwrap()
                .thread
                .update(cx, |thread, cx| {
                    thread.handle_session_update(update, cx).unwrap();
                })
                .unwrap();
        }

        pub fn end_turn(&self, session_id: acp::SessionId, stop_reason: acp::StopReason) {
            self.sessions
                .lock()
                .get_mut(&session_id)
                .unwrap()
                .response_tx
                .take()
                .expect("No pending turn")
                .send(stop_reason)
                .unwrap();
        }
    }

    impl AgentConnection for StubAgentConnection {
        fn telemetry_id(&self) -> SharedString {
            "stub".into()
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn model_selector(
            &self,
            _session_id: &acp::SessionId,
        ) -> Option<Rc<dyn AgentModelSelector>> {
            Some(self.model_selector_impl())
        }

        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = acp::SessionId::new(self.sessions.lock().len().to_string());
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    "Test",
                    self.clone(),
                    project,
                    action_log,
                    session_id.clone(),
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            });
            self.sessions.lock().insert(
                session_id,
                Session {
                    thread: thread.downgrade(),
                    response_tx: None,
                },
            );
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
            let mut sessions = self.sessions.lock();
            let Session {
                thread,
                response_tx,
            } = sessions.get_mut(&params.session_id).unwrap();
            let mut tasks = vec![];
            if self.next_prompt_updates.lock().is_empty() {
                let (tx, rx) = oneshot::channel();
                response_tx.replace(tx);
                cx.spawn(async move |_| {
                    let stop_reason = rx.await?;
                    Ok(acp::PromptResponse::new(stop_reason))
                })
            } else {
                for update in self.next_prompt_updates.lock().drain(..) {
                    let thread = thread.clone();
                    let update = update.clone();
                    let permission_request = if let acp::SessionUpdate::ToolCall(tool_call) =
                        &update
                        && let Some(options) = self.permission_requests.get(&tool_call.tool_call_id)
                    {
                        Some((tool_call.clone(), options.clone()))
                    } else {
                        None
                    };
                    let task = cx.spawn(async move |cx| {
                        if let Some((tool_call, options)) = permission_request {
                            thread
                                .update(cx, |thread, cx| {
                                    thread.request_tool_call_authorization(
                                        tool_call.clone().into(),
                                        options.clone(),
                                        false,
                                        cx,
                                    )
                                })??
                                .await;
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
                    Ok(acp::PromptResponse::new(acp::StopReason::EndTurn))
                })
            }
        }

        fn cancel(&self, session_id: &acp::SessionId, _cx: &mut App) {
            if let Some(end_turn_tx) = self
                .sessions
                .lock()
                .get_mut(session_id)
                .unwrap()
                .response_tx
                .take()
            {
                end_turn_tx.send(acp::StopReason::Cancelled).unwrap();
            }
        }

        fn truncate(
            &self,
            _session_id: &agent_client_protocol::SessionId,
            _cx: &App,
        ) -> Option<Rc<dyn AgentSessionTruncate>> {
            Some(Rc::new(StubAgentSessionEditor))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    struct StubAgentSessionEditor;

    impl AgentSessionTruncate for StubAgentSessionEditor {
        fn run(&self, _: UserMessageId, _: &mut App) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }
    }

    #[derive(Clone)]
    struct StubModelSelector {
        selected_model: Arc<Mutex<AgentModelInfo>>,
    }

    impl StubModelSelector {
        fn new() -> Self {
            Self {
                selected_model: Arc::new(Mutex::new(AgentModelInfo {
                    id: acp::ModelId::new("visual-test-model"),
                    name: "Visual Test Model".into(),
                    description: Some("A stub model for visual testing".into()),
                    icon: Some(AgentModelIcon::Named(ui::IconName::ZedAssistant)),
                })),
            }
        }
    }

    impl AgentModelSelector for StubModelSelector {
        fn list_models(&self, _cx: &mut App) -> Task<Result<AgentModelList>> {
            let model = self.selected_model.lock().clone();
            Task::ready(Ok(AgentModelList::Flat(vec![model])))
        }

        fn select_model(&self, model_id: acp::ModelId, _cx: &mut App) -> Task<Result<()>> {
            self.selected_model.lock().id = model_id;
            Task::ready(Ok(()))
        }

        fn selected_model(&self, _cx: &mut App) -> Task<Result<AgentModelInfo>> {
            Task::ready(Ok(self.selected_model.lock().clone()))
        }
    }

    impl StubAgentConnection {
        /// Returns a model selector for this stub connection.
        pub fn model_selector_impl(&self) -> Rc<dyn AgentModelSelector> {
            Rc::new(StubModelSelector::new())
        }
    }
}

#[cfg(feature = "test-support")]
pub use test_support::*;
