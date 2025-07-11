use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use agentic_coding_protocol::{self as acp};
use assistant_tool::ActionLog;
use buffer_diff::BufferDiff;
use collections::{HashMap, HashSet};
use editor::{
    AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorMode,
    EditorStyle, MinimapVisibility, MultiBuffer, PathKey,
};
use file_icons::FileIcons;
use futures::channel::oneshot;
use gpui::{
    Action, Animation, AnimationExt, App, BorderStyle, EdgesRefinement, Empty, Entity, EntityId,
    FocusHandle, Focusable, Hsla, Length, ListOffset, ListState, SharedString, StyleRefinement,
    Subscription, Task, TextStyle, TextStyleRefinement, Transformation, UnderlineStyle, WeakEntity,
    Window, div, linear_color_stop, linear_gradient, list, percentage, point, prelude::*,
    pulsating_between,
};
use language::language_settings::SoftWrap;
use language::{Buffer, Language};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use parking_lot::Mutex;
use project::Project;
use settings::Settings as _;
use text::Anchor;
use theme::ThemeSettings;
use ui::{Disclosure, Divider, DividerColor, KeyBinding, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::{Chat, NextHistoryMessage, PreviousHistoryMessage};

use ::acp::{
    AcpThread, AcpThreadEvent, AgentThreadEntry, AssistantMessage, AssistantMessageChunk, Diff,
    LoadError, MentionPath, ThreadStatus, ToolCall, ToolCallConfirmation, ToolCallContent,
    ToolCallId, ToolCallStatus,
};

use crate::acp::completion_provider::{ContextPickerCompletionProvider, MentionSet};
use crate::acp::message_history::MessageHistory;
use crate::agent_diff::AgentDiff;
use crate::{AgentDiffPane, Follow, KeepAll, OpenAgentDiff, RejectAll};

const RESPONSE_PADDING_X: Pixels = px(19.);

pub struct AcpThreadView {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    thread_state: ThreadState,
    diff_editors: HashMap<EntityId, Entity<Editor>>,
    message_editor: Entity<Editor>,
    message_set_from_history: bool,
    _message_editor_subscription: Subscription,
    mention_set: Arc<Mutex<MentionSet>>,
    last_error: Option<Entity<Markdown>>,
    list_state: ListState,
    auth_task: Option<Task<()>>,
    expanded_tool_calls: HashSet<ToolCallId>,
    expanded_thinking_blocks: HashSet<(usize, usize)>,
    edits_expanded: bool,
    message_history: Rc<RefCell<MessageHistory<acp::SendUserMessageParams>>>,
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
        thread: Entity<AcpThread>,
    },
}

impl AcpThreadView {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        message_history: Rc<RefCell<MessageHistory<acp::SendUserMessageParams>>>,
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
                    min_lines: 4,
                    max_lines: None,
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

        let message_editor_subscription = cx.subscribe(&message_editor, |this, _, event, _| {
            if let editor::EditorEvent::BufferEdited = &event {
                if !this.message_set_from_history {
                    this.message_history.borrow_mut().reset_position();
                }
                this.message_set_from_history = false;
            }
        });

        let mention_set = mention_set.clone();

        let list_state = ListState::new(
            0,
            gpui::ListAlignment::Bottom,
            px(2048.0),
            cx.processor({
                move |this: &mut Self, index: usize, window, cx| {
                    let Some((entry, len)) = this.thread().and_then(|thread| {
                        let entries = &thread.read(cx).entries();
                        Some((entries.get(index)?, entries.len()))
                    }) else {
                        return Empty.into_any();
                    };
                    this.render_entry(index, len, entry, window, cx)
                }
            }),
        );

        Self {
            workspace: workspace.clone(),
            project: project.clone(),
            thread_state: Self::initial_state(workspace, project, window, cx),
            message_editor,
            message_set_from_history: false,
            _message_editor_subscription: message_editor_subscription,
            mention_set,
            diff_editors: Default::default(),
            list_state: list_state,
            last_error: None,
            auth_task: None,
            expanded_tool_calls: HashSet::default(),
            expanded_thinking_blocks: HashSet::default(),
            edits_expanded: false,
            message_history,
        }
    }

    fn initial_state(
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

        let load_task = cx.spawn_in(window, async move |this, cx| {
            let thread = match AcpThread::spawn(agent_servers::Gemini, &root_dir, project, cx).await
            {
                Ok(thread) => thread,
                Err(err) => {
                    this.update(cx, |this, cx| {
                        this.handle_load_error(err, cx);
                        cx.notify();
                    })
                    .log_err();
                    return;
                }
            };

            let init_response = async {
                let resp = thread
                    .read_with(cx, |thread, _cx| thread.initialize())?
                    .await?;
                anyhow::Ok(resp)
            };

            let result = match init_response.await {
                Err(e) => {
                    let mut cx = cx.clone();
                    if e.downcast_ref::<oneshot::Canceled>().is_some() {
                        let child_status = thread
                            .update(&mut cx, |thread, _| thread.child_status())
                            .ok()
                            .flatten();
                        if let Some(child_status) = child_status {
                            match child_status.await {
                                Ok(_) => Err(e),
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(e)
                        }
                    } else {
                        Err(e)
                    }
                }
                Ok(response) => {
                    if !response.is_authenticated {
                        this.update(cx, |this, _| {
                            this.thread_state = ThreadState::Unauthenticated { thread };
                        })
                        .ok();
                        return;
                    };
                    Ok(())
                }
            };

            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(()) => {
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
            ThreadState::Ready { thread, .. } | ThreadState::Unauthenticated { thread } => {
                Some(thread)
            }
            ThreadState::Loading { .. } | ThreadState::LoadError(..) => None,
        }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => thread.read(cx).title(),
            ThreadState::Loading { .. } => "Loading…".into(),
            ThreadState::LoadError(_) => "Failed to load".into(),
            ThreadState::Unauthenticated { .. } => "Not authenticated".into(),
        }
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        self.last_error.take();

        if let Some(thread) = self.thread() {
            thread.update(cx, |thread, cx| thread.cancel(cx)).detach();
        }
    }

    fn chat(&mut self, _: &Chat, window: &mut Window, cx: &mut Context<Self>) {
        self.last_error.take();

        let mut ix = 0;
        let mut chunks: Vec<acp::UserMessageChunk> = Vec::new();
        let project = self.project.clone();
        self.message_editor.update(cx, |editor, cx| {
            let text = editor.text(cx);
            editor.display_map.update(cx, |map, cx| {
                let snapshot = map.snapshot(cx);
                for (crease_id, crease) in snapshot.crease_snapshot.creases() {
                    if let Some(project_path) =
                        self.mention_set.lock().path_for_crease_id(crease_id)
                    {
                        let crease_range = crease.range().to_offset(&snapshot.buffer_snapshot);
                        if crease_range.start > ix {
                            chunks.push(acp::UserMessageChunk::Text {
                                text: text[ix..crease_range.start].to_string(),
                            });
                        }
                        if let Some(abs_path) = project.read(cx).absolute_path(&project_path, cx) {
                            chunks.push(acp::UserMessageChunk::Path { path: abs_path });
                        }
                        ix = crease_range.end;
                    }
                }

                if ix < text.len() {
                    let last_chunk = text[ix..].trim();
                    if !last_chunk.is_empty() {
                        chunks.push(acp::UserMessageChunk::Text {
                            text: last_chunk.into(),
                        });
                    }
                }
            })
        });

        if chunks.is_empty() {
            return;
        }

        let Some(thread) = self.thread() else { return };
        let message = acp::SendUserMessageParams { chunks };
        let task = thread.update(cx, |thread, cx| thread.send(message.clone(), cx));

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

        self.message_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
            editor.remove_creases(mention_set.lock().drain(), cx)
        });

        self.message_history.borrow_mut().push(message);
    }

    fn previous_history_message(
        &mut self,
        _: &PreviousHistoryMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.message_set_from_history = Self::set_draft_message(
            self.message_editor.clone(),
            self.mention_set.clone(),
            self.project.clone(),
            self.message_history.borrow_mut().prev(),
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
        self.message_set_from_history = Self::set_draft_message(
            self.message_editor.clone(),
            self.mention_set.clone(),
            self.project.clone(),
            self.message_history.borrow_mut().next(),
            window,
            cx,
        );
    }

    fn set_draft_message(
        message_editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
        project: Entity<Project>,
        message: Option<&acp::SendUserMessageParams>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        cx.notify();

        let Some(message) = message else {
            return false;
        };

        let mut text = String::new();
        let mut mentions = Vec::new();

        for chunk in &message.chunks {
            match chunk {
                acp::UserMessageChunk::Text { text: chunk } => {
                    text.push_str(&chunk);
                }
                acp::UserMessageChunk::Path { path } => {
                    let start = text.len();
                    let content = MentionPath::new(path).to_string();
                    text.push_str(&content);
                    let end = text.len();
                    if let Some(project_path) =
                        project.read(cx).project_path_for_absolute_path(path, cx)
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

        true
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
        }
        cx.notify();
    }

    fn sync_thread_entry_view(
        &mut self,
        entry_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(multibuffer) = self.entry_diff_multibuffer(entry_ix, cx) else {
            return;
        };

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
            editor.set_text_style_refinement(TextStyleRefinement {
                font_size: Some(
                    TextSize::Small
                        .rems(cx)
                        .to_pixels(ThemeSettings::get_global(cx).agent_font_size(cx))
                        .into(),
                ),
                ..Default::default()
            });
            editor
        });
        let entity_id = multibuffer.entity_id();
        cx.observe_release(&multibuffer, move |this, _, _| {
            this.diff_editors.remove(&entity_id);
        })
        .detach();

        self.diff_editors.insert(entity_id, editor);
    }

    fn entry_diff_multibuffer(&self, entry_ix: usize, cx: &App) -> Option<Entity<MultiBuffer>> {
        let entry = self.thread()?.read(cx).entries().get(entry_ix)?;
        entry.diff().map(|diff| diff.multibuffer.clone())
    }

    fn authenticate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.thread().cloned() else {
            return;
        };

        self.last_error.take();
        let authenticate = thread.read(cx).authenticate();
        self.auth_task = Some(cx.spawn_in(window, {
            let project = self.project.clone();
            async move |this, cx| {
                let result = authenticate.await;

                this.update_in(cx, |this, window, cx| {
                    if let Err(err) = result {
                        this.last_error = Some(cx.new(|cx| {
                            Markdown::new(format!("Error: {err}").into(), None, None, cx)
                        }))
                    } else {
                        this.thread_state =
                            Self::initial_state(this.workspace.clone(), project.clone(), window, cx)
                    }
                    this.auth_task.take()
                })
                .ok();
            }
        }));
    }

    fn authorize_tool_call(
        &mut self,
        id: ToolCallId,
        outcome: acp::ToolCallConfirmationOutcome,
        cx: &mut Context<Self>,
    ) {
        let Some(thread) = self.thread() else {
            return;
        };
        thread.update(cx, |thread, cx| {
            thread.authorize_tool_call(id, outcome, cx);
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
        match &entry {
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
                        .child(self.render_markdown(
                            message.content.clone(),
                            user_message_markdown_style(window, cx),
                        )),
                )
                .into_any(),
            AgentThreadEntry::AssistantMessage(AssistantMessage { chunks }) => {
                let style = default_markdown_style(false, window, cx);
                let message_body = v_flex()
                    .w_full()
                    .gap_2p5()
                    .children(chunks.iter().enumerate().map(|(chunk_ix, chunk)| {
                        match chunk {
                            AssistantMessageChunk::Text { chunk } => self
                                .render_markdown(chunk.clone(), style.clone())
                                .into_any_element(),
                            AssistantMessageChunk::Thought { chunk } => self.render_thinking_block(
                                index,
                                chunk_ix,
                                chunk.clone(),
                                window,
                                cx,
                            ),
                        }
                    }))
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
                .py_1p5()
                .px_5()
                .child(self.render_tool_call(index, tool_call, window, cx))
                .into_any(),
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
        let key = (entry_ix, chunk_ix);
        let is_open = self.expanded_thinking_blocks.contains(&key);

        v_flex()
            .child(
                h_flex()
                    .id(header_id)
                    .group("disclosure-header")
                    .w_full()
                    .justify_between()
                    .opacity(0.8)
                    .hover(|style| style.opacity(1.))
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                Icon::new(IconName::ToolBulb)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .text_size(self.tool_name_font_size())
                                    .child("Thinking"),
                            ),
                    )
                    .child(
                        div().visible_on_hover("disclosure-header").child(
                            Disclosure::new("thinking-disclosure", is_open)
                                .opened_icon(IconName::ChevronUp)
                                .closed_icon(IconName::ChevronDown)
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

    fn render_tool_call(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let header_id = SharedString::from(format!("tool-call-header-{}", entry_ix));

        let status_icon = match &tool_call.status {
            ToolCallStatus::WaitingForConfirmation { .. } => None,
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Running,
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
                status: acp::ToolCallStatus::Finished,
                ..
            } => None,
            ToolCallStatus::Rejected
            | ToolCallStatus::Canceled
            | ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Error,
                ..
            } => Some(
                Icon::new(IconName::X)
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

        let is_collapsible = tool_call.content.is_some() && !needs_confirmation;
        let is_open = !is_collapsible || self.expanded_tool_calls.contains(&tool_call.id);

        let content = if is_open {
            match &tool_call.status {
                ToolCallStatus::WaitingForConfirmation { confirmation, .. } => {
                    Some(self.render_tool_call_confirmation(
                        tool_call.id,
                        confirmation,
                        tool_call.content.as_ref(),
                        window,
                        cx,
                    ))
                }
                ToolCallStatus::Allowed { .. } | ToolCallStatus::Canceled => {
                    tool_call.content.as_ref().map(|content| {
                        div()
                            .py_1p5()
                            .child(self.render_tool_call_content(content, window, cx))
                            .into_any_element()
                    })
                }
                ToolCallStatus::Rejected => None,
            }
        } else {
            None
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
                            this.px_2()
                                .py_1()
                                .rounded_t_md()
                                .bg(self.tool_card_header_bg(cx))
                                .border_b_1()
                                .border_color(self.tool_card_border_color(cx))
                        } else {
                            this.opacity(0.8).hover(|style| style.opacity(1.))
                        }
                    })
                    .child(
                        h_flex()
                            .id("tool-call-header")
                            .overflow_x_scroll()
                            .map(|this| {
                                if needs_confirmation {
                                    this.text_xs()
                                } else {
                                    this.text_size(self.tool_name_font_size())
                                }
                            })
                            .gap_1p5()
                            .child(
                                Icon::new(tool_call.icon)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(if tool_call.locations.len() == 1 {
                                let name = tool_call.locations[0]
                                    .path
                                    .file_name()
                                    .unwrap_or_default()
                                    .display()
                                    .to_string();

                                h_flex()
                                    .id(("open-tool-call-location", entry_ix))
                                    .child(name)
                                    .w_full()
                                    .max_w_full()
                                    .pr_1()
                                    .gap_0p5()
                                    .cursor_pointer()
                                    .rounded_sm()
                                    .opacity(0.8)
                                    .hover(|label| {
                                        label.opacity(1.).bg(cx
                                            .theme()
                                            .colors()
                                            .element_hover
                                            .opacity(0.5))
                                    })
                                    .tooltip(Tooltip::text("Jump to File"))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.open_tool_call_location(entry_ix, 0, window, cx);
                                    }))
                                    .into_any_element()
                            } else {
                                self.render_markdown(
                                    tool_call.label.clone(),
                                    default_markdown_style(needs_confirmation, window, cx),
                                )
                                .into_any()
                            }),
                    )
                    .child(
                        h_flex()
                            .gap_0p5()
                            .when(is_collapsible, |this| {
                                this.child(
                                    Disclosure::new(("expand", tool_call.id.0), is_open)
                                        .opened_icon(IconName::ChevronUp)
                                        .closed_icon(IconName::ChevronDown)
                                        .on_click(cx.listener({
                                            let id = tool_call.id;
                                            move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                                                if is_open {
                                                    this.expanded_tool_calls.remove(&id);
                                                } else {
                                                    this.expanded_tool_calls.insert(id);
                                                }
                                                cx.notify();
                                            }
                                        })),
                                )
                            })
                            .children(status_icon),
                    )
                    .on_click(cx.listener({
                        let id = tool_call.id;
                        move |this: &mut Self, _, _, cx: &mut Context<Self>| {
                            if is_open {
                                this.expanded_tool_calls.remove(&id);
                            } else {
                                this.expanded_tool_calls.insert(id);
                            }
                            cx.notify();
                        }
                    })),
            )
            .when(is_open, |this| {
                this.child(
                    div()
                        .text_xs()
                        .when(is_collapsible, |this| {
                            this.mt_1()
                                .border_1()
                                .border_color(self.tool_card_border_color(cx))
                                .bg(cx.theme().colors().editor_background)
                                .rounded_lg()
                        })
                        .children(content),
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
            ToolCallContent::Markdown { markdown } => {
                div()
                    .p_2()
                    .child(self.render_markdown(
                        markdown.clone(),
                        default_markdown_style(false, window, cx),
                    ))
                    .into_any_element()
            }
            ToolCallContent::Diff {
                diff: Diff { multibuffer, .. },
                ..
            } => self.render_diff_editor(multibuffer),
        }
    }

    fn render_tool_call_confirmation(
        &self,
        tool_call_id: ToolCallId,
        confirmation: &ToolCallConfirmation,
        content: Option<&ToolCallContent>,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let confirmation_container = v_flex().mt_1().py_1p5();

        let button_container = h_flex()
            .pt_1p5()
            .px_1p5()
            .gap_1()
            .justify_end()
            .border_t_1()
            .border_color(self.tool_card_border_color(cx));

        match confirmation {
            ToolCallConfirmation::Edit { description } => confirmation_container
                .child(
                    div()
                        .px_2()
                        .children(description.clone().map(|description| {
                            self.render_markdown(
                                description,
                                default_markdown_style(false, window, cx),
                            )
                        })),
                )
                .children(content.map(|content| self.render_tool_call_content(content, window, cx)))
                .child(
                    button_container
                        .child(
                            Button::new(("always_allow", tool_call_id.0), "Always Allow Edits")
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::AlwaysAllow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("allow", tool_call_id.0), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Allow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("reject", tool_call_id.0), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Error)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Reject,
                                            cx,
                                        );
                                    }
                                })),
                        ),
                )
                .into_any(),
            ToolCallConfirmation::Execute {
                command,
                root_command,
                description,
            } => confirmation_container
                .child(v_flex().px_2().pb_1p5().child(command.clone()).children(
                    description.clone().map(|description| {
                        self.render_markdown(description, default_markdown_style(false, window, cx))
                            .on_url_click({
                                let workspace = self.workspace.clone();
                                move |text, window, cx| {
                                    Self::open_link(text, &workspace, window, cx);
                                }
                            })
                    }),
                ))
                .children(content.map(|content| self.render_tool_call_content(content, window, cx)))
                .child(
                    button_container
                        .child(
                            Button::new(
                                ("always_allow", tool_call_id.0),
                                format!("Always Allow {root_command}"),
                            )
                            .icon(IconName::CheckDouble)
                            .icon_position(IconPosition::Start)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Success)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener({
                                let id = tool_call_id;
                                move |this, _, _, cx| {
                                    this.authorize_tool_call(
                                        id,
                                        acp::ToolCallConfirmationOutcome::AlwaysAllow,
                                        cx,
                                    );
                                }
                            })),
                        )
                        .child(
                            Button::new(("allow", tool_call_id.0), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Allow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("reject", tool_call_id.0), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Error)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Reject,
                                            cx,
                                        );
                                    }
                                })),
                        ),
                )
                .into_any(),
            ToolCallConfirmation::Mcp {
                server_name,
                tool_name: _,
                tool_display_name,
                description,
            } => confirmation_container
                .child(
                    v_flex()
                        .px_2()
                        .pb_1p5()
                        .child(format!("{server_name} - {tool_display_name}"))
                        .children(description.clone().map(|description| {
                            self.render_markdown(
                                description,
                                default_markdown_style(false, window, cx),
                            )
                        })),
                )
                .children(content.map(|content| self.render_tool_call_content(content, window, cx)))
                .child(
                    button_container
                        .child(
                            Button::new(
                                ("always_allow_server", tool_call_id.0),
                                format!("Always Allow {server_name}"),
                            )
                            .icon(IconName::CheckDouble)
                            .icon_position(IconPosition::Start)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Success)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener({
                                let id = tool_call_id;
                                move |this, _, _, cx| {
                                    this.authorize_tool_call(
                                        id,
                                        acp::ToolCallConfirmationOutcome::AlwaysAllowMcpServer,
                                        cx,
                                    );
                                }
                            })),
                        )
                        .child(
                            Button::new(
                                ("always_allow_tool", tool_call_id.0),
                                format!("Always Allow {tool_display_name}"),
                            )
                            .icon(IconName::CheckDouble)
                            .icon_position(IconPosition::Start)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Success)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener({
                                let id = tool_call_id;
                                move |this, _, _, cx| {
                                    this.authorize_tool_call(
                                        id,
                                        acp::ToolCallConfirmationOutcome::AlwaysAllowTool,
                                        cx,
                                    );
                                }
                            })),
                        )
                        .child(
                            Button::new(("allow", tool_call_id.0), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Allow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("reject", tool_call_id.0), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Error)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Reject,
                                            cx,
                                        );
                                    }
                                })),
                        ),
                )
                .into_any(),
            ToolCallConfirmation::Fetch { description, urls } => confirmation_container
                .child(
                    v_flex()
                        .px_2()
                        .pb_1p5()
                        .gap_1()
                        .children(urls.iter().map(|url| {
                            h_flex().child(
                                Button::new(url.clone(), url)
                                    .icon(IconName::ArrowUpRight)
                                    .icon_color(Color::Muted)
                                    .icon_size(IconSize::XSmall)
                                    .on_click({
                                        let url = url.clone();
                                        move |_, _, cx| cx.open_url(&url)
                                    }),
                            )
                        }))
                        .children(description.clone().map(|description| {
                            self.render_markdown(
                                description,
                                default_markdown_style(false, window, cx),
                            )
                        })),
                )
                .children(content.map(|content| self.render_tool_call_content(content, window, cx)))
                .child(
                    button_container
                        .child(
                            Button::new(("always_allow", tool_call_id.0), "Always Allow")
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::AlwaysAllow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("allow", tool_call_id.0), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Allow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("reject", tool_call_id.0), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Error)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Reject,
                                            cx,
                                        );
                                    }
                                })),
                        ),
                )
                .into_any(),
            ToolCallConfirmation::Other { description } => confirmation_container
                .child(v_flex().px_2().pb_1p5().child(self.render_markdown(
                    description.clone(),
                    default_markdown_style(false, window, cx),
                )))
                .children(content.map(|content| self.render_tool_call_content(content, window, cx)))
                .child(
                    button_container
                        .child(
                            Button::new(("always_allow", tool_call_id.0), "Always Allow")
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::AlwaysAllow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("allow", tool_call_id.0), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Success)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Allow,
                                            cx,
                                        );
                                    }
                                })),
                        )
                        .child(
                            Button::new(("reject", tool_call_id.0), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Error)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener({
                                    let id = tool_call_id;
                                    move |this, _, _, cx| {
                                        this.authorize_tool_call(
                                            id,
                                            acp::ToolCallConfirmationOutcome::Reject,
                                            cx,
                                        );
                                    }
                                })),
                        ),
                )
                .into_any(),
        }
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

    fn render_gemini_logo(&self) -> AnyElement {
        Icon::new(IconName::AiGemini)
            .color(Color::Muted)
            .size(IconSize::XLarge)
            .into_any_element()
    }

    fn render_error_gemini_logo(&self) -> AnyElement {
        let logo = Icon::new(IconName::AiGemini)
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

    fn render_empty_state(&self, loading: bool, cx: &App) -> AnyElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                if loading {
                    h_flex()
                        .justify_center()
                        .child(self.render_gemini_logo())
                        .with_animation(
                            "pulsating_icon",
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.4, 1.0)),
                            |icon, delta| icon.opacity(delta),
                        ).into_any()
                } else {
                    self.render_gemini_logo().into_any_element()
                }
            )
            .child(
                h_flex()
                    .mt_4()
                    .mb_1()
                    .justify_center()
                    .child(Headline::new(if loading {
                        "Connecting to Gemini…"
                    } else {
                        "Welcome to Gemini"
                    }).size(HeadlineSize::Medium)),
            )
            .child(
                div()
                    .max_w_1_2()
                    .text_sm()
                    .text_center()
                    .map(|this| if loading {
                        this.invisible()
                    } else {
                        this.text_color(cx.theme().colors().text_muted)
                    })
                    .child("Ask questions, edit files, run commands.\nBe specific for the best results.")
            )
            .into_any()
    }

    fn render_pending_auth_state(&self) -> AnyElement {
        v_flex()
            .items_center()
            .justify_center()
            .child(self.render_error_gemini_logo())
            .child(
                h_flex()
                    .mt_4()
                    .mb_1()
                    .justify_center()
                    .child(Headline::new("Not Authenticated").size(HeadlineSize::Medium)),
            )
            .into_any()
    }

    fn render_error_state(&self, e: &LoadError, cx: &Context<Self>) -> AnyElement {
        let mut container = v_flex()
            .items_center()
            .justify_center()
            .child(self.render_error_gemini_logo())
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

        if matches!(e, LoadError::Unsupported { .. }) {
            container =
                container.child(Button::new("upgrade", "Upgrade Gemini to Latest").on_click(
                    cx.listener(|this, _, window, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                let project = workspace.project().read(cx);
                                let cwd = project.first_project_directory(cx);
                                let shell = project.terminal_settings(&cwd, cx).shell.clone();
                                let command =
                                    "npm install -g @google/gemini-cli@latest".to_string();
                                let spawn_in_terminal = task::SpawnInTerminal {
                                    id: task::TaskId("install".to_string()),
                                    full_label: command.clone(),
                                    label: command.clone(),
                                    command: Some(command.clone()),
                                    args: Vec::new(),
                                    command_label: command.clone(),
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

    fn render_edits_bar(
        &self,
        thread_entity: &Entity<AcpThread>,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let thread = thread_entity.read(cx);
        let action_log = thread.action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        if changed_buffers.is_empty() {
            return None;
        }

        let editor_bg_color = cx.theme().colors().editor_background;
        let active_color = cx.theme().colors().element_selected;
        let bg_edit_files_disclosure = editor_bg_color.blend(active_color.opacity(0.3));

        let pending_edits = thread.has_pending_edit_tool_calls();
        let expanded = self.edits_expanded;

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
            .child(self.render_edits_bar_summary(
                action_log,
                &changed_buffers,
                expanded,
                pending_edits,
                window,
                cx,
            ))
            .when(expanded, |parent| {
                parent.child(self.render_edits_bar_files(
                    action_log,
                    &changed_buffers,
                    pending_edits,
                    cx,
                ))
            })
            .into_any()
            .into()
    }

    fn render_edits_bar_summary(
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
                    .cursor_pointer()
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

    fn render_edits_bar_files(
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

    fn render_message_editor(&mut self, cx: &mut Context<Self>) -> AnyElement {
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
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                syntax: cx.theme().syntax().clone(),
                ..Default::default()
            },
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
                .on_click(cx.listener(|this, _, window, cx| {
                    this.chat(&Chat, window, cx);
                }))
                .when(!is_editor_empty, |button| {
                    button.tooltip(move |window, cx| Tooltip::for_action("Send", &Chat, window, cx))
                })
                .when(is_editor_empty, |button| {
                    button.tooltip(Tooltip::text("Type a message to submit"))
                })
                .into_any_element()
        } else {
            IconButton::new("stop-generation", IconName::StopFilled)
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

        let (thread_summary, markdown) = match &self.thread_state {
            ThreadState::Ready { thread, .. } | ThreadState::Unauthenticated { thread } => {
                let thread = thread.read(cx);
                (thread.title().to_string(), thread.to_markdown(cx))
            }
            ThreadState::Loading { .. } | ThreadState::LoadError(..) => return Task::ready(Ok(())),
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
}

impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_as_markdown = IconButton::new("open-as-markdown", IconName::DocumentText)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Open Thread as Markdown"))
            .on_click(cx.listener(move |this, _, window, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    this.open_thread_as_markdown(workspace, window, cx)
                        .detach_and_log_err(cx);
                }
            }));

        let scroll_to_top = IconButton::new("scroll_to_top", IconName::ArrowUpAlt)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Scroll To Top"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.scroll_to_top(cx);
            }));

        v_flex()
            .size_full()
            .key_context("AcpThread")
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::previous_history_message))
            .on_action(cx.listener(Self::next_history_message))
            .on_action(cx.listener(Self::open_agent_diff))
            .child(match &self.thread_state {
                ThreadState::Unauthenticated { .. } => v_flex()
                    .p_2()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(self.render_pending_auth_state())
                    .child(h_flex().mt_1p5().justify_center().child(
                        Button::new("sign-in", "Sign in to Gemini").on_click(
                            cx.listener(|this, _, window, cx| this.authenticate(window, cx)),
                        ),
                    )),
                ThreadState::Loading { .. } => {
                    v_flex().flex_1().child(self.render_empty_state(true, cx))
                }
                ThreadState::LoadError(e) => v_flex()
                    .p_2()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(self.render_error_state(e, cx)),
                ThreadState::Ready { thread, .. } => v_flex().flex_1().map(|this| {
                    if self.list_state.item_count() > 0 {
                        this.child(
                            list(self.list_state.clone())
                                .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                                .flex_grow()
                                .into_any(),
                        )
                        .child(
                            h_flex()
                                .group("controls")
                                .mt_1()
                                .mr_1()
                                .py_2()
                                .px(RESPONSE_PADDING_X)
                                .opacity(0.4)
                                .hover(|style| style.opacity(1.))
                                .gap_1()
                                .flex_wrap()
                                .justify_end()
                                .child(open_as_markdown)
                                .child(scroll_to_top)
                                .into_any_element(),
                        )
                        .children(match thread.read(cx).status() {
                            ThreadStatus::Idle | ThreadStatus::WaitingForToolConfirmation => None,
                            ThreadStatus::Generating => div()
                                .px_5()
                                .py_2()
                                .child(LoadingLabel::new("").size(LabelSize::Small))
                                .into(),
                        })
                        .children(self.render_edits_bar(&thread, window, cx))
                    } else {
                        this.child(self.render_empty_state(false, cx))
                    }
                }),
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
            .child(
                v_flex()
                    .p_2()
                    .pt_3()
                    .gap_1()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(self.render_message_editor(cx))
                    .child(
                        h_flex()
                            .justify_between()
                            .child(self.render_follow_toggle(cx))
                            .child(self.render_send_button(cx)),
                    ),
            )
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
