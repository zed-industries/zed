use anyhow::{anyhow, Result};
use collections::BTreeMap;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, FutureExt, StreamExt};
use gpui::{
    AnyView, AppContext, AsyncAppContext, FontStyle, Subscription, Task, TextStyle, View,
    WhiteSpace,
};
use http_client::HttpClient;
use open_ai::stream_completion;
use settings::{Settings, SettingsStore};
use std::{future, sync::Arc, time::Duration};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::prelude::*;
use util::ResultExt;

use crate::{
    settings::AllLanguageModelSettings, LanguageModel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, Role,
};

const PROVIDER_ID: &str = "openai";
const PROVIDER_NAME: &str = "OpenAI";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiSettings {
    pub api_url: String,
    pub low_speed_timeout: Option<Duration>,
    pub available_models: Vec<open_ai::Model>,
}

pub struct OpenAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Model<State>,
}

struct State {
    api_key: Option<String>,
    _subscription: Subscription,
}

impl OpenAiLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut AppContext) -> Self {
        let state = cx.new_model(|cx| State {
            api_key: None,
            _subscription: cx.observe_global::<SettingsStore>(|_this: &mut State, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }
}

impl LanguageModelProviderState for OpenAiLanguageModelProvider {
    fn subscribe<T: 'static>(&self, cx: &mut gpui::ModelContext<T>) -> Option<gpui::Subscription> {
        Some(cx.observe(&self.state, |_, _, cx| {
            cx.notify();
        }))
    }
}

impl LanguageModelProvider for OpenAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from open_ai::Model::iter()
        for model in open_ai::Model::iter() {
            if !matches!(model, open_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &AllLanguageModelSettings::get_global(cx)
            .openai
            .available_models
        {
            models.insert(model.id().to_string(), model.clone());
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(OpenAiLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    state: self.state.clone(),
                    http_client: self.http_client.clone(),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).api_key.is_some()
    }

    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        if self.is_authenticated(cx) {
            Task::ready(Ok(()))
        } else {
            let api_url = AllLanguageModelSettings::get_global(cx)
                .openai
                .api_url
                .clone();
            let state = self.state.clone();
            cx.spawn(|mut cx| async move {
                let api_key = if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                    api_key
                } else {
                    let (_, api_key) = cx
                        .update(|cx| cx.read_credentials(&api_url))?
                        .await?
                        .ok_or_else(|| anyhow!("credentials not found"))?;
                    String::from_utf8(api_key)?
                };
                state.update(&mut cx, |this, cx| {
                    this.api_key = Some(api_key);
                    cx.notify();
                })
            })
        }
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(self.state.clone(), cx))
            .into()
    }

    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        let settings = &AllLanguageModelSettings::get_global(cx).openai;
        let delete_credentials = cx.delete_credentials(&settings.api_url);
        let state = self.state.clone();
        cx.spawn(|mut cx| async move {
            delete_credentials.await.log_err();
            state.update(&mut cx, |this, cx| {
                this.api_key = None;
                cx.notify();
            })
        })
    }
}

pub struct OpenAiLanguageModel {
    id: LanguageModelId,
    model: open_ai::Model,
    state: gpui::Model<State>,
    http_client: Arc<dyn HttpClient>,
}

impl LanguageModel for OpenAiLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("openai/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        count_open_ai_tokens(request, self.model.clone(), cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<String>>>> {
        let request = request.into_open_ai(self.model.id().into());

        let http_client = self.http_client.clone();
        let Ok((api_key, api_url, low_speed_timeout)) = cx.read_model(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).openai;
            (
                state.api_key.clone(),
                settings.api_url.clone(),
                settings.low_speed_timeout,
            )
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

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
            Ok(open_ai::extract_text_from_events(response).boxed())
        }
        .boxed()
    }

    fn use_tool(
        &self,
        _request: LanguageModelRequest,
        _name: String,
        _description: String,
        _schema: serde_json::Value,
        _cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<serde_json::Value>> {
        future::ready(Err(anyhow!("not implemented"))).boxed()
    }
}

pub fn count_open_ai_tokens(
    request: LanguageModelRequest,
    model: open_ai::Model,
    cx: &AppContext,
) -> BoxFuture<'static, Result<usize>> {
    cx.background_executor()
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

            if let open_ai::Model::Custom { .. } = model {
                tiktoken_rs::num_tokens_from_messages("gpt-4", &messages)
            } else {
                tiktoken_rs::num_tokens_from_messages(model.id(), &messages)
            }
        })
        .boxed()
}

struct AuthenticationPrompt {
    api_key: View<Editor>,
    state: gpui::Model<State>,
}

impl AuthenticationPrompt {
    fn new(state: gpui::Model<State>, cx: &mut WindowContext) -> Self {
        Self {
            api_key: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text(
                    "sk-000000000000000000000000000000000000000000000000",
                    cx,
                );
                editor
            }),
            state,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let api_key = self.api_key.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let settings = &AllLanguageModelSettings::get_global(cx).openai;
        let write_credentials =
            cx.write_credentials(&settings.api_url, "Bearer", api_key.as_bytes());
        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
            write_credentials.await?;
            state.update(&mut cx, |this, cx| {
                this.api_key = Some(api_key);
                cx.notify();
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
            font_fallbacks: settings.ui_font.fallbacks.clone(),
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
                    .child(Icon::new(IconName::ZedAssistant).size(IconSize::XSmall))
                    .child(
                        Label::new("in the status bar to close this panel.").size(LabelSize::Small),
                    ),
            )
            .into_any()
    }
}
