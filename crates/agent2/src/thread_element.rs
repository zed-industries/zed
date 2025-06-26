use std::sync::Arc;

use anyhow::Result;
use editor::{Editor, MultiBuffer};
use gpui::{App, Entity, Focusable, SharedString, Window, div, prelude::*};
use gpui::{FocusHandle, Task};
use language::Buffer;
use ui::Tooltip;
use ui::prelude::*;
use zed_actions::agent::Chat;

use crate::{Message, MessageChunk, Role, Thread};

pub struct ThreadElement {
    thread: Entity<Thread>,
    // todo! use full message editor from agent2
    message_editor: Entity<Editor>,
    send_task: Option<Task<Result<()>>>,
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

        Self {
            thread,
            message_editor,
            send_task: None,
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

        self.send_task = Some(self.thread.update(cx, |thread, cx| {
            let message = Message {
                role: Role::User,
                chunks: vec![MessageChunk::Text { chunk: text.into() }],
            };
            thread.send(message, cx)
        }));

        self.message_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
    }
}

impl Focusable for ThreadElement {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl Render for ThreadElement {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = self.message_editor.read(cx).text(cx);
        let is_editor_empty = text.is_empty();
        let focus_handle = self.message_editor.focus_handle(cx);

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .child(div().h_full())
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
