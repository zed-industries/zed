use acp_thread::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, AssistantMessage, AssistantMessageChunk,
    AuthRequired, LoadError, MentionUri, RetryStatus, ThreadStatus, ToolCall, ToolCallContent,
    ToolCallStatus, UserMessageId,
};
use acp_thread::{AgentConnection, Plan};
use action_log::ActionLog;
use agent_client_protocol::{self as acp, PromptCapabilities};
use agent_servers::{AgentServer, AgentServerDelegate, ClaudeCode};
use agent_settings::{AgentProfileId, AgentSettings, CompletionMode, NotifyWhenAgentWaiting};
use agent2::{DbThreadMetadata, HistoryEntry, HistoryEntryId, HistoryStore};
use anyhow::{Context as _, Result, anyhow, bail};
use audio::{Audio, Sound};
use buffer_diff::BufferDiff;
use client::zed_urls;
use collections::{HashMap, HashSet};
use editor::scroll::Autoscroll;
use editor::{Editor, EditorEvent, EditorMode, MultiBuffer, PathKey, SelectionEffects};
use file_icons::FileIcons;
use fs::Fs;
use futures::FutureExt as _;
use gpui::{
    Action, Animation, AnimationExt, AnyView, App, BorderStyle, ClickEvent, ClipboardItem,
    CursorStyle, EdgesRefinement, ElementId, Empty, Entity, FocusHandle, Focusable, Hsla, Length,
    ListOffset, ListState, MouseButton, PlatformDisplay, SharedString, Stateful, StyleRefinement,
    Subscription, Task, TextStyle, TextStyleRefinement, Transformation, UnderlineStyle, WeakEntity,
    Window, WindowHandle, div, ease_in_out, linear_color_stop, linear_gradient, list, percentage,
    point, prelude::*, pulsating_between,
};
use language::Buffer;

use language_model::LanguageModelRegistry;
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use project::{Project, ProjectEntryId};
use prompt_store::{PromptId, PromptStore};
use rope::Point;
use settings::{Settings as _, SettingsStore};
use std::cell::Cell;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use std::{collections::BTreeMap, rc::Rc, time::Duration};
use task::SpawnInTerminal;
use terminal_view::terminal_panel::TerminalPanel;
use text::Anchor;
use theme::ThemeSettings;
use ui::{
    Callout, Disclosure, Divider, DividerColor, ElevationIndex, KeyBinding, PopoverMenuHandle,
    Scrollbar, ScrollbarState, SpinnerLabel, Tooltip, prelude::*,
};
use util::{ResultExt, size::format_file_size, time::duration_alt_display};
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::{Chat, ToggleModelSelector};
use zed_actions::assistant::OpenRulesLibrary;

use super::entry_view_state::EntryViewState;
use crate::acp::AcpModelSelectorPopover;
use crate::acp::entry_view_state::{EntryViewEvent, ViewEvent};
use crate::acp::message_editor::{MessageEditor, MessageEditorEvent};
use crate::agent_diff::AgentDiff;
use crate::profile_selector::{ProfileProvider, ProfileSelector};

use crate::ui::preview::UsageCallout;
use crate::ui::{
    AgentNotification, AgentNotificationEvent, BurnModeTooltip, UnavailableEditingTooltip,
};
use crate::{
    AgentDiffPane, AgentPanel, ContinueThread, ContinueWithBurnMode, ExpandMessageEditor, Follow,
    KeepAll, OpenAgentDiff, OpenHistory, RejectAll, ToggleBurnMode, ToggleProfileSelector,
};

pub const MIN_EDITOR_LINES: usize = 4;
pub const MAX_EDITOR_LINES: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ThreadFeedback {
    Positive,
    Negative,
}

enum ThreadError {
    PaymentRequired,
    ModelRequestLimitReached(cloud_llm_client::Plan),
    ToolUseLimitReached,
    AuthenticationRequired(SharedString),
    Other(SharedString),
}

impl ThreadError {
    fn from_err(error: anyhow::Error, agent: &Rc<dyn AgentServer>) -> Self {
        if error.is::<language_model::PaymentRequiredError>() {
            Self::PaymentRequired
        } else if error.is::<language_model::ToolUseLimitReachedError>() {
            Self::ToolUseLimitReached
        } else if let Some(error) =
            error.downcast_ref::<language_model::ModelRequestLimitReachedError>()
        {
            Self::ModelRequestLimitReached(error.plan)
        } else if let Some(acp_error) = error.downcast_ref::<acp::Error>()
            && acp_error.code == acp::ErrorCode::AUTH_REQUIRED.code
        {
            Self::AuthenticationRequired(acp_error.message.clone().into())
        } else {
            let string = error.to_string();
            // TODO: we should have Gemini return better errors here.
            if agent.clone().downcast::<agent_servers::Gemini>().is_some()
                && string.contains("Could not load the default credentials")
                || string.contains("API key not valid")
                || string.contains("Request had invalid authentication credentials")
            {
                Self::AuthenticationRequired(string.into())
            } else {
                Self::Other(error.to_string().into())
            }
        }
    }
}

impl ProfileProvider for Entity<agent2::Thread> {
    fn profile_id(&self, cx: &App) -> AgentProfileId {
        self.read(cx).profile().clone()
    }

    fn set_profile(&self, profile_id: AgentProfileId, cx: &mut App) {
        self.update(cx, |thread, _cx| {
            thread.set_profile(profile_id);
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
        let agent_name = telemetry.agent_name();
        let task = telemetry.thread_data(&session_id, cx);
        let rating = match feedback {
            ThreadFeedback::Positive => "positive",
            ThreadFeedback::Negative => "negative",
        };
        cx.background_spawn(async move {
            let thread = task.await?;
            telemetry::event!(
                "Agent Thread Rated",
                session_id = session_id,
                rating = rating,
                agent = agent_name,
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
        let agent_name = telemetry.agent_name();
        let task = telemetry.thread_data(&session_id, cx);
        cx.background_spawn(async move {
            let thread = task.await?;
            telemetry::event!(
                "Agent Thread Feedback Comments",
                session_id = session_id,
                comments = comments,
                agent = agent_name,
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
                cx,
            );
            editor
        });

        editor.read(cx).focus_handle(cx).focus(window);
        editor
    }
}

pub struct AcpThreadView {
    agent: Rc<dyn AgentServer>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    thread_state: ThreadState,
    history_store: Entity<HistoryStore>,
    hovered_recent_history_item: Option<usize>,
    entry_view_state: Entity<EntryViewState>,
    message_editor: Entity<MessageEditor>,
    focus_handle: FocusHandle,
    model_selector: Option<Entity<AcpModelSelectorPopover>>,
    profile_selector: Option<Entity<ProfileSelector>>,
    notifications: Vec<WindowHandle<AgentNotification>>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    thread_retry_status: Option<RetryStatus>,
    thread_error: Option<ThreadError>,
    thread_feedback: ThreadFeedbackState,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    auth_task: Option<Task<()>>,
    expanded_tool_calls: HashSet<acp::ToolCallId>,
    expanded_thinking_blocks: HashSet<(usize, usize)>,
    edits_expanded: bool,
    plan_expanded: bool,
    editor_expanded: bool,
    should_be_following: bool,
    editing_message: Option<usize>,
    prompt_capabilities: Rc<Cell<PromptCapabilities>>,
    is_loading_contents: bool,
    _cancel_task: Option<Task<()>>,
    _subscriptions: [Subscription; 3],
}

enum ThreadState {
    Loading(Entity<LoadingView>),
    Ready {
        thread: Entity<AcpThread>,
        title_editor: Option<Entity<Editor>>,
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
        resume_thread: Option<DbThreadMetadata>,
        summarize_thread: Option<DbThreadMetadata>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_capabilities = Rc::new(Cell::new(acp::PromptCapabilities::default()));
        let prevent_slash_commands = agent.clone().downcast::<ClaudeCode>().is_some();

        let placeholder = if agent.name() == "Zed Agent" {
            format!("Message the {} — @ to include context", agent.name())
        } else {
            format!("Message {} — @ to include context", agent.name())
        };

        let message_editor = cx.new(|cx| {
            let mut editor = MessageEditor::new(
                workspace.clone(),
                project.clone(),
                history_store.clone(),
                prompt_store.clone(),
                prompt_capabilities.clone(),
                placeholder,
                prevent_slash_commands,
                editor::EditorMode::AutoHeight {
                    min_lines: MIN_EDITOR_LINES,
                    max_lines: Some(MAX_EDITOR_LINES),
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
                project.clone(),
                history_store.clone(),
                prompt_store.clone(),
                prompt_capabilities.clone(),
                prevent_slash_commands,
            )
        });

        let subscriptions = [
            cx.observe_global_in::<SettingsStore>(window, Self::settings_changed),
            cx.subscribe_in(&message_editor, window, Self::handle_message_editor_event),
            cx.subscribe_in(&entry_view_state, window, Self::handle_entry_view_event),
        ];

        Self {
            agent: agent.clone(),
            workspace: workspace.clone(),
            project: project.clone(),
            entry_view_state,
            thread_state: Self::initial_state(agent, resume_thread, workspace, project, window, cx),
            message_editor,
            model_selector: None,
            profile_selector: None,
            notifications: Vec::new(),
            notification_subscriptions: HashMap::default(),
            list_state: list_state.clone(),
            scrollbar_state: ScrollbarState::new(list_state).parent_entity(&cx.entity()),
            thread_retry_status: None,
            thread_error: None,
            thread_feedback: Default::default(),
            auth_task: None,
            expanded_tool_calls: HashSet::default(),
            expanded_thinking_blocks: HashSet::default(),
            editing_message: None,
            edits_expanded: false,
            plan_expanded: false,
            editor_expanded: false,
            should_be_following: false,
            history_store,
            hovered_recent_history_item: None,
            prompt_capabilities,
            is_loading_contents: false,
            _subscriptions: subscriptions,
            _cancel_task: None,
            focus_handle: cx.focus_handle(),
        }
    }

    fn initial_state(
        agent: Rc<dyn AgentServer>,
        resume_thread: Option<DbThreadMetadata>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ThreadState {
        let root_dir = project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path())
            .unwrap_or_else(|| paths::home_dir().as_path().into());
        let (tx, mut rx) = watch::channel("Loading…".into());
        let delegate = AgentServerDelegate::new(project.clone(), Some(tx));

        let connect_task = agent.connect(&root_dir, delegate, cx);
        let load_task = cx.spawn_in(window, async move |this, cx| {
            let connection = match connect_task.await {
                Ok(connection) => connection,
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

            let result = if let Some(native_agent) = connection
                .clone()
                .downcast::<agent2::NativeAgentConnection>()
                && let Some(resume) = resume_thread.clone()
            {
                cx.update(|_, cx| {
                    native_agent
                        .0
                        .update(cx, |agent, cx| agent.open_thread(resume.id, cx))
                })
                .log_err()
            } else {
                cx.update(|_, cx| {
                    connection
                        .clone()
                        .new_thread(project.clone(), &root_dir, cx)
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
                            .set(thread.read(cx).prompt_capabilities());

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

                        if let Some(resume) = resume_thread {
                            this.history_store.update(cx, |history, cx| {
                                history.push_recently_opened_entry(
                                    HistoryEntryId::AcpThread(resume.id),
                                    cx,
                                );
                            });
                        }

                        AgentDiff::set_active_thread(&workspace, thread.clone(), window, cx);

                        this.model_selector =
                            thread
                                .read(cx)
                                .connection()
                                .model_selector()
                                .map(|selector| {
                                    cx.new(|cx| {
                                        AcpModelSelectorPopover::new(
                                            thread.read(cx).session_id().clone(),
                                            selector,
                                            PopoverMenuHandle::default(),
                                            this.focus_handle(cx),
                                            window,
                                            cx,
                                        )
                                    })
                                });

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
                            _subscriptions: subscriptions,
                        };
                        this.message_editor.focus_handle(cx).focus(window);

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

                        cx.notify();
                    }
                    Err(err) => {
                        this.handle_load_error(err, window, cx);
                    }
                };
            })
            .log_err();
        });

        let loading_view = cx.new(|cx| {
            let update_title_task = cx.spawn(async move |this, cx| {
                loop {
                    let status = rx.recv().await?;
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
        let (configuration_view, subscription) = if let Some(provider_id) = err.provider_id {
            let registry = LanguageModelRegistry::global(cx);

            let sub = window.subscribe(&registry, cx, {
                let provider_id = provider_id.clone();
                let this = this.clone();
                move |_, ev, window, cx| {
                    if let language_model::Event::ProviderStateChanged(updated_provider_id) = &ev
                        && &provider_id == updated_provider_id
                    {
                        this.update(cx, |this, cx| {
                            this.thread_state = Self::initial_state(
                                agent.clone(),
                                None,
                                this.workspace.clone(),
                                this.project.clone(),
                                window,
                                cx,
                            );
                            cx.notify();
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
                    .clone()
                    .map(|desc| cx.new(|cx| Markdown::new(desc.into(), None, None, cx))),
                _subscription: subscription,
            };
            if this.message_editor.focus_handle(cx).is_focused(window) {
                this.focus_handle.focus(window)
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
            self.thread_state = ThreadState::LoadError(LoadError::Other(err.to_string().into()))
        }
        if self.message_editor.focus_handle(cx).is_focused(window) {
            self.focus_handle.focus(window)
        }
        cx.notify();
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

        if let Some(thread) = self.thread() {
            self._cancel_task = Some(thread.update(cx, |thread, cx| thread.cancel(cx)));
        }
    }

    pub fn expand_message_editor(
        &mut self,
        _: &ExpandMessageEditor,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_editor_is_expanded(!self.editor_expanded, cx);
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
                        sized_by_content: false,
                    },
                    cx,
                )
            } else {
                editor.set_mode(
                    EditorMode::AutoHeight {
                        min_lines: MIN_EDITOR_LINES,
                        max_lines: Some(MAX_EDITOR_LINES),
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
            ViewEvent::MessageEditorEvent(editor, MessageEditorEvent::Send) => {
                self.regenerate(event.entry_index, editor, window, cx);
            }
            ViewEvent::MessageEditorEvent(_editor, MessageEditorEvent::Cancel) => {
                self.cancel_editing(&Default::default(), window, cx);
            }
        }
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

        self.history_store.update(cx, |history, cx| {
            history.push_recently_opened_entry(
                HistoryEntryId::AcpThread(thread.read(cx).session_id().clone()),
                cx,
            );
        });

        if thread.read(cx).status() != ThreadStatus::Idle {
            self.stop_current_and_send_new_message(window, cx);
            return;
        }

        let contents = self
            .message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx));
        self.send_impl(contents, window, cx)
    }

    fn stop_current_and_send_new_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };

        let cancelled = thread.update(cx, |thread, cx| thread.cancel(cx));

        let contents = self
            .message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx));

        cx.spawn_in(window, async move |this, cx| {
            cancelled.await;

            this.update_in(cx, |this, window, cx| {
                this.send_impl(contents, window, cx);
            })
            .ok();
        })
        .detach();
    }

    fn send_impl(
        &mut self,
        contents: Task<Result<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let agent_telemetry_id = self.agent.telemetry_id();

        self.thread_error.take();
        self.editing_message.take();
        self.thread_feedback.clear();

        let Some(thread) = self.thread() else {
            return;
        };
        let thread = thread.downgrade();
        if self.should_be_following {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .ok();
        }

        self.is_loading_contents = true;
        let guard = cx.new(|_| ());
        cx.observe_release(&guard, |this, _guard, cx| {
            this.is_loading_contents = false;
            cx.notify();
        })
        .detach();

        let task = cx.spawn_in(window, async move |this, cx| {
            let (contents, tracked_buffers) = contents.await?;

            if contents.is_empty() {
                return Ok(());
            }

            this.update_in(cx, |this, window, cx| {
                this.set_editor_is_expanded(false, cx);
                this.scroll_to_bottom(cx);
                this.message_editor.update(cx, |message_editor, cx| {
                    message_editor.clear(window, cx);
                });
            })?;
            let send = thread.update(cx, |thread, cx| {
                thread.action_log().update(cx, |action_log, cx| {
                    for buffer in tracked_buffers {
                        action_log.buffer_read(buffer, cx)
                    }
                });
                drop(guard);

                telemetry::event!("Agent Message Sent", agent = agent_telemetry_id);

                thread.send(contents, cx)
            })?;
            send.await
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
        self.focus_handle(cx).focus(window);
        cx.notify();
    }

    fn regenerate(
        &mut self,
        entry_ix: usize,
        message_editor: &Entity<MessageEditor>,
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

        let contents = message_editor.update(cx, |message_editor, cx| message_editor.contents(cx));

        let task = cx.spawn(async move |_, cx| {
            let contents = contents.await?;
            thread
                .update(cx, |thread, cx| thread.rewind(user_message_id, cx))?
                .await?;
            Ok(contents)
        });
        self.send_impl(task, window, cx);
    }

    fn open_agent_diff(&mut self, _: &OpenAgentDiff, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(thread) = self.thread() {
            AgentDiffPane::deploy(thread.clone(), self.workspace.clone(), window, cx).log_err();
        }
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
                    self.focus_handle.focus(window)
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
            }
            AcpThreadEvent::PromptCapabilitiesUpdated => {
                self.prompt_capabilities
                    .set(thread.read(cx).prompt_capabilities());
            }
            AcpThreadEvent::TokenUsageUpdated => {}
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
        } else if method.0.as_ref() == "anthropic-api-key" {
            let registry = LanguageModelRegistry::global(cx);
            let provider = registry
                .read(cx)
                .provider(&language_model::ANTHROPIC_PROVIDER_ID)
                .unwrap();
            if !provider.is_authenticated(cx) {
                let this = cx.weak_entity();
                let agent = self.agent.clone();
                let connection = connection.clone();
                window.defer(cx, |window, cx| {
                    Self::handle_auth_required(
                        this,
                        AuthRequired {
                            description: Some("ANTHROPIC_API_KEY must be set".to_owned()),
                            provider_id: Some(language_model::ANTHROPIC_PROVIDER_ID),
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
        let authenticate = if method.0.as_ref() == "claude-login" {
            if let Some(workspace) = self.workspace.upgrade() {
                Self::spawn_claude_login(&workspace, window, cx)
            } else {
                Task::ready(Ok(()))
            }
        } else {
            connection.authenticate(method, cx)
        };
        cx.notify();
        self.auth_task =
            Some(cx.spawn_in(window, {
                let project = self.project.clone();
                let agent = self.agent.clone();
                async move |this, cx| {
                    let result = authenticate.await;

                    match &result {
                        Ok(_) => telemetry::event!(
                            "Authenticate Agent Succeeded",
                            agent = agent.telemetry_id()
                        ),
                        Err(_) => {
                            telemetry::event!(
                                "Authenticate Agent Failed",
                                agent = agent.telemetry_id(),
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
                            this.thread_state = Self::initial_state(
                                agent,
                                None,
                                this.workspace.clone(),
                                project.clone(),
                                window,
                                cx,
                            )
                        }
                        this.auth_task.take()
                    })
                    .ok();
                }
            }));
    }

    fn spawn_claude_login(
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
            return Task::ready(Ok(()));
        };
        let project_entity = workspace.read(cx).project();
        let project = project_entity.read(cx);
        let cwd = project.first_project_directory(cx);
        let shell = project.terminal_settings(&cwd, cx).shell.clone();

        let delegate = AgentServerDelegate::new(project_entity.clone(), None);
        let command = ClaudeCode::login_command(delegate, cx);

        window.spawn(cx, async move |cx| {
            let login_command = command.await?;
            let command = login_command
                .path
                .to_str()
                .with_context(|| format!("invalid login command: {:?}", login_command.path))?;
            let command = shlex::try_quote(command)?;
            let args = login_command
                .arguments
                .iter()
                .map(|arg| {
                    Ok(shlex::try_quote(arg)
                        .context("Failed to quote argument")?
                        .to_string())
                })
                .collect::<Result<Vec<_>>>()?;

            let terminal = terminal_panel.update_in(cx, |terminal_panel, window, cx| {
                terminal_panel.spawn_task(
                    &SpawnInTerminal {
                        id: task::TaskId("claude-login".into()),
                        full_label: "claude /login".to_owned(),
                        label: "claude /login".to_owned(),
                        command: Some(command.into()),
                        args,
                        command_label: "claude /login".to_owned(),
                        cwd,
                        use_new_terminal: true,
                        allow_concurrent_runs: true,
                        hide: task::HideStrategy::Always,
                        shell,
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })?;

            let terminal = terminal.await?;
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
                            if content.contains("Login successful") {
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
                    return Err(anyhow!("exited before logging in"));
                }
            }
            terminal.update(cx, |terminal, _| terminal.kill_active_task())?;
            Ok(())
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

    fn rewind(&mut self, message_id: &UserMessageId, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };
        thread
            .update(cx, |thread, cx| thread.rewind(message_id.clone(), cx))
            .detach_and_log_err(cx);
        cx.notify();
    }

    fn render_entry(
        &self,
        entry_ix: usize,
        total_entries: usize,
        entry: &AgentThreadEntry,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
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
                        if entry_ix == 0 && !has_checkpoint_button && rules_item.is_none()  {
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
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.rewind(&message_id, cx);
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
                                    .child(editor.clone().into_any_element()),
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
                                                                    entry_ix, &editor, window, cx,
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
                                                    .tooltip(move |_window, cx| {
                                                        cx.new(|_| UnavailableEditingTooltip::new(agent_name.clone()))
                                                            .into()
                                                    })
                                            )
                                    )
                                }
                            }),
                    )
                    .into_any()
            }
            AgentThreadEntry::AssistantMessage(AssistantMessage { chunks }) => {
                let is_last = entry_ix + 1 == total_entries;

                let style = default_markdown_style(false, false, window, cx);
                let message_body = v_flex()
                    .w_full()
                    .gap_3()
                    .children(chunks.iter().enumerate().filter_map(
                        |(chunk_ix, chunk)| match chunk {
                            AssistantMessageChunk::Message { block } => {
                                block.markdown().map(|md| {
                                    self.render_markdown(md.clone(), style.clone())
                                        .into_any_element()
                                })
                            }
                            AssistantMessageChunk::Thought { block } => {
                                block.markdown().map(|md| {
                                    self.render_thinking_block(
                                        entry_ix,
                                        chunk_ix,
                                        md.clone(),
                                        window,
                                        cx,
                                    )
                                    .into_any_element()
                                })
                            }
                        },
                    ))
                    .into_any();

                v_flex()
                    .px_5()
                    .py_1p5()
                    .when(is_last, |this| this.pb_4())
                    .w_full()
                    .text_ui(cx)
                    .child(message_body)
                    .into_any()
            }
            AgentThreadEntry::ToolCall(tool_call) => {
                let has_terminals = tool_call.terminals().next().is_some();

                div().w_full().map(|this| {
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
            }
            .into_any(),
        };

        let Some(thread) = self.thread() else {
            return primary;
        };

        let primary = if entry_ix == total_entries - 1 {
            v_flex()
                .w_full()
                .child(primary)
                .child(self.render_thread_controls(&thread, cx))
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
        let card_header_id = SharedString::from("inner-tool-call-header");

        let tool_icon =
            if tool_call.kind == acp::ToolKind::Edit && tool_call.locations.len() == 1 {
                FileIcons::get_icon(&tool_call.locations[0].path, cx)
                    .map(Icon::from_path)
                    .unwrap_or(Icon::new(IconName::ToolPencil))
            } else {
                Icon::new(match tool_call.kind {
                    acp::ToolKind::Read => IconName::ToolSearch,
                    acp::ToolKind::Edit => IconName::ToolPencil,
                    acp::ToolKind::Delete => IconName::ToolDeleteFile,
                    acp::ToolKind::Move => IconName::ArrowRightLeft,
                    acp::ToolKind::Search => IconName::ToolSearch,
                    acp::ToolKind::Execute => IconName::ToolTerminal,
                    acp::ToolKind::Think => IconName::ToolThink,
                    acp::ToolKind::Fetch => IconName::ToolWeb,
                    acp::ToolKind::Other => IconName::ToolHammer,
                })
            }
            .size(IconSize::Small)
            .color(Color::Muted);

        let failed_or_canceled = match &tool_call.status {
            ToolCallStatus::Rejected | ToolCallStatus::Canceled | ToolCallStatus::Failed => true,
            _ => false,
        };

        let has_location = tool_call.locations.len() == 1;
        let needs_confirmation = matches!(
            tool_call.status,
            ToolCallStatus::WaitingForConfirmation { .. }
        );
        let is_edit =
            matches!(tool_call.kind, acp::ToolKind::Edit) || tool_call.diffs().next().is_some();
        let use_card_layout = needs_confirmation || is_edit;

        let is_collapsible = !tool_call.content.is_empty() && !needs_confirmation;

        let is_open = needs_confirmation || self.expanded_tool_calls.contains(&tool_call.id);

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

        let tool_output_display = if is_open {
            match &tool_call.status {
                ToolCallStatus::WaitingForConfirmation { options, .. } => v_flex()
                    .w_full()
                    .children(tool_call.content.iter().map(|content| {
                        div()
                            .child(self.render_tool_call_content(
                                entry_ix,
                                content,
                                tool_call,
                                use_card_layout,
                                window,
                                cx,
                            ))
                            .into_any_element()
                    }))
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
                    .w_full()
                    .children(tool_call.content.iter().map(|content| {
                        div().child(self.render_tool_call_content(
                            entry_ix,
                            content,
                            tool_call,
                            use_card_layout,
                            window,
                            cx,
                        ))
                    }))
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
            .child(
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
                    .child(
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
                                let name = tool_call.locations[0]
                                    .path
                                    .file_name()
                                    .unwrap_or_default()
                                    .display()
                                    .to_string();

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
                                    .child(name)
                                    .tooltip(Tooltip::text("Jump to File"))
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
                            .when(!has_location, |this| this.child(gradient_overlay)),
                    )
                    .when(is_collapsible || failed_or_canceled, |this| {
                        this.child(
                            h_flex()
                                .px_1()
                                .gap_px()
                                .when(is_collapsible, |this| {
                                    this.child(
                                    Disclosure::new(("expand", entry_ix), is_open)
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
                                    this.child(
                                        Icon::new(IconName::Close)
                                            .color(Color::Error)
                                            .size(IconSize::Small),
                                    )
                                }),
                        )
                    }),
            )
            .children(tool_output_display)
    }

    fn render_tool_call_content(
        &self,
        entry_ix: usize,
        content: &ToolCallContent,
        tool_call: &ToolCall,
        card_layout: bool,
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
                        card_layout,
                        window,
                        cx,
                    )
                } else {
                    Empty.into_any_element()
                }
            }
            ToolCallContent::Diff(diff) => self.render_diff_editor(entry_ix, diff, tool_call, cx),
            ToolCallContent::Terminal(terminal) => {
                self.render_terminal_tool_call(entry_ix, terminal, tool_call, window, cx)
            }
        }
    }

    fn render_markdown_output(
        &self,
        markdown: Entity<Markdown>,
        tool_call_id: acp::ToolCallId,
        card_layout: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let button_id = SharedString::from(format!("tool_output-{:?}", tool_call_id));

        v_flex()
            .mt_1p5()
            .gap_2()
            .when(!card_layout, |this| {
                this.ml(rems(0.4))
                    .px_3p5()
                    .border_l_1()
                    .border_color(self.tool_card_border_color(cx))
            })
            .when(card_layout, |this| {
                this.p_2()
                    .border_t_1()
                    .border_color(self.tool_card_border_color(cx))
            })
            .text_sm()
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
        options: &[acp::PermissionOption],
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        h_flex()
            .py_1()
            .pl_2()
            .pr_1()
            .gap_1()
            .justify_between()
            .flex_wrap()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .child(
                div()
                    .min_w(rems_from_px(145.))
                    .child(LoadingLabel::new("Waiting for Confirmation").size(LabelSize::Small)),
            )
            .child(h_flex().gap_0p5().children(options.iter().map(|option| {
                let option_id = SharedString::from(option.id.0.clone());
                Button::new((option_id, entry_ix), option.name.clone())
                    .map(|this| match option.kind {
                        acp::PermissionOptionKind::AllowOnce => {
                            this.icon(IconName::Check).icon_color(Color::Success)
                        }
                        acp::PermissionOptionKind::AllowAlways => {
                            this.icon(IconName::CheckDouble).icon_color(Color::Success)
                        }
                        acp::PermissionOptionKind::RejectOnce => {
                            this.icon(IconName::Close).icon_color(Color::Error)
                        }
                        acp::PermissionOptionKind::RejectAlways => {
                            this.icon(IconName::Close).icon_color(Color::Error)
                        }
                    })
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::XSmall)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener({
                        let tool_call_id = tool_call_id.clone();
                        let option_id = option.id.clone();
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
            })))
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
        cx: &Context<Self>,
    ) -> AnyElement {
        let tool_progress = matches!(
            &tool_call.status,
            ToolCallStatus::InProgress | ToolCallStatus::Pending
        );

        v_flex()
            .h_full()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .child(
                if let Some(entry) = self.entry_view_state.read(cx).entry(entry_ix)
                    && let Some(editor) = entry.editor_for_diff(diff)
                    && diff.read(cx).has_revealed_range(cx)
                {
                    editor.into_any_element()
                } else if tool_progress && self.as_native_connection(cx).is_some() {
                    self.render_diff_loading(cx)
                } else {
                    Empty.into_any()
                },
            )
            .into_any()
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
        let truncated_output = output.is_some_and(|output| output.was_content_truncated);
        let output_line_count = output.map(|output| output.content_line_count).unwrap_or(0);

        let command_failed = command_finished
            && output.is_some_and(|o| o.exit_status.is_none_or(|status| !status.success()));

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
            .map(|path| format!("{}", path.display()))
            .unwrap_or_else(|| "current directory".to_string());

        let is_expanded = self.expanded_tool_calls.contains(&tool_call.id);

        let header = h_flex()
            .id(header_id)
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
                        .tooltip(move |window, cx| {
                            Tooltip::with_meta(
                                "Stop This Command",
                                None,
                                "Also possible by placing your cursor inside the terminal and using regular terminal bindings.",
                                window,
                                cx,
                            )
                        })
                        .on_click({
                            let terminal = terminal.clone();
                            cx.listener(move |_this, _event, _window, cx| {
                                let inner_terminal = terminal.read(cx).inner().clone();
                                inner_terminal.update(cx, |inner_terminal, _cx| {
                                    inner_terminal.kill_active_task();
                                });
                            })
                        }),
                    )
                    .child(Divider::vertical())
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::XSmall)
                            .color(Color::Info)
                            .with_animation(
                                "arrow-circle",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |icon, delta| {
                                    icon.transform(Transformation::rotate(percentage(delta)))
                                },
                            ),
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
            )
            .when(truncated_output, |header| {
                let tooltip = if let Some(output) = output {
                    if output_line_count + 10 > terminal::MAX_SCROLL_HISTORY_LINES {
                        "Output exceeded terminal max lines and was \
                            truncated, the model received the first 16 KB."
                            .to_string()
                    } else {
                        format!(
                            "Output is {} long, and to avoid unexpected token usage, \
                                only 16 KB was sent back to the model.",
                            format_file_size(output.original_content_len as u64, true),
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
            });

        let terminal_view = self
            .entry_view_state
            .read(cx)
            .entry(entry_ix)
            .and_then(|entry| entry.terminal(terminal));
        let show_output = is_expanded && terminal_view.is_some();

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
                    .py_1p5()
                    .pr_1p5()
                    .pl_2()
                    .gap_0p5()
                    .bg(header_bg)
                    .text_xs()
                    .child(header)
                    .child(
                        MarkdownElement::new(
                            command.clone(),
                            terminal_command_markdown_style(window, cx),
                        )
                        .code_block_renderer(
                            markdown::CodeBlockRenderer::Default {
                                copy_button: false,
                                copy_button_on_hover: true,
                                border: false,
                            },
                        ),
                    ),
            )
            .when(show_output, |this| {
                this.child(
                    div()
                        .pt_2()
                        .border_t_1()
                        .when(tool_failed || command_failed, |card| card.border_dashed())
                        .border_color(border_color)
                        .bg(cx.theme().colors().editor_background)
                        .rounded_b_md()
                        .text_ui_sm(cx)
                        .children(terminal_view.clone()),
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

    fn render_recent_history(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let render_history = self
            .agent
            .clone()
            .downcast::<agent2::NativeAgentServer>()
            .is_some()
            && self
                .history_store
                .update(cx, |history_store, cx| !history_store.is_empty(cx));

        v_flex()
            .size_full()
            .when(render_history, |this| {
                let recent_history: Vec<_> = self.history_store.update(cx, |history_store, _| {
                    history_store.entries().take(3).collect()
                });
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
                                                window,
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
                        .child(
                            v_flex().p_1().pr_1p5().gap_1().children(
                                recent_history
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, entry)| {
                                        // TODO: Add keyboard navigation.
                                        let is_hovered =
                                            self.hovered_recent_history_item == Some(index);
                                        crate::acp::thread_history::AcpHistoryEntryElement::new(
                                            entry,
                                            cx.entity().downgrade(),
                                        )
                                        .hovered(is_hovered)
                                        .on_hover(cx.listener(
                                            move |this, is_hovered, _window, cx| {
                                                if *is_hovered {
                                                    this.hovered_recent_history_item = Some(index);
                                                } else if this.hovered_recent_history_item
                                                    == Some(index)
                                                {
                                                    this.hovered_recent_history_item = None;
                                                }
                                                cx.notify();
                                            },
                                        ))
                                        .into_any_element()
                                    }),
                            ),
                        ),
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
    ) -> Div {
        let show_description =
            configuration_view.is_none() && description.is_none() && pending_auth_method.is_none();

        v_flex().flex_1().size_full().justify_end().child(
            v_flex()
                .p_2()
                .pr_3()
                .w_full()
                .gap_1()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().status().warning.opacity(0.04))
                .child(
                    h_flex()
                        .gap_1p5()
                        .child(
                            Icon::new(IconName::Warning)
                                .color(Color::Warning)
                                .size(IconSize::Small),
                        )
                        .child(Label::new("Authentication Required").size(LabelSize::Small)),
                )
                .children(description.map(|desc| {
                    div().text_ui(cx).child(self.render_markdown(
                        desc.clone(),
                        default_markdown_style(false, false, window, cx),
                    ))
                }))
                .children(
                    configuration_view
                        .cloned()
                        .map(|view| div().w_full().child(view)),
                )
                .when(
                    show_description,
                    |el| {
                        el.child(
                            Label::new(format!(
                                "You are not currently authenticated with {}. Please choose one of the following options:",
                                self.agent.name()
                            ))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .mb_1()
                            .ml_5(),
                        )
                    },
                )
                .when_some(pending_auth_method, |el, _| {
                    el.child(
                        h_flex()
                            .py_4()
                            .w_full()
                            .justify_center()
                            .gap_1()
                            .child(
                                Icon::new(IconName::ArrowCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Muted)
                                    .with_animation(
                                        "arrow-circle",
                                        Animation::new(Duration::from_secs(2)).repeat(),
                                        |icon, delta| {
                                            icon.transform(Transformation::rotate(percentage(
                                                delta,
                                            )))
                                        },
                                    )
                                    .into_any_element(),
                            )
                            .child(Label::new("Authenticating…").size(LabelSize::Small)),
                    )
                })
                .when(!connection.auth_methods().is_empty(), |this| {
                    this.child(
                        h_flex()
                            .justify_end()
                            .flex_wrap()
                            .gap_1()
                            .when(!show_description, |this| {
                                this.border_t_1()
                                    .mt_1()
                                    .pt_2()
                                    .border_color(cx.theme().colors().border.opacity(0.8))
                            })
                            .children(
                                connection
                                    .auth_methods()
                                    .iter()
                                    .enumerate()
                                    .rev()
                                    .map(|(ix, method)| {
                                        Button::new(
                                            SharedString::from(method.id.0.clone()),
                                            method.name.clone(),
                                        )
                                        .when(ix == 0, |el| {
                                            el.style(ButtonStyle::Tinted(ui::TintColor::Warning))
                                        })
                                        .label_size(LabelSize::Small)
                                        .on_click({
                                            let method_id = method.id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                telemetry::event!(
                                                    "Authenticate Agent Started",
                                                    agent = this.agent.telemetry_id(),
                                                    method = method_id
                                                );

                                                this.authenticate(method_id.clone(), window, cx)
                                            })
                                        })
                                    }),
                            ),
                    )
                })

        )
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

    fn render_activity_bar(
        &self,
        thread_entity: &Entity<AcpThread>,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let thread = thread_entity.read(cx);
        let action_log = thread.action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);
        let plan = thread.plan();

        if changed_buffers.is_empty() && plan.is_empty() {
            return None;
        }

        let editor_bg_color = cx.theme().colors().editor_background;
        let active_color = cx.theme().colors().element_selected;
        let bg_edit_files_disclosure = editor_bg_color.blend(active_color.opacity(0.3));

        // Temporarily always enable ACP edit controls. This is temporary, to lessen the
        // impact of a nasty bug that causes them to sometimes be disabled when they shouldn't
        // be, which blocks you from being able to accept or reject edits. This switches the
        // bug to be that sometimes it's enabled when it shouldn't be, which at least doesn't
        // block you from using the panel.
        let pending_edits = false;

        v_flex()
            .mt_1()
            .mx_2()
            .bg(bg_edit_files_disclosure)
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
                    window,
                    cx,
                ))
                .when(self.edits_expanded, |parent| {
                    parent.child(self.render_edited_files(
                        action_log,
                        &changed_buffers,
                        pending_edits,
                        cx,
                    ))
                })
            })
            .into_any()
            .into()
    }

    fn render_plan_summary(&self, plan: &Plan, window: &mut Window, cx: &Context<Self>) -> Div {
        let stats = plan.stats();

        let title = if let Some(entry) = stats.in_progress_entry
            && !self.plan_expanded
        {
            h_flex()
                .w_full()
                .cursor_default()
                .gap_1()
                .text_xs()
                .text_color(cx.theme().colors().text_muted)
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Label::new("Current:")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(MarkdownElement::new(
                            entry.content.clone(),
                            plan_label_markdown_style(&entry.status, window, cx),
                        )),
                )
                .when(stats.pending > 0, |this| {
                    this.child(
                        Label::new(format!("{} left", stats.pending))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .mr_1(),
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
            .p_1()
            .justify_between()
            .when(self.plan_expanded, |this| {
                this.border_b_1().border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .id("plan_summary")
                    .w_full()
                    .gap_1()
                    .child(Disclosure::new("plan_disclosure", self.plan_expanded))
                    .child(title)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.plan_expanded = !this.plan_expanded;
                        cx.notify();
                    })),
            )
    }

    fn render_plan_entries(&self, plan: &Plan, window: &mut Window, cx: &Context<Self>) -> Div {
        v_flex().children(plan.entries.iter().enumerate().flat_map(|(index, entry)| {
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
                            acp::PlanEntryStatus::Pending => Icon::new(IconName::TodoPending)
                                .size(IconSize::Small)
                                .color(Color::Muted)
                                .into_any_element(),
                            acp::PlanEntryStatus::InProgress => Icon::new(IconName::TodoProgress)
                                .size(IconSize::Small)
                                .color(Color::Accent)
                                .with_animation(
                                    "running",
                                    Animation::new(Duration::from_secs(2)).repeat(),
                                    |icon, delta| {
                                        icon.transform(Transformation::rotate(percentage(delta)))
                                    },
                                )
                                .into_any_element(),
                            acp::PlanEntryStatus::Completed => Icon::new(IconName::TodoComplete)
                                .size(IconSize::Small)
                                .color(Color::Success)
                                .into_any_element(),
                        })
                        .child(MarkdownElement::new(
                            entry.content.clone(),
                            plan_label_markdown_style(&entry.status, window, cx),
                        )),
                );

            Some(element)
        }))
    }

    fn render_edits_summary(
        &self,
        changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>,
        expanded: bool,
        pending_edits: bool,
        window: &mut Window,
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
                            this.child(
                                Label::new("Edits")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new("•").size(LabelSize::XSmall).color(Color::Muted))
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
                        }
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.edits_expanded = !this.edits_expanded;
                        cx.notify();
                    })),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("review-changes", IconName::ListTodo)
                            .icon_size(IconSize::Small)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |window, cx| {
                                    Tooltip::for_action_in(
                                        "Review Changes",
                                        &OpenAgentDiff,
                                        &focus_handle,
                                        window,
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
                                    window,
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
                                KeyBinding::for_action_in(&KeepAll, &focus_handle, window, cx)
                                    .map(|kb| kb.size(rems_from_px(10.))),
                            )
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.keep_all(&KeepAll, window, cx);
                            })),
                    ),
            )
    }

    fn render_edited_files(
        &self,
        action_log: &Entity<ActionLog>,
        changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>,
        pending_edits: bool,
        cx: &Context<Self>,
    ) -> Div {
        let editor_bg_color = cx.theme().colors().editor_background;

        v_flex().children(changed_buffers.iter().enumerate().flat_map(
            |(index, (buffer, _diff))| {
                let file = buffer.read(cx).file()?;
                let path = file.path();

                let file_path = path.parent().and_then(|parent| {
                    let parent_str = parent.to_string_lossy();

                    if parent_str.is_empty() {
                        None
                    } else {
                        Some(
                            Label::new(format!("/{}{}", parent_str, std::path::MAIN_SEPARATOR_STR))
                                .color(Color::Muted)
                                .size(LabelSize::XSmall)
                                .buffer_font(cx),
                        )
                    }
                });

                let file_name = path.file_name().map(|name| {
                    Label::new(name.to_string_lossy().to_string())
                        .size(LabelSize::XSmall)
                        .buffer_font(cx)
                });

                let file_icon = FileIcons::get_icon(path, cx)
                    .map(Icon::from_path)
                    .map(|icon| icon.color(Color::Muted).size(IconSize::Small))
                    .unwrap_or_else(|| {
                        Icon::new(IconName::File)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    });

                let overlay_gradient = linear_gradient(
                    90.,
                    linear_color_stop(editor_bg_color, 1.),
                    linear_color_stop(editor_bg_color.opacity(0.2), 0.),
                );

                let element = h_flex()
                    .group("edited-code")
                    .id(("file-container", index))
                    .py_1()
                    .pl_2()
                    .pr_1()
                    .gap_2()
                    .justify_between()
                    .bg(editor_bg_color)
                    .when(index < changed_buffers.len() - 1, |parent| {
                        parent.border_color(cx.theme().colors().border).border_b_1()
                    })
                    .child(
                        h_flex()
                            .relative()
                            .id(("file-name", index))
                            .pr_8()
                            .gap_1p5()
                            .max_w_full()
                            .overflow_x_scroll()
                            .child(file_icon)
                            .child(h_flex().gap_0p5().children(file_name).children(file_path))
                            .child(
                                div()
                                    .absolute()
                                    .h_full()
                                    .w_12()
                                    .top_0()
                                    .bottom_0()
                                    .right_0()
                                    .bg(overlay_gradient),
                            )
                            .on_click({
                                let buffer = buffer.clone();
                                cx.listener(move |this, _, window, cx| {
                                    this.open_edited_buffer(&buffer, window, cx);
                                })
                            }),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .visible_on_hover("edited-code")
                            .child(
                                Button::new("review", "Review")
                                    .label_size(LabelSize::Small)
                                    .on_click({
                                        let buffer = buffer.clone();
                                        cx.listener(move |this, _, window, cx| {
                                            this.open_edited_buffer(&buffer, window, cx);
                                        })
                                    }),
                            )
                            .child(Divider::vertical().color(DividerColor::BorderVariant))
                            .child(
                                Button::new("reject-file", "Reject")
                                    .label_size(LabelSize::Small)
                                    .disabled(pending_edits)
                                    .on_click({
                                        let buffer = buffer.clone();
                                        let action_log = action_log.clone();
                                        move |_, _, cx| {
                                            action_log.update(cx, |action_log, cx| {
                                                action_log
                                                    .reject_edits_in_ranges(
                                                        buffer.clone(),
                                                        vec![Anchor::MIN..Anchor::MAX],
                                                        cx,
                                                    )
                                                    .detach_and_log_err(cx);
                                            })
                                        }
                                    }),
                            )
                            .child(
                                Button::new("keep-file", "Keep")
                                    .label_size(LabelSize::Small)
                                    .disabled(pending_edits)
                                    .on_click({
                                        let buffer = buffer.clone();
                                        let action_log = action_log.clone();
                                        move |_, _, cx| {
                                            action_log.update(cx, |action_log, cx| {
                                                action_log.keep_edits_in_range(
                                                    buffer.clone(),
                                                    Anchor::MIN..Anchor::MAX,
                                                    cx,
                                                );
                                            })
                                        }
                                    }),
                            ),
                    );

                Some(element)
            },
        ))
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
            ThreadState::Loading { .. } | ThreadState::Ready { .. } => true,
            ThreadState::Unauthenticated { .. } | ThreadState::LoadError(..) => false,
        };

        v_flex()
            .on_action(cx.listener(Self::expand_message_editor))
            .on_action(cx.listener(|this, _: &ToggleProfileSelector, window, cx| {
                if let Some(profile_selector) = this.profile_selector.as_ref() {
                    profile_selector.read(cx).menu_handle().toggle(window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &ToggleModelSelector, window, cx| {
                if let Some(model_selector) = this.model_selector.as_ref() {
                    model_selector
                        .update(cx, |model_selector, cx| model_selector.toggle(window, cx));
                }
            }))
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
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                expand_tooltip,
                                                &ExpandMessageEditor,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click(cx.listener(|_, _, window, cx| {
                                        window.dispatch_action(Box::new(ExpandMessageEditor), cx);
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
                            .child(self.render_follow_toggle(cx))
                            .children(self.render_burn_mode_toggle(cx)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .children(self.render_token_usage(cx))
                            .children(self.profile_selector.clone())
                            .children(self.model_selector.clone())
                            .child(self.render_send_button(cx)),
                    ),
            )
            .when(!enable_editor, |this| this.child(backdrop))
            .into_any()
    }

    pub(crate) fn as_native_connection(
        &self,
        cx: &App,
    ) -> Option<Rc<agent2::NativeAgentConnection>> {
        let acp_thread = self.thread()?.read(cx);
        acp_thread.connection().clone().downcast()
    }

    pub(crate) fn as_native_thread(&self, cx: &App) -> Option<Entity<agent2::Thread>> {
        let acp_thread = self.thread()?.read(cx);
        self.as_native_connection(cx)?
            .thread(acp_thread.session_id(), cx)
    }

    fn is_using_zed_ai_models(&self, cx: &App) -> bool {
        self.as_native_thread(cx)
            .and_then(|thread| thread.read(cx).model())
            .is_some_and(|model| model.provider_id() == language_model::ZED_CLOUD_PROVIDER_ID)
    }

    fn render_token_usage(&self, cx: &mut Context<Self>) -> Option<Div> {
        let thread = self.thread()?.read(cx);
        let usage = thread.token_usage()?;
        let is_generating = thread.status() != ThreadStatus::Idle;

        let used = crate::text_thread_editor::humanize_token_count(usage.used_tokens);
        let max = crate::text_thread_editor::humanize_token_count(usage.max_tokens);

        Some(
            h_flex()
                .flex_shrink_0()
                .gap_0p5()
                .mr_1p5()
                .child(
                    Label::new(used)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .map(|label| {
                            if is_generating {
                                label
                                    .with_animation(
                                        "used-tokens-label",
                                        Animation::new(Duration::from_secs(2))
                                            .repeat()
                                            .with_easing(pulsating_between(0.3, 0.8)),
                                        |label, delta| label.alpha(delta),
                                    )
                                    .into_any()
                            } else {
                                label.into_any_element()
                            }
                        }),
                )
                .child(
                    Label::new("/")
                        .size(LabelSize::Small)
                        .color(Color::Custom(cx.theme().colors().text_muted.opacity(0.5))),
                )
                .child(Label::new(max).size(LabelSize::Small).color(Color::Muted)),
        )
    }

    fn toggle_burn_mode(
        &mut self,
        _: &ToggleBurnMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.as_native_thread(cx) else {
            return;
        };

        thread.update(cx, |thread, cx| {
            let current_mode = thread.completion_mode();
            thread.set_completion_mode(
                match current_mode {
                    CompletionMode::Burn => CompletionMode::Normal,
                    CompletionMode::Normal => CompletionMode::Burn,
                },
                cx,
            );
        });
    }

    fn keep_all(&mut self, _: &KeepAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };
        let action_log = thread.read(cx).action_log().clone();
        action_log.update(cx, |action_log, cx| action_log.keep_all_edits(cx));
    }

    fn reject_all(&mut self, _: &RejectAll, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread() else {
            return;
        };
        let action_log = thread.read(cx).action_log().clone();
        action_log
            .update(cx, |action_log, cx| action_log.reject_all_edits(cx))
            .detach();
    }

    fn render_burn_mode_toggle(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let thread = self.as_native_thread(cx)?.read(cx);

        if thread
            .model()
            .is_none_or(|model| !model.supports_burn_mode())
        {
            return None;
        }

        let active_completion_mode = thread.completion_mode();
        let burn_mode_enabled = active_completion_mode == CompletionMode::Burn;
        let icon = if burn_mode_enabled {
            IconName::ZedBurnModeOn
        } else {
            IconName::ZedBurnMode
        };

        Some(
            IconButton::new("burn-mode", icon)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .toggle_state(burn_mode_enabled)
                .selected_icon_color(Color::Error)
                .on_click(cx.listener(|this, _event, window, cx| {
                    this.toggle_burn_mode(&ToggleBurnMode, window, cx);
                }))
                .tooltip(move |_window, cx| {
                    cx.new(|_| BurnModeTooltip::new().selected(burn_mode_enabled))
                        .into()
                })
                .into_any_element(),
        )
    }

    fn render_send_button(&self, cx: &mut Context<Self>) -> AnyElement {
        let is_editor_empty = self.message_editor.read(cx).is_empty(cx);
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
                .style(ButtonStyle::Tinted(ui::TintColor::Error))
                .tooltip(move |window, cx| {
                    Tooltip::for_action("Stop Generation", &editor::actions::Cancel, window, cx)
                })
                .on_click(cx.listener(|this, _event, _, cx| this.cancel_generation(cx)))
                .into_any_element()
        } else {
            let send_btn_tooltip = if is_editor_empty && !is_generating {
                "Type to Send"
            } else if is_generating {
                "Stop and Send Message"
            } else {
                "Send"
            };

            IconButton::new("send-message", IconName::Send)
                .style(ButtonStyle::Filled)
                .map(|this| {
                    if is_editor_empty && !is_generating {
                        this.disabled(true).icon_color(Color::Muted)
                    } else {
                        this.icon_color(Color::Accent)
                    }
                })
                .tooltip(move |window, cx| Tooltip::for_action(send_btn_tooltip, &Chat, window, cx))
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
            .tooltip(move |window, cx| {
                if following {
                    Tooltip::for_action(tooltip_label.clone(), &Follow, window, cx)
                } else {
                    Tooltip::with_meta(
                        tooltip_label.clone(),
                        Some(&Follow),
                        "Track the agent's location as it reads and edits files.",
                        window,
                        cx,
                    )
                }
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.toggle_following(window, cx);
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

        if let Some(mention) = MentionUri::parse(&url).log_err() {
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
                    let Some(entry) = project.update(cx, |project, cx| {
                        let path = project.find_project_path(abs_path, cx)?;
                        project.entry_for_path(&path, cx)
                    }) else {
                        return;
                    };

                    project.update(cx, |_, cx| {
                        cx.emit(project::Event::RevealInProjectPanel(entry.id));
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
                    let Some((path, _)) = project.update(cx, |project, cx| {
                        let path = project.find_project_path(path, cx)?;
                        let entry = project.entry_for_path(&path, cx)?;
                        Some((path, entry))
                    }) else {
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
                            panel.load_agent_thread(
                                DbThreadMetadata {
                                    id,
                                    title: name.into(),
                                    updated_at: Default::default(),
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
                                .open_saved_prompt_editor(path.as_path().into(), window, cx)
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
                        let anchor = editor::Anchor::in_buffer(
                            excerpt_id.unwrap(),
                            buffer.unwrap().read(cx).remote_id(),
                            agent_location.position,
                        );
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

        let (thread_summary, markdown) = if let Some(thread) = self.thread() {
            let thread = thread.read(cx);
            (thread.title().to_string(), thread.to_markdown(cx))
        } else {
            return Task::ready(Ok(()));
        };

        window.spawn(cx, async move |cx| {
            let markdown_language = markdown_language_task.await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let project = workspace.project().clone();

                if !project.read(cx).is_local() {
                    bail!("failed to open active thread as markdown in remote project");
                }

                let buffer = project.update(cx, |project, cx| {
                    project.create_local_buffer(&markdown, Some(markdown_language), cx)
                });
                let buffer = cx.new(|cx| {
                    MultiBuffer::singleton(buffer, cx).with_title(thread_summary.clone())
                });

                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|cx| {
                        let mut editor =
                            Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                        editor.set_breadcrumb_header(thread_summary);
                        editor
                    })),
                    None,
                    true,
                    window,
                    cx,
                );

                anyhow::Ok(())
            })??;
            anyhow::Ok(())
        })
    }

    fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        self.list_state.scroll_to(ListOffset::default());
        cx.notify();
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
        if window.is_window_active() || !self.notifications.is_empty() {
            return;
        }

        // TODO: Change this once we have title summarization for external agents.
        let title = self.agent.name();

        match AgentSettings::get_global(cx).notify_when_agent_waiting {
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
                .map(|worktree| worktree.read(cx).root_name().to_string())
        });

        if let Some(screen_window) = cx
            .open_window(options, |_, cx| {
                cx.new(|_| {
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

    fn render_thread_controls(
        &self,
        thread: &Entity<AcpThread>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_generating = matches!(thread.read(cx).status(), ThreadStatus::Generating);
        if is_generating {
            return h_flex().id("thread-controls-container").child(
                div()
                    .py_2()
                    .px(rems_from_px(22.))
                    .child(SpinnerLabel::new().size(LabelSize::Small)),
            );
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

        let scroll_to_top = IconButton::new("scroll_to_top", IconName::ArrowUp)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Scroll To Top"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.scroll_to_top(cx);
            }));

        let mut container = h_flex()
            .id("thread-controls-container")
            .group("thread-controls-container")
            .w_full()
            .py_2()
            .px_5()
            .gap_px()
            .opacity(0.6)
            .hover(|style| style.opacity(1.))
            .flex_wrap()
            .justify_end();

        if AgentSettings::get_global(cx).enable_feedback
            && self
                .thread()
                .is_some_and(|thread| thread.read(cx).connection().telemetry().is_some())
        {
            let feedback = self.thread_feedback.feedback;

            container = container
                .child(
                    div().visible_on_hover("thread-controls-container").child(
                        Label::new(match feedback {
                            Some(ThreadFeedback::Positive) => "Thanks for your feedback!",
                            Some(ThreadFeedback::Negative) => {
                                "We appreciate your feedback and will use it to improve."
                            }
                            None => {
                                "Rating the thread sends all of your current conversation to the Zed team."
                            }
                        })
                        .color(Color::Muted)
                        .size(LabelSize::XSmall)
                        .truncate(),
                    ),
                )
                .child(
                    IconButton::new("feedback-thumbs-up", IconName::ThumbsUp)
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(match feedback {
                            Some(ThreadFeedback::Positive) => Color::Accent,
                            _ => Color::Ignored,
                        })
                        .tooltip(Tooltip::text("Helpful Response"))
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
                        .tooltip(Tooltip::text("Not Helpful"))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.handle_feedback_click(ThreadFeedback::Negative, window, cx);
                        })),
                );
        }

        container.child(open_as_markdown).child(scroll_to_top)
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

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .id("acp-thread-scrollbar")
            .occlude()
            .on_mouse_move(cx.listener(|_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|_, _, _, cx| {
                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .h_full()
            .absolute()
            .right_1()
            .top_1()
            .bottom_0()
            .w(px(12.))
            .cursor_default()
            .children(Scrollbar::vertical(self.scrollbar_state.clone()).map(|s| s.auto_hide(cx)))
    }

    fn render_token_limit_callout(
        &self,
        line_height: Pixels,
        cx: &mut Context<Self>,
    ) -> Option<Callout> {
        let token_usage = self.thread()?.read(cx).token_usage()?;
        let ratio = token_usage.ratio();

        let (severity, title) = match ratio {
            acp_thread::TokenUsageRatio::Normal => return None,
            acp_thread::TokenUsageRatio::Warning => {
                (Severity::Warning, "Thread reaching the token limit soon")
            }
            acp_thread::TokenUsageRatio::Exceeded => {
                (Severity::Error, "Thread reached the token limit")
            }
        };

        let burn_mode_available = self.as_native_thread(cx).is_some_and(|thread| {
            thread.read(cx).completion_mode() == CompletionMode::Normal
                && thread
                    .read(cx)
                    .model()
                    .is_some_and(|model| model.supports_burn_mode())
        });

        let description = if burn_mode_available {
            "To continue, start a new thread from a summary or turn Burn Mode on."
        } else {
            "To continue, start a new thread from a summary."
        };

        Some(
            Callout::new()
                .severity(severity)
                .line_height(line_height)
                .title(title)
                .description(description)
                .actions_slot(
                    h_flex()
                        .gap_0p5()
                        .child(
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
                        )
                        .when(burn_mode_available, |this| {
                            this.child(
                                IconButton::new("burn-mode-callout", IconName::ZedBurnMode)
                                    .icon_size(IconSize::XSmall)
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        this.toggle_burn_mode(&ToggleBurnMode, window, cx);
                                    })),
                            )
                        }),
                ),
        )
    }

    fn render_usage_callout(&self, line_height: Pixels, cx: &mut Context<Self>) -> Option<Div> {
        if !self.is_using_zed_ai_models(cx) {
            return None;
        }

        let user_store = self.project.read(cx).user_store().read(cx);
        if user_store.is_usage_based_billing_enabled() {
            return None;
        }

        let plan = user_store.plan().unwrap_or(cloud_llm_client::Plan::ZedFree);

        let usage = user_store.model_request_usage()?;

        Some(
            div()
                .child(UsageCallout::new(plan, usage))
                .line_height(line_height),
        )
    }

    fn settings_changed(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.entry_view_state.update(cx, |entry_view_state, cx| {
            entry_view_state.settings_changed(cx);
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

    pub(crate) fn insert_selections(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.message_editor.update(cx, |message_editor, cx| {
            message_editor.insert_selections(window, cx);
        })
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

    fn render_thread_error(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<Div> {
        let content = match self.thread_error.as_ref()? {
            ThreadError::Other(error) => self.render_any_thread_error(error.clone(), cx),
            ThreadError::AuthenticationRequired(error) => {
                self.render_authentication_required_error(error.clone(), cx)
            }
            ThreadError::PaymentRequired => self.render_payment_required_error(cx),
            ThreadError::ModelRequestLimitReached(plan) => {
                self.render_model_request_limit_reached_error(*plan, cx)
            }
            ThreadError::ToolUseLimitReached => {
                self.render_tool_use_limit_reached_error(window, cx)?
            }
        };

        Some(div().child(content))
    }

    fn render_any_thread_error(&self, error: SharedString, cx: &mut Context<'_, Self>) -> Callout {
        let can_resume = self
            .thread()
            .map_or(false, |thread| thread.read(cx).can_resume(cx));

        let can_enable_burn_mode = self.as_native_thread(cx).map_or(false, |thread| {
            let thread = thread.read(cx);
            let supports_burn_mode = thread
                .model()
                .map_or(false, |model| model.supports_burn_mode());
            supports_burn_mode && thread.completion_mode() == CompletionMode::Normal
        });

        Callout::new()
            .severity(Severity::Error)
            .title("Error")
            .icon(IconName::XCircle)
            .description(error.clone())
            .actions_slot(
                h_flex()
                    .gap_0p5()
                    .when(can_resume && can_enable_burn_mode, |this| {
                        this.child(
                            Button::new("enable-burn-mode-and-retry", "Enable Burn Mode and Retry")
                                .icon(IconName::ZedBurnMode)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.toggle_burn_mode(&ToggleBurnMode, window, cx);
                                    this.resume_chat(cx);
                                })),
                        )
                    })
                    .when(can_resume, |this| {
                        this.child(
                            Button::new("retry", "Retry")
                                .icon(IconName::RotateCw)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
                                .label_size(LabelSize::Small)
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

    fn render_model_request_limit_reached_error(
        &self,
        plan: cloud_llm_client::Plan,
        cx: &mut Context<Self>,
    ) -> Callout {
        let error_message = match plan {
            cloud_llm_client::Plan::ZedPro => "Upgrade to usage-based billing for more prompts.",
            cloud_llm_client::Plan::ZedProTrial | cloud_llm_client::Plan::ZedFree => {
                "Upgrade to Zed Pro for more prompts."
            }
        };

        Callout::new()
            .severity(Severity::Error)
            .title("Model Prompt Limit Reached")
            .icon(IconName::XCircle)
            .description(error_message)
            .actions_slot(
                h_flex()
                    .gap_0p5()
                    .child(self.upgrade_button(cx))
                    .child(self.create_copy_button(error_message)),
            )
            .dismiss_action(self.dismiss_error_button(cx))
    }

    fn render_tool_use_limit_reached_error(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Callout> {
        let thread = self.as_native_thread(cx)?;
        let supports_burn_mode = thread
            .read(cx)
            .model()
            .is_some_and(|model| model.supports_burn_mode());

        let focus_handle = self.focus_handle(cx);

        Some(
            Callout::new()
                .icon(IconName::Info)
                .title("Consecutive tool use limit reached.")
                .actions_slot(
                    h_flex()
                        .gap_0p5()
                        .when(supports_burn_mode, |this| {
                            this.child(
                                Button::new("continue-burn-mode", "Continue with Burn Mode")
                                    .style(ButtonStyle::Filled)
                                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                    .layer(ElevationIndex::ModalSurface)
                                    .label_size(LabelSize::Small)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &ContinueWithBurnMode,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(10.))),
                                    )
                                    .tooltip(Tooltip::text(
                                        "Enable Burn Mode for unlimited tool use.",
                                    ))
                                    .on_click({
                                        cx.listener(move |this, _, _window, cx| {
                                            thread.update(cx, |thread, cx| {
                                                thread
                                                    .set_completion_mode(CompletionMode::Burn, cx);
                                            });
                                            this.resume_chat(cx);
                                        })
                                    }),
                            )
                        })
                        .child(
                            Button::new("continue-conversation", "Continue")
                                .layer(ElevationIndex::ModalSurface)
                                .label_size(LabelSize::Small)
                                .key_binding(
                                    KeyBinding::for_action_in(
                                        &ContinueThread,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(10.))),
                                )
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.resume_chat(cx);
                                })),
                        ),
                ),
        )
    }

    fn create_copy_button(&self, message: impl Into<String>) -> impl IntoElement {
        let message = message.into();

        IconButton::new("copy", IconName::Copy)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .tooltip(Tooltip::text("Copy Error Message"))
            .on_click(move |_, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(message.clone()))
            })
    }

    fn dismiss_error_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        IconButton::new("dismiss", IconName::Close)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .tooltip(Tooltip::text("Dismiss Error"))
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
                    let err = AuthRequired {
                        description: None,
                        provider_id: None,
                    };
                    this.clear_thread_error(cx);
                    let this = cx.weak_entity();
                    window.defer(cx, |window, cx| {
                        Self::handle_auth_required(this, err, agent, connection, window, cx);
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
        let err = AuthRequired {
            description: None,
            provider_id: None,
        };
        self.clear_thread_error(cx);
        let this = cx.weak_entity();
        window.defer(cx, |window, cx| {
            Self::handle_auth_required(this, err, agent, connection, window, cx);
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

    pub fn delete_history_entry(&mut self, entry: HistoryEntry, cx: &mut Context<Self>) {
        let task = match entry {
            HistoryEntry::AcpThread(thread) => self.history_store.update(cx, |history, cx| {
                history.delete_thread(thread.id.clone(), cx)
            }),
            HistoryEntry::TextThread(context) => self.history_store.update(cx, |history, cx| {
                history.delete_text_thread(context.path.clone(), cx)
            }),
        };
        task.detach_and_log_err(cx);
    }
}

fn loading_contents_spinner(size: IconSize) -> AnyElement {
    Icon::new(IconName::LoadCircle)
        .size(size)
        .color(Color::Accent)
        .with_animation(
            "load_context_circle",
            Animation::new(Duration::from_secs(3)).repeat(),
            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
        )
        .into_any_element()
}

impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.thread_state {
            ThreadState::Loading { .. } | ThreadState::Ready { .. } => {
                self.message_editor.focus_handle(cx)
            }
            ThreadState::LoadError(_) | ThreadState::Unauthenticated { .. } => {
                self.focus_handle.clone()
            }
        }
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_messages = self.list_state.item_count() > 0;
        let line_height = TextSize::Small.rems(cx).to_pixels(window.rem_size()) * 1.5;

        v_flex()
            .size_full()
            .key_context("AcpThread")
            .on_action(cx.listener(Self::open_agent_diff))
            .on_action(cx.listener(Self::toggle_burn_mode))
            .on_action(cx.listener(Self::keep_all))
            .on_action(cx.listener(Self::reject_all))
            .track_focus(&self.focus_handle)
            .bg(cx.theme().colors().panel_background)
            .child(match &self.thread_state {
                ThreadState::Unauthenticated {
                    connection,
                    description,
                    configuration_view,
                    pending_auth_method,
                    ..
                } => self.render_auth_required_state(
                    connection,
                    description.as_ref(),
                    configuration_view.as_ref(),
                    pending_auth_method.as_ref(),
                    window,
                    cx,
                ),
                ThreadState::Loading { .. } => v_flex()
                    .flex_1()
                    .child(self.render_recent_history(window, cx)),
                ThreadState::LoadError(e) => v_flex()
                    .flex_1()
                    .size_full()
                    .items_center()
                    .justify_end()
                    .child(self.render_load_error(e, window, cx)),
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
                        .child(self.render_vertical_scrollbar(cx))
                    } else {
                        this.child(self.render_recent_history(window, cx))
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
            .children(self.render_thread_error(window, cx))
            .children(
                if let Some(usage_callout) = self.render_usage_callout(line_height, cx) {
                    Some(usage_callout.into_any_element())
                } else {
                    self.render_token_limit_callout(line_height, cx)
                        .map(|token_limit_callout| token_limit_callout.into_any_element())
                },
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

    let buffer_font_size = TextSize::Small.rems(cx);

    let mut text_style = window.text_style();
    let line_height = buffer_font_size * 1.75;

    let font_family = if buffer_font {
        theme_settings.buffer_font.family.clone()
    } else {
        theme_settings.ui_font.family.clone()
    };

    let font_size = if buffer_font {
        TextSize::Small.rems(cx)
    } else {
        TextSize::Default.rems(cx)
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
        table_overflow_x_scroll: true,
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
                top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
            },
            margin: EdgesRefinement {
                top: Some(Length::Definite(Pixels(8.).into())),
                left: Some(Length::Definite(Pixels(0.).into())),
                right: Some(Length::Definite(Pixels(0.).into())),
                bottom: Some(Length::Definite(Pixels(12.).into())),
            },
            border_style: Some(BorderStyle::Solid),
            border_widths: EdgesRefinement {
                top: Some(AbsoluteLength::Pixels(Pixels(1.))),
                left: Some(AbsoluteLength::Pixels(Pixels(1.))),
                right: Some(AbsoluteLength::Pixels(Pixels(1.))),
                bottom: Some(AbsoluteLength::Pixels(Pixels(1.))),
            },
            border_color: Some(colors.border_variant),
            background: Some(colors.editor_background.into()),
            text: Some(TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
                font_features: Some(theme_settings.buffer_font.features.clone()),
                font_size: Some(buffer_font_size.into()),
                ..Default::default()
            }),
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

fn terminal_command_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let default_md_style = default_markdown_style(true, false, window, cx);

    MarkdownStyle {
        base_text_style: TextStyle {
            ..default_md_style.base_text_style
        },
        selection_background_color: cx.theme().colors().element_selection_background,
        ..Default::default()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use acp_thread::StubAgentConnection;
    use agent_client_protocol::SessionId;
    use assistant_context::ContextStore;
    use editor::EditorSettings;
    use fs::FakeFs;
    use gpui::{EventEmitter, SemanticVersion, TestAppContext, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::any::Any;
    use std::path::Path;
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
    async fn test_notification_for_tool_authorization(cx: &mut TestAppContext) {
        init_test(cx);

        let tool_call_id = acp::ToolCallId("1".into());
        let tool_call = acp::ToolCall {
            id: tool_call_id.clone(),
            title: "Label".into(),
            kind: acp::ToolKind::Edit,
            status: acp::ToolCallStatus::Pending,
            content: vec!["hi".into()],
            locations: vec![],
            raw_input: None,
            raw_output: None,
        };
        let connection =
            StubAgentConnection::new().with_permission_requests(HashMap::from_iter([(
                tool_call_id,
                vec![acp::PermissionOption {
                    id: acp::PermissionOptionId("1".into()),
                    name: "Allow".into(),
                    kind: acp::PermissionOptionKind::AllowOnce,
                }],
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

    async fn setup_thread_view(
        agent: impl AgentServer + 'static,
        cx: &mut TestAppContext,
    ) -> (Entity<AcpThreadView>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let context_store =
            cx.update(|_window, cx| cx.new(|cx| ContextStore::fake(project.clone(), cx)));
        let history_store =
            cx.update(|_window, cx| cx.new(|cx| HistoryStore::new(context_store, cx)));

        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpThreadView::new(
                    Rc::new(agent),
                    None,
                    None,
                    workspace.downgrade(),
                    project,
                    history_store,
                    None,
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
            conn.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk {
                content: "Default response".into(),
            }]);
            Self::new(conn)
        }
    }

    impl<C> AgentServer for StubAgentServer<C>
    where
        C: 'static + AgentConnection + Send + Clone,
    {
        fn telemetry_id(&self) -> &'static str {
            "test"
        }

        fn logo(&self) -> ui::IconName {
            ui::IconName::Ai
        }

        fn name(&self) -> SharedString {
            "Test".into()
        }

        fn connect(
            &self,
            _root_dir: &Path,
            _delegate: AgentServerDelegate,
            _cx: &mut App,
        ) -> Task<gpui::Result<Rc<dyn AgentConnection>>> {
            Task::ready(Ok(Rc::new(self.connection.clone())))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[derive(Clone)]
    struct SaboteurAgentConnection;

    impl AgentConnection for SaboteurAgentConnection {
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
                    SessionId("test".into()),
                    watch::Receiver::constant(acp::PromptCapabilities {
                        image: true,
                        audio: true,
                        embedded_context: true,
                    }),
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

    pub(crate) fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            AgentSettings::register(cx);
            workspace::init_settings(cx);
            ThemeSettings::register(cx);
            release_channel::init(SemanticVersion::default(), cx);
            EditorSettings::register(cx);
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

        let context_store =
            cx.update(|_window, cx| cx.new(|cx| ContextStore::fake(project.clone(), cx)));
        let history_store =
            cx.update(|_window, cx| cx.new(|cx| HistoryStore::new(context_store, cx)));

        let connection = Rc::new(StubAgentConnection::new());
        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpThreadView::new(
                    Rc::new(StubAgentServer::new(connection.as_ref().clone())),
                    None,
                    None,
                    workspace.downgrade(),
                    project.clone(),
                    history_store.clone(),
                    None,
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
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(acp::ToolCall {
            id: acp::ToolCallId("tool1".into()),
            title: "Edit file 1".into(),
            kind: acp::ToolKind::Edit,
            status: acp::ToolCallStatus::Completed,
            content: vec![acp::ToolCallContent::Diff {
                diff: acp::Diff {
                    path: "/project/test1.txt".into(),
                    old_text: Some("old content 1".into()),
                    new_text: "new content 1".into(),
                },
            }],
            locations: vec![],
            raw_input: None,
            raw_output: None,
        })]);

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
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::ToolCall(acp::ToolCall {
            id: acp::ToolCallId("tool2".into()),
            title: "Edit file 2".into(),
            kind: acp::ToolKind::Edit,
            status: acp::ToolCallStatus::Completed,
            content: vec![acp::ToolCallContent::Diff {
                diff: acp::Diff {
                    path: "/project/test2.txt".into(),
                    old_text: Some("old content 2".into()),
                    new_text: "new content 2".into(),
                },
            }],
            locations: vec![],
            raw_input: None,
            raw_output: None,
        })]);

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
    async fn test_message_editing_cancel(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk {
            content: acp::ContentBlock::Text(acp::TextContent {
                text: "Response".into(),
                annotations: None,
            }),
        }]);

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
        let mut events = cx.events(&message_editor);
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("", window, cx);
        });

        message_editor.update_in(cx, |_editor, window, cx| {
            window.dispatch_action(Box::new(Chat), cx);
        });
        cx.run_until_parked();
        // We shouldn't have received any messages
        assert!(matches!(
            events.try_next(),
            Err(futures::channel::mpsc::TryRecvError { .. })
        ));
    }

    #[gpui::test]
    async fn test_message_editing_regenerate(cx: &mut TestAppContext) {
        init_test(cx);

        let connection = StubAgentConnection::new();

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk {
            content: acp::ContentBlock::Text(acp::TextContent {
                text: "Response".into(),
                annotations: None,
            }),
        }]);

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
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk {
            content: acp::ContentBlock::Text(acp::TextContent {
                text: "New Response".into(),
                annotations: None,
            }),
        }]);

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
                acp::SessionUpdate::AgentMessageChunk {
                    content: acp::ContentBlock::Text(acp::TextContent {
                        text: "Response".into(),
                        annotations: None,
                    }),
                },
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
                acp::SessionUpdate::AgentMessageChunk {
                    content: "Message 1 resp".into(),
                },
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
            thread_view.send(window, cx);
        });

        cx.update(|_, cx| {
            // Simulate a response sent after beginning to cancel
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk {
                    content: "onse".into(),
                },
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
                acp::SessionUpdate::AgentMessageChunk {
                    content: "Message 2 response".into(),
                },
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
}
