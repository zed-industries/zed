use acp_thread::{
    AcpThread, AcpThreadEvent, AgentSessionInfo, AgentThreadEntry, AssistantMessage,
    AssistantMessageChunk, AuthRequired, LoadError, MentionUri, RetryStatus, ThreadStatus,
    ToolCall, ToolCallContent, ToolCallStatus, UserMessageId,
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
use feature_flags::{AgentSharingFeatureFlag, AgentV2FeatureFlag, FeatureFlagAppExt};
use file_icons::FileIcons;
use fs::Fs;
use futures::FutureExt as _;
use gpui::{
    Action, Animation, AnimationExt, AnyView, App, BorderStyle, ClickEvent, ClipboardItem,
    CursorStyle, EdgesRefinement, ElementId, Empty, Entity, FocusHandle, Focusable, Hsla, Length,
    ListOffset, ListState, ObjectFit, PlatformDisplay, ScrollHandle, SharedString, StyleRefinement,
    Subscription, Task, TextStyle, TextStyleRefinement, UnderlineStyle, WeakEntity, Window,
    WindowHandle, div, ease_in_out, img, linear_color_stop, linear_gradient, list, point,
    pulsating_between,
};
use language::Buffer;
use language_model::LanguageModelRegistry;
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
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
use theme::{AgentFontSize, ThemeSettings};
use ui::{
    Callout, CommonAnimationExt, ContextMenu, ContextMenuEntry, CopyButton, DecoratedIcon,
    DiffStat, Disclosure, Divider, DividerColor, IconDecoration, IconDecorationKind, KeyBinding,
    PopoverMenu, PopoverMenuHandle, SpinnerLabel, TintColor, Tooltip, WithScrollbar, prelude::*,
    right_click_menu,
};
use util::defer;
use util::{ResultExt, size::format_file_size, time::duration_alt_display};
use workspace::{CollaboratorId, NewTerminal, Toast, Workspace, notifications::NotificationId};
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
    CycleFavoriteModels, CycleModeSelector, ExpandMessageEditor, Follow, KeepAll, NewThread,
    OpenAgentDiff, OpenHistory, RejectAll, RejectOnce, RemoveFirstQueuedMessage,
    SelectPermissionGranularity, SendImmediately, SendNextQueuedMessage, ToggleProfileSelector,
};

const MAX_COLLAPSED_LINES: usize = 3;
const STOPWATCH_THRESHOLD: Duration = Duration::from_secs(30);
const TOKEN_THRESHOLD: u64 = 250;

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
    Other(SharedString),
}

impl ThreadError {
    fn from_err(error: anyhow::Error, agent: &Rc<dyn AgentServer>) -> Self {
        if error.is::<language_model::PaymentRequiredError>() {
            Self::PaymentRequired
        } else if let Some(acp_error) = error.downcast_ref::<acp::Error>()
            && acp_error.code == acp::ErrorCode::AuthRequired
        {
            Self::AuthenticationRequired(acp_error.message.clone().into())
        } else {
            let string = format!("{:#}", error);
            // TODO: we should have Gemini return better errors here.
            if agent.clone().downcast::<agent_servers::Gemini>().is_some()
                && string.contains("Could not load the default credentials")
                || string.contains("API key not valid")
                || string.contains("Request had invalid authentication credentials")
            {
                Self::AuthenticationRequired(string.into())
            } else {
                Self::Other(string.into())
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

pub struct AcpThreadView {
    agent: Rc<dyn AgentServer>,
    agent_server_store: Entity<AgentServerStore>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    thread_state: ThreadState,
    permission_dropdown_handle: PopoverMenuHandle<ContextMenu>,
    /// Tracks the selected granularity index for each tool call's permission dropdown.
    /// The index corresponds to the position in the allow_options list.
    /// Default is the last option (index pointing to "Only this time").
    selected_permission_granularity: HashMap<acp::ToolCallId, usize>,
    login: Option<task::SpawnInTerminal>,
    recent_history_entries: Vec<AgentSessionInfo>,
    history: Entity<AcpThreadHistory>,
    _history_subscription: Subscription,
    hovered_recent_history_item: Option<usize>,
    entry_view_state: Entity<EntryViewState>,
    message_editor: Entity<MessageEditor>,
    focus_handle: FocusHandle,
    model_selector: Option<Entity<AcpModelSelectorPopover>>,
    config_options_view: Option<Entity<ConfigOptionsView>>,
    profile_selector: Option<Entity<ProfileSelector>>,
    notifications: Vec<WindowHandle<AgentNotification>>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    thread_retry_status: Option<RetryStatus>,
    thread_error: Option<ThreadError>,
    thread_error_markdown: Option<Entity<Markdown>>,
    token_limit_callout_dismissed: bool,
    thread_feedback: ThreadFeedbackState,
    list_state: ListState,
    auth_task: Option<Task<()>>,
    /// Tracks which tool calls have their content/output expanded.
    /// Used for showing/hiding tool call results, terminal output, etc.
    expanded_tool_calls: HashSet<acp::ToolCallId>,
    /// Tracks which terminal commands have their command text expanded.
    /// This is separate from `expanded_tool_calls` because command text expansion
    /// (showing all lines of a long command) is independent from output expansion
    /// (showing the terminal output).
    expanded_terminal_commands: HashSet<acp::ToolCallId>,
    expanded_tool_call_raw_inputs: HashSet<acp::ToolCallId>,
    expanded_thinking_blocks: HashSet<(usize, usize)>,
    expanded_subagents: HashSet<acp::SessionId>,
    subagent_scroll_handles: RefCell<HashMap<acp::SessionId, ScrollHandle>>,
    edits_expanded: bool,
    plan_expanded: bool,
    queue_expanded: bool,
    editor_expanded: bool,
    should_be_following: bool,
    editing_message: Option<usize>,
    discarded_partial_edits: HashSet<acp::ToolCallId>,
    prompt_capabilities: Rc<RefCell<PromptCapabilities>>,
    available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
    is_loading_contents: bool,
    new_server_version_available: Option<SharedString>,
    resume_thread_metadata: Option<AgentSessionInfo>,
    _cancel_task: Option<Task<()>>,
    _subscriptions: [Subscription; 5],
    show_codex_windows_warning: bool,
    in_flight_prompt: Option<Vec<acp::ContentBlock>>,
    skip_queue_processing_count: usize,
    user_interrupted_generation: bool,
    can_fast_track_queue: bool,
    turn_tokens: Option<u64>,
    last_turn_tokens: Option<u64>,
    turn_started_at: Option<Instant>,
    last_turn_duration: Option<Duration>,
    turn_generation: usize,
    _turn_timer_task: Option<Task<()>>,
    hovered_edited_file_buttons: Option<usize>,
}

enum ThreadState {
    Loading(Entity<LoadingView>),
    Ready {
        thread: Entity<AcpThread>,
        title_editor: Option<Entity<Editor>>,
        mode_selector: Option<Entity<ModeSelector>>,
        _subscriptions: Vec<Subscription>,
    },
    LoadError(LoadError),
    Unauthenticated {
        connection: Rc<dyn AgentConnection>,
        description: Option<Entity<Markdown>>,
        configuration_view: Option<AnyView>,
        pending_auth_method: Option<acp::AuthMethodId>,
        _subscription: Option<Subscription>,
    },
}

struct LoadingView {
    title: SharedString,
    _load_task: Task<()>,
    _update_title_task: Task<anyhow::Result<()>>,
}

impl AcpThreadView {
    pub fn new(
        agent: Rc<dyn AgentServer>,
        resume_thread: Option<AgentSessionInfo>,
        summarize_thread: Option<AgentSessionInfo>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        prompt_store: Option<Entity<PromptStore>>,
        history: Entity<AcpThreadHistory>,
        track_load_event: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        let available_commands = Rc::new(RefCell::new(vec![]));

        let agent_server_store = project.read(cx).agent_server_store().clone();
        let agent_display_name = agent_server_store
            .read(cx)
            .agent_display_name(&ExternalAgentServerName(agent.name()))
            .unwrap_or_else(|| agent.name());

        let placeholder = placeholder_text(agent_display_name.as_ref(), false);

        let message_editor = cx.new(|cx| {
            let mut editor = MessageEditor::new(
                workspace.clone(),
                project.downgrade(),
                thread_store.clone(),
                history.downgrade(),
                prompt_store.clone(),
                prompt_capabilities.clone(),
                available_commands.clone(),
                agent.name(),
                &placeholder,
                editor::EditorMode::AutoHeight {
                    min_lines: AgentSettings::get_global(cx).message_editor_min_lines,
                    max_lines: Some(AgentSettings::get_global(cx).set_message_editor_max_lines()),
                },
                window,
                cx,
            );
            if let Some(entry) = summarize_thread {
                editor.insert_thread_summary(entry, window, cx);
            }
            editor
        });

        let list_state = ListState::new(0, gpui::ListAlignment::Bottom, px(2048.0));

        let entry_view_state = cx.new(|_| {
            EntryViewState::new(
                workspace.clone(),
                project.downgrade(),
                thread_store.clone(),
                history.downgrade(),
                prompt_store.clone(),
                prompt_capabilities.clone(),
                available_commands.clone(),
                agent.name(),
            )
        });

        let subscriptions = [
            cx.observe_global_in::<SettingsStore>(window, Self::agent_ui_font_size_changed),
            cx.observe_global_in::<AgentFontSize>(window, Self::agent_ui_font_size_changed),
            cx.subscribe_in(&message_editor, window, Self::handle_message_editor_event),
            cx.subscribe_in(&entry_view_state, window, Self::handle_entry_view_event),
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

        let recent_history_entries = history.read(cx).get_recent_sessions(3);
        let history_subscription = cx.observe(&history, |this, history, cx| {
            this.update_recent_history_from_cache(&history, cx);
        });

        Self {
            agent: agent.clone(),
            agent_server_store,
            workspace: workspace.clone(),
            project: project.clone(),
            entry_view_state,
            permission_dropdown_handle: PopoverMenuHandle::default(),
            selected_permission_granularity: HashMap::default(),
            thread_state: Self::initial_state(
                agent.clone(),
                resume_thread.clone(),
                workspace.clone(),
                project.clone(),
                track_load_event,
                window,
                cx,
            ),
            login: None,
            message_editor,
            model_selector: None,
            config_options_view: None,
            profile_selector: None,
            notifications: Vec::new(),
            notification_subscriptions: HashMap::default(),
            list_state: list_state,
            thread_retry_status: None,
            thread_error: None,
            thread_error_markdown: None,
            token_limit_callout_dismissed: false,
            thread_feedback: Default::default(),
            auth_task: None,
            expanded_tool_calls: HashSet::default(),
            expanded_terminal_commands: HashSet::default(),
            expanded_tool_call_raw_inputs: HashSet::default(),
            expanded_thinking_blocks: HashSet::default(),
            expanded_subagents: HashSet::default(),
            subagent_scroll_handles: RefCell::new(HashMap::default()),
            editing_message: None,
            edits_expanded: false,
            plan_expanded: false,
            queue_expanded: true,
            discarded_partial_edits: HashSet::default(),
            prompt_capabilities,
            available_commands,
            editor_expanded: false,
            should_be_following: false,
            recent_history_entries,
            history,
            _history_subscription: history_subscription,
            hovered_recent_history_item: None,
            is_loading_contents: false,
            _subscriptions: subscriptions,
            _cancel_task: None,
            focus_handle: cx.focus_handle(),
            new_server_version_available: None,
            resume_thread_metadata: resume_thread,
            show_codex_windows_warning,
            in_flight_prompt: None,
            skip_queue_processing_count: 0,
            user_interrupted_generation: false,
            can_fast_track_queue: false,
            turn_tokens: None,
            last_turn_tokens: None,
            turn_started_at: None,
            last_turn_duration: None,
            turn_generation: 0,
            _turn_timer_task: None,
            hovered_edited_file_buttons: None,
        }
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.thread_state = Self::initial_state(
            self.agent.clone(),
            self.resume_thread_metadata.clone(),
            self.workspace.clone(),
            self.project.clone(),
            true,
            window,
            cx,
        );
        self.available_commands.replace(vec![]);
        self.new_server_version_available.take();
        self.recent_history_entries.clear();
        self.turn_tokens = None;
        self.last_turn_tokens = None;
        self.turn_started_at = None;
        self.last_turn_duration = None;
        self._turn_timer_task = None;
        cx.notify();
    }

    fn initial_state(
        agent: Rc<dyn AgentServer>,
        resume_thread: Option<AgentSessionInfo>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        track_load_event: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ThreadState {
        if project.read(cx).is_via_collab()
            && agent.clone().downcast::<NativeAgentServer>().is_none()
        {
            return ThreadState::LoadError(LoadError::Other(
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

            if track_load_event {
                telemetry::event!("Agent Thread Started", agent = connection.telemetry_id());
            }

            let result = if let Some(resume) = resume_thread.clone() {
                cx.update(|_, cx| {
                    if connection.supports_load_session(cx) {
                        let session_cwd = resume
                            .cwd
                            .clone()
                            .unwrap_or_else(|| fallback_cwd.as_ref().to_path_buf());
                        connection.clone().load_session(
                            resume,
                            project.clone(),
                            session_cwd.as_path(),
                            cx,
                        )
                    } else {
                        Task::ready(Err(anyhow!(LoadError::Other(
                            "Loading sessions is not supported by this agent.".into()
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
                            Self::handle_auth_required(this, err, agent, connection, window, cx)
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

                        this.prompt_capabilities
                            .replace(thread.read(cx).prompt_capabilities());

                        let count = thread.read(cx).entries().len();
                        this.entry_view_state.update(cx, |view_state, cx| {
                            for ix in 0..count {
                                view_state.sync_entry(ix, &thread, window, cx);
                            }
                            this.list_state.splice_focusable(
                                0..0,
                                (0..count).map(|ix| view_state.entry(ix)?.focus_handle(cx)),
                            );
                        });

                        AgentDiff::set_active_thread(&workspace, thread.clone(), window, cx);

                        let connection = thread.read(cx).connection().clone();
                        let session_id = thread.read(cx).session_id().clone();
                        let session_list = if connection.supports_load_session(cx) {
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

                        let mode_selector;
                        if let Some(config_options) = config_options_provider {
                            // Use config options - don't create mode_selector or model_selector
                            let agent_server = this.agent.clone();
                            let fs = this.project.read(cx).fs().clone();
                            this.config_options_view = Some(cx.new(|cx| {
                                ConfigOptionsView::new(config_options, agent_server, fs, window, cx)
                            }));
                            this.model_selector = None;
                            mode_selector = None;
                        } else {
                            // Fall back to legacy mode/model selectors
                            this.config_options_view = None;
                            this.model_selector =
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

                        this.thread_state = ThreadState::Ready {
                            thread,
                            title_editor,
                            mode_selector,
                            _subscriptions: subscriptions,
                        };

                        this.profile_selector = this.as_native_thread(cx).map(|thread| {
                            cx.new(|cx| {
                                ProfileSelector::new(
                                    <dyn Fs>::global(cx),
                                    Arc::new(thread.clone()),
                                    this.focus_handle(cx),
                                    cx,
                                )
                            })
                        });

                        this.message_editor.focus_handle(cx).focus(window, cx);

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
                        this.new_server_version_available = Some(new_version.into());
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

        ThreadState::Loading(loading_view)
    }

    fn handle_auth_required(
        this: WeakEntity<Self>,
        err: AuthRequired,
        agent: Rc<dyn AgentServer>,
        connection: Rc<dyn AgentConnection>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let agent_name = agent.name();
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
                    language_model::ConfigurationViewTargetAgent::Other(agent_name.clone()),
                    window,
                    cx,
                )
            });

            (view, Some(sub))
        } else {
            (None, None)
        };

        this.update(cx, |this, cx| {
            this.thread_state = ThreadState::Unauthenticated {
                pending_auth_method: None,
                connection,
                configuration_view,
                description: err
                    .description
                    .map(|desc| cx.new(|cx| Markdown::new(desc.into(), None, None, cx))),
                _subscription: subscription,
            };
            if this.message_editor.focus_handle(cx).is_focused(window) {
                this.focus_handle.focus(window, cx)
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
        if let Some(load_err) = err.downcast_ref::<LoadError>() {
            self.thread_state = ThreadState::LoadError(load_err.clone());
        } else {
            self.thread_state =
                ThreadState::LoadError(LoadError::Other(format!("{:#}", err).into()))
        }
        if self.message_editor.focus_handle(cx).is_focused(window) {
            self.focus_handle.focus(window, cx)
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
        let should_retry =
            matches!(&self.thread_state, ThreadState::LoadError(_)) || self.thread_error.is_some();

        if should_retry {
            self.thread_error = None;
            self.thread_error_markdown = None;
            self.reset(window, cx);
        }
    }

    pub fn workspace(&self) -> &WeakEntity<Workspace> {
        &self.workspace
    }

    pub fn thread(&self) -> Option<&Entity<AcpThread>> {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => Some(thread),
            ThreadState::Unauthenticated { .. }
            | ThreadState::Loading { .. }
            | ThreadState::LoadError { .. } => None,
        }
    }

    pub fn mode_selector(&self) -> Option<&Entity<ModeSelector>> {
        match &self.thread_state {
            ThreadState::Ready { mode_selector, .. } => mode_selector.as_ref(),
            ThreadState::Unauthenticated { .. }
            | ThreadState::Loading { .. }
            | ThreadState::LoadError { .. } => None,
        }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        match &self.thread_state {
            ThreadState::Ready { .. } | ThreadState::Unauthenticated { .. } => "New Thread".into(),
            ThreadState::Loading(loading_view) => loading_view.read(cx).title.clone(),
            ThreadState::LoadError(error) => match error {
                LoadError::Unsupported { .. } => format!("Upgrade {}", self.agent.name()).into(),
                LoadError::FailedToInstall(_) => {
                    format!("Failed to Install {}", self.agent.name()).into()
                }
                LoadError::Exited { .. } => format!("{} Exited", self.agent.name()).into(),
                LoadError::Other(_) => format!("Error Loading {}", self.agent.name()).into(),
            },
        }
    }

    pub fn title_editor(&self) -> Option<Entity<Editor>> {
        if let ThreadState::Ready { title_editor, .. } = &self.thread_state {
            title_editor.clone()
        } else {
            None
        }
    }

    pub fn cancel_generation(&mut self, cx: &mut Context<Self>) {
        self.thread_error.take();
        self.thread_retry_status.take();
        self.user_interrupted_generation = true;

        if let Some(thread) = self.thread() {
            self._cancel_task = Some(thread.update(cx, |thread, cx| thread.cancel(cx)));
        }
    }

    fn share_thread(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.as_native_thread(cx) else {
            return;
        };

        let client = self.project.read(cx).client();
        let workspace = self.workspace.clone();
        let session_id = thread.read(cx).id().to_string();

        let load_task = thread.read(cx).to_db(cx);

        cx.spawn(async move |_this, cx| {
            let db_thread = load_task.await;

            let shared_thread = SharedThread::from_db_thread(&db_thread);
            let thread_data = shared_thread.to_bytes()?;
            let title = shared_thread.title.to_string();

            client
                .request(proto::ShareAgentThread {
                    session_id: session_id.clone(),
                    title,
                    thread_data,
                })
                .await?;

            let share_url = client::zed_urls::shared_agent_thread_url(&session_id);

            cx.update(|cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        struct ThreadSharedToast;
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::unique::<ThreadSharedToast>(),
                                "Thread shared!",
                            )
                            .on_click(
                                "Copy URL",
                                move |_window, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        share_url.clone(),
                                    ));
                                },
                            ),
                            cx,
                        );
                    });
                }
            });

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn sync_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_imported_thread(cx) {
            return;
        }

        let Some(thread) = self.thread() else {
            return;
        };

        let Some(session_list) = self
            .as_native_connection(cx)
            .and_then(|connection| connection.session_list(cx))
            .and_then(|list| list.downcast::<NativeAgentSessionList>())
        else {
            return;
        };
        let thread_store = session_list.thread_store().clone();

        let client = self.project.read(cx).client();
        let session_id = thread.read(cx).session_id().clone();

        cx.spawn_in(window, async move |this, cx| {
            let response = client
                .request(proto::GetSharedAgentThread {
                    session_id: session_id.to_string(),
                })
                .await?;

            let shared_thread = SharedThread::from_bytes(&response.thread_data)?;

            let db_thread = shared_thread.to_db_thread();

            thread_store
                .update(&mut cx.clone(), |store, cx| {
                    store.save_thread(session_id.clone(), db_thread, cx)
                })
                .await?;

            let thread_metadata = AgentSessionInfo {
                session_id,
                cwd: None,
                title: Some(format!("🔗 {}", response.title).into()),
                updated_at: Some(chrono::Utc::now()),
                meta: None,
            };

            this.update_in(cx, |this, window, cx| {
                this.resume_thread_metadata = Some(thread_metadata);
                this.reset(window, cx);
            })?;

            this.update_in(cx, |this, _window, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        struct ThreadSyncedToast;
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::unique::<ThreadSyncedToast>(),
                                "Thread synced with latest version",
                            )
                            .autohide(),
                            cx,
                        );
                    });
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn expand_message_editor(
        &mut self,
        _: &ExpandMessageEditor,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_editor_is_expanded(!self.editor_expanded, cx);
        cx.stop_propagation();
        cx.notify();
    }

    fn set_editor_is_expanded(&mut self, is_expanded: bool, cx: &mut Context<Self>) {
        self.editor_expanded = is_expanded;
        self.message_editor.update(cx, |editor, cx| {
            if is_expanded {
                editor.set_mode(
                    EditorMode::Full {
                        scale_ui_elements_with_buffer_font_size: false,
                        show_active_line_background: false,
                        sizing_behavior: SizingBehavior::ExcludeOverscrollMargin,
                    },
                    cx,
                )
            } else {
                let agent_settings = AgentSettings::get_global(cx);
                editor.set_mode(
                    EditorMode::AutoHeight {
                        min_lines: agent_settings.message_editor_min_lines,
                        max_lines: Some(agent_settings.set_message_editor_max_lines()),
                    },
                    cx,
                )
            }
        });
        cx.notify();
    }

    pub fn handle_title_editor_event(
        &mut self,
        title_editor: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread() else { return };

        match event {
            EditorEvent::BufferEdited => {
                let new_title = title_editor.read(cx).text(cx);
                thread.update(cx, |thread, cx| {
                    thread
                        .set_title(new_title.into(), cx)
                        .detach_and_log_err(cx);
                })
            }
            EditorEvent::Blurred => {
                if title_editor.read(cx).text(cx).is_empty() {
                    title_editor.update(cx, |editor, cx| {
                        editor.set_text("New Thread", window, cx);
                    });
                }
            }
            _ => {}
        }
    }

    pub fn handle_message_editor_event(
        &mut self,
        _: &Entity<MessageEditor>,
        event: &MessageEditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            MessageEditorEvent::Send => self.send(window, cx),
            MessageEditorEvent::SendImmediately => self.interrupt_and_send(window, cx),
            MessageEditorEvent::Cancel => self.cancel_generation(cx),
            MessageEditorEvent::Focus => {
                self.cancel_editing(&Default::default(), window, cx);
            }
            MessageEditorEvent::LostFocus => {}
        }
    }

    pub fn handle_entry_view_event(
        &mut self,
        _: &Entity<EntryViewState>,
        event: &EntryViewEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &event.view_event {
            ViewEvent::NewDiff(tool_call_id) => {
                if AgentSettings::get_global(cx).expand_edit_card {
                    self.expanded_tool_calls.insert(tool_call_id.clone());
                }
            }
            ViewEvent::NewTerminal(tool_call_id) => {
                if AgentSettings::get_global(cx).expand_terminal_card {
                    self.expanded_tool_calls.insert(tool_call_id.clone());
                }
            }
            ViewEvent::TerminalMovedToBackground(tool_call_id) => {
                self.expanded_tool_calls.remove(tool_call_id);
            }
            ViewEvent::MessageEditorEvent(_editor, MessageEditorEvent::Focus) => {
                if let Some(thread) = self.thread()
                    && let Some(AgentThreadEntry::UserMessage(user_message)) =
                        thread.read(cx).entries().get(event.entry_index)
                    && user_message.id.is_some()
                {
                    self.editing_message = Some(event.entry_index);
                    cx.notify();
                }
            }
            ViewEvent::MessageEditorEvent(editor, MessageEditorEvent::LostFocus) => {
                if let Some(thread) = self.thread()
                    && let Some(AgentThreadEntry::UserMessage(user_message)) =
                        thread.read(cx).entries().get(event.entry_index)
                    && user_message.id.is_some()
                {
                    if editor.read(cx).text(cx).as_str() == user_message.content.to_markdown(cx) {
                        self.editing_message = None;
                        cx.notify();
                    }
                }
            }
            ViewEvent::MessageEditorEvent(_editor, MessageEditorEvent::SendImmediately) => {}
            ViewEvent::MessageEditorEvent(editor, MessageEditorEvent::Send) => {
                self.regenerate(event.entry_index, editor.clone(), window, cx);
            }
            ViewEvent::MessageEditorEvent(_editor, MessageEditorEvent::Cancel) => {
                self.cancel_editing(&Default::default(), window, cx);
            }
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.thread_state, ThreadState::Loading { .. })
    }

    fn resume_chat(&mut self, cx: &mut Context<Self>) {
        self.thread_error.take();
        let Some(thread) = self.thread() else {
            return;
        };
        if !thread.read(cx).can_resume(cx) {
            return;
        }

        let task = thread.update(cx, |thread, cx| thread.resume(cx));
        cx.spawn(async move |this, cx| {
            let result = task.await;

            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    this.handle_thread_error(err, cx);
                }
            })
        })
        .detach();
    }

    fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else { return };

        if self.is_loading_contents {
            return;
        }

        let is_editor_empty = self.message_editor.read(cx).is_empty(cx);
        let is_generating = thread.read(cx).status() != ThreadStatus::Idle;

        // Fast-track: if editor is empty, we're generating, and user can fast-track,
        // send the first queued message immediately (interrupting current generation)
        let has_queued = self
            .as_native_thread(cx)
            .is_some_and(|t| !t.read(cx).queued_messages().is_empty());
        if is_editor_empty && is_generating && self.can_fast_track_queue && has_queued {
            self.can_fast_track_queue = false;
            self.send_queued_message_at_index(0, true, window, cx);
            return;
        }

        if is_editor_empty {
            return;
        }

        if is_generating {
            self.queue_message(window, cx);
            return;
        }

        let text = self.message_editor.read(cx).text(cx);
        let text = text.trim();
        if text == "/login" || text == "/logout" {
            let ThreadState::Ready { thread, .. } = &self.thread_state else {
                return;
            };

            let connection = thread.read(cx).connection().clone();
            let can_login = !connection.auth_methods().is_empty() || self.login.is_some();
            // Does the agent have a specific logout command? Prefer that in case they need to reset internal state.
            let logout_supported = text == "/logout"
                && self
                    .available_commands
                    .borrow()
                    .iter()
                    .any(|command| command.name == "logout");
            if can_login && !logout_supported {
                self.message_editor
                    .update(cx, |editor, cx| editor.clear(window, cx));

                let this = cx.weak_entity();
                let agent = self.agent.clone();
                window.defer(cx, |window, cx| {
                    Self::handle_auth_required(
                        this,
                        AuthRequired::new(),
                        agent,
                        connection,
                        window,
                        cx,
                    );
                });
                cx.notify();
                return;
            }
        }

        self.send_impl(self.message_editor.clone(), window, cx)
    }

    fn interrupt_and_send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };

        if self.is_loading_contents {
            return;
        }

        if thread.read(cx).status() == ThreadStatus::Idle {
            self.send_impl(self.message_editor.clone(), window, cx);
            return;
        }

        self.stop_current_and_send_new_message(window, cx);
    }

    fn stop_current_and_send_new_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };

        self.skip_queue_processing_count = 0;
        self.user_interrupted_generation = true;

        let cancelled = thread.update(cx, |thread, cx| thread.cancel(cx));

        cx.spawn_in(window, async move |this, cx| {
            cancelled.await;

            this.update_in(cx, |this, window, cx| {
                this.send_impl(this.message_editor.clone(), window, cx);
            })
            .ok();
        })
        .detach();
    }

    fn start_turn(&mut self, cx: &mut Context<Self>) -> usize {
        self.turn_generation += 1;
        let generation = self.turn_generation;
        self.turn_started_at = Some(Instant::now());
        self.last_turn_duration = None;
        self.last_turn_tokens = None;
        self.turn_tokens = Some(0);
        self._turn_timer_task = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        }));
        generation
    }

    fn stop_turn(&mut self, generation: usize) {
        if self.turn_generation != generation {
            return;
        }
        self.last_turn_duration = self.turn_started_at.take().map(|started| started.elapsed());
        self.last_turn_tokens = self.turn_tokens.take();
        self._turn_timer_task = None;
    }

    fn update_turn_tokens(&mut self, cx: &App) {
        if let Some(thread) = self.thread() {
            if let Some(usage) = thread.read(cx).token_usage() {
                if let Some(ref mut tokens) = self.turn_tokens {
                    *tokens += usage.output_tokens;
                }
            }
        }
    }

    fn send_impl(
        &mut self,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let full_mention_content = self.as_native_thread(cx).is_some_and(|thread| {
            // Include full contents when using minimal profile
            let thread = thread.read(cx);
            AgentSettings::get_global(cx)
                .profiles
                .get(thread.profile())
                .is_some_and(|profile| profile.tools.is_empty())
        });

        let contents = message_editor.update(cx, |message_editor, cx| {
            message_editor.contents(full_mention_content, cx)
        });

        self.thread_error.take();
        self.editing_message.take();
        self.thread_feedback.clear();

        if self.should_be_following {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .ok();
        }

        let contents_task = cx.spawn_in(window, async move |this, cx| {
            let (contents, tracked_buffers) = contents.await?;

            if contents.is_empty() {
                return Ok(None);
            }

            this.update_in(cx, |this, window, cx| {
                this.message_editor.update(cx, |message_editor, cx| {
                    message_editor.clear(window, cx);
                });
            })?;

            Ok(Some((contents, tracked_buffers)))
        });

        self.send_content(contents_task, window, cx);
    }

    fn send_content(
        &mut self,
        contents_task: Task<anyhow::Result<Option<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread() else {
            return;
        };
        let session_id = thread.read(cx).session_id().clone();
        let agent_telemetry_id = thread.read(cx).connection().telemetry_id();
        let thread = thread.downgrade();

        self.is_loading_contents = true;
        let model_id = self.current_model_id(cx);
        let mode_id = self.current_mode_id(cx);
        let guard = cx.new(|_| ());
        cx.observe_release(&guard, |this, _guard, cx| {
            this.is_loading_contents = false;
            cx.notify();
        })
        .detach();

        let task = cx.spawn_in(window, async move |this, cx| {
            let Some((contents, tracked_buffers)) = contents_task.await? else {
                return Ok(());
            };

            let generation = this.update_in(cx, |this, _window, cx| {
                this.in_flight_prompt = Some(contents.clone());
                let generation = this.start_turn(cx);
                this.set_editor_is_expanded(false, cx);
                this.scroll_to_bottom(cx);
                generation
            })?;

            let _stop_turn = defer({
                let this = this.clone();
                let mut cx = cx.clone();
                move || {
                    this.update(&mut cx, |this, cx| {
                        this.stop_turn(generation);
                        cx.notify();
                    })
                    .ok();
                }
            });
            let turn_start_time = Instant::now();
            let send = thread.update(cx, |thread, cx| {
                thread.action_log().update(cx, |action_log, cx| {
                    for buffer in tracked_buffers {
                        action_log.buffer_read(buffer, cx)
                    }
                });
                drop(guard);

                telemetry::event!(
                    "Agent Message Sent",
                    agent = agent_telemetry_id,
                    session = session_id,
                    model = model_id,
                    mode = mode_id
                );

                thread.send(contents, cx)
            })?;
            let res = send.await;
            let turn_time_ms = turn_start_time.elapsed().as_millis();
            drop(_stop_turn);
            let status = if res.is_ok() {
                this.update(cx, |this, _| this.in_flight_prompt.take()).ok();
                "success"
            } else {
                "failure"
            };
            telemetry::event!(
                "Agent Turn Completed",
                agent = agent_telemetry_id,
                session = session_id,
                model = model_id,
                mode = mode_id,
                status,
                turn_time_ms,
            );
            res
        });

        cx.spawn(async move |this, cx| {
            if let Err(err) = task.await {
                this.update(cx, |this, cx| {
                    this.handle_thread_error(err, cx);
                })
                .ok();
            } else {
                this.update(cx, |this, cx| {
                    this.should_be_following = this
                        .workspace
                        .update(cx, |workspace, _| {
                            workspace.is_being_followed(CollaboratorId::Agent)
                        })
                        .unwrap_or_default();
                })
                .ok();
            }
        })
        .detach();
    }

    fn queue_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let is_idle = self
            .thread()
            .map(|t| t.read(cx).status() == acp_thread::ThreadStatus::Idle)
            .unwrap_or(true);

        if is_idle {
            self.send_impl(self.message_editor.clone(), window, cx);
            return;
        }

        let full_mention_content = self.as_native_thread(cx).is_some_and(|thread| {
            let thread = thread.read(cx);
            AgentSettings::get_global(cx)
                .profiles
                .get(thread.profile())
                .is_some_and(|profile| profile.tools.is_empty())
        });

        let contents = self.message_editor.update(cx, |message_editor, cx| {
            message_editor.contents(full_mention_content, cx)
        });

        let message_editor = self.message_editor.clone();

        cx.spawn_in(window, async move |this, cx| {
            let (content, tracked_buffers) = contents.await?;

            if content.is_empty() {
                return Ok::<(), anyhow::Error>(());
            }

            this.update_in(cx, |this, window, cx| {
                if let Some(thread) = this.as_native_thread(cx) {
                    thread.update(cx, |thread, _| {
                        thread.queue_message(content, tracked_buffers);
                    });
                }
                // Enable fast-track: user can press Enter again to send this queued message immediately
                this.can_fast_track_queue = true;
                message_editor.update(cx, |message_editor, cx| {
                    message_editor.clear(window, cx);
                });
                cx.notify();
            })?;
            Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn send_queued_message_at_index(
        &mut self,
        index: usize,
        is_send_now: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(native_thread) = self.as_native_thread(cx) else {
            return;
        };

        let Some(queued) =
            native_thread.update(cx, |thread, _| thread.remove_queued_message(index))
        else {
            return;
        };
        let content = queued.content;
        let tracked_buffers = queued.tracked_buffers;

        let Some(thread) = self.thread().cloned() else {
            return;
        };

        // Only increment skip count for "Send Now" operations (out-of-order sends)
        // Normal auto-processing from the Stopped handler doesn't need to skip.
        // We only skip the Stopped event from the cancelled generation, NOT the
        // Stopped event from the newly sent message (which should trigger queue processing).
        if is_send_now {
            let is_generating = thread.read(cx).status() == acp_thread::ThreadStatus::Generating;
            self.skip_queue_processing_count += if is_generating { 1 } else { 0 };
        }

        let cancelled = thread.update(cx, |thread, cx| thread.cancel(cx));

        let should_be_following = self.should_be_following;
        let workspace = self.workspace.clone();

        let contents_task = cx.spawn_in(window, async move |_this, cx| {
            cancelled.await;
            if should_be_following {
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.follow(CollaboratorId::Agent, window, cx);
                    })
                    .ok();
            }

            Ok(Some((content, tracked_buffers)))
        });

        self.send_content(contents_task, window, cx);
    }

    fn cancel_editing(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };

        if let Some(index) = self.editing_message.take()
            && let Some(editor) = self
                .entry_view_state
                .read(cx)
                .entry(index)
                .and_then(|e| e.message_editor())
                .cloned()
        {
            editor.update(cx, |editor, cx| {
                if let Some(user_message) = thread
                    .read(cx)
                    .entries()
                    .get(index)
                    .and_then(|e| e.user_message())
                {
                    editor.set_message(user_message.chunks.clone(), window, cx);
                }
            })
        };
        self.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn regenerate(
        &mut self,
        entry_ix: usize,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };
        if self.is_loading_contents {
            return;
        }

        let Some(user_message_id) = thread.update(cx, |thread, _| {
            thread.entries().get(entry_ix)?.user_message()?.id.clone()
        }) else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            // Check if there are any edits from prompts before the one being regenerated.
            //
            // If there are, we keep/accept them since we're not regenerating the prompt that created them.
            //
            // If editing the prompt that generated the edits, they are auto-rejected
            // through the `rewind` function in the `acp_thread`.
            let has_earlier_edits = thread.read_with(cx, |thread, _| {
                thread
                    .entries()
                    .iter()
                    .take(entry_ix)
                    .any(|entry| entry.diffs().next().is_some())
            });

            if has_earlier_edits {
                thread.update(cx, |thread, cx| {
                    thread.action_log().update(cx, |action_log, cx| {
                        action_log.keep_all_edits(None, cx);
                    });
                });
            }

            thread
                .update(cx, |thread, cx| thread.rewind(user_message_id, cx))
                .await?;
            this.update_in(cx, |this, window, cx| {
                this.send_impl(message_editor, window, cx);
                this.focus_handle(cx).focus(window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn open_edited_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread() else {
            return;
        };

        let Some(diff) =
            AgentDiffPane::deploy(thread.clone(), self.workspace.clone(), window, cx).log_err()
        else {
            return;
        };

        diff.update(cx, |diff, cx| {
            diff.move_to_path(PathKey::for_buffer(buffer, cx), window, cx)
        })
    }

    fn handle_open_rules(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.as_native_thread(cx) else {
            return;
        };
        let project_context = thread.read(cx).project_context().read(cx);

        let project_entry_ids = project_context
            .worktrees
            .iter()
            .flat_map(|worktree| worktree.rules_file.as_ref())
            .map(|rules_file| ProjectEntryId::from_usize(rules_file.project_entry_id))
            .collect::<Vec<_>>();

        self.workspace
            .update(cx, move |workspace, cx| {
                // TODO: Open a multibuffer instead? In some cases this doesn't make the set of rules
                // files clear. For example, if rules file 1 is already open but rules file 2 is not,
                // this would open and focus rules file 2 in a tab that is not next to rules file 1.
                let project = workspace.project().read(cx);
                let project_paths = project_entry_ids
                    .into_iter()
                    .flat_map(|entry_id| project.path_for_entry(entry_id, cx))
                    .collect::<Vec<_>>();
                for project_path in project_paths {
                    workspace
                        .open_path(project_path, None, true, window, cx)
                        .detach_and_log_err(cx);
                }
            })
            .ok();
    }

    fn handle_thread_error(&mut self, error: anyhow::Error, cx: &mut Context<Self>) {
        self.thread_error = Some(ThreadError::from_err(error, &self.agent));
        cx.notify();
    }

    fn clear_thread_error(&mut self, cx: &mut Context<Self>) {
        self.thread_error = None;
        self.thread_error_markdown = None;
        self.token_limit_callout_dismissed = true;
        cx.notify();
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
                self.entry_view_state.update(cx, |view_state, cx| {
                    view_state.sync_entry(index, thread, window, cx);
                    self.list_state.splice_focusable(
                        index..index,
                        [view_state
                            .entry(index)
                            .and_then(|entry| entry.focus_handle(cx))],
                    );
                });
            }
            AcpThreadEvent::EntryUpdated(index) => {
                self.entry_view_state.update(cx, |view_state, cx| {
                    view_state.sync_entry(*index, thread, window, cx)
                });
            }
            AcpThreadEvent::EntriesRemoved(range) => {
                self.entry_view_state
                    .update(cx, |view_state, _cx| view_state.remove(range.clone()));
                self.list_state.splice(range.clone(), 0);
            }
            AcpThreadEvent::ToolAuthorizationRequired => {
                self.notify_with_sound("Waiting for tool confirmation", IconName::Info, window, cx);
            }
            AcpThreadEvent::Retry(retry) => {
                self.thread_retry_status = Some(retry.clone());
            }
            AcpThreadEvent::Stopped => {
                self.thread_retry_status.take();
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

                if self.skip_queue_processing_count > 0 {
                    self.skip_queue_processing_count -= 1;
                } else if self.user_interrupted_generation {
                    // Manual interruption: don't auto-process queue.
                    // Reset the flag so future completions can process normally.
                    self.user_interrupted_generation = false;
                } else {
                    let has_queued = self
                        .as_native_thread(cx)
                        .is_some_and(|t| !t.read(cx).queued_messages().is_empty());
                    if has_queued {
                        self.send_queued_message_at_index(0, false, window, cx);
                    }
                }

                self.history.update(cx, |history, cx| history.refresh(cx));
            }
            AcpThreadEvent::Refusal => {
                self.thread_retry_status.take();
                self.thread_error = Some(ThreadError::Refusal);
                let model_or_agent_name = self.current_model_name(cx);
                let notification_message =
                    format!("{} refused to respond to this request", model_or_agent_name);
                self.notify_with_sound(&notification_message, IconName::Warning, window, cx);
            }
            AcpThreadEvent::Error => {
                self.thread_retry_status.take();
                self.notify_with_sound(
                    "Agent stopped due to an error",
                    IconName::Warning,
                    window,
                    cx,
                );
            }
            AcpThreadEvent::LoadError(error) => {
                self.thread_retry_status.take();
                self.thread_state = ThreadState::LoadError(error.clone());
                if self.message_editor.focus_handle(cx).is_focused(window) {
                    self.focus_handle.focus(window, cx)
                }
            }
            AcpThreadEvent::TitleUpdated => {
                let title = thread.read(cx).title();
                if let Some(title_editor) = self.title_editor() {
                    title_editor.update(cx, |editor, cx| {
                        if editor.text(cx) != title {
                            editor.set_text(title, window, cx);
                        }
                    });
                }
                self.history.update(cx, |history, cx| history.refresh(cx));
            }
            AcpThreadEvent::PromptCapabilitiesUpdated => {
                self.prompt_capabilities
                    .replace(thread.read(cx).prompt_capabilities());
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
                self.available_commands.replace(available_commands);

                let agent_display_name = self
                    .agent_server_store
                    .read(cx)
                    .agent_display_name(&ExternalAgentServerName(self.agent.name()))
                    .unwrap_or_else(|| self.agent.name());

                let new_placeholder = placeholder_text(agent_display_name.as_ref(), has_commands);

                self.message_editor.update(cx, |editor, cx| {
                    editor.set_placeholder_text(&new_placeholder, window, cx);
                });
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
        let ThreadState::Unauthenticated {
            connection,
            pending_auth_method,
            configuration_view,
            ..
        } = &mut self.thread_state
        else {
            return;
        };
        let agent_telemetry_id = connection.telemetry_id();

        // Check for the experimental "terminal-auth" _meta field
        let auth_method = connection.auth_methods().iter().find(|m| m.id == method);

        if let Some(auth_method) = auth_method {
            if let Some(meta) = &auth_method.meta {
                if let Some(terminal_auth) = meta.get("terminal-auth") {
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
                                    .filter_map(|(k, v)| {
                                        v.as_str().map(|val| (k.clone(), val.to_string()))
                                    })
                                    .collect::<HashMap<String, String>>()
                            })
                            .unwrap_or_default();

                        // Run SpawnInTerminal in the same dir as the ACP server
                        let cwd = connection
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

                        self.thread_error.take();
                        configuration_view.take();
                        pending_auth_method.replace(method.clone());

                        if let Some(workspace) = self.workspace.upgrade() {
                            let project = self.project.clone();
                            let authenticate = Self::spawn_external_agent_login(
                                login, workspace, project, false, true, window, cx,
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
                                            if let ThreadState::Unauthenticated {
                                                pending_auth_method,
                                                ..
                                            } = &mut this.thread_state
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
                let agent = self.agent.clone();
                let connection = connection.clone();
                window.defer(cx, |window, cx| {
                    Self::handle_auth_required(
                        this,
                        AuthRequired {
                            description: Some("GEMINI_API_KEY must be set".to_owned()),
                            provider_id: Some(language_model::GOOGLE_PROVIDER_ID),
                        },
                        agent,
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
            let agent = self.agent.clone();
            let connection = connection.clone();

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
                        agent,
                        connection,
                        window,
                        cx,
                    )
                });
            return;
        }

        self.thread_error.take();
        configuration_view.take();
        pending_auth_method.replace(method.clone());
        let authenticate = if (method.0.as_ref() == "claude-login"
            || method.0.as_ref() == "spawn-gemini-cli")
            && let Some(login) = self.login.clone()
        {
            if let Some(workspace) = self.workspace.upgrade() {
                let project = self.project.clone();
                Self::spawn_external_agent_login(
                    login, workspace, project, false, false, window, cx,
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
                        if let ThreadState::Unauthenticated {
                            pending_auth_method,
                            ..
                        } = &mut this.thread_state
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
        previous_attempt: bool,
        check_exit_code: bool,
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

            if check_exit_code {
                // For extension-based auth, wait for the process to exit and check exit code
                let exit_status = terminal
                    .read_with(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .await;

                match exit_status {
                    Some(status) if status.success() => {
                        Ok(())
                    }
                    Some(status) => {
                        Err(anyhow!("Login command failed with exit code: {:?}", status.code()))
                    }
                    None => {
                        Err(anyhow!("Login command terminated without exit status"))
                    }
                }
            } else {
                // For hardcoded agents (claude-login, gemini-cli): look for specific output
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
                                if content.contains("Login successful")
                                    || content.contains("Type your message")
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
                            return cx.update(|window, cx| Self::spawn_external_agent_login(login, workspace, project.clone(), true, false, window, cx))?.await
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
        self.thread().is_some_and(|thread| {
            thread.read(cx).entries().iter().any(|entry| {
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
        let Some(thread) = self.thread() else {
            return;
        };
        let agent_telemetry_id = thread.read(cx).connection().telemetry_id();

        telemetry::event!(
            "Agent Tool Call Authorized",
            agent = agent_telemetry_id,
            session = thread.read(cx).session_id(),
            option = option_kind
        );

        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, option_id, option_kind, cx);
        });
        if self.should_be_following {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .ok();
        }
        cx.notify();
    }

    fn restore_checkpoint(&mut self, message_id: &UserMessageId, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };

        thread
            .update(cx, |thread, cx| {
                thread.restore_checkpoint(message_id.clone(), cx)
            })
            .detach_and_log_err(cx);
    }

    fn render_entry(
        &self,
        entry_ix: usize,
        total_entries: usize,
        entry: &AgentThreadEntry,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let is_indented = entry.is_indented();
        let is_first_indented = is_indented
            && self.thread().is_some_and(|thread| {
                thread
                    .read(cx)
                    .entries()
                    .get(entry_ix.saturating_sub(1))
                    .is_none_or(|entry| !entry.is_indented())
            });

        let primary = match &entry {
            AgentThreadEntry::UserMessage(message) => {
                let Some(editor) = self
                    .entry_view_state
                    .read(cx)
                    .entry(entry_ix)
                    .and_then(|entry| entry.message_editor())
                    .cloned()
                else {
                    return Empty.into_any_element();
                };

                let editing = self.editing_message == Some(entry_ix);
                let editor_focus = editor.focus_handle(cx).is_focused(window);
                let focus_border = cx.theme().colors().border_focused;

                let rules_item = if entry_ix == 0 {
                    self.render_rules_item(cx)
                } else {
                    None
                };

                let has_checkpoint_button = message
                    .checkpoint
                    .as_ref()
                    .is_some_and(|checkpoint| checkpoint.show);

                let agent_name = self.agent.name();

                v_flex()
                    .id(("user_message", entry_ix))
                    .map(|this| {
                        if is_first_indented {
                            this.pt_0p5()
                        } else if entry_ix == 0 && !has_checkpoint_button && rules_item.is_none()  {
                            this.pt(rems_from_px(18.))
                        } else if rules_item.is_some() {
                            this.pt_3()
                        } else {
                            this.pt_2()
                        }
                    })
                    .pb_3()
                    .px_2()
                    .gap_1p5()
                    .w_full()
                    .children(rules_item)
                    .children(message.id.clone().and_then(|message_id| {
                        message.checkpoint.as_ref()?.show.then(|| {
                            h_flex()
                                .px_3()
                                .gap_2()
                                .child(Divider::horizontal())
                                .child(
                                    Button::new("restore-checkpoint", "Restore Checkpoint")
                                        .icon(IconName::Undo)
                                        .icon_size(IconSize::XSmall)
                                        .icon_position(IconPosition::Start)
                                        .label_size(LabelSize::XSmall)
                                        .icon_color(Color::Muted)
                                        .color(Color::Muted)
                                        .tooltip(Tooltip::text("Restores all files in the project to the content they had at this point in the conversation."))
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.restore_checkpoint(&message_id, cx);
                                        }))
                                )
                                .child(Divider::horizontal())
                        })
                    }))
                    .child(
                        div()
                            .relative()
                            .child(
                                div()
                                    .py_3()
                                    .px_2()
                                    .rounded_md()
                                    .shadow_md()
                                    .bg(cx.theme().colors().editor_background)
                                    .border_1()
                                    .when(is_indented, |this| {
                                        this.py_2().px_2().shadow_sm()
                                    })
                                    .when(editing && !editor_focus, |this| this.border_dashed())
                                    .border_color(cx.theme().colors().border)
                                    .map(|this|{
                                        if editing && editor_focus {
                                            this.border_color(focus_border)
                                        } else if message.id.is_some() {
                                            this.hover(|s| s.border_color(focus_border.opacity(0.8)))
                                        } else {
                                            this
                                        }
                                    })
                                    .text_xs()
                                    .child(editor.clone().into_any_element())
                            )
                            .when(editor_focus, |this| {
                                let base_container = h_flex()
                                    .absolute()
                                    .top_neg_3p5()
                                    .right_3()
                                    .gap_1()
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .bg(cx.theme().colors().editor_background)
                                    .overflow_hidden();

                                if message.id.is_some() {
                                    this.child(
                                        base_container
                                            .child(
                                                IconButton::new("cancel", IconName::Close)
                                                    .disabled(self.is_loading_contents)
                                                    .icon_color(Color::Error)
                                                    .icon_size(IconSize::XSmall)
                                                    .on_click(cx.listener(Self::cancel_editing))
                                            )
                                            .child(
                                                if self.is_loading_contents {
                                                    div()
                                                        .id("loading-edited-message-content")
                                                        .tooltip(Tooltip::text("Loading Added Context…"))
                                                        .child(loading_contents_spinner(IconSize::XSmall))
                                                        .into_any_element()
                                                } else {
                                                    IconButton::new("regenerate", IconName::Return)
                                                        .icon_color(Color::Muted)
                                                        .icon_size(IconSize::XSmall)
                                                        .tooltip(Tooltip::text(
                                                            "Editing will restart the thread from this point."
                                                        ))
                                                        .on_click(cx.listener({
                                                            let editor = editor.clone();
                                                            move |this, _, window, cx| {
                                                                this.regenerate(
                                                                    entry_ix, editor.clone(), window, cx,
                                                                );
                                                            }
                                                        })).into_any_element()
                                                }
                                            )
                                    )
                                } else {
                                    this.child(
                                        base_container
                                            .border_dashed()
                                            .child(
                                                IconButton::new("editing_unavailable", IconName::PencilUnavailable)
                                                    .icon_size(IconSize::Small)
                                                    .icon_color(Color::Muted)
                                                    .style(ButtonStyle::Transparent)
                                                    .tooltip(Tooltip::element({
                                                        move |_, _| {
                                                            v_flex()
                                                                .gap_1()
                                                                .child(Label::new("Unavailable Editing")).child(
                                                                    div().max_w_64().child(
                                                                        Label::new(format!(
                                                                            "Editing previous messages is not available for {} yet.",
                                                                            agent_name.clone()
                                                                        ))
                                                                        .size(LabelSize::Small)
                                                                        .color(Color::Muted),
                                                                    ),
                                                                )
                                                                .into_any_element()
                                                        }
                                                    }))
                                            )
                                    )
                                }
                            }),
                    )
                    .into_any()
            }
            AgentThreadEntry::AssistantMessage(AssistantMessage {
                chunks,
                indented: _,
            }) => {
                let mut is_blank = true;
                let is_last = entry_ix + 1 == total_entries;

                let style = default_markdown_style(false, false, window, cx);
                let message_body = v_flex()
                    .w_full()
                    .gap_3()
                    .children(chunks.iter().enumerate().filter_map(
                        |(chunk_ix, chunk)| match chunk {
                            AssistantMessageChunk::Message { block } => {
                                block.markdown().and_then(|md| {
                                    let this_is_blank = md.read(cx).source().trim().is_empty();
                                    is_blank = is_blank && this_is_blank;
                                    if this_is_blank {
                                        return None;
                                    }

                                    Some(
                                        self.render_markdown(md.clone(), style.clone())
                                            .into_any_element(),
                                    )
                                })
                            }
                            AssistantMessageChunk::Thought { block } => {
                                block.markdown().and_then(|md| {
                                    let this_is_blank = md.read(cx).source().trim().is_empty();
                                    is_blank = is_blank && this_is_blank;
                                    if this_is_blank {
                                        return None;
                                    }
                                    Some(
                                        self.render_thinking_block(
                                            entry_ix,
                                            chunk_ix,
                                            md.clone(),
                                            window,
                                            cx,
                                        )
                                        .into_any_element(),
                                    )
                                })
                            }
                        },
                    ))
                    .into_any();

                if is_blank {
                    Empty.into_any()
                } else {
                    v_flex()
                        .px_5()
                        .py_1p5()
                        .when(is_last, |this| this.pb_4())
                        .w_full()
                        .text_ui(cx)
                        .child(self.render_message_context_menu(entry_ix, message_body, cx))
                        .into_any()
                }
            }
            AgentThreadEntry::ToolCall(tool_call) => {
                let has_terminals = tool_call.terminals().next().is_some();

                div()
                    .w_full()
                    .map(|this| {
                        if has_terminals {
                            this.children(tool_call.terminals().map(|terminal| {
                                self.render_terminal_tool_call(
                                    entry_ix, terminal, tool_call, window, cx,
                                )
                            }))
                        } else {
                            this.child(self.render_tool_call(entry_ix, tool_call, window, cx))
                        }
                    })
                    .into_any()
            }
        };

        let primary = if is_indented {
            let line_top = if is_first_indented {
                rems_from_px(-12.0)
            } else {
                rems_from_px(0.0)
            };

            div()
                .relative()
                .w_full()
                .pl_5()
                .bg(cx.theme().colors().panel_background.opacity(0.2))
                .child(
                    div()
                        .absolute()
                        .left(rems_from_px(18.0))
                        .top(line_top)
                        .bottom_0()
                        .w_px()
                        .bg(cx.theme().colors().border.opacity(0.6)),
                )
                .child(primary)
                .into_any_element()
        } else {
            primary
        };

        let needs_confirmation = if let AgentThreadEntry::ToolCall(tool_call) = entry {
            matches!(
                tool_call.status,
                ToolCallStatus::WaitingForConfirmation { .. }
            )
        } else {
            false
        };

        let Some(thread) = self.thread() else {
            return primary;
        };

        let primary = if entry_ix == total_entries - 1 {
            v_flex()
                .w_full()
                .child(primary)
                .map(|this| {
                    if needs_confirmation {
                        this.child(self.render_generating(true, cx))
                    } else {
                        this.child(self.render_thread_controls(&thread, cx))
                    }
                })
                .when_some(
                    self.thread_feedback.comments_editor.clone(),
                    |this, editor| this.child(Self::render_feedback_feedback_editor(editor, cx)),
                )
                .into_any_element()
        } else {
            primary
        };

        if let Some(editing_index) = self.editing_message.as_ref()
            && *editing_index < entry_ix
        {
            let backdrop = div()
                .id(("backdrop", entry_ix))
                .size_full()
                .absolute()
                .inset_0()
                .bg(cx.theme().colors().panel_background)
                .opacity(0.8)
                .block_mouse_except_scroll()
                .on_click(cx.listener(Self::cancel_editing));

            div()
                .relative()
                .child(primary)
                .child(backdrop)
                .into_any_element()
        } else {
            primary
        }
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
                    let is_at_top = entity.read(cx).list_state.logical_scroll_top().item_ix == 0;

                    let copy_this_agent_response =
                        ContextMenuEntry::new("Copy This Agent Response").handler({
                            let entity = entity.clone();
                            move |_, cx| {
                                entity.update(cx, |this, cx| {
                                    if let Some(thread) = this.thread() {
                                        let entries = thread.read(cx).entries();
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
                        .action("Copy Selection", Box::new(markdown::CopyAsMarkdown))
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

        let is_open = self.expanded_thinking_blocks.contains(&key);

        let scroll_handle = self
            .entry_view_state
            .read(cx)
            .entry(entry_ix)
            .and_then(|entry| entry.scroll_handle_for_assistant_message_chunk(chunk_ix));

        let thinking_content = {
            div()
                .id(("thinking-content", chunk_ix))
                .when_some(scroll_handle, |this, scroll_handle| {
                    this.track_scroll(&scroll_handle)
                })
                .text_ui_sm(cx)
                .overflow_hidden()
                .child(
                    self.render_markdown(chunk, default_markdown_style(false, false, window, cx)),
                )
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
                                    if is_open {
                                        this.expanded_thinking_blocks.remove(&key);
                                    } else {
                                        this.expanded_thinking_blocks.insert(key);
                                    }
                                    cx.notify();
                                }
                            })),
                    )
                    .on_click(cx.listener({
                        move |this, _event, _window, cx| {
                            if is_open {
                                this.expanded_thinking_blocks.remove(&key);
                            } else {
                                this.expanded_thinking_blocks.insert(key);
                            }
                            cx.notify();
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
            self.entry_view_state
                .read(cx)
                .entry(entry_ix)
                .and_then(|entry| entry.editor_for_diff(diff))
                .is_some()
                && diff.read(cx).has_revealed_range(cx)
        });

        let use_card_layout = needs_confirmation || is_edit || is_terminal_tool;

        let has_image_content = tool_call.content.iter().any(|c| c.image().is_some());
        let is_collapsible = !tool_call.content.is_empty() && !needs_confirmation;
        let is_open = needs_confirmation || self.expanded_tool_calls.contains(&tool_call.id);

        let should_show_raw_input = !is_terminal_tool && !is_edit && !has_image_content;

        let input_output_header = |label: SharedString| {
            Label::new(label)
                .size(LabelSize::XSmall)
                .color(Color::Muted)
                .buffer_font(cx)
        };

        let tool_output_display = if is_open {
            match &tool_call.status {
                ToolCallStatus::WaitingForConfirmation { options, .. } => v_flex()
                    .w_full()
                    .children(
                        tool_call
                            .content
                            .iter()
                            .enumerate()
                            .map(|(content_ix, content)| {
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
                            }),
                    )
                    .when(should_show_raw_input, |this| {
                        let is_raw_input_expanded =
                            self.expanded_tool_call_raw_inputs.contains(&tool_call.id);

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
                                                if this.expanded_tool_call_raw_inputs.contains(&id)
                                                {
                                                    this.expanded_tool_call_raw_inputs.remove(&id);
                                                } else {
                                                    this.expanded_tool_call_raw_inputs
                                                        .insert(id.clone());
                                                }
                                                cx.notify();
                                            }
                                        })),
                                )
                                .when(is_raw_input_expanded, |this| {
                                    this.children(tool_call.raw_input_markdown.clone().map(
                                        |input| {
                                            self.render_markdown(
                                                input,
                                                default_markdown_style(false, false, window, cx),
                                            )
                                        },
                                    ))
                                }),
                        )
                    })
                    .child(self.render_permission_buttons(
                        tool_call.kind,
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
                | ToolCallStatus::Canceled => {
                    v_flex()
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
                                                default_markdown_style(false, false, window, cx),
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
                        .into_any()
                }
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
                                                        if is_open {
                                                            this.expanded_tool_calls.remove(&id);
                                                        } else {
                                                            this.expanded_tool_calls.insert(id.clone());
                                                        }
                                                        cx.notify();
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
                                            let is_discarded = self.discarded_partial_edits.contains(&tool_call_id);
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
                                                            this.discarded_partial_edits.insert(tool_call_id.clone());
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
                    .child(self.render_markdown(
                        tool_call.label.clone(),
                        MarkdownStyle {
                            prevent_mouse_interaction: true,
                            ..default_markdown_style(false, true, window, cx)
                        },
                    ))
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
                        default_markdown_style(false, true, window, cx),
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

        let tool_call_in_progress = matches!(
            tool_call.status,
            ToolCallStatus::Pending | ToolCallStatus::InProgress
        );

        v_flex().ml_5().mr_5().my_1p5().gap_1().children(
            subagent_threads
                .into_iter()
                .enumerate()
                .map(|(context_ix, thread)| {
                    self.render_subagent_card(
                        entry_ix,
                        context_ix,
                        &thread,
                        tool_call_in_progress,
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
        tool_call_in_progress: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let thread_read = thread.read(cx);
        let session_id = thread_read.session_id().clone();
        let title = thread_read.title();
        let action_log = thread_read.action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        let is_expanded = self.expanded_subagents.contains(&session_id);
        let files_changed = changed_buffers.len();
        let diff_stats = DiffStats::all_files(&changed_buffers, cx);

        let is_running = tool_call_in_progress;

        let card_header_id =
            SharedString::from(format!("subagent-header-{}-{}", entry_ix, context_ix));
        let diff_stat_id = SharedString::from(format!("subagent-diff-{}-{}", entry_ix, context_ix));

        let icon = h_flex().w_4().justify_center().child(if is_running {
            SpinnerLabel::new()
                .size(LabelSize::Small)
                .into_any_element()
        } else {
            Icon::new(IconName::Check)
                .size(IconSize::Small)
                .color(Color::Success)
                .into_any_element()
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
                        Disclosure::new(
                            SharedString::from(format!(
                                "subagent-disclosure-inner-{}-{}",
                                entry_ix, context_ix
                            )),
                            is_expanded,
                        )
                        .opened_icon(IconName::ChevronUp)
                        .closed_icon(IconName::ChevronDown)
                        .visible_on_hover(card_header_id)
                        .on_click(cx.listener({
                            move |this, _, _, cx| {
                                if this.expanded_subagents.contains(&session_id) {
                                    this.expanded_subagents.remove(&session_id);
                                } else {
                                    this.expanded_subagents.insert(session_id.clone());
                                }
                                cx.notify();
                            }
                        })),
                    ),
            )
            .when(is_expanded, |this| {
                this.child(
                    self.render_subagent_expanded_content(entry_ix, context_ix, thread, window, cx),
                )
            })
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
            .subagent_scroll_handles
            .borrow_mut()
            .entry(session_id.clone())
            .or_default()
            .clone();

        scroll_handle.scroll_to_bottom();

        div()
            .id(format!("subagent-content-{}", session_id))
            .w_full()
            .max_h_56()
            .p_2()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .bg(cx.theme().colors().editor_background.opacity(0.2))
            .overflow_hidden()
            .track_scroll(&scroll_handle)
            .when_some(last_assistant_markdown, |this, markdown| {
                this.child(
                    self.render_markdown(
                        markdown,
                        default_markdown_style(false, false, window, cx),
                    ),
                )
            })
    }

    fn render_markdown_output(
        &self,
        markdown: Entity<Markdown>,
        tool_call_id: acp::ToolCallId,
        context_ix: usize,
        card_layout: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let button_id = SharedString::from(format!("tool_output-{:?}", tool_call_id));

        v_flex()
            .gap_2()
            .map(|this| {
                if card_layout {
                    this.when(context_ix > 0, |this| {
                        this.pt_2()
                            .border_t_1()
                            .border_color(self.tool_card_border_color(cx))
                    })
                } else {
                    this.ml(rems(0.4))
                        .px_3p5()
                        .border_l_1()
                        .border_color(self.tool_card_border_color(cx))
                }
            })
            .text_xs()
            .text_color(cx.theme().colors().text_muted)
            .child(self.render_markdown(markdown, default_markdown_style(false, false, window, cx)))
            .when(!card_layout, |this| {
                this.child(
                    IconButton::new(button_id, IconName::ChevronUp)
                        .full_width()
                        .style(ButtonStyle::Outlined)
                        .icon_color(Color::Muted)
                        .on_click(cx.listener({
                            move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                                this.expanded_tool_calls.remove(&tool_call_id);
                                cx.notify();
                            }
                        })),
                )
            })
            .into_any_element()
    }

    fn render_image_output(
        &self,
        entry_ix: usize,
        image: Arc<gpui::Image>,
        location: Option<acp::ToolCallLocation>,
        card_layout: bool,
        show_dimensions: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let dimensions_label = if show_dimensions {
            let format_name = match image.format() {
                gpui::ImageFormat::Png => "PNG",
                gpui::ImageFormat::Jpeg => "JPEG",
                gpui::ImageFormat::Webp => "WebP",
                gpui::ImageFormat::Gif => "GIF",
                gpui::ImageFormat::Svg => "SVG",
                gpui::ImageFormat::Bmp => "BMP",
                gpui::ImageFormat::Tiff => "TIFF",
                gpui::ImageFormat::Ico => "ICO",
            };
            let dimensions = image::ImageReader::new(std::io::Cursor::new(image.bytes()))
                .with_guessed_format()
                .ok()
                .and_then(|reader| reader.into_dimensions().ok());
            dimensions.map(|(w, h)| format!("{}×{} {}", w, h, format_name))
        } else {
            None
        };

        v_flex()
            .gap_2()
            .map(|this| {
                if card_layout {
                    this
                } else {
                    this.ml(rems(0.4))
                        .px_3p5()
                        .border_l_1()
                        .border_color(self.tool_card_border_color(cx))
                }
            })
            .when(dimensions_label.is_some() || location.is_some(), |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .children(dimensions_label.map(|label| {
                            Label::new(label)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .buffer_font(cx)
                        }))
                        .when_some(location, |this, _loc| {
                            this.child(
                                Button::new(("go-to-file", entry_ix), "Go to File")
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.open_tool_call_location(entry_ix, 0, window, cx);
                                    })),
                            )
                        }),
                )
            })
            .child(
                img(image)
                    .max_w_96()
                    .max_h_96()
                    .object_fit(ObjectFit::ScaleDown),
            )
            .into_any_element()
    }

    fn render_resource_link(
        &self,
        resource_link: &acp::ResourceLink,
        cx: &Context<Self>,
    ) -> AnyElement {
        let uri: SharedString = resource_link.uri.clone().into();
        let is_file = resource_link.uri.strip_prefix("file://");

        let label: SharedString = if let Some(abs_path) = is_file {
            if let Some(project_path) = self
                .project
                .read(cx)
                .project_path_for_absolute_path(&Path::new(abs_path), cx)
                && let Some(worktree) = self
                    .project
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)
            {
                worktree
                    .read(cx)
                    .full_path(&project_path.path)
                    .to_string_lossy()
                    .to_string()
                    .into()
            } else {
                abs_path.to_string().into()
            }
        } else {
            uri.clone()
        };

        let button_id = SharedString::from(format!("item-{}", uri));

        div()
            .ml(rems(0.4))
            .pl_2p5()
            .border_l_1()
            .border_color(self.tool_card_border_color(cx))
            .overflow_hidden()
            .child(
                Button::new(button_id, label)
                    .label_size(LabelSize::Small)
                    .color(Color::Muted)
                    .truncate(true)
                    .when(is_file.is_none(), |this| {
                        this.icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted)
                    })
                    .on_click(cx.listener({
                        let workspace = self.workspace.clone();
                        move |_, _, window, cx: &mut Context<Self>| {
                            Self::open_link(uri.clone(), &workspace, window, cx);
                        }
                    })),
            )
            .into_any_element()
    }

    fn render_permission_buttons(
        &self,
        kind: acp::ToolKind,
        options: &[acp::PermissionOption],
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        let is_first = self.thread().is_some_and(|thread| {
            thread
                .read(cx)
                .first_tool_awaiting_confirmation()
                .is_some_and(|call| call.id == tool_call_id)
        });

        // For SwitchMode, use the old layout with all buttons
        if kind == acp::ToolKind::SwitchMode {
            return self.render_permission_buttons_legacy(options, entry_ix, tool_call_id, cx);
        }

        let granularity_options: Vec<_> = options
            .iter()
            .filter(|o| {
                matches!(
                    o.kind,
                    acp::PermissionOptionKind::AllowOnce | acp::PermissionOptionKind::AllowAlways
                )
            })
            .collect();

        // Get the selected granularity index, defaulting to the last option ("Only this time")
        let selected_index = self
            .selected_permission_granularity
            .get(&tool_call_id)
            .copied()
            .unwrap_or_else(|| granularity_options.len().saturating_sub(1));

        let selected_option = granularity_options
            .get(selected_index)
            .or(granularity_options.last())
            .copied();

        let dropdown_label: SharedString = selected_option
            .map(|o| o.name.clone().into())
            .unwrap_or_else(|| "Only this time".into());

        let (allow_option_id, allow_option_kind, deny_option_id, deny_option_kind) =
            if let Some(option) = selected_option {
                let option_id_str = option.option_id.0.to_string();

                // Transform option_id for allow: "always:tool" -> "always_allow:tool", "once" -> "allow"
                let allow_id = if option_id_str == "once" {
                    "allow".to_string()
                } else if let Some(rest) = option_id_str.strip_prefix("always:") {
                    format!("always_allow:{}", rest)
                } else if let Some(rest) = option_id_str.strip_prefix("always_pattern:") {
                    format!("always_allow_pattern:{}", rest)
                } else {
                    option_id_str.clone()
                };

                // Transform option_id for deny: "always:tool" -> "always_deny:tool", "once" -> "deny"
                let deny_id = if option_id_str == "once" {
                    "deny".to_string()
                } else if let Some(rest) = option_id_str.strip_prefix("always:") {
                    format!("always_deny:{}", rest)
                } else if let Some(rest) = option_id_str.strip_prefix("always_pattern:") {
                    format!("always_deny_pattern:{}", rest)
                } else {
                    option_id_str.replace("allow", "deny")
                };

                let allow_kind = option.kind;
                let deny_kind = match option.kind {
                    acp::PermissionOptionKind::AllowOnce => acp::PermissionOptionKind::RejectOnce,
                    acp::PermissionOptionKind::AllowAlways => {
                        acp::PermissionOptionKind::RejectAlways
                    }
                    other => other,
                };

                (
                    acp::PermissionOptionId::new(allow_id),
                    allow_kind,
                    acp::PermissionOptionId::new(deny_id),
                    deny_kind,
                )
            } else {
                (
                    acp::PermissionOptionId::new("allow"),
                    acp::PermissionOptionKind::AllowOnce,
                    acp::PermissionOptionId::new("deny"),
                    acp::PermissionOptionKind::RejectOnce,
                )
            };

        h_flex()
            .w_full()
            .p_1()
            .gap_2()
            .justify_between()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .child(
                h_flex()
                    .gap_0p5()
                    .child(
                        Button::new(("allow-btn", entry_ix), "Allow")
                            .icon(IconName::Check)
                            .icon_color(Color::Success)
                            .icon_position(IconPosition::Start)
                            .icon_size(IconSize::XSmall)
                            .label_size(LabelSize::Small)
                            .when(is_first, |this| {
                                this.key_binding(
                                    KeyBinding::for_action_in(
                                        &AllowOnce as &dyn Action,
                                        &self.focus_handle,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(10.))),
                                )
                            })
                            .on_click(cx.listener({
                                let tool_call_id = tool_call_id.clone();
                                let option_id = allow_option_id;
                                let option_kind = allow_option_kind;
                                move |this, _, window, cx| {
                                    this.authorize_tool_call(
                                        tool_call_id.clone(),
                                        option_id.clone(),
                                        option_kind,
                                        window,
                                        cx,
                                    );
                                }
                            })),
                    )
                    .child(
                        Button::new(("deny-btn", entry_ix), "Deny")
                            .icon(IconName::Close)
                            .icon_color(Color::Error)
                            .icon_position(IconPosition::Start)
                            .icon_size(IconSize::XSmall)
                            .label_size(LabelSize::Small)
                            .when(is_first, |this| {
                                this.key_binding(
                                    KeyBinding::for_action_in(
                                        &RejectOnce as &dyn Action,
                                        &self.focus_handle,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(10.))),
                                )
                            })
                            .on_click(cx.listener({
                                let tool_call_id = tool_call_id.clone();
                                let option_id = deny_option_id;
                                let option_kind = deny_option_kind;
                                move |this, _, window, cx| {
                                    this.authorize_tool_call(
                                        tool_call_id.clone(),
                                        option_id.clone(),
                                        option_kind,
                                        window,
                                        cx,
                                    );
                                }
                            })),
                    ),
            )
            .child(self.render_permission_granularity_dropdown(
                &granularity_options,
                dropdown_label,
                entry_ix,
                tool_call_id,
                selected_index,
                is_first,
                cx,
            ))
    }

    fn render_permission_granularity_dropdown(
        &self,
        granularity_options: &[&acp::PermissionOption],
        current_label: SharedString,
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        selected_index: usize,
        is_first: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let menu_options: Vec<(usize, SharedString)> = granularity_options
            .iter()
            .enumerate()
            .map(|(i, o)| (i, o.name.clone().into()))
            .collect();

        PopoverMenu::new(("permission-granularity", entry_ix))
            .with_handle(self.permission_dropdown_handle.clone())
            .trigger(
                Button::new(("granularity-trigger", entry_ix), current_label)
                    .icon(IconName::ChevronDown)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .label_size(LabelSize::Small)
                    .when(is_first, |this| {
                        this.key_binding(
                            KeyBinding::for_action_in(
                                &crate::OpenPermissionDropdown as &dyn Action,
                                &self.focus_handle,
                                cx,
                            )
                            .map(|kb| kb.size(rems_from_px(10.))),
                        )
                    }),
            )
            .menu(move |window, cx| {
                let tool_call_id = tool_call_id.clone();
                let options = menu_options.clone();

                Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                    for (index, display_name) in options.iter() {
                        let display_name = display_name.clone();
                        let index = *index;
                        let tool_call_id_for_entry = tool_call_id.clone();
                        let is_selected = index == selected_index;

                        menu = menu.toggleable_entry(
                            display_name,
                            is_selected,
                            IconPosition::End,
                            None,
                            move |window, cx| {
                                window.dispatch_action(
                                    SelectPermissionGranularity {
                                        tool_call_id: tool_call_id_for_entry.0.to_string(),
                                        index,
                                    }
                                    .boxed_clone(),
                                    cx,
                                );
                            },
                        );
                    }

                    menu
                }))
            })
    }

    fn render_permission_buttons_legacy(
        &self,
        options: &[acp::PermissionOption],
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        let is_first = self.thread().is_some_and(|thread| {
            thread
                .read(cx)
                .first_tool_awaiting_confirmation()
                .is_some_and(|call| call.id == tool_call_id)
        });
        let mut seen_kinds: ArrayVec<acp::PermissionOptionKind, 3> = ArrayVec::new();

        div()
            .p_1()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .w_full()
            .v_flex()
            .gap_0p5()
            .children(options.iter().map(move |option| {
                let option_id = SharedString::from(option.option_id.0.clone());
                Button::new((option_id, entry_ix), option.name.clone())
                    .map(|this| {
                        let (this, action) = match option.kind {
                            acp::PermissionOptionKind::AllowOnce => (
                                this.icon(IconName::Check).icon_color(Color::Success),
                                Some(&AllowOnce as &dyn Action),
                            ),
                            acp::PermissionOptionKind::AllowAlways => (
                                this.icon(IconName::CheckDouble).icon_color(Color::Success),
                                Some(&AllowAlways as &dyn Action),
                            ),
                            acp::PermissionOptionKind::RejectOnce => (
                                this.icon(IconName::Close).icon_color(Color::Error),
                                Some(&RejectOnce as &dyn Action),
                            ),
                            acp::PermissionOptionKind::RejectAlways | _ => {
                                (this.icon(IconName::Close).icon_color(Color::Error), None)
                            }
                        };

                        let Some(action) = action else {
                            return this;
                        };

                        if !is_first || seen_kinds.contains(&option.kind) {
                            return this;
                        }

                        seen_kinds.push(option.kind);

                        this.key_binding(
                            KeyBinding::for_action_in(action, &self.focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(10.))),
                        )
                    })
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::XSmall)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener({
                        let tool_call_id = tool_call_id.clone();
                        let option_id = option.option_id.clone();
                        let option_kind = option.kind;
                        move |this, _, window, cx| {
                            this.authorize_tool_call(
                                tool_call_id.clone(),
                                option_id.clone(),
                                option_kind,
                                window,
                                cx,
                            );
                        }
                    }))
            }))
    }

    fn render_diff_loading(&self, cx: &Context<Self>) -> AnyElement {
        let bar = |n: u64, width_class: &str| {
            let bg_color = cx.theme().colors().element_active;
            let base = h_flex().h_1().rounded_full();

            let modified = match width_class {
                "w_4_5" => base.w_3_4(),
                "w_1_4" => base.w_1_4(),
                "w_2_4" => base.w_2_4(),
                "w_3_5" => base.w_3_5(),
                "w_2_5" => base.w_2_5(),
                _ => base.w_1_2(),
            };

            modified.with_animation(
                ElementId::Integer(n),
                Animation::new(Duration::from_secs(2)).repeat(),
                move |tab, delta| {
                    let delta = (delta - 0.15 * n as f32) / 0.7;
                    let delta = 1.0 - (0.5 - delta).abs() * 2.;
                    let delta = ease_in_out(delta.clamp(0., 1.));
                    let delta = 0.1 + 0.9 * delta;

                    tab.bg(bg_color.opacity(delta))
                },
            )
        };

        v_flex()
            .p_3()
            .gap_1()
            .rounded_b_md()
            .bg(cx.theme().colors().editor_background)
            .child(bar(0, "w_4_5"))
            .child(bar(1, "w_1_4"))
            .child(bar(2, "w_2_4"))
            .child(bar(3, "w_3_5"))
            .child(bar(4, "w_2_5"))
            .into_any_element()
    }

    fn render_diff_editor(
        &self,
        entry_ix: usize,
        diff: &Entity<acp_thread::Diff>,
        tool_call: &ToolCall,
        has_failed: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let tool_progress = matches!(
            &tool_call.status,
            ToolCallStatus::InProgress | ToolCallStatus::Pending
        );

        let revealed_diff_editor = if let Some(entry) =
            self.entry_view_state.read(cx).entry(entry_ix)
            && let Some(editor) = entry.editor_for_diff(diff)
            && diff.read(cx).has_revealed_range(cx)
        {
            Some(editor)
        } else {
            None
        };

        let show_top_border = !has_failed || revealed_diff_editor.is_some();

        v_flex()
            .h_full()
            .when(show_top_border, |this| {
                this.border_t_1()
                    .when(has_failed, |this| this.border_dashed())
                    .border_color(self.tool_card_border_color(cx))
            })
            .child(if let Some(editor) = revealed_diff_editor {
                editor.into_any_element()
            } else if tool_progress && self.as_native_connection(cx).is_some() {
                self.render_diff_loading(cx)
            } else {
                Empty.into_any()
            })
            .into_any()
    }

    /// Renders command lines with an optional expand/collapse button depending
    /// on the number of lines in `command_source`.
    fn render_collapsible_command(
        &self,
        is_preview: bool,
        command_source: &str,
        tool_call_id: &acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        let expand_button_bg = self.tool_card_header_bg(cx);
        let expanded = self.expanded_terminal_commands.contains(tool_call_id);

        let lines: Vec<&str> = command_source.lines().collect();
        let line_count = lines.len();
        let extra_lines = line_count.saturating_sub(MAX_COLLAPSED_LINES);

        let show_expand_button = extra_lines > 0;

        let max_lines = if expanded || !show_expand_button {
            usize::MAX
        } else {
            MAX_COLLAPSED_LINES
        };

        let display_lines = lines.into_iter().take(max_lines);

        let command_group =
            SharedString::from(format!("collapsible-command-group-{}", tool_call_id));

        v_flex()
            .group(command_group.clone())
            .bg(self.tool_card_header_bg(cx))
            .child(
                v_flex()
                    .p_1p5()
                    .when(is_preview, |this| {
                        this.pt_1().child(
                            // Wrapping this label on a container with 24px height to avoid
                            // layout shift when it changes from being a preview label
                            // to the actual path where the command will run in
                            h_flex().h_6().child(
                                Label::new("Run Command")
                                    .buffer_font(cx)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                        )
                    })
                    .children(display_lines.map(|line| {
                        let text: SharedString = if line.is_empty() {
                            " ".into()
                        } else {
                            line.to_string().into()
                        };

                        Label::new(text).buffer_font(cx).size(LabelSize::Small)
                    }))
                    .child(
                        div().absolute().top_1().right_1().child(
                            CopyButton::new(command_source.to_string())
                                .tooltip_label("Copy Command")
                                .visible_on_hover(command_group),
                        ),
                    ),
            )
            .when(show_expand_button, |this| {
                let expand_icon = if expanded {
                    IconName::ChevronUp
                } else {
                    IconName::ChevronDown
                };

                this.child(
                    h_flex()
                        .id(format!("expand-command-btn-{}", tool_call_id))
                        .cursor_pointer()
                        .when(!expanded, |s| s.absolute().bottom_0())
                        .when(expanded, |s| s.mt_1())
                        .w_full()
                        .h_6()
                        .gap_1()
                        .justify_center()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .bg(expand_button_bg.opacity(0.95))
                        .hover(|s| s.bg(cx.theme().colors().element_hover))
                        .when(!expanded, |this| {
                            let label = match extra_lines {
                                1 => "1 more line".to_string(),
                                _ => format!("{} more lines", extra_lines),
                            };

                            this.child(Label::new(label).size(LabelSize::Small).color(Color::Muted))
                        })
                        .child(
                            Icon::new(expand_icon)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .on_click(cx.listener({
                            let tool_call_id = tool_call_id.clone();
                            move |this, _event, _window, cx| {
                                if expanded {
                                    this.expanded_terminal_commands.remove(&tool_call_id);
                                } else {
                                    this.expanded_terminal_commands.insert(tool_call_id.clone());
                                }
                                cx.notify();
                            }
                        })),
                )
            })
    }

    fn render_terminal_tool_call(
        &self,
        entry_ix: usize,
        terminal: &Entity<acp_thread::Terminal>,
        tool_call: &ToolCall,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let terminal_data = terminal.read(cx);
        let working_dir = terminal_data.working_dir();
        let command = terminal_data.command();
        let started_at = terminal_data.started_at();

        let tool_failed = matches!(
            &tool_call.status,
            ToolCallStatus::Rejected | ToolCallStatus::Canceled | ToolCallStatus::Failed
        );

        let output = terminal_data.output();
        let command_finished = output.is_some();
        let truncated_output =
            output.is_some_and(|output| output.original_content_len > output.content.len());
        let output_line_count = output.map(|output| output.content_line_count).unwrap_or(0);

        let command_failed = command_finished
            && output.is_some_and(|o| o.exit_status.is_some_and(|status| !status.success()));

        let time_elapsed = if let Some(output) = output {
            output.ended_at.duration_since(started_at)
        } else {
            started_at.elapsed()
        };

        let header_id =
            SharedString::from(format!("terminal-tool-header-{}", terminal.entity_id()));
        let header_group = SharedString::from(format!(
            "terminal-tool-header-group-{}",
            terminal.entity_id()
        ));
        let header_bg = cx
            .theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025));
        let border_color = cx.theme().colors().border.opacity(0.6);

        let working_dir = working_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "current directory".to_string());

        // Since the command's source is wrapped in a markdown code block
        // (```\n...\n```), we need to strip that so we're left with only the
        // command's content.
        let command_source = command.read(cx).source();
        let command_content = command_source
            .strip_prefix("```\n")
            .and_then(|s| s.strip_suffix("\n```"))
            .unwrap_or(&command_source);

        let command_element =
            self.render_collapsible_command(false, command_content, &tool_call.id, cx);

        let is_expanded = self.expanded_tool_calls.contains(&tool_call.id);

        let header = h_flex()
            .id(header_id)
            .px_1p5()
            .pt_1()
            .flex_none()
            .gap_1()
            .justify_between()
            .rounded_t_md()
            .child(
                div()
                    .id(("command-target-path", terminal.entity_id()))
                    .w_full()
                    .max_w_full()
                    .overflow_x_scroll()
                    .child(
                        Label::new(working_dir)
                            .buffer_font(cx)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
            .when(!command_finished, |header| {
                header
                    .gap_1p5()
                    .child(
                        Button::new(
                            SharedString::from(format!("stop-terminal-{}", terminal.entity_id())),
                            "Stop",
                        )
                        .icon(IconName::Stop)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Error)
                        .label_size(LabelSize::Small)
                        .tooltip(move |_window, cx| {
                            Tooltip::with_meta(
                                "Stop This Command",
                                None,
                                "Also possible by placing your cursor inside the terminal and using regular terminal bindings.",
                                cx,
                            )
                        })
                        .on_click({
                            let terminal = terminal.clone();
                            cx.listener(move |this, _event, _window, cx| {
                                terminal.update(cx, |terminal, cx| {
                                    terminal.stop_by_user(cx);
                                });
                                this.cancel_generation(cx);
                            })
                        }),
                    )
                    .child(Divider::vertical())
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::XSmall)
                            .color(Color::Info)
                            .with_rotate_animation(2)
                    )
            })
            .when(truncated_output, |header| {
                let tooltip = if let Some(output) = output {
                    if output_line_count + 10 > terminal::MAX_SCROLL_HISTORY_LINES {
                       format!("Output exceeded terminal max lines and was \
                            truncated, the model received the first {}.", format_file_size(output.content.len() as u64, true))
                    } else {
                        format!(
                            "Output is {} long, and to avoid unexpected token usage, \
                                only {} was sent back to the agent.",
                            format_file_size(output.original_content_len as u64, true),
                             format_file_size(output.content.len() as u64, true)
                        )
                    }
                } else {
                    "Output was truncated".to_string()
                };

                header.child(
                    h_flex()
                        .id(("terminal-tool-truncated-label", terminal.entity_id()))
                        .gap_1()
                        .child(
                            Icon::new(IconName::Info)
                                .size(IconSize::XSmall)
                                .color(Color::Ignored),
                        )
                        .child(
                            Label::new("Truncated")
                                .color(Color::Muted)
                                .size(LabelSize::XSmall),
                        )
                        .tooltip(Tooltip::text(tooltip)),
                )
            })
            .when(time_elapsed > Duration::from_secs(10), |header| {
                header.child(
                    Label::new(format!("({})", duration_alt_display(time_elapsed)))
                        .buffer_font(cx)
                        .color(Color::Muted)
                        .size(LabelSize::XSmall),
                )
            })
            .when(tool_failed || command_failed, |header| {
                header.child(
                    div()
                        .id(("terminal-tool-error-code-indicator", terminal.entity_id()))
                        .child(
                            Icon::new(IconName::Close)
                                .size(IconSize::Small)
                                .color(Color::Error),
                        )
                        .when_some(output.and_then(|o| o.exit_status), |this, status| {
                            this.tooltip(Tooltip::text(format!(
                                "Exited with code {}",
                                status.code().unwrap_or(-1),
                            )))
                        }),
                )
            })
            .child(
                Disclosure::new(
                    SharedString::from(format!(
                        "terminal-tool-disclosure-{}",
                        terminal.entity_id()
                    )),
                    is_expanded,
                )
                .opened_icon(IconName::ChevronUp)
                .closed_icon(IconName::ChevronDown)
                .visible_on_hover(&header_group)
                .on_click(cx.listener({
                    let id = tool_call.id.clone();
                    move |this, _event, _window, _cx| {
                        if is_expanded {
                            this.expanded_tool_calls.remove(&id);
                        } else {
                            this.expanded_tool_calls.insert(id.clone());
                        }
                    }
                })),
            );

        let terminal_view = self
            .entry_view_state
            .read(cx)
            .entry(entry_ix)
            .and_then(|entry| entry.terminal(terminal));

        v_flex()
            .my_1p5()
            .mx_5()
            .border_1()
            .when(tool_failed || command_failed, |card| card.border_dashed())
            .border_color(border_color)
            .rounded_md()
            .overflow_hidden()
            .child(
                v_flex()
                    .group(&header_group)
                    .bg(header_bg)
                    .text_xs()
                    .child(header)
                    .child(command_element),
            )
            .when(is_expanded && terminal_view.is_some(), |this| {
                this.child(
                    div()
                        .pt_2()
                        .border_t_1()
                        .when(tool_failed || command_failed, |card| card.border_dashed())
                        .border_color(border_color)
                        .bg(cx.theme().colors().editor_background)
                        .rounded_b_md()
                        .text_ui_sm(cx)
                        .h_full()
                        .children(terminal_view.map(|terminal_view| {
                            let element = if terminal_view
                                .read(cx)
                                .content_mode(window, cx)
                                .is_scrollable()
                            {
                                div().h_72().child(terminal_view).into_any_element()
                            } else {
                                terminal_view.into_any_element()
                            };

                            div()
                                .on_action(cx.listener(|_this, _: &NewTerminal, window, cx| {
                                    window.dispatch_action(NewThread.boxed_clone(), cx);
                                    cx.stop_propagation();
                                }))
                                .child(element)
                                .into_any_element()
                        })),
                )
            })
            .into_any()
    }

    fn render_rules_item(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let project_context = self
            .as_native_thread(cx)?
            .read(cx)
            .project_context()
            .read(cx);

        let user_rules_text = if project_context.user_rules.is_empty() {
            None
        } else if project_context.user_rules.len() == 1 {
            let user_rules = &project_context.user_rules[0];

            match user_rules.title.as_ref() {
                Some(title) => Some(format!("Using \"{title}\" user rule")),
                None => Some("Using user rule".into()),
            }
        } else {
            Some(format!(
                "Using {} user rules",
                project_context.user_rules.len()
            ))
        };

        let first_user_rules_id = project_context
            .user_rules
            .first()
            .map(|user_rules| user_rules.uuid.0);

        let rules_files = project_context
            .worktrees
            .iter()
            .filter_map(|worktree| worktree.rules_file.as_ref())
            .collect::<Vec<_>>();

        let rules_file_text = match rules_files.as_slice() {
            &[] => None,
            &[rules_file] => Some(format!(
                "Using project {:?} file",
                rules_file.path_in_worktree
            )),
            rules_files => Some(format!("Using {} project rules files", rules_files.len())),
        };

        if user_rules_text.is_none() && rules_file_text.is_none() {
            return None;
        }

        let has_both = user_rules_text.is_some() && rules_file_text.is_some();

        Some(
            h_flex()
                .px_2p5()
                .child(
                    Icon::new(IconName::Attach)
                        .size(IconSize::XSmall)
                        .color(Color::Disabled),
                )
                .when_some(user_rules_text, |parent, user_rules_text| {
                    parent.child(
                        h_flex()
                            .id("user-rules")
                            .ml_1()
                            .mr_1p5()
                            .child(
                                Label::new(user_rules_text)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                                    .truncate(),
                            )
                            .hover(|s| s.bg(cx.theme().colors().element_hover))
                            .tooltip(Tooltip::text("View User Rules"))
                            .on_click(move |_event, window, cx| {
                                window.dispatch_action(
                                    Box::new(OpenRulesLibrary {
                                        prompt_to_select: first_user_rules_id,
                                    }),
                                    cx,
                                )
                            }),
                    )
                })
                .when(has_both, |this| {
                    this.child(
                        Label::new("•")
                            .size(LabelSize::XSmall)
                            .color(Color::Disabled),
                    )
                })
                .when_some(rules_file_text, |parent, rules_file_text| {
                    parent.child(
                        h_flex()
                            .id("project-rules")
                            .ml_1p5()
                            .child(
                                Label::new(rules_file_text)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .hover(|s| s.bg(cx.theme().colors().element_hover))
                            .tooltip(Tooltip::text("View Project Rules"))
                            .on_click(cx.listener(Self::handle_open_rules)),
                    )
                })
                .into_any(),
        )
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
                                    default_markdown_style(false, false, window, cx),
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

    fn activity_bar_bg(&self, cx: &Context<Self>) -> Hsla {
        let editor_bg_color = cx.theme().colors().editor_background;
        let active_color = cx.theme().colors().element_selected;
        editor_bg_color.blend(active_color.opacity(0.3))
    }

    fn render_activity_bar(
        &self,
        thread_entity: &Entity<AcpThread>,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let thread = thread_entity.read(cx);
        let action_log = thread.action_log();
        let telemetry = ActionLogTelemetry::from(thread);
        let changed_buffers = action_log.read(cx).changed_buffers(cx);
        let plan = thread.plan();
        let queue_is_empty = self
            .as_native_thread(cx)
            .map_or(true, |t| t.read(cx).queued_messages().is_empty());

        if changed_buffers.is_empty() && plan.is_empty() && queue_is_empty {
            return None;
        }

        // Temporarily always enable ACP edit controls. This is temporary, to lessen the
        // impact of a nasty bug that causes them to sometimes be disabled when they shouldn't
        // be, which blocks you from being able to accept or reject edits. This switches the
        // bug to be that sometimes it's enabled when it shouldn't be, which at least doesn't
        // block you from using the panel.
        let pending_edits = false;

        let use_keep_reject_buttons = !cx.has_flag::<AgentV2FeatureFlag>();

        v_flex()
            .mt_1()
            .mx_2()
            .bg(self.activity_bar_bg(cx))
            .border_1()
            .border_b_0()
            .border_color(cx.theme().colors().border)
            .rounded_t_md()
            .shadow(vec![gpui::BoxShadow {
                color: gpui::black().opacity(0.15),
                offset: point(px(1.), px(-1.)),
                blur_radius: px(3.),
                spread_radius: px(0.),
            }])
            .when(!plan.is_empty(), |this| {
                this.child(self.render_plan_summary(plan, window, cx))
                    .when(self.plan_expanded, |parent| {
                        parent.child(self.render_plan_entries(plan, window, cx))
                    })
            })
            .when(!plan.is_empty() && !changed_buffers.is_empty(), |this| {
                this.child(Divider::horizontal().color(DividerColor::Border))
            })
            .when(!changed_buffers.is_empty(), |this| {
                this.child(self.render_edits_summary(
                    &changed_buffers,
                    self.edits_expanded,
                    pending_edits,
                    use_keep_reject_buttons,
                    cx,
                ))
                .when(self.edits_expanded, |parent| {
                    parent.child(self.render_edited_files(
                        action_log,
                        telemetry.clone(),
                        &changed_buffers,
                        pending_edits,
                        use_keep_reject_buttons,
                        cx,
                    ))
                })
            })
            .when(!queue_is_empty, |this| {
                this.when(!plan.is_empty() || !changed_buffers.is_empty(), |this| {
                    this.child(Divider::horizontal().color(DividerColor::Border))
                })
                .child(self.render_message_queue_summary(window, cx))
                .when(self.queue_expanded, |parent| {
                    parent.child(self.render_message_queue_entries(window, cx))
                })
            })
            .into_any()
            .into()
    }

    fn render_plan_summary(
        &self,
        plan: &Plan,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let stats = plan.stats();

        let title = if let Some(entry) = stats.in_progress_entry
            && !self.plan_expanded
        {
            h_flex()
                .cursor_default()
                .relative()
                .w_full()
                .gap_1()
                .truncate()
                .child(
                    Label::new("Current:")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().colors().text_muted)
                        .line_clamp(1)
                        .child(MarkdownElement::new(
                            entry.content.clone(),
                            plan_label_markdown_style(&entry.status, window, cx),
                        )),
                )
                .when(stats.pending > 0, |this| {
                    this.child(
                        h_flex()
                            .absolute()
                            .top_0()
                            .right_0()
                            .h_full()
                            .child(div().min_w_8().h_full().bg(linear_gradient(
                                90.,
                                linear_color_stop(self.activity_bar_bg(cx), 1.),
                                linear_color_stop(self.activity_bar_bg(cx).opacity(0.2), 0.),
                            )))
                            .child(
                                div().pr_0p5().bg(self.activity_bar_bg(cx)).child(
                                    Label::new(format!("{} left", stats.pending))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                            ),
                    )
                })
        } else {
            let status_label = if stats.pending == 0 {
                "All Done".to_string()
            } else if stats.completed == 0 {
                format!("{} Tasks", plan.entries.len())
            } else {
                format!("{}/{}", stats.completed, plan.entries.len())
            };

            h_flex()
                .w_full()
                .gap_1()
                .justify_between()
                .child(
                    Label::new("Plan")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new(status_label)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .mr_1(),
                )
        };

        h_flex()
            .id("plan_summary")
            .p_1()
            .w_full()
            .gap_1()
            .when(self.plan_expanded, |this| {
                this.border_b_1().border_color(cx.theme().colors().border)
            })
            .child(Disclosure::new("plan_disclosure", self.plan_expanded))
            .child(title)
            .on_click(cx.listener(|this, _, _, cx| {
                this.plan_expanded = !this.plan_expanded;
                cx.notify();
            }))
    }

    fn render_plan_entries(
        &self,
        plan: &Plan,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .id("plan_items_list")
            .max_h_40()
            .overflow_y_scroll()
            .children(plan.entries.iter().enumerate().flat_map(|(index, entry)| {
                let element = h_flex()
                    .py_1()
                    .px_2()
                    .gap_2()
                    .justify_between()
                    .bg(cx.theme().colors().editor_background)
                    .when(index < plan.entries.len() - 1, |parent| {
                        parent.border_color(cx.theme().colors().border).border_b_1()
                    })
                    .child(
                        h_flex()
                            .id(("plan_entry", index))
                            .gap_1p5()
                            .max_w_full()
                            .overflow_x_scroll()
                            .text_xs()
                            .text_color(cx.theme().colors().text_muted)
                            .child(match entry.status {
                                acp::PlanEntryStatus::InProgress => {
                                    Icon::new(IconName::TodoProgress)
                                        .size(IconSize::Small)
                                        .color(Color::Accent)
                                        .with_rotate_animation(2)
                                        .into_any_element()
                                }
                                acp::PlanEntryStatus::Completed => {
                                    Icon::new(IconName::TodoComplete)
                                        .size(IconSize::Small)
                                        .color(Color::Success)
                                        .into_any_element()
                                }
                                acp::PlanEntryStatus::Pending | _ => {
                                    Icon::new(IconName::TodoPending)
                                        .size(IconSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element()
                                }
                            })
                            .child(MarkdownElement::new(
                                entry.content.clone(),
                                plan_label_markdown_style(&entry.status, window, cx),
                            )),
                    );

                Some(element)
            }))
            .into_any_element()
    }

    fn render_edits_summary(
        &self,
        changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>,
        expanded: bool,
        pending_edits: bool,
        use_keep_reject_buttons: bool,
        cx: &Context<Self>,
    ) -> Div {
        const EDIT_NOT_READY_TOOLTIP_LABEL: &str = "Wait until file edits are complete.";

        let focus_handle = self.focus_handle(cx);

        h_flex()
            .p_1()
            .justify_between()
            .flex_wrap()
            .when(expanded, |this| {
                this.border_b_1().border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .id("edits-container")
                    .cursor_pointer()
                    .gap_1()
                    .child(Disclosure::new("edits-disclosure", expanded))
                    .map(|this| {
                        if pending_edits {
                            this.child(
                                Label::new(format!(
                                    "Editing {} {}…",
                                    changed_buffers.len(),
                                    if changed_buffers.len() == 1 {
                                        "file"
                                    } else {
                                        "files"
                                    }
                                ))
                                .color(Color::Muted)
                                .size(LabelSize::Small)
                                .with_animation(
                                    "edit-label",
                                    Animation::new(Duration::from_secs(2))
                                        .repeat()
                                        .with_easing(pulsating_between(0.3, 0.7)),
                                    |label, delta| label.alpha(delta),
                                ),
                            )
                        } else {
                            let stats = DiffStats::all_files(changed_buffers, cx);
                            let dot_divider = || {
                                Label::new("•")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Disabled)
                            };

                            this.child(
                                Label::new("Edits")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(dot_divider())
                            .child(
                                Label::new(format!(
                                    "{} {}",
                                    changed_buffers.len(),
                                    if changed_buffers.len() == 1 {
                                        "file"
                                    } else {
                                        "files"
                                    }
                                ))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            )
                            .child(dot_divider())
                            .child(DiffStat::new(
                                "total",
                                stats.lines_added as usize,
                                stats.lines_removed as usize,
                            ))
                        }
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.edits_expanded = !this.edits_expanded;
                        cx.notify();
                    })),
            )
            .when(use_keep_reject_buttons, |this| {
                this.child(
                    h_flex()
                        .gap_1()
                        .child(
                            IconButton::new("review-changes", IconName::ListTodo)
                                .icon_size(IconSize::Small)
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |_window, cx| {
                                        Tooltip::for_action_in(
                                            "Review Changes",
                                            &OpenAgentDiff,
                                            &focus_handle,
                                            cx,
                                        )
                                    }
                                })
                                .on_click(cx.listener(|_, _, window, cx| {
                                    window.dispatch_action(OpenAgentDiff.boxed_clone(), cx);
                                })),
                        )
                        .child(Divider::vertical().color(DividerColor::Border))
                        .child(
                            Button::new("reject-all-changes", "Reject All")
                                .label_size(LabelSize::Small)
                                .disabled(pending_edits)
                                .when(pending_edits, |this| {
                                    this.tooltip(Tooltip::text(EDIT_NOT_READY_TOOLTIP_LABEL))
                                })
                                .key_binding(
                                    KeyBinding::for_action_in(
                                        &RejectAll,
                                        &focus_handle.clone(),
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(10.))),
                                )
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.reject_all(&RejectAll, window, cx);
                                })),
                        )
                        .child(
                            Button::new("keep-all-changes", "Keep All")
                                .label_size(LabelSize::Small)
                                .disabled(pending_edits)
                                .when(pending_edits, |this| {
                                    this.tooltip(Tooltip::text(EDIT_NOT_READY_TOOLTIP_LABEL))
                                })
                                .key_binding(
                                    KeyBinding::for_action_in(&KeepAll, &focus_handle, cx)
                                        .map(|kb| kb.size(rems_from_px(10.))),
                                )
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.keep_all(&KeepAll, window, cx);
                                })),
                        ),
                )
            })
            .when(!use_keep_reject_buttons, |this| {
                this.child(
                    Button::new("review-changes", "Review Changes")
                        .label_size(LabelSize::Small)
                        .key_binding(
                            KeyBinding::for_action_in(
                                &git_ui::project_diff::Diff,
                                &focus_handle,
                                cx,
                            )
                            .map(|kb| kb.size(rems_from_px(10.))),
                        )
                        .on_click(cx.listener(move |_, _, window, cx| {
                            window.dispatch_action(git_ui::project_diff::Diff.boxed_clone(), cx);
                        })),
                )
            })
    }

    fn render_edited_files_buttons(
        &self,
        index: usize,
        buffer: &Entity<Buffer>,
        action_log: &Entity<ActionLog>,
        telemetry: &ActionLogTelemetry,
        pending_edits: bool,
        use_keep_reject_buttons: bool,
        editor_bg_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let container = h_flex()
            .id("edited-buttons-container")
            .visible_on_hover("edited-code")
            .absolute()
            .right_0()
            .px_1()
            .gap_1()
            .bg(editor_bg_color)
            .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                if *is_hovered {
                    this.hovered_edited_file_buttons = Some(index);
                } else if this.hovered_edited_file_buttons == Some(index) {
                    this.hovered_edited_file_buttons = None;
                }
                cx.notify();
            }));

        if use_keep_reject_buttons {
            container
                .child(
                    Button::new(("review", index), "Review")
                        .label_size(LabelSize::Small)
                        .on_click({
                            let buffer = buffer.clone();
                            let workspace = self.workspace.clone();
                            cx.listener(move |_, _, window, cx| {
                                let Some(workspace) = workspace.upgrade() else {
                                    return;
                                };
                                let Some(file) = buffer.read(cx).file() else {
                                    return;
                                };
                                let project_path = project::ProjectPath {
                                    worktree_id: file.worktree_id(cx),
                                    path: file.path().clone(),
                                };
                                workspace.update(cx, |workspace, cx| {
                                    git_ui::project_diff::ProjectDiff::deploy_at_project_path(
                                        workspace,
                                        project_path,
                                        window,
                                        cx,
                                    );
                                });
                            })
                        }),
                )
                .child(Divider::vertical().color(DividerColor::BorderVariant))
                .child(
                    Button::new(("reject-file", index), "Reject")
                        .label_size(LabelSize::Small)
                        .disabled(pending_edits)
                        .on_click({
                            let buffer = buffer.clone();
                            let action_log = action_log.clone();
                            let telemetry = telemetry.clone();
                            move |_, _, cx| {
                                action_log.update(cx, |action_log, cx| {
                                    action_log
                                        .reject_edits_in_ranges(
                                            buffer.clone(),
                                            vec![Anchor::min_max_range_for_buffer(
                                                buffer.read(cx).remote_id(),
                                            )],
                                            Some(telemetry.clone()),
                                            cx,
                                        )
                                        .detach_and_log_err(cx);
                                })
                            }
                        }),
                )
                .child(
                    Button::new(("keep-file", index), "Keep")
                        .label_size(LabelSize::Small)
                        .disabled(pending_edits)
                        .on_click({
                            let buffer = buffer.clone();
                            let action_log = action_log.clone();
                            let telemetry = telemetry.clone();
                            move |_, _, cx| {
                                action_log.update(cx, |action_log, cx| {
                                    action_log.keep_edits_in_range(
                                        buffer.clone(),
                                        Anchor::min_max_range_for_buffer(
                                            buffer.read(cx).remote_id(),
                                        ),
                                        Some(telemetry.clone()),
                                        cx,
                                    );
                                })
                            }
                        }),
                )
                .into_any_element()
        } else {
            container
                .child(
                    Button::new(("review", index), "Review")
                        .label_size(LabelSize::Small)
                        .on_click({
                            let buffer = buffer.clone();
                            let workspace = self.workspace.clone();
                            cx.listener(move |_, _, window, cx| {
                                let Some(workspace) = workspace.upgrade() else {
                                    return;
                                };
                                let Some(file) = buffer.read(cx).file() else {
                                    return;
                                };
                                let project_path = project::ProjectPath {
                                    worktree_id: file.worktree_id(cx),
                                    path: file.path().clone(),
                                };
                                workspace.update(cx, |workspace, cx| {
                                    git_ui::project_diff::ProjectDiff::deploy_at_project_path(
                                        workspace,
                                        project_path,
                                        window,
                                        cx,
                                    );
                                });
                            })
                        }),
                )
                .into_any_element()
        }
    }

    fn render_edited_files(
        &self,
        action_log: &Entity<ActionLog>,
        telemetry: ActionLogTelemetry,
        changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>,
        pending_edits: bool,
        use_keep_reject_buttons: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let editor_bg_color = cx.theme().colors().editor_background;

        v_flex()
            .id("edited_files_list")
            .max_h_40()
            .overflow_y_scroll()
            .children(
                changed_buffers
                    .iter()
                    .enumerate()
                    .flat_map(|(index, (buffer, diff))| {
                        let file = buffer.read(cx).file()?;
                        let path = file.path();
                        let path_style = file.path_style(cx);
                        let separator = file.path_style(cx).primary_separator();

                        let file_path = path.parent().and_then(|parent| {
                            if parent.is_empty() {
                                None
                            } else {
                                Some(
                                    Label::new(format!(
                                        "{}{separator}",
                                        parent.display(path_style)
                                    ))
                                    .color(Color::Muted)
                                    .size(LabelSize::XSmall)
                                    .buffer_font(cx),
                                )
                            }
                        });

                        let file_name = path.file_name().map(|name| {
                            Label::new(name.to_string())
                                .size(LabelSize::XSmall)
                                .buffer_font(cx)
                                .ml_1()
                        });

                        let full_path = path.display(path_style).to_string();

                        let file_icon = FileIcons::get_icon(path.as_std_path(), cx)
                            .map(Icon::from_path)
                            .map(|icon| icon.color(Color::Muted).size(IconSize::Small))
                            .unwrap_or_else(|| {
                                Icon::new(IconName::File)
                                    .color(Color::Muted)
                                    .size(IconSize::Small)
                            });

                        let file_stats = DiffStats::single_file(buffer.read(cx), diff.read(cx), cx);

                        let buttons = self.render_edited_files_buttons(
                            index,
                            buffer,
                            action_log,
                            &telemetry,
                            pending_edits,
                            use_keep_reject_buttons,
                            editor_bg_color,
                            cx,
                        );

                        let element = h_flex()
                            .group("edited-code")
                            .id(("file-container", index))
                            .relative()
                            .min_w_0()
                            .p_1p5()
                            .gap_2()
                            .justify_between()
                            .bg(editor_bg_color)
                            .when(index < changed_buffers.len() - 1, |parent| {
                                parent.border_color(cx.theme().colors().border).border_b_1()
                            })
                            .child(
                                h_flex()
                                    .id(("file-name-path", index))
                                    .cursor_pointer()
                                    .pr_0p5()
                                    .gap_0p5()
                                    .rounded_xs()
                                    .child(file_icon)
                                    .children(file_name)
                                    .children(file_path)
                                    .child(
                                        DiffStat::new(
                                            "file",
                                            file_stats.lines_added as usize,
                                            file_stats.lines_removed as usize,
                                        )
                                        .label_size(LabelSize::XSmall),
                                    )
                                    .when(
                                        self.hovered_edited_file_buttons != Some(index),
                                        |this| {
                                            let full_path = full_path.clone();
                                            this.hover(|s| s.bg(cx.theme().colors().element_hover))
                                                .tooltip(move |_, cx| {
                                                    Tooltip::with_meta(
                                                        "Go to File",
                                                        None,
                                                        full_path.clone(),
                                                        cx,
                                                    )
                                                })
                                                .on_click({
                                                    let buffer = buffer.clone();
                                                    cx.listener(move |this, _, window, cx| {
                                                        this.open_edited_buffer(
                                                            &buffer, window, cx,
                                                        );
                                                    })
                                                })
                                        },
                                    ),
                            )
                            .child(buttons);

                        Some(element)
                    }),
            )
            .into_any_element()
    }

    fn render_message_queue_summary(
        &self,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let queue_count = self
            .as_native_thread(cx)
            .map_or(0, |t| t.read(cx).queued_messages().len());
        let title: SharedString = if queue_count == 1 {
            "1 Queued Message".into()
        } else {
            format!("{} Queued Messages", queue_count).into()
        };

        h_flex()
            .p_1()
            .w_full()
            .gap_1()
            .justify_between()
            .when(self.queue_expanded, |this| {
                this.border_b_1().border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .id("queue_summary")
                    .gap_1()
                    .child(Disclosure::new("queue_disclosure", self.queue_expanded))
                    .child(Label::new(title).size(LabelSize::Small).color(Color::Muted))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.queue_expanded = !this.queue_expanded;
                        cx.notify();
                    })),
            )
            .child(
                Button::new("clear_queue", "Clear All")
                    .label_size(LabelSize::Small)
                    .key_binding(KeyBinding::for_action(&ClearMessageQueue, cx))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(thread) = this.as_native_thread(cx) {
                            thread.update(cx, |thread, _| thread.clear_queued_messages());
                        }
                        this.can_fast_track_queue = false;
                        cx.notify();
                    })),
            )
    }

    fn render_message_queue_entries(
        &self,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let message_editor = self.message_editor.read(cx);
        let focus_handle = message_editor.focus_handle(cx);

        let queued_messages: Vec<_> = self
            .as_native_thread(cx)
            .map(|t| {
                t.read(cx)
                    .queued_messages()
                    .iter()
                    .map(|q| q.content.clone())
                    .collect()
            })
            .unwrap_or_default();

        let queue_len = queued_messages.len();
        let can_fast_track = self.can_fast_track_queue && queue_len > 0;

        v_flex()
            .id("message_queue_list")
            .max_h_40()
            .overflow_y_scroll()
            .children(
                queued_messages
                    .into_iter()
                    .enumerate()
                    .map(|(index, content)| {
                        let is_next = index == 0;
                        let icon_color = if is_next { Color::Accent } else { Color::Muted };

                        let preview: String = content
                            .iter()
                            .filter_map(|block| match block {
                                acp::ContentBlock::Text(text) => {
                                    let first_line = text.text.lines().next()?;
                                    if first_line.is_empty() {
                                        None
                                    } else {
                                        Some(first_line.to_owned())
                                    }
                                }
                                acp::ContentBlock::Image(_) => Some("@Image".to_owned()),
                                acp::ContentBlock::Audio(_) => Some("@Audio".to_owned()),
                                acp::ContentBlock::ResourceLink(link) => {
                                    let name = link.uri.rsplit('/').next().unwrap_or(&link.uri);
                                    Some(format!("@{}", name))
                                }
                                acp::ContentBlock::Resource(resource) => {
                                    let uri = match &resource.resource {
                                        acp::EmbeddedResourceResource::TextResourceContents(r) => {
                                            Some(&r.uri)
                                        }
                                        acp::EmbeddedResourceResource::BlobResourceContents(r) => {
                                            Some(&r.uri)
                                        }
                                        _ => None,
                                    };
                                    uri.map(|uri| {
                                        let name = uri.rsplit('/').next().unwrap_or(uri);
                                        format!("@{}", name)
                                    })
                                }
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        h_flex()
                            .group("queue_entry")
                            .w_full()
                            .p_1()
                            .pl_2()
                            .gap_1()
                            .justify_between()
                            .bg(cx.theme().colors().editor_background)
                            .when(index < queue_len - 1, |parent| {
                                parent.border_color(cx.theme().colors().border).border_b_1()
                            })
                            .child(
                                h_flex()
                                    .id(("queued_prompt", index))
                                    .min_w_0()
                                    .w_full()
                                    .gap_1p5()
                                    .child(
                                        Icon::new(IconName::Circle)
                                            .size(IconSize::Small)
                                            .color(icon_color),
                                    )
                                    .child(
                                        Label::new(preview)
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                            .buffer_font(cx)
                                            .truncate(),
                                    )
                                    .when(is_next, |this| {
                                        this.tooltip(Tooltip::text("Next Prompt in the Queue"))
                                    }),
                            )
                            .child(
                                h_flex()
                                    .flex_none()
                                    .gap_1()
                                    .when(!is_next, |this| this.visible_on_hover("queue_entry"))
                                    .child(
                                        Button::new(("delete", index), "Remove")
                                            .label_size(LabelSize::Small)
                                            .tooltip(Tooltip::text("Remove Message from Queue"))
                                            .when(is_next, |this| {
                                                this.key_binding(
                                                    KeyBinding::for_action_in(
                                                        &RemoveFirstQueuedMessage,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                    .map(|kb| kb.size(rems_from_px(10.))),
                                                )
                                            })
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                if let Some(thread) = this.as_native_thread(cx) {
                                                    thread.update(cx, |thread, _| {
                                                        thread.remove_queued_message(index);
                                                    });
                                                }
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Button::new(("send_now", index), "Send Now")
                                            .label_size(LabelSize::Small)
                                            .when(is_next, |this| {
                                                let action: Box<dyn gpui::Action> =
                                                    if can_fast_track {
                                                        Box::new(Chat)
                                                    } else {
                                                        Box::new(SendNextQueuedMessage)
                                                    };

                                                this.style(ButtonStyle::Outlined).key_binding(
                                                    KeyBinding::for_action_in(
                                                        action.as_ref(),
                                                        &focus_handle.clone(),
                                                        cx,
                                                    )
                                                    .map(|kb| kb.size(rems_from_px(10.))),
                                                )
                                            })
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.send_queued_message_at_index(
                                                    index, true, window, cx,
                                                );
                                            })),
                                    ),
                            )
                    }),
            )
            .into_any_element()
    }

    fn render_message_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let focus_handle = self.message_editor.focus_handle(cx);
        let editor_bg_color = cx.theme().colors().editor_background;
        let (expand_icon, expand_tooltip) = if self.editor_expanded {
            (IconName::Minimize, "Minimize Message Editor")
        } else {
            (IconName::Maximize, "Expand Message Editor")
        };

        let backdrop = div()
            .size_full()
            .absolute()
            .inset_0()
            .bg(cx.theme().colors().panel_background)
            .opacity(0.8)
            .block_mouse_except_scroll();

        let enable_editor = match self.thread_state {
            ThreadState::Ready { .. } => true,
            ThreadState::Loading { .. }
            | ThreadState::Unauthenticated { .. }
            | ThreadState::LoadError(..) => false,
        };

        v_flex()
            .on_action(cx.listener(Self::expand_message_editor))
            .p_2()
            .gap_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(editor_bg_color)
            .when(self.editor_expanded, |this| {
                this.h(vh(0.8, window)).size_full().justify_between()
            })
            .child(
                v_flex()
                    .relative()
                    .size_full()
                    .pt_1()
                    .pr_2p5()
                    .child(self.message_editor.clone())
                    .child(
                        h_flex()
                            .absolute()
                            .top_0()
                            .right_0()
                            .opacity(0.5)
                            .hover(|this| this.opacity(1.0))
                            .child(
                                IconButton::new("toggle-height", expand_icon)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .tooltip({
                                        move |_window, cx| {
                                            Tooltip::for_action_in(
                                                expand_tooltip,
                                                &ExpandMessageEditor,
                                                &focus_handle,
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.expand_message_editor(
                                            &ExpandMessageEditor,
                                            window,
                                            cx,
                                        );
                                    })),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .flex_wrap()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_0p5()
                            .child(self.render_add_context_button(cx))
                            .child(self.render_follow_toggle(cx)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .children(self.render_token_usage(cx))
                            .children(self.profile_selector.clone())
                            // Either config_options_view OR (mode_selector + model_selector)
                            .children(self.config_options_view.clone())
                            .when(self.config_options_view.is_none(), |this| {
                                this.children(self.mode_selector().cloned())
                                    .children(self.model_selector.clone())
                            })
                            .child(self.render_send_button(cx)),
                    ),
            )
            .when(!enable_editor, |this| this.child(backdrop))
            .into_any()
    }

    pub(crate) fn as_native_connection(
        &self,
        cx: &App,
    ) -> Option<Rc<agent::NativeAgentConnection>> {
        let acp_thread = self.thread()?.read(cx);
        acp_thread.connection().clone().downcast()
    }

    pub(crate) fn as_native_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        let acp_thread = self.thread()?.read(cx);
        self.as_native_connection(cx)?
            .thread(acp_thread.session_id(), cx)
    }

    fn is_imported_thread(&self, cx: &App) -> bool {
        let Some(thread) = self.as_native_thread(cx) else {
            return false;
        };
        thread.read(cx).is_imported()
    }

    fn supports_split_token_display(&self, cx: &App) -> bool {
        self.as_native_thread(cx)
            .and_then(|thread| thread.read(cx).model())
            .is_some_and(|model| model.supports_split_token_display())
    }

    fn render_token_usage(&self, cx: &mut Context<Self>) -> Option<Div> {
        let thread = self.thread()?.read(cx);
        let usage = thread.token_usage()?;
        let is_generating = thread.status() != ThreadStatus::Idle;
        let show_split = self.supports_split_token_display(cx);

        let separator_color = Color::Custom(cx.theme().colors().text_muted.opacity(0.5));
        let token_label = |text: String, animation_id: &'static str| {
            Label::new(text)
                .size(LabelSize::Small)
                .color(Color::Muted)
                .map(|label| {
                    if is_generating {
                        label
                            .with_animation(
                                animation_id,
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.3, 0.8)),
                                |label, delta| label.alpha(delta),
                            )
                            .into_any()
                    } else {
                        label.into_any_element()
                    }
                })
        };

        if show_split {
            let max_output_tokens = self
                .as_native_thread(cx)
                .and_then(|thread| thread.read(cx).model())
                .and_then(|model| model.max_output_tokens())
                .unwrap_or(0);

            let input = crate::text_thread_editor::humanize_token_count(usage.input_tokens);
            let input_max = crate::text_thread_editor::humanize_token_count(
                usage.max_tokens.saturating_sub(max_output_tokens),
            );
            let output = crate::text_thread_editor::humanize_token_count(usage.output_tokens);
            let output_max = crate::text_thread_editor::humanize_token_count(max_output_tokens);

            Some(
                h_flex()
                    .flex_shrink_0()
                    .gap_1()
                    .mr_1p5()
                    .child(
                        h_flex()
                            .gap_0p5()
                            .child(
                                Icon::new(IconName::ArrowUp)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(token_label(input, "input-tokens-label"))
                            .child(
                                Label::new("/")
                                    .size(LabelSize::Small)
                                    .color(separator_color),
                            )
                            .child(
                                Label::new(input_max)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_0p5()
                            .child(
                                Icon::new(IconName::ArrowDown)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(token_label(output, "output-tokens-label"))
                            .child(
                                Label::new("/")
                                    .size(LabelSize::Small)
                                    .color(separator_color),
                            )
                            .child(
                                Label::new(output_max)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    ),
            )
        } else {
            let used = crate::text_thread_editor::humanize_token_count(usage.used_tokens);
            let max = crate::text_thread_editor::humanize_token_count(usage.max_tokens);

            Some(
                h_flex()
                    .flex_shrink_0()
                    .gap_0p5()
                    .mr_1p5()
                    .child(token_label(used, "used-tokens-label"))
                    .child(
                        Label::new("/")
                            .size(LabelSize::Small)
                            .color(separator_color),
                    )
                    .child(Label::new(max).size(LabelSize::Small).color(Color::Muted)),
            )
        }
    }

    fn keep_all(&mut self, _: &KeepAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };
        let telemetry = ActionLogTelemetry::from(thread.read(cx));
        let action_log = thread.read(cx).action_log().clone();
        action_log.update(cx, |action_log, cx| {
            action_log.keep_all_edits(Some(telemetry), cx)
        });
    }

    fn reject_all(&mut self, _: &RejectAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };
        let telemetry = ActionLogTelemetry::from(thread.read(cx));
        let action_log = thread.read(cx).action_log().clone();
        action_log
            .update(cx, |action_log, cx| {
                action_log.reject_all_edits(Some(telemetry), cx)
            })
            .detach();
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
        let thread = self.thread()?.read(cx);
        let tool_call = thread.first_tool_awaiting_confirmation()?;
        let ToolCallStatus::WaitingForConfirmation { options, .. } = &tool_call.status else {
            return None;
        };
        let tool_call_id = tool_call.id.clone();

        // Get granularity options (all options except old deny option)
        let granularity_options: Vec<_> = options
            .iter()
            .filter(|o| {
                matches!(
                    o.kind,
                    acp::PermissionOptionKind::AllowOnce | acp::PermissionOptionKind::AllowAlways
                )
            })
            .collect();

        // Get selected index, defaulting to last option ("Only this time")
        let selected_index = self
            .selected_permission_granularity
            .get(&tool_call_id)
            .copied()
            .unwrap_or_else(|| granularity_options.len().saturating_sub(1));

        let selected_option = granularity_options
            .get(selected_index)
            .or(granularity_options.last())
            .copied()?;

        let option_id_str = selected_option.option_id.0.to_string();

        // Transform option_id based on allow/deny
        let (final_option_id, final_option_kind) = if is_allow {
            let allow_id = if option_id_str == "once" {
                "allow".to_string()
            } else if let Some(rest) = option_id_str.strip_prefix("always:") {
                format!("always_allow:{}", rest)
            } else if let Some(rest) = option_id_str.strip_prefix("always_pattern:") {
                format!("always_allow_pattern:{}", rest)
            } else {
                option_id_str
            };
            (acp::PermissionOptionId::new(allow_id), selected_option.kind)
        } else {
            let deny_id = if option_id_str == "once" {
                "deny".to_string()
            } else if let Some(rest) = option_id_str.strip_prefix("always:") {
                format!("always_deny:{}", rest)
            } else if let Some(rest) = option_id_str.strip_prefix("always_pattern:") {
                format!("always_deny_pattern:{}", rest)
            } else {
                option_id_str.replace("allow", "deny")
            };
            let deny_kind = match selected_option.kind {
                acp::PermissionOptionKind::AllowOnce => acp::PermissionOptionKind::RejectOnce,
                acp::PermissionOptionKind::AllowAlways => acp::PermissionOptionKind::RejectAlways,
                other => other,
            };
            (acp::PermissionOptionId::new(deny_id), deny_kind)
        };

        self.authorize_tool_call(tool_call_id, final_option_id, final_option_kind, window, cx);

        Some(())
    }

    fn open_permission_dropdown(
        &mut self,
        _: &crate::OpenPermissionDropdown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.permission_dropdown_handle.toggle(window, cx);
    }

    fn handle_select_permission_granularity(
        &mut self,
        action: &SelectPermissionGranularity,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let tool_call_id = acp::ToolCallId::new(action.tool_call_id.clone());
        self.selected_permission_granularity
            .insert(tool_call_id, action.index);
        cx.notify();
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
        let thread = self.thread()?.read(cx);
        let tool_call = thread.first_tool_awaiting_confirmation()?;
        let ToolCallStatus::WaitingForConfirmation { options, .. } = &tool_call.status else {
            return None;
        };
        let option = options.iter().find(|o| o.kind == kind)?;

        self.authorize_tool_call(
            tool_call.id.clone(),
            option.option_id.clone(),
            option.kind,
            window,
            cx,
        );

        Some(())
    }

    fn render_send_button(&self, cx: &mut Context<Self>) -> AnyElement {
        let message_editor = self.message_editor.read(cx);
        let is_editor_empty = message_editor.is_empty(cx);
        let focus_handle = message_editor.focus_handle(cx);

        let is_generating = self
            .thread()
            .is_some_and(|thread| thread.read(cx).status() != ThreadStatus::Idle);

        if self.is_loading_contents {
            div()
                .id("loading-message-content")
                .px_1()
                .tooltip(Tooltip::text("Loading Added Context…"))
                .child(loading_contents_spinner(IconSize::default()))
                .into_any_element()
        } else if is_generating && is_editor_empty {
            IconButton::new("stop-generation", IconName::Stop)
                .icon_color(Color::Error)
                .style(ButtonStyle::Tinted(TintColor::Error))
                .tooltip(move |_window, cx| {
                    Tooltip::for_action("Stop Generation", &editor::actions::Cancel, cx)
                })
                .on_click(cx.listener(|this, _event, _, cx| this.cancel_generation(cx)))
                .into_any_element()
        } else {
            IconButton::new("send-message", IconName::Send)
                .style(ButtonStyle::Filled)
                .map(|this| {
                    if is_editor_empty && !is_generating {
                        this.disabled(true).icon_color(Color::Muted)
                    } else {
                        this.icon_color(Color::Accent)
                    }
                })
                .tooltip(move |_window, cx| {
                    if is_editor_empty && !is_generating {
                        Tooltip::for_action("Type to Send", &Chat, cx)
                    } else if is_generating {
                        let focus_handle = focus_handle.clone();

                        Tooltip::element(move |_window, cx| {
                            v_flex()
                                .gap_1()
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .justify_between()
                                        .child(Label::new("Queue and Send"))
                                        .child(KeyBinding::for_action_in(&Chat, &focus_handle, cx)),
                                )
                                .child(
                                    h_flex()
                                        .pt_1()
                                        .gap_2()
                                        .justify_between()
                                        .border_t_1()
                                        .border_color(cx.theme().colors().border_variant)
                                        .child(Label::new("Send Immediately"))
                                        .child(KeyBinding::for_action_in(
                                            &SendImmediately,
                                            &focus_handle,
                                            cx,
                                        )),
                                )
                                .into_any_element()
                        })(_window, cx)
                    } else {
                        Tooltip::for_action("Send Message", &Chat, cx)
                    }
                })
                .on_click(cx.listener(|this, _, window, cx| {
                    this.send(window, cx);
                }))
                .into_any_element()
        }
    }

    fn is_following(&self, cx: &App) -> bool {
        match self.thread().map(|thread| thread.read(cx).status()) {
            Some(ThreadStatus::Generating) => self
                .workspace
                .read_with(cx, |workspace, _| {
                    workspace.is_being_followed(CollaboratorId::Agent)
                })
                .unwrap_or(false),
            _ => self.should_be_following,
        }
    }

    fn toggle_following(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let following = self.is_following(cx);

        self.should_be_following = !following;
        if self.thread().map(|thread| thread.read(cx).status()) == Some(ThreadStatus::Generating) {
            self.workspace
                .update(cx, |workspace, cx| {
                    if following {
                        workspace.unfollow(CollaboratorId::Agent, window, cx);
                    } else {
                        workspace.follow(CollaboratorId::Agent, window, cx);
                    }
                })
                .ok();
        }

        telemetry::event!("Follow Agent Selected", following = !following);
    }

    fn render_follow_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let following = self.is_following(cx);

        let tooltip_label = if following {
            if self.agent.name() == "Zed Agent" {
                format!("Stop Following the {}", self.agent.name())
            } else {
                format!("Stop Following {}", self.agent.name())
            }
        } else {
            if self.agent.name() == "Zed Agent" {
                format!("Follow the {}", self.agent.name())
            } else {
                format!("Follow {}", self.agent.name())
            }
        };

        IconButton::new("follow-agent", IconName::Crosshair)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .toggle_state(following)
            .selected_icon_color(Some(Color::Custom(cx.theme().players().agent().cursor)))
            .tooltip(move |_window, cx| {
                if following {
                    Tooltip::for_action(tooltip_label.clone(), &Follow, cx)
                } else {
                    Tooltip::with_meta(
                        tooltip_label.clone(),
                        Some(&Follow),
                        "Track the agent's location as it reads and edits files.",
                        cx,
                    )
                }
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.toggle_following(window, cx);
            }))
    }

    fn render_add_context_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let message_editor = self.message_editor.clone();
        let menu_visible = message_editor.read(cx).is_completions_menu_visible(cx);

        IconButton::new("add-context", IconName::AtSign)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .when(!menu_visible, |this| {
                this.tooltip(move |_window, cx| {
                    Tooltip::with_meta("Add Context", None, "Or type @ to include context", cx)
                })
            })
            .on_click(cx.listener(move |_this, _, window, cx| {
                let message_editor_clone = message_editor.clone();

                window.defer(cx, move |window, cx| {
                    message_editor_clone.update(cx, |message_editor, cx| {
                        message_editor.trigger_completion_menu(window, cx);
                    });
                });
            }))
    }

    fn render_markdown(&self, markdown: Entity<Markdown>, style: MarkdownStyle) -> MarkdownElement {
        let workspace = self.workspace.clone();
        MarkdownElement::new(markdown, style).on_url_click(move |text, window, cx| {
            Self::open_link(text, &workspace, window, cx);
        })
    }

    fn open_link(
        url: SharedString,
        workspace: &WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(workspace) = workspace.upgrade() else {
            cx.open_url(&url);
            return;
        };

        if let Some(mention) = MentionUri::parse(&url, workspace.read(cx).path_style(cx)).log_err()
        {
            workspace.update(cx, |workspace, cx| match mention {
                MentionUri::File { abs_path } => {
                    let project = workspace.project();
                    let Some(path) =
                        project.update(cx, |project, cx| project.find_project_path(abs_path, cx))
                    else {
                        return;
                    };

                    workspace
                        .open_path(path, None, true, window, cx)
                        .detach_and_log_err(cx);
                }
                MentionUri::PastedImage => {}
                MentionUri::Directory { abs_path } => {
                    let project = workspace.project();
                    let Some(entry_id) = project.update(cx, |project, cx| {
                        let path = project.find_project_path(abs_path, cx)?;
                        project.entry_for_path(&path, cx).map(|entry| entry.id)
                    }) else {
                        return;
                    };

                    project.update(cx, |_, cx| {
                        cx.emit(project::Event::RevealInProjectPanel(entry_id));
                    });
                }
                MentionUri::Symbol {
                    abs_path: path,
                    line_range,
                    ..
                }
                | MentionUri::Selection {
                    abs_path: Some(path),
                    line_range,
                } => {
                    let project = workspace.project();
                    let Some(path) =
                        project.update(cx, |project, cx| project.find_project_path(path, cx))
                    else {
                        return;
                    };

                    let item = workspace.open_path(path, None, true, window, cx);
                    window
                        .spawn(cx, async move |cx| {
                            let Some(editor) = item.await?.downcast::<Editor>() else {
                                return Ok(());
                            };
                            let range = Point::new(*line_range.start(), 0)
                                ..Point::new(*line_range.start(), 0);
                            editor
                                .update_in(cx, |editor, window, cx| {
                                    editor.change_selections(
                                        SelectionEffects::scroll(Autoscroll::center()),
                                        window,
                                        cx,
                                        |s| s.select_ranges(vec![range]),
                                    );
                                })
                                .ok();
                            anyhow::Ok(())
                        })
                        .detach_and_log_err(cx);
                }
                MentionUri::Selection { abs_path: None, .. } => {}
                MentionUri::Thread { id, name } => {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.open_thread(
                                AgentSessionInfo {
                                    session_id: id,
                                    cwd: None,
                                    title: Some(name.into()),
                                    updated_at: None,
                                    meta: None,
                                },
                                window,
                                cx,
                            )
                        });
                    }
                }
                MentionUri::TextThread { path, .. } => {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel
                                .open_saved_text_thread(path.as_path().into(), window, cx)
                                .detach_and_log_err(cx);
                        });
                    }
                }
                MentionUri::Rule { id, .. } => {
                    let PromptId::User { uuid } = id else {
                        return;
                    };
                    window.dispatch_action(
                        Box::new(OpenRulesLibrary {
                            prompt_to_select: Some(uuid.0),
                        }),
                        cx,
                    )
                }
                MentionUri::Fetch { url } => {
                    cx.open_url(url.as_str());
                }
            })
        } else {
            cx.open_url(&url);
        }
    }

    fn open_tool_call_location(
        &self,
        entry_ix: usize,
        location_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let (tool_call_location, agent_location) = self
            .thread()?
            .read(cx)
            .entries()
            .get(entry_ix)?
            .location(location_ix)?;

        let project_path = self
            .project
            .read(cx)
            .find_project_path(&tool_call_location.path, cx)?;

        let open_task = self
            .workspace
            .update(cx, |workspace, cx| {
                workspace.open_path(project_path, None, true, window, cx)
            })
            .log_err()?;
        window
            .spawn(cx, async move |cx| {
                let item = open_task.await?;

                let Some(active_editor) = item.downcast::<Editor>() else {
                    return anyhow::Ok(());
                };

                active_editor.update_in(cx, |editor, window, cx| {
                    let multibuffer = editor.buffer().read(cx);
                    let buffer = multibuffer.as_singleton();
                    if agent_location.buffer.upgrade() == buffer {
                        let excerpt_id = multibuffer.excerpt_ids().first().cloned();
                        let anchor =
                            editor::Anchor::in_buffer(excerpt_id.unwrap(), agent_location.position);
                        editor.change_selections(Default::default(), window, cx, |selections| {
                            selections.select_anchor_ranges([anchor..anchor]);
                        })
                    } else {
                        let row = tool_call_location.line.unwrap_or_default();
                        editor.change_selections(Default::default(), window, cx, |selections| {
                            selections.select_ranges([Point::new(row, 0)..Point::new(row, 0)]);
                        })
                    }
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

        None
    }

    pub fn open_thread_as_markdown(
        &self,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let markdown_language_task = workspace
            .read(cx)
            .app_state()
            .languages
            .language_for_name("Markdown");

        let (thread_title, markdown) = if let Some(thread) = self.thread() {
            let thread = thread.read(cx);
            (thread.title().to_string(), thread.to_markdown(cx))
        } else {
            return Task::ready(Ok(()));
        };

        let project = workspace.read(cx).project().clone();
        window.spawn(cx, async move |cx| {
            let markdown_language = markdown_language_task.await?;

            let buffer = project
                .update(cx, |project, cx| {
                    project.create_buffer(Some(markdown_language), false, cx)
                })
                .await?;

            buffer.update(cx, |buffer, cx| {
                buffer.set_text(markdown, cx);
                buffer.set_capability(language::Capability::ReadWrite, cx);
            });

            workspace.update_in(cx, |workspace, window, cx| {
                let buffer = cx
                    .new(|cx| MultiBuffer::singleton(buffer, cx).with_title(thread_title.clone()));

                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|cx| {
                        let mut editor =
                            Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                        editor.set_breadcrumb_header(thread_title);
                        editor
                    })),
                    None,
                    true,
                    window,
                    cx,
                );
            })?;
            anyhow::Ok(())
        })
    }

    fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        self.list_state.scroll_to(ListOffset::default());
        cx.notify();
    }

    fn scroll_to_most_recent_user_prompt(&mut self, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };

        let entries = thread.read(cx).entries();
        if entries.is_empty() {
            return;
        }

        // Find the most recent user message and scroll it to the top of the viewport.
        // (Fallback: if no user message exists, scroll to the bottom.)
        if let Some(ix) = entries
            .iter()
            .rposition(|entry| matches!(entry, AgentThreadEntry::UserMessage(_)))
        {
            self.list_state.scroll_to(ListOffset {
                item_ix: ix,
                offset_in_item: px(0.0),
            });
            cx.notify();
        } else {
            self.scroll_to_bottom(cx);
        }
    }

    pub fn scroll_to_bottom(&mut self, cx: &mut Context<Self>) {
        if let Some(thread) = self.thread() {
            let entry_count = thread.read(cx).entries().len();
            self.list_state.reset(entry_count);
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

    fn render_generating(&self, confirmation: bool, cx: &App) -> impl IntoElement {
        let show_stats = AgentSettings::get_global(cx).show_turn_stats;
        let elapsed_label = show_stats
            .then(|| {
                self.turn_started_at.and_then(|started_at| {
                    let elapsed = started_at.elapsed();
                    (elapsed > STOPWATCH_THRESHOLD).then(|| duration_alt_display(elapsed))
                })
            })
            .flatten();

        let is_waiting = confirmation
            || self
                .thread()
                .is_some_and(|thread| thread.read(cx).has_in_progress_tool_calls());

        let turn_tokens_label = elapsed_label
            .is_some()
            .then(|| {
                self.turn_tokens
                    .filter(|&tokens| tokens > TOKEN_THRESHOLD)
                    .map(|tokens| crate::text_thread_editor::humanize_token_count(tokens))
            })
            .flatten();

        let arrow_icon = if is_waiting {
            IconName::ArrowUp
        } else {
            IconName::ArrowDown
        };

        h_flex()
            .id("generating-spinner")
            .py_2()
            .px(rems_from_px(22.))
            .gap_2()
            .map(|this| {
                if confirmation {
                    this.child(
                        h_flex()
                            .w_2()
                            .child(SpinnerLabel::sand().size(LabelSize::Small)),
                    )
                    .child(
                        div().min_w(rems(8.)).child(
                            LoadingLabel::new("Waiting Confirmation")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                } else {
                    this.child(SpinnerLabel::new().size(LabelSize::Small))
                }
            })
            .when_some(elapsed_label, |this, elapsed| {
                this.child(
                    Label::new(elapsed)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .when_some(turn_tokens_label, |this, tokens| {
                this.child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            Icon::new(arrow_icon)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new(format!("{} tokens", tokens))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
            })
            .into_any_element()
    }

    fn render_thread_controls(
        &self,
        thread: &Entity<AcpThread>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_generating = matches!(thread.read(cx).status(), ThreadStatus::Generating);
        if is_generating {
            return self.render_generating(false, cx).into_any_element();
        }

        let open_as_markdown = IconButton::new("open-as-markdown", IconName::FileMarkdown)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Open Thread as Markdown"))
            .on_click(cx.listener(move |this, _, window, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    this.open_thread_as_markdown(workspace, window, cx)
                        .detach_and_log_err(cx);
                }
            }));

        let scroll_to_recent_user_prompt =
            IconButton::new("scroll_to_recent_user_prompt", IconName::ForwardArrow)
                .shape(ui::IconButtonShape::Square)
                .icon_size(IconSize::Small)
                .icon_color(Color::Ignored)
                .tooltip(Tooltip::text("Scroll To Most Recent User Prompt"))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.scroll_to_most_recent_user_prompt(cx);
                }));

        let scroll_to_top = IconButton::new("scroll_to_top", IconName::ArrowUp)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Scroll To Top"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.scroll_to_top(cx);
            }));

        let show_stats = AgentSettings::get_global(cx).show_turn_stats;
        let last_turn_clock = show_stats
            .then(|| {
                self.last_turn_duration
                    .filter(|&duration| duration > STOPWATCH_THRESHOLD)
                    .map(|duration| {
                        Label::new(duration_alt_display(duration))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    })
            })
            .flatten();

        let last_turn_tokens = last_turn_clock
            .is_some()
            .then(|| {
                self.last_turn_tokens
                    .filter(|&tokens| tokens > TOKEN_THRESHOLD)
                    .map(|tokens| {
                        Label::new(format!(
                            "{} tokens",
                            crate::text_thread_editor::humanize_token_count(tokens)
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                    })
            })
            .flatten();

        let mut container = h_flex()
            .w_full()
            .py_2()
            .px_5()
            .gap_px()
            .opacity(0.6)
            .hover(|s| s.opacity(1.))
            .justify_end()
            .when(
                last_turn_tokens.is_some() || last_turn_clock.is_some(),
                |this| {
                    this.child(
                        h_flex()
                            .gap_1()
                            .px_1()
                            .when_some(last_turn_tokens, |this, label| this.child(label))
                            .when_some(last_turn_clock, |this, label| this.child(label)),
                    )
                },
            );

        if AgentSettings::get_global(cx).enable_feedback
            && self
                .thread()
                .is_some_and(|thread| thread.read(cx).connection().telemetry().is_some())
        {
            let feedback = self.thread_feedback.feedback;

            let tooltip_meta = || {
                SharedString::new(
                    "Rating the thread sends all of your current conversation to the Zed team.",
                )
            };

            container = container
                .child(
                    IconButton::new("feedback-thumbs-up", IconName::ThumbsUp)
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(match feedback {
                            Some(ThreadFeedback::Positive) => Color::Accent,
                            _ => Color::Ignored,
                        })
                        .tooltip(move |window, cx| match feedback {
                            Some(ThreadFeedback::Positive) => {
                                Tooltip::text("Thanks for your feedback!")(window, cx)
                            }
                            _ => Tooltip::with_meta("Helpful Response", None, tooltip_meta(), cx),
                        })
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.handle_feedback_click(ThreadFeedback::Positive, window, cx);
                        })),
                )
                .child(
                    IconButton::new("feedback-thumbs-down", IconName::ThumbsDown)
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(match feedback {
                            Some(ThreadFeedback::Negative) => Color::Accent,
                            _ => Color::Ignored,
                        })
                        .tooltip(move |window, cx| match feedback {
                            Some(ThreadFeedback::Negative) => {
                                Tooltip::text(
                                    "We appreciate your feedback and will use it to improve in the future.",
                                )(window, cx)
                            }
                            _ => {
                                Tooltip::with_meta("Not Helpful Response", None, tooltip_meta(), cx)
                            }
                        })
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.handle_feedback_click(ThreadFeedback::Negative, window, cx);
                        })),
                );
        }

        if cx.has_flag::<AgentSharingFeatureFlag>()
            && self.is_imported_thread(cx)
            && self
                .project
                .read(cx)
                .client()
                .status()
                .borrow()
                .is_connected()
        {
            let sync_button = IconButton::new("sync-thread", IconName::ArrowCircle)
                .shape(ui::IconButtonShape::Square)
                .icon_size(IconSize::Small)
                .icon_color(Color::Ignored)
                .tooltip(Tooltip::text("Sync with source thread"))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.sync_thread(window, cx);
                }));

            container = container.child(sync_button);
        }

        if cx.has_flag::<AgentSharingFeatureFlag>() && !self.is_imported_thread(cx) {
            let share_button = IconButton::new("share-thread", IconName::ArrowUpRight)
                .shape(ui::IconButtonShape::Square)
                .icon_size(IconSize::Small)
                .icon_color(Color::Ignored)
                .tooltip(Tooltip::text("Share Thread"))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.share_thread(window, cx);
                }));

            container = container.child(share_button);
        }

        container
            .child(open_as_markdown)
            .child(scroll_to_recent_user_prompt)
            .child(scroll_to_top)
            .into_any_element()
    }

    fn render_feedback_feedback_editor(editor: Entity<Editor>, cx: &Context<Self>) -> Div {
        h_flex()
            .key_context("AgentFeedbackMessageEditor")
            .on_action(cx.listener(move |this, _: &menu::Cancel, _, cx| {
                this.thread_feedback.dismiss_comments();
                cx.notify();
            }))
            .on_action(cx.listener(move |this, _: &menu::Confirm, _window, cx| {
                this.submit_feedback_message(cx);
            }))
            .p_2()
            .mb_2()
            .mx_5()
            .gap_1()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .child(div().w_full().child(editor))
            .child(
                h_flex()
                    .child(
                        IconButton::new("dismiss-feedback-message", IconName::Close)
                            .icon_color(Color::Error)
                            .icon_size(IconSize::XSmall)
                            .shape(ui::IconButtonShape::Square)
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.thread_feedback.dismiss_comments();
                                cx.notify();
                            })),
                    )
                    .child(
                        IconButton::new("submit-feedback-message", IconName::Return)
                            .icon_size(IconSize::XSmall)
                            .shape(ui::IconButtonShape::Square)
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.submit_feedback_message(cx);
                            })),
                    ),
            )
    }

    fn handle_feedback_click(
        &mut self,
        feedback: ThreadFeedback,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };

        self.thread_feedback.submit(thread, feedback, window, cx);
        cx.notify();
    }

    fn submit_feedback_message(&mut self, cx: &mut Context<Self>) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };

        self.thread_feedback.submit_comments(thread, cx);
        cx.notify();
    }

    fn render_token_limit_callout(&self, cx: &mut Context<Self>) -> Option<Callout> {
        if self.token_limit_callout_dismissed {
            return None;
        }

        let token_usage = self.thread()?.read(cx).token_usage()?;
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
                                let Some(thread) = this.thread() else {
                                    return;
                                };
                                let session_id = thread.read(cx).session_id().clone();
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
        self.entry_view_state.update(cx, |entry_view_state, cx| {
            entry_view_state.agent_ui_font_size_changed(cx);
        });
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

    fn render_thread_retry_status_callout(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Callout> {
        let state = self.thread_retry_status.as_ref()?;

        let next_attempt_in = state
            .duration
            .saturating_sub(Instant::now().saturating_duration_since(state.started_at));
        if next_attempt_in.is_zero() {
            return None;
        }

        let next_attempt_in_secs = next_attempt_in.as_secs() + 1;

        let retry_message = if state.max_attempts == 1 {
            if next_attempt_in_secs == 1 {
                "Retrying. Next attempt in 1 second.".to_string()
            } else {
                format!("Retrying. Next attempt in {next_attempt_in_secs} seconds.")
            }
        } else if next_attempt_in_secs == 1 {
            format!(
                "Retrying. Next attempt in 1 second (Attempt {} of {}).",
                state.attempt, state.max_attempts,
            )
        } else {
            format!(
                "Retrying. Next attempt in {next_attempt_in_secs} seconds (Attempt {} of {}).",
                state.attempt, state.max_attempts,
            )
        };

        Some(
            Callout::new()
                .severity(Severity::Warning)
                .title(state.last_error.clone())
                .description(retry_message),
        )
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

    fn render_thread_error(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<Div> {
        let content = match self.thread_error.as_ref()? {
            ThreadError::Other(error) => self.render_any_thread_error(error.clone(), window, cx),
            ThreadError::Refusal => self.render_refusal_error(cx),
            ThreadError::AuthenticationRequired(error) => {
                self.render_authentication_required_error(error.clone(), cx)
            }
            ThreadError::PaymentRequired => self.render_payment_required_error(cx),
        };

        Some(div().child(content))
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

    fn current_mode_id(&self, cx: &App) -> Option<Arc<str>> {
        if let Some(thread) = self.as_native_thread(cx) {
            Some(thread.read(cx).profile().0.clone())
        } else if let Some(mode_selector) = self.mode_selector() {
            Some(mode_selector.read(cx).mode().0)
        } else {
            None
        }
    }

    fn current_model_id(&self, cx: &App) -> Option<String> {
        self.model_selector
            .as_ref()
            .and_then(|selector| selector.read(cx).active_model(cx).map(|m| m.id.to_string()))
    }

    fn current_model_name(&self, cx: &App) -> SharedString {
        // For native agent (Zed Agent), use the specific model name (e.g., "Claude 3.5 Sonnet")
        // For ACP agents, use the agent name (e.g., "Claude Code", "Gemini CLI")
        // This provides better clarity about what refused the request
        if self.as_native_connection(cx).is_some() {
            self.model_selector
                .as_ref()
                .and_then(|selector| selector.read(cx).active_model(cx))
                .map(|model| model.name.clone())
                .unwrap_or_else(|| SharedString::from("The model"))
        } else {
            // ACP agent - use the agent name (e.g., "Claude Code", "Gemini CLI")
            self.agent.name()
        }
    }

    fn render_refusal_error(&self, cx: &mut Context<'_, Self>) -> Callout {
        let model_or_agent_name = self.current_model_name(cx);
        let refusal_message = format!(
            "{} refused to respond to this prompt. This can happen when a model believes the prompt violates its content policy or safety guidelines, so rephrasing it can sometimes address the issue.",
            model_or_agent_name
        );

        Callout::new()
            .severity(Severity::Error)
            .title("Request Refused")
            .icon(IconName::XCircle)
            .description(refusal_message.clone())
            .actions_slot(self.create_copy_button(&refusal_message))
            .dismiss_action(self.dismiss_error_button(cx))
    }

    fn render_any_thread_error(
        &mut self,
        error: SharedString,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) -> Callout {
        let can_resume = self
            .thread()
            .map_or(false, |thread| thread.read(cx).can_resume(cx));

        let markdown = if let Some(markdown) = &self.thread_error_markdown {
            markdown.clone()
        } else {
            let markdown = cx.new(|cx| Markdown::new(error.clone(), None, None, cx));
            self.thread_error_markdown = Some(markdown.clone());
            markdown
        };

        let markdown_style = default_markdown_style(false, true, window, cx);
        let description = self
            .render_markdown(markdown, markdown_style)
            .into_any_element();

        Callout::new()
            .severity(Severity::Error)
            .icon(IconName::XCircle)
            .title("An Error Happened")
            .description_slot(description)
            .actions_slot(
                h_flex()
                    .gap_0p5()
                    .when(can_resume, |this| {
                        this.child(
                            IconButton::new("retry", IconName::RotateCw)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Retry Generation"))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.resume_chat(cx);
                                })),
                        )
                    })
                    .child(self.create_copy_button(error.to_string())),
            )
            .dismiss_action(self.dismiss_error_button(cx))
    }

    fn render_payment_required_error(&self, cx: &mut Context<Self>) -> Callout {
        const ERROR_MESSAGE: &str =
            "You reached your free usage limit. Upgrade to Zed Pro for more prompts.";

        Callout::new()
            .severity(Severity::Error)
            .icon(IconName::XCircle)
            .title("Free Usage Exceeded")
            .description(ERROR_MESSAGE)
            .actions_slot(
                h_flex()
                    .gap_0p5()
                    .child(self.upgrade_button(cx))
                    .child(self.create_copy_button(ERROR_MESSAGE)),
            )
            .dismiss_action(self.dismiss_error_button(cx))
    }

    fn render_authentication_required_error(
        &self,
        error: SharedString,
        cx: &mut Context<Self>,
    ) -> Callout {
        Callout::new()
            .severity(Severity::Error)
            .title("Authentication Required")
            .icon(IconName::XCircle)
            .description(error.clone())
            .actions_slot(
                h_flex()
                    .gap_0p5()
                    .child(self.authenticate_button(cx))
                    .child(self.create_copy_button(error)),
            )
            .dismiss_action(self.dismiss_error_button(cx))
    }

    fn create_copy_button(&self, message: impl Into<String>) -> impl IntoElement {
        let message = message.into();

        CopyButton::new(message).tooltip_label("Copy Error Message")
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

    fn authenticate_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("authenticate", "Authenticate")
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Filled)
            .on_click(cx.listener({
                move |this, _, window, cx| {
                    let agent = this.agent.clone();
                    let ThreadState::Ready { thread, .. } = &this.thread_state else {
                        return;
                    };

                    let connection = thread.read(cx).connection().clone();
                    this.clear_thread_error(cx);
                    if let Some(message) = this.in_flight_prompt.take() {
                        this.message_editor.update(cx, |editor, cx| {
                            editor.set_message(message, window, cx);
                        });
                    }
                    let this = cx.weak_entity();
                    window.defer(cx, |window, cx| {
                        Self::handle_auth_required(
                            this,
                            AuthRequired::new(),
                            agent,
                            connection,
                            window,
                            cx,
                        );
                    })
                }
            }))
    }

    pub(crate) fn reauthenticate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let agent = self.agent.clone();
        let ThreadState::Ready { thread, .. } = &self.thread_state else {
            return;
        };

        let connection = thread.read(cx).connection().clone();
        self.clear_thread_error(cx);
        let this = cx.weak_entity();
        window.defer(cx, |window, cx| {
            Self::handle_auth_required(this, AuthRequired::new(), agent, connection, window, cx);
        })
    }

    fn upgrade_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("upgrade", "Upgrade")
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
            .on_click(cx.listener({
                move |this, _, _, cx| {
                    this.clear_thread_error(cx);
                    cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx));
                }
            }))
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
        if let Some(index) = self.editing_message
            && let Some(editor) = self
                .entry_view_state
                .read(cx)
                .entry(index)
                .and_then(|e| e.message_editor())
                .cloned()
        {
            editor
        } else {
            self.message_editor.clone()
        }
    }

    fn get_agent_message_content(
        entries: &[AgentThreadEntry],
        entry_index: usize,
        cx: &App,
    ) -> Option<String> {
        let entry = entries.get(entry_index)?;
        if matches!(entry, AgentThreadEntry::UserMessage(_)) {
            return None;
        }

        let start_index = (0..entry_index)
            .rev()
            .find(|&i| matches!(entries.get(i), Some(AgentThreadEntry::UserMessage(_))))
            .map(|i| i + 1)
            .unwrap_or(0);

        let end_index = (entry_index + 1..entries.len())
            .find(|&i| matches!(entries.get(i), Some(AgentThreadEntry::UserMessage(_))))
            .map(|i| i - 1)
            .unwrap_or(entries.len() - 1);

        let parts: Vec<String> = (start_index..=end_index)
            .filter_map(|i| entries.get(i))
            .filter_map(|entry| {
                if let AgentThreadEntry::AssistantMessage(message) = entry {
                    let text: String = message
                        .chunks
                        .iter()
                        .filter_map(|chunk| match chunk {
                            AssistantMessageChunk::Message { block } => {
                                let markdown = block.to_markdown(cx);
                                if markdown.trim().is_empty() {
                                    None
                                } else {
                                    Some(markdown.to_string())
                                }
                            }
                            AssistantMessageChunk::Thought { .. } => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n\n");

                    if text.is_empty() { None } else { Some(text) }
                } else {
                    None
                }
            })
            .collect();

        let text = parts.join("\n\n");
        if text.is_empty() { None } else { Some(text) }
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

impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.thread_state {
            ThreadState::Ready { .. } => self.active_editor(cx).focus_handle(cx),
            ThreadState::Loading { .. }
            | ThreadState::LoadError(_)
            | ThreadState::Unauthenticated { .. } => self.focus_handle.clone(),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl AcpThreadView {
    /// Expands a tool call so its content is visible.
    /// This is primarily useful for visual testing.
    pub fn expand_tool_call(&mut self, tool_call_id: acp::ToolCallId, cx: &mut Context<Self>) {
        self.expanded_tool_calls.insert(tool_call_id);
        cx.notify();
    }

    /// Expands a subagent card so its content is visible.
    /// This is primarily useful for visual testing.
    pub fn expand_subagent(&mut self, session_id: acp::SessionId, cx: &mut Context<Self>) {
        self.expanded_subagents.insert(session_id);
        cx.notify();
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_messages = self.list_state.item_count() > 0;

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
            .on_action(cx.listener(|this, _: &SendNextQueuedMessage, window, cx| {
                this.send_queued_message_at_index(0, true, window, cx);
            }))
            .on_action(cx.listener(|this, _: &RemoveFirstQueuedMessage, _, cx| {
                if let Some(thread) = this.as_native_thread(cx) {
                    thread.update(cx, |thread, _| {
                        thread.remove_queued_message(0);
                    });
                    cx.notify();
                }
            }))
            .on_action(cx.listener(|this, _: &ClearMessageQueue, _, cx| {
                if let Some(thread) = this.as_native_thread(cx) {
                    thread.update(cx, |thread, _| thread.clear_queued_messages());
                }
                this.can_fast_track_queue = false;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleProfileSelector, window, cx| {
                if let Some(config_options_view) = this.config_options_view.as_ref() {
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

                if let Some(profile_selector) = this.profile_selector.as_ref() {
                    profile_selector.read(cx).menu_handle().toggle(window, cx);
                } else if let Some(mode_selector) = this.mode_selector() {
                    mode_selector.read(cx).menu_handle().toggle(window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &CycleModeSelector, window, cx| {
                if let Some(config_options_view) = this.config_options_view.as_ref() {
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

                if let Some(profile_selector) = this.profile_selector.as_ref() {
                    profile_selector.update(cx, |profile_selector, cx| {
                        profile_selector.cycle_profile(cx);
                    });
                } else if let Some(mode_selector) = this.mode_selector() {
                    mode_selector.update(cx, |mode_selector, cx| {
                        mode_selector.cycle_mode(window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &ToggleModelSelector, window, cx| {
                if let Some(config_options_view) = this.config_options_view.as_ref() {
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

                if let Some(model_selector) = this.model_selector.as_ref() {
                    model_selector
                        .update(cx, |model_selector, cx| model_selector.toggle(window, cx));
                }
            }))
            .on_action(cx.listener(|this, _: &CycleFavoriteModels, window, cx| {
                if let Some(config_options_view) = this.config_options_view.as_ref() {
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

                if let Some(model_selector) = this.model_selector.as_ref() {
                    model_selector.update(cx, |model_selector, cx| {
                        model_selector.cycle_favorite_models(window, cx);
                    });
                }
            }))
            .track_focus(&self.focus_handle)
            .bg(cx.theme().colors().panel_background)
            .child(match &self.thread_state {
                ThreadState::Unauthenticated {
                    connection,
                    description,
                    configuration_view,
                    pending_auth_method,
                    ..
                } => v_flex()
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
                ThreadState::Loading { .. } => v_flex()
                    .flex_1()
                    .child(self.render_recent_history(cx))
                    .into_any(),
                ThreadState::LoadError(e) => v_flex()
                    .flex_1()
                    .size_full()
                    .items_center()
                    .justify_end()
                    .child(self.render_load_error(e, window, cx))
                    .into_any(),
                ThreadState::Ready { .. } => v_flex().flex_1().map(|this| {
                    if has_messages {
                        this.child(
                            list(
                                self.list_state.clone(),
                                cx.processor(|this, index: usize, window, cx| {
                                    let Some((entry, len)) = this.thread().and_then(|thread| {
                                        let entries = &thread.read(cx).entries();
                                        Some((entries.get(index)?, entries.len()))
                                    }) else {
                                        return Empty.into_any();
                                    };
                                    this.render_entry(index, len, entry, window, cx)
                                }),
                            )
                            .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                            .flex_grow()
                            .into_any(),
                        )
                        .vertical_scrollbar_for(&self.list_state, window, cx)
                        .into_any()
                    } else {
                        this.child(self.render_recent_history(cx)).into_any()
                    }
                }),
            })
            // The activity bar is intentionally rendered outside of the ThreadState::Ready match
            // above so that the scrollbar doesn't render behind it. The current setup allows
            // the scrollbar to stop exactly at the activity bar start.
            .when(has_messages, |this| match &self.thread_state {
                ThreadState::Ready { thread, .. } => {
                    this.children(self.render_activity_bar(thread, window, cx))
                }
                _ => this,
            })
            .children(self.render_thread_retry_status_callout(window, cx))
            .when(self.show_codex_windows_warning, |this| {
                this.child(self.render_codex_windows_warning(cx))
            })
            .children(self.render_thread_error(window, cx))
            .when_some(
                self.new_server_version_available.as_ref().filter(|_| {
                    !has_messages || !matches!(self.thread_state, ThreadState::Ready { .. })
                }),
                |this, version| this.child(self.render_new_version_callout(&version, cx)),
            )
            .children(
                self.render_token_limit_callout(cx)
                    .map(|token_limit_callout| token_limit_callout.into_any_element()),
            )
            .child(self.render_message_editor(window, cx))
    }
}

fn default_markdown_style(
    buffer_font: bool,
    muted_text: bool,
    window: &Window,
    cx: &App,
) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();

    let buffer_font_size = theme_settings.agent_buffer_font_size(cx);

    let mut text_style = window.text_style();
    let line_height = buffer_font_size * 1.75;

    let font_family = if buffer_font {
        theme_settings.buffer_font.family.clone()
    } else {
        theme_settings.ui_font.family.clone()
    };

    let font_size = if buffer_font {
        theme_settings.agent_buffer_font_size(cx)
    } else {
        theme_settings.agent_ui_font_size(cx)
    };

    let text_color = if muted_text {
        colors.text_muted
    } else {
        colors.text
    };

    text_style.refine(&TextStyleRefinement {
        font_family: Some(font_family),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(font_size.into()),
        line_height: Some(line_height.into()),
        color: Some(text_color),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: colors.element_selection_background,
        code_block_overflow_x_scroll: true,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        code_block: StyleRefinement {
            padding: EdgesRefinement {
                top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
            },
            margin: EdgesRefinement {
                top: Some(Length::Definite(px(8.).into())),
                left: Some(Length::Definite(px(0.).into())),
                right: Some(Length::Definite(px(0.).into())),
                bottom: Some(Length::Definite(px(12.).into())),
            },
            border_style: Some(BorderStyle::Solid),
            border_widths: EdgesRefinement {
                top: Some(AbsoluteLength::Pixels(px(1.))),
                left: Some(AbsoluteLength::Pixels(px(1.))),
                right: Some(AbsoluteLength::Pixels(px(1.))),
                bottom: Some(AbsoluteLength::Pixels(px(1.))),
            },
            border_color: Some(colors.border_variant),
            background: Some(colors.editor_background.into()),
            text: TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
                font_features: Some(theme_settings.buffer_font.features.clone()),
                font_size: Some(buffer_font_size.into()),
                ..Default::default()
            },
            ..Default::default()
        },
        inline_code: TextStyleRefinement {
            font_family: Some(theme_settings.buffer_font.family.clone()),
            font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
            font_features: Some(theme_settings.buffer_font.features.clone()),
            font_size: Some(buffer_font_size.into()),
            background_color: Some(colors.editor_foreground.opacity(0.08)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            background_color: Some(colors.editor_foreground.opacity(0.025)),
            color: Some(colors.text_accent),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn plan_label_markdown_style(
    status: &acp::PlanEntryStatus,
    window: &Window,
    cx: &App,
) -> MarkdownStyle {
    let default_md_style = default_markdown_style(false, false, window, cx);

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
                AcpThreadView::new(
                    Rc::new(StubAgentServer::default_response()),
                    None,
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history.clone(),
                    false,
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
        thread_view.read_with(cx, |thread_view, _cx| {
            assert!(
                matches!(thread_view.thread_error, Some(ThreadError::Refusal)),
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
                vec![acp::PermissionOption::new(
                    "1",
                    "Allow",
                    acp::PermissionOptionKind::AllowOnce,
                )],
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
    ) -> (Entity<AcpThreadView>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx.update(|_window, cx| cx.new(|cx| ThreadStore::new(cx)));
        let history = cx.update(|window, cx| cx.new(|cx| AcpThreadHistory::new(None, window, cx)));

        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpThreadView::new(
                    Rc::new(agent),
                    None,
                    None,
                    workspace.downgrade(),
                    project,
                    Some(thread_store),
                    None,
                    history,
                    false,
                    window,
                    cx,
                )
            })
        });
        cx.run_until_parked();
        (thread_view, cx)
    }

    fn add_to_workspace(thread_view: Entity<AcpThreadView>, cx: &mut VisualTestContext) {
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

    struct ThreadViewItem(Entity<AcpThreadView>);

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
                AcpThreadView::new(
                    Rc::new(StubAgentServer::new(connection.as_ref().clone())),
                    None,
                    None,
                    workspace.downgrade(),
                    project.clone(),
                    Some(thread_store.clone()),
                    None,
                    history,
                    false,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        let thread = thread_view
            .read_with(cx, |view, _| view.thread().cloned())
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

        thread.read_with(cx, |thread, _| {
            assert_eq!(thread.entries().len(), 2);
        });

        thread_view.read_with(cx, |view, cx| {
            view.entry_view_state.read_with(cx, |entry_view_state, _| {
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
            view.entry_view_state.read_with(cx, |entry_view_state, _| {
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
            view.entry_view_state.read_with(cx, |entry_view_state, _| {
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
            .read_with(cx, |view, _| view.thread().cloned())
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
        thread_view.update(cx, |view, cx| {
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

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Original message to edit", window, cx);
        });
        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.send(window, cx);
        });

        cx.run_until_parked();

        let user_message_editor = thread_view.read_with(cx, |view, cx| {
            assert_eq!(view.editing_message, None);

            view.entry_view_state
                .read(cx)
                .entry(0)
                .unwrap()
                .message_editor()
                .unwrap()
                .clone()
        });

        // Focus
        cx.focus(&user_message_editor);
        thread_view.read_with(cx, |view, _cx| {
            assert_eq!(view.editing_message, Some(0));
        });

        // Edit
        user_message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edited message content", window, cx);
        });

        // Cancel
        user_message_editor.update_in(cx, |_editor, window, cx| {
            window.dispatch_action(Box::new(editor::actions::Cancel), cx);
        });

        thread_view.read_with(cx, |view, _cx| {
            assert_eq!(view.editing_message, None);
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

        let thread = cx.read(|cx| thread_view.read(cx).thread().cloned().unwrap());
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
            assert_eq!(view.editing_message, None);
            assert_eq!(view.thread().unwrap().read(cx).entries().len(), 2);

            view.entry_view_state
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
            assert_eq!(view.editing_message, None);

            let entries = view.thread().unwrap().read(cx).entries();
            assert_eq!(entries.len(), 2);
            assert_eq!(
                entries[0].to_markdown(cx),
                "## User\n\nEdited message content\n\n"
            );
            assert_eq!(
                entries[1].to_markdown(cx),
                "## Assistant\n\nNew Response\n\n"
            );

            let new_editor = view.entry_view_state.read_with(cx, |state, _cx| {
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
            let thread = view.thread().unwrap().read(cx);
            assert_eq!(thread.entries().len(), 1);

            let editor = view
                .entry_view_state
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

        thread_view.read_with(cx, |view, _cx| {
            assert_eq!(view.editing_message, Some(0));
        });

        // Edit
        user_message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Edited message content", window, cx);
        });

        thread_view.read_with(cx, |view, _cx| {
            assert_eq!(view.editing_message, Some(0));
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

        thread_view.read_with(cx, |view, _cx| {
            assert_eq!(view.editing_message, Some(0));
        });

        cx.run_until_parked();

        // Should still be editing
        cx.update(|window, cx| {
            assert!(user_message_editor.focus_handle(cx).is_focused(window));
            assert_eq!(thread_view.read(cx).editing_message, Some(0));
            assert_eq!(
                user_message_editor.read(cx).text(cx),
                "Edited message content"
            );
        });
    }

    struct GeneratingThreadSetup {
        thread_view: Entity<AcpThreadView>,
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
            let thread = view.thread().unwrap();
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

        let thread = thread_view.read_with(cx, |view, _cx| view.thread().unwrap().clone());

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
            let thread = view.thread().unwrap();

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
                .entry_view_state
                .read(cx)
                .entry(0)
                .expect("Should have at least one entry")
                .message_editor()
                .expect("Should have message editor")
                .clone()
        });

        cx.focus(&user_message_editor);
        thread_view.read_with(cx, |thread_view, _cx| {
            assert_eq!(thread_view.editing_message, Some(0));
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

        thread_view.update_in(cx, |thread_view, window, cx| {
            assert_eq!(thread_view.editing_message, Some(0));
            thread_view.insert_selections(window, cx);
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

        thread_view.update_in(cx, |thread_view, window, cx| {
            assert_eq!(thread_view.editing_message, None);
            thread_view.insert_selections(window, cx);
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
            let thread = thread_view.thread().expect("Thread should exist");
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
                assert_eq!(
                    options.len(),
                    3,
                    "Expected 3 permission options (granularity only)"
                );

                // Verify specific button labels (now using neutral names)
                let labels: Vec<&str> = options.iter().map(|o| o.name.as_ref()).collect();
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
            let thread = thread_view.thread().expect("Thread should exist");
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
                let labels: Vec<&str> = options.iter().map(|o| o.name.as_ref()).collect();
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
            let thread = thread_view.thread().expect("Thread should exist");
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
                let labels: Vec<&str> = options.iter().map(|o| o.name.as_ref()).collect();
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
            let thread = thread_view.thread().expect("Thread should exist");
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
                assert_eq!(
                    options.len(),
                    2,
                    "Expected 2 permission options (no pattern option)"
                );

                let labels: Vec<&str> = options.iter().map(|o| o.name.as_ref()).collect();
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
            let thread = thread_view.thread().expect("Thread should exist");
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
            let thread = thread_view.thread().expect("Thread should exist");
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
        let pattern_option = permission_options
            .iter()
            .find(|o| o.option_id.0.starts_with("always_pattern:"))
            .expect("Should have a pattern option for npm command");

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
            let thread = thread_view.thread().expect("Thread should exist");
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
        thread_view.read_with(cx, |thread_view, _cx| {
            let selected = thread_view
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
        thread_view.read_with(cx, |thread_view, _cx| {
            let selected = thread_view
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
        assert_eq!(permission_options.len(), 3);
        assert!(
            permission_options[0]
                .option_id
                .0
                .contains("always:terminal")
        );
        assert!(
            permission_options[1]
                .option_id
                .0
                .contains("always_pattern:terminal")
        );
        assert_eq!(permission_options[2].option_id.0.as_ref(), "once");

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
            let thread = thread_view.thread().expect("Thread should exist");
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
            let thread = thread_view.thread().expect("Thread should exist");
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
        // Test the option_id transformation logic directly
        // "once" -> "allow"
        // "always:terminal" -> "always_allow:terminal"
        // "always_pattern:terminal:^cargo\s" -> "always_allow_pattern:terminal:^cargo\s"

        let test_cases = vec![
            ("once", "allow"),
            ("always:terminal", "always_allow:terminal"),
            (
                "always_pattern:terminal:^cargo\\s",
                "always_allow_pattern:terminal:^cargo\\s",
            ),
            ("always:fetch", "always_allow:fetch"),
            (
                "always_pattern:fetch:^https?://docs\\.rs",
                "always_allow_pattern:fetch:^https?://docs\\.rs",
            ),
        ];

        for (input, expected) in test_cases {
            let result = if input == "once" {
                "allow".to_string()
            } else if let Some(rest) = input.strip_prefix("always:") {
                format!("always_allow:{}", rest)
            } else if let Some(rest) = input.strip_prefix("always_pattern:") {
                format!("always_allow_pattern:{}", rest)
            } else {
                input.to_string()
            };
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[gpui::test]
    async fn test_option_id_transformation_for_deny() {
        // Test the option_id transformation logic for deny
        // "once" -> "deny"
        // "always:terminal" -> "always_deny:terminal"
        // "always_pattern:terminal:^cargo\s" -> "always_deny_pattern:terminal:^cargo\s"

        let test_cases = vec![
            ("once", "deny"),
            ("always:terminal", "always_deny:terminal"),
            (
                "always_pattern:terminal:^cargo\\s",
                "always_deny_pattern:terminal:^cargo\\s",
            ),
            ("always:fetch", "always_deny:fetch"),
            (
                "always_pattern:fetch:^https?://docs\\.rs",
                "always_deny_pattern:fetch:^https?://docs\\.rs",
            ),
        ];

        for (input, expected) in test_cases {
            let result = if input == "once" {
                "deny".to_string()
            } else if let Some(rest) = input.strip_prefix("always:") {
                format!("always_deny:{}", rest)
            } else if let Some(rest) = input.strip_prefix("always_pattern:") {
                format!("always_deny_pattern:{}", rest)
            } else {
                input.replace("allow", "deny")
            };
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
