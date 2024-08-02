use anyhow::{anyhow, Result};
use collections::BTreeMap;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, FutureExt, StreamExt};
use google_ai::stream_generate_content;
use gpui::{
    AnyView, AppContext, AsyncAppContext, FocusHandle, FocusableView, FontStyle, ModelContext,
    Subscription, Task, TextStyle, View, WhiteSpace,
};
use http_client::HttpClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{future, sync::Arc, time::Duration};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::{prelude::*, Indicator};
use util::ResultExt;

use crate::{
    settings::AllLanguageModelSettings, LanguageModel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter,
};

const PROVIDER_ID: &str = "google";
const PROVIDER_NAME: &str = "Google AI";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleSettings {
    pub api_url: String,
    pub low_speed_timeout: Option<Duration>,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    name: String,
    max_tokens: usize,
}

pub struct GoogleLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Model<State>,
}

pub struct State {
    api_key: Option<String>,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let delete_credentials =
            cx.delete_credentials(&AllLanguageModelSettings::get_global(cx).google.api_url);
        cx.spawn(|this, mut cx| async move {
            delete_credentials.await.ok();
            this.update(&mut cx, |this, cx| {
                this.api_key = None;
                cx.notify();
            })
        })
    }
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
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Model<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for GoogleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiGoogle
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
            models.insert(
                model.name.clone(),
                google_ai::Model::Custom {
                    name: model.name.clone(),
                    max_tokens: model.max_tokens,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(GoogleLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    state: self.state.clone(),
                    http_client: self.http_client.clone(),
                    rate_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut AppContext) -> Task<Result<()>> {
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

    fn configuration_view(&self, cx: &mut WindowContext) -> (AnyView, Option<FocusHandle>) {
        let view = cx.new_view(|cx| ConfigurationView::new(self.state.clone(), cx));

        let focus_handle = view.focus_handle(cx);
        (view.into(), Some(focus_handle))
    }

    fn reset_credentials(&self, cx: &mut AppContext) -> Task<Result<()>> {
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
    rate_limiter: RateLimiter,
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

        let future = self.rate_limiter.stream(async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let response =
                stream_generate_content(http_client.as_ref(), &api_url, &api_key, request);
            let events = response.await?;
            Ok(google_ai::extract_text_from_events(events).boxed())
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn use_any_tool(
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

struct ConfigurationView {
    api_key_editor: View<Editor>,
    state: gpui::Model<State>,
}

impl ConfigurationView {
    fn new(state: gpui::Model<State>, cx: &mut WindowContext) -> Self {
        Self {
            api_key_editor: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("AIzaSy...", cx);
                editor
            }),
            state,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
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

    fn reset_api_key(&mut self, cx: &mut ViewContext<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", cx));
        self.state
            .update(cx, |state, cx| state.reset_api_key(cx))
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
            &self.api_key_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl FocusableView for ConfigurationView {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.api_key_editor.read(cx).focus_handle(cx)
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const INSTRUCTIONS: [&str; 4] = [
            "To use the Google AI assistant, you need to add your Google AI API key.",
            "You can create an API key at: https://makersuite.google.com/app/apikey",
            "",
            "Paste your Google AI API key below and hit enter to use the assistant:",
        ];

        if self.state.read(cx).is_authenticated() {
            h_flex()
                .size_full()
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .child(Indicator::dot().color(Color::Success))
                        .child(Label::new("API Key configured").size(LabelSize::Small)),
                )
                .child(
                    Button::new("reset-key", "Reset key")
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .on_click(cx.listener(|this, _, cx| this.reset_api_key(cx))),
                )
                .into_any()
        } else {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .children(
                    INSTRUCTIONS.map(|instruction| Label::new(instruction)),
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
                .into_any()
        }
    }
}
