use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use agentic_coding_protocol::{self as acp};
use editor::{Editor, EditorMode, MinimapVisibility, MultiBuffer};
use gpui::{
    Animation, AnimationExt, App, EdgesRefinement, Empty, Entity, Focusable, ListState,
    SharedString, StyleRefinement, Subscription, TextStyleRefinement, Transformation,
    UnderlineStyle, Window, div, list, percentage, prelude::*,
};
use gpui::{FocusHandle, Task};
use language::Buffer;
use language::language_settings::SoftWrap;
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use project::Project;
use settings::Settings as _;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::{Button, Tooltip};
use util::{ResultExt, paths};
use zed_actions::agent::Chat;

use crate::{
    AcpServer, AcpThread, AcpThreadEvent, AgentThreadEntryContent, AssistantMessage,
    AssistantMessageChunk, Diff, ThreadEntry, ThreadStatus, ToolCall, ToolCallConfirmation,
    ToolCallContent, ToolCallId, ToolCallStatus, UserMessageChunk,
};

pub struct AcpThreadView {
    agent: Arc<AcpServer>,
    thread_state: ThreadState,
    // todo! reconsider structure. currently pretty sparse, but easy to clean up if we need to delete entries.
    thread_entry_views: Vec<Option<ThreadEntryView>>,
    message_editor: Entity<Editor>,
    last_error: Option<Entity<Markdown>>,
    list_state: ListState,
    auth_task: Option<Task<()>>,
}

#[derive(Debug)]
enum ThreadEntryView {
    Diff { editor: Entity<Editor> },
}

enum ThreadState {
    Loading {
        _task: Task<()>,
    },
    Ready {
        thread: Entity<AcpThread>,
        _subscription: Subscription,
    },
    LoadError(SharedString),
    Unauthenticated,
}

impl AcpThreadView {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let message_editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx));
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
            editor.set_placeholder_text("Send a message", cx);
            editor.set_soft_wrap();
            editor
        });

        let list_state = ListState::new(
            0,
            gpui::ListAlignment::Bottom,
            px(2048.0),
            cx.processor({
                move |this: &mut Self, item: usize, window, cx| {
                    let Some(entry) = this
                        .thread()
                        .and_then(|thread| thread.read(cx).entries.get(item))
                    else {
                        return Empty.into_any();
                    };
                    this.render_entry(item, entry, window, cx)
                }
            }),
        );

        let root_dir = project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path())
            .unwrap_or_else(|| paths::home_dir().as_path().into());

        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../gemini-cli/packages/cli");

        let child = util::command::new_smol_command("node")
            .arg(cli_path)
            .arg("--acp")
            .current_dir(root_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .unwrap();

        let agent = AcpServer::stdio(child, project, cx);

        Self {
            thread_state: Self::initial_state(agent.clone(), window, cx),
            agent,
            message_editor,
            thread_entry_views: Vec::new(),
            list_state: list_state,
            last_error: None,
            auth_task: None,
        }
    }

    fn initial_state(
        agent: Arc<AcpServer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ThreadState {
        let load_task = cx.spawn_in(window, async move |this, cx| {
            let result = match agent.initialize().await {
                Err(e) => Err(e),
                Ok(response) => {
                    if !response.is_authenticated {
                        this.update(cx, |this, _| {
                            this.thread_state = ThreadState::Unauthenticated;
                        })
                        .ok();
                        return;
                    }
                    agent.clone().create_thread(cx).await
                }
            };

            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(thread) => {
                        let subscription =
                            cx.subscribe_in(&thread, window, Self::handle_thread_event);
                        this.list_state
                            .splice(0..0, thread.read(cx).entries().len());

                        this.thread_state = ThreadState::Ready {
                            thread,
                            _subscription: subscription,
                        };
                    }
                    Err(e) => {
                        if let Some(exit_status) = agent.exit_status() {
                            this.thread_state = ThreadState::LoadError(
                                format!(
                                    "Gemini exited with status {}",
                                    exit_status.code().unwrap_or(-127)
                                )
                                .into(),
                            )
                        } else {
                            this.thread_state = ThreadState::LoadError(e.to_string().into())
                        }
                    }
                };
                cx.notify();
            })
            .log_err();
        });

        ThreadState::Loading { _task: load_task }
    }

    fn thread(&self) -> Option<&Entity<AcpThread>> {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => Some(thread),
            ThreadState::Loading { .. }
            | ThreadState::LoadError(..)
            | ThreadState::Unauthenticated => None,
        }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => thread.read(cx).title(),
            ThreadState::Loading { .. } => "Loading...".into(),
            ThreadState::LoadError(_) => "Failed to load".into(),
            ThreadState::Unauthenticated => "Not authenticated".into(),
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
        let text = self.message_editor.read(cx).text(cx);
        if text.is_empty() {
            return;
        }
        let Some(thread) = self.thread() else { return };

        let task = thread.update(cx, |thread, cx| thread.send(&text, cx));

        cx.spawn(async move |this, cx| {
            let result = task.await;

            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    this.last_error =
                        Some(cx.new(|cx| {
                            Markdown::new(format!("Error: {err}").into(), None, None, cx)
                        }))
                }
            })
        })
        .detach();

        self.message_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
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
                self.sync_thread_entry_view(thread.read(cx).entries.len() - 1, window, cx);
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

    // todo! should we do this on the fly from render?
    fn sync_thread_entry_view(
        &mut self,
        entry_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let multibuffer = match (
            self.entry_diff_multibuffer(entry_ix, cx),
            self.thread_entry_views.get(entry_ix),
        ) {
            (Some(multibuffer), Some(Some(ThreadEntryView::Diff { editor }))) => {
                if editor.read(cx).buffer() == &multibuffer {
                    // same buffer, all synced up
                    return;
                }
                // new buffer, replace editor
                multibuffer
            }
            (Some(multibuffer), _) => multibuffer,
            (None, Some(Some(ThreadEntryView::Diff { .. }))) => {
                // no longer displaying a diff, drop editor
                self.thread_entry_views[entry_ix] = None;
                return;
            }
            (None, _) => return,
        };

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

        if entry_ix >= self.thread_entry_views.len() {
            self.thread_entry_views
                .resize_with(entry_ix + 1, Default::default);
        }

        self.thread_entry_views[entry_ix] = Some(ThreadEntryView::Diff {
            editor: editor.clone(),
        });
    }

    fn entry_diff_multibuffer(&self, entry_ix: usize, cx: &App) -> Option<Entity<MultiBuffer>> {
        let entry = self.thread()?.read(cx).entries().get(entry_ix)?;
        if let AgentThreadEntryContent::ToolCall(ToolCall {
            content: Some(ToolCallContent::Diff { diff }),
            ..
        }) = &entry.content
        {
            Some(diff.multibuffer.clone())
        } else {
            None
        }
    }

    fn authenticate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let agent = self.agent.clone();
        self.last_error.take();
        self.auth_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = agent.authenticate().await;

            this.update_in(cx, |this, window, cx| {
                if let Err(err) = result {
                    this.last_error =
                        Some(cx.new(|cx| {
                            Markdown::new(format!("Error: {err}").into(), None, None, cx)
                        }))
                } else {
                    this.thread_state = Self::initial_state(agent, window, cx)
                }
                this.auth_task.take()
            })
            .ok();
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
        entry: &ThreadEntry,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match &entry.content {
            AgentThreadEntryContent::UserMessage(message) => {
                let style = user_message_markdown_style(window, cx);
                let message_body = div().children(message.chunks.iter().map(|chunk| match chunk {
                    UserMessageChunk::Text { chunk } => {
                        // todo!() open link
                        MarkdownElement::new(chunk.clone(), style.clone())
                    }
                    _ => todo!(),
                }));
                div()
                    .p_2()
                    .pt_5()
                    .child(
                        div()
                            .text_xs()
                            .p_3()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_lg()
                            .shadow_md()
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .child(message_body),
                    )
                    .into_any()
            }
            AgentThreadEntryContent::AssistantMessage(AssistantMessage { chunks }) => {
                let style = default_markdown_style(window, cx);
                let message_body = div()
                    .children(chunks.iter().map(|chunk| match chunk {
                        AssistantMessageChunk::Text { chunk } => {
                            // todo!() open link
                            MarkdownElement::new(chunk.clone(), style.clone()).into_any_element()
                        }
                        AssistantMessageChunk::Thought { chunk } => {
                            self.render_thinking_block(chunk.clone(), window, cx)
                        }
                    }))
                    .into_any();

                div()
                    .text_ui(cx)
                    .p_5()
                    .pt_2()
                    .child(message_body)
                    .into_any()
            }
            AgentThreadEntryContent::ToolCall(tool_call) => div()
                .px_2()
                .py_4()
                .child(self.render_tool_call(index, tool_call, window, cx))
                .into_any(),
        }
    }

    fn render_thinking_block(
        &self,
        chunk: Entity<Markdown>,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        v_flex()
            .mt_neg_2()
            .mb_1p5()
            .child(
                h_flex().group("disclosure-header").justify_between().child(
                    h_flex()
                        .gap_1p5()
                        .child(
                            Icon::new(IconName::LightBulb)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(Label::new("Thinking").size(LabelSize::Small)),
                ),
            )
            .child(div().relative().rounded_b_lg().mt_2().pl_4().child(
                div().max_h_20().text_ui_sm(cx).overflow_hidden().child(
                    // todo! url click
                    MarkdownElement::new(chunk, default_markdown_style(window, cx)),
                    // .on_url_click({
                    //     let workspace = self.workspace.clone();
                    //     move |text, window, cx| {
                    //         open_markdown_link(text, workspace.clone(), window, cx);
                    //     }
                    // }),
                ),
            ))
            .into_any_element()
    }

    fn render_tool_call(
        &self,
        entry_ix: usize,
        tool_call: &ToolCall,
        window: &Window,
        cx: &Context<Self>,
    ) -> Div {
        let status_icon = match &tool_call.status {
            ToolCallStatus::WaitingForConfirmation { .. } => Empty.into_element().into_any(),
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Running,
                ..
            } => Icon::new(IconName::ArrowCircle)
                .color(Color::Success)
                .size(IconSize::Small)
                .with_animation(
                    "running",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                )
                .into_any_element(),
            ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Finished,
                ..
            } => Icon::new(IconName::Check)
                .color(Color::Success)
                .size(IconSize::Small)
                .into_any_element(),
            ToolCallStatus::Rejected
            | ToolCallStatus::Canceled
            | ToolCallStatus::Allowed {
                status: acp::ToolCallStatus::Error,
                ..
            } => Icon::new(IconName::X)
                .color(Color::Error)
                .size(IconSize::Small)
                .into_any_element(),
        };

        let content = match &tool_call.status {
            ToolCallStatus::WaitingForConfirmation { confirmation, .. } => {
                Some(self.render_tool_call_confirmation(
                    entry_ix,
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
                        .border_color(cx.theme().colors().border)
                        .border_t_1()
                        .px_2()
                        .py_1p5()
                        .child(self.render_tool_call_content(entry_ix, content, window, cx))
                        .into_any_element()
                })
            }
            ToolCallStatus::Rejected => None,
        };

        v_flex()
            .text_xs()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1p5()
                    .w_full()
                    .gap_1p5()
                    .child(
                        Icon::new(tool_call.icon)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    // todo! danilo please help
                    .child(MarkdownElement::new(
                        tool_call.label.clone(),
                        default_markdown_style(window, cx),
                    ))
                    .child(div().w_full())
                    .child(status_icon),
            )
            .children(content)
    }

    fn render_tool_call_content(
        &self,
        entry_ix: usize,
        content: &ToolCallContent,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match content {
            ToolCallContent::Markdown { markdown } => {
                MarkdownElement::new(markdown.clone(), default_markdown_style(window, cx))
                    .into_any_element()
            }
            ToolCallContent::Diff {
                diff: Diff { path, .. },
                ..
            } => self.render_diff_editor(entry_ix, path),
        }
    }

    fn render_tool_call_confirmation(
        &self,
        entry_ix: usize,
        tool_call_id: ToolCallId,
        confirmation: &ToolCallConfirmation,
        content: Option<&ToolCallContent>,
        window: &Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match confirmation {
            ToolCallConfirmation::Edit { description } => {
                v_flex()
                    .border_color(cx.theme().colors().border)
                    .border_t_1()
                    .px_2()
                    .py_1p5()
                    .children(description.clone().map(|description| {
                        MarkdownElement::new(description, default_markdown_style(window, cx))
                    }))
                    .children(content.map(|content| {
                        self.render_tool_call_content(entry_ix, content, window, cx)
                    }))
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_1()
                            .child(
                                Button::new(
                                    ("always_allow", tool_call_id.as_u64()),
                                    "Always Allow Edits",
                                )
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
                                Button::new(("allow", tool_call_id.as_u64()), "Allow")
                                    .icon(IconName::Check)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
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
                                Button::new(("reject", tool_call_id.as_u64()), "Reject")
                                    .icon(IconName::X)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
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
                    .into_any()
            }
            ToolCallConfirmation::Execute {
                command,
                root_command,
                description,
            } => {
                v_flex()
                    .border_color(cx.theme().colors().border)
                    .border_t_1()
                    .px_2()
                    .py_1p5()
                    // todo! nicer rendering
                    .child(command.clone())
                    .children(description.clone().map(|description| {
                        MarkdownElement::new(description, default_markdown_style(window, cx))
                    }))
                    .children(content.map(|content| {
                        self.render_tool_call_content(entry_ix, content, window, cx)
                    }))
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_1()
                            .child(
                                Button::new(
                                    ("always_allow", tool_call_id.as_u64()),
                                    format!("Always Allow {root_command}"),
                                )
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
                                Button::new(("allow", tool_call_id.as_u64()), "Allow")
                                    .icon(IconName::Check)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
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
                                Button::new(("reject", tool_call_id.as_u64()), "Reject")
                                    .icon(IconName::X)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
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
                    .into_any()
            }
            ToolCallConfirmation::Mcp {
                server_name,
                tool_name: _,
                tool_display_name,
                description,
            } => {
                v_flex()
                    .border_color(cx.theme().colors().border)
                    .border_t_1()
                    .px_2()
                    .py_1p5()
                    // todo! nicer rendering
                    .child(format!("{server_name} - {tool_display_name}"))
                    .children(description.clone().map(|description| {
                        MarkdownElement::new(description, default_markdown_style(window, cx))
                    }))
                    .children(content.map(|content| {
                        self.render_tool_call_content(entry_ix, content, window, cx)
                    }))
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_1()
                            .child(
                                Button::new(
                                    ("always_allow_server", tool_call_id.as_u64()),
                                    format!("Always Allow {server_name}"),
                                )
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Success)
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
                                    ("always_allow_tool", tool_call_id.as_u64()),
                                    format!("Always Allow {tool_display_name}"),
                                )
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Success)
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
                                Button::new(("allow", tool_call_id.as_u64()), "Allow")
                                    .icon(IconName::Check)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
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
                                Button::new(("reject", tool_call_id.as_u64()), "Reject")
                                    .icon(IconName::X)
                                    .icon_position(IconPosition::Start)
                                    .icon_size(IconSize::Small)
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
                    .into_any()
            }
            ToolCallConfirmation::Fetch { description, urls } => v_flex()
                .border_color(cx.theme().colors().border)
                .border_t_1()
                .px_2()
                .py_1p5()
                // todo! nicer rendering
                .children(urls.clone())
                .children(description.clone().map(|description| {
                    MarkdownElement::new(description, default_markdown_style(window, cx))
                }))
                .children(
                    content.map(|content| {
                        self.render_tool_call_content(entry_ix, content, window, cx)
                    }),
                )
                .child(
                    h_flex()
                        .justify_end()
                        .gap_1()
                        .child(
                            Button::new(("always_allow", tool_call_id.as_u64()), "Always Allow")
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
                            Button::new(("allow", tool_call_id.as_u64()), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
                            Button::new(("reject", tool_call_id.as_u64()), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
            ToolCallConfirmation::Other { description } => v_flex()
                .border_color(cx.theme().colors().border)
                .border_t_1()
                .px_2()
                .py_1p5()
                // todo! nicer rendering
                .child(MarkdownElement::new(
                    description.clone(),
                    default_markdown_style(window, cx),
                ))
                .children(
                    content.map(|content| {
                        self.render_tool_call_content(entry_ix, content, window, cx)
                    }),
                )
                .child(
                    h_flex()
                        .justify_end()
                        .gap_1()
                        .child(
                            Button::new(("always_allow", tool_call_id.as_u64()), "Always Allow")
                                .icon(IconName::CheckDouble)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
                            Button::new(("allow", tool_call_id.as_u64()), "Allow")
                                .icon(IconName::Check)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
                            Button::new(("reject", tool_call_id.as_u64()), "Reject")
                                .icon(IconName::X)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
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
        }
    }

    fn render_diff_editor(&self, entry_ix: usize, path: &Path) -> AnyElement {
        v_flex()
            .h_full()
            .child(path.to_string_lossy().to_string())
            .child(
                if let Some(Some(ThreadEntryView::Diff { editor })) =
                    self.thread_entry_views.get(entry_ix)
                {
                    editor.clone().into_any_element()
                } else {
                    Empty.into_any()
                },
            )
            .into_any()
    }
}

impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.message_editor.read(cx).text(cx);
        let is_editor_empty = text.is_empty();
        let focus_handle = self.message_editor.focus_handle(cx);

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .h_full()
            .child(match &self.thread_state {
                ThreadState::Unauthenticated => v_flex()
                    .p_2()
                    .flex_1()
                    .justify_end()
                    .child(Label::new("Not authenticated"))
                    .child(Button::new("sign-in", "Sign in via Gemini CLI").on_click(
                        cx.listener(|this, _, window, cx| this.authenticate(window, cx)),
                    )),
                ThreadState::Loading { .. } => v_flex()
                    .p_2()
                    .flex_1()
                    .justify_end()
                    .child(Label::new("Connecting to Gemini...")),
                ThreadState::LoadError(e) => div()
                    .p_2()
                    .flex_1()
                    .justify_end()
                    .child(Label::new(format!("Failed to load: {e}")).into_any_element()),
                ThreadState::Ready { thread, .. } => v_flex()
                    .flex_1()
                    .gap_2()
                    .pb_2()
                    .child(
                        list(self.list_state.clone())
                            .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                            .flex_grow(),
                    )
                    .child(
                        div().px_3().children(match thread.read(cx).status() {
                            ThreadStatus::Idle => None,
                            ThreadStatus::WaitingForToolConfirmation => {
                                Label::new("Waiting for tool confirmation")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small)
                                    .into()
                            }
                            ThreadStatus::Generating => Label::new("Generating...")
                                .color(Color::Muted)
                                .size(LabelSize::Small)
                                .into(),
                        }),
                    ),
            })
            .when_some(self.last_error.clone(), |el, error| {
                el.child(
                    div()
                        .text_xs()
                        .p_2()
                        .gap_2()
                        .border_t_1()
                        .border_color(cx.theme().status().error_border)
                        .bg(cx.theme().status().error_background)
                        .child(MarkdownElement::new(
                            error,
                            default_markdown_style(window, cx),
                        )),
                )
            })
            .child(
                v_flex()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .gap_2()
                    .child(self.message_editor.clone())
                    .child({
                        let thread = self.thread();

                        h_flex().justify_end().child(
                            if thread.map_or(true, |thread| {
                                thread.read(cx).status() == ThreadStatus::Idle
                            }) {
                                IconButton::new("send-message", IconName::Send)
                                    .icon_color(Color::Accent)
                                    .style(ButtonStyle::Filled)
                                    .disabled(thread.is_none() || is_editor_empty)
                                    .on_click({
                                        let focus_handle = focus_handle.clone();
                                        move |_event, window, cx| {
                                            focus_handle.dispatch_action(&Chat, window, cx);
                                        }
                                    })
                                    .when(!is_editor_empty, |button| {
                                        button.tooltip(move |window, cx| {
                                            Tooltip::for_action("Send", &Chat, window, cx)
                                        })
                                    })
                                    .when(is_editor_empty, |button| {
                                        button.tooltip(Tooltip::text("Type a message to submit"))
                                    })
                            } else {
                                IconButton::new("stop-generation", IconName::StopFilled)
                                    .icon_color(Color::Error)
                                    .style(ButtonStyle::Tinted(ui::TintColor::Error))
                                    .tooltip(move |window, cx| {
                                        Tooltip::for_action(
                                            "Stop Generation",
                                            &editor::actions::Cancel,
                                            window,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|this, _event, _, cx| this.cancel(cx)))
                            },
                        )
                    }),
            )
    }
}

fn user_message_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let mut style = default_markdown_style(window, cx);
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
    style
}

fn default_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();
    let ui_font_size = TextSize::Default.rems(cx);
    let buffer_font_size = TextSize::Small.rems(cx);
    let mut text_style = window.text_style();
    let line_height = buffer_font_size * 1.75;

    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(ui_font_size.into()),
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
        link_callback: Some(Rc::new(move |_url, _cx| {
            // todo!()
            // if MentionLink::is_valid(url) {
            //     let colors = cx.theme().colors();
            //     Some(TextStyleRefinement {
            //         background_color: Some(colors.element_background),
            //         ..Default::default()
            //     })
            // } else {
            None
            // }
        })),
        ..Default::default()
    }
}
