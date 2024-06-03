use crate::assistant_settings::ZedDotDevModel;
use crate::{
    assistant_settings::OpenAiModel, CompletionProvider, LanguageModel, LanguageModelRequest, Role,
};
use anyhow::{anyhow, Result};
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, FontStyle, Task, TextStyle, View, WhiteSpace};
use http::HttpClient;
use open_ai::{stream_completion, Request, RequestMessage, Role as OpenAiRole};
use settings::Settings;
use std::time::Duration;
use std::{env, sync::Arc};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::prelude::*;
use util::ResultExt;

pub struct OpenAiCompletionProvider {
    api_key: Option<String>,
    api_url: String,
    model: OpenAiModel,
    http_client: Arc<dyn HttpClient>,
    low_speed_timeout: Option<Duration>,
    settings_version: usize,
}

impl OpenAiCompletionProvider {
    pub fn new(
        model: OpenAiModel,
        api_url: String,
        http_client: Arc<dyn HttpClient>,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) -> Self {
        Self {
            api_key: None,
            api_url,
            model,
            http_client,
            low_speed_timeout,
            settings_version,
        }
    }

    pub fn update(
        &mut self,
        model: OpenAiModel,
        api_url: String,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) {
        self.model = model;
        self.api_url = api_url;
        self.low_speed_timeout = low_speed_timeout;
        self.settings_version = settings_version;
    }

    pub fn available_models(&self) -> impl Iterator<Item = OpenAiModel> {
        OpenAiModel::iter()
    }

    pub fn settings_version(&self) -> usize {
        self.settings_version
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            let api_url = self.api_url.clone();
            cx.spawn(|mut cx| async move {
                let api_key = if let Ok(api_key) = env::var("OPENAI_API_KEY") {
                    api_key
                } else {
                    let (_, api_key) = cx
                        .update(|cx| cx.read_credentials(&api_url))?
                        .await?
                        .ok_or_else(|| anyhow!("credentials not found"))?;
                    String::from_utf8(api_key)?
                };
                cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    if let CompletionProvider::OpenAi(provider) = provider {
                        provider.api_key = Some(api_key);
                    }
                })
            })
        }
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        let delete_credentials = cx.delete_credentials(&self.api_url);
        cx.spawn(|mut cx| async move {
            delete_credentials.await.log_err();
            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                if let CompletionProvider::OpenAi(provider) = provider {
                    provider.api_key = None;
                }
            })
        })
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(self.api_url.clone(), cx))
            .into()
    }

    pub fn model(&self) -> OpenAiModel {
        self.model.clone()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        count_open_ai_tokens(request, cx.background_executor())
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = self.to_open_ai_request(request);

        let http_client = self.http_client.clone();
        let api_key = self.api_key.clone();
        let api_url = self.api_url.clone();
        let low_speed_timeout = self.low_speed_timeout;
        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let request = stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                low_speed_timeout,
            );
            let response = request.await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(mut response) => Some(Ok(response.choices.pop()?.delta.content?)),
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        }
        .boxed()
    }

    fn to_open_ai_request(&self, request: LanguageModelRequest) -> Request {
        let model = match request.model {
            LanguageModel::OpenAi(model) => model,
            _ => self.model(),
        };

        Request {
            model,
            messages: request
                .messages
                .into_iter()
                .map(|msg| match msg.role {
                    Role::User => RequestMessage::User {
                        content: msg.content,
                    },
                    Role::Assistant => RequestMessage::Assistant {
                        content: Some(msg.content),
                        tool_calls: Vec::new(),
                    },
                    Role::System => RequestMessage::System {
                        content: msg.content,
                    },
                })
                .collect(),
            stream: true,
            stop: request.stop,
            temperature: request.temperature,
            tools: Vec::new(),
            tool_choice: None,
        }
    }
}

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    background_executor: &gpui::BackgroundExecutor,
) -> BoxFuture<'static, Result<usize>> {
    background_executor
        .spawn(async move {
            let messages = request
                .messages
                .into_iter()
                .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: Some(message.content),
                    name: None,
                    function_call: None,
                })
                .collect::<Vec<_>>();

            match request.model {
                LanguageModel::Anthropic(_)
                | LanguageModel::ZedDotDev(ZedDotDevModel::Claude3Opus)
                | LanguageModel::ZedDotDev(ZedDotDevModel::Claude3Sonnet)
                | LanguageModel::ZedDotDev(ZedDotDevModel::Claude3Haiku) => {
                    // Tiktoken doesn't yet support these models, so we manually use the
                    // same tokenizer as GPT-4.
                    tiktoken_rs::num_tokens_from_messages("gpt-4", &messages)
                }
                _ => tiktoken_rs::num_tokens_from_messages(request.model.id(), &messages),
            }
        })
        .boxed()
}

impl From<Role> for open_ai::Role {
    fn from(val: Role) -> Self {
        match val {
            Role::User => OpenAiRole::User,
            Role::Assistant => OpenAiRole::Assistant,
            Role::System => OpenAiRole::System,
        }
    }
}

struct AuthenticationPrompt {
    api_key: View<Editor>,
    api_url: String,
}

impl AuthenticationPrompt {
    fn new(api_url: String, cx: &mut WindowContext) -> Self {
        Self {
            api_key: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text(
                    "sk-000000000000000000000000000000000000000000000000",
                    cx,
                );
                editor
            }),
            api_url,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let api_key = self.api_key.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let write_credentials = cx.write_credentials(&self.api_url, "Bearer", api_key.as_bytes());
        cx.spawn(|_, mut cx| async move {
            write_credentials.await?;
            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                if let CompletionProvider::OpenAi(provider) = provider {
                    provider.api_key = Some(api_key);
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn render_api_key_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };
        EditorElement::new(
            &self.api_key,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl Render for AuthenticationPrompt {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const INSTRUCTIONS: [&str; 6] = [
            "To use the assistant panel or inline assistant, you need to add your OpenAI API key.",
            " - You can create an API key at: platform.openai.com/api-keys",
            " - Make sure your OpenAI account has credits",
            " - Having a subscription for another service like GitHub Copilot won't work.",
            "",
            "Paste your OpenAI API key below and hit enter to use the assistant:",
        ];

        v_flex()
            .p_4()
            .size_full()
            .on_action(cx.listener(Self::save_api_key))
            .children(
                INSTRUCTIONS.map(|instruction| Label::new(instruction).size(LabelSize::Small)),
            )
            .child(
                h_flex()
                    .w_full()
                    .my_2()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().editor_background)
                    .rounded_md()
                    .child(self.render_api_key_editor(cx)),
            )
            .child(
                Label::new(
                    "You can also assign the OPENAI_API_KEY environment variable and restart Zed.",
                )
                .size(LabelSize::Small),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Click on").size(LabelSize::Small))
                    .child(Icon::new(IconName::Ai).size(IconSize::XSmall))
                    .child(
                        Label::new("in the status bar to close this panel.").size(LabelSize::Small),
                    ),
            )
            .into_any()
    }
}
