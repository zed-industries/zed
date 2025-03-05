use std::sync::Arc;

use assistant_tool::ToolWorkingSet;
use collections::HashMap;
use editor::{Editor, MultiBuffer};
use gpui::{
    list, AbsoluteLength, AnyElement, App, ClickEvent, DefiniteLength, EdgesRefinement, Empty,
    Entity, Focusable, Length, ListAlignment, ListOffset, ListState, StyleRefinement, Subscription,
    Task, TextStyleRefinement, UnderlineStyle, WeakEntity,
};
use language::{Buffer, LanguageRegistry};
use language_model::{LanguageModelRegistry, LanguageModelToolUseId, Role};
use markdown::{Markdown, MarkdownStyle};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{prelude::*, Disclosure, KeyBinding};
use util::ResultExt as _;
use workspace::Workspace;

use crate::thread::{MessageId, RequestKind, Thread, ThreadError, ThreadEvent};
use crate::thread_store::ThreadStore;
use crate::tool_use::{ToolUse, ToolUseStatus};
use crate::ui::ContextPill;

pub struct ActiveThread {
    workspace: WeakEntity<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    tools: Arc<ToolWorkingSet>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<Thread>,
    save_thread_task: Option<Task<()>>,
    messages: Vec<MessageId>,
    list_state: ListState,
    rendered_messages_by_id: HashMap<MessageId, Entity<Markdown>>,
    editing_message: Option<(MessageId, EditMessageState)>,
    expanded_tool_uses: HashMap<LanguageModelToolUseId, bool>,
    last_error: Option<ThreadError>,
    _subscriptions: Vec<Subscription>,
}

struct EditMessageState {
    editor: Entity<Editor>,
}

impl ActiveThread {
    pub fn new(
        thread: Entity<Thread>,
        thread_store: Entity<ThreadStore>,
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        tools: Arc<ToolWorkingSet>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe_in(&thread, window, Self::handle_thread_event),
        ];

        let mut this = Self {
            workspace,
            language_registry,
            tools,
            thread_store,
            thread: thread.clone(),
            save_thread_task: None,
            messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            expanded_tool_uses: HashMap::default(),
            list_state: ListState::new(0, ListAlignment::Bottom, px(1024.), {
                let this = cx.entity().downgrade();
                move |ix, window: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| this.render_message(ix, window, cx))
                        .unwrap()
                }
            }),
            editing_message: None,
            last_error: None,
            _subscriptions: subscriptions,
        };

        for message in thread.read(cx).messages().cloned().collect::<Vec<_>>() {
            this.push_message(&message.id, message.text.clone(), window, cx);
        }

        this
    }

    pub fn thread(&self) -> &Entity<Thread> {
        &self.thread
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn summary(&self, cx: &App) -> Option<SharedString> {
        self.thread.read(cx).summary()
    }

    pub fn summary_or_default(&self, cx: &App) -> SharedString {
        self.thread.read(cx).summary_or_default()
    }

    pub fn cancel_last_completion(&mut self, cx: &mut App) -> bool {
        self.last_error.take();
        self.thread
            .update(cx, |thread, _cx| thread.cancel_last_completion())
    }

    pub fn last_error(&self) -> Option<ThreadError> {
        self.last_error.clone()
    }

    pub fn clear_last_error(&mut self) {
        self.last_error.take();
    }

    fn push_message(
        &mut self,
        id: &MessageId,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_len = self.messages.len();
        self.messages.push(*id);
        self.list_state.splice(old_len..old_len, 1);

        let markdown = self.render_markdown(text.into(), window, cx);
        self.rendered_messages_by_id.insert(*id, markdown);
        self.list_state.scroll_to(ListOffset {
            item_ix: old_len,
            offset_in_item: Pixels(0.0),
        });
    }

    fn edited_message(
        &mut self,
        id: &MessageId,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.messages.iter().position(|message_id| message_id == id) else {
            return;
        };
        self.list_state.splice(index..index + 1, 1);
        let markdown = self.render_markdown(text.into(), window, cx);
        self.rendered_messages_by_id.insert(*id, markdown);
    }

    fn deleted_message(&mut self, id: &MessageId) {
        let Some(index) = self.messages.iter().position(|message_id| message_id == id) else {
            return;
        };
        self.messages.remove(index);
        self.list_state.splice(index..index + 1, 0);
        self.rendered_messages_by_id.remove(id);
    }

    fn render_markdown(
        &self,
        text: SharedString,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Entity<Markdown> {
        let theme_settings = ThemeSettings::get_global(cx);
        let colors = cx.theme().colors();
        let ui_font_size = TextSize::Default.rems(cx);
        let buffer_font_size = TextSize::Small.rems(cx);
        let mut text_style = window.text_style();

        text_style.refine(&TextStyleRefinement {
            font_family: Some(theme_settings.ui_font.family.clone()),
            font_size: Some(ui_font_size.into()),
            color: Some(cx.theme().colors().text),
            ..Default::default()
        });

        let markdown_style = MarkdownStyle {
            base_text_style: text_style,
            syntax: cx.theme().syntax().clone(),
            selection_background_color: cx.theme().players().local().selection,
            code_block_overflow_x_scroll: true,
            table_overflow_x_scroll: true,
            code_block: StyleRefinement {
                margin: EdgesRefinement {
                    top: Some(Length::Definite(rems(0.).into())),
                    left: Some(Length::Definite(rems(0.).into())),
                    right: Some(Length::Definite(rems(0.).into())),
                    bottom: Some(Length::Definite(rems(0.5).into())),
                },
                padding: EdgesRefinement {
                    top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                    left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                    right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                    bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                },
                background: Some(colors.editor_background.into()),
                border_color: Some(colors.border_variant),
                border_widths: EdgesRefinement {
                    top: Some(AbsoluteLength::Pixels(Pixels(1.))),
                    left: Some(AbsoluteLength::Pixels(Pixels(1.))),
                    right: Some(AbsoluteLength::Pixels(Pixels(1.))),
                    bottom: Some(AbsoluteLength::Pixels(Pixels(1.))),
                },
                text: Some(TextStyleRefinement {
                    font_family: Some(theme_settings.buffer_font.family.clone()),
                    font_size: Some(buffer_font_size.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            inline_code: TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_size: Some(buffer_font_size.into()),
                background_color: Some(colors.editor_foreground.opacity(0.1)),
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
        };

        cx.new(|cx| {
            Markdown::new(
                text,
                markdown_style,
                Some(self.language_registry.clone()),
                None,
                cx,
            )
        })
    }

    fn handle_thread_event(
        &mut self,
        _: &Entity<Thread>,
        event: &ThreadEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ThreadEvent::ShowError(error) => {
                self.last_error = Some(error.clone());
            }
            ThreadEvent::StreamedCompletion | ThreadEvent::SummaryChanged => {
                self.save_thread(cx);
            }
            ThreadEvent::StreamedAssistantText(message_id, text) => {
                if let Some(markdown) = self.rendered_messages_by_id.get_mut(&message_id) {
                    markdown.update(cx, |markdown, cx| {
                        markdown.append(text, cx);
                    });
                }
            }
            ThreadEvent::MessageAdded(message_id) => {
                if let Some(message_text) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.text.clone())
                {
                    self.push_message(message_id, message_text, window, cx);
                }

                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::MessageEdited(message_id) => {
                if let Some(message_text) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.text.clone())
                {
                    self.edited_message(message_id, message_text, window, cx);
                }

                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::MessageDeleted(message_id) => {
                self.deleted_message(message_id);
                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::UsePendingTools => {
                let pending_tool_uses = self
                    .thread
                    .read(cx)
                    .pending_tool_uses()
                    .into_iter()
                    .filter(|tool_use| tool_use.status.is_idle())
                    .cloned()
                    .collect::<Vec<_>>();

                for tool_use in pending_tool_uses {
                    if let Some(tool) = self.tools.tool(&tool_use.name, cx) {
                        let task = tool.run(tool_use.input, self.workspace.clone(), window, cx);

                        self.thread.update(cx, |thread, cx| {
                            thread.insert_tool_output(tool_use.id.clone(), task, cx);
                        });
                    }
                }
            }
            ThreadEvent::ToolFinished { .. } => {
                let all_tools_finished = self
                    .thread
                    .read(cx)
                    .pending_tool_uses()
                    .into_iter()
                    .all(|tool_use| tool_use.status.is_error());
                if all_tools_finished {
                    let model_registry = LanguageModelRegistry::read_global(cx);
                    if let Some(model) = model_registry.active_model() {
                        self.thread.update(cx, |thread, cx| {
                            // Insert a user message to contain the tool results.
                            thread.insert_user_message(
                                // TODO: Sending up a user message without any content results in the model sending back
                                // responses that also don't have any content. We currently don't handle this case well,
                                // so for now we provide some text to keep the model on track.
                                "Here are the tool results.",
                                Vec::new(),
                                cx,
                            );
                            thread.send_to_model(model, RequestKind::Chat, true, cx);
                        });
                    }
                }
            }
        }
    }

    /// Spawns a task to save the active thread.
    ///
    /// Only one task to save the thread will be in flight at a time.
    fn save_thread(&mut self, cx: &mut Context<Self>) {
        let thread = self.thread.clone();
        self.save_thread_task = Some(cx.spawn(|this, mut cx| async move {
            let task = this
                .update(&mut cx, |this, cx| {
                    this.thread_store
                        .update(cx, |thread_store, cx| thread_store.save_thread(&thread, cx))
                })
                .ok();

            if let Some(task) = task {
                task.await.log_err();
            }
        }));
    }

    fn start_editing_message(
        &mut self,
        message_id: MessageId,
        message_text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = cx.new(|cx| {
            MultiBuffer::singleton(cx.new(|cx| Buffer::local(message_text.clone(), cx)), cx)
        });
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight { max_lines: 8 },
                buffer,
                None,
                false,
                window,
                cx,
            );
            editor.focus_handle(cx).focus(window);
            editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
            editor
        });
        self.editing_message = Some((
            message_id,
            EditMessageState {
                editor: editor.clone(),
            },
        ));
        cx.notify();
    }

    fn cancel_editing_message(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.editing_message.take();
        cx.notify();
    }

    fn confirm_editing_message(
        &mut self,
        _: &menu::Confirm,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((message_id, state)) = self.editing_message.take() else {
            return;
        };
        let edited_text = state.editor.read(cx).text(cx);
        self.thread.update(cx, |thread, cx| {
            thread.edit_message(message_id, Role::User, edited_text, cx);
            for message_id in self.messages_after(message_id) {
                thread.delete_message(*message_id, cx);
            }
        });

        let provider = LanguageModelRegistry::read_global(cx).active_provider();
        if provider
            .as_ref()
            .map_or(false, |provider| provider.must_accept_terms(cx))
        {
            cx.notify();
            return;
        }
        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(model) = model_registry.active_model() else {
            return;
        };

        self.thread.update(cx, |thread, cx| {
            thread.send_to_model(model, RequestKind::Chat, false, cx)
        });
        cx.notify();
    }

    fn last_user_message(&self, cx: &Context<Self>) -> Option<MessageId> {
        self.messages
            .iter()
            .rev()
            .find(|message_id| {
                self.thread
                    .read(cx)
                    .message(**message_id)
                    .map_or(false, |message| message.role == Role::User)
            })
            .cloned()
    }

    fn messages_after(&self, message_id: MessageId) -> &[MessageId] {
        self.messages
            .iter()
            .position(|id| *id == message_id)
            .map(|index| &self.messages[index + 1..])
            .unwrap_or(&[])
    }

    fn handle_cancel_click(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_editing_message(&menu::Cancel, window, cx);
    }

    fn handle_regenerate_click(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.confirm_editing_message(&menu::Confirm, window, cx);
    }

    fn render_message(&self, ix: usize, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let message_id = self.messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(markdown) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let context = self.thread.read(cx).context_for_message(message_id);
        let tool_uses = self.thread.read(cx).tool_uses_for_message(message_id);
        let colors = cx.theme().colors();

        // Don't render user messages that are just there for returning tool results.
        if message.role == Role::User && self.thread.read(cx).message_has_tool_results(message_id) {
            return Empty.into_any();
        }

        let allow_editing_message =
            message.role == Role::User && self.last_user_message(cx) == Some(message_id);

        let edit_message_editor = self
            .editing_message
            .as_ref()
            .filter(|(id, _)| *id == message_id)
            .map(|(_, state)| state.editor.clone());

        let message_content = v_flex()
            .child(
                if let Some(edit_message_editor) = edit_message_editor.clone() {
                    div()
                        .key_context("EditMessageEditor")
                        .on_action(cx.listener(Self::cancel_editing_message))
                        .on_action(cx.listener(Self::confirm_editing_message))
                        .p_2p5()
                        .child(edit_message_editor)
                } else {
                    div().p_2p5().text_ui(cx).child(markdown.clone())
                },
            )
            .when_some(context, |parent, context| {
                if !context.is_empty() {
                    parent.child(
                        h_flex().flex_wrap().gap_1().px_1p5().pb_1p5().children(
                            context
                                .into_iter()
                                .map(|context| ContextPill::added(context, false, false, None)),
                        ),
                    )
                } else {
                    parent
                }
            });

        let styled_message = match message.role {
            Role::User => v_flex()
                .id(("message-container", ix))
                .pt_2p5()
                .px_2p5()
                .child(
                    v_flex()
                        .bg(colors.editor_background)
                        .rounded_lg()
                        .border_1()
                        .border_color(colors.border)
                        .shadow_sm()
                        .child(
                            h_flex()
                                .py_1()
                                .pl_2()
                                .pr_1()
                                .bg(colors.editor_foreground.opacity(0.05))
                                .border_b_1()
                                .border_color(colors.border)
                                .justify_between()
                                .rounded_t(px(6.))
                                .child(
                                    h_flex()
                                        .gap_1p5()
                                        .child(
                                            Icon::new(IconName::PersonCircle)
                                                .size(IconSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new("You")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                )
                                .when_some(
                                    edit_message_editor.clone(),
                                    |this, edit_message_editor| {
                                        let focus_handle = edit_message_editor.focus_handle(cx);
                                        this.child(
                                            h_flex()
                                                .gap_1()
                                                .child(
                                                    Button::new("cancel-edit-message", "Cancel")
                                                        .label_size(LabelSize::Small)
                                                        .key_binding(
                                                            KeyBinding::for_action_in(
                                                                &menu::Cancel,
                                                                &focus_handle,
                                                                window,
                                                                cx,
                                                            )
                                                            .map(|kb| kb.size(rems_from_px(12.))),
                                                        )
                                                        .on_click(
                                                            cx.listener(Self::handle_cancel_click),
                                                        ),
                                                )
                                                .child(
                                                    Button::new(
                                                        "confirm-edit-message",
                                                        "Regenerate",
                                                    )
                                                    .label_size(LabelSize::Small)
                                                    .key_binding(
                                                        KeyBinding::for_action_in(
                                                            &menu::Confirm,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                        .map(|kb| kb.size(rems_from_px(12.))),
                                                    )
                                                    .on_click(
                                                        cx.listener(Self::handle_regenerate_click),
                                                    ),
                                                ),
                                        )
                                    },
                                )
                                .when(
                                    edit_message_editor.is_none() && allow_editing_message,
                                    |this| {
                                        this.child(
                                            Button::new("edit-message", "Edit")
                                                .label_size(LabelSize::Small)
                                                .on_click(cx.listener({
                                                    let message_text = message.text.clone();
                                                    move |this, _, window, cx| {
                                                        this.start_editing_message(
                                                            message_id,
                                                            message_text.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    }
                                                })),
                                        )
                                    },
                                ),
                        )
                        .child(message_content),
                ),
            Role::Assistant => div()
                .id(("message-container", ix))
                .child(message_content)
                .map(|parent| {
                    if tool_uses.is_empty() {
                        return parent;
                    }

                    parent.child(
                        v_flex().children(
                            tool_uses
                                .into_iter()
                                .map(|tool_use| self.render_tool_use(tool_use, cx)),
                        ),
                    )
                }),
            Role::System => div().id(("message-container", ix)).py_1().px_2().child(
                v_flex()
                    .bg(colors.editor_background)
                    .rounded_md()
                    .child(message_content),
            ),
        };

        styled_message.into_any()
    }

    fn render_tool_use(&self, tool_use: ToolUse, cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self
            .expanded_tool_uses
            .get(&tool_use.id)
            .copied()
            .unwrap_or_default();

        div().px_2p5().child(
            v_flex()
                .gap_1()
                .rounded_lg()
                .border_1()
                .border_color(cx.theme().colors().border)
                .child(
                    h_flex()
                        .justify_between()
                        .py_0p5()
                        .pl_1()
                        .pr_2()
                        .bg(cx.theme().colors().editor_foreground.opacity(0.02))
                        .when(is_open, |element| element.border_b_1().rounded_t(px(6.)))
                        .when(!is_open, |element| element.rounded(px(6.)))
                        .border_color(cx.theme().colors().border)
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Disclosure::new("tool-use-disclosure", is_open).on_click(
                                    cx.listener({
                                        let tool_use_id = tool_use.id.clone();
                                        move |this, _event, _window, _cx| {
                                            let is_open = this
                                                .expanded_tool_uses
                                                .entry(tool_use_id.clone())
                                                .or_insert(false);

                                            *is_open = !*is_open;
                                        }
                                    }),
                                ))
                                .child(Label::new(tool_use.name)),
                        )
                        .child(
                            Label::new(match tool_use.status {
                                ToolUseStatus::Pending => "Pending",
                                ToolUseStatus::Running => "Running",
                                ToolUseStatus::Finished(_) => "Finished",
                                ToolUseStatus::Error(_) => "Error",
                            })
                            .size(LabelSize::XSmall)
                            .buffer_font(cx),
                        ),
                )
                .map(|parent| {
                    if !is_open {
                        return parent;
                    }

                    parent.child(
                        v_flex()
                            .child(
                                v_flex()
                                    .gap_0p5()
                                    .py_1()
                                    .px_2p5()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border)
                                    .child(Label::new("Input:"))
                                    .child(Label::new(
                                        serde_json::to_string_pretty(&tool_use.input)
                                            .unwrap_or_default(),
                                    )),
                            )
                            .map(|parent| match tool_use.status {
                                ToolUseStatus::Finished(output) => parent.child(
                                    v_flex()
                                        .gap_0p5()
                                        .py_1()
                                        .px_2p5()
                                        .child(Label::new("Result:"))
                                        .child(Label::new(output)),
                                ),
                                ToolUseStatus::Error(err) => parent.child(
                                    v_flex()
                                        .gap_0p5()
                                        .py_1()
                                        .px_2p5()
                                        .child(Label::new("Error:"))
                                        .child(Label::new(err)),
                                ),
                                ToolUseStatus::Pending | ToolUseStatus::Running => parent,
                            }),
                    )
                }),
        )
    }
}

impl Render for ActiveThread {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(list(self.list_state.clone()).flex_grow())
    }
}
