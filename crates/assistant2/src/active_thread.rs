use std::sync::Arc;

use assistant_tool::ToolWorkingSet;
use collections::HashMap;
use gpui::{
    list, AnyElement, AppContext, Empty, ListAlignment, ListState, Model, StyleRefinement,
    Subscription, TextStyleRefinement, View, WeakView,
};
use language::LanguageRegistry;
use language_model::Role;
use markdown::{Markdown, MarkdownStyle};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::prelude::*;
use workspace::Workspace;

use crate::thread::{MessageId, Thread, ThreadError, ThreadEvent};
use crate::ui::ContextPill;

pub struct ActiveThread {
    workspace: WeakView<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    tools: Arc<ToolWorkingSet>,
    thread: Model<Thread>,
    messages: Vec<MessageId>,
    list_state: ListState,
    rendered_messages_by_id: HashMap<MessageId, View<Markdown>>,
    last_error: Option<ThreadError>,
    _subscriptions: Vec<Subscription>,
}

impl ActiveThread {
    pub fn new(
        thread: Model<Thread>,
        workspace: WeakView<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe(&thread, Self::handle_thread_event),
        ];

        let mut this = Self {
            workspace,
            language_registry,
            tools,
            thread: thread.clone(),
            messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            list_state: ListState::new(0, ListAlignment::Bottom, px(1024.), {
                let this = cx.view().downgrade();
                move |ix, cx: &mut WindowContext| {
                    this.update(cx, |this, cx| this.render_message(ix, cx))
                        .unwrap()
                }
            }),
            last_error: None,
            _subscriptions: subscriptions,
        };

        for message in thread.read(cx).messages().cloned().collect::<Vec<_>>() {
            this.push_message(&message.id, message.text.clone(), cx);
        }

        this
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn summary(&self, cx: &AppContext) -> Option<SharedString> {
        self.thread.read(cx).summary()
    }

    pub fn last_error(&self) -> Option<ThreadError> {
        self.last_error.clone()
    }

    pub fn clear_last_error(&mut self) {
        self.last_error.take();
    }

    fn push_message(&mut self, id: &MessageId, text: String, cx: &mut ViewContext<Self>) {
        let old_len = self.messages.len();
        self.messages.push(*id);
        self.list_state.splice(old_len..old_len, 1);

        let theme_settings = ThemeSettings::get_global(cx);
        let ui_font_size = TextSize::Default.rems(cx);
        let buffer_font_size = theme_settings.buffer_font_size;

        let mut text_style = cx.text_style();
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
                text: Some(TextStyleRefinement {
                    font_family: Some(theme_settings.buffer_font.family.clone()),
                    font_size: Some(buffer_font_size.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            inline_code: TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_size: Some(ui_font_size.into()),
                background_color: Some(cx.theme().colors().editor_background),
                ..Default::default()
            },
            ..Default::default()
        };

        let markdown = cx.new_view(|cx| {
            Markdown::new(
                text,
                markdown_style,
                Some(self.language_registry.clone()),
                None,
                cx,
            )
        });
        self.rendered_messages_by_id.insert(*id, markdown);
    }

    fn handle_thread_event(
        &mut self,
        _: Model<Thread>,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ThreadEvent::ShowError(error) => {
                self.last_error = Some(error.clone());
            }
            ThreadEvent::StreamedCompletion => {}
            ThreadEvent::SummaryChanged => {}
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
                    self.push_message(message_id, message_text, cx);
                }

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
                        let task = tool.run(tool_use.input, self.workspace.clone(), cx);

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

    fn render_message(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let message_id = self.messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(markdown) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let context = self.thread.read(cx).context_for_message(message_id);

        let (role_icon, role_name) = match message.role {
            Role::User => (IconName::Person, "You"),
            Role::Assistant => (IconName::ZedAssistant, "Assistant"),
            Role::System => (IconName::Settings, "System"),
        };

        div()
            .id(("message-container", ix))
            .p_2()
            .child(
                v_flex()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .child(
                        h_flex()
                            .justify_between()
                            .p_1p5()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(role_icon).size(IconSize::Small))
                                    .child(Label::new(role_name).size(LabelSize::Small)),
                            ),
                    )
                    .child(v_flex().p_1p5().text_ui(cx).child(markdown.clone()))
                    .when_some(context, |parent, context| {
                        parent.child(
                            h_flex().flex_wrap().gap_2().p_1p5().children(
                                context
                                    .iter()
                                    .map(|context| ContextPill::new(context.clone())),
                            ),
                        )
                    }),
            )
            .into_any()
    }
}

impl Render for ActiveThread {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        list(self.list_state.clone()).flex_1()
    }
}
