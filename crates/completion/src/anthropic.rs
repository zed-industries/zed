use crate::{count_open_ai_tokens, LanguageModelCompletionProvider};
use crate::{CompletionProvider, LanguageModel, LanguageModelRequest};
use anthropic::{stream_completion, Model as AnthropicModel, Request, RequestMessage};
use anyhow::{anyhow, Result};
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, FontStyle, Task, TextStyle, View, WhiteSpace};
use http::HttpClient;
use language_model::Role;
use settings::Settings;
use std::time::Duration;
use std::{env, sync::Arc};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::prelude::*;
use util::ResultExt;

pub struct AnthropicCompletionProvider {
    api_key: Option<String>,
    api_url: String,
    model: AnthropicModel,
    http_client: Arc<dyn HttpClient>,
    low_speed_timeout: Option<Duration>,
    settings_version: usize,
}

impl LanguageModelCompletionProvider for AnthropicCompletionProvider {
    fn available_models(&self) -> Vec<LanguageModel> {
        AnthropicModel::iter()
            .map(LanguageModel::Anthropic)
            .collect()
    }

    fn settings_version(&self) -> usize {
        self.settings_version
    }

    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            let api_url = self.api_url.clone();
            cx.spawn(|mut cx| async move {
                let api_key = if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
                    api_key
                } else {
                    let (_, api_key) = cx
                        .update(|cx| cx.read_credentials(&api_url))?
                        .await?
                        .ok_or_else(|| anyhow!("credentials not found"))?;
                    String::from_utf8(api_key)?
                };
                cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    provider.update_current_as::<_, AnthropicCompletionProvider>(|provider| {
                        provider.api_key = Some(api_key);
                    });
                })
            })
        }
    }

    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        let delete_credentials = cx.delete_credentials(&self.api_url);
        cx.spawn(|mut cx| async move {
            delete_credentials.await.log_err();
            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                provider.update_current_as::<_, AnthropicCompletionProvider>(|provider| {
                    provider.api_key = None;
                });
            })
        })
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(self.api_url.clone(), cx))
            .into()
    }

    fn model(&self) -> LanguageModel {
        LanguageModel::Anthropic(self.model.clone())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        count_open_ai_tokens(request, cx.background_executor())
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = self.to_anthropic_request(request);

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
                        Ok(response) => match response {
                            anthropic::ResponseEvent::ContentBlockStart {
                                content_block, ..
                            } => match content_block {
                                anthropic::ContentBlock::Text { text } => Some(Ok(text)),
                            },
                            anthropic::ResponseEvent::ContentBlockDelta { delta, .. } => {
                                match delta {
                                    anthropic::TextDelta::TextDelta { text } => Some(Ok(text)),
                                }
                            }
                            _ => None,
                        },
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        }
        .boxed()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl AnthropicCompletionProvider {
    pub fn new(
        model: AnthropicModel,
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
        model: AnthropicModel,
        api_url: String,
        low_speed_timeout: Option<Duration>,
        settings_version: usize,
    ) {
        self.model = model;
        self.api_url = api_url;
        self.low_speed_timeout = low_speed_timeout;
        self.settings_version = settings_version;
    }

    fn to_anthropic_request(&self, mut request: LanguageModelRequest) -> Request {
        request.preprocess_anthropic();

        let model = match request.model {
            LanguageModel::Anthropic(model) => model,
            _ => self.model.clone(),
        };

        let mut system_message = String::new();
        if request
            .messages
            .first()
            .map_or(false, |message| message.role == Role::System)
        {
            system_message = request.messages.remove(0).content;
        }

        Request {
            model,
            messages: request
                .messages
                .iter()
                .map(|msg| RequestMessage {
                    role: match msg.role {
                        Role::User => anthropic::Role::User,
                        Role::Assistant => anthropic::Role::Assistant,
                        Role::System => unreachable!("filtered out by preprocess_request"),
                    },
                    content: msg.content.clone(),
                })
                .collect(),
            stream: true,
            system: system_message,
            max_tokens: 4092,
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
                provider.update_current_as::<_, AnthropicCompletionProvider>(|provider| {
                    provider.api_key = Some(api_key);
                });
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
        const INSTRUCTIONS: [&str; 4] = [
            "To use the assistant panel or inline assistant, you need to add your Anthropic API key.",
            "You can create an API key at: https://console.anthropic.com/settings/keys",
            "",
            "Paste your Anthropic API key below and hit enter to use the assistant:",
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
                    "You can also assign the ANTHROPIC_API_KEY environment variable and restart Zed.",
                )
                .size(LabelSize::Small),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Click on").size(LabelSize::Small))
                    .child(Icon::new(IconName::ZedAssistant).size(IconSize::XSmall))
                    .child(
                        Label::new("in the status bar to close this panel.").size(LabelSize::Small),
                    ),
            )
            .into_any()
    }
}
