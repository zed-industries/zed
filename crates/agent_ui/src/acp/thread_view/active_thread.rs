use super::*;

pub struct ActiveThreadState {
    pub thread: Entity<AcpThread>,
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
    pub(super) thread_feedback: ThreadFeedbackState,
    pub list_state: ListState,
    pub prompt_capabilities: Rc<RefCell<PromptCapabilities>>,
    pub available_commands: Rc<RefCell<Vec<agent_client_protocol::AvailableCommand>>>,
    pub cached_user_commands: Rc<RefCell<HashMap<String, UserSlashCommand>>>,
    pub cached_user_command_errors: Rc<RefCell<Vec<CommandLoadError>>>,
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
    pub command_load_errors_dismissed: bool,
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
    pub _subscriptions: Vec<Subscription>,
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

impl ActiveThreadState {
    pub fn new(
        thread: Entity<AcpThread>,
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
        cached_user_commands: Rc<RefCell<HashMap<String, UserSlashCommand>>>,
        cached_user_command_errors: Rc<RefCell<Vec<CommandLoadError>>>,
        resumed_without_history: bool,
        resume_thread_metadata: Option<AgentSessionInfo>,
        subscriptions: Vec<Subscription>,
    ) -> Self {
        Self {
            thread,
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
            cached_user_commands,
            cached_user_command_errors,
            resumed_without_history,
            resume_thread_metadata,
            command_load_errors_dismissed: false,
            _subscriptions: subscriptions,
            permission_dropdown_handle: PopoverMenuHandle::default(),
            thread_retry_status: None,
            thread_error: None,
            thread_error_markdown: None,
            token_limit_callout_dismissed: false,
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

    pub fn has_queued_messages(&self) -> bool {
        !self.local_queued_messages.is_empty()
    }

    pub fn is_imported_thread(&self, cx: &App) -> bool {
        let Some(thread) = self.as_native_thread(cx) else {
            return false;
        };
        thread.read(cx).is_imported()
    }

    // turns

    pub fn start_turn(&mut self, cx: &mut Context<AcpThreadView>) -> usize {
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

    pub fn send(
        &mut self,
        message_editor: Entity<MessageEditor>,
        agent: Rc<dyn AgentServer>,
        login: Option<task::SpawnInTerminal>,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
    ) {
        let thread = &self.thread;

        if self.is_loading_contents {
            return;
        }

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
            let can_login = !connection.auth_methods().is_empty() || login.is_some();
            // Does the agent have a specific logout command? Prefer that in case they need to reset internal state.
            let logout_supported = text == "/logout"
                && self
                    .available_commands
                    .borrow()
                    .iter()
                    .any(|command| command.name == "logout");
            if can_login && !logout_supported {
                message_editor.update(cx, |editor, cx| editor.clear(window, cx));

                let this = cx.weak_entity();
                let agent = agent.clone();
                window.defer(cx, |window, cx| {
                    AcpThreadView::handle_auth_required(
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

        self.send_impl(message_editor, window, cx)
    }

    pub fn send_impl(
        &mut self,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
    ) {
        let full_mention_content = self.as_native_thread(cx).is_some_and(|thread| {
            // Include full contents when using minimal profile
            let thread = thread.read(cx);
            AgentSettings::get_global(cx)
                .profiles
                .get(thread.profile())
                .is_some_and(|profile| profile.tools.is_empty())
        });

        let cached_commands = &self.cached_user_commands;
        let cached_errors = &self.cached_user_command_errors;
        let contents = message_editor.update(cx, |message_editor, cx| {
            message_editor.contents_with_cache(
                full_mention_content,
                Some(cached_commands.borrow().clone()),
                Some(cached_errors.borrow().clone()),
                cx,
            )
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
        cx: &mut Context<AcpThreadView>,
    ) {
        let session_id = self.thread.read(cx).session_id().clone();
        let agent_telemetry_id = self.thread.read(cx).connection().telemetry_id();
        let thread = self.thread.downgrade();

        self.is_loading_contents = true;

        let model_id = self.current_model_id(cx);
        let mode_id = self.current_mode_id(cx);
        let guard = cx.new(|_| ());
        cx.observe_release(&guard, |this, _guard, cx| {
            if let ThreadState::Active(ActiveThreadState {
                is_loading_contents,
                ..
            }) = &mut this.thread_state
            {
                *is_loading_contents = false;
            }
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
                    if let ThreadState::Active(ActiveThreadState {
                        should_be_following,
                        ..
                    }) = &mut this.thread_state
                    {
                        *should_be_following = this
                            .workspace
                            .update(cx, |workspace, _| {
                                workspace.is_being_followed(CollaboratorId::Agent)
                            })
                            .unwrap_or_default();
                    }
                })
                .ok();
            }
        })
        .detach();
    }

    pub fn interrupt_and_send(
        &mut self,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
    ) {
        let thread = &self.thread;

        if self.is_loading_contents {
            return;
        }

        if thread.read(cx).status() == ThreadStatus::Idle {
            self.send_impl(message_editor, window, cx);
            return;
        }

        self.stop_current_and_send_new_message(window, cx);
    }

    pub fn stop_current_and_send_new_message(
        &mut self,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
    ) {
        let thread = self.thread.clone();
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

    // generation

    pub fn cancel_generation(&mut self, cx: &mut Context<AcpThreadView>) {
        self.thread_retry_status.take();
        self.thread_error.take();
        self.user_interrupted_generation = true;
        self._cancel_task = Some(self.thread.update(cx, |thread, cx| thread.cancel(cx)));
    }

    pub fn retry_generation(&mut self, cx: &mut Context<AcpThreadView>) {
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
                    this.handle_thread_error(err, cx);
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
        cx: &mut Context<AcpThreadView>,
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
            this.update_in(cx, |this, window, cx| {
                this.send_impl(message_editor, window, cx);
                this.focus_handle(cx).focus(window, cx);
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    // message queueing

    pub fn queue_message(
        &mut self,
        message_editor: Entity<MessageEditor>,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
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

        let cached_commands = self.cached_user_commands.borrow().clone();
        let cached_errors = self.cached_user_command_errors.borrow().clone();
        let contents = message_editor.update(cx, |message_editor, cx| {
            message_editor.contents_with_cache(
                full_mention_content,
                Some(cached_commands),
                Some(cached_errors),
                cx,
            )
        });

        cx.spawn_in(window, async move |this, cx| {
            let (content, tracked_buffers) = contents.await?;

            if content.is_empty() {
                return Ok::<(), anyhow::Error>(());
            }

            this.update_in(cx, |this, window, cx| {
                this.add_to_queue(content, tracked_buffers, cx);
                // Enable fast-track: user can press Enter again to send this queued message immediately
                this.set_can_fast_track_queue(true);
                message_editor.update(cx, |message_editor, cx| {
                    message_editor.clear(window, cx);
                });
                cx.notify();
            })?;
            Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn remove_from_queue(
        &mut self,
        index: usize,
        cx: &mut Context<AcpThreadView>,
    ) -> Option<QueuedMessage> {
        if index < self.local_queued_messages.len() {
            let removed = self.local_queued_messages.remove(index);
            self.sync_queue_flag_to_native_thread(cx);
            Some(removed)
        } else {
            None
        }
    }

    pub fn sync_queue_flag_to_native_thread(&self, cx: &mut Context<AcpThreadView>) {
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
        cx: &mut Context<AcpThreadView>,
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
        message_editor: Entity<MessageEditor>,
        cx: &mut Context<AcpThreadView>,
    ) {
        self.set_editor_is_expanded(!self.editor_expanded, message_editor, cx);
        cx.stop_propagation();
        cx.notify();
    }

    pub fn set_editor_is_expanded(
        &mut self,
        is_expanded: bool,
        message_editor: Entity<MessageEditor>,
        cx: &mut Context<AcpThreadView>,
    ) {
        self.editor_expanded = is_expanded;
        message_editor.update(cx, |editor, cx| {
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
        cx: &mut Context<AcpThreadView>,
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

    pub fn cancel_editing(
        &mut self,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
    ) {
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
        focus_handle.focus(window, cx);
        cx.notify();
    }

    // tool permissions

    pub fn authorize_tool_call(
        &mut self,
        tool_call_id: acp::ToolCallId,
        option_id: acp::PermissionOptionId,
        option_kind: acp::PermissionOptionKind,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
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

    pub fn authorize_pending_tool_call(
        &mut self,
        kind: acp::PermissionOptionKind,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
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

    pub fn handle_select_permission_granularity(
        &mut self,
        action: &SelectPermissionGranularity,
        cx: &mut Context<AcpThreadView>,
    ) {
        let tool_call_id = acp::ToolCallId::new(action.tool_call_id.clone());
        self.selected_permission_granularity
            .insert(tool_call_id, action.index);

        cx.notify();
    }

    // edits

    pub fn keep_all(&mut self, cx: &mut Context<AcpThreadView>) {
        let thread = &self.thread;
        let telemetry = ActionLogTelemetry::from(thread.read(cx));
        let action_log = thread.read(cx).action_log().clone();
        action_log.update(cx, |action_log, cx| {
            action_log.keep_all_edits(Some(telemetry), cx)
        });
    }

    pub fn reject_all(&mut self, cx: &mut Context<AcpThreadView>) {
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
        cx: &mut Context<AcpThreadView>,
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

    pub fn sync_thread(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<AcpThreadView>,
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
                if let ThreadState::Active(ActiveThreadState {
                    resume_thread_metadata,
                    ..
                }) = &mut this.thread_state
                {
                    *resume_thread_metadata = Some(thread_metadata);
                }
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

    pub fn restore_checkpoint(
        &mut self,
        message_id: &UserMessageId,
        cx: &mut Context<AcpThreadView>,
    ) {
        self.thread
            .update(cx, |thread, cx| {
                thread.restore_checkpoint(message_id.clone(), cx)
            })
            .detach_and_log_err(cx);
    }

    pub fn clear_thread_error(&mut self, cx: &mut Context<AcpThreadView>) {
        self.thread_error = None;
        self.thread_error_markdown = None;
        self.token_limit_callout_dismissed = true;
        cx.notify();
    }

    // other

    pub fn refresh_cached_user_commands_from_registry(
        &mut self,
        registry: &Entity<SlashCommandRegistry>,
        cx: &App,
    ) {
        let (mut commands, mut errors) = registry.read_with(cx, |registry, _| {
            (registry.commands().clone(), registry.errors().to_vec())
        });
        let server_command_names = self
            .available_commands
            .borrow()
            .iter()
            .map(|command| command.name.clone())
            .collect::<HashSet<_>>();
        user_slash_command::apply_server_command_conflicts_to_map(
            &mut commands,
            &mut errors,
            &server_command_names,
        );

        self.command_load_errors_dismissed = false;
        *self.cached_user_commands.borrow_mut() = commands;
        *self.cached_user_command_errors.borrow_mut() = errors;
    }

    pub fn render_command_load_errors(
        &self,
        cx: &mut Context<AcpThreadView>,
    ) -> Option<impl IntoElement> {
        let errors = self.cached_user_command_errors.borrow();

        if self.command_load_errors_dismissed || errors.is_empty() {
            return None;
        }

        let workspace = self.workspace.clone();

        let error_count = errors.len();
        let title = if error_count == 1 {
            "Failed to load slash command"
        } else {
            "Failed to load slash commands"
        };

        Some(
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .title(title)
                .actions_slot(
                    IconButton::new("dismiss-command-errors", IconName::Close)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text("Dismiss Error"))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.clear_command_load_errors(cx);
                        })),
                )
                .description_slot(v_flex().children(errors.iter().enumerate().map({
                    move |(i, error)| {
                        let path = error.path.clone();
                        let workspace = workspace.clone();
                        let file_name = error
                            .path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| error.path.display().to_string());
                        let id = ElementId::Name(format!("command-error-{i}").into());
                        let label = format!("— {}: {}", file_name, error.message);

                        Button::new(id, label)
                            .label_size(LabelSize::Small)
                            .truncate(true)
                            .tooltip({
                                let message: SharedString = error.message.clone().into();
                                let path: SharedString = error.path.display().to_string().into();
                                move |_, cx| {
                                    Tooltip::with_meta(message.clone(), None, path.clone(), cx)
                                }
                            })
                            .on_click({
                                move |_, window, cx| {
                                    if let Some(workspace) = workspace.upgrade() {
                                        workspace.update(cx, |workspace, cx| {
                                            workspace
                                                .open_abs_path(
                                                    path.clone(),
                                                    OpenOptions::default(),
                                                    window,
                                                    cx,
                                                )
                                                .detach_and_log_err(cx);
                                        });
                                    }
                                }
                            })
                    }
                }))),
        )
    }

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

    pub fn handle_open_rules(&mut self, window: &mut Window, cx: &mut Context<AcpThreadView>) {
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
}
