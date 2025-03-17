use crate::thread::{MessageId, RequestKind, Thread, ThreadError, ThreadEvent};
use crate::thread_store::ThreadStore;
use crate::tool_use::{ToolUse, ToolUseStatus};
use crate::ui::ContextPill;
use collections::HashMap;
use editor::{Editor, MultiBuffer};
use gpui::{
    list, percentage, AbsoluteLength, Animation, AnimationExt, AnyElement, App, ClickEvent,
    DefiniteLength, EdgesRefinement, Empty, Entity, Focusable, Length, ListAlignment, ListOffset,
    ListState, StyleRefinement, Subscription, Task, TextStyleRefinement, Transformation,
    UnderlineStyle,
};
use language::{Buffer, LanguageRegistry};
use language_model::{LanguageModelRegistry, LanguageModelToolUseId, Role};
use markdown::{Markdown, MarkdownStyle};
use scripting_tool::{ScriptingTool, ScriptingToolInput};
use settings::Settings as _;
use std::sync::Arc;
use std::time::Duration;
use theme::ThemeSettings;
use ui::Color;
use ui::{prelude::*, Disclosure, KeyBinding};
use util::ResultExt as _;

use crate::context_store::{refresh_context_store_text, ContextStore};

pub struct ActiveThread {
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<Thread>,
    context_store: Entity<ContextStore>,
    save_thread_task: Option<Task<()>>,
    messages: Vec<MessageId>,
    list_state: ListState,
    rendered_messages_by_id: HashMap<MessageId, Entity<Markdown>>,
    rendered_scripting_tool_uses: HashMap<LanguageModelToolUseId, Entity<Markdown>>,
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
        language_registry: Arc<LanguageRegistry>,
        context_store: Entity<ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe_in(&thread, window, Self::handle_thread_event),
        ];

        let mut this = Self {
            language_registry,
            thread_store,
            thread: thread.clone(),
            context_store,
            save_thread_task: None,
            messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            rendered_scripting_tool_uses: HashMap::default(),
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

            for tool_use in thread.read(cx).scripting_tool_uses_for_message(message.id) {
                this.render_scripting_tool_use_markdown(
                    tool_use.id.clone(),
                    tool_use.name.as_ref(),
                    tool_use.input.clone(),
                    window,
                    cx,
                );
            }
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
            font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
            font_features: Some(theme_settings.ui_font.features.clone()),
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

    /// Renders the input of a scripting tool use to Markdown.
    ///
    /// Does nothing if the tool use does not correspond to the scripting tool.
    fn render_scripting_tool_use_markdown(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: &str,
        tool_input: serde_json::Value,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if tool_name != ScriptingTool::NAME {
            return;
        }

        let lua_script = serde_json::from_value::<ScriptingToolInput>(tool_input)
            .map(|input| input.lua_script)
            .unwrap_or_default();

        let lua_script =
            self.render_markdown(format!("```lua\n{lua_script}\n```").into(), window, cx);

        self.rendered_scripting_tool_uses
            .insert(tool_use_id, lua_script);
    }

    fn handle_thread_event(
        &mut self,
        _thread: &Entity<Thread>,
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
            ThreadEvent::DoneStreaming => {}
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
                self.thread.update(cx, |thread, cx| {
                    thread.use_pending_tools(cx);
                });
            }
            ThreadEvent::ToolFinished {
                pending_tool_use, ..
            } => {
                if let Some(tool_use) = pending_tool_use {
                    self.render_scripting_tool_use_markdown(
                        tool_use.id.clone(),
                        tool_use.name.as_ref(),
                        tool_use.input.clone(),
                        window,
                        cx,
                    );
                }

                if self.thread.read(cx).all_tools_finished() {
                    let pending_refresh_buffers = self.thread.update(cx, |thread, cx| {
                        thread.action_log().update(cx, |action_log, _cx| {
                            action_log.take_pending_refresh_buffers()
                        })
                    });

                    let context_update_task = if !pending_refresh_buffers.is_empty() {
                        let refresh_task = refresh_context_store_text(
                            self.context_store.clone(),
                            &pending_refresh_buffers,
                            cx,
                        );

                        cx.spawn(|this, mut cx| async move {
                            let updated_context_ids = refresh_task.await;

                            this.update(&mut cx, |this, cx| {
                                this.context_store.read_with(cx, |context_store, cx| {
                                    context_store
                                        .context()
                                        .iter()
                                        .filter(|context| {
                                            updated_context_ids.contains(&context.id())
                                        })
                                        .flat_map(|context| context.snapshot(cx))
                                        .collect()
                                })
                            })
                        })
                    } else {
                        Task::ready(anyhow::Ok(Vec::new()))
                    };

                    let model_registry = LanguageModelRegistry::read_global(cx);
                    if let Some(model) = model_registry.active_model() {
                        cx.spawn(|this, mut cx| async move {
                            let updated_context = context_update_task.await?;

                            this.update(&mut cx, |this, cx| {
                                this.thread.update(cx, |thread, cx| {
                                    thread.send_tool_results_to_model(model, updated_context, cx);
                                });
                            })
                        })
                        .detach();
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
            thread.send_to_model(model, RequestKind::Chat, cx)
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

        let thread = self.thread.read(cx);
        // Get all the data we need from thread before we start using it in closures
        let context = thread.context_for_message(message_id);
        let tool_uses = thread.tool_uses_for_message(message_id);
        let scripting_tool_uses = thread.scripting_tool_uses_for_message(message_id);

        // Don't render user messages that are just there for returning tool results.
        if message.role == Role::User
            && (thread.message_has_tool_results(message_id)
                || thread.message_has_scripting_tool_results(message_id))
        {
            return Empty.into_any();
        }

        let allow_editing_message =
            message.role == Role::User && self.last_user_message(cx) == Some(message_id);

        let edit_message_editor = self
            .editing_message
            .as_ref()
            .filter(|(id, _)| *id == message_id)
            .map(|(_, state)| state.editor.clone());

        let colors = cx.theme().colors();

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
            Role::Assistant => {
                v_flex()
                    .id(("message-container", ix))
                    .child(message_content)
                    .when(
                        !tool_uses.is_empty() || !scripting_tool_uses.is_empty(),
                        |parent| {
                            parent.child(
                                v_flex()
                                    .children(
                                        tool_uses
                                            .into_iter()
                                            .map(|tool_use| self.render_tool_use(tool_use, cx)),
                                    )
                                    .children(scripting_tool_uses.into_iter().map(|tool_use| {
                                        self.render_scripting_tool_use(tool_use, cx)
                                    })),
                            )
                        },
                    )
            }
            Role::System => div().id(("message-container", ix)).py_1().px_2().child(
                v_flex()
                    .bg(colors.editor_background)
                    .rounded_sm()
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

        let lighter_border = cx.theme().colors().border.opacity(0.5);

        div().px_2p5().child(
            v_flex()
                .rounded_lg()
                .border_1()
                .border_color(lighter_border)
                .child(
                    h_flex()
                        .justify_between()
                        .py_1()
                        .pl_1()
                        .pr_2()
                        .bg(cx.theme().colors().editor_foreground.opacity(0.025))
                        .map(|element| {
                            if is_open {
                                element.border_b_1().rounded_t_md()
                            } else {
                                element.rounded_md()
                            }
                        })
                        .border_color(lighter_border)
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
                                .child(
                                    Label::new(tool_use.name)
                                        .size(LabelSize::Small)
                                        .buffer_font(cx),
                                ),
                        )
                        .child({
                            let (icon_name, color, animated) = match &tool_use.status {
                                ToolUseStatus::Pending => {
                                    (IconName::Warning, Color::Warning, false)
                                }
                                ToolUseStatus::Running => {
                                    (IconName::ArrowCircle, Color::Accent, true)
                                }
                                ToolUseStatus::Finished(_) => {
                                    (IconName::Check, Color::Success, false)
                                }
                                ToolUseStatus::Error(_) => (IconName::Close, Color::Error, false),
                            };

                            let icon = Icon::new(icon_name).color(color).size(IconSize::Small);

                            if animated {
                                icon.with_animation(
                                    "arrow-circle",
                                    Animation::new(Duration::from_secs(2)).repeat(),
                                    |icon, delta| {
                                        icon.transform(Transformation::rotate(percentage(delta)))
                                    },
                                )
                                .into_any_element()
                            } else {
                                icon.into_any_element()
                            }
                        }),
                )
                .map(|parent| {
                    if !is_open {
                        return parent;
                    }

                    let content_container = || v_flex().py_1().gap_0p5().px_2p5();

                    parent.child(
                        v_flex()
                            .gap_1()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_b_lg()
                            .child(
                                content_container()
                                    .border_b_1()
                                    .border_color(lighter_border)
                                    .child(
                                        Label::new("Input")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                            .buffer_font(cx),
                                    )
                                    .child(
                                        Label::new(
                                            serde_json::to_string_pretty(&tool_use.input)
                                                .unwrap_or_default(),
                                        )
                                        .size(LabelSize::Small)
                                        .buffer_font(cx),
                                    ),
                            )
                            .map(|container| match tool_use.status {
                                ToolUseStatus::Finished(output) => container.child(
                                    content_container()
                                        .child(
                                            Label::new("Result")
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted)
                                                .buffer_font(cx),
                                        )
                                        .child(
                                            Label::new(output)
                                                .size(LabelSize::Small)
                                                .buffer_font(cx),
                                        ),
                                ),
                                ToolUseStatus::Running => container.child(
                                    content_container().child(
                                        h_flex()
                                            .gap_1()
                                            .pb_1()
                                            .child(
                                                Icon::new(IconName::ArrowCircle)
                                                    .size(IconSize::Small)
                                                    .color(Color::Accent)
                                                    .with_animation(
                                                        "arrow-circle",
                                                        Animation::new(Duration::from_secs(2))
                                                            .repeat(),
                                                        |icon, delta| {
                                                            icon.transform(Transformation::rotate(
                                                                percentage(delta),
                                                            ))
                                                        },
                                                    ),
                                            )
                                            .child(
                                                Label::new("Running…")
                                                    .size(LabelSize::XSmall)
                                                    .color(Color::Muted)
                                                    .buffer_font(cx),
                                            ),
                                    ),
                                ),
                                ToolUseStatus::Error(err) => container.child(
                                    content_container()
                                        .child(
                                            Label::new("Error")
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted)
                                                .buffer_font(cx),
                                        )
                                        .child(
                                            Label::new(err).size(LabelSize::Small).buffer_font(cx),
                                        ),
                                ),
                                ToolUseStatus::Pending => container,
                            }),
                    )
                }),
        )
    }

    fn render_scripting_tool_use(
        &self,
        tool_use: ToolUse,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
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
                        .map(|element| {
                            if is_open {
                                element.border_b_1().rounded_t_md()
                            } else {
                                element.rounded_md()
                            }
                        })
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

                    let lua_script_markdown =
                        self.rendered_scripting_tool_uses.get(&tool_use.id).cloned();

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
                                    .map(|parent| {
                                        if let Some(markdown) = lua_script_markdown {
                                            parent.child(markdown)
                                        } else {
                                            parent.child(Label::new(
                                                "Failed to render script input to Markdown",
                                            ))
                                        }
                                    }),
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
