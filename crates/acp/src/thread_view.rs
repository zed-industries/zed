use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use agentic_coding_protocol::{self as acp, ToolCallConfirmation};
use anyhow::Result;
use editor::{Editor, MultiBuffer};
use gpui::{
    Animation, AnimationExt, App, EdgesRefinement, Empty, Entity, Focusable, ListState,
    SharedString, StyleRefinement, Subscription, TextStyleRefinement, Transformation,
    UnderlineStyle, Window, div, list, percentage, prelude::*,
};
use gpui::{FocusHandle, Task};
use language::Buffer;
use markdown::{HeadingLevelStyles, MarkdownElement, MarkdownStyle};
use project::Project;
use settings::Settings as _;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::{Button, Tooltip};
use util::ResultExt;
use zed_actions::agent::Chat;

use crate::{
    AcpServer, AcpThread, AcpThreadEvent, AgentThreadEntryContent, MessageChunk, Role, ThreadEntry,
    ToolCall, ToolCallId, ToolCallStatus,
};

pub struct AcpThreadView {
    thread_state: ThreadState,
    // todo! use full message editor from agent2
    message_editor: Entity<Editor>,
    list_state: ListState,
    send_task: Option<Task<Result<()>>>,
    root: Arc<Path>,
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
}

impl AcpThreadView {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // todo!(): This should probably be contextual, like the terminal
        let Some(root_dir) = project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path())
        else {
            todo!();
        };

        let cli_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../gemini-cli/packages/cli");

        let child = util::command::new_smol_command("node")
            .arg(cli_path)
            .arg("--acp")
            .current_dir(&root_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .unwrap();

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

        let project = project.clone();
        let load_task = cx.spawn_in(window, async move |this, cx| {
            let agent = AcpServer::stdio(child, project, cx);
            let result = agent.create_thread(cx).await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(thread) => {
                        let subscription = cx.subscribe(&thread, |this, _, event, cx| {
                            let count = this.list_state.item_count();
                            match event {
                                AcpThreadEvent::NewEntry => {
                                    this.list_state.splice(count..count, 1);
                                }
                                AcpThreadEvent::EntryUpdated(index) => {
                                    this.list_state.splice(*index..*index + 1, 1);
                                }
                            }
                            cx.notify();
                        });
                        this.list_state
                            .splice(0..0, thread.read(cx).entries().len());

                        this.thread_state = ThreadState::Ready {
                            thread,
                            _subscription: subscription,
                        };
                    }
                    Err(e) => this.thread_state = ThreadState::LoadError(e.to_string().into()),
                };
                cx.notify();
            })
            .log_err();
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
                    this.render_entry(entry, window, cx)
                }
            }),
        );

        Self {
            thread_state: ThreadState::Loading { _task: load_task },
            message_editor,
            send_task: None,
            list_state: list_state,
            root: root_dir,
        }
    }

    fn thread(&self) -> Option<&Entity<AcpThread>> {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => Some(thread),
            ThreadState::Loading { .. } | ThreadState::LoadError(..) => None,
        }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        match &self.thread_state {
            ThreadState::Ready { thread, .. } => thread.read(cx).title(),
            ThreadState::Loading { .. } => "Loading...".into(),
            ThreadState::LoadError(_) => "Failed to load".into(),
        }
    }

    pub fn cancel(&mut self) {
        self.send_task.take();
    }

    fn chat(&mut self, _: &Chat, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.message_editor.read(cx).text(cx);
        if text.is_empty() {
            return;
        }
        let Some(thread) = self.thread() else { return };

        let task = thread.update(cx, |thread, cx| thread.send(&text, cx));

        self.send_task = Some(cx.spawn(async move |this, cx| {
            task.await?;

            this.update(cx, |this, _cx| {
                this.send_task.take();
            })
        }));

        self.message_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
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
        entry: &ThreadEntry,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match &entry.content {
            AgentThreadEntryContent::Message(message) => {
                let style = if message.role == Role::User {
                    user_message_markdown_style(window, cx)
                } else {
                    default_markdown_style(window, cx)
                };
                let message_body = div()
                    .children(message.chunks.iter().map(|chunk| match chunk {
                        MessageChunk::Text { chunk } => {
                            // todo!() open link
                            MarkdownElement::new(chunk.clone(), style.clone())
                        }
                        _ => todo!(),
                    }))
                    .into_any();

                match message.role {
                    Role::User => div()
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
                        .into_any(),
                    Role::Assistant => div()
                        .text_ui(cx)
                        .p_5()
                        .pt_2()
                        .child(message_body)
                        .into_any(),
                }
            }
            AgentThreadEntryContent::ToolCall(tool_call) => div()
                .px_2()
                .py_4()
                .child(self.render_tool_call(tool_call, window, cx))
                .into_any(),
        }
    }

    fn render_tool_call(&self, tool_call: &ToolCall, window: &Window, cx: &Context<Self>) -> Div {
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
                Some(self.render_tool_call_confirmation(tool_call.id, confirmation, cx))
            }
            ToolCallStatus::Allowed { content, .. } => content.clone().map(|content| {
                div()
                    .border_color(cx.theme().colors().border)
                    .border_t_1()
                    .px_2()
                    .py_1p5()
                    .child(MarkdownElement::new(
                        content,
                        default_markdown_style(window, cx),
                    ))
                    .into_any_element()
            }),
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
                        Icon::new(tool_call.icon.into())
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

    fn render_tool_call_confirmation(
        &self,
        tool_call_id: ToolCallId,
        confirmation: &ToolCallConfirmation,
        cx: &Context<Self>,
    ) -> AnyElement {
        match confirmation {
            ToolCallConfirmation::Edit {
                file_name,
                file_diff,
                description,
            } => v_flex()
                .border_color(cx.theme().colors().border)
                .border_t_1()
                .px_2()
                .py_1p5()
                // todo! nicer rendering
                .child(file_name.clone())
                .child(file_diff.clone())
                .children(description.clone())
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
                .into_any(),
            ToolCallConfirmation::Execute {
                command,
                root_command,
                description,
            } => v_flex()
                .border_color(cx.theme().colors().border)
                .border_t_1()
                .px_2()
                .py_1p5()
                // todo! nicer rendering
                .child(command.clone())
                .children(description.clone())
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
                .into_any(),
            ToolCallConfirmation::Mcp {
                server_name,
                tool_name: _,
                tool_display_name,
                description,
            } => v_flex()
                .border_color(cx.theme().colors().border)
                .border_t_1()
                .px_2()
                .py_1p5()
                // todo! nicer rendering
                .child(format!("{server_name} - {tool_display_name}"))
                .children(description.clone())
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
                .into_any(),
            ToolCallConfirmation::Fetch { description, urls } => v_flex()
                .border_color(cx.theme().colors().border)
                .border_t_1()
                .px_2()
                .py_1p5()
                // todo! nicer rendering
                .children(urls.clone())
                .children(description.clone())
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
                .child(description.clone())
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
}

impl Focusable for AcpThreadView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl Render for AcpThreadView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.message_editor.read(cx).text(cx);
        let is_editor_empty = text.is_empty();
        let focus_handle = self.message_editor.focus_handle(cx);

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .h_full()
            .child(match &self.thread_state {
                ThreadState::Loading { .. } => v_flex()
                    .p_2()
                    .flex_1()
                    .justify_end()
                    .child(Label::new("Connecting to Gemini...")),
                ThreadState::LoadError(e) => div()
                    .p_2()
                    .flex_1()
                    .justify_end()
                    .child(Label::new(format!("Failed to load {e}")).into_any_element()),
                ThreadState::Ready { thread, .. } => v_flex()
                    .flex_1()
                    .gap_2()
                    .pb_2()
                    .child(
                        list(self.list_state.clone())
                            .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                            .flex_grow(),
                    )
                    .child(div().px_3().children(if self.send_task.is_none() {
                        None
                    } else {
                        Label::new(if thread.read(cx).waiting_for_tool_confirmation() {
                            "Waiting for tool confirmation"
                        } else {
                            "Generating..."
                        })
                        .color(Color::Muted)
                        .size(LabelSize::Small)
                        .into()
                    })),
            })
            .child(
                v_flex()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .gap_2()
                    .child(self.message_editor.clone())
                    .child(h_flex().justify_end().child(if self.send_task.is_some() {
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
                            .disabled(is_editor_empty)
                            .on_click(cx.listener(|this, _event, _, _| this.cancel()))
                    } else {
                        IconButton::new("send-message", IconName::Send)
                            .icon_color(Color::Accent)
                            .style(ButtonStyle::Filled)
                            .disabled(is_editor_empty)
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
                    })),
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
