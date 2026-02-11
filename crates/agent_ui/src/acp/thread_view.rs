use acp_thread::{
    AcpThread, AcpThreadEvent, AgentSessionInfo, AgentThreadEntry, AssistantMessage,
    AssistantMessageChunk, AuthRequired, LoadError, MentionUri, PermissionOptionChoice,
    PermissionOptions, RetryStatus, ThreadStatus, ToolCall, ToolCallContent, ToolCallStatus,
    UserMessageId,
};
use acp_thread::{AgentConnection, Plan};
use action_log::{ActionLog, ActionLogTelemetry};
use agent::{NativeAgentServer, NativeAgentSessionList, SharedThread, ThreadStore};
use agent_client_protocol::{self as acp, PromptCapabilities};
use agent_servers::{AgentServer, AgentServerDelegate};
use agent_settings::{AgentProfileId, AgentSettings};
use anyhow::{Result, anyhow};
use arrayvec::ArrayVec;
use audio::{Audio, Sound};
use buffer_diff::BufferDiff;
use client::zed_urls;
use collections::{HashMap, HashSet};
use editor::scroll::Autoscroll;
use editor::{
    Editor, EditorEvent, EditorMode, MultiBuffer, PathKey, SelectionEffects, SizingBehavior,
};
use feature_flags::{
    AgentSharingFeatureFlag, AgentV2FeatureFlag, CloudThinkingEffortFeatureFlag,
    FeatureFlagAppExt as _,
};
use file_icons::FileIcons;
use fs::Fs;
use futures::FutureExt as _;
use gpui::{
    Action, Animation, AnimationExt, AnyView, App, ClickEvent, ClipboardItem, CursorStyle,
    ElementId, Empty, Entity, FocusHandle, Focusable, Hsla, ListOffset, ListState, ObjectFit,
    PlatformDisplay, ScrollHandle, SharedString, Subscription, Task, TextStyle, WeakEntity, Window,
    WindowHandle, div, ease_in_out, img, linear_color_stop, linear_gradient, list, point,
    pulsating_between,
};
use language::Buffer;
use language_model::LanguageModelRegistry;
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use project::{AgentServerStore, ExternalAgentServerName, Project, ProjectEntryId};
use prompt_store::{PromptId, PromptStore};
use rope::Point;
use settings::{NotifyWhenAgentWaiting, Settings as _, SettingsStore};
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use std::{collections::BTreeMap, rc::Rc, time::Duration};
use terminal_view::terminal_panel::TerminalPanel;
use text::{Anchor, ToPoint as _};
use theme::AgentFontSize;
use ui::{
    Callout, CommonAnimationExt, ContextMenu, ContextMenuEntry, CopyButton, DecoratedIcon,
    DiffStat, Disclosure, Divider, DividerColor, IconDecoration, IconDecorationKind, KeyBinding,
    PopoverMenu, PopoverMenuHandle, SpinnerLabel, TintColor, Tooltip, WithScrollbar, prelude::*,
    right_click_menu,
};
use util::{ResultExt, size::format_file_size, time::duration_alt_display};
use util::{debug_panic, defer};
use workspace::{
    CollaboratorId, NewTerminal, NotificationSource, Toast, Workspace,
    notifications::NotificationId,
};
use zed_actions::agent::{Chat, ToggleModelSelector};
use zed_actions::assistant::OpenRulesLibrary;

use super::config_options::ConfigOptionsView;
use super::entry_view_state::EntryViewState;
use super::thread_history::AcpThreadHistory;
use crate::acp::AcpModelSelectorPopover;
use crate::acp::ModeSelector;
use crate::acp::entry_view_state::{EntryViewEvent, ViewEvent};
use crate::acp::message_editor::{MessageEditor, MessageEditorEvent};
use crate::agent_diff::AgentDiff;
use crate::profile_selector::{ProfileProvider, ProfileSelector};
use crate::ui::{AgentNotification, AgentNotificationEvent};
use crate::{
    AgentDiffPane, AgentPanel, AllowAlways, AllowOnce, AuthorizeToolCall, ClearMessageQueue,
    CycleFavoriteModels, CycleModeSelector, CycleThinkingEffort, EditFirstQueuedMessage,
    ExpandMessageEditor, ExternalAgentInitialContent, Follow, KeepAll, NewThread,
    OpenAddContextMenu, OpenAgentDiff, OpenHistory, RejectAll, RejectOnce,
    RemoveFirstQueuedMessage, SelectPermissionGranularity, SendImmediately, SendNextQueuedMessage,
    ToggleProfileSelector, ToggleThinkingEffortMenu, ToggleThinkingMode,
};

const STOPWATCH_THRESHOLD: Duration = Duration::from_secs(30);
const TOKEN_THRESHOLD: u64 = 250;

mod active_thread;
pub use active_thread::*;

pub struct QueuedMessage {
    pub content: Vec<acp::ContentBlock>,
    pub tracked_buffers: Vec<Entity<Buffer>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ThreadFeedback {
    Positive,
    Negative,
}

#[derive(Debug)]
pub(crate) enum ThreadError {
    PaymentRequired,
    Refusal,
    AuthenticationRequired(SharedString),
    Other {
        message: SharedString,
        acp_error_code: Option<SharedString>,
    },
}

impl ThreadError {
    fn from_err(error: anyhow::Error, agent_name: &str) -> Self {
        if error.is::<language_model::PaymentRequiredError>() {
            Self::PaymentRequired
        } else if let Some(acp_error) = error.downcast_ref::<acp::Error>()
            && acp_error.code == acp::ErrorCode::AuthRequired
        {
            Self::AuthenticationRequired(acp_error.message.clone().into())
        } else {
            let message: SharedString = format!("{:#}", error).into();

            // Extract ACP error code if available
            let acp_error_code = error
                .downcast_ref::<acp::Error>()
                .map(|acp_error| SharedString::from(acp_error.code.to_string()));

            // TODO: we should have Gemini return better errors here.
            if agent_name == "Gemini CLI"
                && message.contains("Could not load the default credentials")
                || message.contains("API key not valid")
                || message.contains("Request had invalid authentication credentials")
            {
                Self::AuthenticationRequired(message)
            } else {
                Self::Other {
                    message,
                    acp_error_code,
                }
            }
        }
    }
}

impl ProfileProvider for Entity<agent::Thread> {
    fn profile_id(&self, cx: &App) -> AgentProfileId {
        self.read(cx).profile().clone()
    }

    fn set_profile(&self, profile_id: AgentProfileId, cx: &mut App) {
        self.update(cx, |thread, cx| {
            // Apply the profile and let the thread swap to its default model.
            thread.set_profile(profile_id, cx);
        });
    }

    fn profiles_supported(&self, cx: &App) -> bool {
        self.read(cx)
            .model()
            .is_some_and(|model| model.supports_tools())
    }
}

pub struct AcpServerView {
    agent: Rc<dyn AgentServer>,
    agent_server_store: Entity<AgentServerStore>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    thread_store: Option<Entity<ThreadStore>>,
    prompt_store: Option<Entity<PromptStore>>,
    server_state: ServerState,
    login: Option<task::SpawnInTerminal>, // is some <=> Active | Unauthenticated
    history: Entity<AcpThreadHistory>,
    focus_handle: FocusHandle,
    notifications: Vec<WindowHandle<AgentNotification>>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    auth_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl AcpServerView {
    pub fn active_thread(&self) -> Option<&Entity<AcpThreadView>> {
        match &self.server_state {
            ServerState::Connected(connected) => connected.active_view(),
            _ => None,
        }
    }

    pub fn parent_thread(&self, cx: &App) -> Option<Entity<AcpThreadView>> {
        match &self.server_state {
            ServerState::Connected(connected) => {
                let mut current = connected.active_view()?;
                while let Some(parent_id) = current.read(cx).parent_id.clone() {
                    if let Some(parent) = connected.threads.get(&parent_id) {
                        current = parent;
                    } else {
                        break;
                    }
                }
                Some(current.clone())
            }
            _ => None,
        }
    }

    pub fn thread_view(&self, session_id: &acp::SessionId) -> Option<Entity<AcpThreadView>> {
        let connected = self.as_connected()?;
        connected.threads.get(session_id).cloned()
    }

    pub fn as_connected(&self) -> Option<&ConnectedServerState> {
        match &self.server_state {
            ServerState::Connected(connected) => Some(connected),
            _ => None,
        }
    }

    pub fn as_connected_mut(&mut self) -> Option<&mut ConnectedServerState> {
        match &mut self.server_state {
            ServerState::Connected(connected) => Some(connected),
            _ => None,
        }
    }

    pub fn navigate_to_session(
        &mut self,
        session_id: acp::SessionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(connected) = self.as_connected_mut() else {
            return;
        };

        connected.navigate_to_session(session_id);
        if let Some(view) = self.active_thread() {
            view.focus_handle(cx).focus(window, cx);
        }
        cx.notify();
    }
}

enum ServerState {
    Loading(Entity<LoadingView>),
    LoadError(LoadError),
    Connected(ConnectedServerState),
}

// current -> Entity
// hashmap of threads, current becomes session_id
pub struct ConnectedServerState {
    auth_state: AuthState,
    active_id: Option<acp::SessionId>,
    threads: HashMap<acp::SessionId, Entity<AcpThreadView>>,
    connection: Rc<dyn AgentConnection>,
}

enum AuthState {
    Ok,
    Unauthenticated {
        description: Option<Entity<Markdown>>,
        configuration_view: Option<AnyView>,
        pending_auth_method: Option<acp::AuthMethodId>,
        _subscription: Option<Subscription>,
    },
}

impl AuthState {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

struct LoadingView {
    title: SharedString,
    _load_task: Task<()>,
    _update_title_task: Task<anyhow::Result<()>>,
}

impl ConnectedServerState {
    pub fn active_view(&self) -> Option<&Entity<AcpThreadView>> {
        self.active_id.as_ref().and_then(|id| self.threads.get(id))
    }

    pub fn has_thread_error(&self, cx: &App) -> bool {
        self.active_view()
            .map_or(false, |view| view.read(cx).thread_error.is_some())
    }

    pub fn navigate_to_session(&mut self, session_id: acp::SessionId) {
        if self.threads.contains_key(&session_id) {
            self.active_id = Some(session_id);
        }
    }

    pub fn close_all_sessions(&self, cx: &mut App) -> Task<()> {
        let tasks = self
            .threads
            .keys()
            .map(|id| self.connection.close_session(id, cx));
        let task = futures::future::join_all(tasks);
        cx.background_spawn(async move {
            task.await;
        })
    }
}

impl AcpServerView {
    pub fn new(
        agent: Rc<dyn AgentServer>,
        resume_thread: Option<AgentSessionInfo>,
        initial_content: Option<ExternalAgentInitialContent>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        prompt_store: Option<Entity<PromptStore>>,
        history: Entity<AcpThreadHistory>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let subscriptions = vec![
            cx.observe_global_in::<SettingsStore>(window, Self::agent_ui_font_size_changed),
            cx.observe_global_in::<AgentFontSize>(window, Self::agent_ui_font_size_changed),
            cx.subscribe_in(
                &agent_server_store,
                window,
                Self::handle_agent_servers_updated,
            ),
        ];

        cx.on_release(|this, cx| {
            if let Some(connected) = this.as_connected() {
                connected.close_all_sessions(cx).detach();
            }
            for window in this.notifications.drain(..) {
                window
                    .update(cx, |_, window, _| {
                        window.remove_window();
                    })
                    .ok();
            }
        })
        .detach();

        Self {
            agent: agent.clone(),
            agent_server_store,
            workspace,
            project: project.clone(),
            thread_store,
            prompt_store,
            server_state: Self::initial_state(
                agent.clone(),
                resume_thread,
                project,
                initial_content,
                window,
                cx,
            ),
            login: None,
            notifications: Vec::new(),
            notification_subscriptions: HashMap::default(),
            auth_task: None,
            history,
            _subscriptions: subscriptions,
            focus_handle: cx.focus_handle(),
        }
    }

    fn set_server_state(&mut self, state: ServerState, cx: &mut Context<Self>) {
        if let Some(connected) = self.as_connected() {
            connected.close_all_sessions(cx).detach();
        }

        self.server_state = state;
        cx.notify();
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let resume_thread_metadata = self
            .active_thread()
            .and_then(|thread| thread.read(cx).resume_thread_metadata.clone());

        let state = Self::initial_state(
            self.agent.clone(),
            resume_thread_metadata,
            self.project.clone(),
            None,
            window,
            cx,
        );
        self.set_server_state(state, cx);

        if let Some(view) = self.active_thread() {
            view.update(cx, |this, cx| {
                this.message_editor.update(cx, |editor, cx| {
                    editor.set_command_state(
                        this.prompt_capabilities.clone(),
                        this.available_commands.clone(),
                        cx,
                    );
                });
            });
        }
        cx.notify();
    }

    fn initial_state(
        agent: Rc<dyn AgentServer>,
        resume_thread: Option<AgentSessionInfo>,
        project: Entity<Project>,
        initial_content: Option<ExternalAgentInitialContent>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ServerState {
        if project.read(cx).is_via_collab()
            && agent.clone().downcast::<NativeAgentServer>().is_none()
        {
            return ServerState::LoadError(LoadError::Other(
                "External agents are not yet supported in shared projects.".into(),
            ));
        }
        let mut worktrees = project.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
        // Pick the first non-single-file worktree for the root directory if there are any,
        // and otherwise the parent of a single-file worktree, falling back to $HOME if there are no visible worktrees.
        worktrees.sort_by(|l, r| {
            l.read(cx)
                .is_single_file()
                .cmp(&r.read(cx).is_single_file())
        });
        let worktree_roots: Vec<Arc<Path>> = worktrees
            .iter()
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                if worktree.is_single_file() {
                    Some(worktree.abs_path().parent()?.into())
                } else {
                    Some(worktree.abs_path())
                }
            })
            .collect();
        let root_dir = worktree_roots.first().cloned();
        let session_cwd = resume_thread
            .as_ref()
            .and_then(|resume| {
                resume
                    .cwd
                    .as_ref()
                    .and_then(|cwd| util::paths::normalize_lexically(cwd).ok())
                    .filter(|cwd| {
                        worktree_roots
                            .iter()
                            .any(|root| cwd.starts_with(root.as_ref()))
                    })
                    .map(|path| path.into())
            })
            .or_else(|| root_dir.clone())
            .unwrap_or_else(|| paths::home_dir().as_path().into());

        let (status_tx, mut status_rx) = watch::channel("Loading…".into());
        let (new_version_available_tx, mut new_version_available_rx) = watch::channel(None);
        let delegate = AgentServerDelegate::new(
            project.read(cx).agent_server_store().clone(),
            project.clone(),
            Some(status_tx),
            Some(new_version_available_tx),
        );

        let connect_task = agent.connect(root_dir.as_deref(), delegate, cx);
        let load_task = cx.spawn_in(window, async move |this, cx| {
            let connection = match connect_task.await {
                Ok((connection, login)) => {
                    this.update(cx, |this, _| this.login = login).ok();
                    connection
                }
                Err(err) => {
                    this.update_in(cx, |this, window, cx| {
                        if err.downcast_ref::<LoadError>().is_some() {
                            this.handle_load_error(err, window, cx);
                        } else if let Some(active) = this.active_thread() {
                            active.update(cx, |active, cx| active.handle_any_thread_error(err, cx));
                        }
                        cx.notify();
                    })
                    .log_err();
                    return;
                }
            };

            telemetry::event!("Agent Thread Started", agent = connection.telemetry_id());

            let mut resumed_without_history = false;
            let result = if let Some(resume) = resume_thread.clone() {
                cx.update(|_, cx| {
                    if connection.supports_load_session(cx) {
                        connection
                            .clone()
                            .load_session(resume, project.clone(), &session_cwd, cx)
                    } else if connection.supports_resume_session(cx) {
                        resumed_without_history = true;
                        connection
                            .clone()
                            .resume_session(resume, project.clone(), &session_cwd, cx)
                    } else {
                        Task::ready(Err(anyhow!(LoadError::Other(
                            "Loading or resuming sessions is not supported by this agent.".into()
                        ))))
                    }
                })
                .log_err()
            } else {
                cx.update(|_, cx| {
                    connection
                        .clone()
                        .new_session(project.clone(), session_cwd.as_ref(), cx)
                })
                .log_err()
            };

            let Some(result) = result else {
                return;
            };

            let result = match result.await {
                Err(e) => match e.downcast::<acp_thread::AuthRequired>() {
                    Ok(err) => {
                        cx.update(|window, cx| {
                            Self::handle_auth_required(
                                this,
                                err,
                                agent.name(),
                                connection,
                                window,
                                cx,
                            )
                        })
                        .log_err();
                        return;
                    }
                    Err(err) => Err(err),
                },
                Ok(thread) => Ok(thread),
            };

            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(thread) => {
                        let current = this.new_thread_view(
                            None,
                            thread,
                            resumed_without_history,
                            resume_thread,
                            initial_content,
                            window,
                            cx,
                        );

                        if this.focus_handle.contains_focused(window, cx) {
                            current
                                .read(cx)
                                .message_editor
                                .focus_handle(cx)
                                .focus(window, cx);
                        }

                        let id = current.read(cx).thread.read(cx).session_id().clone();
                        this.set_server_state(
                            ServerState::Connected(ConnectedServerState {
                                connection,
                                auth_state: AuthState::Ok,
                                active_id: Some(id.clone()),
                                threads: HashMap::from_iter([(id, current)]),
                            }),
                            cx,
                        );
                    }
                    Err(err) => {
                        this.handle_load_error(err, window, cx);
                    }
                };
            })
            .log_err();
        });

        cx.spawn(async move |this, cx| {
            while let Ok(new_version) = new_version_available_rx.recv().await {
                if let Some(new_version) = new_version {
                    this.update(cx, |this, cx| {
                        if let Some(thread) = this.active_thread() {
                            thread.update(cx, |thread, _cx| {
                                thread.new_server_version_available = Some(new_version.into());
                            });
                        }
                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();

        let loading_view = cx.new(|cx| {
            let update_title_task = cx.spawn(async move |this, cx| {
                loop {
                    let status = status_rx.recv().await?;
                    this.update(cx, |this: &mut LoadingView, cx| {
                        this.title = status;
                        cx.notify();
                    })?;
                }
            });

            LoadingView {
                title: "Loading…".into(),
                _load_task: load_task,
                _update_title_task: update_title_task,
            }
        });

        ServerState::Loading(loading_view)
    }

    fn new_thread_view(
        &self,
        parent_id: Option<acp::SessionId>,
        thread: Entity<AcpThread>,
        resumed_without_history: bool,
        resume_thread: Option<AgentSessionInfo>,
        initial_content: Option<ExternalAgentInitialContent>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<AcpThreadView> {
        let agent_name = self.agent.name();
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        let available_commands = Rc::new(RefCell::new(vec![]));

        let action_log = thread.read(cx).action_log().clone();

        prompt_capabilities.replace(thread.read(cx).prompt_capabilities());

        let entry_view_state = cx.new(|_| {
            EntryViewState::new(
                self.workspace.clone(),
                self.project.downgrade(),
                self.thread_store.clone(),
                self.history.downgrade(),
                self.prompt_store.clone(),
                prompt_capabilities.clone(),
                available_commands.clone(),
                self.agent.name(),
            )
        });

        let count = thread.read(cx).entries().len();
        let list_state = ListState::new(0, gpui::ListAlignment::Bottom, px(2048.0));
        entry_view_state.update(cx, |view_state, cx| {
            for ix in 0..count {
                view_state.sync_entry(ix, &thread, window, cx);
            }
            list_state.splice_focusable(
                0..0,
                (0..count).map(|ix| view_state.entry(ix)?.focus_handle(cx)),
            );
        });

        AgentDiff::set_active_thread(&self.workspace, thread.clone(), window, cx);

        let connection = thread.read(cx).connection().clone();
        let session_id = thread.read(cx).session_id().clone();
        let session_list = if connection.supports_session_history(cx) {
            connection.session_list(cx)
        } else {
            None
        };
        self.history.update(cx, |history, cx| {
            history.set_session_list(session_list, cx);
        });

        // Check for config options first
        // Config options take precedence over legacy mode/model selectors
        // (feature flag gating happens at the data layer)
        let config_options_provider = connection.session_config_options(&session_id, cx);

        let config_options_view;
        let mode_selector;
        let model_selector;
        if let Some(config_options) = config_options_provider {
            // Use config options - don't create mode_selector or model_selector
            let agent_server = self.agent.clone();
            let fs = self.project.read(cx).fs().clone();
            config_options_view =
                Some(cx.new(|cx| {
                    ConfigOptionsView::new(config_options, agent_server, fs, window, cx)
                }));
            model_selector = None;
            mode_selector = None;
        } else {
            // Fall back to legacy mode/model selectors
            config_options_view = None;
            model_selector = connection.model_selector(&session_id).map(|selector| {
                let agent_server = self.agent.clone();
                let fs = self.project.read(cx).fs().clone();
                cx.new(|cx| {
                    AcpModelSelectorPopover::new(
                        selector,
                        agent_server,
                        fs,
                        PopoverMenuHandle::default(),
                        self.focus_handle(cx),
                        window,
                        cx,
                    )
                })
            });

            mode_selector = connection
                .session_modes(&session_id, cx)
                .map(|session_modes| {
                    let fs = self.project.read(cx).fs().clone();
                    cx.new(|_cx| ModeSelector::new(session_modes, self.agent.clone(), fs))
                });
        }

        let mut subscriptions = vec![
            cx.subscribe_in(&thread, window, Self::handle_thread_event),
            cx.observe(&action_log, |_, _, cx| cx.notify()),
        ];

        let parent_session_id = thread.read(cx).session_id().clone();
        let subagent_sessions = thread
            .read(cx)
            .entries()
            .iter()
            .filter_map(|entry| match entry {
                AgentThreadEntry::ToolCall(call) => call.subagent_session_id.clone(),
                _ => None,
            })
            .collect::<Vec<_>>();

        if !subagent_sessions.is_empty() {
            cx.spawn_in(window, async move |this, cx| {
                this.update_in(cx, |this, window, cx| {
                    for subagent_id in subagent_sessions {
                        this.load_subagent_session(
                            subagent_id,
                            parent_session_id.clone(),
                            window,
                            cx,
                        );
                    }
                })
            })
            .detach();
        }

        let title_editor = if thread.update(cx, |thread, cx| thread.can_set_title(cx)) {
            let editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(thread.read(cx).title(), window, cx);
                editor
            });
            subscriptions.push(cx.subscribe_in(&editor, window, Self::handle_title_editor_event));
            Some(editor)
        } else {
            None
        };

        let profile_selector: Option<Rc<agent::NativeAgentConnection>> =
            connection.clone().downcast();
        let profile_selector = profile_selector
            .and_then(|native_connection| native_connection.thread(&session_id, cx))
            .map(|native_thread| {
                cx.new(|cx| {
                    ProfileSelector::new(
                        <dyn Fs>::global(cx),
                        Arc::new(native_thread),
                        self.focus_handle(cx),
                        cx,
                    )
                })
            });

        let agent_display_name = self
            .agent_server_store
            .read(cx)
            .agent_display_name(&ExternalAgentServerName(agent_name.clone()))
            .unwrap_or_else(|| agent_name.clone());

        let agent_icon = self.agent.logo();

        let weak = cx.weak_entity();
        cx.new(|cx| {
            AcpThreadView::new(
                parent_id,
                thread,
                self.login.clone(),
                weak,
                agent_icon,
                agent_name,
                agent_display_name,
                self.workspace.clone(),
                entry_view_state,
                title_editor,
                config_options_view,
                mode_selector,
                model_selector,
                profile_selector,
                list_state,
                prompt_capabilities,
                available_commands,
                resumed_without_history,
                resume_thread,
                self.project.downgrade(),
                self.thread_store.clone(),
                self.history.clone(),
                self.prompt_store.clone(),
                initial_content,
                subscriptions,
                window,
                cx,
            )
        })
    }

    fn handle_auth_required(
        this: WeakEntity<Self>,
        err: AuthRequired,
        agent_name: SharedString,
        connection: Rc<dyn AgentConnection>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (configuration_view, subscription) = if let Some(provider_id) = &err.provider_id {
            let registry = LanguageModelRegistry::global(cx);

            let sub = window.subscribe(&registry, cx, {
                let provider_id = provider_id.clone();
                let this = this.clone();
                move |_, ev, window, cx| {
                    if let language_model::Event::ProviderStateChanged(updated_provider_id) = &ev
                        && &provider_id == updated_provider_id
                        && LanguageModelRegistry::global(cx)
                            .read(cx)
                            .provider(&provider_id)
                            .map_or(false, |provider| provider.is_authenticated(cx))
                    {
                        this.update(cx, |this, cx| {
                            this.reset(window, cx);
                        })
                        .ok();
                    }
                }
            });

            let view = registry.read(cx).provider(&provider_id).map(|provider| {
                provider.configuration_view(
                    language_model::ConfigurationViewTargetAgent::Other(agent_name),
                    window,
                    cx,
                )
            });

            (view, Some(sub))
        } else {
            (None, None)
        };

        this.update(cx, |this, cx| {
            let description = err
                .description
                .map(|desc| cx.new(|cx| Markdown::new(desc.into(), None, None, cx)));
            let auth_state = AuthState::Unauthenticated {
                pending_auth_method: None,
                configuration_view,
                description,
                _subscription: subscription,
            };
            if let Some(connected) = this.as_connected_mut() {
                connected.auth_state = auth_state;
                if let Some(view) = connected.active_view()
                    && view
                        .read(cx)
                        .message_editor
                        .focus_handle(cx)
                        .is_focused(window)
                {
                    this.focus_handle.focus(window, cx)
                }
            } else {
                this.set_server_state(
                    ServerState::Connected(ConnectedServerState {
                        auth_state,
                        active_id: None,
                        threads: HashMap::default(),
                        connection,
                    }),
                    cx,
                );
            }
            cx.notify();
        })
        .ok();
    }

    fn handle_load_error(
        &mut self,
        err: anyhow::Error,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(view) = self.active_thread() {
            if view
                .read(cx)
                .message_editor
                .focus_handle(cx)
                .is_focused(window)
            {
                self.focus_handle.focus(window, cx)
            }
        }
        let load_error = if let Some(load_err) = err.downcast_ref::<LoadError>() {
            load_err.clone()
        } else {
            LoadError::Other(format!("{:#}", err).into())
        };
        self.emit_load_error_telemetry(&load_error);
        self.set_server_state(ServerState::LoadError(load_error), cx);
    }

    fn handle_agent_servers_updated(
        &mut self,
        _agent_server_store: &Entity<project::AgentServerStore>,
        _event: &project::AgentServersUpdated,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // If we're in a LoadError state OR have a thread_error set (which can happen
        // when agent.connect() fails during loading), retry loading the thread.
        // This handles the case where a thread is restored before authentication completes.
        let should_retry = match &self.server_state {
            ServerState::Loading(_) => false,
            ServerState::LoadError(_) => true,
            ServerState::Connected(connected) => {
                connected.auth_state.is_ok() && connected.has_thread_error(cx)
            }
        };

        if should_retry {
            if let Some(active) = self.active_thread() {
                active.update(cx, |active, cx| {
                    active.clear_thread_error(cx);
                });
            }
            self.reset(window, cx);
        }
    }

    pub fn workspace(&self) -> &WeakEntity<Workspace> {
        &self.workspace
    }

    pub fn title(&self, cx: &App) -> SharedString {
        match &self.server_state {
            ServerState::Connected(_) => "New Thread".into(),
            ServerState::Loading(loading_view) => loading_view.read(cx).title.clone(),
            ServerState::LoadError(error) => match error {
                LoadError::Unsupported { .. } => format!("Upgrade {}", self.agent.name()).into(),
                LoadError::FailedToInstall(_) => {
                    format!("Failed to Install {}", self.agent.name()).into()
                }
                LoadError::Exited { .. } => format!("{} Exited", self.agent.name()).into(),
                LoadError::Other(_) => format!("Error Loading {}", self.agent.name()).into(),
            },
        }
    }

    pub fn cancel_generation(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, cx| {
                active.cancel_generation(cx);
            });
        }
    }

    pub fn handle_title_editor_event(
        &mut self,
        title_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, cx| {
                active.handle_title_editor_event(title_editor, event, window, cx);
            });
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.server_state, ServerState::Loading { .. })
    }

    fn update_turn_tokens(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, cx| {
                active.update_turn_tokens(cx);
            });
        }
    }

    fn send_queued_message_at_index(
        &mut self,
        index: usize,
        is_send_now: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, cx| {
                active.send_queued_message_at_index(index, is_send_now, window, cx);
            });
        }
    }

    fn handle_thread_event(
        &mut self,
        thread: &Entity<AcpThread>,
        event: &AcpThreadEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread_id = thread.read(cx).session_id().clone();
        let is_subagent = thread.read(cx).parent_session_id().is_some();
        match event {
            AcpThreadEvent::NewEntry => {
                let len = thread.read(cx).entries().len();
                let index = len - 1;
                if let Some(active) = self.thread_view(&thread_id) {
                    let entry_view_state = active.read(cx).entry_view_state.clone();
                    let list_state = active.read(cx).list_state.clone();
                    entry_view_state.update(cx, |view_state, cx| {
                        view_state.sync_entry(index, thread, window, cx);
                        list_state.splice_focusable(
                            index..index,
                            [view_state
                                .entry(index)
                                .and_then(|entry| entry.focus_handle(cx))],
                        );
                    });
                }
            }
            AcpThreadEvent::EntryUpdated(index) => {
                if let Some(entry_view_state) = self
                    .thread_view(&thread_id)
                    .map(|active| active.read(cx).entry_view_state.clone())
                {
                    entry_view_state.update(cx, |view_state, cx| {
                        view_state.sync_entry(*index, thread, window, cx)
                    });
                }
            }
            AcpThreadEvent::EntriesRemoved(range) => {
                if let Some(active) = self.thread_view(&thread_id) {
                    let entry_view_state = active.read(cx).entry_view_state.clone();
                    let list_state = active.read(cx).list_state.clone();
                    entry_view_state.update(cx, |view_state, _cx| view_state.remove(range.clone()));
                    list_state.splice(range.clone(), 0);
                }
            }
            AcpThreadEvent::SubagentSpawned(session_id) => self.load_subagent_session(
                session_id.clone(),
                thread.read(cx).session_id().clone(),
                window,
                cx,
            ),
            AcpThreadEvent::ToolAuthorizationRequired => {
                self.notify_with_sound("Waiting for tool confirmation", IconName::Info, window, cx);
            }
            AcpThreadEvent::Retry(retry) => {
                if let Some(active) = self.thread_view(&thread_id) {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status = Some(retry.clone());
                    });
                }
            }
            AcpThreadEvent::Stopped => {
                if let Some(active) = self.thread_view(&thread_id) {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status.take();
                    });
                }
                if is_subagent {
                    return;
                }

                let used_tools = thread.read(cx).used_tools_since_last_user_message();
                self.notify_with_sound(
                    if used_tools {
                        "Finished running tools"
                    } else {
                        "New message"
                    },
                    IconName::ZedAssistant,
                    window,
                    cx,
                );

                let should_send_queued = if let Some(active) = self.active_thread() {
                    active.update(cx, |active, cx| {
                        if active.skip_queue_processing_count > 0 {
                            active.skip_queue_processing_count -= 1;
                            false
                        } else if active.user_interrupted_generation {
                            // Manual interruption: don't auto-process queue.
                            // Reset the flag so future completions can process normally.
                            active.user_interrupted_generation = false;
                            false
                        } else {
                            let has_queued = !active.local_queued_messages.is_empty();
                            // Don't auto-send if the first message editor is currently focused
                            let is_first_editor_focused = active
                                .queued_message_editors
                                .first()
                                .is_some_and(|editor| editor.focus_handle(cx).is_focused(window));
                            has_queued && !is_first_editor_focused
                        }
                    })
                } else {
                    false
                };
                if should_send_queued {
                    self.send_queued_message_at_index(0, false, window, cx);
                }

                self.history.update(cx, |history, cx| history.refresh(cx));
            }
            AcpThreadEvent::Refusal => {
                let error = ThreadError::Refusal;
                if let Some(active) = self.thread_view(&thread_id) {
                    active.update(cx, |active, cx| {
                        active.handle_thread_error(error, cx);
                        active.thread_retry_status.take();
                    });
                }
                if !is_subagent {
                    let model_or_agent_name = self.current_model_name(cx);
                    let notification_message =
                        format!("{} refused to respond to this request", model_or_agent_name);
                    self.notify_with_sound(&notification_message, IconName::Warning, window, cx);
                }
            }
            AcpThreadEvent::Error => {
                if let Some(active) = self.thread_view(&thread_id) {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status.take();
                    });
                }
                if !is_subagent {
                    self.notify_with_sound(
                        "Agent stopped due to an error",
                        IconName::Warning,
                        window,
                        cx,
                    );
                }
            }
            AcpThreadEvent::LoadError(error) => {
                if let Some(view) = self.active_thread() {
                    if view
                        .read(cx)
                        .message_editor
                        .focus_handle(cx)
                        .is_focused(window)
                    {
                        self.focus_handle.focus(window, cx)
                    }
                }
                self.set_server_state(ServerState::LoadError(error.clone()), cx);
            }
            AcpThreadEvent::TitleUpdated => {
                let title = thread.read(cx).title();
                if let Some(title_editor) = self
                    .thread_view(&thread_id)
                    .and_then(|active| active.read(cx).title_editor.clone())
                {
                    title_editor.update(cx, |editor, cx| {
                        if editor.text(cx) != title {
                            editor.set_text(title, window, cx);
                        }
                    });
                }
                self.history.update(cx, |history, cx| history.refresh(cx));
            }
            AcpThreadEvent::PromptCapabilitiesUpdated => {
                if let Some(active) = self.thread_view(&thread_id) {
                    active.update(cx, |active, _cx| {
                        active
                            .prompt_capabilities
                            .replace(thread.read(_cx).prompt_capabilities());
                    });
                }
            }
            AcpThreadEvent::TokenUsageUpdated => {
                self.update_turn_tokens(cx);
                self.emit_token_limit_telemetry_if_needed(thread, cx);
            }
            AcpThreadEvent::AvailableCommandsUpdated(available_commands) => {
                let mut available_commands = available_commands.clone();

                if thread
                    .read(cx)
                    .connection()
                    .auth_methods()
                    .iter()
                    .any(|method| method.id.0.as_ref() == "claude-login")
                {
                    available_commands.push(acp::AvailableCommand::new("login", "Authenticate"));
                    available_commands.push(acp::AvailableCommand::new("logout", "Authenticate"));
                }

                let has_commands = !available_commands.is_empty();
                if let Some(active) = self.active_thread() {
                    active.update(cx, |active, _cx| {
                        active.available_commands.replace(available_commands);
                    });
                }

                let agent_display_name = self
                    .agent_server_store
                    .read(cx)
                    .agent_display_name(&ExternalAgentServerName(self.agent.name()))
                    .unwrap_or_else(|| self.agent.name());

                if let Some(active) = self.active_thread() {
                    let new_placeholder =
                        placeholder_text(agent_display_name.as_ref(), has_commands);
                    active.update(cx, |active, cx| {
                        active.message_editor.update(cx, |editor, cx| {
                            editor.set_placeholder_text(&new_placeholder, window, cx);
                        });
                    });
                }
            }
            AcpThreadEvent::ModeUpdated(_mode) => {
                // The connection keeps track of the mode
                cx.notify();
            }
            AcpThreadEvent::ConfigOptionsUpdated(_) => {
                // The watch task in ConfigOptionsView handles rebuilding selectors
                cx.notify();
            }
        }
        cx.notify();
    }

    fn authenticate(
        &mut self,
        method: acp::AuthMethodId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(connected) = self.as_connected_mut() else {
            return;
        };
        let connection = connected.connection.clone();

        let AuthState::Unauthenticated {
            configuration_view,
            pending_auth_method,
            ..
        } = &mut connected.auth_state
        else {
            return;
        };

        let agent_telemetry_id = connection.telemetry_id();

        // Check for the experimental "terminal-auth" _meta field
        let auth_method = connection.auth_methods().iter().find(|m| m.id == method);

        if let Some(terminal_auth) = auth_method
            .and_then(|a| a.meta.as_ref())
            .and_then(|m| m.get("terminal-auth"))
        {
            // Extract terminal auth details from meta
            if let (Some(command), Some(label)) = (
                terminal_auth.get("command").and_then(|v| v.as_str()),
                terminal_auth.get("label").and_then(|v| v.as_str()),
            ) {
                let args = terminal_auth
                    .get("args")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                let env = terminal_auth
                    .get("env")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                            .collect::<HashMap<String, String>>()
                    })
                    .unwrap_or_default();

                // Run SpawnInTerminal in the same dir as the ACP server
                let cwd = connected
                    .connection
                    .clone()
                    .downcast::<agent_servers::AcpConnection>()
                    .map(|acp_conn| acp_conn.root_dir().to_path_buf());

                // Build SpawnInTerminal from _meta
                let login = task::SpawnInTerminal {
                    id: task::TaskId(format!("external-agent-{}-login", label)),
                    full_label: label.to_string(),
                    label: label.to_string(),
                    command: Some(command.to_string()),
                    args,
                    command_label: label.to_string(),
                    cwd,
                    env,
                    use_new_terminal: true,
                    allow_concurrent_runs: true,
                    hide: task::HideStrategy::Always,
                    ..Default::default()
                };

                configuration_view.take();
                pending_auth_method.replace(method.clone());

                if let Some(workspace) = self.workspace.upgrade() {
                    let project = self.project.clone();
                    let authenticate = Self::spawn_external_agent_login(
                        login,
                        workspace,
                        project,
                        method.clone(),
                        false,
                        window,
                        cx,
                    );
                    cx.notify();
                    self.auth_task = Some(cx.spawn_in(window, {
                        async move |this, cx| {
                            let result = authenticate.await;

                            match &result {
                                Ok(_) => telemetry::event!(
                                    "Authenticate Agent Succeeded",
                                    agent = agent_telemetry_id
                                ),
                                Err(_) => {
                                    telemetry::event!(
                                        "Authenticate Agent Failed",
                                        agent = agent_telemetry_id,
                                    )
                                }
                            }

                            this.update_in(cx, |this, window, cx| {
                                if let Err(err) = result {
                                    if let Some(ConnectedServerState {
                                        auth_state:
                                            AuthState::Unauthenticated {
                                                pending_auth_method,
                                                ..
                                            },
                                        ..
                                    }) = this.as_connected_mut()
                                    {
                                        pending_auth_method.take();
                                    }
                                    if let Some(active) = this.active_thread() {
                                        active.update(cx, |active, cx| {
                                            active.handle_any_thread_error(err, cx);
                                        })
                                    }
                                } else {
                                    this.reset(window, cx);
                                }
                                this.auth_task.take()
                            })
                            .ok();
                        }
                    }));
                }
                return;
            }
        }

        if method.0.as_ref() == "gemini-api-key" {
            let registry = LanguageModelRegistry::global(cx);
            let provider = registry
                .read(cx)
                .provider(&language_model::GOOGLE_PROVIDER_ID)
                .unwrap();
            if !provider.is_authenticated(cx) {
                let this = cx.weak_entity();
                let agent_name = self.agent.name();
                window.defer(cx, |window, cx| {
                    Self::handle_auth_required(
                        this,
                        AuthRequired {
                            description: Some("GEMINI_API_KEY must be set".to_owned()),
                            provider_id: Some(language_model::GOOGLE_PROVIDER_ID),
                        },
                        agent_name,
                        connection,
                        window,
                        cx,
                    );
                });
                return;
            }
        } else if method.0.as_ref() == "vertex-ai"
            && std::env::var("GOOGLE_API_KEY").is_err()
            && (std::env::var("GOOGLE_CLOUD_PROJECT").is_err()
                || (std::env::var("GOOGLE_CLOUD_PROJECT").is_err()))
        {
            let this = cx.weak_entity();
            let agent_name = self.agent.name();

            window.defer(cx, |window, cx| {
                    Self::handle_auth_required(
                        this,
                        AuthRequired {
                            description: Some(
                                "GOOGLE_API_KEY must be set in the environment to use Vertex AI authentication for Gemini CLI. Please export it and restart Zed."
                                    .to_owned(),
                            ),
                            provider_id: None,
                        },
                        agent_name,
                        connection,
                        window,
                        cx,
                    )
                });
            return;
        }

        configuration_view.take();
        pending_auth_method.replace(method.clone());
        let authenticate = if let Some(login) = self.login.clone() {
            if let Some(workspace) = self.workspace.upgrade() {
                let project = self.project.clone();
                Self::spawn_external_agent_login(
                    login,
                    workspace,
                    project,
                    method.clone(),
                    false,
                    window,
                    cx,
                )
            } else {
                Task::ready(Ok(()))
            }
        } else {
            connection.authenticate(method, cx)
        };
        cx.notify();
        self.auth_task = Some(cx.spawn_in(window, {
            async move |this, cx| {
                let result = authenticate.await;

                match &result {
                    Ok(_) => telemetry::event!(
                        "Authenticate Agent Succeeded",
                        agent = agent_telemetry_id
                    ),
                    Err(_) => {
                        telemetry::event!("Authenticate Agent Failed", agent = agent_telemetry_id,)
                    }
                }

                this.update_in(cx, |this, window, cx| {
                    if let Err(err) = result {
                        if let Some(ConnectedServerState {
                            auth_state:
                                AuthState::Unauthenticated {
                                    pending_auth_method,
                                    ..
                                },
                            ..
                        }) = this.as_connected_mut()
                        {
                            pending_auth_method.take();
                        }
                        if let Some(active) = this.active_thread() {
                            active.update(cx, |active, cx| active.handle_any_thread_error(err, cx));
                        }
                    } else {
                        this.reset(window, cx);
                    }
                    this.auth_task.take()
                })
                .ok();
            }
        }));
    }

    fn load_subagent_session(
        &mut self,
        subagent_id: acp::SessionId,
        parent_id: acp::SessionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(connected) = self.as_connected() else {
            return;
        };
        if connected.threads.contains_key(&subagent_id)
            || !connected.connection.supports_load_session(cx)
        {
            return;
        }
        let root_dir = self
            .project
            .read(cx)
            .worktrees(cx)
            .filter_map(|worktree| {
                if worktree.read(cx).is_single_file() {
                    Some(worktree.read(cx).abs_path().parent()?.into())
                } else {
                    Some(worktree.read(cx).abs_path())
                }
            })
            .next();
        let cwd = root_dir.unwrap_or_else(|| paths::home_dir().as_path().into());

        let subagent_thread_task = connected.connection.clone().load_session(
            AgentSessionInfo::new(subagent_id.clone()),
            self.project.clone(),
            &cwd,
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            let subagent_thread = subagent_thread_task.await?;
            this.update_in(cx, |this, window, cx| {
                let view = this.new_thread_view(
                    Some(parent_id),
                    subagent_thread,
                    false,
                    None,
                    None,
                    window,
                    cx,
                );
                let Some(connected) = this.as_connected_mut() else {
                    return;
                };
                connected.threads.insert(subagent_id, view);
            })
        })
        .detach();
    }

    fn spawn_external_agent_login(
        login: task::SpawnInTerminal,
        workspace: Entity<Workspace>,
        project: Entity<Project>,
        method: acp::AuthMethodId,
        previous_attempt: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
            return Task::ready(Ok(()));
        };

        window.spawn(cx, async move |cx| {
            let mut task = login.clone();
            if let Some(cmd) = &task.command {
                // Have "node" command use Zed's managed Node runtime by default
                if cmd == "node" {
                    let resolved_node_runtime = project
                        .update(cx, |project, cx| {
                            let agent_server_store = project.agent_server_store().clone();
                            agent_server_store.update(cx, |store, cx| {
                                store.node_runtime().map(|node_runtime| {
                                    cx.background_spawn(async move {
                                        node_runtime.binary_path().await
                                    })
                                })
                            })
                        });

                    if let Some(resolve_task) = resolved_node_runtime {
                        if let Ok(node_path) = resolve_task.await {
                            task.command = Some(node_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
            task.shell = task::Shell::WithArguments {
                program: task.command.take().expect("login command should be set"),
                args: std::mem::take(&mut task.args),
                title_override: None
            };
            task.full_label = task.label.clone();
            task.id = task::TaskId(format!("external-agent-{}-login", task.label));
            task.command_label = task.label.clone();
            task.use_new_terminal = true;
            task.allow_concurrent_runs = true;
            task.hide = task::HideStrategy::Always;

            let terminal = terminal_panel
                .update_in(cx, |terminal_panel, window, cx| {
                    terminal_panel.spawn_task(&task, window, cx)
                })?
                .await?;

            let success_patterns = match method.0.as_ref() {
                "claude-login" | "spawn-gemini-cli" => vec![
                    "Login successful".to_string(),
                    "Type your message".to_string(),
                ],
                _ => Vec::new(),
            };
            if success_patterns.is_empty() {
                // No success patterns specified: wait for the process to exit and check exit code
                let exit_status = terminal
                    .read_with(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .await;

                match exit_status {
                    Some(status) if status.success() => Ok(()),
                    Some(status) => Err(anyhow!(
                        "Login command failed with exit code: {:?}",
                        status.code()
                    )),
                    None => Err(anyhow!("Login command terminated without exit status")),
                }
            } else {
                // Look for specific output patterns to detect successful login
                let mut exit_status = terminal
                    .read_with(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .fuse();

                let logged_in = cx
                    .spawn({
                        let terminal = terminal.clone();
                        async move |cx| {
                            loop {
                                cx.background_executor().timer(Duration::from_secs(1)).await;
                                let content =
                                    terminal.update(cx, |terminal, _cx| terminal.get_content())?;
                                if success_patterns.iter().any(|pattern| content.contains(pattern))
                                {
                                    return anyhow::Ok(());
                                }
                            }
                        }
                    })
                    .fuse();
                futures::pin_mut!(logged_in);
                futures::select_biased! {
                    result = logged_in => {
                        if let Err(e) = result {
                            log::error!("{e}");
                            return Err(anyhow!("exited before logging in"));
                        }
                    }
                    _ = exit_status => {
                        if !previous_attempt && project.read_with(cx, |project, _| project.is_via_remote_server()) && login.label.contains("gemini") {
                            return cx.update(|window, cx| Self::spawn_external_agent_login(login, workspace, project.clone(), method, true, window, cx))?.await
                        }
                        return Err(anyhow!("exited before logging in"));
                    }
                }
                terminal.update(cx, |terminal, _| terminal.kill_active_task())?;
                Ok(())
            }
        })
    }

    pub fn has_user_submitted_prompt(&self, cx: &App) -> bool {
        self.active_thread().is_some_and(|active| {
            active
                .read(cx)
                .thread
                .read(cx)
                .entries()
                .iter()
                .any(|entry| {
                    matches!(
                        entry,
                        AgentThreadEntry::UserMessage(user_message) if user_message.id.is_some()
                    )
                })
        })
    }

    fn render_auth_required_state(
        &self,
        connection: &Rc<dyn AgentConnection>,
        description: Option<&Entity<Markdown>>,
        configuration_view: Option<&AnyView>,
        pending_auth_method: Option<&acp::AuthMethodId>,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let auth_methods = connection.auth_methods();

        let agent_display_name = self
            .agent_server_store
            .read(cx)
            .agent_display_name(&ExternalAgentServerName(self.agent.name()))
            .unwrap_or_else(|| self.agent.name());

        let show_fallback_description = auth_methods.len() > 1
            && configuration_view.is_none()
            && description.is_none()
            && pending_auth_method.is_none();

        let auth_buttons = || {
            h_flex().justify_end().flex_wrap().gap_1().children(
                connection
                    .auth_methods()
                    .iter()
                    .enumerate()
                    .rev()
                    .map(|(ix, method)| {
                        let (method_id, name) = if self.project.read(cx).is_via_remote_server()
                            && method.id.0.as_ref() == "oauth-personal"
                            && method.name == "Log in with Google"
                        {
                            ("spawn-gemini-cli".into(), "Log in with Gemini CLI".into())
                        } else {
                            (method.id.0.clone(), method.name.clone())
                        };

                        let agent_telemetry_id = connection.telemetry_id();

                        Button::new(method_id.clone(), name)
                            .label_size(LabelSize::Small)
                            .map(|this| {
                                if ix == 0 {
                                    this.style(ButtonStyle::Tinted(TintColor::Accent))
                                } else {
                                    this.style(ButtonStyle::Outlined)
                                }
                            })
                            .when_some(method.description.clone(), |this, description| {
                                this.tooltip(Tooltip::text(description))
                            })
                            .on_click({
                                cx.listener(move |this, _, window, cx| {
                                    telemetry::event!(
                                        "Authenticate Agent Started",
                                        agent = agent_telemetry_id,
                                        method = method_id
                                    );

                                    this.authenticate(
                                        acp::AuthMethodId::new(method_id.clone()),
                                        window,
                                        cx,
                                    )
                                })
                            })
                    }),
            )
        };

        if pending_auth_method.is_some() {
            return Callout::new()
                .icon(IconName::Info)
                .title(format!("Authenticating to {}…", agent_display_name))
                .actions_slot(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_rotate_animation(2)
                        .into_any_element(),
                )
                .into_any_element();
        }

        Callout::new()
            .icon(IconName::Info)
            .title(format!("Authenticate to {}", agent_display_name))
            .when(auth_methods.len() == 1, |this| {
                this.actions_slot(auth_buttons())
            })
            .description_slot(
                v_flex()
                    .text_ui(cx)
                    .map(|this| {
                        if show_fallback_description {
                            this.child(
                                Label::new("Choose one of the following authentication options:")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        } else {
                            this.children(
                                configuration_view
                                    .cloned()
                                    .map(|view| div().w_full().child(view)),
                            )
                            .children(description.map(|desc| {
                                self.render_markdown(
                                    desc.clone(),
                                    MarkdownStyle::themed(MarkdownFont::Agent, window, cx),
                                )
                            }))
                        }
                    })
                    .when(auth_methods.len() > 1, |this| {
                        this.gap_1().child(auth_buttons())
                    }),
            )
            .into_any_element()
    }

    fn emit_token_limit_telemetry_if_needed(
        &mut self,
        thread: &Entity<AcpThread>,
        cx: &mut Context<Self>,
    ) {
        let Some(active_thread) = self.active_thread() else {
            return;
        };

        let (ratio, agent_telemetry_id, session_id) = {
            let thread_data = thread.read(cx);
            let Some(token_usage) = thread_data.token_usage() else {
                return;
            };
            (
                token_usage.ratio(),
                thread_data.connection().telemetry_id(),
                thread_data.session_id().clone(),
            )
        };

        let kind = match ratio {
            acp_thread::TokenUsageRatio::Normal => {
                active_thread.update(cx, |active, _cx| {
                    active.last_token_limit_telemetry = None;
                });
                return;
            }
            acp_thread::TokenUsageRatio::Warning => "warning",
            acp_thread::TokenUsageRatio::Exceeded => "exceeded",
        };

        let should_skip = active_thread
            .read(cx)
            .last_token_limit_telemetry
            .as_ref()
            .is_some_and(|last| *last >= ratio);
        if should_skip {
            return;
        }

        active_thread.update(cx, |active, _cx| {
            active.last_token_limit_telemetry = Some(ratio);
        });

        telemetry::event!(
            "Agent Token Limit Warning",
            agent = agent_telemetry_id,
            session_id = session_id,
            kind = kind,
        );
    }

    fn emit_load_error_telemetry(&self, error: &LoadError) {
        let error_kind = match error {
            LoadError::Unsupported { .. } => "unsupported",
            LoadError::FailedToInstall(_) => "failed_to_install",
            LoadError::Exited { .. } => "exited",
            LoadError::Other(_) => "other",
        };

        let agent_name = self.agent.name();

        telemetry::event!(
            "Agent Panel Error Shown",
            agent = agent_name,
            kind = error_kind,
            message = error.to_string(),
        );
    }

    fn render_load_error(
        &self,
        e: &LoadError,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (title, message, action_slot): (_, SharedString, _) = match e {
            LoadError::Unsupported {
                command: path,
                current_version,
                minimum_version,
            } => {
                return self.render_unsupported(path, current_version, minimum_version, window, cx);
            }
            LoadError::FailedToInstall(msg) => (
                "Failed to Install",
                msg.into(),
                Some(self.create_copy_button(msg.to_string()).into_any_element()),
            ),
            LoadError::Exited { status } => (
                "Failed to Launch",
                format!("Server exited with status {status}").into(),
                None,
            ),
            LoadError::Other(msg) => (
                "Failed to Launch",
                msg.into(),
                Some(self.create_copy_button(msg.to_string()).into_any_element()),
            ),
        };

        Callout::new()
            .severity(Severity::Error)
            .icon(IconName::XCircleFilled)
            .title(title)
            .description(message)
            .actions_slot(div().children(action_slot))
            .into_any_element()
    }

    fn render_unsupported(
        &self,
        path: &SharedString,
        version: &SharedString,
        minimum_version: &SharedString,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (heading_label, description_label) = (
            format!("Upgrade {} to work with Zed", self.agent.name()),
            if version.is_empty() {
                format!(
                    "Currently using {}, which does not report a valid --version",
                    path,
                )
            } else {
                format!(
                    "Currently using {}, which is only version {} (need at least {minimum_version})",
                    path, version
                )
            },
        );

        v_flex()
            .w_full()
            .p_3p5()
            .gap_2p5()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(linear_gradient(
                180.,
                linear_color_stop(cx.theme().colors().editor_background.opacity(0.4), 4.),
                linear_color_stop(cx.theme().status().info_background.opacity(0.), 0.),
            ))
            .child(
                v_flex().gap_0p5().child(Label::new(heading_label)).child(
                    Label::new(description_label)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
            .into_any_element()
    }

    pub(crate) fn as_native_connection(
        &self,
        cx: &App,
    ) -> Option<Rc<agent::NativeAgentConnection>> {
        let acp_thread = self.active_thread()?.read(cx).thread.read(cx);
        acp_thread.connection().clone().downcast()
    }

    pub(crate) fn as_native_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        let acp_thread = self.active_thread()?.read(cx).thread.read(cx);
        self.as_native_connection(cx)?
            .thread(acp_thread.session_id(), cx)
    }

    fn queued_messages_len(&self, cx: &App) -> usize {
        self.active_thread()
            .map(|thread| thread.read(cx).local_queued_messages.len())
            .unwrap_or_default()
    }

    fn update_queued_message(
        &mut self,
        index: usize,
        content: Vec<acp::ContentBlock>,
        tracked_buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) -> bool {
        match self.active_thread() {
            Some(thread) => thread.update(cx, |thread, _cx| {
                if index < thread.local_queued_messages.len() {
                    thread.local_queued_messages[index] = QueuedMessage {
                        content,
                        tracked_buffers,
                    };
                    true
                } else {
                    false
                }
            }),
            None => false,
        }
    }

    fn queued_message_contents(&self, cx: &App) -> Vec<Vec<acp::ContentBlock>> {
        match self.active_thread() {
            None => Vec::new(),
            Some(thread) => thread
                .read(cx)
                .local_queued_messages
                .iter()
                .map(|q| q.content.clone())
                .collect(),
        }
    }

    fn save_queued_message_at_index(&mut self, index: usize, cx: &mut Context<Self>) {
        let editor = match self.active_thread() {
            Some(thread) => thread.read(cx).queued_message_editors.get(index).cloned(),
            None => None,
        };
        let Some(editor) = editor else {
            return;
        };

        let contents_task = editor.update(cx, |editor, cx| editor.contents(false, cx));

        cx.spawn(async move |this, cx| {
            let Ok((content, tracked_buffers)) = contents_task.await else {
                return Ok::<(), anyhow::Error>(());
            };

            this.update(cx, |this, cx| {
                this.update_queued_message(index, content, tracked_buffers, cx);
                cx.notify();
            })?;

            Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn sync_queued_message_editors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let needed_count = self.queued_messages_len(cx);
        let queued_messages = self.queued_message_contents(cx);

        let agent_name = self.agent.name();
        let workspace = self.workspace.clone();
        let project = self.project.downgrade();
        let history = self.history.downgrade();

        let Some(thread) = self.active_thread() else {
            return;
        };
        let prompt_capabilities = thread.read(cx).prompt_capabilities.clone();
        let available_commands = thread.read(cx).available_commands.clone();

        let current_count = thread.read(cx).queued_message_editors.len();
        let last_synced = thread.read(cx).last_synced_queue_length;

        if current_count == needed_count && needed_count == last_synced {
            return;
        }

        if current_count > needed_count {
            thread.update(cx, |thread, _cx| {
                thread.queued_message_editors.truncate(needed_count);
                thread
                    .queued_message_editor_subscriptions
                    .truncate(needed_count);
            });

            let editors = thread.read(cx).queued_message_editors.clone();
            for (index, editor) in editors.into_iter().enumerate() {
                if let Some(content) = queued_messages.get(index) {
                    editor.update(cx, |editor, cx| {
                        editor.set_message(content.clone(), window, cx);
                    });
                }
            }
        }

        while thread.read(cx).queued_message_editors.len() < needed_count {
            let index = thread.read(cx).queued_message_editors.len();
            let content = queued_messages.get(index).cloned().unwrap_or_default();

            let editor = cx.new(|cx| {
                let mut editor = MessageEditor::new(
                    workspace.clone(),
                    project.clone(),
                    None,
                    history.clone(),
                    None,
                    prompt_capabilities.clone(),
                    available_commands.clone(),
                    agent_name.clone(),
                    "",
                    EditorMode::AutoHeight {
                        min_lines: 1,
                        max_lines: Some(10),
                    },
                    window,
                    cx,
                );
                editor.set_message(content, window, cx);
                editor
            });

            let subscription = cx.subscribe_in(
                &editor,
                window,
                move |this, _editor, event, window, cx| match event {
                    MessageEditorEvent::LostFocus => {
                        this.save_queued_message_at_index(index, cx);
                    }
                    MessageEditorEvent::Cancel => {
                        window.focus(&this.focus_handle(cx), cx);
                    }
                    MessageEditorEvent::Send => {
                        window.focus(&this.focus_handle(cx), cx);
                    }
                    MessageEditorEvent::SendImmediately => {
                        this.send_queued_message_at_index(index, true, window, cx);
                    }
                    _ => {}
                },
            );

            thread.update(cx, |thread, _cx| {
                thread.queued_message_editors.push(editor);
                thread
                    .queued_message_editor_subscriptions
                    .push(subscription);
            });
        }

        if let Some(active) = self.active_thread() {
            active.update(cx, |active, _cx| {
                active.last_synced_queue_length = needed_count;
            });
        }
    }

    fn render_markdown(&self, markdown: Entity<Markdown>, style: MarkdownStyle) -> MarkdownElement {
        let workspace = self.workspace.clone();
        MarkdownElement::new(markdown, style).on_url_click(move |text, window, cx| {
            crate::acp::thread_view::active_thread::open_link(text, &workspace, window, cx);
        })
    }

    fn notify_with_sound(
        &mut self,
        caption: impl Into<SharedString>,
        icon: IconName,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.play_notification_sound(window, cx);
        self.show_notification(caption, icon, window, cx);
    }

    fn play_notification_sound(&self, window: &Window, cx: &mut App) {
        let settings = AgentSettings::get_global(cx);
        if settings.play_sound_when_agent_done && !window.is_window_active() {
            Audio::play_sound(Sound::AgentDone, cx);
        }
    }

    fn show_notification(
        &mut self,
        caption: impl Into<SharedString>,
        icon: IconName,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.notifications.is_empty() {
            return;
        }

        let settings = AgentSettings::get_global(cx);

        let window_is_inactive = !window.is_window_active();
        let panel_is_hidden = self
            .workspace
            .upgrade()
            .map(|workspace| AgentPanel::is_hidden(&workspace, cx))
            .unwrap_or(true);

        let should_notify = window_is_inactive || panel_is_hidden;

        if !should_notify {
            return;
        }

        // TODO: Change this once we have title summarization for external agents.
        let title = self.agent.name();

        match settings.notify_when_agent_waiting {
            NotifyWhenAgentWaiting::PrimaryScreen => {
                if let Some(primary) = cx.primary_display() {
                    self.pop_up(icon, caption.into(), title, window, primary, cx);
                }
            }
            NotifyWhenAgentWaiting::AllScreens => {
                let caption = caption.into();
                for screen in cx.displays() {
                    self.pop_up(icon, caption.clone(), title.clone(), window, screen, cx);
                }
            }
            NotifyWhenAgentWaiting::Never => {
                // Don't show anything
            }
        }
    }

    fn pop_up(
        &mut self,
        icon: IconName,
        caption: SharedString,
        title: SharedString,
        window: &mut Window,
        screen: Rc<dyn PlatformDisplay>,
        cx: &mut Context<Self>,
    ) {
        let options = AgentNotification::window_options(screen, cx);

        let project_name = self.workspace.upgrade().and_then(|workspace| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).root_name_str().to_string())
        });

        if let Some(screen_window) = cx
            .open_window(options, |_window, cx| {
                cx.new(|_cx| {
                    AgentNotification::new(title.clone(), caption.clone(), icon, project_name)
                })
            })
            .log_err()
            && let Some(pop_up) = screen_window.entity(cx).log_err()
        {
            self.notification_subscriptions
                .entry(screen_window)
                .or_insert_with(Vec::new)
                .push(cx.subscribe_in(&pop_up, window, {
                    |this, _, event, window, cx| match event {
                        AgentNotificationEvent::Accepted => {
                            let handle = window.window_handle();
                            cx.activate(true);

                            let workspace_handle = this.workspace.clone();

                            // If there are multiple Zed windows, activate the correct one.
                            cx.defer(move |cx| {
                                handle
                                    .update(cx, |_view, window, _cx| {
                                        window.activate_window();

                                        if let Some(workspace) = workspace_handle.upgrade() {
                                            workspace.update(_cx, |workspace, cx| {
                                                workspace.focus_panel::<AgentPanel>(window, cx);
                                            });
                                        }
                                    })
                                    .log_err();
                            });

                            this.dismiss_notifications(cx);
                        }
                        AgentNotificationEvent::Dismissed => {
                            this.dismiss_notifications(cx);
                        }
                    }
                }));

            self.notifications.push(screen_window);

            // If the user manually refocuses the original window, dismiss the popup.
            self.notification_subscriptions
                .entry(screen_window)
                .or_insert_with(Vec::new)
                .push({
                    let pop_up_weak = pop_up.downgrade();

                    cx.observe_window_activation(window, move |_, window, cx| {
                        if window.is_window_active()
                            && let Some(pop_up) = pop_up_weak.upgrade()
                        {
                            pop_up.update(cx, |_, cx| {
                                cx.emit(AgentNotificationEvent::Dismissed);
                            });
                        }
                    })
                });
        }
    }

    fn dismiss_notifications(&mut self, cx: &mut Context<Self>) {
        for window in self.notifications.drain(..) {
            window
                .update(cx, |_, window, _| {
                    window.remove_window();
                })
                .ok();

            self.notification_subscriptions.remove(&window);
        }
    }

    fn agent_ui_font_size_changed(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry_view_state) = self
            .active_thread()
            .map(|active| active.read(cx).entry_view_state.clone())
        {
            entry_view_state.update(cx, |entry_view_state, cx| {
                entry_view_state.agent_ui_font_size_changed(cx);
            });
        }
    }

    pub(crate) fn insert_dragged_files(
        &self,
        paths: Vec<project::ProjectPath>,
        added_worktrees: Vec<Entity<project::Worktree>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active_thread) = self.active_thread() {
            active_thread.update(cx, |thread, cx| {
                thread.message_editor.update(cx, |editor, cx| {
                    editor.insert_dragged_files(paths, added_worktrees, window, cx);
                    editor.focus_handle(cx).focus(window, cx);
                })
            });
        }
    }

    /// Inserts the selected text into the message editor or the message being
    /// edited, if any.
    pub(crate) fn insert_selections(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_thread) = self.active_thread() {
            active_thread.update(cx, |thread, cx| {
                thread.active_editor(cx).update(cx, |editor, cx| {
                    editor.insert_selections(window, cx);
                })
            });
        }
    }

    /// Inserts terminal text as a crease into the message editor.
    pub(crate) fn insert_terminal_text(
        &self,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active_thread) = self.active_thread() {
            active_thread.update(cx, |thread, cx| {
                thread.message_editor.update(cx, |editor, cx| {
                    editor.insert_terminal_crease(text, window, cx);
                })
            });
        }
    }

    fn current_model_name(&self, cx: &App) -> SharedString {
        // For native agent (Zed Agent), use the specific model name (e.g., "Claude 3.5 Sonnet")
        // For ACP agents, use the agent name (e.g., "Claude Code", "Gemini CLI")
        // This provides better clarity about what refused the request
        if self.as_native_connection(cx).is_some() {
            self.active_thread()
                .and_then(|active| active.read(cx).model_selector.clone())
                .and_then(|selector| selector.read(cx).active_model(cx))
                .map(|model| model.name.clone())
                .unwrap_or_else(|| SharedString::from("The model"))
        } else {
            // ACP agent - use the agent name (e.g., "Claude Code", "Gemini CLI")
            self.agent.name()
        }
    }

    fn create_copy_button(&self, message: impl Into<String>) -> impl IntoElement {
        let message = message.into();

        CopyButton::new("copy-error-message", message).tooltip_label("Copy Error Message")
    }

    pub(crate) fn reauthenticate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let agent_name = self.agent.name();
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, cx| active.clear_thread_error(cx));
        }
        let this = cx.weak_entity();
        let Some(connection) = self.as_connected().map(|c| c.connection.clone()) else {
            debug_panic!("This should not be possible");
            return;
        };
        window.defer(cx, |window, cx| {
            Self::handle_auth_required(
                this,
                AuthRequired::new(),
                agent_name,
                connection,
                window,
                cx,
            );
        })
    }

    pub fn delete_history_entry(&mut self, entry: AgentSessionInfo, cx: &mut Context<Self>) {
        let task = self.history.update(cx, |history, cx| {
            history.delete_session(&entry.session_id, cx)
        });
        task.detach_and_log_err(cx);
    }
}

fn loading_contents_spinner(size: IconSize) -> AnyElement {
    Icon::new(IconName::LoadCircle)
        .size(size)
        .color(Color::Accent)
        .with_rotate_animation(3)
        .into_any_element()
}

fn placeholder_text(agent_name: &str, has_commands: bool) -> String {
    if agent_name == "Zed Agent" {
        format!("Message the {} — @ to include context", agent_name)
    } else if has_commands {
        format!(
            "Message {} — @ to include context, / for commands",
            agent_name
        )
    } else {
        format!("Message {} — @ to include context", agent_name)
    }
}

impl Focusable for AcpServerView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.active_thread() {
            Some(thread) => thread.read(cx).focus_handle(cx),
            None => self.focus_handle.clone(),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl AcpServerView {
    /// Expands a tool call so its content is visible.
    /// This is primarily useful for visual testing.
    pub fn expand_tool_call(&mut self, tool_call_id: acp::ToolCallId, cx: &mut Context<Self>) {
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, _cx| {
                active.expanded_tool_calls.insert(tool_call_id);
            });
            cx.notify();
        }
    }

    /// Expands a subagent card so its content is visible.
    /// This is primarily useful for visual testing.
    pub fn expand_subagent(&mut self, session_id: acp::SessionId, cx: &mut Context<Self>) {
        if let Some(active) = self.active_thread() {
            active.update(cx, |active, _cx| {
                active.expanded_subagents.insert(session_id);
            });
            cx.notify();
        }
    }
}

impl Render for AcpServerView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_queued_message_editors(window, cx);

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(match &self.server_state {
                ServerState::Loading { .. } => v_flex()
                    .flex_1()
                    // .child(self.render_recent_history(cx))
                    .into_any(),
                ServerState::LoadError(e) => v_flex()
                    .flex_1()
                    .size_full()
                    .items_center()
                    .justify_end()
                    .child(self.render_load_error(e, window, cx))
                    .into_any(),
                ServerState::Connected(ConnectedServerState {
                    connection,
                    auth_state:
                        AuthState::Unauthenticated {
                            description,
                            configuration_view,
                            pending_auth_method,
                            _subscription,
                        },
                    ..
                }) => v_flex()
                    .flex_1()
                    .size_full()
                    .justify_end()
                    .child(self.render_auth_required_state(
                        connection,
                        description.as_ref(),
                        configuration_view.as_ref(),
                        pending_auth_method.as_ref(),
                        window,
                        cx,
                    ))
                    .into_any_element(),
                ServerState::Connected(connected) => {
                    if let Some(view) = connected.active_view() {
                        view.clone().into_any_element()
                    } else {
                        debug_panic!("This state should never be reached");
                        div().into_any_element()
                    }
                }
            })
    }
}

fn plan_label_markdown_style(
    status: &acp::PlanEntryStatus,
    window: &Window,
    cx: &App,
) -> MarkdownStyle {
    let default_md_style = MarkdownStyle::themed(MarkdownFont::Agent, window, cx);

    MarkdownStyle {
        base_text_style: TextStyle {
            color: cx.theme().colors().text_muted,
            strikethrough: if matches!(status, acp::PlanEntryStatus::Completed) {
                Some(gpui::StrikethroughStyle {
                    thickness: px(1.),
                    color: Some(cx.theme().colors().text_muted.opacity(0.8)),
                })
            } else {
                None
            },
            ..default_md_style.base_text_style
        },
        ..default_md_style
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use acp_thread::{
        AgentSessionList, AgentSessionListRequest, AgentSessionListResponse, StubAgentConnection,
    };
    use action_log::ActionLog;
    use agent::{AgentTool, EditFileTool, FetchTool, TerminalTool, ToolPermissionContext};
    use agent_client_protocol::SessionId;
    use editor::MultiBufferOffset;
    use fs::FakeFs;
    use gpui::{EventEmitter, TestAppContext, VisualTestContext};
    use parking_lot::Mutex;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::any::Any;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;
    use std::sync::Arc;
    use workspace::Item;

    use super::*;

    #[gpui::test]
    async fn test_drop(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, _cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;
        let weak_view = thread_view.downgrade();
        drop(thread_view);
        assert!(!weak_view.is_upgradable());
    }

    #[gpui::test]
    async fn test_notification_for_stop_event(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some())
        );
    }

    #[gpui::test]
    async fn test_notification_for_error(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(SaboteurAgentConnection), cx).await;

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some())
        );
    }

    #[gpui::test]
    async fn test_recent_history_refreshes_when_history_cache_updated(cx: &mut TestAppContext) {
        init_test(cx);

        let session_a = AgentSessionInfo::new(SessionId::new("session-a"));
        let session_b = AgentSessionInfo::new(SessionId::new("session-b"));

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        // Create history without an initial session list - it will be set after connection
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(StubAgentServer::default_response()),
                    None,
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history.clone(),
                    window,
                    cx,
                )
            })
        });

        // Wait for connection to establish
        cx.run_until_parked();

        // Initially empty because StubAgentConnection.session_list() returns None
        active_thread(&thread_view, cx).read_with(cx, |view, _cx| {
            assert_eq!(view.recent_history_entries.len(), 0);
        });

        // Now set the session list - this simulates external agents providing their history
        let list_a: Rc<dyn AgentSessionList> =
            Rc::new(StubSessionList::new(vec![session_a.clone()]));
        history.update(cx, |history, cx| {
            history.set_session_list(Some(list_a), cx);
        });
        cx.run_until_parked();

        active_thread(&thread_view, cx).read_with(cx, |view, _cx| {
            assert_eq!(view.recent_history_entries.len(), 1);
            assert_eq!(
                view.recent_history_entries[0].session_id,
                session_a.session_id
            );
        });

        // Update to a different session list
        let list_b: Rc<dyn AgentSessionList> =
            Rc::new(StubSessionList::new(vec![session_b.clone()]));
        history.update(cx, |history, cx| {
            history.set_session_list(Some(list_b), cx);
        });
        cx.run_until_parked();

        active_thread(&thread_view, cx).read_with(cx, |view, _cx| {
            assert_eq!(view.recent_history_entries.len(), 1);
            assert_eq!(
                view.recent_history_entries[0].session_id,
                session_b.session_id
            );
        });
    }

    #[gpui::test]
    async fn test_resume_without_history_adds_notice(cx: &mut TestAppContext) {
        init_test(cx);

        let session = AgentSessionInfo::new(SessionId::new("resume-session"));
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(StubAgentServer::new(ResumeOnlyAgentConnection)),
                    Some(session),
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        thread_view.read_with(cx, |view, cx| {
            let state = view.active_thread().unwrap();
            assert!(state.read(cx).resumed_without_history);
            assert_eq!(state.read(cx).list_state.item_count(), 0);
        });
    }

    #[gpui::test]
    async fn test_resume_thread_uses_session_cwd_when_inside_project(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "subdir": {
                    "file.txt": "hello"
                }
            }),
        )
        .await;
        let project = Project::test(fs, [Path::new("/project")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let connection = CwdCapturingConnection::new();
        let captured_cwd = connection.captured_cwd.clone();

        let mut session = AgentSessionInfo::new(SessionId::new("session-1"));
        session.cwd = Some(PathBuf::from("/project/subdir"));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let _thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(StubAgentServer::new(connection)),
                    Some(session),
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        assert_eq!(
            captured_cwd.lock().as_deref(),
            Some(Path::new("/project/subdir")),
            "Should use session cwd when it's inside the project"
        );
    }

    #[gpui::test]
    async fn test_resume_thread_uses_fallback_cwd_when_outside_project(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "file.txt": "hello"
            }),
        )
        .await;
        let project = Project::test(fs, [Path::new("/project")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let connection = CwdCapturingConnection::new();
        let captured_cwd = connection.captured_cwd.clone();

        let mut session = AgentSessionInfo::new(SessionId::new("session-1"));
        session.cwd = Some(PathBuf::from("/some/other/path"));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let _thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(StubAgentServer::new(connection)),
                    Some(session),
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        assert_eq!(
            captured_cwd.lock().as_deref(),
            Some(Path::new("/project")),
            "Should use fallback project cwd when session cwd is outside the project"
        );
    }

    #[gpui::test]
    async fn test_resume_thread_rejects_unnormalized_cwd_outside_project(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "file.txt": "hello"
            }),
        )
        .await;
        let project = Project::test(fs, [Path::new("/project")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let connection = CwdCapturingConnection::new();
        let captured_cwd = connection.captured_cwd.clone();

        let mut session = AgentSessionInfo::new(SessionId::new("session-1"));
        session.cwd = Some(PathBuf::from("/project/../outside"));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let _thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(StubAgentServer::new(connection)),
                    Some(session),
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        assert_eq!(
            captured_cwd.lock().as_deref(),
            Some(Path::new("/project")),
            "Should reject unnormalized cwd that resolves outside the project and use fallback cwd"
        );
    }

    #[gpui::test]
    async fn test_refusal_handling(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(RefusalAgentConnection), cx).await;

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Do something harmful", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Check that the refusal error is set
        thread_view.read_with(cx, |thread_view, cx| {
            let state = thread_view.active_thread().unwrap();
            assert!(
                matches!(state.read(cx).thread_error, Some(ThreadError::Refusal)),
                "Expected refusal error to be set"
            );
        });
    }

    #[gpui::test]
    async fn test_notification_for_tool_authorization(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("1");
        let tool_call = acp::ToolCall::new(tool_call_id.clone(), "Label")
            .kind(acp::ToolKind::Edit)
            .content(vec!["hi".into()]);
        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id,
                PermissionOptions::Flat(vec![acp::PermissionOption::new(
                    "1",
                    "Allow",
                    acp::PermissionOptionKind::AllowOnce,
                )]),
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some())
        );
    }

    #[gpui::test]
    async fn test_notification_when_panel_hidden(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;

        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);

        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        // Window is active (don't deactivate), but panel will be hidden
        // Note: In the test environment, the panel is not actually added to the dock,
        // so is_agent_panel_hidden will return true

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Should show notification because window is active but panel is hidden
        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some()),
            "Expected notification when panel is hidden"
        );
    }

    #[gpui::test]
    async fn test_notification_still_works_when_window_inactive(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        // Deactivate window - should show notification regardless of setting
        cx.deactivate_window();

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Should still show notification when window is inactive (existing behavior)
        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some()),
            "Expected notification when window is inactive"
        );
    }

    #[gpui::test]
    async fn test_notification_respects_never_setting(cx: &mut TestAppContext) {
        init_test(cx);

        // Set notify_when_agent_waiting to Never
        cx.update(|cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        // Window is active

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Should NOT show notification because notify_when_agent_waiting is Never
        assert!(
            !cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some()),
            "Expected no notification when notify_when_agent_waiting is Never"
        );
    }

    #[gpui::test]
    async fn test_notification_closed_when_thread_view_dropped(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;

        let weak_view = thread_view.downgrade();

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify notification is shown
        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some()),
            "Expected notification to be shown"
        );

        // Drop the thread view (simulating navigation to a new thread)
        drop(thread_view);
        drop(message_editor);
        // Trigger an update to flush effects, which will call release_dropped_entities
        cx.update(|_window, _cx| {});
        cx.run_until_parked();

        // Verify the entity was actually released
        assert!(
            !weak_view.is_upgradable(),
            "Thread view entity should be released after dropping"
        );

        // The notification should be automatically closed via on_release
        assert!(
            !cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some()),
            "Notification should be closed when thread view is dropped"
        );
    }

    async fn setup_thread_view(
        agent: impl AgentServer + 'static,
        cx: &mut TestAppContext,
    ) -> (Entity<AcpServerView>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(agent),
                    None,
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history,
                    window,
                    cx,
                )
            })
        });
        cx.run_until_parked();
        (thread_view, cx)
    }

    fn add_to_workspace(thread_view: Entity<AcpServerView>, cx: &mut VisualTestContext) {
        let workspace = thread_view.read_with(cx, |thread_view, _cx| thread_view.workspace.clone());

        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|_| ThreadViewItem(thread_view.clone()))),
                    None,
                    true,
                    window,
                    cx,
                );
            })
            .unwrap();
    }

    struct ThreadViewItem(Entity<AcpServerView>);

    impl Item for ThreadViewItem {
        type Event = ();

        fn include_in_nav_history() -> bool {
            false
        }

        fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
            "Test".into()
        }
    }

    impl EventEmitter<()> for ThreadViewItem {}

    impl Focusable for ThreadViewItem {
        fn focus_handle(&self, cx: &App) -> FocusHandle {
            self.0.read(cx).focus_handle(cx)
        }
    }

    impl Render for ThreadViewItem {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.0.clone().into_any_element()
        }
    }

    struct StubAgentServer<C> {
        connection: C,
    }

    impl<C> StubAgentServer<C> {
        fn new(connection: C) -> Self {
            Self { connection }
        }
    }

    impl StubAgentServer<StubAgentConnection> {
        fn default_response() -> Self {
            let conn = StubAgentConnection::new();
            conn.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
                acp::ContentChunk::new("Default response".into()),
            )]);
            Self::new(conn)
        }
    }

    impl<C> AgentServer for StubAgentServer<C>
    where
        C: 'static + AgentConnection + Send + Clone,
    {
        fn logo(&self) -> ui::IconName {
            ui::IconName::Ai
        }

        fn name(&self) -> SharedString {
            "Test".into()
        }

        fn connect(
            &self,
            _root_dir: Option<&Path>,
            _delegate: AgentServerDelegate,
            _cx: &mut App,
        ) -> Task<gpui::Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
            Task::ready(Ok((Rc::new(self.connection.clone()), None)))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct StubSessionList {
        sessions: Vec<AgentSessionInfo>,
    }

    impl StubSessionList {
        fn new(sessions: Vec<AgentSessionInfo>) -> Self {
            Self { sessions }
        }
    }

    impl AgentSessionList for StubSessionList {
        fn list_sessions(
            &self,
            _request: AgentSessionListRequest,
            _cx: &mut App,
        ) -> Task<anyhow::Result<AgentSessionListResponse>> {
            Task::ready(Ok(AgentSessionListResponse::new(self.sessions.clone())))
        }
        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct ResumeOnlyAgentConnection;

    impl AgentConnection for ResumeOnlyAgentConnection {
        fn telemetry_id(&self) -> SharedString {
            "resume-only".into()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    None,
                    "ResumeOnlyAgentConnection",
                    self.clone(),
                    project,
                    action_log,
                    SessionId::new("new-session"),
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            });
            Task::ready(Ok(thread))
        }

        fn supports_resume_session(&self, _cx: &App) -> bool {
            true
        }

        fn resume_session(
            self: Rc<Self>,
            session: AgentSessionInfo,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    None,
                    "ResumeOnlyAgentConnection",
                    self.clone(),
                    project,
                    action_log,
                    session.session_id,
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            });
            Task::ready(Ok(thread))
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn authenticate(
            &self,
            _method_id: acp::AuthMethodId,
            _cx: &mut App,
        ) -> Task<gpui::Result<()>> {
            Task::ready(Ok(()))
        }

        fn prompt(
            &self,
            _id: Option<acp_thread::UserMessageId>,
            _params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            Task::ready(Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {}

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct SaboteurAgentConnection;

    impl AgentConnection for SaboteurAgentConnection {
        fn telemetry_id(&self) -> SharedString {
            "saboteur".into()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            Task::ready(Ok(cx.new(|cx| {
                let action_log = cx.new(|_| ActionLog::new(project.clone()));
                AcpThread::new(
                    None,
                    "SaboteurAgentConnection",
                    self,
                    project,
                    action_log,
                    SessionId::new("test"),
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            })))
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
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
            _id: Option<acp_thread::UserMessageId>,
            _params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            Task::ready(Err(anyhow::anyhow!("Error prompting")))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {
            unimplemented!()
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    /// Simulates a model which always returns a refusal response
    #[derive(Clone)]
    struct RefusalAgentConnection;

    impl AgentConnection for RefusalAgentConnection {
        fn telemetry_id(&self) -> SharedString {
            "refusal".into()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            Task::ready(Ok(cx.new(|cx| {
                let action_log = cx.new(|_| ActionLog::new(project.clone()));
                AcpThread::new(
                    None,
                    "RefusalAgentConnection",
                    self,
                    project,
                    action_log,
                    SessionId::new("test"),
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            })))
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
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
            _id: Option<acp_thread::UserMessageId>,
            _params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            Task::ready(Ok(acp::PromptResponse::new(acp::StopReason::Refusal)))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {
            unimplemented!()
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct CwdCapturingConnection {
        captured_cwd: Arc<Mutex<Option<PathBuf>>>,
    }

    impl CwdCapturingConnection {
        fn new() -> Self {
            Self {
                captured_cwd: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl AgentConnection for CwdCapturingConnection {
        fn telemetry_id(&self) -> SharedString {
            "cwd-capturing".into()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            *self.captured_cwd.lock() = Some(cwd.to_path_buf());
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    None,
                    "CwdCapturingConnection",
                    self.clone(),
                    project,
                    action_log,
                    SessionId::new("new-session"),
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            });
            Task::ready(Ok(thread))
        }

        fn supports_load_session(&self, _cx: &App) -> bool {
            true
        }

        fn load_session(
            self: Rc<Self>,
            session: AgentSessionInfo,
            project: Entity<Project>,
            cwd: &Path,
            cx: &mut App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            *self.captured_cwd.lock() = Some(cwd.to_path_buf());
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
                    None,
                    "CwdCapturingConnection",
                    self.clone(),
                    project,
                    action_log,
                    session.session_id,
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            });
            Task::ready(Ok(thread))
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn authenticate(
            &self,
            _method_id: acp::AuthMethodId,
            _cx: &mut App,
        ) -> Task<gpui::Result<()>> {
            Task::ready(Ok(()))
        }

        fn prompt(
            &self,
            _id: Option<acp_thread::UserMessageId>,
            _params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            Task::ready(Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {}

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            prompt_store::init(cx)
        });
    }

    fn active_thread(
        thread_view: &Entity<AcpServerView>,
        cx: &TestAppContext,
    ) -> Entity<AcpThreadView> {
        cx.read(|cx| {
            thread_view
                .read(cx)
                .active_thread()
                .expect("No active thread")
                .clone()
        })
    }

    fn message_editor(
        thread_view: &Entity<AcpServerView>,
        cx: &TestAppContext,
    ) -> Entity<MessageEditor> {
        let thread = active_thread(thread_view, cx);
        cx.read(|cx| thread.read(cx).message_editor.clone())
    }

    #[gpui::test]
    async fn test_rewind_views(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "test1.txt": "old content 1",
                "test2.txt": "old content 2"
            }),
        )
        .await;
        let project = Project::test(fs, [Path::new("/project")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let connection = Rc::new(StubAgentConnection::new());
        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpServerView::new(
                    Rc::new(StubAgentServer::new(connection.as_ref().clone())),
                    None,
                    None,
                    workspace.downgrade(),
                    project.clone(),
                    Some(thread_store.clone()),
                    None,
                    history,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        let thread = thread_view
            .read_with(cx, |view, cx| {
                view.active_thread().map(|r| r.read(cx).thread.clone())
            })
            .unwrap();

        // First user message
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(
            acp::ToolCall::new("tool1", "Edit file 1")
                .kind(acp::ToolKind::Edit)
                .status(acp::ToolCallStatus::Completed)
                .content(vec![acp::ToolCallContent::Diff(
                    acp::Diff::new("/project/test1.txt", "new content 1").old_text("old content 1"),
                )]),
        )]);

        thread
            .update(cx, |thread, cx| thread.send_raw("Give me a diff", cx))
            .await
            .unwrap();
        cx.run_until_parked();

        thread.read_with(cx, |thread, _cx| {
            assert_eq!(thread.entries().len(), 2);
        });

        thread_view.read_with(cx, |view, cx| {
            let entry_view_state = view
                .active_thread()
                .map(|active| active.read(cx).entry_view_state.clone())
                .unwrap();
            entry_view_state.read_with(cx, |entry_view_state, _| {
                assert!(
                    entry_view_state
                        .entry(0)
                        .unwrap()
                        .message_editor()
                        .is_some()
                );
                assert!(entry_view_state.entry(1).unwrap().has_content());
            });
        });

        // Second user message
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(
            acp::ToolCall::new("tool2", "Edit file 2")
                .kind(acp::ToolKind::Edit)
                .status(acp::ToolCallStatus::Completed)
                .content(vec![acp::ToolCallContent::Diff(
                    acp::Diff::new("/project/test2.txt", "new content 2").old_text("old content 2"),
                )]),
        )]);

        thread
            .update(cx, |thread, cx| thread.send_raw("Another one", cx))
            .await
            .unwrap();
        cx.run_until_parked();

        let second_user_message_id = thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries().len(), 4);
            let AgentThreadEntry::UserMessage(user_message) = &thread.entries()[2] else {
                panic!();
            };
            user_message.id.clone().unwrap()
        });

        thread_view.read_with(cx, |view, cx| {
            let entry_view_state = view
                .active_thread()
                .unwrap()
                .read(cx)
                .entry_view_state
                .clone();
            entry_view_state.read_with(cx, |entry_view_state, _| {
                assert!(
                    entry_view_state
                        .entry(0)
                        .unwrap()
                        .message_editor()
                        .is_some()
                );
                assert!(entry_view_state.entry(1).unwrap().has_content());
                assert!(
                    entry_view_state
                        .entry(2)
                        .unwrap()
                        .message_editor()
                        .is_some()
                );
                assert!(entry_view_state.entry(3).unwrap().has_content());
            });
        });

        // Rewind to first message
        thread
            .update(cx, |thread, cx| thread.rewind(second_user_message_id, cx))
            .await
            .unwrap();

        cx.run_until_parked();

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries().len(), 2);
        });

        thread_view.read_with(cx, |view, cx| {
            let active = view.active_thread().unwrap();
            active
                .read(cx)
                .entry_view_state
                .read_with(cx, |entry_view_state, _| {
                    assert!(
                        entry_view_state
                            .entry(0)
                            .unwrap()
                            .message_editor()
                            .is_some()
                    );
                    assert!(entry_view_state.entry(1).unwrap().has_content());

                    // Old views should be dropped
                    assert!(entry_view_state.entry(2).is_none());
                    assert!(entry_view_state.entry(3).is_none());
                });
        });
    }

    #[gpui::test]
    async fn test_scroll_to_most_recent_user_prompt(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        // Each user prompt will result in a user message entry plus an agent message entry.
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response 1".into()),
        )]);

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(connection.clone()), cx).await;

        let thread = thread_view
            .read_with(cx, |view, cx| {
                view.active_thread().map(|r| r.read(cx).thread.clone())
            })
            .unwrap();

        thread
            .update(cx, |thread, cx| thread.send_raw("Prompt 1", cx))
            .await
            .unwrap();
        cx.run_until_parked();

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response 2".into()),
        )]);

        thread
            .update(cx, |thread, cx| thread.send_raw("Prompt 2", cx))
            .await
            .unwrap();
        cx.run_until_parked();

        // Move somewhere else first so we're not trivially already on the last user prompt.
        active_thread(&thread_view, cx).update(cx, |view, cx| {
            view.scroll_to_top(cx);
        });
        cx.run_until_parked();

        active_thread(&thread_view, cx).update(cx, |view, cx| {
            view.scroll_to_most_recent_user_prompt(cx);
            let scroll_top = view.list_state.logical_scroll_top();
            // Entries layout is: [User1, Assistant1, User2, Assistant2]
            assert_eq!(scroll_top.item_ix, 2);
        });
    }

    #[gpui::test]
    async fn test_scroll_to_most_recent_user_prompt_falls_back_to_bottom_without_user_messages(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default_response(), cx).await;

        // With no entries, scrolling should be a no-op and must not panic.
        active_thread(&thread_view, cx).update(cx, |view, cx| {
            view.scroll_to_most_recent_user_prompt(cx);
            let scroll_top = view.list_state.logical_scroll_top();
            assert_eq!(scroll_top.item_ix, 0);
        });
    }

    #[gpui::test]
    async fn test_message_editing_cancel(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response".into()),
        )]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );

            view.active_thread()
                .map(|active| &active.read(cx).entry_view_state)
                .as_ref()
                .unwrap()
                .read(cx)
                .entry(0)
                .unwrap()
                .message_editor()
                .unwrap()
                .clone()
        });

        // Focus
        cx.focus(&user_message_editor);
        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
        });

        // Edit
        user_message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edited message content", window, cx);
        });

        // Cancel
        user_message_editor.update_in(cx, |_editor, window, cx| {
            window.dispatch_action(Box::new(editor::actions::Cancel), cx);
        });

        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );
        });

        user_message_editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Original message to edit");
        });
    }

    #[gpui::test]
    async fn test_message_doesnt_send_if_empty(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("", window, cx);
        });

        let thread = cx.read(|cx| {
            thread_view
                .read(cx)
                .active_thread()
                .unwrap()
                .read(cx)
                .thread
                .clone()
        });
        let entries_before = cx.read(|cx| thread.read(cx).entries().len());

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| {
            view.send(window, cx);
        });
        cx.run_until_parked();

        let entries_after = cx.read(|cx| thread.read(cx).entries().len());
        assert_eq!(
            entries_before, entries_after,
            "No message should be sent when editor is empty"
        );
    }

    #[gpui::test]
    async fn test_message_editing_regenerate(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response".into()),
        )]);

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(connection.clone()), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );
            assert_eq!(
                view.active_thread()
                    .unwrap()
                    .read(cx)
                    .thread
                    .read(cx)
                    .entries()
                    .len(),
                2
            );

            view.active_thread()
                .map(|active| &active.read(cx).entry_view_state)
                .as_ref()
                .unwrap()
                .read(cx)
                .entry(0)
                .unwrap()
                .message_editor()
                .unwrap()
                .clone()
        });

        // Focus
        cx.focus(&user_message_editor);

        // Edit
        user_message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edited message content", window, cx);
        });

        // Send
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("New Response".into()),
        )]);

        user_message_editor.update_in(cx, |_editor, window, cx| {
            window.dispatch_action(Box::new(Chat), cx);
        });

        cx.run_until_parked();

        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );

            let entries = view
                .active_thread()
                .unwrap()
                .read(cx)
                .thread
                .read(cx)
                .entries();
            assert_eq!(entries.len(), 2);
            assert_eq!(
                entries[0].to_markdown(cx),
                "## User\n\nEdited message content\n\n"
            );
            assert_eq!(
                entries[1].to_markdown(cx),
                "## Assistant\n\nNew Response\n\n"
            );

            let entry_view_state = view
                .active_thread()
                .map(|active| &active.read(cx).entry_view_state)
                .unwrap();
            let new_editor = entry_view_state.read_with(cx, |state, _cx| {
                assert!(!state.entry(1).unwrap().has_content());
                state.entry(0).unwrap().message_editor().unwrap().clone()
            });

            assert_eq!(new_editor.read(cx).text(cx), "Edited message content");
        })
    }

    #[gpui::test]
    async fn test_message_editing_while_generating(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(connection.clone()), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        let (user_message_editor, session_id) = thread_view.read_with(cx, |view, cx| {
            let thread = view.active_thread().unwrap().read(cx).thread.read(cx);
            assert_eq!(thread.entries().len(), 1);

            let editor = view
                .active_thread()
                .map(|active| &active.read(cx).entry_view_state)
                .as_ref()
                .unwrap()
                .read(cx)
                .entry(0)
                .unwrap()
                .message_editor()
                .unwrap()
                .clone();

            (editor, thread.session_id().clone())
        });

        // Focus
        cx.focus(&user_message_editor);

        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
        });

        // Edit
        user_message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edited message content", window, cx);
        });

        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
        });

        // Finish streaming response
        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("Response".into())),
                cx,
            );
            connection.end_turn(session_id, acp::StopReason::EndTurn);
        });

        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
        });

        cx.run_until_parked();

        // Should still be editing
        cx.update(|window, cx| {
            assert!(user_message_editor.focus_handle(cx).is_focused(window));
            assert_eq!(
                thread_view
                    .read(cx)
                    .active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
            assert_eq!(
                user_message_editor.read(cx).text(cx),
                "Edited message content"
            );
        });
    }

    struct GeneratingThreadSetup {
        thread_view: Entity<AcpServerView>,
        thread: Entity<AcpThread>,
        message_editor: Entity<MessageEditor>,
    }

    async fn setup_generating_thread(
        cx: &mut TestAppContext,
    ) -> (GeneratingThreadSetup, &mut VisualTestContext) {
        let connection = StubAgentConnection::new();

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(connection.clone()), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        let (thread, session_id) = thread_view.read_with(cx, |view, cx| {
            let thread = view
                .active_thread()
                .as_ref()
                .unwrap()
                .read(cx)
                .thread
                .clone();
            (thread.clone(), thread.read(cx).session_id().clone())
        });

        cx.run_until_parked();

        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                    "Response chunk".into(),
                )),
                cx,
            );
        });

        cx.run_until_parked();

        thread.read_with(cx, |thread, _cx| {
            assert_eq!(thread.status(), ThreadStatus::Generating);
        });

        (
            GeneratingThreadSetup {
                thread_view,
                thread,
                message_editor,
            },
            cx,
        )
    }

    #[gpui::test]
    async fn test_escape_cancels_generation_from_conversation_focus(cx: &mut TestAppContext) {
        init_test(cx);

        let (setup, cx) = setup_generating_thread(cx).await;

        let focus_handle = setup
            .thread_view
            .read_with(cx, |view, cx| view.focus_handle(cx));
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
        });

        setup.thread_view.update_in(cx, |_, window, cx| {
            window.dispatch_action(menu::Cancel.boxed_clone(), cx);
        });

        cx.run_until_parked();

        setup.thread.read_with(cx, |thread, _cx| {
            assert_eq!(thread.status(), ThreadStatus::Idle);
        });
    }

    #[gpui::test]
    async fn test_escape_cancels_generation_from_editor_focus(cx: &mut TestAppContext) {
        init_test(cx);

        let (setup, cx) = setup_generating_thread(cx).await;

        let editor_focus_handle = setup
            .message_editor
            .read_with(cx, |editor, cx| editor.focus_handle(cx));
        cx.update(|window, cx| {
            window.focus(&editor_focus_handle, cx);
        });

        setup.message_editor.update_in(cx, |_, window, cx| {
            window.dispatch_action(editor::actions::Cancel.boxed_clone(), cx);
        });

        cx.run_until_parked();

        setup.thread.read_with(cx, |thread, _cx| {
            assert_eq!(thread.status(), ThreadStatus::Idle);
        });
    }

    #[gpui::test]
    async fn test_escape_when_idle_is_noop(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(StubAgentConnection::new()), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let thread = thread_view.read_with(cx, |view, cx| {
            view.active_thread().unwrap().read(cx).thread.clone()
        });

        thread.read_with(cx, |thread, _cx| {
            assert_eq!(thread.status(), ThreadStatus::Idle);
        });

        let focus_handle = thread_view.read_with(cx, |view, _cx| view.focus_handle.clone());
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
        });

        thread_view.update_in(cx, |_, window, cx| {
            window.dispatch_action(menu::Cancel.boxed_clone(), cx);
        });

        cx.run_until_parked();

        thread.read_with(cx, |thread, _cx| {
            assert_eq!(thread.status(), ThreadStatus::Idle);
        });
    }

    #[gpui::test]
    async fn test_interrupt(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(connection.clone()), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Message 1", window, cx);
        });
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        let (thread, session_id) = thread_view.read_with(cx, |view, cx| {
            let thread = view.active_thread().unwrap().read(cx).thread.clone();

            (thread.clone(), thread.read(cx).session_id().clone())
        });

        cx.run_until_parked();

        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                    "Message 1 resp".into(),
                )),
                cx,
            );
        });

        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc::indoc! {"
                        ## User

                        Message 1

                        ## Assistant

                        Message 1 resp

                    "}
            )
        });

        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Message 2", window, cx);
        });
        active_thread(&thread_view, cx)
            .update_in(cx, |view, window, cx| view.interrupt_and_send(window, cx));

        cx.update(|_, cx| {
            // Simulate a response sent after beginning to cancel
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("onse".into())),
                cx,
            );
        });

        cx.run_until_parked();

        // Last Message 1 response should appear before Message 2
        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc::indoc! {"
                        ## User

                        Message 1

                        ## Assistant

                        Message 1 response

                        ## User

                        Message 2

                    "}
            )
        });

        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                    "Message 2 response".into(),
                )),
                cx,
            );
            connection.end_turn(session_id.clone(), acp::StopReason::EndTurn);
        });

        cx.run_until_parked();

        thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc::indoc! {"
                        ## User

                        Message 1

                        ## Assistant

                        Message 1 response

                        ## User

                        Message 2

                        ## Assistant

                        Message 2 response

                    "}
            )
        });
    }

    #[gpui::test]
    async fn test_message_editing_insert_selections(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response".into()),
        )]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx)
        });
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));
        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |thread_view, cx| {
            thread_view
                .active_thread()
                .map(|active| &active.read(cx).entry_view_state)
                .as_ref()
                .unwrap()
                .read(cx)
                .entry(0)
                .expect("Should have at least one entry")
                .message_editor()
                .expect("Should have message editor")
                .clone()
        });

        cx.focus(&user_message_editor);
        thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
        });

        // Ensure to edit the focused message before proceeding otherwise, since
        // its content is not different from what was sent, focus will be lost.
        user_message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit with ", window, cx)
        });

        // Create a simple buffer with some text so we can create a selection
        // that will then be added to the message being edited.
        let (workspace, project) = thread_view.read_with(cx, |thread_view, _cx| {
            (thread_view.workspace.clone(), thread_view.project.clone())
        });
        let buffer = project.update(cx, |project, cx| {
            project.create_local_buffer("let a = 10 + 10;", None, false, cx)
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                let editor = cx.new(|cx| {
                    let mut editor =
                        Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx);

                    editor.change_selections(Default::default(), window, cx, |selections| {
                        selections.select_ranges([MultiBufferOffset(8)..MultiBufferOffset(15)]);
                    });

                    editor
                });
                workspace.add_item_to_active_pane(Box::new(editor), None, false, window, cx);
            })
            .unwrap();

        thread_view.update_in(cx, |view, window, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                Some(0)
            );
            view.insert_selections(window, cx);
        });

        user_message_editor.read_with(cx, |editor, cx| {
            let text = editor.editor().read(cx).text(cx);
            let expected_text = String::from("Original message to edit with selection ");

            assert_eq!(text, expected_text);
        });
    }

    #[gpui::test]
    async fn test_insert_selections(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response".into()),
        )]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Can you review this snippet ", window, cx)
        });

        // Create a simple buffer with some text so we can create a selection
        // that will then be added to the message being edited.
        let (workspace, project) = thread_view.read_with(cx, |thread_view, _cx| {
            (thread_view.workspace.clone(), thread_view.project.clone())
        });
        let buffer = project.update(cx, |project, cx| {
            project.create_local_buffer("let a = 10 + 10;", None, false, cx)
        });

        workspace
            .update_in(cx, |workspace, window, cx| {
                let editor = cx.new(|cx| {
                    let mut editor =
                        Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx);

                    editor.change_selections(Default::default(), window, cx, |selections| {
                        selections.select_ranges([MultiBufferOffset(8)..MultiBufferOffset(15)]);
                    });

                    editor
                });
                workspace.add_item_to_active_pane(Box::new(editor), None, false, window, cx);
            })
            .unwrap();

        thread_view.update_in(cx, |view, window, cx| {
            assert_eq!(
                view.active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );
            view.insert_selections(window, cx);
        });

        message_editor.read_with(cx, |editor, cx| {
            let text = editor.text(cx);
            let expected_txt = String::from("Can you review this snippet selection ");

            assert_eq!(text, expected_txt);
        })
    }

    #[gpui::test]
    async fn test_tool_permission_buttons_terminal_with_pattern(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("terminal-1");
        let tool_call = acp::ToolCall::new(tool_call_id.clone(), "Run `cargo build --release`")
            .kind(acp::ToolKind::Edit);

        let permission_options = ToolPermissionContext::new(
            TerminalTool::NAME,
            vec!["cargo build --release".to_string()],
        )
        .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options,
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;

        // Disable notifications to avoid popup windows
        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Run cargo build", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify the tool call is in WaitingForConfirmation state with the expected options
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);

            let tool_call = thread.entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(call) = entry {
                    Some(call)
                } else {
                    None
                }
            });

            assert!(tool_call.is_some(), "Expected a tool call entry");
            let tool_call = tool_call.unwrap();

            // Verify it's waiting for confirmation
            assert!(
                matches!(
                    tool_call.status,
                    acp_thread::ToolCallStatus::WaitingForConfirmation { .. }
                ),
                "Expected WaitingForConfirmation status, got {:?}",
                tool_call.status
            );

            // Verify the options count (granularity options only, no separate Deny option)
            if let acp_thread::ToolCallStatus::WaitingForConfirmation { options, .. } =
                &tool_call.status
            {
                let PermissionOptions::Dropdown(choices) = options else {
                    panic!("Expected dropdown permission options");
                };

                assert_eq!(
                    choices.len(),
                    3,
                    "Expected 3 permission options (granularity only)"
                );

                // Verify specific button labels (now using neutral names)
                let labels: Vec<&str> = choices
                    .iter()
                    .map(|choice| choice.allow.name.as_ref())
                    .collect();
                assert!(
                    labels.contains(&"Always for terminal"),
                    "Missing 'Always for terminal' option"
                );
                assert!(
                    labels.contains(&"Always for `cargo` commands"),
                    "Missing pattern option"
                );
                assert!(
                    labels.contains(&"Only this time"),
                    "Missing 'Only this time' option"
                );
            }
        });
    }

    #[gpui::test]
    async fn test_tool_permission_buttons_edit_file_with_path_pattern(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("edit-file-1");
        let tool_call = acp::ToolCall::new(tool_call_id.clone(), "Edit `src/main.rs`")
            .kind(acp::ToolKind::Edit);

        let permission_options =
            ToolPermissionContext::new(EditFileTool::NAME, vec!["src/main.rs".to_string()])
                .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options,
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;

        // Disable notifications
        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edit the main file", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify the options
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);

            let tool_call = thread.entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(call) = entry {
                    Some(call)
                } else {
                    None
                }
            });

            assert!(tool_call.is_some(), "Expected a tool call entry");
            let tool_call = tool_call.unwrap();

            if let acp_thread::ToolCallStatus::WaitingForConfirmation { options, .. } =
                &tool_call.status
            {
                let PermissionOptions::Dropdown(choices) = options else {
                    panic!("Expected dropdown permission options");
                };

                let labels: Vec<&str> = choices
                    .iter()
                    .map(|choice| choice.allow.name.as_ref())
                    .collect();
                assert!(
                    labels.contains(&"Always for edit file"),
                    "Missing 'Always for edit file' option"
                );
                assert!(
                    labels.contains(&"Always for `src/`"),
                    "Missing path pattern option"
                );
            } else {
                panic!("Expected WaitingForConfirmation status");
            }
        });
    }

    #[gpui::test]
    async fn test_tool_permission_buttons_fetch_with_domain_pattern(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("fetch-1");
        let tool_call = acp::ToolCall::new(tool_call_id.clone(), "Fetch `https://docs.rs/gpui`")
            .kind(acp::ToolKind::Fetch);

        let permission_options =
            ToolPermissionContext::new(FetchTool::NAME, vec!["https://docs.rs/gpui".to_string()])
                .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options,
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;

        // Disable notifications
        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Fetch the docs", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify the options
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);

            let tool_call = thread.entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(call) = entry {
                    Some(call)
                } else {
                    None
                }
            });

            assert!(tool_call.is_some(), "Expected a tool call entry");
            let tool_call = tool_call.unwrap();

            if let acp_thread::ToolCallStatus::WaitingForConfirmation { options, .. } =
                &tool_call.status
            {
                let PermissionOptions::Dropdown(choices) = options else {
                    panic!("Expected dropdown permission options");
                };

                let labels: Vec<&str> = choices
                    .iter()
                    .map(|choice| choice.allow.name.as_ref())
                    .collect();
                assert!(
                    labels.contains(&"Always for fetch"),
                    "Missing 'Always for fetch' option"
                );
                assert!(
                    labels.contains(&"Always for `docs.rs`"),
                    "Missing domain pattern option"
                );
            } else {
                panic!("Expected WaitingForConfirmation status");
            }
        });
    }

    #[gpui::test]
    async fn test_tool_permission_buttons_without_pattern(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("terminal-no-pattern-1");
        let tool_call = acp::ToolCall::new(tool_call_id.clone(), "Run `./deploy.sh --production`")
            .kind(acp::ToolKind::Edit);

        // No pattern button since ./deploy.sh doesn't match the alphanumeric pattern
        let permission_options = ToolPermissionContext::new(
            TerminalTool::NAME,
            vec!["./deploy.sh --production".to_string()],
        )
        .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options,
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;

        // Disable notifications
        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Run the deploy script", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify only 2 options (no pattern button when command doesn't match pattern)
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);

            let tool_call = thread.entries().iter().find_map(|entry| {
                if let acp_thread::AgentThreadEntry::ToolCall(call) = entry {
                    Some(call)
                } else {
                    None
                }
            });

            assert!(tool_call.is_some(), "Expected a tool call entry");
            let tool_call = tool_call.unwrap();

            if let acp_thread::ToolCallStatus::WaitingForConfirmation { options, .. } =
                &tool_call.status
            {
                let PermissionOptions::Dropdown(choices) = options else {
                    panic!("Expected dropdown permission options");
                };

                assert_eq!(
                    choices.len(),
                    2,
                    "Expected 2 permission options (no pattern option)"
                );

                let labels: Vec<&str> = choices
                    .iter()
                    .map(|choice| choice.allow.name.as_ref())
                    .collect();
                assert!(
                    labels.contains(&"Always for terminal"),
                    "Missing 'Always for terminal' option"
                );
                assert!(
                    labels.contains(&"Only this time"),
                    "Missing 'Only this time' option"
                );
                // Should NOT contain a pattern option
                assert!(
                    !labels.iter().any(|l| l.contains("commands")),
                    "Should not have pattern option"
                );
            } else {
                panic!("Expected WaitingForConfirmation status");
            }
        });
    }

    #[gpui::test]
    async fn test_authorize_tool_call_action_triggers_authorization(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("action-test-1");
        let tool_call =
            acp::ToolCall::new(tool_call_id.clone(), "Run `cargo test`").kind(acp::ToolKind::Edit);

        let permission_options =
            ToolPermissionContext::new(TerminalTool::NAME, vec!["cargo test".to_string()])
                .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options,
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Run tests", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify tool call is waiting for confirmation
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);
            let tool_call = thread.first_tool_awaiting_confirmation();
            assert!(
                tool_call.is_some(),
                "Expected a tool call waiting for confirmation"
            );
        });

        // Dispatch the AuthorizeToolCall action (simulating dropdown menu selection)
        thread_view.update_in(cx, |_, window, cx| {
            window.dispatch_action(
                crate::AuthorizeToolCall {
                    tool_call_id: "action-test-1".to_string(),
                    option_id: "allow".to_string(),
                    option_kind: "AllowOnce".to_string(),
                }
                .boxed_clone(),
                cx,
            );
        });

        cx.run_until_parked();

        // Verify tool call is no longer waiting for confirmation (was authorized)
        thread_view.read_with(cx, |thread_view, cx| {
                let thread = thread_view.active_thread().expect("Thread should exist").read(cx).thread.clone();
                let thread = thread.read(cx);
                let tool_call = thread.first_tool_awaiting_confirmation();
                assert!(
                    tool_call.is_none(),
                    "Tool call should no longer be waiting for confirmation after AuthorizeToolCall action"
                );
            });
    }

    #[gpui::test]
    async fn test_authorize_tool_call_action_with_pattern_option(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("pattern-action-test-1");
        let tool_call =
            acp::ToolCall::new(tool_call_id.clone(), "Run `npm install`").kind(acp::ToolKind::Edit);

        let permission_options =
            ToolPermissionContext::new(TerminalTool::NAME, vec!["npm install".to_string()])
                .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options.clone(),
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Install dependencies", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Find the pattern option ID
        let pattern_option = match &permission_options {
            PermissionOptions::Dropdown(choices) => choices
                .iter()
                .find(|choice| {
                    choice
                        .allow
                        .option_id
                        .0
                        .starts_with("always_allow_pattern:")
                })
                .map(|choice| &choice.allow)
                .expect("Should have a pattern option for npm command"),
            _ => panic!("Expected dropdown permission options"),
        };

        // Dispatch action with the pattern option (simulating "Always allow `npm` commands")
        thread_view.update_in(cx, |_, window, cx| {
            window.dispatch_action(
                crate::AuthorizeToolCall {
                    tool_call_id: "pattern-action-test-1".to_string(),
                    option_id: pattern_option.option_id.0.to_string(),
                    option_kind: "AllowAlways".to_string(),
                }
                .boxed_clone(),
                cx,
            );
        });

        cx.run_until_parked();

        // Verify tool call was authorized
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);
            let tool_call = thread.first_tool_awaiting_confirmation();
            assert!(
                tool_call.is_none(),
                "Tool call should be authorized after selecting pattern option"
            );
        });
    }

    #[gpui::test]
    async fn test_granularity_selection_updates_state(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("granularity-test-1");
        let tool_call =
            acp::ToolCall::new(tool_call_id.clone(), "Run `cargo build`").kind(acp::ToolKind::Edit);

        let permission_options =
            ToolPermissionContext::new(TerminalTool::NAME, vec!["cargo build".to_string()])
                .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options.clone(),
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Build the project", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Verify default granularity is the last option (index 2 = "Only this time")
        thread_view.read_with(cx, |thread_view, cx| {
            let state = thread_view.active_thread().unwrap();
            let selected = state
                .read(cx)
                .selected_permission_granularity
                .get(&tool_call_id);
            assert!(
                selected.is_none(),
                "Should have no selection initially (defaults to last)"
            );
        });

        // Select the first option (index 0 = "Always for terminal")
        thread_view.update_in(cx, |_, window, cx| {
            window.dispatch_action(
                crate::SelectPermissionGranularity {
                    tool_call_id: "granularity-test-1".to_string(),
                    index: 0,
                }
                .boxed_clone(),
                cx,
            );
        });

        cx.run_until_parked();

        // Verify the selection was updated
        thread_view.read_with(cx, |thread_view, cx| {
            let state = thread_view.active_thread().unwrap();
            let selected = state
                .read(cx)
                .selected_permission_granularity
                .get(&tool_call_id);
            assert_eq!(selected, Some(&0), "Should have selected index 0");
        });
    }

    #[gpui::test]
    async fn test_allow_button_uses_selected_granularity(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("allow-granularity-test-1");
        let tool_call =
            acp::ToolCall::new(tool_call_id.clone(), "Run `npm install`").kind(acp::ToolKind::Edit);

        let permission_options =
            ToolPermissionContext::new(TerminalTool::NAME, vec!["npm install".to_string()])
                .build_permission_options();

        // Verify we have the expected options
        let PermissionOptions::Dropdown(choices) = &permission_options else {
            panic!("Expected dropdown permission options");
        };

        assert_eq!(choices.len(), 3);
        assert!(
            choices[0]
                .allow
                .option_id
                .0
                .contains("always_allow:terminal")
        );
        assert!(
            choices[1]
                .allow
                .option_id
                .0
                .contains("always_allow_pattern:terminal")
        );
        assert_eq!(choices[2].allow.option_id.0.as_ref(), "allow");

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options.clone(),
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Install dependencies", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Select the pattern option (index 1 = "Always for `npm` commands")
        thread_view.update_in(cx, |_, window, cx| {
            window.dispatch_action(
                crate::SelectPermissionGranularity {
                    tool_call_id: "allow-granularity-test-1".to_string(),
                    index: 1,
                }
                .boxed_clone(),
                cx,
            );
        });

        cx.run_until_parked();

        // Simulate clicking the Allow button by dispatching AllowOnce action
        // which should use the selected granularity
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| {
            view.allow_once(&AllowOnce, window, cx)
        });

        cx.run_until_parked();

        // Verify tool call was authorized
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);
            let tool_call = thread.first_tool_awaiting_confirmation();
            assert!(
                tool_call.is_none(),
                "Tool call should be authorized after Allow with pattern granularity"
            );
        });
    }

    #[gpui::test]
    async fn test_deny_button_uses_selected_granularity(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId::new("deny-granularity-test-1");
        let tool_call =
            acp::ToolCall::new(tool_call_id.clone(), "Run `git push`").kind(acp::ToolKind::Edit);

        let permission_options =
            ToolPermissionContext::new(TerminalTool::NAME, vec!["git push".to_string()])
                .build_permission_options();

        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id.clone(),
                permission_options.clone(),
            )]));

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(tool_call)]);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;
        add_to_workspace(thread_view.clone(), cx);

        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let message_editor = message_editor(&thread_view, cx);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Push changes", window, cx);
        });

        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| view.send(window, cx));

        cx.run_until_parked();

        // Use default granularity (last option = "Only this time")
        // Simulate clicking the Deny button
        active_thread(&thread_view, cx).update_in(cx, |view, window, cx| {
            view.reject_once(&RejectOnce, window, cx)
        });

        cx.run_until_parked();

        // Verify tool call was rejected (no longer waiting for confirmation)
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .active_thread()
                .expect("Thread should exist")
                .read(cx)
                .thread
                .clone();
            let thread = thread.read(cx);
            let tool_call = thread.first_tool_awaiting_confirmation();
            assert!(
                tool_call.is_none(),
                "Tool call should be rejected after Deny"
            );
        });
    }

    #[gpui::test]
    async fn test_option_id_transformation_for_allow() {
        let permission_options = ToolPermissionContext::new(
            TerminalTool::NAME,
            vec!["cargo build --release".to_string()],
        )
        .build_permission_options();

        let PermissionOptions::Dropdown(choices) = permission_options else {
            panic!("Expected dropdown permission options");
        };

        let allow_ids: Vec<String> = choices
            .iter()
            .map(|choice| choice.allow.option_id.0.to_string())
            .collect();

        assert!(allow_ids.contains(&"always_allow:terminal".to_string()));
        assert!(allow_ids.contains(&"allow".to_string()));
        assert!(
            allow_ids
                .iter()
                .any(|id| id.starts_with("always_allow_pattern:terminal\n")),
            "Missing allow pattern option"
        );
    }

    #[gpui::test]
    async fn test_option_id_transformation_for_deny() {
        let permission_options = ToolPermissionContext::new(
            TerminalTool::NAME,
            vec!["cargo build --release".to_string()],
        )
        .build_permission_options();

        let PermissionOptions::Dropdown(choices) = permission_options else {
            panic!("Expected dropdown permission options");
        };

        let deny_ids: Vec<String> = choices
            .iter()
            .map(|choice| choice.deny.option_id.0.to_string())
            .collect();

        assert!(deny_ids.contains(&"always_deny:terminal".to_string()));
        assert!(deny_ids.contains(&"deny".to_string()));
        assert!(
            deny_ids
                .iter()
                .any(|id| id.starts_with("always_deny_pattern:terminal\n")),
            "Missing deny pattern option"
        );
    }
}
