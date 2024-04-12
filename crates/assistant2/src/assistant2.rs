mod completion_provider;

use std::sync::Arc;

use client::Client;
use completion_provider::*;
use editor::Editor;
use futures::StreamExt;
use gpui::{
    list, prelude::*, AnyElement, AppContext, Global, ListAlignment, ListState, Render, View,
};
use language::{language_settings::SoftWrap, LanguageRegistry};
use rich_text::RichText;
use semantic_index::SearchResult;
use settings::Settings;
use theme::ThemeSettings;
use ui::{popover_menu, prelude::*, ButtonLike, Color, ContextMenu, Tooltip};

gpui::actions!(assistant, [Submit]);

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    cx.set_global(CompletionProvider::new(CloudCompletionProvider::new(
        client,
    )));
}

pub struct AssistantPanel {
    language_registry: Arc<LanguageRegistry>,
    chat: View<AssistantChat>,
}

impl AssistantPanel {
    pub fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ViewContext<Self>) -> Self {
        let chat = cx.new_view(|cx| AssistantChat::new(language_registry.clone(), cx));
        Self {
            language_registry,
            chat,
        }
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .v_flex()
            .p_2()
            .bg(cx.theme().colors().background)
            .child(self.chat.clone())
    }
}

struct AssistantChat {
    model: String,
    messages: Vec<AssistantMessage>,
    list_state: ListState,
    language_registry: Arc<LanguageRegistry>,
    next_message_id: usize,
}

impl AssistantChat {
    fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ViewContext<Self>) -> Self {
        let messages = vec![AssistantMessage::User {
            id: 0,
            body: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                editor.set_text("Hello, I'm trying to understand how to optimize a piece of Rust code for better performance. Could you provide some insights or guidelines on how to profile and identify bottlenecks in a Rust application?", cx);
                editor
            }),
            contexts: Vec::new(),
        }];

        let this = cx.view().downgrade();
        let list_state = ListState::new(
            messages.len(),
            ListAlignment::Top,
            px(1024.),
            move |ix, cx| {
                this.update(cx, |this, cx| this.render_message(ix, cx))
                    .unwrap()
            },
        );

        let model = CompletionProvider::get(cx).default_model();

        Self {
            model,
            messages,
            list_state,
            language_registry,
            next_message_id: 1,
        }
    }

    fn submit(&mut self, _: &Submit, cx: &mut ViewContext<Self>) {
        // todo!(Detect which message is focused and send the ones above it)
        let completion = CompletionProvider::get(cx).complete(
            self.model.clone(),
            self.messages(cx),
            Vec::new(),
            1.0,
        );

        let message_id = self.next_message_id;
        self.next_message_id += 1;
        self.push_message(
            AssistantMessage::Assistant {
                id: message_id,
                body: RichText::new(String::new(), &[], &self.language_registry),
            },
            cx,
        );

        cx.spawn(|this, mut cx| async move {
            let mut stream = completion.await?;

            let mut body = String::new();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                this.update(&mut cx, |this, cx| {
                    if let Some(AssistantMessage::Assistant {
                        body: message_body, ..
                    }) = this.messages.last_mut()
                    {
                        body.push_str(&chunk);
                        *message_body = RichText::new(body.clone(), &[], &this.language_registry);
                        cx.notify();
                    } else {
                        unreachable!()
                    }
                })?;
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn push_message(&mut self, message: AssistantMessage, cx: &mut ViewContext<Self>) {
        let old_len = self.messages.len();
        self.messages.push(message);
        self.list_state.splice(old_len..old_len, 1);
        cx.notify();
    }

    fn render_message(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let is_last = ix == self.messages.len() - 1;

        match &self.messages[ix] {
            AssistantMessage::User { body, .. } => div()
                .on_action(cx.listener(Self::submit))
                .p_2()
                .when(!is_last, |element| element.mb_2())
                .text_color(cx.theme().colors().editor_foreground)
                .font(ThemeSettings::get_global(cx).buffer_font.clone())
                .bg(cx.theme().colors().editor_background)
                .child(body.clone())
                .into_any(),
            AssistantMessage::Assistant { id, body } => div()
                .p_2()
                .when(!is_last, |element| element.mb_2())
                .child(body.element(ElementId::from(*id), cx))
                .into_any(),
        }
    }

    fn messages(&self, cx: &WindowContext) -> Vec<CompletionMessage> {
        self.messages
            .iter()
            .map(|message| match message {
                AssistantMessage::User { body, contexts, .. } => CompletionMessage {
                    role: CompletionRole::User,
                    body: body.read(cx).text(cx),
                },
                AssistantMessage::Assistant { body, .. } => CompletionMessage {
                    role: CompletionRole::Assistant,
                    body: body.text.to_string(),
                },
            })
            .collect()
    }

    fn render_model_dropdown(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let this = cx.view().downgrade();
        div().w_32().child(
            popover_menu("user-menu")
                .menu(move |cx| {
                    ContextMenu::build(cx, |mut menu, cx| {
                        for model in CompletionProvider::get(cx).available_models() {
                            menu = menu.custom_entry(
                                {
                                    let model = model.clone();
                                    move |_| Label::new(model.clone()).into_any_element()
                                },
                                {
                                    let this = this.clone();
                                    move |cx| {
                                        _ = this.update(cx, |this, cx| {
                                            this.model = model.clone();
                                            cx.notify();
                                        });
                                    }
                                },
                            );
                        }
                        menu
                    })
                    .into()
                })
                .trigger(
                    ButtonLike::new("active-model")
                        .child(
                            h_flex()
                                .w_full()
                                .gap_0p5()
                                .child(
                                    div()
                                        .overflow_x_hidden()
                                        .flex_grow()
                                        .whitespace_nowrap()
                                        .child(Label::new(self.model.clone())),
                                )
                                .child(
                                    div().child(
                                        Icon::new(IconName::ChevronDown).color(Color::Muted),
                                    ),
                                ),
                        )
                        .style(ButtonStyle::Subtle)
                        .tooltip(move |cx| Tooltip::text("Change Model", cx)),
                )
                .anchor(gpui::AnchorCorner::TopRight),
        )
    }
}

impl Render for AssistantChat {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .v_flex()
            .key_context("AssistantChat")
            .text_color(Color::Default.color(cx))
            .child(self.render_model_dropdown(cx))
            .child(list(self.list_state.clone()).flex_1())
    }
}

enum AssistantMessage {
    User {
        id: usize,
        body: View<Editor>,
        contexts: Vec<AssistantContext>,
    },
    Assistant {
        id: usize,
        body: RichText,
    },
}

enum AssistantContext {
    Codebase { results: Vec<SearchResult> },
}
