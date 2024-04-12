mod completion_provider;

use std::sync::Arc;

use client::Client;
use completion_provider::*;
use editor::Editor;
use futures::StreamExt;
use gpui::{
    list, prelude::*, AnyElement, AppContext, Global, ListAlignment, ListState, Render, Task, View,
};
use language::{language_settings::SoftWrap, LanguageRegistry};
use rich_text::RichText;
use semantic_index::SearchResult;
use settings::Settings;
use theme::ThemeSettings;
use ui::{popover_menu, prelude::*, ButtonLike, Color, ContextMenu, Tooltip};
use util::{post_inc, ResultExt, TryFutureExt};

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
    pending_completion: Option<Task<()>>,
}

impl AssistantChat {
    fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ViewContext<Self>) -> Self {
        let this = cx.view().downgrade();
        let list_state = ListState::new(0, ListAlignment::Bottom, px(1024.), move |ix, cx| {
            this.update(cx, |this, cx| this.render_message(ix, cx))
                .unwrap()
        });
        let model = CompletionProvider::get(cx).default_model();

        let mut this = Self {
            model,
            messages: Vec::new(),
            list_state,
            language_registry,
            next_message_id: 0,
            pending_completion: None,
        };
        this.push_user_message(true, cx);
        this
    }

    fn submit(&mut self, _: &Submit, cx: &mut ViewContext<Self>) {
        let Some((selected_message_ix, selected_message_focus_handle)) =
            self.messages.iter().enumerate().find_map(|(ix, message)| {
                if let AssistantMessage::User { body, .. } = message {
                    let focus_handle = body.focus_handle(cx);
                    if focus_handle.contains_focused(cx) {
                        Some((ix, focus_handle))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        else {
            log::error!("unexpected state: no user message editor is focused.");
            return;
        };

        self.truncate_messages(selected_message_ix + 1, cx);
        self.push_assistant_message(cx);

        let completion = CompletionProvider::get(cx).complete(
            self.model.clone(),
            self.messages(cx),
            Vec::new(),
            1.0,
        );
        self.pending_completion = Some(cx.spawn(|this, mut cx| async move {
            let complete = async {
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
                            *message_body =
                                RichText::new(body.clone(), &[], &this.language_registry);
                            cx.notify();
                        } else {
                            unreachable!()
                        }
                    })?;
                }

                anyhow::Ok(())
            }
            .await;

            this.update(&mut cx, |this, cx| {
                if let Err(error) = complete {
                    if let Some(AssistantMessage::Assistant {
                        error: message_error,
                        ..
                    }) = this.messages.last_mut()
                    {
                        message_error.replace(SharedString::from(error.to_string()));
                        cx.notify();
                    } else {
                        unreachable!()
                    }
                }

                let focus = selected_message_focus_handle.contains_focused(cx);
                this.push_user_message(focus, cx);
            })
            .log_err();
        }));
    }

    fn push_user_message(&mut self, focus: bool, cx: &mut ViewContext<Self>) {
        let message = AssistantMessage::User {
            id: post_inc(&mut self.next_message_id),
            body: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                if focus {
                    cx.focus_self();
                }
                editor
            }),
            contexts: Vec::new(),
        };
        self.push_message(message, cx);
    }

    fn push_assistant_message(&mut self, cx: &mut ViewContext<Self>) {
        let message = AssistantMessage::Assistant {
            id: post_inc(&mut self.next_message_id),
            body: RichText::default(),
            error: None,
        };
        self.push_message(message, cx);
    }

    fn push_message(&mut self, message: AssistantMessage, cx: &mut ViewContext<Self>) {
        let old_len = self.messages.len();
        self.messages.push(message);
        self.list_state.splice(old_len..old_len, 1);
        cx.notify();
    }

    fn truncate_messages(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if index < self.messages.len() {
            self.list_state.splice(index..self.messages.len(), 0);
            self.messages.truncate(index);
            cx.notify();
        }
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
            AssistantMessage::Assistant { id, body, error } => div()
                .p_2()
                .when(!is_last, |element| element.mb_2())
                .children(error.clone())
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
        div().h_flex().justify_end().child(
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
                                    .child(div().child(
                                        Icon::new(IconName::ChevronDown).color(Color::Muted),
                                    )),
                            )
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |cx| Tooltip::text("Change Model", cx)),
                    )
                    .anchor(gpui::AnchorCorner::TopRight),
            ),
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
        error: Option<SharedString>,
    },
}

enum AssistantContext {
    Codebase { results: Vec<SearchResult> },
}
