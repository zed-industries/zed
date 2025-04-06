use crate::AssistantPanel;
use crate::context::{AssistantContext, ContextId};
use crate::context_picker::MentionLink;
use crate::thread::{
    LastRestoreCheckpoint, MessageId, MessageSegment, RequestKind, Thread, ThreadError,
    ThreadEvent, ThreadFeedback,
};
use crate::thread_store::ThreadStore;
use crate::tool_use::{PendingToolUseStatus, ToolUse, ToolUseStatus};
use crate::ui::{AddedContext, AgentNotification, AgentNotificationEvent, ContextPill};
use anyhow::Context as _;
use assistant_settings::{AssistantSettings, NotifyWhenAgentWaiting};
use collections::HashMap;
use editor::scroll::Autoscroll;
use editor::{Editor, MultiBuffer};
use gpui::{
    AbsoluteLength, Animation, AnimationExt, AnyElement, App, ClickEvent, DefiniteLength,
    EdgesRefinement, Empty, Entity, Focusable, Hsla, Length, ListAlignment, ListState, MouseButton,
    PlatformDisplay, ScrollHandle, Stateful, StyleRefinement, Subscription, Task,
    TextStyleRefinement, Transformation, UnderlineStyle, WeakEntity, WindowHandle,
    linear_color_stop, linear_gradient, list, percentage, pulsating_between,
};
use language::{Buffer, LanguageRegistry};
use language_model::{ConfiguredModel, LanguageModelRegistry, LanguageModelToolUseId, Role};
use markdown::{Markdown, MarkdownStyle};
use project::ProjectItem as _;
use settings::{Settings as _, update_settings_file};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use text::ToPoint;
use theme::ThemeSettings;
use ui::{Disclosure, IconButton, KeyBinding, Scrollbar, ScrollbarState, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{OpenOptions, Workspace};

use crate::context_store::ContextStore;

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
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    rendered_messages_by_id: HashMap<MessageId, RenderedMessage>,
    rendered_tool_uses: HashMap<LanguageModelToolUseId, RenderedToolUse>,
    editing_message: Option<(MessageId, EditMessageState)>,
    expanded_tool_uses: HashMap<LanguageModelToolUseId, bool>,
    expanded_thinking_segments: HashMap<(MessageId, usize), bool>,
    last_error: Option<ThreadError>,
    notifications: Vec<WindowHandle<AgentNotification>>,
    _subscriptions: Vec<Subscription>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    feedback_message_editor: Option<Entity<Editor>>,
}

struct RenderedMessage {
    language_registry: Arc<LanguageRegistry>,
    segments: Vec<RenderedMessageSegment>,
}

#[derive(Clone)]
struct RenderedToolUse {
    label: Entity<Markdown>,
    input: Entity<Markdown>,
    output: Entity<Markdown>,
}

impl RenderedMessage {
    fn from_segments(
        segments: &[MessageSegment],
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        window: &Window,
        cx: &mut App,
    ) -> Self {
        let mut this = Self {
            language_registry,
            segments: Vec::with_capacity(segments.len()),
        };
        for segment in segments {
            this.push_segment(segment, workspace.clone(), window, cx);
        }
        this
    }

    fn append_thinking(
        &mut self,
        text: &String,
        workspace: WeakEntity<Workspace>,
        window: &Window,
        cx: &mut App,
    ) {
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
                content: render_markdown(
                    text.into(),
                    self.language_registry.clone(),
                    workspace,
                    window,
                    cx,
                ),
                scroll_handle: ScrollHandle::default(),
            });
        }
    }

    fn append_text(
        &mut self,
        text: &String,
        workspace: WeakEntity<Workspace>,
        window: &Window,
        cx: &mut App,
    ) {
        if let Some(RenderedMessageSegment::Text(markdown)) = self.segments.last_mut() {
            markdown.update(cx, |markdown, cx| markdown.append(text, cx));
        } else {
            self.segments
                .push(RenderedMessageSegment::Text(render_markdown(
                    SharedString::from(text),
                    self.language_registry.clone(),
                    workspace,
                    window,
                    cx,
                )));
        }
    }

    fn push_segment(
        &mut self,
        segment: &MessageSegment,
        workspace: WeakEntity<Workspace>,
        window: &Window,
        cx: &mut App,
    ) {
        let rendered_segment = match segment {
            MessageSegment::Thinking(text) => RenderedMessageSegment::Thinking {
                content: render_markdown(
                    text.into(),
                    self.language_registry.clone(),
                    workspace,
                    window,
                    cx,
                ),
                scroll_handle: ScrollHandle::default(),
            },
            MessageSegment::Text(text) => RenderedMessageSegment::Text(render_markdown(
                text.into(),
                self.language_registry.clone(),
                workspace,
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
    workspace: WeakEntity<Workspace>,
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
        link_callback: Some(Rc::new(move |url, cx| {
            if MentionLink::is_valid(url) {
                let colors = cx.theme().colors();
                Some(TextStyleRefinement {
                    background_color: Some(colors.element_background),
                    ..Default::default()
                })
            } else {
                None
            }
        })),
        ..Default::default()
    };

    cx.new(|cx| {
        Markdown::new(text, markdown_style, Some(language_registry), None, cx).open_url(
            move |text, window, cx| {
                open_markdown_link(text, workspace.clone(), window, cx);
            },
        )
    })
}

fn render_tool_use_markdown(
    text: SharedString,
    language_registry: Arc<LanguageRegistry>,
    workspace: WeakEntity<Workspace>,
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
        code_block: StyleRefinement {
            margin: EdgesRefinement::default(),
            padding: EdgesRefinement::default(),
            background: Some(colors.editor_background.into()),
            border_color: None,
            border_widths: EdgesRefinement::default(),
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
            font_size: Some(TextSize::XSmall.rems(cx).into()),
            ..Default::default()
        },
        heading: StyleRefinement {
            text: Some(TextStyleRefinement {
                font_size: Some(ui_font_size.into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    cx.new(|cx| {
        Markdown::new(text, markdown_style, Some(language_registry), None, cx).open_url(
            move |text, window, cx| {
                open_markdown_link(text, workspace.clone(), window, cx);
            },
        )
    })
}

fn open_markdown_link(
    text: SharedString,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        cx.open_url(&text);
        return;
    };

    match MentionLink::try_parse(&text, &workspace, cx) {
        Some(MentionLink::File(path, entry)) => workspace.update(cx, |workspace, cx| {
            if entry.is_dir() {
                workspace.project().update(cx, |_, cx| {
                    cx.emit(project::Event::RevealInProjectPanel(entry.id));
                })
            } else {
                workspace
                    .open_path(path, None, true, window, cx)
                    .detach_and_log_err(cx);
            }
        }),
        Some(MentionLink::Symbol(path, symbol_name)) => {
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_path(path, None, true, window, cx)
            });
            window
                .spawn(cx, async move |cx| {
                    let active_editor = open_task
                        .await?
                        .downcast::<Editor>()
                        .context("Item is not an editor")?;
                    active_editor.update_in(cx, |editor, window, cx| {
                        let symbol_range = editor
                            .buffer()
                            .read(cx)
                            .snapshot(cx)
                            .outline(None)
                            .and_then(|outline| {
                                outline
                                    .find_most_similar(&symbol_name)
                                    .map(|(_, item)| item.range.clone())
                            })
                            .context("Could not find matching symbol")?;

                        editor.change_selections(Some(Autoscroll::center()), window, cx, |s| {
                            s.select_anchor_ranges([symbol_range.start..symbol_range.start])
                        });
                        anyhow::Ok(())
                    })
                })
                .detach_and_log_err(cx);
        }
        Some(MentionLink::Thread(thread_id)) => workspace.update(cx, |workspace, cx| {
            if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel
                        .open_thread(&thread_id, window, cx)
                        .detach_and_log_err(cx)
                });
            }
        }),
        Some(MentionLink::Fetch(url)) => cx.open_url(&url),
        None => cx.open_url(&text),
    }
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
            rendered_tool_uses: HashMap::default(),
            expanded_tool_uses: HashMap::default(),
            expanded_thinking_segments: HashMap::default(),
            list_state: list_state.clone(),
            scrollbar_state: ScrollbarState::new(list_state),
            show_scrollbar: false,
            hide_scrollbar_task: None,
            editing_message: None,
            last_error: None,
            notifications: Vec::new(),
            _subscriptions: subscriptions,
            notification_subscriptions: HashMap::default(),
            feedback_message_editor: None,
        };

        for message in thread.read(cx).messages().cloned().collect::<Vec<_>>() {
            this.push_message(&message.id, &message.segments, window, cx);

            for tool_use in thread.read(cx).tool_uses_for_message(message.id, cx) {
                this.render_tool_use_markdown(
                    tool_use.id.clone(),
                    tool_use.ui_text.clone(),
                    &tool_use.input,
                    tool_use.status.text(),
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

        let rendered_message = RenderedMessage::from_segments(
            segments,
            self.language_registry.clone(),
            self.workspace.clone(),
            window,
            cx,
        );
        self.rendered_messages_by_id.insert(*id, rendered_message);
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
        let rendered_message = RenderedMessage::from_segments(
            segments,
            self.language_registry.clone(),
            self.workspace.clone(),
            window,
            cx,
        );
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

    fn render_tool_use_markdown(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_label: impl Into<SharedString>,
        tool_input: &serde_json::Value,
        tool_output: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rendered = RenderedToolUse {
            label: render_tool_use_markdown(
                tool_label.into(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            ),
            input: render_tool_use_markdown(
                format!(
                    "```json\n{}\n```",
                    serde_json::to_string_pretty(tool_input).unwrap_or_default()
                )
                .into(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            ),
            output: render_tool_use_markdown(
                tool_output,
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            ),
        };
        self.rendered_tool_uses
            .insert(tool_use_id.clone(), rendered);
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
            ThreadEvent::StreamedCompletion
            | ThreadEvent::SummaryGenerated
            | ThreadEvent::SummaryChanged => {
                self.save_thread(cx);
            }
            ThreadEvent::DoneStreaming => {
                let thread = self.thread.read(cx);

                if !thread.is_generating() {
                    self.show_notification(
                        if thread.used_tools_since_last_user_message() {
                            "Finished running tools"
                        } else {
                            "New message"
                        },
                        IconName::ZedAssistant,
                        window,
                        cx,
                    );
                }
            }
            ThreadEvent::ToolConfirmationNeeded => {
                self.show_notification("Waiting for tool confirmation", IconName::Info, window, cx);
            }
            ThreadEvent::StreamedAssistantText(message_id, text) => {
                if let Some(rendered_message) = self.rendered_messages_by_id.get_mut(&message_id) {
                    rendered_message.append_text(text, self.workspace.clone(), window, cx);
                }
            }
            ThreadEvent::StreamedAssistantThinking(message_id, text) => {
                if let Some(rendered_message) = self.rendered_messages_by_id.get_mut(&message_id) {
                    rendered_message.append_thinking(text, self.workspace.clone(), window, cx);
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
                    self.render_tool_use_markdown(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        &tool_use.input,
                        "".into(),
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
                    self.render_tool_use_markdown(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        &tool_use.input,
                        self.thread
                            .read(cx)
                            .tool_result(&tool_use.id)
                            .map(|result| result.content.clone().into())
                            .unwrap_or("".into()),
                        window,
                        cx,
                    );
                }

                if self.thread.read(cx).all_tools_finished() {
                    let model_registry = LanguageModelRegistry::read_global(cx);
                    if let Some(ConfiguredModel { model, .. }) = model_registry.default_model() {
                        self.thread.update(cx, |thread, cx| {
                            thread.attach_tool_results(cx);
                            if !canceled {
                                thread.send_to_model(model, RequestKind::Chat, cx);
                            }
                        });
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
        window: &mut Window,
        cx: &mut Context<ActiveThread>,
    ) {
        if window.is_window_active() || !self.notifications.is_empty() {
            return;
        }

        let title = self
            .thread
            .read(cx)
            .summary()
            .unwrap_or("Agent Panel".into());

        match AssistantSettings::get_global(cx).notify_when_agent_waiting {
            NotifyWhenAgentWaiting::PrimaryScreen => {
                if let Some(primary) = cx.primary_display() {
                    self.pop_up(icon, caption.into(), title.clone(), window, primary, cx);
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
        cx: &mut Context<'_, ActiveThread>,
    ) {
        let options = AgentNotification::window_options(screen, cx);

        if let Some(screen_window) = cx
            .open_window(options, |_, cx| {
                cx.new(|_| AgentNotification::new(title.clone(), caption.clone(), icon))
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
                                cx.activate(true); // Switch back to the Zed application

                                let workspace_handle = this.workspace.clone();

                                // If there are multiple Zed windows, activate the correct one.
                                cx.defer(move |cx| {
                                    handle
                                        .update(cx, |_view, window, _cx| {
                                            window.activate_window();

                                            if let Some(workspace) = workspace_handle.upgrade() {
                                                workspace.update(_cx, |workspace, cx| {
                                                    workspace
                                                        .focus_panel::<AssistantPanel>(window, cx);
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

        let Some(model) = LanguageModelRegistry::read_global(cx).default_model() else {
            return;
        };

        if model.provider.must_accept_terms(cx) {
            cx.notify();
            return;
        }

        self.thread.update(cx, |thread, cx| {
            thread.send_to_model(model.model, RequestKind::Chat, cx)
        });
        cx.notify();
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match feedback {
            ThreadFeedback::Positive => {
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
            ThreadFeedback::Negative => {
                self.handle_show_feedback_comments(window, cx);
            }
        }
    }

    fn handle_show_feedback_comments(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.feedback_message_editor.is_some() {
            return;
        }

        let buffer = cx.new(|cx| {
            let empty_string = String::new();
            MultiBuffer::singleton(cx.new(|cx| Buffer::local(empty_string, cx)), cx)
        });

        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight { max_lines: 4 },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_placeholder_text(
                "What went wrong? Share your feedback so we can improve.",
                cx,
            );
            editor
        });

        editor.read(cx).focus_handle(cx).focus(window);
        self.feedback_message_editor = Some(editor);
        cx.notify();
    }

    fn submit_feedback_message(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.feedback_message_editor.clone() else {
            return;
        };

        let report_task = self.thread.update(cx, |thread, cx| {
            thread.report_feedback(ThreadFeedback::Negative, cx)
        });

        let comments = editor.read(cx).text(cx);
        if !comments.is_empty() {
            let thread_id = self.thread.read(cx).id().clone();

            telemetry::event!("Assistant Thread Feedback Comments", thread_id, comments);
        }

        self.feedback_message_editor = None;

        let this = cx.entity().downgrade();
        cx.spawn(async move |_, cx| {
            report_task.await?;
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

        let context_store = self.context_store.clone();
        let workspace = self.workspace.clone();

        let thread = self.thread.read(cx);
        // Get all the data we need from thread before we start using it in closures
        let checkpoint = thread.checkpoint_for_message(message_id);
        let context = thread.context_for_message(message_id).collect::<Vec<_>>();
        let tool_uses = thread.tool_uses_for_message(message_id, cx);
        let has_tool_uses = !tool_uses.is_empty();

        // Don't render user messages that are just there for returning tool results.
        if message.role == Role::User && thread.message_has_tool_results(message_id) {
            return Empty.into_any();
        }

        let allow_editing_message = message.role == Role::User;

        let edit_message_editor = self
            .editing_message
            .as_ref()
            .filter(|(id, _)| *id == message_id)
            .map(|(_, state)| state.editor.clone());

        let first_message = ix == 0;
        let show_feedback = ix == self.messages.len() - 1 && message.role != Role::User;

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

        let message_is_empty = message.should_display_content();
        let has_content = !message_is_empty || !context.is_empty();

        let message_content =
            has_content.then(|| {
                v_flex()
                    .gap_1p5()
                    .when(!message_is_empty, |parent| {
                        parent.child(
                            if let Some(edit_message_editor) = edit_message_editor.clone() {
                                div()
                                    .key_context("EditMessageEditor")
                                    .on_action(cx.listener(Self::cancel_editing_message))
                                    .on_action(cx.listener(Self::confirm_editing_message))
                                    .min_h_6()
                                    .child(edit_message_editor)
                                    .into_any()
                            } else {
                                div()
                                    .min_h_6()
                                    .text_ui(cx)
                                    .child(self.render_message_content(
                                        message_id,
                                        rendered_message,
                                        has_tool_uses,
                                        cx,
                                    ))
                                    .into_any()
                            },
                        )
                    })
                    .when(!context.is_empty(), |parent| {
                        parent.child(h_flex().flex_wrap().gap_1().children(
                            context.into_iter().map(|context| {
                                let context_id = context.id();
                                ContextPill::added(
                                    AddedContext::new(context, cx),
                                    false,
                                    false,
                                    None,
                                )
                                .on_click(Rc::new(cx.listener({
                                    let workspace = workspace.clone();
                                    let context_store = context_store.clone();
                                    move |_, _, window, cx| {
                                        if let Some(workspace) = workspace.upgrade() {
                                            open_context(
                                                context_id,
                                                context_store.clone(),
                                                workspace,
                                                window,
                                                cx,
                                            );
                                            cx.notify();
                                        }
                                    }
                                })))
                            }),
                        ))
                    })
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
                        .child(div().p_2().children(message_content)),
                ),
            Role::Assistant => v_flex()
                .id(("message-container", ix))
                .ml_2()
                .pl_2()
                .pr_4()
                .border_l_1()
                .border_color(cx.theme().colors().border_variant)
                .children(message_content)
                .gap_2p5()
                .pb_2p5()
                .when(!tool_uses.is_empty(), |parent| {
                    parent.child(
                        div().children(
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
                    .child(div().p_4().children(message_content)),
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
                show_feedback && !self.thread.read(cx).is_generating(),
                |parent| {
                    parent.child(feedback_items).when_some(
                        self.feedback_message_editor.clone(),
                        |parent, feedback_editor| {
                            let focus_handle = feedback_editor.focus_handle(cx);
                            parent.child(
                                v_flex()
                                    .key_context("AgentFeedbackMessageEditor")
                                    .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                                        this.feedback_message_editor = None;
                                        cx.notify();
                                    }))
                                    .on_action(cx.listener(|this, _: &menu::Confirm, _, cx| {
                                        this.submit_feedback_message(cx);
                                        cx.notify();
                                    }))
                                    .on_action(cx.listener(Self::confirm_editing_message))
                                    .mx_4()
                                    .mb_3()
                                    .p_2()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .bg(cx.theme().colors().editor_background)
                                    .child(feedback_editor)
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .justify_end()
                                            .child(
                                                Button::new("dismiss-feedback-message", "Cancel")
                                                    .label_size(LabelSize::Small)
                                                    .key_binding(
                                                        KeyBinding::for_action_in(
                                                            &menu::Cancel,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                        .map(|kb| kb.size(rems_from_px(10.))),
                                                    )
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.feedback_message_editor = None;
                                                        cx.notify();
                                                    })),
                                            )
                                            .child(
                                                Button::new(
                                                    "submit-feedback-message",
                                                    "Share Feedback",
                                                )
                                                .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                                .label_size(LabelSize::Small)
                                                .key_binding(
                                                    KeyBinding::for_action_in(
                                                        &menu::Confirm,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                    .map(|kb| kb.size(rems_from_px(10.))),
                                                )
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.submit_feedback_message(cx);
                                                    cx.notify();
                                                })),
                                            ),
                                    ),
                            )
                        },
                    )
                },
            )
            .into_any()
    }

    fn render_message_content(
        &self,
        message_id: MessageId,
        rendered_message: &RenderedMessage,
        has_tool_uses: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_last_message = self.messages.last() == Some(&message_id);
        let is_generating = self.thread.read(cx).is_generating();
        let pending_thinking_segment_index = if is_generating && is_last_message && !has_tool_uses {
            rendered_message
                .segments
                .iter()
                .enumerate()
                .next_back()
                .filter(|(_, segment)| matches!(segment, RenderedMessageSegment::Thinking { .. }))
                .map(|(index, _)| index)
        } else {
            None
        };

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

        div().pt_0p5().pb_2().child(
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

    fn render_tool_use(
        &self,
        tool_use: ToolUse,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let is_open = self
            .expanded_tool_uses
            .get(&tool_use.id)
            .copied()
            .unwrap_or_default();

        let is_status_finished = matches!(&tool_use.status, ToolUseStatus::Finished(_));

        let fs = self
            .workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).app_state().fs.clone());
        let needs_confirmation = matches!(&tool_use.status, ToolUseStatus::NeedsConfirmation);

        let status_icons = div().child(match &tool_use.status {
            ToolUseStatus::Pending | ToolUseStatus::NeedsConfirmation => {
                let icon = Icon::new(IconName::Warning)
                    .color(Color::Warning)
                    .size(IconSize::Small);
                icon.into_any_element()
            }
            ToolUseStatus::Running => {
                let icon = Icon::new(IconName::ArrowCircle)
                    .color(Color::Accent)
                    .size(IconSize::Small);
                icon.with_animation(
                    "arrow-circle",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                )
                .into_any_element()
            }
            ToolUseStatus::Finished(_) => div().w_0().into_any_element(),
            ToolUseStatus::Error(_) => {
                let icon = Icon::new(IconName::Close)
                    .color(Color::Error)
                    .size(IconSize::Small);
                icon.into_any_element()
            }
        });

        let rendered_tool_use = self.rendered_tool_uses.get(&tool_use.id).cloned();
        let results_content_container = || v_flex().p_2().gap_0p5();

        let results_content = v_flex()
            .gap_1()
            .child(
                results_content_container()
                    .child(
                        Label::new("Input")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(
                        div().w_full().text_ui_sm(cx).children(
                            rendered_tool_use
                                .as_ref()
                                .map(|rendered| rendered.input.clone()),
                        ),
                    ),
            )
            .map(|container| match tool_use.status {
                ToolUseStatus::Finished(_) => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            Label::new("Result")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .buffer_font(cx),
                        )
                        .child(
                            div().w_full().text_ui_sm(cx).children(
                                rendered_tool_use
                                    .as_ref()
                                    .map(|rendered| rendered.output.clone()),
                            ),
                        ),
                ),
                ToolUseStatus::Running => container.child(
                    results_content_container().child(
                        h_flex()
                            .gap_1()
                            .pb_1()
                            .border_t_1()
                            .border_color(self.tool_card_border_color(cx))
                            .child(
                                Icon::new(IconName::ArrowCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Accent)
                                    .with_animation(
                                        "arrow-circle",
                                        Animation::new(Duration::from_secs(2)).repeat(),
                                        |icon, delta| {
                                            icon.transform(Transformation::rotate(percentage(
                                                delta,
                                            )))
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
                ToolUseStatus::Error(_) => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            Label::new("Error")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .buffer_font(cx),
                        )
                        .child(
                            div().text_ui_sm(cx).children(
                                rendered_tool_use
                                    .as_ref()
                                    .map(|rendered| rendered.output.clone()),
                            ),
                        ),
                ),
                ToolUseStatus::Pending => container,
                ToolUseStatus::NeedsConfirmation => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            Label::new("Asking Permission")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .buffer_font(cx),
                        ),
                ),
            });

        let gradient_overlay = |color: Hsla| {
            div()
                .h_full()
                .absolute()
                .w_12()
                .bottom_0()
                .map(|element| {
                    if is_status_finished {
                        element.right_6()
                    } else {
                        element.right(px(44.))
                    }
                })
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(color, 1.),
                    linear_color_stop(color.opacity(0.2), 0.),
                ))
        };

        div().map(|element| {
            if !tool_use.needs_confirmation {
                element.child(
                    v_flex()
                        .child(
                            h_flex()
                                .group("disclosure-header")
                                .relative()
                                .gap_1p5()
                                .justify_between()
                                .opacity(0.8)
                                .hover(|style| style.opacity(1.))
                                .when(!is_status_finished, |this| this.pr_2())
                                .child(
                                    h_flex()
                                        .id("tool-label-container")
                                        .gap_1p5()
                                        .max_w_full()
                                        .overflow_x_scroll()
                                        .child(
                                            Icon::new(tool_use.icon)
                                                .size(IconSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            h_flex().pr_8().text_ui_sm(cx).children(
                                                rendered_tool_use.map(|rendered| rendered.label)
                                            ),
                                        ),
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
                                        .child(status_icons),
                                )
                                .child(gradient_overlay(cx.theme().colors().panel_background)),
                        )
                        .map(|parent| {
                            if !is_open {
                                return parent;
                            }

                            parent.child(
                                v_flex()
                                    .mt_1()
                                    .border_1()
                                    .border_color(self.tool_card_border_color(cx))
                                    .bg(cx.theme().colors().editor_background)
                                    .rounded_lg()
                                    .child(results_content),
                            )
                        }),
                )
            } else {
                v_flex()
                    .rounded_lg()
                    .border_1()
                    .border_color(self.tool_card_border_color(cx))
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .group("disclosure-header")
                            .relative()
                            .justify_between()
                            .py_1()
                            .map(|element| {
                                if is_status_finished {
                                    element.pl_2().pr_0p5()
                                } else {
                                    element.px_2()
                                }
                            })
                            .bg(self.tool_card_header_bg(cx))
                            .map(|element| {
                                if is_open {
                                    element.border_b_1().rounded_t_md()
                                } else if needs_confirmation {
                                    element.rounded_t_md()
                                } else {
                                    element.rounded_md()
                                }
                            })
                            .border_color(self.tool_card_border_color(cx))
                            .child(
                                h_flex()
                                    .id("tool-label-container")
                                    .gap_1p5()
                                    .max_w_full()
                                    .overflow_x_scroll()
                                    .child(
                                        Icon::new(tool_use.icon)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        h_flex().pr_8().text_ui_sm(cx).children(
                                            rendered_tool_use.map(|rendered| rendered.label)
                                        ),
                                    ),
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
                                    .child(status_icons),
                            )
                            .child(gradient_overlay(self.tool_card_header_bg(cx))),
                    )
                    .map(|parent| {
                        if !is_open {
                            return parent;
                        }

                        parent.child(
                            v_flex()
                                .bg(cx.theme().colors().editor_background)
                                .map(|element| {
                                    if  needs_confirmation {
                                        element.rounded_none()
                                    } else {
                                        element.rounded_b_lg()
                                    }
                                })
                                .child(results_content),
                        )
                    })
                    .when(needs_confirmation, |this| {
                        this.child(
                            h_flex()
                                .py_1()
                                .pl_2()
                                .pr_1()
                                .gap_1()
                                .justify_between()
                                .bg(cx.theme().colors().editor_background)
                                .border_t_1()
                                .border_color(self.tool_card_border_color(cx))
                                .rounded_b_lg()
                                .child(Label::new("Action Confirmation").color(Color::Muted).size(LabelSize::Small))
                                .child(
                                    h_flex()
                                        .gap_0p5()
                                        .child({
                                            let tool_id = tool_use.id.clone();
                                            Button::new(
                                                "always-allow-tool-action",
                                                "Always Allow",
                                            )
                                            .label_size(LabelSize::Small)
                                            .icon(IconName::CheckDouble)
                                            .icon_position(IconPosition::Start)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Success)
                                            .tooltip(move |window, cx|  {
                                                Tooltip::with_meta(
                                                    "Never ask for permission",
                                                    None,
                                                    "Restore the original behavior in your Agent Panel settings",
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .on_click(cx.listener(
                                                move |this, event, window, cx| {
                                                    if let Some(fs) = fs.clone() {
                                                        update_settings_file::<AssistantSettings>(
                                                            fs.clone(),
                                                            cx,
                                                            |settings, _| {
                                                                settings.set_always_allow_tool_actions(true);
                                                            },
                                                        );
                                                    }
                                                    this.handle_allow_tool(
                                                        tool_id.clone(),
                                                        event,
                                                        window,
                                                        cx,
                                                    )
                                                },
                                            ))
                                        })
                                        .child(ui::Divider::vertical())
                                        .child({
                                            let tool_id = tool_use.id.clone();
                                            Button::new("allow-tool-action", "Allow")
                                                .label_size(LabelSize::Small)
                                                .icon(IconName::Check)
                                                .icon_position(IconPosition::Start)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Success)
                                                .on_click(cx.listener(
                                                    move |this, event, window, cx| {
                                                        this.handle_allow_tool(
                                                            tool_id.clone(),
                                                            event,
                                                            window,
                                                            cx,
                                                        )
                                                    },
                                                ))
                                        })
                                        .child({
                                            let tool_id = tool_use.id.clone();
                                            let tool_name: Arc<str> = tool_use.name.into();
                                            Button::new("deny-tool", "Deny")
                                                .label_size(LabelSize::Small)
                                                .icon(IconName::Close)
                                                .icon_position(IconPosition::Start)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Error)
                                                .on_click(cx.listener(
                                                    move |this, event, window, cx| {
                                                        this.handle_deny_tool(
                                                            tool_id.clone(),
                                                            tool_name.clone(),
                                                            event,
                                                            window,
                                                            cx,
                                                        )
                                                    },
                                                ))
                                        }),
                                ),
                        )
                    })
            }
        })
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
        tool_name: Arc<str>,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread.update(cx, |thread, cx| {
            thread.deny_tool_use(tool_use_id, tool_name, cx);
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

    fn dismiss_notifications(&mut self, cx: &mut Context<ActiveThread>) {
        for window in self.notifications.drain(..) {
            window
                .update(cx, |_, window, _| {
                    window.remove_window();
                })
                .ok();

            self.notification_subscriptions.remove(&window);
        }
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !self.show_scrollbar && !self.scrollbar_state.is_dragging() {
            return None;
        }

        Some(
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
                .children(Scrollbar::vertical(self.scrollbar_state.clone())),
        )
    }

    fn hide_scrollbar_later(&mut self, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        self.hide_scrollbar_task = Some(cx.spawn(async move |thread, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            thread
                .update(cx, |thread, cx| {
                    if !thread.scrollbar_state.is_dragging() {
                        thread.show_scrollbar = false;
                        cx.notify();
                    }
                })
                .log_err();
        }))
    }
}

impl Render for ActiveThread {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .relative()
            .on_mouse_move(cx.listener(|this, _, _, cx| {
                this.show_scrollbar = true;
                this.hide_scrollbar_later(cx);
                cx.notify();
            }))
            .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                this.show_scrollbar = true;
                this.hide_scrollbar_later(cx);
                cx.notify();
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.hide_scrollbar_later(cx);
                }),
            )
            .child(list(self.list_state.clone()).flex_grow())
            .when_some(self.render_vertical_scrollbar(cx), |this, scrollbar| {
                this.child(scrollbar)
            })
    }
}

pub(crate) fn open_context(
    id: ContextId,
    context_store: Entity<ContextStore>,
    workspace: Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(context) = context_store.read(cx).context_for_id(id) else {
        return;
    };

    match context {
        AssistantContext::File(file_context) => {
            if let Some(project_path) = file_context.context_buffer.buffer.read(cx).project_path(cx)
            {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_path(project_path, None, true, window, cx)
                        .detach_and_log_err(cx);
                });
            }
        }
        AssistantContext::Directory(directory_context) => {
            let path = directory_context.project_path.clone();
            workspace.update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    if let Some(entry) = project.entry_for_path(&path, cx) {
                        cx.emit(project::Event::RevealInProjectPanel(entry.id));
                    }
                })
            })
        }
        AssistantContext::Symbol(symbol_context) => {
            if let Some(project_path) = symbol_context
                .context_symbol
                .buffer
                .read(cx)
                .project_path(cx)
            {
                let snapshot = symbol_context.context_symbol.buffer.read(cx).snapshot();
                let target_position = symbol_context
                    .context_symbol
                    .id
                    .range
                    .start
                    .to_point(&snapshot);

                let open_task = workspace.update(cx, |workspace, cx| {
                    workspace.open_path(project_path, None, true, window, cx)
                });
                window
                    .spawn(cx, async move |cx| {
                        if let Some(active_editor) = open_task
                            .await
                            .log_err()
                            .and_then(|item| item.downcast::<Editor>())
                        {
                            active_editor
                                .downgrade()
                                .update_in(cx, |editor, window, cx| {
                                    editor.go_to_singleton_buffer_point(
                                        target_position,
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    })
                    .detach();
            }
        }
        AssistantContext::FetchedUrl(fetched_url_context) => {
            cx.open_url(&fetched_url_context.url);
        }
        AssistantContext::Thread(thread_context) => {
            let thread_id = thread_context.thread.read(cx).id().clone();
            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel
                            .open_thread(&thread_id, window, cx)
                            .detach_and_log_err(cx)
                    });
                }
            })
        }
    }
}
