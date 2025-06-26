use anyhow::Result;
use editor::{Editor, MultiBuffer};
use gpui::{App, Entity, Focusable, SharedString, Subscription, Window, div, prelude::*};
use gpui::{FocusHandle, Task};
use language::Buffer;
use ui::Tooltip;
use ui::prelude::*;
use zed_actions::agent::Chat;

use crate::{AgentThreadEntryContent, Message, MessageChunk, Role, Thread, ThreadEntry};

pub struct ThreadElement {
    thread: Entity<Thread>,
    // todo! use full message editor from agent2
    message_editor: Entity<Editor>,
    send_task: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

impl ThreadElement {
    pub fn new(thread: Entity<Thread>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let message_editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight {
                    min_lines: 5,
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

        let subscription = cx.observe(&thread, |_, _, cx| {
            cx.notify();
        });

        Self {
            thread,
            message_editor,
            send_task: None,
            _subscription: subscription,
        }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        self.thread.read(cx).title()
    }

    pub fn cancel(&mut self) {
        self.send_task.take();
    }

    fn chat(&mut self, _: &Chat, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.message_editor.read(cx).text(cx);
        if text.is_empty() {
            return;
        }

        let task = self.thread.update(cx, |thread, cx| {
            let message = Message {
                role: Role::User,
                chunks: vec![MessageChunk::Text { chunk: text.into() }],
            };
            thread.send(message, cx)
        });

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

    fn render_entry(
        &self,
        entry: &ThreadEntry,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        match &entry.content {
            AgentThreadEntryContent::Message(message) => {
                let message_body = div()
                    .children(message.chunks.iter().map(|chunk| match chunk {
                        MessageChunk::Text { chunk } => {
                            // todo! markdown
                            Label::new(chunk.clone())
                        }
                        _ => todo!(),
                    }))
                    .into_any();

                match message.role {
                    Role::User => div()
                        .my_1()
                        .p_2()
                        .bg(cx.theme().colors().editor_background)
                        .rounded_lg()
                        .shadow_md()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .child(message_body)
                        .into_any(),
                    Role::Assistant => message_body,
                }
            }
            AgentThreadEntryContent::ReadFile { path, content: _ } => {
                // todo!
                div()
                    .child(format!("<Reading file {}>", path.display()))
                    .into_any()
            }
        }
    }
}

impl Focusable for ThreadElement {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl Render for ThreadElement {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.message_editor.read(cx).text(cx);
        let is_editor_empty = text.is_empty();
        let focus_handle = self.message_editor.focus_handle(cx);

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .child(
                v_flex().p_2().h_full().gap_1().children(
                    self.thread
                        .read(cx)
                        .entries()
                        .iter()
                        .map(|entry| self.render_entry(entry, window, cx)),
                ),
            )
            .when(self.send_task.is_some(), |this| {
                this.child(
                    div().p_2().child(
                        Label::new("Generating...")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
                )
            })
            .child(
                div()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .child(self.message_editor.clone()),
            )
            .child(
                h_flex().p_2().justify_end().child(
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
                        }),
                ),
            )
    }
}
