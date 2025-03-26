use crate::thread::{
    LastRestoreCheckpoint, MessageId, MessageSegment, RequestKind, Thread, ThreadError,
    ThreadEvent, ThreadFeedback,
};
use crate::thread_store::ThreadStore;
use crate::tool_use::{PendingToolUseStatus, ToolUse, ToolUseStatus};
use crate::ui::{ContextPill, ToolReadyPopUp, ToolReadyPopupEvent};
use crate::AssistantPanel;
use assistant_settings::AssistantSettings;
use collections::HashMap;
use editor::{Editor, MultiBuffer};
use gpui::{
    linear_color_stop, linear_gradient, list, percentage, pulsating_between, AbsoluteLength,
    Animation, AnimationExt, AnyElement, App, ClickEvent, DefiniteLength, EdgesRefinement, Empty,
    Entity, Focusable, Hsla, Length, ListAlignment, ListOffset, ListState, MouseButton,
    ScrollHandle, Stateful, StyleRefinement, Subscription, Task, TextStyleRefinement,
    Transformation, UnderlineStyle, WeakEntity, WindowHandle,
};
use language::{Buffer, LanguageRegistry};
use language_model::{LanguageModelRegistry, LanguageModelToolUseId, Role};
use markdown::{Markdown, MarkdownStyle};
use settings::Settings as _;
use std::sync::Arc;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{prelude::*, Disclosure, IconButton, KeyBinding, Scrollbar, ScrollbarState, Tooltip};
use util::ResultExt as _;
use workspace::{OpenOptions, Workspace};

use crate::context_store::{refresh_context_store_text, ContextStore};

pub struct ActiveThread {
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<Thread>,
    context_store: Entity<ContextStore>,
    workspace: WeakEntity<Workspace>,
    save_thread_task: Option<Task<()>>,
    messages: Vec<MessageId>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    rendered_messages_by_id: HashMap<MessageId, RenderedMessage>,
    rendered_tool_use_labels: HashMap<LanguageModelToolUseId, Entity<Markdown>>,
    editing_message: Option<(MessageId, EditMessageState)>,
    expanded_tool_uses: HashMap<LanguageModelToolUseId, bool>,
    expanded_thinking_segments: HashMap<(MessageId, usize), bool>,
    last_error: Option<ThreadError>,
    pop_ups: Vec<WindowHandle<ToolReadyPopUp>>,
    _subscriptions: Vec<Subscription>,
}

struct RenderedMessage {
    language_registry: Arc<LanguageRegistry>,
    segments: Vec<RenderedMessageSegment>,
}

impl RenderedMessage {
    fn from_segments(
        segments: &[MessageSegment],
        language_registry: Arc<LanguageRegistry>,
        window: &Window,
        cx: &mut App,
    ) -> Self {
        let mut this = Self {
            language_registry,
            segments: Vec::with_capacity(segments.len()),
        };
        for segment in segments {
            this.push_segment(segment, window, cx);
        }
        this
    }

    fn append_thinking(&mut self, text: &String, window: &Window, cx: &mut App) {
        if let Some(RenderedMessageSegment::Thinking {
            content,
            scroll_handle,
        }) = self.segments.last_mut()
        {
            content.update(cx, |markdown, cx| {
                markdown.append(text, cx);
            });
            scroll_handle.scroll_to_bottom();
        } else {
            self.segments.push(RenderedMessageSegment::Thinking {
                content: render_markdown(text.into(), self.language_registry.clone(), window, cx),
                scroll_handle: ScrollHandle::default(),
            });
        }
    }

    fn append_text(&mut self, text: &String, window: &Window, cx: &mut App) {
        if let Some(RenderedMessageSegment::Text(markdown)) = self.segments.last_mut() {
            markdown.update(cx, |markdown, cx| markdown.append(text, cx));
        } else {
            self.segments
                .push(RenderedMessageSegment::Text(render_markdown(
                    SharedString::from(text),
                    self.language_registry.clone(),
                    window,
                    cx,
                )));
        }
    }

    fn push_segment(&mut self, segment: &MessageSegment, window: &Window, cx: &mut App) {
        let rendered_segment = match segment {
            MessageSegment::Thinking(text) => RenderedMessageSegment::Thinking {
                content: render_markdown(text.into(), self.language_registry.clone(), window, cx),
                scroll_handle: ScrollHandle::default(),
            },
            MessageSegment::Text(text) => RenderedMessageSegment::Text(render_markdown(
                text.into(),
                self.language_registry.clone(),
                window,
                cx,
            )),
        };
        self.segments.push(rendered_segment);
    }
}

enum RenderedMessageSegment {
    Thinking {
        content: Entity<Markdown>,
        scroll_handle: ScrollHandle,
    },
    Text(Entity<Markdown>),
}

fn render_markdown(
    text: SharedString,
    language_registry: Arc<LanguageRegistry>,
    window: &Window,
    cx: &mut App,
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

    cx.new(|cx| Markdown::new(text, markdown_style, Some(language_registry), None, cx))
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
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe_in(&thread, window, Self::handle_thread_event),
        ];

        let list_state = ListState::new(0, ListAlignment::Bottom, px(2048.), {
            let this = cx.entity().downgrade();
            move |ix, window: &mut Window, cx: &mut App| {
                this.update(cx, |this, cx| this.render_message(ix, window, cx))
                    .unwrap()
            }
        });

        let mut this = Self {
            language_registry,
            thread_store,
            thread: thread.clone(),
            context_store,
            workspace,
            save_thread_task: None,
            messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            rendered_tool_use_labels: HashMap::default(),
            expanded_tool_uses: HashMap::default(),
            expanded_thinking_segments: HashMap::default(),
            list_state: list_state.clone(),
            scrollbar_state: ScrollbarState::new(list_state),
            editing_message: None,
            last_error: None,
            pop_ups: Vec::new(),
            _subscriptions: subscriptions,
        };

        for message in thread.read(cx).messages().cloned().collect::<Vec<_>>() {
            this.push_message(&message.id, &message.segments, window, cx);

            for tool_use in thread.read(cx).tool_uses_for_message(message.id, cx) {
                this.render_tool_use_label_markdown(
                    tool_use.id.clone(),
                    tool_use.ui_text.clone(),
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
            .update(cx, |thread, cx| thread.cancel_last_completion(cx))
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
        segments: &[MessageSegment],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_len = self.messages.len();
        self.messages.push(*id);
        self.list_state.splice(old_len..old_len, 1);

        let rendered_message =
            RenderedMessage::from_segments(segments, self.language_registry.clone(), window, cx);
        self.rendered_messages_by_id.insert(*id, rendered_message);
        self.list_state.scroll_to(ListOffset {
            item_ix: old_len,
            offset_in_item: Pixels(0.0),
        });
    }

    fn edited_message(
        &mut self,
        id: &MessageId,
        segments: &[MessageSegment],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.messages.iter().position(|message_id| message_id == id) else {
            return;
        };
        self.list_state.splice(index..index + 1, 1);
        let rendered_message =
            RenderedMessage::from_segments(segments, self.language_registry.clone(), window, cx);
        self.rendered_messages_by_id.insert(*id, rendered_message);
    }

    fn deleted_message(&mut self, id: &MessageId) {
        let Some(index) = self.messages.iter().position(|message_id| message_id == id) else {
            return;
        };
        self.messages.remove(index);
        self.list_state.splice(index..index + 1, 0);
        self.rendered_messages_by_id.remove(id);
    }

    fn render_tool_use_label_markdown(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_label: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.rendered_tool_use_labels.insert(
            tool_use_id,
            render_markdown(
                tool_label.into(),
                self.language_registry.clone(),
                window,
                cx,
            ),
        );
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
            ThreadEvent::DoneStreaming => {
                if !self.thread().read(cx).is_generating() {
                    self.show_notification(
                        "Your changes have been applied.",
                        IconName::Check,
                        Color::Success,
                        window,
                        cx,
                    );
                }
            }
            ThreadEvent::ToolConfirmationNeeded => {
                self.show_notification(
                    "There's a tool confirmation needed.",
                    IconName::Info,
                    Color::Muted,
                    window,
                    cx,
                );
            }
            ThreadEvent::StreamedAssistantText(message_id, text) => {
                if let Some(rendered_message) = self.rendered_messages_by_id.get_mut(&message_id) {
                    rendered_message.append_text(text, window, cx);
                }
            }
            ThreadEvent::StreamedAssistantThinking(message_id, text) => {
                if let Some(rendered_message) = self.rendered_messages_by_id.get_mut(&message_id) {
                    rendered_message.append_thinking(text, window, cx);
                }
            }
            ThreadEvent::MessageAdded(message_id) => {
                if let Some(message_segments) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.segments.clone())
                {
                    self.push_message(message_id, &message_segments, window, cx);
                }

                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::MessageEdited(message_id) => {
                if let Some(message_segments) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.segments.clone())
                {
                    self.edited_message(message_id, &message_segments, window, cx);
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
                let tool_uses = self
                    .thread
                    .update(cx, |thread, cx| thread.use_pending_tools(cx));

                for tool_use in tool_uses {
                    self.render_tool_use_label_markdown(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        window,
                        cx,
                    );
                }
            }
            ThreadEvent::ToolFinished {
                pending_tool_use,
                canceled,
                ..
            } => {
                let canceled = *canceled;
                if let Some(tool_use) = pending_tool_use {
                    self.render_tool_use_label_markdown(
                        tool_use.id.clone(),
                        SharedString::from(tool_use.ui_text.clone()),
                        window,
                        cx,
                    );
                }

                if self.thread.read(cx).all_tools_finished() {
                    let pending_refresh_buffers = self.thread.update(cx, |thread, cx| {
                        thread.action_log().update(cx, |action_log, _cx| {
                            action_log.take_stale_buffers_in_context()
                        })
                    });

                    let context_update_task = if !pending_refresh_buffers.is_empty() {
                        let refresh_task = refresh_context_store_text(
                            self.context_store.clone(),
                            &pending_refresh_buffers,
                            cx,
                        );

                        cx.spawn(async move |this, cx| {
                            let updated_context_ids = refresh_task.await;

                            this.update(cx, |this, cx| {
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
                        cx.spawn(async move |this, cx| {
                            let updated_context = context_update_task.await?;

                            this.update(cx, |this, cx| {
                                this.thread.update(cx, |thread, cx| {
                                    thread.attach_tool_results(updated_context, cx);
                                    if !canceled {
                                        thread.send_to_model(model, RequestKind::Chat, cx);
                                    }
                                });
                            })
                        })
                        .detach();
                    }
                }
            }
            ThreadEvent::CheckpointChanged => cx.notify(),
        }
    }

    fn show_notification(
        &mut self,
        caption: impl Into<SharedString>,
        icon: IconName,
        icon_color: Color,
        window: &mut Window,
        cx: &mut Context<'_, ActiveThread>,
    ) {
        if !window.is_window_active()
            && self.pop_ups.is_empty()
            && AssistantSettings::get_global(cx).notify_when_agent_waiting
        {
            let caption = caption.into();

            for screen in cx.displays() {
                let options = ToolReadyPopUp::window_options(screen, cx);

                if let Some(screen_window) = cx
                    .open_window(options, |_, cx| {
                        cx.new(|_| ToolReadyPopUp::new(caption.clone(), icon, icon_color))
                    })
                    .log_err()
                {
                    if let Some(pop_up) = screen_window.entity(cx).log_err() {
                        cx.subscribe_in(&pop_up, window, {
                            |this, _, event, window, cx| match event {
                                ToolReadyPopupEvent::Accepted => {
                                    let handle = window.window_handle();
                                    cx.activate(true); // Switch back to the Zed application

                                    let workspace_handle = this.workspace.clone();

                                    // If there are multiple Zed windows, activate the correct one.
                                    cx.defer(move |cx| {
                                        handle
                                            .update(cx, |_view, window, _cx| {
                                                window.activate_window();

                                                if let Some(workspace) = workspace_handle.upgrade()
                                                {
                                                    workspace.update(_cx, |workspace, cx| {
                                                        workspace.focus_panel::<AssistantPanel>(
                                                            window, cx,
                                                        );
                                                    });
                                                }
                                            })
                                            .log_err();
                                    });

                                    this.dismiss_notifications(cx);
                                }
                                ToolReadyPopupEvent::Dismissed => {
                                    this.dismiss_notifications(cx);
                                }
                            }
                        })
                        .detach();

                        self.pop_ups.push(screen_window);
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
        self.save_thread_task = Some(cx.spawn(async move |this, cx| {
            let task = this
                .update(cx, |this, cx| {
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
        message_segments: &[MessageSegment],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // User message should always consist of a single text segment,
        // therefore we can skip returning early if it's not a text segment.
        let Some(MessageSegment::Text(message_text)) = message_segments.first() else {
            return;
        };

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
            thread.edit_message(
                message_id,
                Role::User,
                vec![MessageSegment::Text(edited_text)],
                cx,
            );
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

    fn handle_feedback_click(
        &mut self,
        feedback: ThreadFeedback,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let report = self
            .thread
            .update(cx, |thread, cx| thread.report_feedback(feedback, cx));

        let this = cx.entity().downgrade();
        cx.spawn(async move |_, cx| {
            report.await?;
            this.update(cx, |_this, cx| cx.notify())
        })
        .detach_and_log_err(cx);
    }

    fn render_message(&self, ix: usize, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let message_id = self.messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(rendered_message) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let thread = self.thread.read(cx);
        // Get all the data we need from thread before we start using it in closures
        let checkpoint = thread.checkpoint_for_message(message_id);
        let context = thread.context_for_message(message_id);
        let tool_uses = thread.tool_uses_for_message(message_id, cx);

        // Don't render user messages that are just there for returning tool results.
        if message.role == Role::User && thread.message_has_tool_results(message_id) {
            return Empty.into_any();
        }

        let allow_editing_message =
            message.role == Role::User && self.last_user_message(cx) == Some(message_id);

        let edit_message_editor = self
            .editing_message
            .as_ref()
            .filter(|(id, _)| *id == message_id)
            .map(|(_, state)| state.editor.clone());

        let first_message = ix == 0;
        let is_last_message = ix == self.messages.len() - 1;

        let colors = cx.theme().colors();
        let active_color = colors.element_active;
        let editor_bg_color = colors.editor_background;
        let bg_user_message_header = editor_bg_color.blend(active_color.opacity(0.25));

        let feedback_container = h_flex().pt_2().pb_4().px_4().gap_1().justify_between();
        let feedback_items = match self.thread.read(cx).feedback() {
            Some(feedback) => feedback_container
                .child(
                    Label::new(match feedback {
                        ThreadFeedback::Positive => "Thanks for your feedback!",
                        ThreadFeedback::Negative => {
                            "We appreciate your feedback and will use it to improve."
                        }
                    })
                    .color(Color::Muted)
                    .size(LabelSize::XSmall),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            IconButton::new("feedback-thumbs-up", IconName::ThumbsUp)
                                .icon_size(IconSize::XSmall)
                                .icon_color(match feedback {
                                    ThreadFeedback::Positive => Color::Accent,
                                    ThreadFeedback::Negative => Color::Ignored,
                                })
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Helpful Response"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        ThreadFeedback::Positive,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            IconButton::new("feedback-thumbs-down", IconName::ThumbsDown)
                                .icon_size(IconSize::XSmall)
                                .icon_color(match feedback {
                                    ThreadFeedback::Positive => Color::Ignored,
                                    ThreadFeedback::Negative => Color::Accent,
                                })
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Not Helpful"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        ThreadFeedback::Negative,
                                        window,
                                        cx,
                                    );
                                })),
                        ),
                )
                .into_any_element(),
            None => feedback_container
                .child(
                    Label::new(
                        "Rating the thread sends all of your current conversation to the Zed team.",
                    )
                    .color(Color::Muted)
                    .size(LabelSize::XSmall),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            IconButton::new("feedback-thumbs-up", IconName::ThumbsUp)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Ignored)
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Helpful Response"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        ThreadFeedback::Positive,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            IconButton::new("feedback-thumbs-down", IconName::ThumbsDown)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Ignored)
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Not Helpful"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        ThreadFeedback::Negative,
                                        window,
                                        cx,
                                    );
                                })),
                        ),
                )
                .into_any_element(),
        };

        let message_content = v_flex()
            .gap_1p5()
            .child(
                if let Some(edit_message_editor) = edit_message_editor.clone() {
                    div()
                        .key_context("EditMessageEditor")
                        .on_action(cx.listener(Self::cancel_editing_message))
                        .on_action(cx.listener(Self::confirm_editing_message))
                        .min_h_6()
                        .child(edit_message_editor)
                } else {
                    div()
                        .min_h_6()
                        .text_ui(cx)
                        .child(self.render_message_content(message_id, rendered_message, cx))
                },
            )
            .when_some(context, |parent, context| {
                if !context.is_empty() {
                    parent.child(
                        h_flex().flex_wrap().gap_1().children(
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
                .map(|this| {
                    if first_message {
                        this.pt_2()
                    } else {
                        this.pt_4()
                    }
                })
                .pb_4()
                .pl_2()
                .pr_2p5()
                .child(
                    v_flex()
                        .bg(colors.editor_background)
                        .rounded_lg()
                        .border_1()
                        .border_color(colors.border)
                        .shadow_md()
                        .child(
                            h_flex()
                                .py_1()
                                .pl_2()
                                .pr_1()
                                .bg(bg_user_message_header)
                                .border_b_1()
                                .border_color(colors.border)
                                .justify_between()
                                .rounded_t_md()
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
                                .child(
                                    h_flex()
                                        // DL: To double-check whether we want to fully remove
                                        // the editing feature from meassages. Checkpoint sort of
                                        // solve the same problem.
                                        .invisible()
                                        .gap_1()
                                        .when_some(
                                            edit_message_editor.clone(),
                                            |this, edit_message_editor| {
                                                let focus_handle =
                                                    edit_message_editor.focus_handle(cx);
                                                this.child(
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
                                                            let message_segments =
                                                                message.segments.clone();
                                                            move |this, _, window, cx| {
                                                                this.start_editing_message(
                                                                    message_id,
                                                                    &message_segments,
                                                                    window,
                                                                    cx,
                                                                );
                                                            }
                                                        })),
                                                )
                                            },
                                        ),
                                ),
                        )
                        .child(div().p_2().child(message_content)),
                ),
            Role::Assistant => v_flex()
                .id(("message-container", ix))
                .ml_2()
                .pl_2()
                .pr_4()
                .border_l_1()
                .border_color(cx.theme().colors().border_variant)
                .child(message_content)
                .when(!tool_uses.is_empty(), |parent| {
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
                    .rounded_sm()
                    .child(div().p_4().child(message_content)),
            ),
        };

        v_flex()
            .w_full()
            .when(first_message, |parent| {
                parent.child(self.render_rules_item(cx))
            })
            .when_some(checkpoint, |parent, checkpoint| {
                let mut is_pending = false;
                let mut error = None;
                if let Some(last_restore_checkpoint) =
                    self.thread.read(cx).last_restore_checkpoint()
                {
                    if last_restore_checkpoint.message_id() == message_id {
                        match last_restore_checkpoint {
                            LastRestoreCheckpoint::Pending { .. } => is_pending = true,
                            LastRestoreCheckpoint::Error { error: err, .. } => {
                                error = Some(err.clone());
                            }
                        }
                    }
                }

                let restore_checkpoint_button =
                    Button::new(("restore-checkpoint", ix), "Restore Checkpoint")
                        .icon(if error.is_some() {
                            IconName::XCircle
                        } else {
                            IconName::Undo
                        })
                        .icon_size(IconSize::XSmall)
                        .icon_position(IconPosition::Start)
                        .icon_color(if error.is_some() {
                            Some(Color::Error)
                        } else {
                            None
                        })
                        .label_size(LabelSize::XSmall)
                        .disabled(is_pending)
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.thread.update(cx, |thread, cx| {
                                thread
                                    .restore_checkpoint(checkpoint.clone(), cx)
                                    .detach_and_log_err(cx);
                            });
                        }));

                let restore_checkpoint_button = if is_pending {
                    restore_checkpoint_button
                        .with_animation(
                            ("pulsating-restore-checkpoint-button", ix),
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.6, 1.)),
                            |label, delta| label.alpha(delta),
                        )
                        .into_any_element()
                } else if let Some(error) = error {
                    restore_checkpoint_button
                        .tooltip(Tooltip::text(error.to_string()))
                        .into_any_element()
                } else {
                    restore_checkpoint_button.into_any_element()
                };

                parent.child(
                    h_flex()
                        .pt_2p5()
                        .px_2p5()
                        .w_full()
                        .gap_1()
                        .child(ui::Divider::horizontal())
                        .child(restore_checkpoint_button)
                        .child(ui::Divider::horizontal()),
                )
            })
            .child(styled_message)
            .when(
                is_last_message && !self.thread.read(cx).is_generating(),
                |parent| parent.child(feedback_items),
            )
            .into_any()
    }

    fn render_message_content(
        &self,
        message_id: MessageId,
        rendered_message: &RenderedMessage,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let pending_thinking_segment_index = rendered_message
            .segments
            .iter()
            .enumerate()
            .last()
            .filter(|(_, segment)| matches!(segment, RenderedMessageSegment::Thinking { .. }))
            .map(|(index, _)| index);

        div()
            .text_ui(cx)
            .gap_2()
            .children(
                rendered_message.segments.iter().enumerate().map(
                    |(index, segment)| match segment {
                        RenderedMessageSegment::Thinking {
                            content,
                            scroll_handle,
                        } => self
                            .render_message_thinking_segment(
                                message_id,
                                index,
                                content.clone(),
                                &scroll_handle,
                                Some(index) == pending_thinking_segment_index,
                                cx,
                            )
                            .into_any_element(),
                        RenderedMessageSegment::Text(markdown) => {
                            div().child(markdown.clone()).into_any_element()
                        }
                    },
                ),
            )
    }

    fn tool_card_border_color(&self, cx: &Context<Self>) -> Hsla {
        cx.theme().colors().border.opacity(0.5)
    }

    fn tool_card_header_bg(&self, cx: &Context<Self>) -> Hsla {
        cx.theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025))
    }

    fn render_message_thinking_segment(
        &self,
        message_id: MessageId,
        ix: usize,
        markdown: Entity<Markdown>,
        scroll_handle: &ScrollHandle,
        pending: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_open = self
            .expanded_thinking_segments
            .get(&(message_id, ix))
            .copied()
            .unwrap_or_default();

        let editor_bg = cx.theme().colors().editor_background;

        div().py_2().child(
            v_flex()
                .rounded_lg()
                .border_1()
                .border_color(self.tool_card_border_color(cx))
                .child(
                    h_flex()
                        .group("disclosure-header")
                        .justify_between()
                        .py_1()
                        .px_2()
                        .bg(self.tool_card_header_bg(cx))
                        .map(|this| {
                            if pending || is_open {
                                this.rounded_t_md()
                                    .border_b_1()
                                    .border_color(self.tool_card_border_color(cx))
                            } else {
                                this.rounded_md()
                            }
                        })
                        .child(
                            h_flex()
                                .gap_1p5()
                                .child(
                                    Icon::new(IconName::Brain)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted),
                                )
                                .child({
                                    if pending {
                                        Label::new("Thinking…")
                                            .size(LabelSize::Small)
                                            .buffer_font(cx)
                                            .with_animation(
                                                "pulsating-label",
                                                Animation::new(Duration::from_secs(2))
                                                    .repeat()
                                                    .with_easing(pulsating_between(0.4, 0.8)),
                                                |label, delta| label.alpha(delta),
                                            )
                                            .into_any_element()
                                    } else {
                                        Label::new("Thought Process")
                                            .size(LabelSize::Small)
                                            .buffer_font(cx)
                                            .into_any_element()
                                    }
                                }),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    div().visible_on_hover("disclosure-header").child(
                                        Disclosure::new("thinking-disclosure", is_open)
                                            .opened_icon(IconName::ChevronUp)
                                            .closed_icon(IconName::ChevronDown)
                                            .on_click(cx.listener({
                                                move |this, _event, _window, _cx| {
                                                    let is_open = this
                                                        .expanded_thinking_segments
                                                        .entry((message_id, ix))
                                                        .or_insert(false);

                                                    *is_open = !*is_open;
                                                }
                                            })),
                                    ),
                                )
                                .child({
                                    let (icon_name, color, animated) = if pending {
                                        (IconName::ArrowCircle, Color::Accent, true)
                                    } else {
                                        (IconName::Check, Color::Success, false)
                                    };

                                    let icon =
                                        Icon::new(icon_name).color(color).size(IconSize::Small);

                                    if animated {
                                        icon.with_animation(
                                            "arrow-circle",
                                            Animation::new(Duration::from_secs(2)).repeat(),
                                            |icon, delta| {
                                                icon.transform(Transformation::rotate(percentage(
                                                    delta,
                                                )))
                                            },
                                        )
                                        .into_any_element()
                                    } else {
                                        icon.into_any_element()
                                    }
                                }),
                        ),
                )
                .when(pending && !is_open, |this| {
                    let gradient_overlay = div()
                        .rounded_b_lg()
                        .h_20()
                        .absolute()
                        .w_full()
                        .bottom_0()
                        .left_0()
                        .bg(linear_gradient(
                            180.,
                            linear_color_stop(editor_bg, 1.),
                            linear_color_stop(editor_bg.opacity(0.2), 0.),
                        ));

                    this.child(
                        div()
                            .relative()
                            .bg(editor_bg)
                            .rounded_b_lg()
                            .child(
                                div()
                                    .id(("thinking-content", ix))
                                    .p_2()
                                    .h_20()
                                    .track_scroll(scroll_handle)
                                    .text_ui_sm(cx)
                                    .child(markdown.clone())
                                    .overflow_hidden(),
                            )
                            .child(gradient_overlay),
                    )
                })
                .when(is_open, |this| {
                    this.child(
                        div()
                            .id(("thinking-content", ix))
                            .h_full()
                            .p_2()
                            .rounded_b_lg()
                            .bg(editor_bg)
                            .text_ui_sm(cx)
                            .child(markdown.clone()),
                    )
                }),
        )
    }

    fn render_tool_use(&self, tool_use: ToolUse, cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self
            .expanded_tool_uses
            .get(&tool_use.id)
            .copied()
            .unwrap_or_default();

        div().py_2().child(
            v_flex()
                .rounded_lg()
                .border_1()
                .border_color(self.tool_card_border_color(cx))
                .overflow_hidden()
                .child(
                    h_flex()
                        .group("disclosure-header")
                        .relative()
                        .gap_1p5()
                        .justify_between()
                        .py_1()
                        .px_2()
                        .bg(self.tool_card_header_bg(cx))
                        .map(|element| {
                            if is_open {
                                element.border_b_1().rounded_t_md()
                            } else {
                                element.rounded_md()
                            }
                        })
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            h_flex()
                                .id("tool-label-container")
                                .relative()
                                .gap_1p5()
                                .max_w_full()
                                .overflow_x_scroll()
                                .child(
                                    Icon::new(tool_use.icon)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted),
                                )
                                .child(h_flex().pr_8().text_ui_sm(cx).children(
                                    self.rendered_tool_use_labels.get(&tool_use.id).cloned(),
                                )),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    div().visible_on_hover("disclosure-header").child(
                                        Disclosure::new("tool-use-disclosure", is_open)
                                            .opened_icon(IconName::ChevronUp)
                                            .closed_icon(IconName::ChevronDown)
                                            .on_click(cx.listener({
                                                let tool_use_id = tool_use.id.clone();
                                                move |this, _event, _window, _cx| {
                                                    let is_open = this
                                                        .expanded_tool_uses
                                                        .entry(tool_use_id.clone())
                                                        .or_insert(false);

                                                    *is_open = !*is_open;
                                                }
                                            })),
                                    ),
                                )
                                .child({
                                    let (icon_name, color, animated) = match &tool_use.status {
                                        ToolUseStatus::Pending
                                        | ToolUseStatus::NeedsConfirmation => {
                                            (IconName::Warning, Color::Warning, false)
                                        }
                                        ToolUseStatus::Running => {
                                            (IconName::ArrowCircle, Color::Accent, true)
                                        }
                                        ToolUseStatus::Finished(_) => {
                                            (IconName::Check, Color::Success, false)
                                        }
                                        ToolUseStatus::Error(_) => {
                                            (IconName::Close, Color::Error, false)
                                        }
                                    };

                                    let icon =
                                        Icon::new(icon_name).color(color).size(IconSize::Small);

                                    if animated {
                                        icon.with_animation(
                                            "arrow-circle",
                                            Animation::new(Duration::from_secs(2)).repeat(),
                                            |icon, delta| {
                                                icon.transform(Transformation::rotate(percentage(
                                                    delta,
                                                )))
                                            },
                                        )
                                        .into_any_element()
                                    } else {
                                        icon.into_any_element()
                                    }
                                }),
                        )
                        .child(div().h_full().absolute().w_8().bottom_0().right_12().bg(
                            linear_gradient(
                                90.,
                                linear_color_stop(self.tool_card_header_bg(cx), 1.),
                                linear_color_stop(self.tool_card_header_bg(cx).opacity(0.2), 0.),
                            ),
                        )),
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
                                    .border_color(self.tool_card_border_color(cx))
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
                                ToolUseStatus::NeedsConfirmation => container.child(
                                    content_container().child(
                                        Label::new("Asking Permission")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .buffer_font(cx),
                                    ),
                                ),
                            }),
                    )
                }),
        )
    }

    fn render_rules_item(&self, cx: &Context<Self>) -> AnyElement {
        let Some(system_prompt_context) = self.thread.read(cx).system_prompt_context().as_ref()
        else {
            return div().into_any();
        };

        let rules_files = system_prompt_context
            .worktrees
            .iter()
            .filter_map(|worktree| worktree.rules_file.as_ref())
            .collect::<Vec<_>>();

        let label_text = match rules_files.as_slice() {
            &[] => return div().into_any(),
            &[rules_file] => {
                format!("Using {:?} file", rules_file.rel_path)
            }
            rules_files => {
                format!("Using {} rules files", rules_files.len())
            }
        };

        div()
            .pt_1()
            .px_2p5()
            .child(
                h_flex()
                    .w_full()
                    .gap_0p5()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                Icon::new(IconName::File)
                                    .size(IconSize::XSmall)
                                    .color(Color::Disabled),
                            )
                            .child(
                                Label::new(label_text)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                                    .buffer_font(cx),
                            ),
                    )
                    .child(
                        IconButton::new("open-rule", IconName::ArrowUpRightAlt)
                            .shape(ui::IconButtonShape::Square)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Ignored)
                            .on_click(cx.listener(Self::handle_open_rules))
                            .tooltip(Tooltip::text("View Rules")),
                    ),
            )
            .into_any()
    }

    fn handle_allow_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(PendingToolUseStatus::NeedsConfirmation(c)) = self
            .thread
            .read(cx)
            .pending_tool(&tool_use_id)
            .map(|tool_use| tool_use.status.clone())
        {
            self.thread.update(cx, |thread, cx| {
                thread.run_tool(
                    c.tool_use_id.clone(),
                    c.ui_text.clone(),
                    c.input.clone(),
                    &c.messages,
                    c.tool.clone(),
                    cx,
                );
            });
        }
    }

    fn handle_deny_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread.update(cx, |thread, cx| {
            thread.deny_tool_use(tool_use_id, cx);
        });
    }

    fn handle_open_rules(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(system_prompt_context) = self.thread.read(cx).system_prompt_context().as_ref()
        else {
            return;
        };

        let abs_paths = system_prompt_context
            .worktrees
            .iter()
            .flat_map(|worktree| worktree.rules_file.as_ref())
            .map(|rules_file| rules_file.abs_path.to_path_buf())
            .collect::<Vec<_>>();

        if let Ok(task) = self.workspace.update(cx, move |workspace, cx| {
            // TODO: Open a multibuffer instead? In some cases this doesn't make the set of rules
            // files clear. For example, if rules file 1 is already open but rules file 2 is not,
            // this would open and focus rules file 2 in a tab that is not next to rules file 1.
            workspace.open_paths(abs_paths, OpenOptions::default(), None, window, cx)
        }) {
            task.detach();
        }
    }

    fn render_confirmations<'a>(
        &'a mut self,
        cx: &'a mut Context<Self>,
    ) -> impl Iterator<Item = AnyElement> + 'a {
        let thread = self.thread.read(cx);

        thread
            .tools_needing_confirmation()
            .map(|tool| {
                div()
                    .m_3()
                    .p_2()
                    .bg(cx.theme().colors().editor_background)
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_lg()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                v_flex()
                                    .gap_0p5()
                                    .child(
                                        Label::new("The agent wants to run this action:")
                                            .color(Color::Muted),
                                    )
                                    .child(div().p_3().child(Label::new(&tool.ui_text))),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child({
                                        let tool_id = tool.id.clone();
                                        Button::new("allow-tool-action", "Allow").on_click(
                                            cx.listener(move |this, event, window, cx| {
                                                this.handle_allow_tool(
                                                    tool_id.clone(),
                                                    event,
                                                    window,
                                                    cx,
                                                )
                                            }),
                                        )
                                    })
                                    .child({
                                        let tool_id = tool.id.clone();
                                        Button::new("deny-tool", "Deny").on_click(cx.listener(
                                            move |this, event, window, cx| {
                                                this.handle_deny_tool(
                                                    tool_id.clone(),
                                                    event,
                                                    window,
                                                    cx,
                                                )
                                            },
                                        ))
                                    }),
                            )
                            .child(
                                Label::new("Note: A future release will introduce a way to remember your answers to these. In the meantime, you can avoid these prompts by adding \"assistant\": { \"always_allow_tool_actions\": true } to your settings.json.")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .into_any()
            })
    }

    fn dismiss_notifications(&mut self, cx: &mut Context<'_, ActiveThread>) {
        for window in self.pop_ups.drain(..) {
            window
                .update(cx, |_, window, _| {
                    window.remove_window();
                })
                .ok();
        }
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .occlude()
            .id("active-thread-scrollbar")
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
            .children(Scrollbar::vertical(self.scrollbar_state.clone()))
    }
}

impl Render for ActiveThread {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .relative()
            .child(list(self.list_state.clone()).flex_grow())
            .children(self.render_confirmations(cx))
            .child(self.render_vertical_scrollbar(cx))
    }
}
