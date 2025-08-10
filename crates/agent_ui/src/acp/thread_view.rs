use acp_thread::{AgentConnection, Plan};
use agent_servers::AgentServer;
use agent_settings::{AgentSettings, NotifyWhenAgentWaiting};
use audio::{Audio, Sound};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitStatus;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use action_log::ActionLog;
use agent_client_protocol as acp;
use buffer_diff::BufferDiff;
use collections::{HashMap, HashSet};
use editor::{
    AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorMode,
    EditorStyle, MinimapVisibility, MultiBuffer, PathKey,
};
use file_icons::FileIcons;
use gpui::{
    Action, Animation, AnimationExt, App, BorderStyle, EdgesRefinement, Empty, Entity, EntityId,
    FocusHandle, Focusable, Hsla, Length, ListOffset, ListState, MouseButton, PlatformDisplay,
    SharedString, Stateful, StyleRefinement, Subscription, Task, TextStyle, TextStyleRefinement,
    Transformation, UnderlineStyle, WeakEntity, Window, WindowHandle, div, linear_color_stop,
    linear_gradient, list, percentage, point, prelude::*, pulsating_between,
};
use language::language_settings::SoftWrap;
use language::{Buffer, Language};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use parking_lot::Mutex;
use project::{CompletionIntent, Project};
use settings::{Settings as _, SettingsStore};
use text::{Anchor, BufferSnapshot};
use theme::ThemeSettings;
use ui::{
    Disclosure, Divider, DividerColor, KeyBinding, Scrollbar, ScrollbarState, Tooltip, prelude::*,
};
use util::ResultExt;
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::{Chat, NextHistoryMessage, PreviousHistoryMessage};

use ::acp_thread::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, AssistantMessage, AssistantMessageChunk,
    LoadError, MentionPath, ThreadStatus, ToolCall, ToolCallContent, ToolCallStatus,
};

use crate::acp::completion_provider::{ContextPickerCompletionProvider, MentionSet};
use crate::acp::message_history::MessageHistory;
use crate::agent_diff::AgentDiff;
use crate::message_editor::{MAX_EDITOR_LINES, MIN_EDITOR_LINES};
use crate::ui::{AgentNotification, AgentNotificationEvent};
use crate::{
    AgentDiffPane, AgentPanel, ExpandMessageEditor, Follow, KeepAll, OpenAgentDiff, RejectAll,
};

const RESPONSE_PADDING_X: Pixels = px(19.);

pub struct AcpThreadView {
    agent: Rc<dyn AgentServer>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    thread_state: ThreadState,
    diff_editors: HashMap<EntityId, Entity<Editor>>,
    message_editor: Entity<Editor>,
    message_set_from_history: Option<BufferSnapshot>,
    _message_editor_subscription: Subscription,
    mention_set: Arc<Mutex<MentionSet>>,
    notifications: Vec<WindowHandle<AgentNotification>>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    last_error: Option<Entity<Markdown>>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    auth_task: Option<Task<()>>,
    expanded_tool_calls: HashSet<acp::ToolCallId>,
    expanded_thinking_blocks: HashSet<(usize, usize)>,
    edits_expanded: bool,
    plan_expanded: bool,
    editor_expanded: bool,
    message_history: Rc<RefCell<MessageHistory<Vec<acp::ContentBlock>>>>,
    _cancel_task: Option<Task<()>>,
    _subscriptions: [Subscription; 1],
}

enum ThreadState {
    Loading {
        _task: Task<()>,
    },
    Ready {
        thread: Entity<AcpThread>,
        _subscription: [Subscription; 2],
    },
    LoadError(LoadError),
    Unauthenticated {
        connection: Rc<dyn AgentConnection>,
    },
    ServerExited {
        status: ExitStatus,
    },
}

impl AcpThreadView {
    pub fn new(
        agent: Rc<dyn AgentServer>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        message_history: Rc<RefCell<MessageHistory<Vec<acp::ContentBlock>>>>,
        min_lines: usize,
        max_lines: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language = Language::new(
            language::LanguageConfig {
                completion_query_characters: HashSet::from_iter(['.', '-', '_', '@']),
                ..Default::default()
            },
            None,
        );

        let mention_set = Arc::new(Mutex::new(MentionSet::default()));

        let message_editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx).with_language(Arc::new(language), cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight {
                    min_lines,
                    max_lines: max_lines,
                },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_placeholder_text("Message the agent － @ to include files", cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_completion_provider(Some(Rc::new(ContextPickerCompletionProvider::new(
                mention_set.clone(),
                workspace.clone(),
                cx.weak_entity(),
            ))));
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor
        });

        let message_editor_subscription =
            cx.subscribe(&message_editor, |this, editor, event, cx| {
                if let editor::EditorEvent::BufferEdited = &event {
                    let buffer = editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .unwrap()
                        .read(cx)
                        .snapshot();
                    if let Some(message) = this.message_set_from_history.clone()
                        && message.version() != buffer.version()
                    {
                        this.message_set_from_history = None;
                    }

                    if this.message_set_from_history.is_none() {
                        this.message_history.borrow_mut().reset_position();
                    }
                }
            });

        let mention_set = mention_set.clone();

        let list_state = ListState::new(0, gpui::ListAlignment::Bottom, px(2048.0));

        let subscription = cx.observe_global_in::<SettingsStore>(window, Self::settings_changed);

        Self {
            agent: agent.clone(),
            workspace: workspace.clone(),
            project: project.clone(),
            thread_state: Self::initial_state(agent, workspace, project, window, cx),
            message_editor,
            message_set_from_history: None,
            _message_editor_subscription: message_editor_subscription,
            mention_set,
            notifications: Vec::new(),
            notification_subscriptions: HashMap::default(),
            diff_editors: Default::default(),
            list_state: list_state.clone(),
            scrollbar_state: ScrollbarState::new(list_state).parent_entity(&cx.entity()),
            last_error: None,
            auth_task: None,
            expanded_tool_calls: HashSet::default(),
            expanded_thinking_blocks: HashSet::default(),
            edits_expanded: false,
            plan_expanded: false,
            editor_expanded: false,
            message_history,
            _subscriptions: [subscription],
            _cancel_task: None,
        }
    }

    fn initial_state(
        agent: Rc<dyn AgentServer>,
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

        let connect_task = agent.connect(&root_dir, &project, cx);
        let load_task = cx.spawn_in(window, async move |this, cx| {
            let connection = match connect_task.await {
                Ok(connection) => connection,
                Err(err) => {
                    this.update(cx, |this, cx| {
                        this.handle_load_error(err, cx);
                        cx.notify();
                    })
                    .log_err();
                    return;
                }
            };

            // this.update_in(cx, |_this, _window, cx| {
            //     let status = connection.exit_status(cx);
            //     cx.spawn(async move |this, cx| {
            //         let status = status.await.ok();
            //         this.update(cx, |this, cx| {
            //             this.thread_state = ThreadState::ServerExited { status };
            //             cx.notify();
            //         })
            //         .ok();
            //     })
            //     .detach();
            // })
            // .ok();

            let result = match connection
                .clone()
                .new_thread(project.clone(), &root_dir, cx)
                .await
            {
                Err(e) => {
                    let mut cx = cx.clone();
                    if e.is::<acp_thread::AuthRequired>() {
                        this.update(&mut cx, |this, cx| {
                            this.thread_state = ThreadState::Unauthenticated { connection };
                            cx.notify();
                        })
                        .ok();
                        return;
                    } else {
                        Err(e)
                    }
                }
                Ok(session_id) => Ok(session_id),
            };

            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(thread) => {
                        let thread_subscription =
                            cx.subscribe_in(&thread, window, Self::handle_thread_event);

                        let action_log = thread.read(cx).action_log().clone();
                        let action_log_subscription =
                            cx.observe(&action_log, |_, _, cx| cx.notify());

                        this.list_state
                            .splice(0..0, thread.read(cx).entries().len());

                        AgentDiff::set_active_thread(&workspace, thread.clone(), window, cx);

                        this.thread_state = ThreadState::Ready {
                            thread,
                            _subscription: [thread_subscription, action_log_subscription],
                        };

                        cx.notify();
                    }
                    Err(err) => {
                        this.handle_load_error(err, cx);
                    }
                };
            })
            .log_err();
        });

        ThreadState::Loading { _task: load_task }
    }

    fn handle_load_error(&mut self, err: anyhow::Error, cx: &mut Context<Self>) {
        if let Some(load_err) = err.downcast_ref::<LoadError>() {
            self.thread_state = ThreadState::LoadError(load_err.clone());
        } else {
            self.thread_state = ThreadState::LoadError(LoadError::Other(err.to_string().into()))
        }
        cx.notify();
    }

    pub fn thread(&self) -> Option<&Entity<AcpThread>> {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => Some(thread),
            ThreadState::Unauthenticated { .. }
            | ThreadState::Loading { .. }
            | ThreadState::LoadError(..)
            | ThreadState::ServerExited { .. } => None,
        }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => thread.read(cx).title(),
            ThreadState::Loading { .. } => "Loading…".into(),
            ThreadState::LoadError(_) => "Failed to load".into(),
            ThreadState::Unauthenticated { .. } => "Not authenticated".into(),
            ThreadState::ServerExited { .. } => "Server exited unexpectedly".into(),
        }
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        self.last_error.take();

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
        self.message_editor.update(cx, |editor, _| {
            if self.editor_expanded {
                editor.set_mode(EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sized_by_content: false,
                })
            } else {
                editor.set_mode(EditorMode::AutoHeight {
                    min_lines: MIN_EDITOR_LINES,
                    max_lines: Some(MAX_EDITOR_LINES),
                })
            }
        });
        cx.notify();
    }

    fn chat(&mut self, _: &Chat, window: &mut Window, cx: &mut Context<Self>) {
        self.last_error.take();

        let mut ix = 0;
        let mut chunks: Vec<acp::ContentBlock> = Vec::new();
        let project = self.project.clone();
        self.message_editor.update(cx, |editor, cx| {
            let text = editor.text(cx);
            editor.display_map.update(cx, |map, cx| {
                let snapshot = map.snapshot(cx);
                for (crease_id, crease) in snapshot.crease_snapshot.creases() {
                    // Skip creases that have been edited out of the message buffer.
                    if !crease.range().start.is_valid(&snapshot.buffer_snapshot) {
                        continue;
                    }

                    if let Some(project_path) =
                        self.mention_set.lock().path_for_crease_id(crease_id)
                    {
                        let crease_range = crease.range().to_offset(&snapshot.buffer_snapshot);
                        if crease_range.start > ix {
                            chunks.push(text[ix..crease_range.start].into());
                        }
                        if let Some(abs_path) = project.read(cx).absolute_path(&project_path, cx) {
                            let path_str = abs_path.display().to_string();
                            chunks.push(acp::ContentBlock::ResourceLink(acp::ResourceLink {
                                uri: path_str.clone(),
                                name: path_str,
                                annotations: None,
                                description: None,
                                mime_type: None,
                                size: None,
                                title: None,
                            }));
                        }
                        ix = crease_range.end;
                    }
                }

                if ix < text.len() {
                    let last_chunk = text[ix..].trim_end();
                    if !last_chunk.is_empty() {
                        chunks.push(last_chunk.into());
                    }
                }
            })
        });

        if chunks.is_empty() {
            return;
        }

        let Some(thread) = self.thread() else {
            return;
        };
        let task = thread.update(cx, |thread, cx| thread.send(chunks.clone(), cx));

        cx.spawn(async move |this, cx| {
            let result = task.await;

            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    this.last_error =
                        Some(cx.new(|cx| Markdown::new(err.to_string().into(), None, None, cx)))
                }
            })
        })
        .detach();

        let mention_set = self.mention_set.clone();

        self.set_editor_is_expanded(false, cx);

        self.message_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
            editor.remove_creases(mention_set.lock().drain(), cx)
        });

        self.scroll_to_bottom(cx);

        self.message_history.borrow_mut().push(chunks);
    }

    fn previous_history_message(
        &mut self,
        _: &PreviousHistoryMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.message_set_from_history.is_none() && !self.message_editor.read(cx).is_empty(cx) {
            self.message_editor.update(cx, |editor, cx| {
                editor.move_up(&Default::default(), window, cx);
            });
            return;
        }

        self.message_set_from_history = Self::set_draft_message(
            self.message_editor.clone(),
            self.mention_set.clone(),
            self.project.clone(),
            self.message_history
                .borrow_mut()
                .prev()
                .map(|blocks| blocks.as_slice()),
            window,
            cx,
        );
    }

    fn next_history_message(
        &mut self,
        _: &NextHistoryMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.message_set_from_history.is_none() {
            self.message_editor.update(cx, |editor, cx| {
                editor.move_down(&Default::default(), window, cx);
            });
            return;
        }

        let mut message_history = self.message_history.borrow_mut();
        let next_history = message_history.next();

        let set_draft_message = Self::set_draft_message(
            self.message_editor.clone(),
            self.mention_set.clone(),
            self.project.clone(),
            Some(
                next_history
                    .map(|blocks| blocks.as_slice())
                    .unwrap_or_else(|| &[]),
            ),
            window,
            cx,
        );
        // If we reset the text to an empty string because we ran out of history,
        // we don't want to mark it as coming from the history
        self.message_set_from_history = if next_history.is_some() {
            set_draft_message
        } else {
            None
        };
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
            diff.move_to_path(PathKey::for_buffer(&buffer, cx), window, cx)
        })
    }

    fn set_draft_message(
        message_editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
        project: Entity<Project>,
        message: Option<&[acp::ContentBlock]>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<BufferSnapshot> {
        cx.notify();

        let message = message?;

        let mut text = String::new();
        let mut mentions = Vec::new();

        for chunk in message {
            match chunk {
                acp::ContentBlock::Text(text_content) => {
                    text.push_str(&text_content.text);
                }
                acp::ContentBlock::ResourceLink(resource_link) => {
                    let path = Path::new(&resource_link.uri);
                    let start = text.len();
                    let content = MentionPath::new(&path).to_string();
                    text.push_str(&content);
                    let end = text.len();
                    if let Some(project_path) =
                        project.read(cx).project_path_for_absolute_path(&path, cx)
                    {
                        let filename: SharedString = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                            .into();
                        mentions.push((start..end, project_path, filename));
                    }
                }
                acp::ContentBlock::Image(_)
                | acp::ContentBlock::Audio(_)
                | acp::ContentBlock::Resource(_) => {}
            }
        }

        let snapshot = message_editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
            editor.buffer().read(cx).snapshot(cx)
        });

        for (range, project_path, filename) in mentions {
            let crease_icon_path = if project_path.path.is_dir() {
                FileIcons::get_folder_icon(false, cx)
                    .unwrap_or_else(|| IconName::Folder.path().into())
            } else {
                FileIcons::get_icon(Path::new(project_path.path.as_ref()), cx)
                    .unwrap_or_else(|| IconName::File.path().into())
            };

            let anchor = snapshot.anchor_before(range.start);
            let crease_id = crate::context_picker::insert_crease_for_mention(
                anchor.excerpt_id,
                anchor.text_anchor,
                range.end - range.start,
                filename,
                crease_icon_path,
                message_editor.clone(),
                window,
                cx,
            );
            if let Some(crease_id) = crease_id {
                mention_set.lock().insert(crease_id, project_path);
            }
        }

        let snapshot = snapshot.as_singleton().unwrap().2.clone();
        Some(snapshot.text)
    }

    fn handle_thread_event(
        &mut self,
        thread: &Entity<AcpThread>,
        event: &AcpThreadEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.list_state.item_count();
        match event {
            AcpThreadEvent::NewEntry => {
                let index = thread.read(cx).entries().len() - 1;
                self.sync_thread_entry_view(index, window, cx);
                self.list_state.splice(count..count, 1);
            }
            AcpThreadEvent::EntryUpdated(index) => {
                let index = *index;
                self.sync_thread_entry_view(index, window, cx);
                self.list_state.splice(index..index + 1, 1);
            }
            AcpThreadEvent::ToolAuthorizationRequired => {
                self.notify_with_sound("Waiting for tool confirmation", IconName::Info, window, cx);
            }
            AcpThreadEvent::Stopped => {
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
                self.notify_with_sound(
                    "Agent stopped due to an error",
                    IconName::Warning,
                    window,
                    cx,
                );
            }
            AcpThreadEvent::ServerExited(status) => {
                self.thread_state = ThreadState::ServerExited { status: *status };
            }
        }
        cx.notify();
    }

    fn sync_thread_entry_view(
        &mut self,
        entry_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multibuffers) = self.entry_diff_multibuffers(entry_ix, cx) else {
            return;
        };

        let multibuffers = multibuffers.collect::<Vec<_>>();

        for multibuffer in multibuffers {
            if self.diff_editors.contains_key(&multibuffer.entity_id()) {
                return;
            }

            let editor = cx.new(|cx| {
                let mut editor = Editor::new(
                    EditorMode::Full {
                        scale_ui_elements_with_buffer_font_size: false,
                        show_active_line_background: false,
                        sized_by_content: true,
                    },
                    multibuffer.clone(),
                    None,
                    window,
                    cx,
                );
                editor.set_show_gutter(false, cx);
                editor.disable_inline_diagnostics();
                editor.disable_expand_excerpt_buttons(cx);
                editor.set_show_vertical_scrollbar(false, cx);
                editor.set_minimap_visibility(MinimapVisibility::Disabled, window, cx);
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.scroll_manager.set_forbid_vertical_scroll(true);
                editor.set_show_indent_guides(false, cx);
                editor.set_read_only(true);
                editor.set_show_breakpoints(false, cx);
                editor.set_show_code_actions(false, cx);
                editor.set_show_git_diff_gutter(false, cx);
                editor.set_expand_all_diff_hunks(cx);
                editor.set_text_style_refinement(diff_editor_text_style_refinement(cx));
                editor
            });
            let entity_id = multibuffer.entity_id();
            cx.observe_release(&multibuffer, move |this, _, _| {
                this.diff_editors.remove(&entity_id);
            })
            .detach();

            self.diff_editors.insert(entity_id, editor);
        }
    }

    fn entry_diff_multibuffers(
        &self,
        entry_ix: usize,
        cx: &App,
    ) -> Option<impl Iterator<Item = Entity<MultiBuffer>>> {
        let entry = self.thread()?.read(cx).entries().get(entry_ix)?;
        Some(
            entry
                .diffs()
                .map(|diff| diff.read(cx).multibuffer().clone()),
        )
    }

    fn authenticate(
        &mut self,
        method: acp::AuthMethodId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ThreadState::Unauthenticated { ref connection } = self.thread_state else {
            return;
        };

        self.last_error.take();
        let authenticate = connection.authenticate(method, cx);
        self.auth_task = Some(cx.spawn_in(window, {
            let project = self.project.clone();
            let agent = self.agent.clone();
            async move |this, cx| {
                let result = authenticate.await;

                this.update_in(cx, |this, window, cx| {
                    if let Err(err) = result {
                        this.last_error = Some(cx.new(|cx| {
                            Markdown::new(format!("Error: {err}").into(), None, None, cx)
                        }))
                    } else {
                        this.thread_state = Self::initial_state(
                            agent,
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

    fn authorize_tool_call(
        &mut self,
        tool_call_id: acp::ToolCallId,
        option_id: acp::PermissionOptionId,
        option_kind: acp::PermissionOptionKind,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread() else {
            return;
        };
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(tool_call_id, option_id, option_kind, cx);
        });
        cx.notify();
    }

    fn render_entry(
        &self,
        index: usize,
        total_entries: usize,
        entry: &AgentThreadEntry,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let primary = match &entry {
            AgentThreadEntry::UserMessage(message) => div()
                .py_4()
                .px_2()
                .child(
                    v_flex()
                        .p_3()
                        .gap_1p5()
                        .rounded_lg()
                        .shadow_md()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .text_xs()
                        .children(message.content.markdown().map(|md| {
                            self.render_markdown(
                                md.clone(),
                                user_message_markdown_style(window, cx),
                            )
                        })),
                )
                .into_any(),
            AgentThreadEntry::AssistantMessage(AssistantMessage { chunks }) => {
                let style = default_markdown_style(false, window, cx);
                let message_body = v_flex()
                    .w_full()
                    .gap_2p5()
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
                                        index,
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
                    .py_1()
                    .when(index + 1 == total_entries, |this| this.pb_4())
                    .w_full()
                    .text_ui(cx)
                    .child(message_body)
                    .into_any()
            }
            AgentThreadEntry::ToolCall(tool_call) => div()
                .w_full()
                .py_1p5()
                .px_5()
                .child(self.render_tool_call(index, tool_call, window, cx))
                .into_any(),
        };

        let Some(thread) = self.thread() else {
            return primary;
        };
        let is_generating = matches!(thread.read(cx).status(), ThreadStatus::Generating);
        if index == total_entries - 1 && !is_generating {
            v_flex()
                .w_full()
                .child(primary)
                .child(self.render_thread_controls(cx))
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
        cx.theme().colors().border.opacity(0.6)
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

        v_flex()
            .child(
                h_flex()
                    .id(header_id)
                    .group(&card_header_id)
                    .relative()
                    .w_full()
                    .gap_1p5()
                    .opacity(0.8)
                    .hover(|style| style.opacity(1.))
                    .child(
                        h_flex()
                            .size_4()
                            .justify_center()
                            .child(
                                div()
                                    .group_hover(&card_header_id, |s| s.invisible().w_0())
                                    .child(
                                        Icon::new(IconName::ToolThink)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .absolute()
                                    .inset_0()
                                    .invisible()
                                    .justify_center()
                                    .group_hover(&card_header_id, |s| s.visible())
                                    .child(
                                        Disclosure::new(("expand", entry_ix), is_open)
                                            .opened_icon(IconName::ChevronUp)
                                            .closed_icon(IconName::ChevronRight)
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
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .text_size(self.tool_name_font_size())
                            .child("Thinking"),
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
                        .relative()
                        .mt_1p5()
                        .ml(px(7.))
                        .pl_4()
                        .border_l_1()
                        .border_color(self.tool_card_border_color(cx))
                        .text_ui_sm(cx)
                        .child(
                            self.render_markdown(chunk, default_markdown_style(false, window, cx)),
                        ),
                )
            })
            .into_any_element()
    }

    fn render_tool_call_icon(
        &self,
        group_name: SharedString,
        entry_ix: usize,
        is_collapsible: bool,
        is_open: bool,
        tool_call: &ToolCall,
        cx: &Context<Self>,
    ) -> Div {
        let tool_icon = Icon::new(match tool_call.kind {
            acp::ToolKind::Read => IconName::ToolRead,
            acp::ToolKind::Edit => IconName::ToolPencil,
            acp::ToolKind::Delete => IconName::ToolDeleteFile,
            acp::ToolKind::Move => IconName::ArrowRightLeft,
            acp::ToolKind::Search => IconName::ToolSearch,
            acp::ToolKind::Execute => IconName::ToolTerminal,
            acp::ToolKind::Think => IconName::ToolThink,
            acp::ToolKind::Fetch => IconName::ToolWeb,
            acp::ToolKind::Other => IconName::ToolHammer,
        })
        .size(IconSize::Small)
        .color(Color::Muted);

        if is_collapsible {
            h_flex()
                .size_4()
                .justify_center()
                .child(
                    div()
                        .group_hover(&group_name, |s| s.invisible().w_0())
                        .child(tool_icon),
                )
                .child(
                    h_flex()
                        .absolute()
                        .inset_0()
                        .invisible()
                        .justify_center()
                        .group_hover(&group_name, |s| s.visible())
                        .child(
                            Disclosure::new(("expand", entry_ix), is_open)
                                .opened_icon(IconName::ChevronUp)
                                .closed_icon(IconName::ChevronRight)
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
                        ),
                )
        } else {
            div().child(tool_icon)
        }
    }

    fn render_tool_call(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let header_id = SharedString::from(format!("outer-tool-call-header-{}", entry_ix));
        let card_header_id = SharedString::from("inner-tool-call-header");

        let status_icon = match &tool_call.status {
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Pending,
            }
            | ToolCallStatus::WaitingForConfirmation { .. } => None,
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::InProgress,
                ..
            } => Some(
                Icon::new(IconName::ArrowCircle)
                    .color(Color::Accent)
                    .size(IconSize::Small)
                    .with_animation(
                        "running",
                        Animation::new(Duration::from_secs(2)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    )
                    .into_any(),
            ),
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Completed,
                ..
            } => None,
            ToolCallStatus::Rejected
            | ToolCallStatus::Canceled
            | ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Failed,
                ..
            } => Some(
                Icon::new(IconName::Close)
                    .color(Color::Error)
                    .size(IconSize::Small)
                    .into_any_element(),
            ),
        };

        let needs_confirmation = match &tool_call.status {
            ToolCallStatus::WaitingForConfirmation { .. } => true,
            _ => tool_call
                .content
                .iter()
                .any(|content| matches!(content, ToolCallContent::Diff { .. })),
        };

        let is_collapsible = !tool_call.content.is_empty() && !needs_confirmation;
        let is_open = !is_collapsible || self.expanded_tool_calls.contains(&tool_call.id);

        let gradient_color = cx.theme().colors().panel_background;
        let gradient_overlay = {
            div()
                .absolute()
                .top_0()
                .right_0()
                .w_12()
                .h_full()
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(gradient_color, 1.),
                    linear_color_stop(gradient_color.opacity(0.2), 0.),
                ))
        };

        v_flex()
            .when(needs_confirmation, |this| {
                this.rounded_lg()
                    .border_1()
                    .border_color(self.tool_card_border_color(cx))
                    .bg(cx.theme().colors().editor_background)
                    .overflow_hidden()
            })
            .child(
                h_flex()
                    .id(header_id)
                    .w_full()
                    .gap_1()
                    .justify_between()
                    .map(|this| {
                        if needs_confirmation {
                            this.pl_2()
                                .pr_1()
                                .py_1()
                                .rounded_t_md()
                                .border_b_1()
                                .border_color(self.tool_card_border_color(cx))
                                .bg(self.tool_card_header_bg(cx))
                        } else {
                            this.opacity(0.8).hover(|style| style.opacity(1.))
                        }
                    })
                    .child(
                        h_flex()
                            .group(&card_header_id)
                            .relative()
                            .w_full()
                            .map(|this| {
                                if tool_call.locations.len() == 1 {
                                    this.gap_0()
                                } else {
                                    this.gap_1p5()
                                }
                            })
                            .text_size(self.tool_name_font_size())
                            .child(self.render_tool_call_icon(
                                card_header_id,
                                entry_ix,
                                is_collapsible,
                                is_open,
                                tool_call,
                                cx,
                            ))
                            .child(if tool_call.locations.len() == 1 {
                                let name = tool_call.locations[0]
                                    .path
                                    .file_name()
                                    .unwrap_or_default()
                                    .display()
                                    .to_string();

                                h_flex()
                                    .id(("open-tool-call-location", entry_ix))
                                    .w_full()
                                    .max_w_full()
                                    .px_1p5()
                                    .rounded_sm()
                                    .overflow_x_scroll()
                                    .opacity(0.8)
                                    .hover(|label| {
                                        label.opacity(1.).bg(cx
                                            .theme()
                                            .colors()
                                            .element_hover
                                            .opacity(0.5))
                                    })
                                    .child(name)
                                    .tooltip(Tooltip::text("Jump to File"))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.open_tool_call_location(entry_ix, 0, window, cx);
                                    }))
                                    .into_any_element()
                            } else {
                                h_flex()
                                    .id("non-card-label-container")
                                    .w_full()
                                    .relative()
                                    .overflow_hidden()
                                    .child(
                                        h_flex()
                                            .id("non-card-label")
                                            .pr_8()
                                            .w_full()
                                            .overflow_x_scroll()
                                            .child(self.render_markdown(
                                                tool_call.label.clone(),
                                                default_markdown_style(
                                                    needs_confirmation,
                                                    window,
                                                    cx,
                                                ),
                                            )),
                                    )
                                    .child(gradient_overlay)
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
                                    }))
                                    .into_any()
                            }),
                    )
                    .children(status_icon),
            )
            .when(is_open, |this| {
                this.child(
                    v_flex()
                        .text_xs()
                        .when(is_collapsible, |this| {
                            this.mt_1()
                                .border_1()
                                .border_color(self.tool_card_border_color(cx))
                                .bg(cx.theme().colors().editor_background)
                                .rounded_lg()
                        })
                        .map(|this| {
                            if is_open {
                                match &tool_call.status {
                                    ToolCallStatus::WaitingForConfirmation { options, .. } => this
                                        .children(tool_call.content.iter().map(|content| {
                                            div()
                                                .py_1p5()
                                                .child(
                                                    self.render_tool_call_content(
                                                        content, window, cx,
                                                    ),
                                                )
                                                .into_any_element()
                                        }))
                                        .child(self.render_permission_buttons(
                                            options,
                                            entry_ix,
                                            tool_call.id.clone(),
                                            tool_call.content.is_empty(),
                                            cx,
                                        )),
                                    ToolCallStatus::Allowed { .. } | ToolCallStatus::Canceled => {
                                        this.children(tool_call.content.iter().map(|content| {
                                            div()
                                                .py_1p5()
                                                .child(
                                                    self.render_tool_call_content(
                                                        content, window, cx,
                                                    ),
                                                )
                                                .into_any_element()
                                        }))
                                    }
                                    ToolCallStatus::Rejected => this,
                                }
                            } else {
                                this
                            }
                        }),
                )
            })
    }

    fn render_tool_call_content(
        &self,
        content: &ToolCallContent,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match content {
            ToolCallContent::ContentBlock { content } => {
                if let Some(md) = content.markdown() {
                    div()
                        .p_2()
                        .child(
                            self.render_markdown(
                                md.clone(),
                                default_markdown_style(false, window, cx),
                            ),
                        )
                        .into_any_element()
                } else {
                    Empty.into_any_element()
                }
            }
            ToolCallContent::Diff { diff, .. } => {
                self.render_diff_editor(&diff.read(cx).multibuffer())
            }
        }
    }

    fn render_permission_buttons(
        &self,
        options: &[acp::PermissionOption],
        entry_ix: usize,
        tool_call_id: acp::ToolCallId,
        empty_content: bool,
        cx: &Context<Self>,
    ) -> Div {
        h_flex()
            .p_1p5()
            .gap_1()
            .justify_end()
            .when(!empty_content, |this| {
                this.border_t_1()
                    .border_color(self.tool_card_border_color(cx))
            })
            .children(options.iter().map(|option| {
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
                        move |this, _, _, cx| {
                            this.authorize_tool_call(
                                tool_call_id.clone(),
                                option_id.clone(),
                                option_kind,
                                cx,
                            );
                        }
                    }))
            }))
    }

    fn render_diff_editor(&self, multibuffer: &Entity<MultiBuffer>) -> AnyElement {
        v_flex()
            .h_full()
            .child(
                if let Some(editor) = self.diff_editors.get(&multibuffer.entity_id()) {
                    editor.clone().into_any_element()
                } else {
                    Empty.into_any()
                },
            )
            .into_any()
    }

    fn render_agent_logo(&self) -> AnyElement {
        Icon::new(self.agent.logo())
            .color(Color::Muted)
            .size(IconSize::XLarge)
            .into_any_element()
    }

    fn render_error_agent_logo(&self) -> AnyElement {
        let logo = Icon::new(self.agent.logo())
            .color(Color::Muted)
            .size(IconSize::XLarge)
            .into_any_element();

        h_flex()
            .relative()
            .justify_center()
            .child(div().opacity(0.3).child(logo))
            .child(
                h_flex().absolute().right_1().bottom_0().child(
                    Icon::new(IconName::XCircle)
                        .color(Color::Error)
                        .size(IconSize::Small),
                ),
            )
            .into_any_element()
    }

    fn render_empty_state(&self, cx: &App) -> AnyElement {
        let loading = matches!(&self.thread_state, ThreadState::Loading { .. });

        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(if loading {
                h_flex()
                    .justify_center()
                    .child(self.render_agent_logo())
                    .with_animation(
                        "pulsating_icon",
                        Animation::new(Duration::from_secs(2))
                            .repeat()
                            .with_easing(pulsating_between(0.4, 1.0)),
                        |icon, delta| icon.opacity(delta),
                    )
                    .into_any()
            } else {
                self.render_agent_logo().into_any_element()
            })
            .child(h_flex().mt_4().mb_1().justify_center().child(if loading {
                div()
                    .child(LoadingLabel::new("").size(LabelSize::Large))
                    .into_any_element()
            } else {
                Headline::new(self.agent.empty_state_headline())
                    .size(HeadlineSize::Medium)
                    .into_any_element()
            }))
            .child(
                div()
                    .max_w_1_2()
                    .text_sm()
                    .text_center()
                    .map(|this| {
                        if loading {
                            this.invisible()
                        } else {
                            this.text_color(cx.theme().colors().text_muted)
                        }
                    })
                    .child(self.agent.empty_state_message()),
            )
            .into_any()
    }

    fn render_pending_auth_state(&self) -> AnyElement {
        v_flex()
            .items_center()
            .justify_center()
            .child(self.render_error_agent_logo())
            .child(
                h_flex()
                    .mt_4()
                    .mb_1()
                    .justify_center()
                    .child(Headline::new("Not Authenticated").size(HeadlineSize::Medium)),
            )
            .into_any()
    }

    fn render_server_exited(&self, status: ExitStatus, _cx: &Context<Self>) -> AnyElement {
        v_flex()
            .items_center()
            .justify_center()
            .child(self.render_error_agent_logo())
            .child(
                v_flex()
                    .mt_4()
                    .mb_2()
                    .gap_0p5()
                    .text_center()
                    .items_center()
                    .child(Headline::new("Server exited unexpectedly").size(HeadlineSize::Medium))
                    .child(
                        Label::new(format!("Exit status: {}", status.code().unwrap_or(-127)))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .into_any_element()
    }

    fn render_load_error(&self, e: &LoadError, cx: &Context<Self>) -> AnyElement {
        let mut container = v_flex()
            .items_center()
            .justify_center()
            .child(self.render_error_agent_logo())
            .child(
                v_flex()
                    .mt_4()
                    .mb_2()
                    .gap_0p5()
                    .text_center()
                    .items_center()
                    .child(Headline::new("Failed to launch").size(HeadlineSize::Medium))
                    .child(
                        Label::new(e.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            );

        if let LoadError::Unsupported {
            upgrade_message,
            upgrade_command,
            ..
        } = &e
        {
            let upgrade_message = upgrade_message.clone();
            let upgrade_command = upgrade_command.clone();
            container = container.child(Button::new("upgrade", upgrade_message).on_click(
                cx.listener(move |this, _, window, cx| {
                    this.workspace
                        .update(cx, |workspace, cx| {
                            let project = workspace.project().read(cx);
                            let cwd = project.first_project_directory(cx);
                            let shell = project.terminal_settings(&cwd, cx).shell.clone();
                            let spawn_in_terminal = task::SpawnInTerminal {
                                id: task::TaskId("install".to_string()),
                                full_label: upgrade_command.clone(),
                                label: upgrade_command.clone(),
                                command: Some(upgrade_command.clone()),
                                args: Vec::new(),
                                command_label: upgrade_command.clone(),
                                cwd,
                                env: Default::default(),
                                use_new_terminal: true,
                                allow_concurrent_runs: true,
                                reveal: Default::default(),
                                reveal_target: Default::default(),
                                hide: Default::default(),
                                shell,
                                show_summary: true,
                                show_command: true,
                                show_rerun: false,
                            };
                            workspace
                                .spawn_in_terminal(spawn_in_terminal, window, cx)
                                .detach();
                        })
                        .ok();
                }),
            ));
        }

        container.into_any()
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

        let pending_edits = thread.has_pending_edit_tool_calls();

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
            .when(!changed_buffers.is_empty(), |this| {
                this.child(Divider::horizontal().color(DividerColor::Border))
                    .child(self.render_edits_summary(
                        action_log,
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
        action_log: &Entity<ActionLog>,
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
            .when(expanded, |this| {
                this.border_b_1().border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .id("edits-container")
                    .w_full()
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
                            .on_click({
                                let action_log = action_log.clone();
                                cx.listener(move |_, _, _, cx| {
                                    action_log.update(cx, |action_log, cx| {
                                        action_log.reject_all_edits(cx).detach();
                                    })
                                })
                            }),
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
                            .on_click({
                                let action_log = action_log.clone();
                                cx.listener(move |_, _, _, cx| {
                                    action_log.update(cx, |action_log, cx| {
                                        action_log.keep_all_edits(cx);
                                    })
                                })
                            }),
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

        v_flex().children(changed_buffers.into_iter().enumerate().flat_map(
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

                let file_icon = FileIcons::get_icon(&path, cx)
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
                    .relative()
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
                            .id(("file-name", index))
                            .pr_8()
                            .gap_1p5()
                            .max_w_full()
                            .overflow_x_scroll()
                            .child(file_icon)
                            .child(h_flex().gap_0p5().children(file_name).children(file_path))
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
                    )
                    .child(
                        div()
                            .id("gradient-overlay")
                            .absolute()
                            .h_full()
                            .w_12()
                            .top_0()
                            .bottom_0()
                            .right(px(152.))
                            .bg(overlay_gradient),
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
                    .child(div().flex_1().child({
                        let settings = ThemeSettings::get_global(cx);
                        let font_size = TextSize::Small
                            .rems(cx)
                            .to_pixels(settings.agent_font_size(cx));
                        let line_height = settings.buffer_line_height.value() * font_size;

                        let text_style = TextStyle {
                            color: cx.theme().colors().text,
                            font_family: settings.buffer_font.family.clone(),
                            font_fallbacks: settings.buffer_font.fallbacks.clone(),
                            font_features: settings.buffer_font.features.clone(),
                            font_size: font_size.into(),
                            line_height: line_height.into(),
                            ..Default::default()
                        };

                        EditorElement::new(
                            &self.message_editor,
                            EditorStyle {
                                background: editor_bg_color,
                                local_player: cx.theme().players().local(),
                                text: text_style,
                                syntax: cx.theme().syntax().clone(),
                                ..Default::default()
                            },
                        )
                    }))
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
                                        let focus_handle = focus_handle.clone();
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
                    .justify_between()
                    .child(self.render_follow_toggle(cx))
                    .child(self.render_send_button(cx)),
            )
            .into_any()
    }

    fn render_send_button(&self, cx: &mut Context<Self>) -> AnyElement {
        if self.thread().map_or(true, |thread| {
            thread.read(cx).status() == ThreadStatus::Idle
        }) {
            let is_editor_empty = self.message_editor.read(cx).is_empty(cx);
            IconButton::new("send-message", IconName::Send)
                .icon_color(Color::Accent)
                .style(ButtonStyle::Filled)
                .disabled(self.thread().is_none() || is_editor_empty)
                .when(!is_editor_empty, |button| {
                    button.tooltip(move |window, cx| Tooltip::for_action("Send", &Chat, window, cx))
                })
                .when(is_editor_empty, |button| {
                    button.tooltip(Tooltip::text("Type a message to submit"))
                })
                .on_click(cx.listener(|this, _, window, cx| {
                    this.chat(&Chat, window, cx);
                }))
                .into_any_element()
        } else {
            IconButton::new("stop-generation", IconName::Stop)
                .icon_color(Color::Error)
                .style(ButtonStyle::Tinted(ui::TintColor::Error))
                .tooltip(move |window, cx| {
                    Tooltip::for_action("Stop Generation", &editor::actions::Cancel, window, cx)
                })
                .on_click(cx.listener(|this, _event, _, cx| this.cancel(cx)))
                .into_any_element()
        }
    }

    fn render_follow_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let following = self
            .workspace
            .read_with(cx, |workspace, _| {
                workspace.is_being_followed(CollaboratorId::Agent)
            })
            .unwrap_or(false);

        IconButton::new("follow-agent", IconName::Crosshair)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .toggle_state(following)
            .selected_icon_color(Some(Color::Custom(cx.theme().players().agent().cursor)))
            .tooltip(move |window, cx| {
                if following {
                    Tooltip::for_action("Stop Following Agent", &Follow, window, cx)
                } else {
                    Tooltip::with_meta(
                        "Follow Agent",
                        Some(&Follow),
                        "Track the agent's location as it reads and edits files.",
                        window,
                        cx,
                    )
                }
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.workspace
                    .update(cx, |workspace, cx| {
                        if following {
                            workspace.unfollow(CollaboratorId::Agent, window, cx);
                        } else {
                            workspace.follow(CollaboratorId::Agent, window, cx);
                        }
                    })
                    .ok();
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

        if let Some(mention_path) = MentionPath::try_parse(&url) {
            workspace.update(cx, |workspace, cx| {
                let project = workspace.project();
                let Some((path, entry)) = project.update(cx, |project, cx| {
                    let path = project.find_project_path(mention_path.path(), cx)?;
                    let entry = project.entry_for_path(&path, cx)?;
                    Some((path, entry))
                }) else {
                    return;
                };

                if entry.is_dir() {
                    project.update(cx, |_, cx| {
                        cx.emit(project::Event::RevealInProjectPanel(entry.id));
                    });
                } else {
                    workspace
                        .open_path(path, None, true, window, cx)
                        .detach_and_log_err(cx);
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
        let location = self
            .thread()?
            .read(cx)
            .entries()
            .get(entry_ix)?
            .locations()?
            .get(location_ix)?;

        let project_path = self
            .project
            .read(cx)
            .find_project_path(&location.path, cx)?;

        let open_task = self
            .workspace
            .update(cx, |worskpace, cx| {
                worskpace.open_path(project_path, None, true, window, cx)
            })
            .log_err()?;

        window
            .spawn(cx, async move |cx| {
                let item = open_task.await?;

                let Some(active_editor) = item.downcast::<Editor>() else {
                    return anyhow::Ok(());
                };

                active_editor.update_in(cx, |editor, window, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let first_hunk = editor
                        .diff_hunks_in_ranges(
                            &[editor::Anchor::min()..editor::Anchor::max()],
                            &snapshot,
                        )
                        .next();
                    if let Some(first_hunk) = first_hunk {
                        let first_hunk_start = first_hunk.multi_buffer_range().start;
                        editor.change_selections(Default::default(), window, cx, |selections| {
                            selections.select_anchor_ranges([first_hunk_start..first_hunk_start]);
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
    ) -> Task<anyhow::Result<()>> {
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
                    anyhow::bail!("failed to open active thread as markdown in remote project");
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

        let title = self.title(cx);

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
        {
            if let Some(pop_up) = screen_window.entity(cx).log_err() {
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
                            if window.is_window_active() {
                                if let Some(pop_up) = pop_up_weak.upgrade() {
                                    pop_up.update(cx, |_, cx| {
                                        cx.emit(AgentNotificationEvent::Dismissed);
                                    });
                                }
                            }
                        })
                    });
            }
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

    fn render_thread_controls(&self, cx: &Context<Self>) -> impl IntoElement {
        let open_as_markdown = IconButton::new("open-as-markdown", IconName::FileMarkdown)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Open Thread as Markdown"))
            .on_click(cx.listener(move |this, _, window, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    this.open_thread_as_markdown(workspace, window, cx)
                        .detach_and_log_err(cx);
                }
            }));

        let scroll_to_top = IconButton::new("scroll_to_top", IconName::ArrowUp)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Scroll To Top"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.scroll_to_top(cx);
            }));

        h_flex()
            .w_full()
            .mr_1()
            .pb_2()
            .gap_1()
            .px(RESPONSE_PADDING_X)
            .opacity(0.4)
            .hover(|style| style.opacity(1.))
            .flex_wrap()
            .justify_end()
            .child(open_as_markdown)
            .child(scroll_to_top)
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

    fn settings_changed(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        for diff_editor in self.diff_editors.values() {
            diff_editor.update(cx, |diff_editor, cx| {
                diff_editor.set_text_style_refinement(diff_editor_text_style_refinement(cx));
                cx.notify();
            })
        }
    }

    pub(crate) fn insert_dragged_files(
        &self,
        paths: Vec<project::ProjectPath>,
        _added_worktrees: Vec<Entity<project::Worktree>>,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) {
        let buffer = self.message_editor.read(cx).buffer().clone();
        let Some((&excerpt_id, _, _)) = buffer.read(cx).snapshot(cx).as_singleton() else {
            return;
        };
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        for path in paths {
            let Some(entry) = self.project.read(cx).entry_for_path(&path, cx) else {
                continue;
            };
            let Some(abs_path) = self.project.read(cx).absolute_path(&path, cx) else {
                continue;
            };

            let anchor = buffer.update(cx, |buffer, _cx| buffer.anchor_before(buffer.len()));
            let path_prefix = abs_path
                .file_name()
                .unwrap_or(path.path.as_os_str())
                .display()
                .to_string();
            let completion = ContextPickerCompletionProvider::completion_for_path(
                path,
                &path_prefix,
                false,
                entry.is_dir(),
                excerpt_id,
                anchor..anchor,
                self.message_editor.clone(),
                self.mention_set.clone(),
                cx,
            );

            self.message_editor.update(cx, |message_editor, cx| {
                message_editor.edit(
                    [(
                        multi_buffer::Anchor::max()..multi_buffer::Anchor::max(),
                        completion.new_text,
                    )],
                    cx,
                );
            });
            if let Some(confirm) = completion.confirm.clone() {
                confirm(CompletionIntent::Complete, window, cx);
            }
        }
    }
}

impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .key_context("AcpThread")
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::previous_history_message))
            .on_action(cx.listener(Self::next_history_message))
            .on_action(cx.listener(Self::open_agent_diff))
            .bg(cx.theme().colors().panel_background)
            .child(match &self.thread_state {
                ThreadState::Unauthenticated { connection } => v_flex()
                    .p_2()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(self.render_pending_auth_state())
                    .child(h_flex().mt_1p5().justify_center().children(
                        connection.auth_methods().into_iter().map(|method| {
                            Button::new(
                                SharedString::from(method.id.0.clone()),
                                method.name.clone(),
                            )
                            .on_click({
                                let method_id = method.id.clone();
                                cx.listener(move |this, _, window, cx| {
                                    this.authenticate(method_id.clone(), window, cx)
                                })
                            })
                        }),
                    )),
                ThreadState::Loading { .. } => v_flex().flex_1().child(self.render_empty_state(cx)),
                ThreadState::LoadError(e) => v_flex()
                    .p_2()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(self.render_load_error(e, cx)),
                ThreadState::ServerExited { status } => v_flex()
                    .p_2()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(self.render_server_exited(*status, cx)),
                ThreadState::Ready { thread, .. } => {
                    let thread_clone = thread.clone();

                    v_flex().flex_1().map(|this| {
                        if self.list_state.item_count() > 0 {
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
                            .children(match thread_clone.read(cx).status() {
                                ThreadStatus::Idle | ThreadStatus::WaitingForToolConfirmation => {
                                    None
                                }
                                ThreadStatus::Generating => div()
                                    .px_5()
                                    .py_2()
                                    .child(LoadingLabel::new("").size(LabelSize::Small))
                                    .into(),
                            })
                            .children(self.render_activity_bar(&thread_clone, window, cx))
                        } else {
                            this.child(self.render_empty_state(cx))
                        }
                    })
                }
            })
            .when_some(self.last_error.clone(), |el, error| {
                el.child(
                    div()
                        .p_2()
                        .text_xs()
                        .border_t_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().status().error_background)
                        .child(
                            self.render_markdown(error, default_markdown_style(false, window, cx)),
                        ),
                )
            })
            .child(self.render_message_editor(window, cx))
    }
}

fn user_message_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let mut style = default_markdown_style(false, window, cx);
    let mut text_style = window.text_style();
    let theme_settings = ThemeSettings::get_global(cx);

    let buffer_font = theme_settings.buffer_font.family.clone();
    let buffer_font_size = TextSize::Small.rems(cx);

    text_style.refine(&TextStyleRefinement {
        font_family: Some(buffer_font),
        font_size: Some(buffer_font_size.into()),
        ..Default::default()
    });

    style.base_text_style = text_style;
    style.link_callback = Some(Rc::new(move |url, cx| {
        if MentionPath::try_parse(url).is_some() {
            let colors = cx.theme().colors();
            Some(TextStyleRefinement {
                background_color: Some(colors.element_background),
                ..Default::default()
            })
        } else {
            None
        }
    }));
    style
}

fn default_markdown_style(buffer_font: bool, window: &Window, cx: &App) -> MarkdownStyle {
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

    text_style.refine(&TextStyleRefinement {
        font_family: Some(font_family),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(font_size.into()),
        line_height: Some(line_height.into()),
        color: Some(cx.theme().colors().text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: cx.theme().colors().element_selection_background,
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
    let default_md_style = default_markdown_style(false, window, cx);

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

fn diff_editor_text_style_refinement(cx: &mut App) -> TextStyleRefinement {
    TextStyleRefinement {
        font_size: Some(
            TextSize::Small
                .rems(cx)
                .to_pixels(ThemeSettings::get_global(cx).agent_font_size(cx))
                .into(),
        ),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use agent_client_protocol::SessionId;
    use editor::EditorSettings;
    use fs::FakeFs;
    use futures::future::try_join_all;
    use gpui::{SemanticVersion, TestAppContext, VisualTestContext};
    use lsp::{CompletionContext, CompletionTriggerKind};
    use project::CompletionIntent;
    use rand::Rng;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use super::*;

    #[gpui::test]
    async fn test_drop(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, _cx) = setup_thread_view(StubAgentServer::default(), cx).await;
        let weak_view = thread_view.downgrade();
        drop(thread_view);
        assert!(!weak_view.is_upgradable());
    }

    #[gpui::test]
    async fn test_notification_for_stop_event(cx: &mut TestAppContext) {
        init_test(cx);

        let (thread_view, cx) = setup_thread_view(StubAgentServer::default(), cx).await;

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.chat(&Chat, window, cx);
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
            thread_view.chat(&Chat, window, cx);
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
        let connection = StubAgentConnection::new(vec![acp::SessionUpdate::ToolCall(tool_call)])
            .with_permission_requests(HashMap::from_iter([(
                tool_call_id,
                vec![acp::PermissionOption {
                    id: acp::PermissionOptionId("1".into()),
                    name: "Allow".into(),
                    kind: acp::PermissionOptionKind::AllowOnce,
                }],
            )]));
        let (thread_view, cx) = setup_thread_view(StubAgentServer::new(connection), cx).await;

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello", window, cx);
        });

        cx.deactivate_window();

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.chat(&Chat, window, cx);
        });

        cx.run_until_parked();

        assert!(
            cx.windows()
                .iter()
                .any(|window| window.downcast::<AgentNotification>().is_some())
        );
    }

    #[gpui::test]
    async fn test_crease_removal(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;
        let agent = StubAgentServer::default();
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpThreadView::new(
                    Rc::new(agent),
                    workspace.downgrade(),
                    project,
                    Rc::new(RefCell::new(MessageHistory::default())),
                    1,
                    None,
                    window,
                    cx,
                )
            })
        });

        cx.run_until_parked();

        let message_editor = cx.read(|cx| thread_view.read(cx).message_editor.clone());
        let excerpt_id = message_editor.update(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .excerpt_ids()
                .into_iter()
                .next()
                .unwrap()
        });
        let completions = message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello @", window, cx);
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            let completion_provider = editor.completion_provider().unwrap();
            completion_provider.completions(
                excerpt_id,
                &buffer,
                Anchor::MAX,
                CompletionContext {
                    trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
                    trigger_character: Some("@".into()),
                },
                window,
                cx,
            )
        });
        let [_, completion]: [_; 2] = completions
            .await
            .unwrap()
            .into_iter()
            .flat_map(|response| response.completions)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        message_editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let start = snapshot
                .anchor_in_excerpt(excerpt_id, completion.replace_range.start)
                .unwrap();
            let end = snapshot
                .anchor_in_excerpt(excerpt_id, completion.replace_range.end)
                .unwrap();
            editor.edit([(start..end, completion.new_text)], cx);
            (completion.confirm.unwrap())(CompletionIntent::Complete, window, cx);
        });

        cx.run_until_parked();

        // Backspace over the inserted crease (and the following space).
        message_editor.update_in(cx, |editor, window, cx| {
            editor.backspace(&Default::default(), window, cx);
            editor.backspace(&Default::default(), window, cx);
        });

        thread_view.update_in(cx, |thread_view, window, cx| {
            thread_view.chat(&Chat, window, cx);
        });

        cx.run_until_parked();

        let content = thread_view.update_in(cx, |thread_view, _window, _cx| {
            thread_view
                .message_history
                .borrow()
                .items()
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>()
        });

        // We don't send a resource link for the deleted crease.
        pretty_assertions::assert_matches!(content.as_slice(), [acp::ContentBlock::Text { .. }]);
    }

    async fn setup_thread_view(
        agent: impl AgentServer + 'static,
        cx: &mut TestAppContext,
    ) -> (Entity<AcpThreadView>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_view = cx.update(|window, cx| {
            cx.new(|cx| {
                AcpThreadView::new(
                    Rc::new(agent),
                    workspace.downgrade(),
                    project,
                    Rc::new(RefCell::new(MessageHistory::default())),
                    1,
                    None,
                    window,
                    cx,
                )
            })
        });
        cx.run_until_parked();
        (thread_view, cx)
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
        fn default() -> Self {
            Self::new(StubAgentConnection::default())
        }
    }

    impl<C> AgentServer for StubAgentServer<C>
    where
        C: 'static + AgentConnection + Send + Clone,
    {
        fn logo(&self) -> ui::IconName {
            unimplemented!()
        }

        fn name(&self) -> &'static str {
            unimplemented!()
        }

        fn empty_state_headline(&self) -> &'static str {
            unimplemented!()
        }

        fn empty_state_message(&self) -> &'static str {
            unimplemented!()
        }

        fn connect(
            &self,
            _root_dir: &Path,
            _project: &Entity<Project>,
            _cx: &mut App,
        ) -> Task<gpui::Result<Rc<dyn AgentConnection>>> {
            Task::ready(Ok(Rc::new(self.connection.clone())))
        }
    }

    #[derive(Clone, Default)]
    struct StubAgentConnection {
        sessions: Arc<Mutex<HashMap<acp::SessionId, WeakEntity<AcpThread>>>>,
        permission_requests: HashMap<acp::ToolCallId, Vec<acp::PermissionOption>>,
        updates: Vec<acp::SessionUpdate>,
    }

    impl StubAgentConnection {
        fn new(updates: Vec<acp::SessionUpdate>) -> Self {
            Self {
                updates,
                permission_requests: HashMap::default(),
                sessions: Arc::default(),
            }
        }

        fn with_permission_requests(
            mut self,
            permission_requests: HashMap<acp::ToolCallId, Vec<acp::PermissionOption>>,
        ) -> Self {
            self.permission_requests = permission_requests;
            self
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
            cx: &mut gpui::AsyncApp,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            let session_id = SessionId(
                rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect::<String>()
                    .into(),
            );
            let thread = cx
                .new(|cx| AcpThread::new("Test", self.clone(), project, session_id.clone(), cx))
                .unwrap();
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
            params: acp::PromptRequest,
            cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            let sessions = self.sessions.lock();
            let thread = sessions.get(&params.session_id).unwrap();
            let mut tasks = vec![];
            for update in &self.updates {
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
    }

    #[derive(Clone)]
    struct SaboteurAgentConnection;

    impl AgentConnection for SaboteurAgentConnection {
        fn new_thread(
            self: Rc<Self>,
            project: Entity<Project>,
            _cwd: &Path,
            cx: &mut gpui::AsyncApp,
        ) -> Task<gpui::Result<Entity<AcpThread>>> {
            Task::ready(Ok(cx
                .new(|cx| {
                    AcpThread::new(
                        "SaboteurAgentConnection",
                        self,
                        project,
                        SessionId("test".into()),
                        cx,
                    )
                })
                .unwrap()))
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
            _params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<gpui::Result<acp::PromptResponse>> {
            Task::ready(Err(anyhow::anyhow!("Error prompting")))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {
            unimplemented!()
        }
    }

    fn init_test(cx: &mut TestAppContext) {
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
        });
    }
}
