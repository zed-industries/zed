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
    AgentSharingFeatureFlag, AgentV2FeatureFlag, CloudThinkingToggleFeatureFlag,
    FeatureFlagAppExt as _, UserSlashCommandsFeatureFlag,
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
    DiffStat, Disclosure, Divider, DividerColor, IconButtonShape, IconDecoration,
    IconDecorationKind, KeyBinding, PopoverMenu, PopoverMenuHandle, SpinnerLabel, TintColor,
    Tooltip, WithScrollbar, prelude::*, right_click_menu,
};
use util::defer;
use util::{ResultExt, size::format_file_size, time::duration_alt_display};
use workspace::{
    CollaboratorId, NewTerminal, OpenOptions, Toast, Workspace, notifications::NotificationId,
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
use crate::user_slash_command::{
    self, CommandLoadError, SlashCommandRegistry, SlashCommandRegistryEvent, UserSlashCommand,
};
use crate::{
    AgentDiffPane, AgentPanel, AllowAlways, AllowOnce, AuthorizeToolCall, ClearMessageQueue,
    CycleFavoriteModels, CycleModeSelector, EditFirstQueuedMessage, ExpandMessageEditor,
    ExternalAgentInitialContent, Follow, KeepAll, NewThread, OpenAddContextMenu, OpenAgentDiff,
    OpenHistory, RejectAll, RejectOnce, RemoveFirstQueuedMessage, SelectPermissionGranularity,
    SendImmediately, SendNextQueuedMessage, ToggleProfileSelector, ToggleThinkingMode,
    text_thread_history,
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
enum ThreadError {
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

#[derive(Default)]
struct ThreadFeedbackState {
    feedback: Option<ThreadFeedback>,
    comments_editor: Option<Entity<Editor>>,
}

impl ThreadFeedbackState {
    pub fn submit(
        &mut self,
        thread: Entity<AcpThread>,
        feedback: ThreadFeedback,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(telemetry) = thread.read(cx).connection().telemetry() else {
            return;
        };

        if self.feedback == Some(feedback) {
            return;
        }

        self.feedback = Some(feedback);
        match feedback {
            ThreadFeedback::Positive => {
                self.comments_editor = None;
            }
            ThreadFeedback::Negative => {
                self.comments_editor = Some(Self::build_feedback_comments_editor(window, cx));
            }
        }
        let session_id = thread.read(cx).session_id().clone();
        let agent_telemetry_id = thread.read(cx).connection().telemetry_id();
        let task = telemetry.thread_data(&session_id, cx);
        let rating = match feedback {
            ThreadFeedback::Positive => "positive",
            ThreadFeedback::Negative => "negative",
        };
        cx.background_spawn(async move {
            let thread = task.await?;
            telemetry::event!(
                "Agent Thread Rated",
                agent = agent_telemetry_id,
                session_id = session_id,
                rating = rating,
                thread = thread
            );
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn submit_comments(&mut self, thread: Entity<AcpThread>, cx: &mut App) {
        let Some(telemetry) = thread.read(cx).connection().telemetry() else {
            return;
        };

        let Some(comments) = self
            .comments_editor
            .as_ref()
            .map(|editor| editor.read(cx).text(cx))
            .filter(|text| !text.trim().is_empty())
        else {
            return;
        };

        self.comments_editor.take();

        let session_id = thread.read(cx).session_id().clone();
        let agent_telemetry_id = thread.read(cx).connection().telemetry_id();
        let task = telemetry.thread_data(&session_id, cx);
        cx.background_spawn(async move {
            let thread = task.await?;
            telemetry::event!(
                "Agent Thread Feedback Comments",
                agent = agent_telemetry_id,
                session_id = session_id,
                comments = comments,
                thread = thread
            );
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn clear(&mut self) {
        *self = Self::default()
    }

    pub fn dismiss_comments(&mut self) {
        self.comments_editor.take();
    }

    fn build_feedback_comments_editor(window: &mut Window, cx: &mut App) -> Entity<Editor> {
        let buffer = cx.new(|cx| {
            let empty_string = String::new();
            MultiBuffer::singleton(cx.new(|cx| Buffer::local(empty_string, cx)), cx)
        });

        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(4),
                },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_placeholder_text(
                "What went wrong? Share your feedback so we can improve.",
                window,
                cx,
            );
            editor
        });

        editor.read(cx).focus_handle(cx).focus(window, cx);
        editor
    }
}

#[derive(Default, Clone, Copy)]
struct DiffStats {
    lines_added: u32,
    lines_removed: u32,
}

impl DiffStats {
    fn single_file(buffer: &Buffer, diff: &BufferDiff, cx: &App) -> Self {
        let mut stats = DiffStats::default();
        let diff_snapshot = diff.snapshot(cx);
        let buffer_snapshot = buffer.snapshot();
        let base_text = diff_snapshot.base_text();

        for hunk in diff_snapshot.hunks(&buffer_snapshot) {
            let added_rows = hunk.range.end.row.saturating_sub(hunk.range.start.row);
            stats.lines_added += added_rows;

            let base_start = hunk.diff_base_byte_range.start.to_point(base_text).row;
            let base_end = hunk.diff_base_byte_range.end.to_point(base_text).row;
            let removed_rows = base_end.saturating_sub(base_start);
            stats.lines_removed += removed_rows;
        }

        stats
    }

    fn all_files(changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>, cx: &App) -> Self {
        let mut total = DiffStats::default();
        for (buffer, diff) in changed_buffers {
            let stats = DiffStats::single_file(buffer.read(cx), diff.read(cx), cx);
            total.lines_added += stats.lines_added;
            total.lines_removed += stats.lines_removed;
        }
        total
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
    recent_history_entries: Vec<AgentSessionInfo>,
    history: Entity<AcpThreadHistory>,
    _history_subscription: Subscription,
    hovered_recent_history_item: Option<usize>,
    focus_handle: FocusHandle,
    notifications: Vec<WindowHandle<AgentNotification>>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    slash_command_registry: Option<Entity<SlashCommandRegistry>>,
    auth_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
    show_codex_windows_warning: bool,
}

impl AcpServerView {
    pub fn as_active_thread(&self) -> Option<Entity<AcpThreadView>> {
        match &self.server_state {
            ServerState::Connected(connected) => Some(connected.current.clone()),
            _ => None,
        }
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
    current: Entity<AcpThreadView>,
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
    pub fn has_thread_error(&self, cx: &App) -> bool {
        self.current.read(cx).thread_error.is_some()
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
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        let available_commands = Rc::new(RefCell::new(vec![]));
        let cached_user_commands = Rc::new(RefCell::new(collections::HashMap::default()));
        let cached_user_command_errors = Rc::new(RefCell::new(Vec::new()));

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
            for window in this.notifications.drain(..) {
                window
                    .update(cx, |_, window, _| {
                        window.remove_window();
                    })
                    .ok();
            }
        })
        .detach();

        let show_codex_windows_warning = cfg!(windows)
            && project.read(cx).is_local()
            && agent.clone().downcast::<agent_servers::Codex>().is_some();

        // Create SlashCommandRegistry to cache user-defined slash commands and watch for changes
        let slash_command_registry = if cx.has_flag::<UserSlashCommandsFeatureFlag>() {
            let fs = project.read(cx).fs().clone();
            let worktree_roots: Vec<std::path::PathBuf> = project
                .read(cx)
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                .collect();
            let registry = cx.new(|cx| SlashCommandRegistry::new(fs, worktree_roots, cx));

            // Subscribe to registry changes to update error display and cached commands
            cx.subscribe(&registry, move |this, registry, event, cx| match event {
                SlashCommandRegistryEvent::CommandsChanged => {
                    this.refresh_cached_user_commands_from_registry(&registry, cx);
                }
            })
            .detach();

            // Initialize cached commands and errors from registry
            let mut commands = registry.read(cx).commands().clone();
            let mut errors = registry.read(cx).errors().to_vec();
            let server_command_names = available_commands
                .borrow()
                .iter()
                .map(|command| command.name.clone())
                .collect::<HashSet<_>>();
            user_slash_command::apply_server_command_conflicts_to_map(
                &mut commands,
                &mut errors,
                &server_command_names,
            );
            *cached_user_commands.borrow_mut() = commands;
            *cached_user_command_errors.borrow_mut() = errors;

            Some(registry)
        } else {
            None
        };

        let recent_history_entries = history.read(cx).get_recent_sessions(3);
        let history_subscription = cx.observe(&history, |this, history, cx| {
            this.update_recent_history_from_cache(&history, cx);
        });

        Self {
            agent: agent.clone(),
            agent_server_store,
            workspace: workspace.clone(),
            project: project.clone(),
            thread_store,
            prompt_store,
            server_state: Self::initial_state(
                agent.clone(),
                resume_thread,
                workspace.clone(),
                project.clone(),
                prompt_capabilities,
                available_commands,
                cached_user_commands,
                cached_user_command_errors,
                initial_content,
                window,
                cx,
            ),
            login: None,
            notifications: Vec::new(),
            notification_subscriptions: HashMap::default(),
            slash_command_registry,
            auth_task: None,
            recent_history_entries,
            history,
            _history_subscription: history_subscription,
            hovered_recent_history_item: None,
            _subscriptions: subscriptions,
            focus_handle: cx.focus_handle(),
            show_codex_windows_warning,
        }
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        let available_commands = Rc::new(RefCell::new(vec![]));
        let cached_user_commands = Rc::new(RefCell::new(collections::HashMap::default()));
        let cached_user_command_errors = Rc::new(RefCell::new(Vec::new()));

        let resume_thread_metadata = self
            .as_active_thread()
            .and_then(|thread| thread.read(cx).resume_thread_metadata.clone());

        self.server_state = Self::initial_state(
            self.agent.clone(),
            resume_thread_metadata,
            self.workspace.clone(),
            self.project.clone(),
            prompt_capabilities.clone(),
            available_commands.clone(),
            cached_user_commands.clone(),
            cached_user_command_errors.clone(),
            None,
            window,
            cx,
        );

        match &mut self.server_state {
            ServerState::Connected(state) => {
                state.current.update(cx, |this, cx| {
                    this.message_editor.update(cx, |editor, cx| {
                        editor.set_command_state(
                            prompt_capabilities,
                            available_commands,
                            cached_user_commands,
                            cached_user_command_errors,
                            cx,
                        );
                    });
                });
            }
            _ => {}
        }

        self.refresh_cached_user_commands(cx);
        self.recent_history_entries.clear();
        cx.notify();
    }

    fn initial_state(
        agent: Rc<dyn AgentServer>,
        resume_thread: Option<AgentSessionInfo>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        prompt_capabilities: Rc<RefCell<PromptCapabilities>>,
        available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
        cached_user_commands: Rc<RefCell<HashMap<String, UserSlashCommand>>>,
        cached_user_command_errors: Rc<RefCell<Vec<CommandLoadError>>>,
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
        let root_dir = worktrees
            .into_iter()
            .filter_map(|worktree| {
                if worktree.read(cx).is_single_file() {
                    Some(worktree.read(cx).abs_path().parent()?.into())
                } else {
                    Some(worktree.read(cx).abs_path())
                }
            })
            .next();
        let fallback_cwd = root_dir
            .clone()
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
                        } else {
                            this.handle_thread_error(err, cx);
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
                    let session_cwd = resume
                        .cwd
                        .clone()
                        .unwrap_or_else(|| fallback_cwd.as_ref().to_path_buf());
                    if connection.supports_load_session(cx) {
                        connection.clone().load_session(
                            resume,
                            project.clone(),
                            session_cwd.as_path(),
                            cx,
                        )
                    } else if connection.supports_resume_session(cx) {
                        resumed_without_history = true;
                        connection.clone().resume_session(
                            resume,
                            project.clone(),
                            session_cwd.as_path(),
                            cx,
                        )
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
                        .new_thread(project.clone(), fallback_cwd.as_ref(), cx)
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
                            Self::handle_auth_required(this, err, agent.name(), window, cx)
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
                        let action_log = thread.read(cx).action_log().clone();

                        prompt_capabilities.replace(thread.read(cx).prompt_capabilities());

                        let entry_view_state = cx.new(|_| {
                            EntryViewState::new(
                                this.workspace.clone(),
                                this.project.downgrade(),
                                this.thread_store.clone(),
                                this.history.downgrade(),
                                this.prompt_store.clone(),
                                prompt_capabilities.clone(),
                                available_commands.clone(),
                                cached_user_commands.clone(),
                                cached_user_command_errors.clone(),
                                this.agent.name(),
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

                        AgentDiff::set_active_thread(&workspace, thread.clone(), window, cx);

                        let connection = thread.read(cx).connection().clone();
                        let session_id = thread.read(cx).session_id().clone();
                        let session_list = if connection.supports_session_history(cx) {
                            connection.session_list(cx)
                        } else {
                            None
                        };
                        this.history.update(cx, |history, cx| {
                            history.set_session_list(session_list, cx);
                        });

                        // Check for config options first
                        // Config options take precedence over legacy mode/model selectors
                        // (feature flag gating happens at the data layer)
                        let config_options_provider =
                            connection.session_config_options(&session_id, cx);

                        let config_options_view;
                        let mode_selector;
                        let model_selector;
                        if let Some(config_options) = config_options_provider {
                            // Use config options - don't create mode_selector or model_selector
                            let agent_server = this.agent.clone();
                            let fs = this.project.read(cx).fs().clone();
                            config_options_view = Some(cx.new(|cx| {
                                ConfigOptionsView::new(config_options, agent_server, fs, window, cx)
                            }));
                            model_selector = None;
                            mode_selector = None;
                        } else {
                            // Fall back to legacy mode/model selectors
                            config_options_view = None;
                            model_selector =
                                connection.model_selector(&session_id).map(|selector| {
                                    let agent_server = this.agent.clone();
                                    let fs = this.project.read(cx).fs().clone();
                                    cx.new(|cx| {
                                        AcpModelSelectorPopover::new(
                                            selector,
                                            agent_server,
                                            fs,
                                            PopoverMenuHandle::default(),
                                            this.focus_handle(cx),
                                            window,
                                            cx,
                                        )
                                    })
                                });

                            mode_selector =
                                connection
                                    .session_modes(&session_id, cx)
                                    .map(|session_modes| {
                                        let fs = this.project.read(cx).fs().clone();
                                        let focus_handle = this.focus_handle(cx);
                                        cx.new(|_cx| {
                                            ModeSelector::new(
                                                session_modes,
                                                this.agent.clone(),
                                                fs,
                                                focus_handle,
                                            )
                                        })
                                    });
                        }

                        let mut subscriptions = vec![
                            cx.subscribe_in(&thread, window, Self::handle_thread_event),
                            cx.observe(&action_log, |_, _, cx| cx.notify()),
                            // cx.subscribe_in(
                            //     &entry_view_state,
                            //     window,
                            //     Self::handle_entry_view_event,
                            // ),
                        ];

                        let title_editor =
                            if thread.update(cx, |thread, cx| thread.can_set_title(cx)) {
                                let editor = cx.new(|cx| {
                                    let mut editor = Editor::single_line(window, cx);
                                    editor.set_text(thread.read(cx).title(), window, cx);
                                    editor
                                });
                                subscriptions.push(cx.subscribe_in(
                                    &editor,
                                    window,
                                    Self::handle_title_editor_event,
                                ));
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
                                        this.focus_handle(cx),
                                        cx,
                                    )
                                })
                            });

                        let agent_display_name = this
                            .agent_server_store
                            .read(cx)
                            .agent_display_name(&ExternalAgentServerName(agent.name()))
                            .unwrap_or_else(|| agent.name());

                        let current = cx.new(|cx| {
                            AcpThreadView::new(
                                thread,
                                agent.name(),
                                agent_display_name,
                                workspace.clone(),
                                entry_view_state,
                                title_editor,
                                config_options_view,
                                mode_selector,
                                model_selector,
                                profile_selector,
                                list_state,
                                prompt_capabilities,
                                available_commands,
                                cached_user_commands,
                                cached_user_command_errors,
                                resumed_without_history,
                                resume_thread.clone(),
                                project.downgrade(),
                                this.thread_store.clone(),
                                this.history.downgrade(),
                                this.prompt_store.clone(),
                                initial_content,
                                subscriptions,
                                window,
                                cx,
                            )
                        });

                        if this.focus_handle.contains_focused(window, cx) {
                            current
                                .read(cx)
                                .message_editor
                                .focus_handle(cx)
                                .focus(window, cx);
                        }

                        this.server_state = ServerState::Connected(ConnectedServerState {
                            connection,
                            auth_state: AuthState::Ok,
                            current,
                        });

                        cx.notify();
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
                        if let Some(thread) = this.as_active_thread() {
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

    fn handle_auth_required(
        this: WeakEntity<Self>,
        err: AuthRequired,
        agent_name: SharedString,
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
            if let Some(connected) = this.as_connected_mut() {
                let description = err
                    .description
                    .map(|desc| cx.new(|cx| Markdown::new(desc.into(), None, None, cx)));

                connected.auth_state = AuthState::Unauthenticated {
                    pending_auth_method: None,
                    configuration_view,
                    description,
                    _subscription: subscription,
                };
                if connected
                    .current
                    .read(cx)
                    .message_editor
                    .focus_handle(cx)
                    .is_focused(window)
                {
                    this.focus_handle.focus(window, cx)
                }
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
        match &self.server_state {
            ServerState::Connected(connected) => {
                if connected
                    .current
                    .read(cx)
                    .message_editor
                    .focus_handle(cx)
                    .is_focused(window)
                {
                    self.focus_handle.focus(window, cx)
                }
            }
            _ => {}
        }
        if let Some(load_err) = err.downcast_ref::<LoadError>() {
            self.server_state = ServerState::LoadError(load_err.clone());
        } else {
            self.server_state =
                ServerState::LoadError(LoadError::Other(format!("{:#}", err).into()))
        }
        cx.notify();
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
            if let Some(active) = self.as_active_thread() {
                active.update(cx, |active, _cx| {
                    active.thread_error = None;
                    active.thread_error_markdown = None;
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
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.cancel_generation(cx);
            });
        }
    }

    fn sync_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_imported_thread(cx) {
            return;
        }

        let Some(active) = self.as_active_thread() else {
            return;
        };

        let project = self.project.clone();
        let this = cx.entity();

        active.update(cx, |active, cx| {
            active.sync_thread(project, this, window, cx)
        });
    }

    pub fn handle_title_editor_event(
        &mut self,
        title_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.handle_title_editor_event(title_editor, event, window, cx);
            });
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.server_state, ServerState::Loading { .. })
    }

    fn update_turn_tokens(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
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
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.send_queued_message_at_index(index, is_send_now, window, cx);
            });
        }
    }

    fn open_edited_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.open_edited_buffer(buffer, window, cx);
            });
        };
    }

    fn handle_thread_event(
        &mut self,
        thread: &Entity<AcpThread>,
        event: &AcpThreadEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            AcpThreadEvent::NewEntry => {
                let len = thread.read(cx).entries().len();
                let index = len - 1;
                if let Some(active) = self.as_active_thread() {
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
                    .as_active_thread()
                    .map(|active| active.read(cx).entry_view_state.clone())
                {
                    entry_view_state.update(cx, |view_state, cx| {
                        view_state.sync_entry(*index, thread, window, cx)
                    });
                }
            }
            AcpThreadEvent::EntriesRemoved(range) => {
                if let Some(active) = self.as_active_thread() {
                    let entry_view_state = active.read(cx).entry_view_state.clone();
                    let list_state = active.read(cx).list_state.clone();
                    entry_view_state.update(cx, |view_state, _cx| view_state.remove(range.clone()));
                    list_state.splice(range.clone(), 0);
                }
            }
            AcpThreadEvent::ToolAuthorizationRequired => {
                self.notify_with_sound("Waiting for tool confirmation", IconName::Info, window, cx);
            }
            AcpThreadEvent::Retry(retry) => {
                if let Some(active) = self.as_active_thread() {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status = Some(retry.clone());
                    });
                }
            }
            AcpThreadEvent::Stopped => {
                if let Some(active) = self.as_active_thread() {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status.take();
                    });
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

                let should_send_queued = if let Some(active) = self.as_active_thread() {
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
                self.emit_thread_error_telemetry(&error, cx);

                if let Some(active) = self.as_active_thread() {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status.take();
                        active.thread_error = Some(error);
                    });
                }
                let model_or_agent_name = self.current_model_name(cx);
                let notification_message =
                    format!("{} refused to respond to this request", model_or_agent_name);
                self.notify_with_sound(&notification_message, IconName::Warning, window, cx);
            }
            AcpThreadEvent::Error => {
                if let Some(active) = self.as_active_thread() {
                    active.update(cx, |active, _cx| {
                        active.thread_retry_status.take();
                    });
                }
                self.notify_with_sound(
                    "Agent stopped due to an error",
                    IconName::Warning,
                    window,
                    cx,
                );
            }
            AcpThreadEvent::LoadError(error) => {
                match &self.server_state {
                    ServerState::Connected(connected) => {
                        if connected
                            .current
                            .read(cx)
                            .message_editor
                            .focus_handle(cx)
                            .is_focused(window)
                        {
                            self.focus_handle.focus(window, cx)
                        }
                    }
                    _ => {}
                }
                self.server_state = ServerState::LoadError(error.clone());
            }
            AcpThreadEvent::TitleUpdated => {
                let title = thread.read(cx).title();
                if let Some(title_editor) = self
                    .as_active_thread()
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
                if let Some(active) = self.as_active_thread() {
                    active.update(cx, |active, _cx| {
                        active
                            .prompt_capabilities
                            .replace(thread.read(_cx).prompt_capabilities());
                    });
                }
            }
            AcpThreadEvent::TokenUsageUpdated => {
                self.update_turn_tokens(cx);
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
                if let Some(active) = self.as_active_thread() {
                    active.update(cx, |active, _cx| {
                        active.available_commands.replace(available_commands);
                    });
                }
                self.refresh_cached_user_commands(cx);

                let agent_display_name = self
                    .agent_server_store
                    .read(cx)
                    .agent_display_name(&ExternalAgentServerName(self.agent.name()))
                    .unwrap_or_else(|| self.agent.name());

                if let Some(active) = self.as_active_thread() {
                    let new_placeholder =
                        placeholder_text(agent_display_name.as_ref(), has_commands);
                    active.update(cx, |active, _cx| {
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
                                    this.handle_thread_error(err, cx);
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
                        this.handle_thread_error(err, cx);
                    } else {
                        this.reset(window, cx);
                    }
                    this.auth_task.take()
                })
                .ok();
            }
        }));
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
        self.as_active_thread().is_some_and(|active| {
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

    fn authorize_tool_call(
        &mut self,
        tool_call_id: acp::ToolCallId,
        option_id: acp::PermissionOptionId,
        option_kind: acp::PermissionOptionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.authorize_tool_call(tool_call_id, option_id, option_kind, window, cx);
            });
        };
    }

    fn restore_checkpoint(&mut self, message_id: &UserMessageId, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.restore_checkpoint(message_id, cx);
            });
        };
    }

    fn render_message_context_menu(
        &self,
        entry_ix: usize,
        message_body: AnyElement,
        cx: &Context<Self>,
    ) -> AnyElement {
        let entity = cx.entity();
        let workspace = self.workspace.clone();

        right_click_menu(format!("agent_context_menu-{}", entry_ix))
            .trigger(move |_, _, _| message_body)
            .menu(move |window, cx| {
                let focus = window.focused(cx);
                let entity = entity.clone();
                let workspace = workspace.clone();

                ContextMenu::build(window, cx, move |menu, _, cx| {
                    let active_thread = entity.read(cx).as_active_thread();
                    let is_at_top = active_thread
                        .as_ref()
                        .map(|active| active.read(cx).list_state.logical_scroll_top().item_ix == 0)
                        .unwrap_or(true);

                    let has_selection = active_thread
                        .as_ref()
                        .and_then(|active| active.read(cx).thread.read(cx).entries().get(entry_ix))
                        .and_then(|entry| match &entry {
                            AgentThreadEntry::AssistantMessage(msg) => Some(&msg.chunks),
                            _ => None,
                        })
                        .map(|chunks| {
                            chunks.iter().any(|chunk| {
                                let md = match chunk {
                                    AssistantMessageChunk::Message { block } => block.markdown(),
                                    AssistantMessageChunk::Thought { block } => block.markdown(),
                                };
                                md.map_or(false, |m| m.read(cx).selected_text().is_some())
                            })
                        })
                        .unwrap_or(false);

                    let copy_this_agent_response =
                        ContextMenuEntry::new("Copy This Agent Response").handler({
                            let entity = entity.clone();
                            move |_, cx| {
                                entity.update(cx, |this, cx| {
                                    if let Some(active) = this.as_active_thread() {
                                        let entries = active.read(cx).thread.read(cx).entries();
                                        if let Some(text) =
                                            Self::get_agent_message_content(entries, entry_ix, cx)
                                        {
                                            cx.write_to_clipboard(ClipboardItem::new_string(text));
                                        }
                                    }
                                });
                            }
                        });

                    let scroll_item = if is_at_top {
                        ContextMenuEntry::new("Scroll to Bottom").handler({
                            let entity = entity.clone();
                            move |_, cx| {
                                entity.update(cx, |this, cx| {
                                    this.scroll_to_bottom(cx);
                                });
                            }
                        })
                    } else {
                        ContextMenuEntry::new("Scroll to Top").handler({
                            let entity = entity.clone();
                            move |_, cx| {
                                entity.update(cx, |this, cx| {
                                    this.scroll_to_top(cx);
                                });
                            }
                        })
                    };

                    let open_thread_as_markdown = ContextMenuEntry::new("Open Thread as Markdown")
                        .handler({
                            let entity = entity.clone();
                            let workspace = workspace.clone();
                            move |window, cx| {
                                if let Some(workspace) = workspace.upgrade() {
                                    entity
                                        .update(cx, |this, cx| {
                                            this.open_thread_as_markdown(workspace, window, cx)
                                        })
                                        .detach_and_log_err(cx);
                                }
                            }
                        });

                    menu.when_some(focus, |menu, focus| menu.context(focus))
                        .action_disabled_when(
                            !has_selection,
                            "Copy Selection",
                            Box::new(markdown::CopyAsMarkdown),
                        )
                        .item(copy_this_agent_response)
                        .separator()
                        .item(scroll_item)
                        .item(open_thread_as_markdown)
                })
            })
            .into_any_element()
    }

    fn tool_card_header_bg(&self, cx: &Context<Self>) -> Hsla {
        cx.theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025))
    }

    fn tool_card_border_color(&self, cx: &Context<Self>) -> Hsla {
        cx.theme().colors().border.opacity(0.8)
    }

    fn tool_name_font_size(&self) -> Rems {
        rems_from_px(13.)
    }

    fn render_thinking_block(
        &self,
        entry_ix: usize,
        chunk_ix: usize,
        chunk: Entity<Markdown>,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let header_id = SharedString::from(format!("thinking-block-header-{}", entry_ix));
        let card_header_id = SharedString::from("inner-card-header");

        let key = (entry_ix, chunk_ix);

        let is_open = self
            .as_active_thread()
            .is_some_and(|active| active.read(cx).expanded_thinking_blocks.contains(&key));

        let scroll_handle = self.as_active_thread().and_then(|active| {
            active
                .read(cx)
                .entry_view_state
                .read(cx)
                .entry(entry_ix)
                .and_then(|entry| entry.scroll_handle_for_assistant_message_chunk(chunk_ix))
        });

        let thinking_content = {
            div()
                .id(("thinking-content", chunk_ix))
                .when_some(scroll_handle, |this, scroll_handle| {
                    this.track_scroll(&scroll_handle)
                })
                .text_ui_sm(cx)
                .overflow_hidden()
                .child(self.render_markdown(
                    chunk,
                    MarkdownStyle::themed(MarkdownFont::Agent, window, cx),
                ))
        };

        v_flex()
            .gap_1()
            .child(
                h_flex()
                    .id(header_id)
                    .group(&card_header_id)
                    .relative()
                    .w_full()
                    .pr_1()
                    .justify_between()
                    .child(
                        h_flex()
                            .h(window.line_height() - px(2.))
                            .gap_1p5()
                            .overflow_hidden()
                            .child(
                                Icon::new(IconName::ToolThink)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .text_size(self.tool_name_font_size())
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Thinking"),
                            ),
                    )
                    .child(
                        Disclosure::new(("expand", entry_ix), is_open)
                            .opened_icon(IconName::ChevronUp)
                            .closed_icon(IconName::ChevronDown)
                            .visible_on_hover(&card_header_id)
                            .on_click(cx.listener({
                                move |this, _event, _window, cx| {
                                    if let Some(active) = this.as_active_thread() {
                                        active.update(cx, |active, _cx| {
                                            if is_open {
                                                active.expanded_thinking_blocks.remove(&key);
                                            } else {
                                                active.expanded_thinking_blocks.insert(key);
                                            }
                                        });
                                        cx.notify();
                                    }
                                }
                            })),
                    )
                    .on_click(cx.listener({
                        move |this, _event, _window, cx| {
                            if let Some(active) = this.as_active_thread() {
                                active.update(cx, |active, _cx| {
                                    if is_open {
                                        active.expanded_thinking_blocks.remove(&key);
                                    } else {
                                        active.expanded_thinking_blocks.insert(key);
                                    }
                                });
                                cx.notify();
                            }
                        }
                    })),
            )
            .when(is_open, |this| {
                this.child(
                    div()
                        .ml_1p5()
                        .pl_3p5()
                        .border_l_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(thinking_content),
                )
            })
            .into_any_element()
    }

    fn render_tool_call(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let has_location = tool_call.locations.len() == 1;
        let card_header_id = SharedString::from("inner-tool-call-header");

        let failed_or_canceled = match &tool_call.status {
            ToolCallStatus::Rejected | ToolCallStatus::Canceled | ToolCallStatus::Failed => true,
            _ => false,
        };

        let needs_confirmation = matches!(
            tool_call.status,
            ToolCallStatus::WaitingForConfirmation { .. }
        );
        let is_terminal_tool = matches!(tool_call.kind, acp::ToolKind::Execute);

        let is_edit =
            matches!(tool_call.kind, acp::ToolKind::Edit) || tool_call.diffs().next().is_some();
        let is_subagent = tool_call.is_subagent();

        // For subagent tool calls, render the subagent cards directly without wrapper
        if is_subagent {
            return self.render_subagent_tool_call(entry_ix, tool_call, window, cx);
        }

        let is_cancelled_edit = is_edit && matches!(tool_call.status, ToolCallStatus::Canceled);
        let has_revealed_diff = tool_call.diffs().next().is_some_and(|diff| {
            self.as_active_thread()
                .and_then(|active| {
                    active
                        .read(cx)
                        .entry_view_state
                        .read(cx)
                        .entry(entry_ix)
                        .and_then(|entry| entry.editor_for_diff(diff))
                })
                .is_some()
                && diff.read(cx).has_revealed_range(cx)
        });

        let use_card_layout = needs_confirmation || is_edit || is_terminal_tool;

        let has_image_content = tool_call.content.iter().any(|c| c.image().is_some());
        let is_collapsible = !tool_call.content.is_empty() && !needs_confirmation;
        let mut is_open = self
            .as_active_thread()
            .is_some_and(|active| active.read(cx).expanded_tool_calls.contains(&tool_call.id));

        is_open |= needs_confirmation;

        let should_show_raw_input = !is_terminal_tool && !is_edit && !has_image_content;

        let input_output_header = |label: SharedString| {
            Label::new(label)
                .size(LabelSize::XSmall)
                .color(Color::Muted)
                .buffer_font(cx)
        };

        let tool_output_display =
            if is_open {
                match &tool_call.status {
                    ToolCallStatus::WaitingForConfirmation { options, .. } => v_flex()
                        .w_full()
                        .children(tool_call.content.iter().enumerate().map(
                            |(content_ix, content)| {
                                div()
                                    .child(self.render_tool_call_content(
                                        entry_ix,
                                        content,
                                        content_ix,
                                        tool_call,
                                        use_card_layout,
                                        has_image_content,
                                        failed_or_canceled,
                                        window,
                                        cx,
                                    ))
                                    .into_any_element()
                            },
                        ))
                        .when(should_show_raw_input, |this| {
                            let is_raw_input_expanded =
                                self.as_active_thread().is_some_and(|active| {
                                    active
                                        .read(cx)
                                        .expanded_tool_call_raw_inputs
                                        .contains(&tool_call.id)
                                });

                            let input_header = if is_raw_input_expanded {
                                "Raw Input:"
                            } else {
                                "View Raw Input"
                            };

                            this.child(
                                v_flex()
                                    .p_2()
                                    .gap_1()
                                    .border_t_1()
                                    .border_color(self.tool_card_border_color(cx))
                                    .child(
                                        h_flex()
                                            .id("disclosure_container")
                                            .pl_0p5()
                                            .gap_1()
                                            .justify_between()
                                            .rounded_xs()
                                            .hover(|s| s.bg(cx.theme().colors().element_hover))
                                            .child(input_output_header(input_header.into()))
                                            .child(
                                                Disclosure::new(
                                                    ("raw-input-disclosure", entry_ix),
                                                    is_raw_input_expanded,
                                                )
                                                .opened_icon(IconName::ChevronUp)
                                                .closed_icon(IconName::ChevronDown),
                                            )
                                            .on_click(cx.listener({
                                                let id = tool_call.id.clone();

                                                move |this: &mut Self, _, _, cx| {
                                                    if let Some(active) = this.as_active_thread() {
                                                        active.update(cx, |active, _cx| {
                                                            if active
                                                                .expanded_tool_call_raw_inputs
                                                                .contains(&id)
                                                            {
                                                                active
                                                                    .expanded_tool_call_raw_inputs
                                                                    .remove(&id);
                                                            } else {
                                                                active
                                                                    .expanded_tool_call_raw_inputs
                                                                    .insert(id.clone());
                                                            }
                                                        });
                                                        cx.notify();
                                                    }
                                                }
                                            })),
                                    )
                                    .when(is_raw_input_expanded, |this| {
                                        this.children(tool_call.raw_input_markdown.clone().map(
                                            |input| {
                                                self.render_markdown(
                                                    input,
                                                    MarkdownStyle::themed(
                                                        MarkdownFont::Agent,
                                                        window,
                                                        cx,
                                                    ),
                                                )
                                            },
                                        ))
                                    }),
                            )
                        })
                        .child(self.render_permission_buttons(
                            options,
                            entry_ix,
                            tool_call.id.clone(),
                            cx,
                        ))
                        .into_any(),
                    ToolCallStatus::Pending | ToolCallStatus::InProgress
                        if is_edit
                            && tool_call.content.is_empty()
                            && self.as_native_connection(cx).is_some() =>
                    {
                        self.render_diff_loading(cx).into_any()
                    }
                    ToolCallStatus::Pending
                    | ToolCallStatus::InProgress
                    | ToolCallStatus::Completed
                    | ToolCallStatus::Failed
                    | ToolCallStatus::Canceled => v_flex()
                        .when(should_show_raw_input, |this| {
                            this.mt_1p5().w_full().child(
                                v_flex()
                                    .ml(rems(0.4))
                                    .px_3p5()
                                    .pb_1()
                                    .gap_1()
                                    .border_l_1()
                                    .border_color(self.tool_card_border_color(cx))
                                    .child(input_output_header("Raw Input:".into()))
                                    .children(tool_call.raw_input_markdown.clone().map(|input| {
                                        div().id(("tool-call-raw-input-markdown", entry_ix)).child(
                                            self.render_markdown(
                                                input,
                                                MarkdownStyle::themed(
                                                    MarkdownFont::Agent,
                                                    window,
                                                    cx,
                                                ),
                                            ),
                                        )
                                    }))
                                    .child(input_output_header("Output:".into())),
                            )
                        })
                        .children(tool_call.content.iter().enumerate().map(
                            |(content_ix, content)| {
                                div().id(("tool-call-output", entry_ix)).child(
                                    self.render_tool_call_content(
                                        entry_ix,
                                        content,
                                        content_ix,
                                        tool_call,
                                        use_card_layout,
                                        has_image_content,
                                        failed_or_canceled,
                                        window,
                                        cx,
                                    ),
                                )
                            },
                        ))
                        .into_any(),
                    ToolCallStatus::Rejected => Empty.into_any(),
                }
                .into()
            } else {
                None
            };

        v_flex()
            .map(|this| {
                if use_card_layout {
                    this.my_1p5()
                        .rounded_md()
                        .border_1()
                        .when(failed_or_canceled, |this| this.border_dashed())
                        .border_color(self.tool_card_border_color(cx))
                        .bg(cx.theme().colors().editor_background)
                        .overflow_hidden()
                } else {
                    this.my_1()
                }
            })
            .map(|this| {
                if has_location && !use_card_layout {
                    this.ml_4()
                } else {
                    this.ml_5()
                }
            })
            .mr_5()
            .map(|this| {
                if is_terminal_tool {
                    let label_source = tool_call.label.read(cx).source();
                    this.child(self.render_collapsible_command(true, label_source, &tool_call.id, cx))
                } else {
                    this.child(
                        h_flex()
                            .group(&card_header_id)
                            .relative()
                            .w_full()
                            .gap_1()
                            .justify_between()
                            .when(use_card_layout, |this| {
                                this.p_0p5()
                                    .rounded_t(rems_from_px(5.))
                                    .bg(self.tool_card_header_bg(cx))
                            })
                            .child(self.render_tool_call_label(
                                entry_ix,
                                tool_call,
                                is_edit,
                                is_cancelled_edit,
                                has_revealed_diff,
                                use_card_layout,
                                window,
                                cx,
                            ))
                            .when(is_collapsible || failed_or_canceled, |this| {
                                let diff_for_discard =
                                    if has_revealed_diff && is_cancelled_edit && cx.has_flag::<AgentV2FeatureFlag>() {
                                        tool_call.diffs().next().cloned()
                                    } else {
                                        None
                                    };
                                this.child(
                                    h_flex()
                                        .px_1()
                                        .when_some(diff_for_discard.clone(), |this, _| this.pr_0p5())
                                        .gap_1()
                                        .when(is_collapsible, |this| {
                                            this.child(
                                            Disclosure::new(("expand-output", entry_ix), is_open)
                                                .opened_icon(IconName::ChevronUp)
                                                .closed_icon(IconName::ChevronDown)
                                                .visible_on_hover(&card_header_id)
                                                .on_click(cx.listener({
                                                    let id = tool_call.id.clone();
                                                    move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                                                        if let Some(active) = this.as_active_thread() {
                                                            active.update(cx, |active, _cx| {
                                                                if is_open {
                                                                    active.expanded_tool_calls.remove(&id);
                                                                } else {
                                                                    active.expanded_tool_calls.insert(id.clone());
                                                                }
                                                            });
                                                            cx.notify();
                                                        }
                                                    }
                                                })),
                                        )
                                        })
                                        .when(failed_or_canceled, |this| {
                                            if is_cancelled_edit && !has_revealed_diff {
                                                this.child(
                                                    div()
                                                        .id(entry_ix)
                                                        .tooltip(Tooltip::text(
                                                            "Interrupted Edit",
                                                        ))
                                                        .child(
                                                            Icon::new(IconName::XCircle)
                                                                .color(Color::Muted)
                                                                .size(IconSize::Small),
                                                        ),
                                                )
                                            } else if is_cancelled_edit {
                                                this
                                            } else {
                                                this.child(
                                                    Icon::new(IconName::Close)
                                                        .color(Color::Error)
                                                        .size(IconSize::Small),
                                                )
                                            }
                                        })
                                        .when_some(diff_for_discard, |this, diff| {
                                            let tool_call_id = tool_call.id.clone();
                                            let is_discarded = self.as_active_thread().is_some_and(|active| active.read(cx).discarded_partial_edits.contains(&tool_call_id));
                                            this.when(!is_discarded, |this| {
                                                this.child(
                                                    IconButton::new(
                                                        ("discard-partial-edit", entry_ix),
                                                        IconName::Undo,
                                                    )
                                                    .icon_size(IconSize::Small)
                                                    .tooltip(move |_, cx| Tooltip::with_meta(
                                                        "Discard Interrupted Edit",
                                                        None,
                                                        "You can discard this interrupted partial edit and restore the original file content.",
                                                        cx
                                                    ))
                                                    .on_click(cx.listener({
                                                        let tool_call_id = tool_call_id.clone();
                                                        move |this, _, _window, cx| {
                                                            let diff_data = diff.read(cx);
                                                            let base_text = diff_data.base_text().clone();
                                                            let buffer = diff_data.buffer().clone();
                                                            buffer.update(cx, |buffer, cx| {
                                                                buffer.set_text(base_text.as_ref(), cx);
                                                            });
                                                            if let Some(active) = this.as_active_thread() {
                                                                active.update(cx, |active, _cx| {
                                                                    active.discarded_partial_edits.insert(tool_call_id.clone());
                                                                });
                                                            }
                                                            cx.notify();
                                                        }
                                                    })),
                                                )
                                            })
                                        })

                                )
                            }),
                    )
                }
            })
            .children(tool_output_display)
    }

    fn render_tool_call_label(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        is_edit: bool,
        has_failed: bool,
        has_revealed_diff: bool,
        use_card_layout: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let has_location = tool_call.locations.len() == 1;
        let is_file = tool_call.kind == acp::ToolKind::Edit && has_location;

        let file_icon = if has_location {
            FileIcons::get_icon(&tool_call.locations[0].path, cx)
                .map(Icon::from_path)
                .unwrap_or(Icon::new(IconName::ToolPencil))
        } else {
            Icon::new(IconName::ToolPencil)
        };

        let tool_icon = if is_file && has_failed && has_revealed_diff {
            div()
                .id(entry_ix)
                .tooltip(Tooltip::text("Interrupted Edit"))
                .child(DecoratedIcon::new(
                    file_icon,
                    Some(
                        IconDecoration::new(
                            IconDecorationKind::Triangle,
                            self.tool_card_header_bg(cx),
                            cx,
                        )
                        .color(cx.theme().status().warning)
                        .position(gpui::Point {
                            x: px(-2.),
                            y: px(-2.),
                        }),
                    ),
                ))
                .into_any_element()
        } else if is_file {
            div().child(file_icon).into_any_element()
        } else {
            div()
                .child(
                    Icon::new(match tool_call.kind {
                        acp::ToolKind::Read => IconName::ToolSearch,
                        acp::ToolKind::Edit => IconName::ToolPencil,
                        acp::ToolKind::Delete => IconName::ToolDeleteFile,
                        acp::ToolKind::Move => IconName::ArrowRightLeft,
                        acp::ToolKind::Search => IconName::ToolSearch,
                        acp::ToolKind::Execute => IconName::ToolTerminal,
                        acp::ToolKind::Think => IconName::ToolThink,
                        acp::ToolKind::Fetch => IconName::ToolWeb,
                        acp::ToolKind::SwitchMode => IconName::ArrowRightLeft,
                        acp::ToolKind::Other | _ => IconName::ToolHammer,
                    })
                    .size(IconSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        };

        let gradient_overlay = {
            div()
                .absolute()
                .top_0()
                .right_0()
                .w_12()
                .h_full()
                .map(|this| {
                    if use_card_layout {
                        this.bg(linear_gradient(
                            90.,
                            linear_color_stop(self.tool_card_header_bg(cx), 1.),
                            linear_color_stop(self.tool_card_header_bg(cx).opacity(0.2), 0.),
                        ))
                    } else {
                        this.bg(linear_gradient(
                            90.,
                            linear_color_stop(cx.theme().colors().panel_background, 1.),
                            linear_color_stop(
                                cx.theme().colors().panel_background.opacity(0.2),
                                0.,
                            ),
                        ))
                    }
                })
        };

        h_flex()
            .relative()
            .w_full()
            .h(window.line_height() - px(2.))
            .text_size(self.tool_name_font_size())
            .gap_1p5()
            .when(has_location || use_card_layout, |this| this.px_1())
            .when(has_location, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .rounded(rems_from_px(3.)) // Concentric border radius
                    .hover(|s| s.bg(cx.theme().colors().element_hover.opacity(0.5)))
            })
            .overflow_hidden()
            .child(tool_icon)
            .child(if has_location {
                h_flex()
                    .id(("open-tool-call-location", entry_ix))
                    .w_full()
                    .map(|this| {
                        if use_card_layout {
                            this.text_color(cx.theme().colors().text)
                        } else {
                            this.text_color(cx.theme().colors().text_muted)
                        }
                    })
                    .child(
                        self.render_markdown(
                            tool_call.label.clone(),
                            MarkdownStyle {
                                prevent_mouse_interaction: true,
                                ..MarkdownStyle::themed(MarkdownFont::Agent, window, cx)
                                    .with_muted_text(cx)
                            },
                        ),
                    )
                    .tooltip(Tooltip::text("Go to File"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.open_tool_call_location(entry_ix, 0, window, cx);
                    }))
                    .into_any_element()
            } else {
                h_flex()
                    .w_full()
                    .child(self.render_markdown(
                        tool_call.label.clone(),
                        MarkdownStyle::themed(MarkdownFont::Agent, window, cx).with_muted_text(cx),
                    ))
                    .into_any()
            })
            .when(!is_edit, |this| this.child(gradient_overlay))
    }

    fn render_tool_call_content(
        &self,
        entry_ix: usize,
        content: &ToolCallContent,
        context_ix: usize,
        tool_call: &ToolCall,
        card_layout: bool,
        is_image_tool_call: bool,
        has_failed: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match content {
            ToolCallContent::ContentBlock(content) => {
                if let Some(resource_link) = content.resource_link() {
                    self.render_resource_link(resource_link, cx)
                } else if let Some(markdown) = content.markdown() {
                    self.render_markdown_output(
                        markdown.clone(),
                        tool_call.id.clone(),
                        context_ix,
                        card_layout,
                        window,
                        cx,
                    )
                } else if let Some(image) = content.image() {
                    let location = tool_call.locations.first().cloned();
                    self.render_image_output(
                        entry_ix,
                        image.clone(),
                        location,
                        card_layout,
                        is_image_tool_call,
                        cx,
                    )
                } else {
                    Empty.into_any_element()
                }
            }
            ToolCallContent::Diff(diff) => {
                self.render_diff_editor(entry_ix, diff, tool_call, has_failed, cx)
            }
            ToolCallContent::Terminal(terminal) => {
                self.render_terminal_tool_call(entry_ix, terminal, tool_call, window, cx)
            }
            ToolCallContent::SubagentThread(_thread) => {
                // Subagent threads are rendered by render_subagent_tool_call, not here
                Empty.into_any_element()
            }
        }
    }

    fn render_subagent_tool_call(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let subagent_threads: Vec<_> = tool_call
            .content
            .iter()
            .filter_map(|c| c.subagent_thread().cloned())
            .collect();

        let tool_call_status = &tool_call.status;

        v_flex()
            .mx_5()
            .my_1p5()
            .gap_3()
            .children(
                subagent_threads
                    .into_iter()
                    .enumerate()
                    .map(|(context_ix, thread)| {
                        self.render_subagent_card(
                            entry_ix,
                            context_ix,
                            &thread,
                            tool_call_status,
                            window,
                            cx,
                        )
                    }),
            )
    }

    fn render_subagent_card(
        &self,
        entry_ix: usize,
        context_ix: usize,
        thread: &Entity<AcpThread>,
        tool_call_status: &ToolCallStatus,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let thread_read = thread.read(cx);
        let session_id = thread_read.session_id().clone();
        let title = thread_read.title();
        let action_log = thread_read.action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        let is_expanded = self
            .as_active_thread()
            .is_some_and(|active| active.read(cx).expanded_subagents.contains(&session_id));
        let files_changed = changed_buffers.len();
        let diff_stats = DiffStats::all_files(&changed_buffers, cx);

        let is_running = matches!(
            tool_call_status,
            ToolCallStatus::Pending | ToolCallStatus::InProgress
        );
        let is_canceled_or_failed = matches!(
            tool_call_status,
            ToolCallStatus::Canceled | ToolCallStatus::Failed | ToolCallStatus::Rejected
        );

        let card_header_id =
            SharedString::from(format!("subagent-header-{}-{}", entry_ix, context_ix));
        let diff_stat_id = SharedString::from(format!("subagent-diff-{}-{}", entry_ix, context_ix));

        let icon = h_flex().w_4().justify_center().child(if is_running {
            SpinnerLabel::new()
                .size(LabelSize::Small)
                .into_any_element()
        } else if is_canceled_or_failed {
            Icon::new(IconName::Close)
                .size(IconSize::Small)
                .color(Color::Error)
                .into_any_element()
        } else {
            Icon::new(IconName::Check)
                .size(IconSize::Small)
                .color(Color::Success)
                .into_any_element()
        });

        let has_expandable_content = thread_read.entries().iter().rev().any(|entry| {
            if let AgentThreadEntry::AssistantMessage(msg) = entry {
                msg.chunks.iter().any(|chunk| match chunk {
                    AssistantMessageChunk::Message { block } => block.markdown().is_some(),
                    AssistantMessageChunk::Thought { block } => block.markdown().is_some(),
                })
            } else {
                false
            }
        });

        v_flex()
            .w_full()
            .rounded_md()
            .border_1()
            .border_color(self.tool_card_border_color(cx))
            .overflow_hidden()
            .child(
                h_flex()
                    .group(&card_header_id)
                    .py_1()
                    .px_1p5()
                    .w_full()
                    .gap_1()
                    .justify_between()
                    .bg(self.tool_card_header_bg(cx))
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(icon)
                            .child(
                                Label::new(title.to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Default),
                            )
                            .when(files_changed > 0, |this| {
                                this.child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Label::new(format!(
                                                "— {} {} changed",
                                                files_changed,
                                                if files_changed == 1 { "file" } else { "files" }
                                            ))
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                        )
                                        .child(DiffStat::new(
                                            diff_stat_id.clone(),
                                            diff_stats.lines_added as usize,
                                            diff_stats.lines_removed as usize,
                                        )),
                                )
                            }),
                    )
                    .child(
                        h_flex()
                            .gap_1p5()
                            .when(is_running, |buttons| {
                                buttons.child(
                                    Button::new(
                                        SharedString::from(format!(
                                            "stop-subagent-{}-{}",
                                            entry_ix, context_ix
                                        )),
                                        "Stop",
                                    )
                                    .icon(IconName::Stop)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Error)
                                    .label_size(LabelSize::Small)
                                    .tooltip(Tooltip::text("Stop this subagent"))
                                    .on_click({
                                        let thread = thread.clone();
                                        cx.listener(move |_this, _event, _window, cx| {
                                            thread.update(cx, |thread, _cx| {
                                                thread.stop_by_user();
                                            });
                                        })
                                    }),
                                )
                            })
                            .child(
                                IconButton::new(
                                    SharedString::from(format!(
                                        "subagent-disclosure-{}-{}",
                                        entry_ix, context_ix
                                    )),
                                    if is_expanded {
                                        IconName::ChevronUp
                                    } else {
                                        IconName::ChevronDown
                                    },
                                )
                                .shape(IconButtonShape::Square)
                                .icon_color(Color::Muted)
                                .icon_size(IconSize::Small)
                                .disabled(!has_expandable_content)
                                .when(has_expandable_content, |button| {
                                    button.on_click(cx.listener({
                                        move |this, _, _, cx| {
                                            if let Some(active) = this.as_active_thread() {
                                                active.update(cx, |active, _cx| {
                                                    if active
                                                        .expanded_subagents
                                                        .contains(&session_id)
                                                    {
                                                        active
                                                            .expanded_subagents
                                                            .remove(&session_id);
                                                    } else {
                                                        active
                                                            .expanded_subagents
                                                            .insert(session_id.clone());
                                                    }
                                                });
                                            }
                                            cx.notify();
                                        }
                                    }))
                                })
                                .when(
                                    !has_expandable_content,
                                    |button| {
                                        button.tooltip(Tooltip::text("Waiting for content..."))
                                    },
                                ),
                            ),
                    ),
            )
            .when(is_expanded, |this| {
                this.child(
                    self.render_subagent_expanded_content(entry_ix, context_ix, thread, window, cx),
                )
            })
            .children(
                thread_read
                    .first_tool_awaiting_confirmation()
                    .and_then(|tc| {
                        if let ToolCallStatus::WaitingForConfirmation { options, .. } = &tc.status {
                            Some(self.render_subagent_pending_tool_call(
                                entry_ix,
                                context_ix,
                                thread.clone(),
                                tc,
                                options,
                                window,
                                cx,
                            ))
                        } else {
                            None
                        }
                    }),
            )
            .into_any_element()
    }

    fn render_subagent_expanded_content(
        &self,
        _entry_ix: usize,
        _context_ix: usize,
        thread: &Entity<AcpThread>,
        window: &Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let thread_read = thread.read(cx);
        let session_id = thread_read.session_id().clone();
        let entries = thread_read.entries();

        // Find the most recent agent message with any content (message or thought)
        let last_assistant_markdown = entries.iter().rev().find_map(|entry| {
            if let AgentThreadEntry::AssistantMessage(msg) = entry {
                msg.chunks.iter().find_map(|chunk| match chunk {
                    AssistantMessageChunk::Message { block } => block.markdown().cloned(),
                    AssistantMessageChunk::Thought { block } => block.markdown().cloned(),
                })
            } else {
                None
            }
        });

        let scroll_handle = self
            .as_active_thread()
            .map(|state| {
                state
                    .read(cx)
                    .subagent_scroll_handles
                    .borrow_mut()
                    .entry(session_id.clone())
                    .or_default()
                    .clone()
            })
            .unwrap_or_default();

        scroll_handle.scroll_to_bottom();
        let editor_bg = cx.theme().colors().editor_background;

        let gradient_overlay = {
            div().absolute().inset_0().bg(linear_gradient(
                180.,
                linear_color_stop(editor_bg, 0.),
                linear_color_stop(editor_bg.opacity(0.), 0.15),
            ))
        };

        div()
            .relative()
            .w_full()
            .max_h_56()
            .p_2p5()
            .text_ui(cx)
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .bg(editor_bg.opacity(0.4))
            .overflow_hidden()
            .child(
                div()
                    .id(format!("subagent-content-{}", session_id))
                    .size_full()
                    .track_scroll(&scroll_handle)
                    .when_some(last_assistant_markdown, |this, markdown| {
                        this.child(self.render_markdown(
                            markdown,
                            MarkdownStyle::themed(MarkdownFont::Agent, window, cx),
                        ))
                    }),
            )
            .child(gradient_overlay)
    }

    fn render_empty_state_section_header(
        &self,
        label: impl Into<SharedString>,
        action_slot: Option<AnyElement>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().pl_1().pr_1p5().child(
            h_flex()
                .mt_2()
                .pl_1p5()
                .pb_1()
                .w_full()
                .justify_between()
                .border_b_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Label::new(label.into())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .children(action_slot),
        )
    }

    fn render_resume_notice(&self, _cx: &Context<Self>) -> AnyElement {
        let description = "This agent does not support viewing previous messages. However, your session will still continue from where you last left off.";

        div()
            .px_2()
            .pt_2()
            .pb_3()
            .w_full()
            .child(
                Callout::new()
                    .severity(Severity::Info)
                    .icon(IconName::Info)
                    .title("Resumed Session")
                    .description(description),
            )
            .into_any_element()
    }

    fn update_recent_history_from_cache(
        &mut self,
        history: &Entity<AcpThreadHistory>,
        cx: &mut Context<Self>,
    ) {
        self.recent_history_entries = history.read(cx).get_recent_sessions(3);
        self.hovered_recent_history_item = None;
        cx.notify();
    }

    fn render_recent_history(&self, cx: &mut Context<Self>) -> AnyElement {
        let render_history = !self.recent_history_entries.is_empty();

        v_flex()
            .size_full()
            .when(render_history, |this| {
                let recent_history = self.recent_history_entries.clone();
                this.justify_end().child(
                    v_flex()
                        .child(
                            self.render_empty_state_section_header(
                                "Recent",
                                Some(
                                    Button::new("view-history", "View All")
                                        .style(ButtonStyle::Subtle)
                                        .label_size(LabelSize::Small)
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &OpenHistory,
                                                &self.focus_handle(cx),
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(move |_event, window, cx| {
                                            window.dispatch_action(OpenHistory.boxed_clone(), cx);
                                        })
                                        .into_any_element(),
                                ),
                                cx,
                            ),
                        )
                        .child(v_flex().p_1().pr_1p5().gap_1().children({
                            let supports_delete = self.history.read(cx).supports_delete();
                            recent_history
                                .into_iter()
                                .enumerate()
                                .map(move |(index, entry)| {
                                    // TODO: Add keyboard navigation.
                                    let is_hovered =
                                        self.hovered_recent_history_item == Some(index);
                                    crate::acp::thread_history::AcpHistoryEntryElement::new(
                                        entry,
                                        cx.entity().downgrade(),
                                    )
                                    .hovered(is_hovered)
                                    .supports_delete(supports_delete)
                                    .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                                        if *is_hovered {
                                            this.hovered_recent_history_item = Some(index);
                                        } else if this.hovered_recent_history_item == Some(index) {
                                            this.hovered_recent_history_item = None;
                                        }
                                        cx.notify();
                                    }))
                                    .into_any_element()
                                })
                        })),
                )
            })
            .into_any()
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
        let acp_thread = self.as_active_thread()?.read(cx).thread.read(cx);
        acp_thread.connection().clone().downcast()
    }

    pub(crate) fn as_native_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        let acp_thread = self.as_active_thread()?.read(cx).thread.read(cx);
        self.as_native_connection(cx)?
            .thread(acp_thread.session_id(), cx)
    }

    fn queued_messages_len(&self, cx: &App) -> usize {
        self.as_active_thread()
            .map(|thread| thread.read(cx).local_queued_messages.len())
            .unwrap_or_default()
    }

    fn remove_from_queue(&mut self, index: usize, cx: &mut Context<Self>) -> Option<QueuedMessage> {
        self.as_active_thread()
            .and_then(|active| active.update(cx, |active, cx| active.remove_from_queue(index, cx)))
    }

    fn update_queued_message(
        &mut self,
        index: usize,
        content: Vec<acp::ContentBlock>,
        tracked_buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) -> bool {
        match self.as_active_thread() {
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

    fn clear_queue(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.local_queued_messages.clear();
                active.sync_queue_flag_to_native_thread(cx);
            });
        }
    }

    fn queued_message_contents(&self, cx: &App) -> Vec<Vec<acp::ContentBlock>> {
        match self.as_active_thread() {
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
        let editor = match self.as_active_thread() {
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
        let message_editor = self.message_editor.clone();

        let Some(thread) = self.as_active_thread() else {
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

            let message_editor = message_editor.clone();
            let subscription = cx.subscribe_in(
                &editor,
                window,
                move |this, _editor, event, window, cx| match event {
                    MessageEditorEvent::LostFocus => {
                        this.save_queued_message_at_index(index, cx);
                    }
                    MessageEditorEvent::Cancel => {
                        window.focus(&message_editor.focus_handle(cx), cx);
                    }
                    MessageEditorEvent::Send => {
                        window.focus(&message_editor.focus_handle(cx), cx);
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

        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, _cx| {
                active.last_synced_queue_length = needed_count;
            });
        }
    }

    fn is_imported_thread(&self, cx: &App) -> bool {
        if let Some(active) = self.as_active_thread() {
            active.read(cx).is_imported_thread(cx)
        } else {
            false
        }
    }

    fn keep_all(&mut self, _: &KeepAll, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.keep_all(window, cx);
            });
        };
    }

    fn reject_all(&mut self, _: &RejectAll, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.reject_all(window, cx);
            });
        };
    }

    fn allow_always(&mut self, _: &AllowAlways, window: &mut Window, cx: &mut Context<Self>) {
        self.authorize_pending_tool_call(acp::PermissionOptionKind::AllowAlways, window, cx);
    }

    fn allow_once(&mut self, _: &AllowOnce, window: &mut Window, cx: &mut Context<Self>) {
        self.authorize_pending_with_granularity(true, window, cx);
    }

    fn reject_once(&mut self, _: &RejectOnce, window: &mut Window, cx: &mut Context<Self>) {
        self.authorize_pending_with_granularity(false, window, cx);
    }

    fn authorize_pending_with_granularity(
        &mut self,
        is_allow: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let active = self.as_active_thread()?;
        let thread = active.read(cx).thread.read(cx);
        let tool_call = thread.first_tool_awaiting_confirmation()?;
        let ToolCallStatus::WaitingForConfirmation { options, .. } = &tool_call.status else {
            return None;
        };
        let tool_call_id = tool_call.id.clone();

        let PermissionOptions::Dropdown(choices) = options else {
            let kind = if is_allow {
                acp::PermissionOptionKind::AllowOnce
            } else {
                acp::PermissionOptionKind::RejectOnce
            };
            return self.authorize_pending_tool_call(kind, window, cx);
        };

        // Get selected index, defaulting to last option ("Only this time")
        let selected_index = if let Some(active) = self.as_active_thread() {
            active
                .read(cx)
                .selected_permission_granularity
                .get(&tool_call_id)
                .copied()
                .unwrap_or_else(|| choices.len().saturating_sub(1))
        } else {
            choices.len().saturating_sub(1)
        };

        let selected_choice = choices.get(selected_index).or(choices.last())?;

        let selected_option = if is_allow {
            &selected_choice.allow
        } else {
            &selected_choice.deny
        };

        self.authorize_tool_call(
            tool_call_id,
            selected_option.option_id.clone(),
            selected_option.kind,
            window,
            cx,
        );

        Some(())
    }

    fn open_permission_dropdown(
        &mut self,
        _: &crate::OpenPermissionDropdown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.as_active_thread() {
            active
                .read(cx)
                .permission_dropdown_handle
                .clone()
                .toggle(window, cx);
        }
    }

    fn handle_select_permission_granularity(
        &mut self,
        action: &SelectPermissionGranularity,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, cx| {
                active.handle_select_permission_granularity(action, cx);
            });
        }
    }

    fn handle_authorize_tool_call(
        &mut self,
        action: &AuthorizeToolCall,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let tool_call_id = acp::ToolCallId::new(action.tool_call_id.clone());
        let option_id = acp::PermissionOptionId::new(action.option_id.clone());
        let option_kind = match action.option_kind.as_str() {
            "AllowOnce" => acp::PermissionOptionKind::AllowOnce,
            "AllowAlways" => acp::PermissionOptionKind::AllowAlways,
            "RejectOnce" => acp::PermissionOptionKind::RejectOnce,
            "RejectAlways" => acp::PermissionOptionKind::RejectAlways,
            _ => acp::PermissionOptionKind::AllowOnce,
        };

        self.authorize_tool_call(tool_call_id, option_id, option_kind, window, cx);
    }

    fn authorize_pending_tool_call(
        &mut self,
        kind: acp::PermissionOptionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        self.as_active_thread()?.update(cx, |active, cx| {
            active.authorize_pending_tool_call(kind, window, cx)
        })
    }

    fn open_add_context_menu(
        &mut self,
        _action: &OpenAddContextMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let menu_handle = self.add_context_menu_handle.clone();
        window.defer(cx, move |window, cx| {
            menu_handle.toggle(window, cx);
        });
    }

    fn render_markdown(&self, markdown: Entity<Markdown>, style: MarkdownStyle) -> MarkdownElement {
        let workspace = self.workspace.clone();
        MarkdownElement::new(markdown, style).on_url_click(move |text, window, cx| {
            Self::open_link(text, &workspace, window, cx);
        })
    }

    fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.read(cx).list_state.scroll_to(ListOffset::default());
            cx.notify();
        }
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

    fn render_token_limit_callout(&self, cx: &mut Context<Self>) -> Option<Callout> {
        let Some(active) = self.as_active_thread() else {
            return None;
        };

        let active_read = active.read(cx);
        if active_read.token_limit_callout_dismissed {
            return None;
        }

        let token_usage = active_read.thread.read(cx).token_usage()?;
        let ratio = token_usage.ratio();

        let (severity, icon, title) = match ratio {
            acp_thread::TokenUsageRatio::Normal => return None,
            acp_thread::TokenUsageRatio::Warning => (
                Severity::Warning,
                IconName::Warning,
                "Thread reaching the token limit soon",
            ),
            acp_thread::TokenUsageRatio::Exceeded => (
                Severity::Error,
                IconName::XCircle,
                "Thread reached the token limit",
            ),
        };

        let description = "To continue, start a new thread from a summary.";

        Some(
            Callout::new()
                .severity(severity)
                .icon(icon)
                .title(title)
                .description(description)
                .actions_slot(
                    h_flex().gap_0p5().child(
                        Button::new("start-new-thread", "Start New Thread")
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, window, cx| {
                                let Some(active) = this.as_active_thread() else {
                                    return;
                                };
                                let session_id =
                                    active.read(cx).thread.read(cx).session_id().clone();
                                window.dispatch_action(
                                    crate::NewNativeAgentThreadFromSummary {
                                        from_session_id: session_id,
                                    }
                                    .boxed_clone(),
                                    cx,
                                );
                            })),
                    ),
                )
                .dismiss_action(self.dismiss_error_button(cx)),
        )
    }

    fn agent_ui_font_size_changed(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry_view_state) = self
            .as_active_thread()
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
        self.message_editor.update(cx, |message_editor, cx| {
            message_editor.insert_dragged_files(paths, added_worktrees, window, cx);
        })
    }

    /// Inserts the selected text into the message editor or the message being
    /// edited, if any.
    pub(crate) fn insert_selections(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_editor(cx).update(cx, |editor, cx| {
            editor.insert_selections(window, cx);
        });
    }

    /// Inserts terminal text as a crease into the message editor.
    pub(crate) fn insert_terminal_text(
        &self,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.message_editor.update(cx, |message_editor, cx| {
            message_editor.insert_terminal_crease(text, window, cx);
        });
    }

    /// Inserts code snippets as creases into the message editor.
    pub(crate) fn insert_code_crease(
        &self,
        creases: Vec<(String, String)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.message_editor.update(cx, |message_editor, cx| {
            message_editor.insert_code_creases(creases, window, cx);
        });
    }

    fn render_codex_windows_warning(&self, cx: &mut Context<Self>) -> Callout {
        Callout::new()
            .icon(IconName::Warning)
            .severity(Severity::Warning)
            .title("Codex on Windows")
            .description("For best performance, run Codex in Windows Subsystem for Linux (WSL2)")
            .actions_slot(
                Button::new("open-wsl-modal", "Open in WSL")
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .on_click(cx.listener({
                        move |_, _, _window, cx| {
                            #[cfg(windows)]
                            _window.dispatch_action(
                                zed_actions::wsl_actions::OpenWsl::default().boxed_clone(),
                                cx,
                            );
                            cx.notify();
                        }
                    })),
            )
            .dismiss_action(
                IconButton::new("dismiss", IconName::Close)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .tooltip(Tooltip::text("Dismiss Warning"))
                    .on_click(cx.listener({
                        move |this, _, _, cx| {
                            this.show_codex_windows_warning = false;
                            cx.notify();
                        }
                    })),
            )
    }

    fn clear_command_load_errors(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, _cx| {
                active.command_load_errors_dismissed = true;
            });
        }
        cx.notify();
    }

    fn refresh_cached_user_commands(&mut self, cx: &mut Context<Self>) {
        let Some(registry) = self.slash_command_registry.clone() else {
            return;
        };
        self.refresh_cached_user_commands_from_registry(&registry, cx);
    }

    fn refresh_cached_user_commands_from_registry(
        &mut self,
        registry: &Entity<SlashCommandRegistry>,
        cx: &mut Context<Self>,
    ) {
        let Some(thread_state) = self.as_active_thread() else {
            return;
        };
        thread_state.update(cx, |thread_state, cx| {
            thread_state.refresh_cached_user_commands_from_registry(registry, cx);
        });
        cx.notify();
    }

    /// Returns the cached slash commands, if available.
    pub fn cached_slash_commands(
        &self,
        _cx: &App,
    ) -> collections::HashMap<String, UserSlashCommand> {
        let Some(thread_state) = &self.as_active_thread() else {
            return collections::HashMap::default();
        };
        thread_state.read(_cx).cached_user_commands.borrow().clone()
    }

    /// Returns the cached slash command errors, if available.
    fn cached_slash_command_errors(&self, _cx: &App) -> Vec<CommandLoadError> {
        let Some(thread_state) = &self.as_active_thread() else {
            return Vec::new();
        };
        thread_state
            .read(_cx)
            .cached_user_command_errors
            .borrow()
            .clone()
    }

    fn render_new_version_callout(&self, version: &SharedString, cx: &mut Context<Self>) -> Div {
        v_flex().w_full().justify_end().child(
            h_flex()
                .p_2()
                .pr_3()
                .w_full()
                .gap_1p5()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().element_background)
                .child(
                    h_flex()
                        .flex_1()
                        .gap_1p5()
                        .child(
                            Icon::new(IconName::Download)
                                .color(Color::Accent)
                                .size(IconSize::Small),
                        )
                        .child(Label::new("New version available").size(LabelSize::Small)),
                )
                .child(
                    Button::new("update-button", format!("Update to v{}", version))
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Tinted(TintColor::Accent))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.reset(window, cx);
                        })),
                ),
        )
    }

    fn current_model_name(&self, cx: &App) -> SharedString {
        // For native agent (Zed Agent), use the specific model name (e.g., "Claude 3.5 Sonnet")
        // For ACP agents, use the agent name (e.g., "Claude Code", "Gemini CLI")
        // This provides better clarity about what refused the request
        if self.as_native_connection(cx).is_some() {
            self.as_active_thread()
                .and_then(|active| active.read(cx).model_selector.clone())
                .and_then(|selector| selector.read(cx).active_model(cx))
                .map(|model| model.name.clone())
                .unwrap_or_else(|| SharedString::from("The model"))
        } else {
            // ACP agent - use the agent name (e.g., "Claude Code", "Gemini CLI")
            self.agent.name()
        }
    }

    fn set_can_fast_track_queue(&mut self, value: bool, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, _cx| {
                active.can_fast_track_queue = value;
            });
        }
    }

    fn create_copy_button(&self, message: impl Into<String>) -> impl IntoElement {
        let message = message.into();

        CopyButton::new("copy-error-message", message).tooltip_label("Copy Error Message")
    }

    fn dismiss_error_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        IconButton::new("dismiss", IconName::Close)
            .icon_size(IconSize::Small)
            .tooltip(Tooltip::text("Dismiss"))
            .on_click(cx.listener({
                move |this, _, _, cx| {
                    this.clear_thread_error(cx);
                    cx.notify();
                }
            }))
    }

    pub(crate) fn reauthenticate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let agent_name = self.agent.name();
        self.clear_thread_error(cx);
        let this = cx.weak_entity();
        window.defer(cx, |window, cx| {
            Self::handle_auth_required(this, AuthRequired::new(), agent_name, window, cx);
        })
    }

    pub fn delete_history_entry(&mut self, entry: AgentSessionInfo, cx: &mut Context<Self>) {
        let task = self.history.update(cx, |history, cx| {
            history.delete_session(&entry.session_id, cx)
        });
        task.detach_and_log_err(cx);
    }

    /// Returns the currently active editor, either for a message that is being
    /// edited or the editor for a new message.
    fn active_editor(&self, cx: &App) -> Entity<MessageEditor> {
        if let Some(thread_state) = self.as_active_thread()
            && let Some(index) = thread_state.read(cx).editing_message
            && let Some(editor) = thread_state
                .read(cx)
                .entry_view_state
                .read(cx)
                .entry(index)
                .and_then(|entry| entry.message_editor())
                .cloned()
        {
            editor
        } else {
            self.message_editor.clone()
        }
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
        match self.as_active_thread() {
            Some(_) => self.active_editor(cx).focus_handle(cx),
            None => self.focus_handle.clone(),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl AcpServerView {
    /// Expands a tool call so its content is visible.
    /// This is primarily useful for visual testing.
    pub fn expand_tool_call(&mut self, tool_call_id: acp::ToolCallId, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
            active.update(cx, |active, _cx| {
                active.expanded_tool_calls.insert(tool_call_id);
            });
            cx.notify();
        }
    }

    /// Expands a subagent card so its content is visible.
    /// This is primarily useful for visual testing.
    pub fn expand_subagent(&mut self, session_id: acp::SessionId, cx: &mut Context<Self>) {
        if let Some(active) = self.as_active_thread() {
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

        let has_messages = self
            .as_active_thread()
            .is_some_and(|active| active.read(cx).list_state.item_count() > 0);

        v_flex()
            .size_full()
            .key_context("AcpThread")
            .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                this.cancel_generation(cx);
            }))
            .on_action(cx.listener(Self::keep_all))
            .on_action(cx.listener(Self::reject_all))
            .on_action(cx.listener(Self::allow_always))
            .on_action(cx.listener(Self::allow_once))
            .on_action(cx.listener(Self::reject_once))
            .on_action(cx.listener(Self::handle_authorize_tool_call))
            .on_action(cx.listener(Self::handle_select_permission_granularity))
            .on_action(cx.listener(Self::open_permission_dropdown))
            .on_action(cx.listener(Self::open_add_context_menu))
            .on_action(cx.listener(|this, _: &ToggleThinkingMode, _window, cx| {
                if let Some(thread) = this.as_native_thread(cx) {
                    thread.update(cx, |thread, cx| {
                        thread.set_thinking_enabled(!thread.thinking_enabled(), cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &SendNextQueuedMessage, window, cx| {
                this.send_queued_message_at_index(0, true, window, cx);
            }))
            .on_action(cx.listener(|this, _: &RemoveFirstQueuedMessage, _, cx| {
                this.remove_from_queue(0, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &EditFirstQueuedMessage, window, cx| {
                if let Some(active) = this.as_active_thread()
                    && let Some(editor) = active.read(cx).queued_message_editors.first()
                {
                    window.focus(&editor.focus_handle(cx), cx);
                }
            }))
            .on_action(cx.listener(|this, _: &ClearMessageQueue, _, cx| {
                this.clear_queue(cx);
                this.set_can_fast_track_queue(false, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleProfileSelector, window, cx| {
                if let Some(config_options_view) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).config_options_view.clone())
                {
                    let handled = config_options_view.update(cx, |view, cx| {
                        view.toggle_category_picker(
                            acp::SessionConfigOptionCategory::Mode,
                            window,
                            cx,
                        )
                    });
                    if handled {
                        return;
                    }
                }

                if let Some(profile_selector) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).profile_selector.clone())
                {
                    profile_selector.read(cx).menu_handle().toggle(window, cx);
                } else if let Some(mode_selector) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).mode_selector.clone())
                {
                    mode_selector.read(cx).menu_handle().toggle(window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &CycleModeSelector, window, cx| {
                if let Some(config_options_view) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).config_options_view.clone())
                {
                    let handled = config_options_view.update(cx, |view, cx| {
                        view.cycle_category_option(
                            acp::SessionConfigOptionCategory::Mode,
                            false,
                            cx,
                        )
                    });
                    if handled {
                        return;
                    }
                }

                if let Some(profile_selector) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).profile_selector.clone())
                {
                    profile_selector.update(cx, |profile_selector, cx| {
                        profile_selector.cycle_profile(cx);
                    });
                } else if let Some(mode_selector) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).mode_selector.clone())
                {
                    mode_selector.update(cx, |mode_selector, cx| {
                        mode_selector.cycle_mode(window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &ToggleModelSelector, window, cx| {
                if let Some(config_options_view) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).config_options_view.clone())
                {
                    let handled = config_options_view.update(cx, |view, cx| {
                        view.toggle_category_picker(
                            acp::SessionConfigOptionCategory::Model,
                            window,
                            cx,
                        )
                    });
                    if handled {
                        return;
                    }
                }

                if let Some(model_selector) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).model_selector.clone())
                {
                    model_selector
                        .update(cx, |model_selector, cx| model_selector.toggle(window, cx));
                }
            }))
            .on_action(cx.listener(|this, _: &CycleFavoriteModels, window, cx| {
                if let Some(config_options_view) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).config_options_view.clone())
                {
                    let handled = config_options_view.update(cx, |view, cx| {
                        view.cycle_category_option(
                            acp::SessionConfigOptionCategory::Model,
                            true,
                            cx,
                        )
                    });
                    if handled {
                        return;
                    }
                }

                if let Some(model_selector) = this
                    .as_active_thread()
                    .and_then(|active| active.read(cx).model_selector.clone())
                {
                    model_selector.update(cx, |model_selector, cx| {
                        model_selector.cycle_favorite_models(window, cx);
                    });
                }
            }))
            .track_focus(&self.focus_handle)
            .bg(cx.theme().colors().panel_background)
            .child(match &self.server_state {
                ServerState::Loading { .. } => v_flex()
                    .flex_1()
                    .child(self.render_recent_history(cx))
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
                ServerState::Connected(connected) => v_flex().flex_1().map(|this| {
                    let this = this
                        .when(connected.current.read(cx).resumed_without_history, |this| {
                            this.child(self.render_resume_notice(cx))
                        });
                    if has_messages {
                        if let Some(thread) = self.as_active_thread() {
                            this.child(thread.update(cx, |thread, cx| thread.render_entries(cx)))
                                .vertical_scrollbar_for(&thread.read(cx).list_state, window, cx)
                                .into_any()
                        } else {
                            this.into_any()
                        }
                    } else {
                        this.child(self.render_recent_history(cx)).into_any()
                    }
                }),
            })
            // The activity bar is intentionally rendered outside of the ThreadState::Active match
            // above so that the scrollbar doesn't render behind it. The current setup allows
            // the scrollbar to stop exactly at the activity bar start.
            .when(has_messages, |this| match self.as_active_thread() {
                Some(thread) => this.children(thread.read(cx).render_activity_bar(window, cx)),
                _ => this,
            })
            .when(self.show_codex_windows_warning, |this| {
                this.child(self.render_codex_windows_warning(cx))
            })
            .when_some(self.as_active_thread(), |this, thread_state| {
                this.children(thread_state.read(cx).render_thread_retry_status_callout())
                    .children(thread_state.read(cx).render_command_load_errors(cx))
            })
            .when_some(self.as_active_thread(), |this, thread_state| {
                this.children(
                    thread_state.update(cx, |state, cx| state.render_thread_error(window, cx)),
                )
            })
            .when_some(
                match has_messages {
                    true => None,
                    false => self
                        .as_active_thread()
                        .and_then(|active| active.read(cx).new_server_version_available.clone()),
                },
                |this, version| this.child(self.render_new_version_callout(&version, cx)),
            )
            .children(self.render_token_limit_callout(cx))
            .when_some(self.as_active_thread(), |this, thread_state| {
                this.children(
                    thread_state.update(cx, |state, cx| state.render_message_editor(window, cx)),
                )
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
    use agent::ToolPermissionContext;
    use agent_client_protocol::SessionId;
    use editor::MultiBufferOffset;
    use fs::FakeFs;
    use gpui::{EventEmitter, TestAppContext, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::any::Any;
    use std::path::Path;
    use std::rc::Rc;
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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
        thread_view.read_with(cx, |view, _cx| {
            assert_eq!(view.recent_history_entries.len(), 0);
        });

        // Now set the session list - this simulates external agents providing their history
        let list_a: Rc<dyn AgentSessionList> =
            Rc::new(StubSessionList::new(vec![session_a.clone()]));
        history.update(cx, |history, cx| {
            history.set_session_list(Some(list_a), cx);
        });
        cx.run_until_parked();

        thread_view.read_with(cx, |view, _cx| {
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

        thread_view.read_with(cx, |view, _cx| {
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
            let state = view.as_active_thread().unwrap();
            assert!(state.read(cx).resumed_without_history);
            assert_eq!(state.read(cx).list_state.item_count(), 0);
        });
    }

    #[gpui::test]
    async fn test_refusal_handling(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) =
            setup_thread_view(StubAgentServer::new(RefusalAgentConnection), cx).await;

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Do something harmful", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Check that the refusal error is set
        thread_view.read_with(cx, |thread_view, cx| {
            let state = thread_view.as_active_thread().unwrap();
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());

        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        // Window is active (don't deactivate), but panel will be hidden
        // Note: In the test environment, the panel is not actually added to the dock,
        // so is_agent_panel_hidden will return true

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        // Deactivate window - should show notification regardless of setting
        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        // Window is active

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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

        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            let thread = cx.new(|cx| {
                AcpThread::new(
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
    struct SaboteurAgentConnection;

    impl AgentConnection for SaboteurAgentConnection {
        fn telemetry_id(&self) -> SharedString {
            "saboteur".into()
        }

        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            Task::ready(Ok(cx.new(|cx| {
                let action_log = cx.new(|_| ActionLog::new(project.clone()));
                AcpThread::new(
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

        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::App,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            Task::ready(Ok(cx.new(|cx| {
                let action_log = cx.new(|_| ActionLog::new(project.clone()));
                AcpThread::new(
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
                view.as_active_thread().map(|r| r.read(cx).thread.clone())
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
                .as_active_thread()
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
                .as_active_thread()
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
            let active = view.as_active_thread().unwrap();
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
                view.as_active_thread().map(|r| r.read(cx).thread.clone())
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
        thread_view.update(cx, |view, cx| {
            view.scroll_to_top(cx);
        });
        cx.run_until_parked();

        thread_view.update(cx, |view, cx| {
            view.scroll_to_most_recent_user_prompt(cx);
            let scroll_top = view
                .as_active_thread()
                .map(|active| &active.read(cx).list_state)
                .unwrap()
                .logical_scroll_top();
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
        thread_view.update(cx, |view, cx| {
            view.scroll_to_most_recent_user_prompt(cx);
            let scroll_top = view
                .as_active_thread()
                .map(|active| &active.read(cx).list_state)
                .unwrap()
                .logical_scroll_top();
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.as_active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );

            view.as_active_thread()
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
                view.as_active_thread()
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
                view.as_active_thread()
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("", window, cx);
        });

        let thread = cx.read(|cx| {
            thread_view
                .read(cx)
                .as_active_thread()
                .unwrap()
                .read(cx)
                .thread
                .clone()
        });
        let entries_before = cx.read(|cx| thread.read(cx).entries().len());

        thread_view.update_in(cx, |view, window, cx| {
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |view, cx| {
            assert_eq!(
                view.as_active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );
            assert_eq!(
                view.as_active_thread()
                    .unwrap()
                    .read(cx)
                    .thread
                    .read(cx)
                    .entries()
                    .len(),
                2
            );

            view.as_active_thread()
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
                view.as_active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );

            let entries = view
                .as_active_thread()
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
                .as_active_thread()
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        let (user_message_editor, session_id) = thread_view.read_with(cx, |view, cx| {
            let thread = view.as_active_thread().unwrap().read(cx).thread.read(cx);
            assert_eq!(thread.entries().len(), 1);

            let editor = view
                .as_active_thread()
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
                view.as_active_thread()
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
                view.as_active_thread()
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
                view.as_active_thread()
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
                    .as_active_thread()
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        let (thread, session_id) = thread_view.read_with(cx, |view, cx| {
            let thread = view
                .as_active_thread()
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
            .read_with(cx, |view, _cx| view.focus_handle.clone());
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
            view.as_active_thread().unwrap().read(cx).thread.clone()
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Message 1", window, cx);
        });
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        let (thread, session_id) = thread_view.read_with(cx, |view, cx| {
            let thread = view.as_active_thread().unwrap().read(cx).thread.clone();

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
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.interrupt_and_send(window, cx);
        });

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx)
        });
        thread_view.update_in(cx, |thread_view, window, cx| thread_view.send(window, cx));
        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |thread_view, cx| {
            thread_view
                .as_active_thread()
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
                view.as_active_thread()
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
                view.as_active_thread()
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
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
                view.as_active_thread()
                    .and_then(|active| active.read(cx).editing_message),
                None
            );
            view.insert_selections(window, cx);
        });

        thread_view.read_with(cx, |thread_view, cx| {
            let text = thread_view.message_editor.read(cx).text(cx);
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

        let permission_options = ToolPermissionContext::new("terminal", "cargo build --release")
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Run cargo build", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Verify the tool call is in WaitingForConfirmation state with the expected options
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
            ToolPermissionContext::new("edit_file", "src/main.rs").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edit the main file", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Verify the options
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
            ToolPermissionContext::new("fetch", "https://docs.rs/gpui").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Fetch the docs", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Verify the options
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
        let permission_options = ToolPermissionContext::new("terminal", "./deploy.sh --production")
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Run the deploy script", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Verify only 2 options (no pattern button when command doesn't match pattern)
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
            ToolPermissionContext::new("terminal", "cargo test").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Run tests", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Verify tool call is waiting for confirmation
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
            let thread = thread_view.as_active_thread().expect("Thread should exist").read(cx).thread.clone();
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
            ToolPermissionContext::new("terminal", "npm install").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Install dependencies", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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
                .as_active_thread()
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
            ToolPermissionContext::new("terminal", "cargo build").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Build the project", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Verify default granularity is the last option (index 2 = "Only this time")
        thread_view.read_with(cx, |thread_view, cx| {
            let state = thread_view.as_active_thread().unwrap();
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
            let state = thread_view.as_active_thread().unwrap();
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
            ToolPermissionContext::new("terminal", "npm install").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Install dependencies", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

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
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.allow_once(&AllowOnce, window, cx);
        });

        cx.run_until_parked();

        // Verify tool call was authorized
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
            ToolPermissionContext::new("terminal", "git push").build_permission_options();

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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Push changes", window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        // Use default granularity (last option = "Only this time")
        // Simulate clicking the Deny button
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.reject_once(&RejectOnce, window, cx);
        });

        cx.run_until_parked();

        // Verify tool call was rejected (no longer waiting for confirmation)
        thread_view.read_with(cx, |thread_view, cx| {
            let thread = thread_view
                .as_active_thread()
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
        let permission_options = ToolPermissionContext::new("terminal", "cargo build --release")
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
                .any(|id| id.starts_with("always_allow_pattern:terminal:")),
            "Missing allow pattern option"
        );
    }

    #[gpui::test]
    async fn test_option_id_transformation_for_deny() {
        let permission_options = ToolPermissionContext::new("terminal", "cargo build --release")
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
                .any(|id| id.starts_with("always_deny_pattern:terminal:")),
            "Missing deny pattern option"
        );
    }
}
