use std::sync::Arc;

use assistant_tool::ToolWorkingSet;
use collections::HashMap;
use gpui::{
    list, AbsoluteLength, AnyElement, App, DefiniteLength, EdgesRefinement, Empty, Entity, Length,
    ListAlignment, ListOffset, ListState, StyleRefinement, Subscription, TextStyleRefinement,
    UnderlineStyle, WeakEntity,
};
use language::LanguageRegistry;
use language_model::Role;
use markdown::{Markdown, MarkdownStyle};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::prelude::*;
use workspace::Workspace;

use crate::thread::{MessageId, Thread, ThreadError, ThreadEvent};
use crate::thread_store::ThreadStore;
use crate::ui::ContextPill;

pub struct ActiveThread {
    workspace: WeakEntity<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    tools: Arc<ToolWorkingSet>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<Thread>,
    messages: Vec<MessageId>,
    list_state: ListState,
    rendered_messages_by_id: HashMap<MessageId, Entity<Markdown>>,
    last_error: Option<ThreadError>,
    _subscriptions: Vec<Subscription>,
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
            messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            list_state: ListState::new(0, ListAlignment::Bottom, px(1024.), {
                let this = cx.entity().downgrade();
                move |ix, _: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| this.render_message(ix, cx))
                        .unwrap()
                }
            }),
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

        let markdown = cx.new(|cx| {
            Markdown::new(
                text.into(),
                markdown_style,
                Some(self.language_registry.clone()),
                None,
                cx,
            )
        });
        self.rendered_messages_by_id.insert(*id, markdown);
        self.list_state.scroll_to(ListOffset {
            item_ix: old_len,
            offset_in_item: Pixels(0.0),
        });
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
                self.thread_store
                    .update(cx, |thread_store, cx| {
                        thread_store.save_thread(&self.thread, cx)
                    })
                    .detach_and_log_err(cx);
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

                self.thread_store
                    .update(cx, |thread_store, cx| {
                        thread_store.save_thread(&self.thread, cx)
                    })
                    .detach_and_log_err(cx);

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
                            thread.insert_tool_output(
                                tool_use.assistant_message_id,
                                tool_use.id.clone(),
                                task,
                                cx,
                            );
                        });
                    }
                }
            }
            ThreadEvent::ToolFinished { .. } => {}
        }
    }

    fn render_message(&self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let message_id = self.messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(markdown) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let context = self.thread.read(cx).context_for_message(message_id);
        let colors = cx.theme().colors();

        let message_content = v_flex()
            .child(div().p_2p5().text_ui(cx).child(markdown.clone()))
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
                                .px_2()
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
                                ),
                        )
                        .child(message_content),
                ),
            Role::Assistant => div().id(("message-container", ix)).child(message_content),
            Role::System => div().id(("message-container", ix)).py_1().px_2().child(
                v_flex()
                    .bg(colors.editor_background)
                    .rounded_md()
                    .child(message_content),
            ),
        };

        styled_message.into_any()
    }
}

impl Render for ActiveThread {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(list(self.list_state.clone()).flex_grow())
    }
}
