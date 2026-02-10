use gpui::{Corner, List};
use language_model::LanguageModelEffortLevel;
use settings::update_settings_file;
use ui::{ButtonLike, SplitButton, SplitButtonStyle, Tab};

use super::*;

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
    pub id: acp::SessionId,
    pub parent_id: Option<acp::SessionId>,
    pub login: Option<task::SpawnInTerminal>, // is some <=> Active | Unauthenticated
    pub thread: Entity<AcpThread>,
    pub server_view: WeakEntity<AcpServerView>,
    pub agent_icon: IconName,
    pub agent_name: SharedString,
    pub focus_handle: FocusHandle,
    pub workspace: WeakEntity<Workspace>,
    pub entry_view_state: Entity<EntryViewState>,
    pub title_editor: Option<Entity<Editor>>,
    pub config_options_view: Option<Entity<ConfigOptionsView>>,
    pub mode_selector: Option<Entity<ModeSelector>>,
    pub model_selector: Option<Entity<AcpModelSelectorPopover>>,
    pub profile_selector: Option<Entity<ProfileSelector>>,
    pub permission_dropdown_handle: PopoverMenuHandle<ContextMenu>,
    pub thread_retry_status: Option<RetryStatus>,
    pub(super) thread_error: Option<ThreadError>,
    pub thread_error_markdown: Option<Entity<Markdown>>,
    pub token_limit_callout_dismissed: bool,
    pub last_token_limit_telemetry: Option<acp_thread::TokenUsageRatio>,
    thread_feedback: ThreadFeedbackState,
    pub list_state: ListState,
    pub prompt_capabilities: Rc<RefCell<PromptCapabilities>>,
    pub available_commands: Rc<RefCell<Vec<agent_client_protocol::AvailableCommand>>>,
    /// Tracks which tool calls have their content/output expanded.
    /// Used for showing/hiding tool call results, terminal output, etc.
    pub expanded_tool_calls: HashSet<agent_client_protocol::ToolCallId>,
    pub expanded_tool_call_raw_inputs: HashSet<agent_client_protocol::ToolCallId>,
    pub expanded_thinking_blocks: HashSet<(usize, usize)>,
    pub expanded_subagents: HashSet<agent_client_protocol::SessionId>,
    pub subagent_scroll_handles: RefCell<HashMap<agent_client_protocol::SessionId, ScrollHandle>>,
    pub edits_expanded: bool,
    pub plan_expanded: bool,
    pub queue_expanded: bool,
    pub editor_expanded: bool,
    pub should_be_following: bool,
    pub editing_message: Option<usize>,
    pub local_queued_messages: Vec<QueuedMessage>,
    pub queued_message_editors: Vec<Entity<MessageEditor>>,
    pub queued_message_editor_subscriptions: Vec<Subscription>,
    pub last_synced_queue_length: usize,
    pub turn_fields: TurnFields,
    pub discarded_partial_edits: HashSet<agent_client_protocol::ToolCallId>,
    pub is_loading_contents: bool,
    pub new_server_version_available: Option<SharedString>,
    pub resumed_without_history: bool,
    /// Tracks the selected granularity index for each tool call's permission dropdown.
    /// The index corresponds to the position in the allow_options list.
    /// Default is the last option (index pointing to "Only this time").
    pub selected_permission_granularity: HashMap<agent_client_protocol::ToolCallId, usize>,
    pub resume_thread_metadata: Option<AgentSessionInfo>,
    pub _cancel_task: Option<Task<()>>,
    pub skip_queue_processing_count: usize,
    pub user_interrupted_generation: bool,
    pub can_fast_track_queue: bool,
    pub hovered_edited_file_buttons: Option<usize>,
    pub in_flight_prompt: Option<Vec<acp::ContentBlock>>,
    pub _subscriptions: Vec<Subscription>,
    pub message_editor: Entity<MessageEditor>,
    pub add_context_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub thinking_effort_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub project: WeakEntity<Project>,
    pub recent_history_entries: Vec<AgentSessionInfo>,
    pub hovered_recent_history_item: Option<usize>,
    pub show_codex_windows_warning: bool,
    pub history: Entity<AcpThreadHistory>,
    pub _history_subscription: Subscription,
}
impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.parent_id.is_some() {
            self.focus_handle.clone()
        } else {
            self.active_editor(cx).focus_handle(cx)
        }
    }
}

#[derive(Default)]
pub struct TurnFields {
    pub _turn_timer_task: Option<Task<()>>,
    pub last_turn_duration: Option<Duration>,
    pub last_turn_tokens: Option<u64>,
    pub turn_generation: usize,
    pub turn_started_at: Option<Instant>,
    pub turn_tokens: Option<u64>,
}

impl AcpThreadView {
    pub fn new(
        parent_id: Option<acp::SessionId>,
        thread: Entity<AcpThread>,
        login: Option<task::SpawnInTerminal>,
        server_view: WeakEntity<AcpServerView>,
        agent_icon: IconName,
        agent_name: SharedString,
        agent_display_name: SharedString,
        workspace: WeakEntity<Workspace>,
        entry_view_state: Entity<EntryViewState>,
        title_editor: Option<Entity<Editor>>,
        config_options_view: Option<Entity<ConfigOptionsView>>,
        mode_selector: Option<Entity<ModeSelector>>,
        model_selector: Option<Entity<AcpModelSelectorPopover>>,
        profile_selector: Option<Entity<ProfileSelector>>,
        list_state: ListState,
        prompt_capabilities: Rc<RefCell<PromptCapabilities>>,
        available_commands: Rc<RefCell<Vec<agent_client_protocol::AvailableCommand>>>,
        resumed_without_history: bool,
        resume_thread_metadata: Option<AgentSessionInfo>,
        project: WeakEntity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        history: Entity<AcpThreadHistory>,
        prompt_store: Option<Entity<PromptStore>>,
        initial_content: Option<ExternalAgentInitialContent>,
        mut subscriptions: Vec<Subscription>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let id = thread.read(cx).session_id().clone();

        let placeholder = placeholder_text(agent_display_name.as_ref(), false);

        let history_subscription = cx.observe(&history, |this, history, cx| {
            this.update_recent_history_from_cache(&history, cx);
        });

        let message_editor = cx.new(|cx| {
            let mut editor = MessageEditor::new(
                workspace.clone(),
                project.clone(),
                thread_store,
                history.downgrade(),
                prompt_store,
                prompt_capabilities.clone(),
                available_commands.clone(),
                agent_name.clone(),
                &placeholder,
                editor::EditorMode::AutoHeight {
                    min_lines: AgentSettings::get_global(cx).message_editor_min_lines,
                    max_lines: Some(AgentSettings::get_global(cx).set_message_editor_max_lines()),
                },
                window,
                cx,
            );
            if let Some(content) = initial_content {
                match content {
                    ExternalAgentInitialContent::ThreadSummary(entry) => {
                        editor.insert_thread_summary(entry, window, cx);
                    }
                    ExternalAgentInitialContent::Text(prompt) => {
                        editor.set_message(
                            vec![acp::ContentBlock::Text(acp::TextContent::new(prompt))],
                            window,
                            cx,
                        );
                    }
                }
            }
            editor
        });

        let show_codex_windows_warning = cfg!(windows)
            && project.upgrade().is_some_and(|p| p.read(cx).is_local())
            && agent_name == "Codex";

        subscriptions.push(cx.subscribe_in(
            &entry_view_state,
            window,
            Self::handle_entry_view_event,
        ));

        subscriptions.push(cx.subscribe_in(
            &message_editor,
            window,
            Self::handle_message_editor_event,
        ));

        let recent_history_entries = history.read(cx).get_recent_sessions(3);

        Self {
            id,
            parent_id,
            focus_handle: cx.focus_handle(),
            thread,
            login,
            server_view,
            agent_icon,
            agent_name,
            workspace,
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
            resume_thread_metadata,
            _subscriptions: subscriptions,
            permission_dropdown_handle: PopoverMenuHandle::default(),
            thread_retry_status: None,
            thread_error: None,
            thread_error_markdown: None,
            token_limit_callout_dismissed: false,
            last_token_limit_telemetry: None,
            thread_feedback: Default::default(),
            expanded_tool_calls: HashSet::default(),
            expanded_tool_call_raw_inputs: HashSet::default(),
            expanded_thinking_blocks: HashSet::default(),
            expanded_subagents: HashSet::default(),
            subagent_scroll_handles: RefCell::new(HashMap::default()),
            edits_expanded: false,
            plan_expanded: false,
            queue_expanded: true,
            editor_expanded: false,
            should_be_following: false,
            editing_message: None,
            local_queued_messages: Vec::new(),
            queued_message_editors: Vec::new(),
            queued_message_editor_subscriptions: Vec::new(),
            last_synced_queue_length: 0,
            turn_fields: TurnFields::default(),
            discarded_partial_edits: HashSet::default(),
            is_loading_contents: false,
            new_server_version_available: None,
            selected_permission_granularity: HashMap::default(),
            _cancel_task: None,
            skip_queue_processing_count: 0,
            user_interrupted_generation: false,
            can_fast_track_queue: false,
            hovered_edited_file_buttons: None,
            in_flight_prompt: None,
            message_editor,
            add_context_menu_handle: PopoverMenuHandle::default(),
            thinking_effort_menu_handle: PopoverMenuHandle::default(),
            project,
            recent_history_entries,
            hovered_recent_history_item: None,
            history,
            _history_subscription: history_subscription,
            show_codex_windows_warning,
        }
    }

    pub fn handle_message_editor_event(
        &mut self,
        _editor: &Entity<MessageEditor>,
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

    pub(crate) fn as_native_connection(
        &self,
        cx: &App,
    ) -> Option<Rc<agent::NativeAgentConnection>> {
        let acp_thread = self.thread.read(cx);
        acp_thread.connection().clone().downcast()
    }

    pub(crate) fn as_native_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        let acp_thread = self.thread.read(cx);
        self.as_native_connection(cx)?
            .thread(acp_thread.session_id(), cx)
    }

    pub fn current_model_id(&self, cx: &App) -> Option<String> {
        let selector = self.model_selector.as_ref()?;
        let model = selector.read(cx).active_model(cx)?;
        Some(model.id.to_string())
    }

    pub fn current_mode_id(&self, cx: &App) -> Option<Arc<str>> {
        if let Some(thread) = self.as_native_thread(cx) {
            Some(thread.read(cx).profile().0.clone())
        } else {
            let mode_selector = self.mode_selector.as_ref()?;
            Some(mode_selector.read(cx).mode().0)
        }
    }

    fn is_subagent(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Returns the currently active editor, either for a message that is being
    /// edited or the editor for a new message.
    pub(crate) fn active_editor(&self, cx: &App) -> Entity<MessageEditor> {
        if let Some(index) = self.editing_message
            && let Some(editor) = self
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

    pub fn has_queued_messages(&self) -> bool {
        !self.local_queued_messages.is_empty()
    }

    pub fn is_imported_thread(&self, cx: &App) -> bool {
        let Some(thread) = self.as_native_thread(cx) else {
            return false;
        };
        thread.read(cx).is_imported()
    }

    // events

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
                if let Some(AgentThreadEntry::UserMessage(user_message)) =
                    self.thread.read(cx).entries().get(event.entry_index)
                    && user_message.id.is_some()
                {
                    self.editing_message = Some(event.entry_index);
                    cx.notify();
                }
            }
            ViewEvent::MessageEditorEvent(editor, MessageEditorEvent::LostFocus) => {
                if let Some(AgentThreadEntry::UserMessage(user_message)) =
                    self.thread.read(cx).entries().get(event.entry_index)
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

    // turns

    pub fn start_turn(&mut self, cx: &mut Context<Self>) -> usize {
        self.turn_fields.turn_generation += 1;
        let generation = self.turn_fields.turn_generation;
        self.turn_fields.turn_started_at = Some(Instant::now());
        self.turn_fields.last_turn_duration = None;
        self.turn_fields.last_turn_tokens = None;
        self.turn_fields.turn_tokens = Some(0);
        self.turn_fields._turn_timer_task = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        }));
        generation
    }

    pub fn stop_turn(&mut self, generation: usize) {
        if self.turn_fields.turn_generation != generation {
            return;
        }
        self.turn_fields.last_turn_duration = self
            .turn_fields
            .turn_started_at
            .take()
            .map(|started| started.elapsed());
        self.turn_fields.last_turn_tokens = self.turn_fields.turn_tokens.take();
        self.turn_fields._turn_timer_task = None;
    }

    pub fn update_turn_tokens(&mut self, cx: &App) {
        if let Some(usage) = self.thread.read(cx).token_usage() {
            if let Some(tokens) = &mut self.turn_fields.turn_tokens {
                *tokens += usage.output_tokens;
            }
        }
    }

    // sending

    pub fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let thread = &self.thread;

        if self.is_loading_contents {
            return;
        }

        let message_editor = self.message_editor.clone();
        let is_editor_empty = message_editor.read(cx).is_empty(cx);
        let is_generating = thread.read(cx).status() != ThreadStatus::Idle;

        let has_queued = self.has_queued_messages();
        if is_editor_empty && self.can_fast_track_queue && has_queued {
            self.can_fast_track_queue = false;
            self.send_queued_message_at_index(0, true, window, cx);
            return;
        }

        if is_editor_empty {
            return;
        }

        if is_generating {
            self.queue_message(message_editor, window, cx);
            return;
        }

        let text = message_editor.read(cx).text(cx);
        let text = text.trim();
        if text == "/login" || text == "/logout" {
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
                message_editor.update(cx, |editor, cx| editor.clear(window, cx));

                window.defer(cx, {
                    let agent_name = self.agent_name.clone();
                    let server_view = self.server_view.clone();
                    move |window, cx| {
                        AcpServerView::handle_auth_required(
                            server_view.clone(),
                            AuthRequired::new(),
                            agent_name,
                            window,
                            cx,
                        );
                    }
                });
                cx.notify();
                return;
            }
        }

        self.send_impl(message_editor, window, cx)
    }

    pub fn send_impl(
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
        self.thread_feedback.clear();
        self.editing_message.take();

        if self.should_be_following {
            self.workspace
                .update(cx, |workspace, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .ok();
        }

        let contents_task = cx.spawn_in(window, async move |_this, cx| {
            let (contents, tracked_buffers) = contents.await?;

            if contents.is_empty() {
                return Ok(None);
            }

            let _ = cx.update(|window, cx| {
                message_editor.update(cx, |message_editor, cx| {
                    message_editor.clear(window, cx);
                });
            });

            Ok(Some((contents, tracked_buffers)))
        });

        self.send_content(contents_task, window, cx);
    }

    pub fn send_content(
        &mut self,
        contents_task: Task<anyhow::Result<Option<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let session_id = self.thread.read(cx).session_id().clone();
        let agent_telemetry_id = self.thread.read(cx).connection().telemetry_id();
        let thread = self.thread.downgrade();

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

            let generation = this.update(cx, |this, cx| {
                let generation = this.start_turn(cx);
                this.in_flight_prompt = Some(contents.clone());
                generation
            })?;

            this.update_in(cx, |this, _window, cx| {
                this.set_editor_is_expanded(false, cx);
            })?;
            let _ = this.update(cx, |this, cx| this.scroll_to_bottom(cx));

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
                let _ = this.update(cx, |this, _| this.in_flight_prompt.take());
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
                    this.handle_any_thread_error(err, cx);
                })
                .ok();
            } else {
                this.update(cx, |this, cx| {
                    let should_be_following = this
                        .workspace
                        .update(cx, |workspace, _| {
                            workspace.is_being_followed(CollaboratorId::Agent)
                        })
                        .unwrap_or_default();
                    this.should_be_following = should_be_following;
                })
                .ok();
            }
        })
        .detach();
    }

    pub fn interrupt_and_send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let thread = &self.thread;

        if self.is_loading_contents {
            return;
        }

        let message_editor = self.message_editor.clone();
        if thread.read(cx).status() == ThreadStatus::Idle {
            self.send_impl(message_editor, window, cx);
            return;
        }

        self.stop_current_and_send_new_message(message_editor, window, cx);
    }

    fn stop_current_and_send_new_message(
        &mut self,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread = self.thread.clone();
        self.skip_queue_processing_count = 0;
        self.user_interrupted_generation = true;

        let cancelled = thread.update(cx, |thread, cx| thread.cancel(cx));

        cx.spawn_in(window, async move |this, cx| {
            cancelled.await;

            this.update_in(cx, |this, window, cx| {
                this.send_impl(message_editor, window, cx);
            })
            .ok();
        })
        .detach();
    }

    pub(crate) fn handle_any_thread_error(&mut self, error: anyhow::Error, cx: &mut Context<Self>) {
        let error = ThreadError::from_err(error, &self.agent_name);
        self.handle_thread_error(error, cx);
    }

    pub(crate) fn handle_thread_error(&mut self, error: ThreadError, cx: &mut Context<Self>) {
        self.emit_thread_error_telemetry(&error, cx);
        self.thread_error = Some(error);
        cx.notify();
    }

    fn emit_thread_error_telemetry(&self, error: &ThreadError, cx: &mut Context<Self>) {
        let (error_kind, acp_error_code, message): (&str, Option<SharedString>, SharedString) =
            match error {
                ThreadError::PaymentRequired => (
                    "payment_required",
                    None,
                    "You reached your free usage limit. Upgrade to Zed Pro for more prompts."
                        .into(),
                ),
                ThreadError::Refusal => {
                    let model_or_agent_name = self.current_model_name(cx);
                    let message = format!(
                        "{} refused to respond to this prompt. This can happen when a model believes the prompt violates its content policy or safety guidelines, so rephrasing it can sometimes address the issue.",
                        model_or_agent_name
                    );
                    ("refusal", None, message.into())
                }
                ThreadError::AuthenticationRequired(message) => {
                    ("authentication_required", None, message.clone())
                }
                ThreadError::Other {
                    acp_error_code,
                    message,
                } => ("other", acp_error_code.clone(), message.clone()),
            };

        let agent_telemetry_id = self.thread.read(cx).connection().telemetry_id();
        let session_id = self.thread.read(cx).session_id().clone();

        telemetry::event!(
            "Agent Panel Error Shown",
            agent = agent_telemetry_id,
            session_id = session_id,
            kind = error_kind,
            acp_error_code = acp_error_code,
            message = message,
        );
    }

    // generation

    pub fn cancel_generation(&mut self, cx: &mut Context<Self>) {
        self.thread_retry_status.take();
        self.thread_error.take();
        self.user_interrupted_generation = true;
        self._cancel_task = Some(self.thread.update(cx, |thread, cx| thread.cancel(cx)));
    }

    pub fn retry_generation(&mut self, cx: &mut Context<Self>) {
        self.thread_error.take();

        let thread = &self.thread;
        if !thread.read(cx).can_retry(cx) {
            return;
        }

        let task = thread.update(cx, |thread, cx| thread.retry(cx));
        cx.spawn(async move |this, cx| {
            let result = task.await;

            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    this.handle_any_thread_error(err, cx);
                }
            })
        })
        .detach();
    }

    pub fn regenerate(
        &mut self,
        entry_ix: usize,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_loading_contents {
            return;
        }
        let thread = self.thread.clone();

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
            this.update_in(cx, |thread, window, cx| {
                thread.send_impl(message_editor, window, cx);
                thread.focus_handle(cx).focus(window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    // message queueing

    fn queue_message(
        &mut self,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_idle = self.thread.read(cx).status() == acp_thread::ThreadStatus::Idle;

        if is_idle {
            self.send_impl(message_editor.clone(), window, cx);
            return;
        }

        let full_mention_content = self.as_native_thread(cx).is_some_and(|thread| {
            let thread = thread.read(cx);
            AgentSettings::get_global(cx)
                .profiles
                .get(thread.profile())
                .is_some_and(|profile| profile.tools.is_empty())
        });

        let contents = message_editor.update(cx, |message_editor, cx| {
            message_editor.contents(full_mention_content, cx)
        });

        cx.spawn_in(window, async move |this, cx| {
            let (content, tracked_buffers) = contents.await?;

            if content.is_empty() {
                return Ok::<(), anyhow::Error>(());
            }

            this.update_in(cx, |this, window, cx| {
                this.add_to_queue(content, tracked_buffers, cx);
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

    pub fn add_to_queue(
        &mut self,
        content: Vec<acp::ContentBlock>,
        tracked_buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        self.local_queued_messages.push(QueuedMessage {
            content,
            tracked_buffers,
        });
        self.sync_queue_flag_to_native_thread(cx);
    }

    pub fn remove_from_queue(
        &mut self,
        index: usize,
        cx: &mut Context<Self>,
    ) -> Option<QueuedMessage> {
        if index < self.local_queued_messages.len() {
            let removed = self.local_queued_messages.remove(index);
            self.sync_queue_flag_to_native_thread(cx);
            Some(removed)
        } else {
            None
        }
    }

    pub fn sync_queue_flag_to_native_thread(&self, cx: &mut Context<Self>) {
        if let Some(native_thread) = self.as_native_thread(cx) {
            let has_queued = self.has_queued_messages();
            native_thread.update(cx, |thread, _| {
                thread.set_has_queued_message(has_queued);
            });
        }
    }

    pub fn send_queued_message_at_index(
        &mut self,
        index: usize,
        is_send_now: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(queued) = self.remove_from_queue(index, cx) else {
            return;
        };
        let content = queued.content;
        let tracked_buffers = queued.tracked_buffers;

        // Only increment skip count for "Send Now" operations (out-of-order sends)
        // Normal auto-processing from the Stopped handler doesn't need to skip.
        // We only skip the Stopped event from the cancelled generation, NOT the
        // Stopped event from the newly sent message (which should trigger queue processing).
        if is_send_now {
            let is_generating =
                self.thread.read(cx).status() == acp_thread::ThreadStatus::Generating;
            self.skip_queue_processing_count += if is_generating { 1 } else { 0 };
        }

        let cancelled = self.thread.update(cx, |thread, cx| thread.cancel(cx));

        let workspace = self.workspace.clone();

        let should_be_following = self.should_be_following;
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

    // editor methods

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

    pub fn set_editor_is_expanded(&mut self, is_expanded: bool, cx: &mut Context<Self>) {
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
        let thread = &self.thread;

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

    pub fn cancel_editing(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.editing_message.take()
            && let Some(editor) = &self
                .entry_view_state
                .read(cx)
                .entry(index)
                .and_then(|e| e.message_editor())
                .cloned()
        {
            editor.update(cx, |editor, cx| {
                if let Some(user_message) = self
                    .thread
                    .read(cx)
                    .entries()
                    .get(index)
                    .and_then(|e| e.user_message())
                {
                    editor.set_message(user_message.chunks.clone(), window, cx);
                }
            })
        };
        cx.notify();
    }

    // tool permissions

    pub fn authorize_tool_call(
        &mut self,
        tool_call_id: acp::ToolCallId,
        option_id: acp::PermissionOptionId,
        option_kind: acp::PermissionOptionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread = &self.thread;
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

    pub fn allow_always(&mut self, _: &AllowAlways, window: &mut Window, cx: &mut Context<Self>) {
        self.authorize_pending_tool_call(acp::PermissionOptionKind::AllowAlways, window, cx);
    }

    pub fn allow_once(&mut self, _: &AllowOnce, window: &mut Window, cx: &mut Context<Self>) {
        self.authorize_pending_with_granularity(true, window, cx);
    }

    pub fn reject_once(&mut self, _: &RejectOnce, window: &mut Window, cx: &mut Context<Self>) {
        self.authorize_pending_with_granularity(false, window, cx);
    }

    pub fn authorize_pending_tool_call(
        &mut self,
        kind: acp::PermissionOptionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let thread = self.thread.read(cx);
        let tool_call = thread.first_tool_awaiting_confirmation()?;
        let ToolCallStatus::WaitingForConfirmation { options, .. } = &tool_call.status else {
            return None;
        };
        let option = options.first_option_of_kind(kind)?;

        self.authorize_tool_call(
            tool_call.id.clone(),
            option.option_id.clone(),
            option.kind,
            window,
            cx,
        );

        Some(())
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

    pub fn handle_select_permission_granularity(
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

    fn authorize_pending_with_granularity(
        &mut self,
        is_allow: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let thread = self.thread.read(cx);
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
        let selected_index = self
            .selected_permission_granularity
            .get(&tool_call_id)
            .copied()
            .unwrap_or_else(|| choices.len().saturating_sub(1));

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

    // edits

    pub fn keep_all(&mut self, _: &KeepAll, _window: &mut Window, cx: &mut Context<Self>) {
        let thread = &self.thread;
        let telemetry = ActionLogTelemetry::from(thread.read(cx));
        let action_log = thread.read(cx).action_log().clone();
        action_log.update(cx, |action_log, cx| {
            action_log.keep_all_edits(Some(telemetry), cx)
        });
    }

    pub fn reject_all(&mut self, _: &RejectAll, _window: &mut Window, cx: &mut Context<Self>) {
        let thread = &self.thread;
        let telemetry = ActionLogTelemetry::from(thread.read(cx));
        let action_log = thread.read(cx).action_log().clone();
        action_log
            .update(cx, |action_log, cx| {
                action_log.reject_all_edits(Some(telemetry), cx)
            })
            .detach();
    }

    pub fn open_edited_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread = &self.thread;

        let Some(diff) =
            AgentDiffPane::deploy(thread.clone(), self.workspace.clone(), window, cx).log_err()
        else {
            return;
        };

        diff.update(cx, |diff, cx| {
            diff.move_to_path(PathKey::for_buffer(buffer, cx), window, cx)
        })
    }

    // thread stuff

    fn share_thread(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let Some((thread, project)) = self.as_native_thread(cx).zip(self.project.upgrade()) else {
            return;
        };

        let client = project.read(cx).client();
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

    pub fn sync_thread(
        &mut self,
        project: Entity<Project>,
        server_view: Entity<AcpServerView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_imported_thread(cx) {
            return;
        }

        let Some(session_list) = self
            .as_native_connection(cx)
            .and_then(|connection| connection.session_list(cx))
            .and_then(|list| list.downcast::<NativeAgentSessionList>())
        else {
            return;
        };
        let thread_store = session_list.thread_store().clone();

        let client = project.read(cx).client();
        let session_id = self.thread.read(cx).session_id().clone();
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
                server_view.update(cx, |server_view, cx| server_view.reset(window, cx));
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

    pub fn restore_checkpoint(&mut self, message_id: &UserMessageId, cx: &mut Context<Self>) {
        self.thread
            .update(cx, |thread, cx| {
                thread.restore_checkpoint(message_id.clone(), cx)
            })
            .detach_and_log_err(cx);
    }

    pub fn clear_thread_error(&mut self, cx: &mut Context<Self>) {
        self.thread_error = None;
        self.thread_error_markdown = None;
        self.token_limit_callout_dismissed = true;
        cx.notify();
    }

    fn is_following(&self, cx: &App) -> bool {
        match self.thread.read(cx).status() {
            ThreadStatus::Generating => self
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
        if self.thread.read(cx).status() == ThreadStatus::Generating {
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

    // other

    pub fn render_thread_retry_status_callout(&self) -> Option<Callout> {
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
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .title(state.last_error.clone())
                .description(retry_message),
        )
    }

    pub fn handle_open_rules(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    fn activity_bar_bg(&self, cx: &Context<Self>) -> Hsla {
        let editor_bg_color = cx.theme().colors().editor_background;
        let active_color = cx.theme().colors().element_selected;
        editor_bg_color.blend(active_color.opacity(0.3))
    }

    pub fn render_activity_bar(
        &self,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let thread = self.thread.read(cx);
        let action_log = thread.action_log();
        let telemetry = ActionLogTelemetry::from(thread);
        let changed_buffers = action_log.read(cx).changed_buffers(cx);
        let plan = thread.plan();
        let queue_is_empty = !self.has_queued_messages();

        if changed_buffers.is_empty() && plan.is_empty() && queue_is_empty {
            return None;
        }

        // Temporarily always enable ACP edit controls. This is temporary, to lessen the
        // impact of a nasty bug that causes them to sometimes be disabled when they shouldn't
        // be, which blocks you from being able to accept or reject edits. This switches the
        // bug to be that sometimes it's enabled when it shouldn't be, which at least doesn't
        // block you from using the panel.
        let pending_edits = false;

        let plan_expanded = self.plan_expanded;
        let edits_expanded = self.edits_expanded;
        let queue_expanded = self.queue_expanded;

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
                    .when(plan_expanded, |parent| {
                        parent.child(self.render_plan_entries(plan, window, cx))
                    })
            })
            .when(!plan.is_empty() && !changed_buffers.is_empty(), |this| {
                this.child(Divider::horizontal().color(DividerColor::Border))
            })
            .when(!changed_buffers.is_empty(), |this| {
                this.child(self.render_edits_summary(
                    &changed_buffers,
                    edits_expanded,
                    pending_edits,
                    cx,
                ))
                .when(edits_expanded, |parent| {
                    parent.child(self.render_edited_files(
                        action_log,
                        telemetry.clone(),
                        &changed_buffers,
                        pending_edits,
                        cx,
                    ))
                })
            })
            .when(!queue_is_empty, |this| {
                this.when(!plan.is_empty() || !changed_buffers.is_empty(), |this| {
                    this.child(Divider::horizontal().color(DividerColor::Border))
                })
                .child(self.render_message_queue_summary(window, cx))
                .when(queue_expanded, |parent| {
                    parent.child(self.render_message_queue_entries(window, cx))
                })
            })
            .into_any()
            .into()
    }

    fn render_edited_files(
        &self,
        action_log: &Entity<ActionLog>,
        telemetry: ActionLogTelemetry,
        changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>,
        pending_edits: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let editor_bg_color = cx.theme().colors().editor_background;

        // Sort edited files alphabetically for consistency with Git diff view
        let mut sorted_buffers: Vec<_> = changed_buffers.iter().collect();
        sorted_buffers.sort_by(|(buffer_a, _), (buffer_b, _)| {
            let path_a = buffer_a.read(cx).file().map(|f| f.path().clone());
            let path_b = buffer_b.read(cx).file().map(|f| f.path().clone());
            path_a.cmp(&path_b)
        });

        v_flex()
            .id("edited_files_list")
            .max_h_40()
            .overflow_y_scroll()
            .children(
                sorted_buffers
                    .into_iter()
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
                                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                                    .tooltip({
                                        move |_, cx| {
                                            Tooltip::with_meta(
                                                "Go to File",
                                                None,
                                                full_path.clone(),
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click({
                                        let buffer = buffer.clone();
                                        cx.listener(move |this, _, window, cx| {
                                            this.open_edited_buffer(&buffer, window, cx);
                                        })
                                    }),
                            )
                            .child(buttons);

                        Some(element)
                    }),
            )
            .into_any_element()
    }

    fn render_edited_files_buttons(
        &self,
        index: usize,
        buffer: &Entity<Buffer>,
        action_log: &Entity<ActionLog>,
        telemetry: &ActionLogTelemetry,
        pending_edits: bool,
        editor_bg_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        h_flex()
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
            }))
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
                                    Anchor::min_max_range_for_buffer(buffer.read(cx).remote_id()),
                                    Some(telemetry.clone()),
                                    cx,
                                );
                            })
                        }
                    }),
            )
    }

    fn render_message_queue_summary(
        &self,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let queue_count = self.local_queued_messages.len();
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
                        this.clear_queue(cx);
                        this.can_fast_track_queue = false;
                        cx.notify();
                    })),
            )
            .into_any_element()
    }

    fn clear_queue(&mut self, cx: &mut Context<Self>) {
        self.local_queued_messages.clear();
        self.sync_queue_flag_to_native_thread(cx);
    }

    fn render_plan_summary(
        &self,
        plan: &Plan,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let plan_expanded = self.plan_expanded;
        let stats = plan.stats();

        let title = if let Some(entry) = stats.in_progress_entry
            && !plan_expanded
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
            .when(plan_expanded, |this| {
                this.border_b_1().border_color(cx.theme().colors().border)
            })
            .child(Disclosure::new("plan_disclosure", plan_expanded))
            .child(title)
            .on_click(cx.listener(|this, _, _, cx| {
                this.plan_expanded = !this.plan_expanded;
                cx.notify();
            }))
            .into_any_element()
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
            .child(
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
                                KeyBinding::for_action_in(&RejectAll, &focus_handle.clone(), cx)
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
    }

    pub(crate) fn render_subagent_titlebar(&mut self, cx: &mut Context<Self>) -> Option<Div> {
        let Some(parent_session_id) = self.parent_id.clone() else {
            return None;
        };

        let title = self.thread.read(cx).title();
        let server_view = self.server_view.clone();

        Some(
            h_flex()
                .h(Tab::container_height(cx))
                .pl_2()
                .pr_1p5()
                .w_full()
                .justify_between()
                .border_b_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().editor_background.opacity(0.2))
                .child(Label::new(title).color(Color::Muted))
                .child(
                    IconButton::new("minimize_subagent", IconName::Minimize)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Minimize Subagent"))
                        .on_click(move |_, window, cx| {
                            let _ = server_view.update(cx, |server_view, cx| {
                                server_view.navigate_to_session(
                                    parent_session_id.clone(),
                                    window,
                                    cx,
                                );
                            });
                        }),
                ),
        )
    }

    pub(crate) fn render_message_editor(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if self.is_subagent() {
            return div().into_any_element();
        }

        let focus_handle = self.message_editor.focus_handle(cx);
        let editor_bg_color = cx.theme().colors().editor_background;
        let editor_expanded = self.editor_expanded;
        let (expand_icon, expand_tooltip) = if editor_expanded {
            (IconName::Minimize, "Minimize Message Editor")
        } else {
            (IconName::Maximize, "Expand Message Editor")
        };

        v_flex()
            .on_action(cx.listener(Self::expand_message_editor))
            .p_2()
            .gap_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(editor_bg_color)
            .when(editor_expanded, |this| {
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
                            .child(self.render_follow_toggle(cx))
                            .children(self.render_thinking_control(cx)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .children(self.render_token_usage(cx))
                            .children(self.profile_selector.clone())
                            .map(|this| {
                                // Either config_options_view OR (mode_selector + model_selector)
                                match self.config_options_view.clone() {
                                    Some(config_view) => this.child(config_view),
                                    None => this
                                        .children(self.mode_selector.clone())
                                        .children(self.model_selector.clone()),
                                }
                            })
                            .child(self.render_send_button(cx)),
                    ),
            )
            .into_any()
    }

    fn render_message_queue_entries(
        &self,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let message_editor = self.message_editor.read(cx);
        let focus_handle = message_editor.focus_handle(cx);

        let queued_message_editors = &self.queued_message_editors;
        let queue_len = queued_message_editors.len();
        let can_fast_track = self.can_fast_track_queue && queue_len > 0;

        v_flex()
            .id("message_queue_list")
            .max_h_40()
            .overflow_y_scroll()
            .children(
                queued_message_editors
                    .iter()
                    .enumerate()
                    .map(|(index, editor)| {
                        let is_next = index == 0;
                        let (icon_color, tooltip_text) = if is_next {
                            (Color::Accent, "Next in Queue")
                        } else {
                            (Color::Muted, "In Queue")
                        };

                        let editor_focused = editor.focus_handle(cx).is_focused(_window);
                        let keybinding_size = rems_from_px(12.);

                        h_flex()
                            .group("queue_entry")
                            .w_full()
                            .p_1p5()
                            .gap_1()
                            .bg(cx.theme().colors().editor_background)
                            .when(index < queue_len - 1, |this| {
                                this.border_b_1()
                                    .border_color(cx.theme().colors().border_variant)
                            })
                            .child(
                                div()
                                    .id("next_in_queue")
                                    .child(
                                        Icon::new(IconName::Circle)
                                            .size(IconSize::Small)
                                            .color(icon_color),
                                    )
                                    .tooltip(Tooltip::text(tooltip_text)),
                            )
                            .child(editor.clone())
                            .child(if editor_focused {
                                h_flex()
                                    .gap_1()
                                    .min_w_40()
                                    .child(
                                        IconButton::new(("cancel_edit", index), IconName::Close)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Error)
                                            .tooltip({
                                                let focus_handle = editor.focus_handle(cx);
                                                move |_window, cx| {
                                                    Tooltip::for_action_in(
                                                        "Cancel Edit",
                                                        &editor::actions::Cancel,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                }
                                            })
                                            .on_click({
                                                let main_editor = self.message_editor.clone();
                                                cx.listener(move |_, _, window, cx| {
                                                    window.focus(&main_editor.focus_handle(cx), cx);
                                                })
                                            }),
                                    )
                                    .child(
                                        IconButton::new(("save_edit", index), IconName::Check)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Success)
                                            .tooltip({
                                                let focus_handle = editor.focus_handle(cx);
                                                move |_window, cx| {
                                                    Tooltip::for_action_in(
                                                        "Save Edit",
                                                        &Chat,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                }
                                            })
                                            .on_click({
                                                let main_editor = self.message_editor.clone();
                                                cx.listener(move |_, _, window, cx| {
                                                    window.focus(&main_editor.focus_handle(cx), cx);
                                                })
                                            }),
                                    )
                                    .child(
                                        Button::new(("send_now_focused", index), "Send Now")
                                            .label_size(LabelSize::Small)
                                            .style(ButtonStyle::Outlined)
                                            .key_binding(
                                                KeyBinding::for_action_in(
                                                    &SendImmediately,
                                                    &editor.focus_handle(cx),
                                                    cx,
                                                )
                                                .map(|kb| kb.size(keybinding_size)),
                                            )
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.send_queued_message_at_index(
                                                    index, true, window, cx,
                                                );
                                            })),
                                    )
                            } else {
                                h_flex()
                                    .gap_1()
                                    .when(!is_next, |this| this.visible_on_hover("queue_entry"))
                                    .child(
                                        IconButton::new(("edit", index), IconName::Pencil)
                                            .icon_size(IconSize::Small)
                                            .tooltip({
                                                let focus_handle = focus_handle.clone();
                                                move |_window, cx| {
                                                    if is_next {
                                                        Tooltip::for_action_in(
                                                            "Edit",
                                                            &EditFirstQueuedMessage,
                                                            &focus_handle,
                                                            cx,
                                                        )
                                                    } else {
                                                        Tooltip::simple("Edit", cx)
                                                    }
                                                }
                                            })
                                            .on_click({
                                                let editor = editor.clone();
                                                cx.listener(move |_, _, window, cx| {
                                                    window.focus(&editor.focus_handle(cx), cx);
                                                })
                                            }),
                                    )
                                    .child(
                                        IconButton::new(("delete", index), IconName::Trash)
                                            .icon_size(IconSize::Small)
                                            .tooltip({
                                                let focus_handle = focus_handle.clone();
                                                move |_window, cx| {
                                                    if is_next {
                                                        Tooltip::for_action_in(
                                                            "Remove Message from Queue",
                                                            &RemoveFirstQueuedMessage,
                                                            &focus_handle,
                                                            cx,
                                                        )
                                                    } else {
                                                        Tooltip::simple(
                                                            "Remove Message from Queue",
                                                            cx,
                                                        )
                                                    }
                                                }
                                            })
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.remove_from_queue(index, cx);
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Button::new(("send_now", index), "Send Now")
                                            .label_size(LabelSize::Small)
                                            .when(is_next && message_editor.is_empty(cx), |this| {
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
                                                    .map(|kb| kb.size(keybinding_size)),
                                                )
                                            })
                                            .when(is_next && !message_editor.is_empty(cx), |this| {
                                                this.style(ButtonStyle::Outlined)
                                            })
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.send_queued_message_at_index(
                                                    index, true, window, cx,
                                                );
                                            })),
                                    )
                            })
                    }),
            )
            .into_any_element()
    }

    fn supports_split_token_display(&self, cx: &App) -> bool {
        self.as_native_thread(cx)
            .and_then(|thread| thread.read(cx).model())
            .is_some_and(|model| model.supports_split_token_display())
    }

    fn render_token_usage(&self, cx: &mut Context<Self>) -> Option<Div> {
        let thread = self.thread.read(cx);
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

    fn render_thinking_control(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !cx.has_flag::<CloudThinkingEffortFeatureFlag>() {
            return None;
        }

        let thread = self.as_native_thread(cx)?.read(cx);
        let model = thread.model()?;

        let supports_thinking = model.supports_thinking();
        if !supports_thinking {
            return None;
        }

        let thinking = thread.thinking_enabled();

        let (tooltip_label, icon) = if thinking {
            ("Disable Thinking Mode", IconName::ThinkingMode)
        } else {
            ("Enable Thinking Mode", IconName::ToolThink)
        };

        let focus_handle = self.message_editor.focus_handle(cx);

        let thinking_toggle = IconButton::new("thinking-mode", icon)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .toggle_state(thinking)
            .tooltip(move |_, cx| {
                Tooltip::for_action_in(tooltip_label, &ToggleThinkingMode, &focus_handle, cx)
            })
            .on_click(cx.listener(move |this, _, _window, cx| {
                if let Some(thread) = this.as_native_thread(cx) {
                    thread.update(cx, |thread, cx| {
                        let enable_thinking = !thread.thinking_enabled();
                        thread.set_thinking_enabled(enable_thinking, cx);

                        let fs = thread.project().read(cx).fs().clone();
                        update_settings_file(fs, cx, move |settings, _| {
                            if let Some(agent) = settings.agent.as_mut()
                                && let Some(default_model) = agent.default_model.as_mut()
                            {
                                default_model.enable_thinking = enable_thinking;
                            }
                        });
                    });
                }
            }));

        if model.supported_effort_levels().is_empty() {
            return Some(thinking_toggle.into_any_element());
        }

        if !model.supported_effort_levels().is_empty() && !thinking {
            return Some(thinking_toggle.into_any_element());
        }

        let left_btn = thinking_toggle;
        let right_btn = self.render_effort_selector(
            model.supported_effort_levels(),
            thread.thinking_effort().cloned(),
            cx,
        );

        Some(
            SplitButton::new(left_btn, right_btn.into_any_element())
                .style(SplitButtonStyle::Transparent)
                .into_any_element(),
        )
    }

    fn render_effort_selector(
        &self,
        supported_effort_levels: Vec<LanguageModelEffortLevel>,
        selected_effort: Option<String>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let weak_self = cx.weak_entity();

        let default_effort_level = supported_effort_levels
            .iter()
            .find(|effort_level| effort_level.is_default)
            .cloned();

        let selected = selected_effort.and_then(|effort| {
            supported_effort_levels
                .iter()
                .find(|level| level.value == effort)
                .cloned()
        });

        let label = selected
            .clone()
            .or(default_effort_level)
            .map_or("Select Effort".into(), |effort| effort.name);

        let (label_color, icon) = if self.thinking_effort_menu_handle.is_deployed() {
            (Color::Accent, IconName::ChevronUp)
        } else {
            (Color::Muted, IconName::ChevronDown)
        };

        let focus_handle = self.message_editor.focus_handle(cx);
        let show_cycle_row = supported_effort_levels.len() > 1;

        let tooltip = Tooltip::element({
            move |_, cx| {
                let mut content = v_flex().gap_1().child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(Label::new("Change Thinking Effort"))
                        .child(KeyBinding::for_action_in(
                            &ToggleThinkingEffortMenu,
                            &focus_handle,
                            cx,
                        )),
                );

                if show_cycle_row {
                    content = content.child(
                        h_flex()
                            .pt_1()
                            .gap_2()
                            .justify_between()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(Label::new("Cycle Thinking Effort"))
                            .child(KeyBinding::for_action_in(
                                &CycleThinkingEffort,
                                &focus_handle,
                                cx,
                            )),
                    );
                }

                content.into_any_element()
            }
        });

        PopoverMenu::new("effort-selector")
            .trigger_with_tooltip(
                ButtonLike::new_rounded_right("effort-selector-trigger")
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .child(Label::new(label).size(LabelSize::Small).color(label_color))
                    .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted)),
                tooltip,
            )
            .menu(move |window, cx| {
                Some(ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                    menu = menu.header("Change Thinking Effort");

                    for effort_level in supported_effort_levels.clone() {
                        let is_selected = selected
                            .as_ref()
                            .is_some_and(|selected| selected.value == effort_level.value);
                        let entry = ContextMenuEntry::new(effort_level.name)
                            .toggleable(IconPosition::End, is_selected);

                        menu.push_item(entry.handler({
                            let effort = effort_level.value.clone();
                            let weak_self = weak_self.clone();
                            move |_window, cx| {
                                let effort = effort.clone();
                                weak_self
                                    .update(cx, |this, cx| {
                                        if let Some(thread) = this.as_native_thread(cx) {
                                            thread.update(cx, |thread, cx| {
                                                thread.set_thinking_effort(
                                                    Some(effort.to_string()),
                                                    cx,
                                                );

                                                let fs = thread.project().read(cx).fs().clone();
                                                update_settings_file(fs, cx, move |settings, _| {
                                                    if let Some(agent) = settings.agent.as_mut()
                                                        && let Some(default_model) =
                                                            agent.default_model.as_mut()
                                                    {
                                                        default_model.effort =
                                                            Some(effort.to_string());
                                                    }
                                                });
                                            });
                                        }
                                    })
                                    .ok();
                            }
                        }));
                    }

                    menu
                }))
            })
            .with_handle(self.thinking_effort_menu_handle.clone())
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .anchor(Corner::BottomLeft)
    }

    fn render_send_button(&self, cx: &mut Context<Self>) -> AnyElement {
        let message_editor = self.message_editor.read(cx);
        let is_editor_empty = message_editor.is_empty(cx);
        let focus_handle = message_editor.focus_handle(cx);

        let is_generating = self.thread.read(cx).status() != ThreadStatus::Idle;

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

    fn render_add_context_button(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.message_editor.focus_handle(cx);
        let weak_self = cx.weak_entity();

        PopoverMenu::new("add-context-menu")
            .trigger_with_tooltip(
                IconButton::new("add-context", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted),
                {
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "Add Context",
                            &OpenAddContextMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                },
            )
            .anchor(Corner::BottomLeft)
            .with_handle(self.add_context_menu_handle.clone())
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .menu(move |window, cx| {
                weak_self
                    .update(cx, |this, cx| this.build_add_context_menu(window, cx))
                    .ok()
            })
    }

    fn build_add_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let message_editor = self.message_editor.clone();
        let workspace = self.workspace.clone();
        let supports_images = self.prompt_capabilities.borrow().image;

        let has_editor_selection = workspace
            .upgrade()
            .and_then(|ws| {
                ws.read(cx)
                    .active_item(cx)
                    .and_then(|item| item.downcast::<Editor>())
            })
            .is_some_and(|editor| {
                editor.update(cx, |editor, cx| {
                    editor.has_non_empty_selection(&editor.display_snapshot(cx))
                })
            });

        let has_terminal_selection = workspace
            .upgrade()
            .and_then(|ws| ws.read(cx).panel::<TerminalPanel>(cx))
            .is_some_and(|panel| !panel.read(cx).terminal_selections(cx).is_empty());

        let has_selection = has_editor_selection || has_terminal_selection;

        ContextMenu::build(window, cx, move |menu, _window, _cx| {
            menu.key_context("AddContextMenu")
                .header("Context")
                .item(
                    ContextMenuEntry::new("Files & Directories")
                        .icon(IconName::File)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::XSmall)
                        .handler({
                            let message_editor = message_editor.clone();
                            move |window, cx| {
                                message_editor.focus_handle(cx).focus(window, cx);
                                message_editor.update(cx, |editor, cx| {
                                    editor.insert_context_type("file", window, cx);
                                });
                            }
                        }),
                )
                .item(
                    ContextMenuEntry::new("Symbols")
                        .icon(IconName::Code)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::XSmall)
                        .handler({
                            let message_editor = message_editor.clone();
                            move |window, cx| {
                                message_editor.focus_handle(cx).focus(window, cx);
                                message_editor.update(cx, |editor, cx| {
                                    editor.insert_context_type("symbol", window, cx);
                                });
                            }
                        }),
                )
                .item(
                    ContextMenuEntry::new("Threads")
                        .icon(IconName::Thread)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::XSmall)
                        .handler({
                            let message_editor = message_editor.clone();
                            move |window, cx| {
                                message_editor.focus_handle(cx).focus(window, cx);
                                message_editor.update(cx, |editor, cx| {
                                    editor.insert_context_type("thread", window, cx);
                                });
                            }
                        }),
                )
                .item(
                    ContextMenuEntry::new("Rules")
                        .icon(IconName::Reader)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::XSmall)
                        .handler({
                            let message_editor = message_editor.clone();
                            move |window, cx| {
                                message_editor.focus_handle(cx).focus(window, cx);
                                message_editor.update(cx, |editor, cx| {
                                    editor.insert_context_type("rule", window, cx);
                                });
                            }
                        }),
                )
                .item(
                    ContextMenuEntry::new("Image")
                        .icon(IconName::Image)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::XSmall)
                        .disabled(!supports_images)
                        .handler({
                            let message_editor = message_editor.clone();
                            move |window, cx| {
                                message_editor.focus_handle(cx).focus(window, cx);
                                message_editor.update(cx, |editor, cx| {
                                    editor.add_images_from_picker(window, cx);
                                });
                            }
                        }),
                )
                .item(
                    ContextMenuEntry::new("Selection")
                        .icon(IconName::CursorIBeam)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::XSmall)
                        .disabled(!has_selection)
                        .handler({
                            move |window, cx| {
                                window.dispatch_action(
                                    zed_actions::agent::AddSelectionToThread.boxed_clone(),
                                    cx,
                                );
                            }
                        }),
                )
        })
    }

    fn render_follow_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let following = self.is_following(cx);

        let tooltip_label = if following {
            if self.agent_name == "Zed Agent" {
                format!("Stop Following the {}", self.agent_name)
            } else {
                format!("Stop Following {}", self.agent_name)
            }
        } else {
            if self.agent_name == "Zed Agent" {
                format!("Follow the {}", self.agent_name)
            } else {
                format!("Follow {}", self.agent_name)
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
}

impl AcpThreadView {
    pub(crate) fn render_entries(&mut self, cx: &mut Context<Self>) -> List {
        list(
            self.list_state.clone(),
            cx.processor(|this, index: usize, window, cx| {
                let entries = this.thread.read(cx).entries();
                let Some(entry) = entries.get(index) else {
                    return Empty.into_any();
                };
                this.render_entry(index, entries.len(), entry, window, cx)
            }),
        )
        .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
        .flex_grow()
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
            && self
                .thread
                .read(cx)
                .entries()
                .get(entry_ix.saturating_sub(1))
                .is_none_or(|entry| !entry.is_indented());

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

                let agent_name = self.agent_name.clone();
                let is_subagent = self.is_subagent();

                let non_editable_icon = || {
                    IconButton::new("non_editable", IconName::PencilUnavailable)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .style(ButtonStyle::Transparent)
                };

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
                                    .bg(cx.theme().colors().editor_background)
                                    .border_1()
                                    .when(is_indented, |this| {
                                        this.py_2().px_2().shadow_sm()
                                    })
                                    .border_color(cx.theme().colors().border)
                                    .map(|this| {
                                        if is_subagent {
                                            return this.border_dashed();
                                        }
                                        if editing && editor_focus {
                                            return this.border_color(focus_border);
                                        }
                                        if editing && !editor_focus {
                                            return this.border_dashed()
                                        }
                                        if message.id.is_some() {
                                            return this.shadow_md().hover(|s| {
                                                s.border_color(focus_border.opacity(0.8))
                                            });
                                        }
                                        this
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

                                let is_loading_contents = self.is_loading_contents;
                                if is_subagent {
                                    this.child(
                                        base_container.border_dashed().child(
                                            non_editable_icon().tooltip(move |_, cx| {
                                                Tooltip::with_meta(
                                                    "Unavailable Editing",
                                                    None,
                                                    "Editing subagent messages is currently not supported.",
                                                    cx,
                                                )
                                            }),
                                        ),
                                    )
                                } else if message.id.is_some() {
                                    this.child(
                                        base_container
                                            .child(
                                                IconButton::new("cancel", IconName::Close)
                                                    .disabled(is_loading_contents)
                                                    .icon_color(Color::Error)
                                                    .icon_size(IconSize::XSmall)
                                                    .on_click(cx.listener(Self::cancel_editing))
                                            )
                                            .child(
                                                if is_loading_contents {
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
                                                non_editable_icon()
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

                let style = MarkdownStyle::themed(MarkdownFont::Agent, window, cx);
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

        let thread = self.thread.clone();
        let comments_editor = self.thread_feedback.comments_editor.clone();

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
                .when_some(comments_editor, |this, editor| {
                    this.child(Self::render_feedback_feedback_editor(editor, cx))
                })
                .into_any_element()
        } else {
            primary
        };

        if let Some(editing_index) = self.editing_message
            && editing_index < entry_ix
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
                self.turn_fields
                    .last_turn_duration
                    .filter(|&duration| duration > STOPWATCH_THRESHOLD)
                    .map(|duration| {
                        Label::new(duration_alt_display(duration))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    })
            })
            .flatten();

        let last_turn_tokens_label = last_turn_clock
            .is_some()
            .then(|| {
                self.turn_fields
                    .last_turn_tokens
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
                last_turn_tokens_label.is_some() || last_turn_clock.is_some(),
                |this| {
                    this.child(
                        h_flex()
                            .gap_1()
                            .px_1()
                            .when_some(last_turn_tokens_label, |this, label| this.child(label))
                            .when_some(last_turn_clock, |this, label| this.child(label)),
                    )
                },
            );

        if AgentSettings::get_global(cx).enable_feedback
            && self.thread.read(cx).connection().telemetry().is_some()
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
                                _ => {
                                    Tooltip::with_meta("Helpful Response", None, tooltip_meta(), cx)
                                }
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
                                    Tooltip::with_meta(
                                        "Not Helpful Response",
                                        None,
                                        tooltip_meta(),
                                        cx,
                                    )
                                }
                            })
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.handle_feedback_click(ThreadFeedback::Negative, window, cx);
                            })),
                    );
        }

        if let Some(project) = self.project.upgrade()
            && let Some(server_view) = self.server_view.upgrade()
            && cx.has_flag::<AgentSharingFeatureFlag>()
            && project.read(cx).client().status().borrow().is_connected()
        {
            let button = if self.is_imported_thread(cx) {
                IconButton::new("sync-thread", IconName::ArrowCircle)
                    .shape(ui::IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Ignored)
                    .tooltip(Tooltip::text("Sync with source thread"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.sync_thread(project.clone(), server_view.clone(), window, cx);
                    }))
            } else {
                IconButton::new("share-thread", IconName::ArrowUpRight)
                    .shape(ui::IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Ignored)
                    .tooltip(Tooltip::text("Share Thread"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.share_thread(window, cx);
                    }))
            };

            container = container.child(button);
        }

        container
            .child(open_as_markdown)
            .child(scroll_to_recent_user_prompt)
            .child(scroll_to_top)
            .into_any_element()
    }

    pub(crate) fn scroll_to_most_recent_user_prompt(&mut self, cx: &mut Context<Self>) {
        let entries = self.thread.read(cx).entries();
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
        let entry_count = self.thread.read(cx).entries().len();
        self.list_state.reset(entry_count);
        cx.notify();
    }

    fn handle_feedback_click(
        &mut self,
        feedback: ThreadFeedback,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread_feedback
            .submit(self.thread.clone(), feedback, window, cx);
        cx.notify();
    }

    fn submit_feedback_message(&mut self, cx: &mut Context<Self>) {
        let thread = self.thread.clone();
        self.thread_feedback.submit_comments(thread, cx);
        cx.notify();
    }

    pub(crate) fn scroll_to_top(&mut self, cx: &mut Context<Self>) {
        self.list_state.scroll_to(ListOffset::default());
        cx.notify();
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

        let thread = self.thread.read(cx);
        let thread_title = thread.title().to_string();
        let markdown = thread.to_markdown(cx);

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

    fn render_generating(&self, confirmation: bool, cx: &App) -> impl IntoElement {
        let show_stats = AgentSettings::get_global(cx).show_turn_stats;
        let elapsed_label = show_stats
            .then(|| {
                self.turn_fields.turn_started_at.and_then(|started_at| {
                    let elapsed = started_at.elapsed();
                    (elapsed > STOPWATCH_THRESHOLD).then(|| duration_alt_display(elapsed))
                })
            })
            .flatten();

        let is_waiting = confirmation || self.thread.read(cx).has_in_progress_tool_calls();

        let turn_tokens_label = elapsed_label
            .is_some()
            .then(|| {
                self.turn_fields
                    .turn_tokens
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
                            LoadingLabel::new("Awaiting Confirmation")
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
                                    if is_open {
                                        this.expanded_thinking_blocks.remove(&key);
                                    } else {
                                        this.expanded_thinking_blocks.insert(key);
                                    }
                                    cx.notify();
                                }
                            })),
                    )
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        if is_open {
                            this.expanded_thinking_blocks.remove(&key);
                        } else {
                            this.expanded_thinking_blocks.insert(key);
                        }
                        cx.notify();
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
                    let this = entity.read(cx);
                    let is_at_top = this.list_state.logical_scroll_top().item_ix == 0;

                    let has_selection = this
                        .thread
                        .read(cx)
                        .entries()
                        .get(entry_ix)
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
                                    let entries = this.thread.read(cx).entries();
                                    if let Some(text) =
                                        Self::get_agent_message_content(entries, entry_ix, cx)
                                    {
                                        cx.write_to_clipboard(ClipboardItem::new_string(text));
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

    fn render_collapsible_command(
        &self,
        is_preview: bool,
        command_source: &str,
        tool_call_id: &acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
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
                    .children(command_source.lines().map(|line| {
                        let text: SharedString = if line.is_empty() {
                            " ".into()
                        } else {
                            line.to_string().into()
                        };

                        Label::new(text).buffer_font(cx).size(LabelSize::Small)
                    }))
                    .child(
                        div().absolute().top_1().right_1().child(
                            CopyButton::new("copy-command", command_source.to_string())
                                .tooltip_label("Copy Command")
                                .visible_on_hover(command_group),
                        ),
                    ),
            )
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
                                if AgentSettings::get_global(cx).cancel_generation_on_terminal_stop {
                                    this.cancel_generation(cx);
                                }
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
                    move |this, _event, _window, cx| {
                        if is_expanded {
                            this.expanded_tool_calls.remove(&id);
                        } else {
                            this.expanded_tool_calls.insert(id.clone());
                        }
                        cx.notify();
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

        // For subagent tool calls, render the subagent cards directly without wrapper
        if tool_call.is_subagent() {
            return self.render_subagent_tool_call(
                entry_ix,
                tool_call,
                tool_call.subagent_session_id.clone(),
                window,
                cx,
            );
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
        let mut is_open = self.expanded_tool_calls.contains(&tool_call.id);

        is_open |= needs_confirmation;

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
                    self.render_diff_loading(cx)
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
                                            MarkdownStyle::themed(MarkdownFont::Agent, window, cx),
                                        ),
                                    )
                                }))
                                .child(input_output_header("Output:".into())),
                        )
                    })
                    .children(
                        tool_call
                            .content
                            .iter()
                            .enumerate()
                            .map(|(content_ix, content)| {
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
                            }),
                    )
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
                                                                if is_open {
                                                                    this
                                                                        .expanded_tool_calls.remove(&id);
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

    fn render_permission_buttons(
        &self,
        options: &PermissionOptions,
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        match options {
            PermissionOptions::Flat(options) => {
                self.render_permission_buttons_flat(options, entry_ix, tool_call_id, cx)
            }
            PermissionOptions::Dropdown(options) => {
                self.render_permission_buttons_dropdown(options, entry_ix, tool_call_id, cx)
            }
        }
    }

    fn render_permission_buttons_dropdown(
        &self,
        choices: &[PermissionOptionChoice],
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        let is_first = self
            .thread
            .read(cx)
            .first_tool_awaiting_confirmation()
            .is_some_and(|call| call.id == tool_call_id);

        // Get the selected granularity index, defaulting to the last option ("Only this time")
        let selected_index = self
            .selected_permission_granularity
            .get(&tool_call_id)
            .copied()
            .unwrap_or_else(|| choices.len().saturating_sub(1));

        let selected_choice = choices.get(selected_index).or(choices.last());

        let dropdown_label: SharedString = selected_choice
            .map(|choice| choice.label())
            .unwrap_or_else(|| "Only this time".into());

        let (allow_option_id, allow_option_kind, deny_option_id, deny_option_kind) =
            if let Some(choice) = selected_choice {
                (
                    choice.allow.option_id.clone(),
                    choice.allow.kind,
                    choice.deny.option_id.clone(),
                    choice.deny.kind,
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
                                        &self.focus_handle(cx),
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
                                        &self.focus_handle(cx),
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
                choices,
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
        choices: &[PermissionOptionChoice],
        current_label: SharedString,
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        selected_index: usize,
        is_first: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let menu_options: Vec<(usize, SharedString)> = choices
            .iter()
            .enumerate()
            .map(|(i, choice)| (i, choice.label()))
            .collect();

        let permission_dropdown_handle = self.permission_dropdown_handle.clone();

        PopoverMenu::new(("permission-granularity", entry_ix))
            .with_handle(permission_dropdown_handle)
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
                                &self.focus_handle(cx),
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
            .into_any_element()
    }

    fn render_permission_buttons_flat(
        &self,
        options: &[acp::PermissionOption],
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        cx: &Context<Self>,
    ) -> Div {
        let is_first = self
            .thread
            .read(cx)
            .first_tool_awaiting_confirmation()
            .is_some_and(|call| call.id == tool_call_id);
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
                            KeyBinding::for_action_in(action, &self.focus_handle(cx), cx)
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
        let is_subagent_tool_call = tool_call.is_subagent();

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
        } else if is_subagent_tool_call {
            Icon::new(self.agent_icon)
                .size(IconSize::Small)
                .color(Color::Muted)
                .into_any_element()
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
                acp::ToolKind::SwitchMode => IconName::ArrowRightLeft,
                acp::ToolKind::Other | _ => IconName::ToolHammer,
            })
            .size(IconSize::Small)
            .color(Color::Muted)
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

    fn open_tool_call_location(
        &self,
        entry_ix: usize,
        location_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let (tool_call_location, agent_location) = self
            .thread
            .read(cx)
            .entries()
            .get(entry_ix)?
            .location(location_ix)?;

        let project_path = self
            .project
            .upgrade()?
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
        }
    }

    fn render_resource_link(
        &self,
        resource_link: &acp::ResourceLink,
        cx: &Context<Self>,
    ) -> AnyElement {
        let uri: SharedString = resource_link.uri.clone().into();
        let is_file = resource_link.uri.strip_prefix("file://");

        let Some(project) = self.project.upgrade() else {
            return Empty.into_any_element();
        };

        let label: SharedString = if let Some(abs_path) = is_file {
            if let Some(project_path) = project
                .read(cx)
                .project_path_for_absolute_path(&Path::new(abs_path), cx)
                && let Some(worktree) = project
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
                            open_link(uri.clone(), &workspace, window, cx);
                        }
                    })),
            )
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
            .child(self.render_markdown(
                markdown,
                MarkdownStyle::themed(MarkdownFont::Agent, window, cx),
            ))
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

    fn render_subagent_tool_call(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        subagent_session_id: Option<acp::SessionId>,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let tool_call_status = &tool_call.status;

        let subagent_thread_view = subagent_session_id.and_then(|id| {
            self.server_view
                .upgrade()
                .and_then(|server_view| server_view.read(cx).as_connected())
                .and_then(|connected| connected.threads.get(&id))
        });

        let content = self.render_subagent_card(
            entry_ix,
            0,
            subagent_thread_view,
            tool_call_status,
            window,
            cx,
        );

        v_flex().mx_5().my_1p5().gap_3().child(content)
    }

    fn render_subagent_card(
        &self,
        entry_ix: usize,
        context_ix: usize,
        thread_view: Option<&Entity<AcpThreadView>>,
        tool_call_status: &ToolCallStatus,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let thread = thread_view
            .as_ref()
            .map(|view| view.read(cx).thread.clone());
        let session_id = thread
            .as_ref()
            .map(|thread| thread.read(cx).session_id().clone());
        let action_log = thread.as_ref().map(|thread| thread.read(cx).action_log());
        let changed_buffers = action_log
            .map(|log| log.read(cx).changed_buffers(cx))
            .unwrap_or_default();

        let is_expanded = if let Some(session_id) = &session_id {
            self.expanded_subagents.contains(session_id)
        } else {
            false
        };
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

        let title = thread
            .as_ref()
            .map(|t| t.read(cx).title())
            .unwrap_or_else(|| {
                if is_canceled_or_failed {
                    "Subagent Canceled"
                } else {
                    "Spawning Subagent…"
                }
                .into()
            });

        let card_header_id = format!("subagent-header-{}-{}", entry_ix, context_ix);
        let diff_stat_id = format!("subagent-diff-{}-{}", entry_ix, context_ix);

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

        let has_expandable_content = thread.as_ref().map_or(false, |thread| {
            thread.read(cx).entries().iter().rev().any(|entry| {
                if let AgentThreadEntry::AssistantMessage(msg) = entry {
                    msg.chunks.iter().any(|chunk| match chunk {
                        AssistantMessageChunk::Message { block } => block.markdown().is_some(),
                        AssistantMessageChunk::Thought { block } => block.markdown().is_some(),
                    })
                } else {
                    false
                }
            })
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
                    .p_1()
                    .pl_1p5()
                    .w_full()
                    .gap_1()
                    .justify_between()
                    .bg(self.tool_card_header_bg(cx))
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(icon)
                            .child(Label::new(title.to_string()).size(LabelSize::Small))
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
                    .when_some(session_id, |this, session_id| {
                        this.child(
                            h_flex()
                                .when(has_expandable_content, |this| {
                                    this.child(
                                        IconButton::new(
                                            format!(
                                                "subagent-disclosure-{}-{}",
                                                entry_ix, context_ix
                                            ),
                                            if is_expanded {
                                                IconName::ChevronUp
                                            } else {
                                                IconName::ChevronDown
                                            },
                                        )
                                        .icon_color(Color::Muted)
                                        .icon_size(IconSize::Small)
                                        .disabled(!has_expandable_content)
                                        .visible_on_hover(card_header_id.clone())
                                        .on_click(
                                            cx.listener({
                                                let session_id = session_id.clone();
                                                move |this, _, _, cx| {
                                                    if this.expanded_subagents.contains(&session_id)
                                                    {
                                                        this.expanded_subagents.remove(&session_id);
                                                    } else {
                                                        this.expanded_subagents
                                                            .insert(session_id.clone());
                                                    }
                                                    cx.notify();
                                                }
                                            }),
                                        ),
                                    )
                                })
                                .child(
                                    IconButton::new(
                                        format!("expand-subagent-{}-{}", entry_ix, context_ix),
                                        IconName::Maximize,
                                    )
                                    .icon_color(Color::Muted)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Expand Subagent"))
                                    .visible_on_hover(card_header_id)
                                    .on_click(cx.listener(
                                        move |this, _event, window, cx| {
                                            this.server_view
                                                .update(cx, |this, cx| {
                                                    this.navigate_to_session(
                                                        session_id.clone(),
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        },
                                    )),
                                )
                                .when(is_running, |buttons| {
                                    buttons.child(
                                        IconButton::new(
                                            format!("stop-subagent-{}-{}", entry_ix, context_ix),
                                            IconName::Stop,
                                        )
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Error)
                                        .tooltip(Tooltip::text("Stop Subagent"))
                                        .when_some(
                                            thread_view
                                                .as_ref()
                                                .map(|view| view.read(cx).thread.clone()),
                                            |this, thread| {
                                                this.on_click(cx.listener(
                                                    move |_this, _event, _window, cx| {
                                                        thread.update(cx, |thread, _cx| {
                                                            thread.stop_by_user();
                                                        });
                                                    },
                                                ))
                                            },
                                        ),
                                    )
                                }),
                        )
                    }),
            )
            .when_some(thread_view, |this, thread_view| {
                let thread = &thread_view.read(cx).thread;
                this.when(is_expanded, |this| {
                    this.child(
                        self.render_subagent_expanded_content(
                            entry_ix, context_ix, thread, window, cx,
                        ),
                    )
                })
                .children(
                    thread
                        .read(cx)
                        .first_tool_awaiting_confirmation()
                        .and_then(|tc| {
                            if let ToolCallStatus::WaitingForConfirmation { options, .. } =
                                &tc.status
                            {
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

    fn render_subagent_pending_tool_call(
        &self,
        entry_ix: usize,
        context_ix: usize,
        subagent_thread: Entity<AcpThread>,
        tool_call: &ToolCall,
        options: &PermissionOptions,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let tool_call_id = tool_call.id.clone();
        let is_edit =
            matches!(tool_call.kind, acp::ToolKind::Edit) || tool_call.diffs().next().is_some();
        let has_image_content = tool_call.content.iter().any(|c| c.image().is_some());

        v_flex()
            .w_full()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .child(
                self.render_tool_call_label(
                    entry_ix, tool_call, is_edit, false, // has_failed
                    false, // has_revealed_diff
                    true,  // use_card_layout
                    window, cx,
                )
                .py_1(),
            )
            .children(
                tool_call
                    .content
                    .iter()
                    .enumerate()
                    .map(|(content_ix, content)| {
                        self.render_tool_call_content(
                            entry_ix,
                            content,
                            content_ix,
                            tool_call,
                            true, // card_layout
                            has_image_content,
                            false, // has_failed
                            window,
                            cx,
                        )
                    }),
            )
            .child(self.render_subagent_permission_buttons(
                entry_ix,
                context_ix,
                subagent_thread,
                tool_call_id,
                options,
                cx,
            ))
    }

    fn render_subagent_permission_buttons(
        &self,
        entry_ix: usize,
        context_ix: usize,
        subagent_thread: Entity<AcpThread>,
        tool_call_id: acp::ToolCallId,
        options: &PermissionOptions,
        cx: &Context<Self>,
    ) -> Div {
        match options {
            PermissionOptions::Flat(options) => self.render_subagent_permission_buttons_flat(
                entry_ix,
                context_ix,
                subagent_thread,
                tool_call_id,
                options,
                cx,
            ),
            PermissionOptions::Dropdown(options) => self
                .render_subagent_permission_buttons_dropdown(
                    entry_ix,
                    context_ix,
                    subagent_thread,
                    tool_call_id,
                    options,
                    cx,
                ),
        }
    }

    fn render_subagent_permission_buttons_flat(
        &self,
        entry_ix: usize,
        context_ix: usize,
        subagent_thread: Entity<AcpThread>,
        tool_call_id: acp::ToolCallId,
        options: &[acp::PermissionOption],
        cx: &Context<Self>,
    ) -> Div {
        div()
            .p_1()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx))
            .w_full()
            .v_flex()
            .gap_0p5()
            .children(options.iter().map(move |option| {
                let option_id = SharedString::from(format!(
                    "subagent-{}-{}-{}",
                    entry_ix, context_ix, option.option_id.0
                ));
                Button::new((option_id, entry_ix), option.name.clone())
                    .map(|this| match option.kind {
                        acp::PermissionOptionKind::AllowOnce => {
                            this.icon(IconName::Check).icon_color(Color::Success)
                        }
                        acp::PermissionOptionKind::AllowAlways => {
                            this.icon(IconName::CheckDouble).icon_color(Color::Success)
                        }
                        acp::PermissionOptionKind::RejectOnce
                        | acp::PermissionOptionKind::RejectAlways
                        | _ => this.icon(IconName::Close).icon_color(Color::Error),
                    })
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::XSmall)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener({
                        let subagent_thread = subagent_thread.clone();
                        let tool_call_id = tool_call_id.clone();
                        let option_id = option.option_id.clone();
                        let option_kind = option.kind;
                        move |this, _, window, cx| {
                            this.authorize_subagent_tool_call(
                                subagent_thread.clone(),
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

    fn authorize_subagent_tool_call(
        &mut self,
        subagent_thread: Entity<AcpThread>,
        tool_call_id: acp::ToolCallId,
        option_id: acp::PermissionOptionId,
        option_kind: acp::PermissionOptionKind,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        subagent_thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, option_id, option_kind, cx);
        });
    }

    fn render_subagent_permission_buttons_dropdown(
        &self,
        entry_ix: usize,
        context_ix: usize,
        subagent_thread: Entity<AcpThread>,
        tool_call_id: acp::ToolCallId,
        choices: &[PermissionOptionChoice],
        cx: &Context<Self>,
    ) -> Div {
        let selected_index = self
            .selected_permission_granularity
            .get(&tool_call_id)
            .copied()
            .unwrap_or_else(|| choices.len().saturating_sub(1));

        let selected_choice = choices.get(selected_index).or(choices.last());

        let dropdown_label: SharedString = selected_choice
            .map(|choice| choice.label())
            .unwrap_or_else(|| "Only this time".into());

        let (allow_option_id, allow_option_kind, deny_option_id, deny_option_kind) =
            if let Some(choice) = selected_choice {
                (
                    choice.allow.option_id.clone(),
                    choice.allow.kind,
                    choice.deny.option_id.clone(),
                    choice.deny.kind,
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
                        Button::new(
                            (
                                SharedString::from(format!(
                                    "subagent-allow-btn-{}-{}",
                                    entry_ix, context_ix
                                )),
                                entry_ix,
                            ),
                            "Allow",
                        )
                        .icon(IconName::Check)
                        .icon_color(Color::Success)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::XSmall)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener({
                            let subagent_thread = subagent_thread.clone();
                            let tool_call_id = tool_call_id.clone();
                            let option_id = allow_option_id;
                            let option_kind = allow_option_kind;
                            move |this, _, window, cx| {
                                this.authorize_subagent_tool_call(
                                    subagent_thread.clone(),
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
                        Button::new(
                            (
                                SharedString::from(format!(
                                    "subagent-deny-btn-{}-{}",
                                    entry_ix, context_ix
                                )),
                                entry_ix,
                            ),
                            "Deny",
                        )
                        .icon(IconName::Close)
                        .icon_color(Color::Error)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::XSmall)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener({
                            let tool_call_id = tool_call_id.clone();
                            let option_id = deny_option_id;
                            let option_kind = deny_option_kind;
                            move |this, _, window, cx| {
                                this.authorize_subagent_tool_call(
                                    subagent_thread.clone(),
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
            .child(self.render_subagent_permission_granularity_dropdown(
                choices,
                dropdown_label,
                entry_ix,
                context_ix,
                tool_call_id,
                selected_index,
                cx,
            ))
    }

    fn render_subagent_permission_granularity_dropdown(
        &self,
        choices: &[PermissionOptionChoice],
        current_label: SharedString,
        entry_ix: usize,
        context_ix: usize,
        tool_call_id: acp::ToolCallId,
        selected_index: usize,
        _cx: &Context<Self>,
    ) -> AnyElement {
        let menu_options: Vec<(usize, SharedString)> = choices
            .iter()
            .enumerate()
            .map(|(i, choice)| (i, choice.label()))
            .collect();

        let permission_dropdown_handle = self.permission_dropdown_handle.clone();

        PopoverMenu::new((
            SharedString::from(format!(
                "subagent-permission-granularity-{}-{}",
                entry_ix, context_ix
            )),
            entry_ix,
        ))
        .with_handle(permission_dropdown_handle)
        .trigger(
            Button::new(
                (
                    SharedString::from(format!(
                        "subagent-granularity-trigger-{}-{}",
                        entry_ix, context_ix
                    )),
                    entry_ix,
                ),
                current_label,
            )
            .icon(IconName::ChevronDown)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Muted)
            .label_size(LabelSize::Small),
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
        .into_any_element()
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

    pub(crate) fn render_thread_error(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Div> {
        let content = match self.thread_error.as_ref()? {
            ThreadError::Other { message, .. } => {
                self.render_any_thread_error(message.clone(), window, cx)
            }
            ThreadError::Refusal => self.render_refusal_error(cx),
            ThreadError::AuthenticationRequired(error) => {
                self.render_authentication_required_error(error.clone(), cx)
            }
            ThreadError::PaymentRequired => self.render_payment_required_error(cx),
        };

        Some(div().child(content))
    }

    fn render_refusal_error(&self, cx: &mut Context<'_, Self>) -> Callout {
        let model_or_agent_name = self.current_model_name(cx);
        let refusal_message = format!(
            "{} refused to respond to this prompt. \
            This can happen when a model believes the prompt violates its content policy \
            or safety guidelines, so rephrasing it can sometimes address the issue.",
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

    fn authenticate_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("authenticate", "Authenticate")
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Filled)
            .on_click(cx.listener({
                move |this, _, window, cx| {
                    let server_view = this.server_view.clone();
                    let agent_name = this.agent_name.clone();

                    this.clear_thread_error(cx);
                    if let Some(message) = this.in_flight_prompt.take() {
                        this.message_editor.update(cx, |editor, cx| {
                            editor.set_message(message, window, cx);
                        });
                    }
                    window.defer(cx, |window, cx| {
                        AcpServerView::handle_auth_required(
                            server_view,
                            AuthRequired::new(),
                            agent_name,
                            window,
                            cx,
                        );
                    })
                }
            }))
    }

    fn current_model_name(&self, cx: &App) -> SharedString {
        // For native agent (Zed Agent), use the specific model name (e.g., "Claude 3.5 Sonnet")
        // For ACP agents, use the agent name (e.g., "Claude Code", "Gemini CLI")
        // This provides better clarity about what refused the request
        if self.as_native_connection(cx).is_some() {
            self.model_selector
                .clone()
                .and_then(|selector| selector.read(cx).active_model(cx))
                .map(|model| model.name.clone())
                .unwrap_or_else(|| SharedString::from("The model"))
        } else {
            // ACP agent - use the agent name (e.g., "Claude Code", "Gemini CLI")
            self.agent_name.clone()
        }
    }

    fn render_any_thread_error(
        &mut self,
        error: SharedString,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) -> Callout {
        let can_resume = self.thread.read(cx).can_retry(cx);

        let markdown = if let Some(markdown) = &self.thread_error_markdown {
            markdown.clone()
        } else {
            let markdown = cx.new(|cx| Markdown::new(error.clone(), None, None, cx));
            self.thread_error_markdown = Some(markdown.clone());
            markdown
        };

        let markdown_style =
            MarkdownStyle::themed(MarkdownFont::Agent, window, cx).with_muted_text(cx);
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
                                    this.retry_generation(cx);
                                })),
                        )
                    })
                    .child(self.create_copy_button(error.to_string())),
            )
            .dismiss_action(self.dismiss_error_button(cx))
    }

    fn render_markdown(&self, markdown: Entity<Markdown>, style: MarkdownStyle) -> MarkdownElement {
        let workspace = self.workspace.clone();
        MarkdownElement::new(markdown, style).on_url_click(move |text, window, cx| {
            open_link(text, &workspace, window, cx);
        })
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

    fn render_resume_notice(_cx: &Context<Self>) -> AnyElement {
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
                                        self.server_view.clone(),
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

    fn render_new_version_callout(&self, version: &SharedString, cx: &mut Context<Self>) -> Div {
        let server_view = self.server_view.clone();
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
                        .on_click(move |_, window, cx| {
                            server_view
                                .update(cx, |view, cx| view.reset(window, cx))
                                .ok();
                        }),
                ),
        )
    }

    fn render_token_limit_callout(&self, cx: &mut Context<Self>) -> Option<Callout> {
        if self.token_limit_callout_dismissed {
            return None;
        }

        let token_usage = self.thread.read(cx).token_usage()?;
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
                                let session_id = this.thread.read(cx).session_id().clone();
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

    fn open_permission_dropdown(
        &mut self,
        _: &crate::OpenPermissionDropdown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.permission_dropdown_handle.clone().toggle(window, cx);
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

    fn cycle_thinking_effort(&mut self, cx: &mut Context<Self>) {
        if !cx.has_flag::<CloudThinkingEffortFeatureFlag>() {
            return;
        }

        let Some(thread) = self.as_native_thread(cx) else {
            return;
        };

        let (effort_levels, current_effort) = {
            let thread_ref = thread.read(cx);
            let Some(model) = thread_ref.model() else {
                return;
            };
            if !model.supports_thinking() || !thread_ref.thinking_enabled() {
                return;
            }
            let effort_levels = model.supported_effort_levels();
            if effort_levels.is_empty() {
                return;
            }
            let current_effort = thread_ref.thinking_effort().cloned();
            (effort_levels, current_effort)
        };

        let current_index = current_effort.and_then(|current| {
            effort_levels
                .iter()
                .position(|level| level.value == current)
        });
        let next_index = match current_index {
            Some(index) => (index + 1) % effort_levels.len(),
            None => 0,
        };
        let next_effort = effort_levels[next_index].value.to_string();

        thread.update(cx, |thread, cx| {
            thread.set_thinking_effort(Some(next_effort.clone()), cx);

            let fs = thread.project().read(cx).fs().clone();
            update_settings_file(fs, cx, move |settings, _| {
                if let Some(agent) = settings.agent.as_mut()
                    && let Some(default_model) = agent.default_model.as_mut()
                {
                    default_model.effort = Some(next_effort);
                }
            });
        });
    }

    fn toggle_thinking_effort_menu(
        &mut self,
        _action: &ToggleThinkingEffortMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let menu_handle = self.thinking_effort_menu_handle.clone();
        window.defer(cx, move |window, cx| {
            menu_handle.toggle(window, cx);
        });
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_messages = self.list_state.item_count() > 0;

        let conversation = v_flex().flex_1().map(|this| {
            let this = this.when(self.resumed_without_history, |this| {
                this.child(Self::render_resume_notice(cx))
            });
            if has_messages {
                let list_state = self.list_state.clone();
                this.child(self.render_entries(cx))
                    .vertical_scrollbar_for(&list_state, window, cx)
                    .into_any()
            } else {
                this.child(self.render_recent_history(cx)).into_any()
            }
        });

        v_flex()
            .key_context("AcpThread")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                if this.parent_id.is_none() {
                    this.cancel_generation(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &workspace::GoBack, window, cx| {
                if let Some(parent_session_id) = this.parent_id.clone() {
                    this.server_view
                        .update(cx, |view, cx| {
                            view.navigate_to_session(parent_session_id, window, cx);
                        })
                        .ok();
                }
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
            .on_action(cx.listener(|this, _: &CycleThinkingEffort, _window, cx| {
                this.cycle_thinking_effort(cx);
            }))
            .on_action(cx.listener(Self::toggle_thinking_effort_menu))
            .on_action(cx.listener(|this, _: &SendNextQueuedMessage, window, cx| {
                this.send_queued_message_at_index(0, true, window, cx);
            }))
            .on_action(cx.listener(|this, _: &RemoveFirstQueuedMessage, _, cx| {
                this.remove_from_queue(0, cx);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &EditFirstQueuedMessage, window, cx| {
                if let Some(editor) = this.queued_message_editors.first() {
                    window.focus(&editor.focus_handle(cx), cx);
                }
            }))
            .on_action(cx.listener(|this, _: &ClearMessageQueue, _, cx| {
                this.local_queued_messages.clear();
                this.sync_queue_flag_to_native_thread(cx);
                this.can_fast_track_queue = false;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleProfileSelector, window, cx| {
                if let Some(config_options_view) = this.config_options_view.clone() {
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

                if let Some(profile_selector) = this.profile_selector.clone() {
                    profile_selector.read(cx).menu_handle().toggle(window, cx);
                } else if let Some(mode_selector) = this.mode_selector.clone() {
                    mode_selector.read(cx).menu_handle().toggle(window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &CycleModeSelector, window, cx| {
                if let Some(config_options_view) = this.config_options_view.clone() {
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

                if let Some(profile_selector) = this.profile_selector.clone() {
                    profile_selector.update(cx, |profile_selector, cx| {
                        profile_selector.cycle_profile(cx);
                    });
                } else if let Some(mode_selector) = this.mode_selector.clone() {
                    mode_selector.update(cx, |mode_selector, cx| {
                        mode_selector.cycle_mode(window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &ToggleModelSelector, window, cx| {
                if let Some(config_options_view) = this.config_options_view.clone() {
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

                if let Some(model_selector) = this.model_selector.clone() {
                    model_selector
                        .update(cx, |model_selector, cx| model_selector.toggle(window, cx));
                }
            }))
            .on_action(cx.listener(|this, _: &CycleFavoriteModels, window, cx| {
                if let Some(config_options_view) = this.config_options_view.clone() {
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

                if let Some(model_selector) = this.model_selector.clone() {
                    model_selector.update(cx, |model_selector, cx| {
                        model_selector.cycle_favorite_models(window, cx);
                    });
                }
            }))
            .size_full()
            .children(self.render_subagent_titlebar(cx))
            .child(conversation)
            .children(self.render_activity_bar(window, cx))
            .when(self.show_codex_windows_warning, |this| {
                this.child(self.render_codex_windows_warning(cx))
            })
            .children(self.render_thread_retry_status_callout())
            .children(self.render_thread_error(window, cx))
            .when_some(
                match has_messages {
                    true => None,
                    false => self.new_server_version_available.clone(),
                },
                |this, version| this.child(self.render_new_version_callout(&version, cx)),
            )
            .children(self.render_token_limit_callout(cx))
            .child(self.render_message_editor(window, cx))
    }
}

pub(crate) fn open_link(
    url: SharedString,
    workspace: &WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        cx.open_url(&url);
        return;
    };

    if let Some(mention) = MentionUri::parse(&url, workspace.read(cx).path_style(cx)).log_err() {
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
                        let range =
                            Point::new(*line_range.start(), 0)..Point::new(*line_range.start(), 0);
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
            MentionUri::Diagnostics { .. } => {}
            MentionUri::TerminalSelection { .. } => {}
        })
    } else {
        cx.open_url(&url);
    }
}
