use anyhow::{anyhow, Result};
use collections::BTreeMap;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, FutureExt, StreamExt};
use google_ai::stream_generate_content;
use gpui::{
    AnyView, AppContext, AsyncAppContext, FontStyle, Subscription, Task, TextStyle, View,
    WhiteSpace,
};
use http_client::HttpClient;
use settings::{Settings, SettingsStore};
use std::{future, sync::Arc, time::Duration};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::prelude::*;
use util::ResultExt;

use crate::{
    settings::AllLanguageModelSettings, LanguageModel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest,
};

const PROVIDER_ID: &str = "google";
const PROVIDER_NAME: &str = "Google AI";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleSettings {
    pub api_url: String,
    pub low_speed_timeout: Option<Duration>,
    pub available_models: Vec<google_ai::Model>,
}

pub struct GoogleLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Model<State>,
}

struct State {
    api_key: Option<String>,
    _subscription: Subscription,
}

impl GoogleLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut AppContext) -> Self {
        let state = cx.new_model(|cx| State {
            api_key: None,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }
}

impl LanguageModelProviderState for GoogleLanguageModelProvider {
    fn subscribe<T: 'static>(&self, cx: &mut gpui::ModelContext<T>) -> Option<gpui::Subscription> {
        Some(cx.observe(&self.state, |_, _, cx| {
            cx.notify();
        }))
    }
}

impl LanguageModelProvider for GoogleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from google_ai::Model::iter()
        for model in google_ai::Model::iter() {
            if !matches!(model, google_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in &AllLanguageModelSettings::get_global(cx)
            .google
            .available_models
        {
            models.insert(model.id().to_string(), model.clone());
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(GoogleLanguageModel {
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
                .google
                .api_url
                .clone();
            let state = self.state.clone();
            cx.spawn(|mut cx| async move {
                let api_key = if let Ok(api_key) = std::env::var("GOOGLE_AI_API_KEY") {
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
        let state = self.state.clone();
        let delete_credentials =
            cx.delete_credentials(&AllLanguageModelSettings::get_global(cx).google.api_url);
        cx.spawn(|mut cx| async move {
            delete_credentials.await.log_err();
            state.update(&mut cx, |this, cx| {
                this.api_key = None;
                cx.notify();
            })
        })
    }
}

pub struct GoogleLanguageModel {
    id: LanguageModelId,
    model: google_ai::Model,
    state: gpui::Model<State>,
    http_client: Arc<dyn HttpClient>,
}

impl LanguageModel for GoogleLanguageModel {
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
        format!("google/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        let request = request.into_google(self.model.id().to_string());
        let http_client = self.http_client.clone();
        let api_key = self.state.read(cx).api_key.clone();
        let api_url = AllLanguageModelSettings::get_global(cx)
            .google
            .api_url
            .clone();

        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let response = google_ai::count_tokens(
                http_client.as_ref(),
                &api_url,
                &api_key,
                google_ai::CountTokensRequest {
                    contents: request.contents,
                },
            )
            .await?;
            Ok(response.total_tokens)
        }
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<String>>>> {
        let request = request.into_google(self.model.id().to_string());

        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = cx.read_model(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).google;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let response =
                stream_generate_content(http_client.as_ref(), &api_url, &api_key, request);
            let events = response.await?;
            Ok(google_ai::extract_text_from_events(events).boxed())
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

struct AuthenticationPrompt {
    api_key: View<Editor>,
    state: gpui::Model<State>,
}

impl AuthenticationPrompt {
    fn new(state: gpui::Model<State>, cx: &mut WindowContext) -> Self {
        Self {
            api_key: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("AIzaSy...", cx);
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

        let settings = &AllLanguageModelSettings::get_global(cx).google;
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
        const INSTRUCTIONS: [&str; 4] = [
            "To use the Google AI assistant, you need to add your Google AI API key.",
            "You can create an API key at: https://makersuite.google.com/app/apikey",
            "",
            "Paste your Google AI API key below and hit enter to use the assistant:",
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
                    "You can also assign the GOOGLE_AI_API_KEY environment variable and restart Zed.",
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
