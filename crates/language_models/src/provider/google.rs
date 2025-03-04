use anyhow::{anyhow, Context as _, Result};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, FutureExt, StreamExt};
use google_ai::stream_generate_content;
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, Subscription, Task, TextStyle, WhiteSpace,
};
use http_client::HttpClient;
use language_model::{AuthenticateError, LanguageModelCompletionEvent};
use language_model::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, RateLimiter, Role,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::{future, sync::Arc};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::{prelude::*, Icon, IconName, List, Tooltip};
use util::ResultExt;

use crate::ui::InstructionListItem;
use crate::AllLanguageModelSettings;

const PROVIDER_ID: &str = "google";
const PROVIDER_NAME: &str = "Google AI";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    name: String,
    display_name: Option<String>,
    max_tokens: usize,
}

pub struct GoogleLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    _subscription: Subscription,
}

const GOOGLE_AI_API_KEY_VAR: &str = "GOOGLE_AI_API_KEY";

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .google
            .api_url
            .clone();
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .delete_credentials(&api_url, &cx)
                .await
                .log_err();
            this.update(&mut cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    fn set_api_key(&mut self, api_key: String, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .google
            .api_url
            .clone();
        cx.spawn(|this, mut cx| async move {
            credentials_provider
                .write_credentials(&api_url, "Bearer", api_key.as_bytes(), &cx)
                .await?;
            this.update(&mut cx, |this, cx| {
                this.api_key = Some(api_key);
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = AllLanguageModelSettings::get_global(cx)
            .google
            .api_url
            .clone();

        cx.spawn(|this, mut cx| async move {
            let (api_key, from_env) = if let Ok(api_key) = std::env::var(GOOGLE_AI_API_KEY_VAR) {
                (api_key, true)
            } else {
                let (_, api_key) = credentials_provider
                    .read_credentials(&api_url, &cx)
                    .await?
                    .ok_or(AuthenticateError::CredentialsNotFound)?;
                (
                    String::from_utf8(api_key).context("invalid {PROVIDER_NAME} API key")?,
                    false,
                )
            };

            this.update(&mut cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

impl GoogleLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            api_key: None,
            api_key_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        Self { http_client, state }
    }
}

impl LanguageModelProviderState for GoogleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
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

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let model = google_ai::Model::default();
        Some(Arc::new(GoogleLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            rate_limiter: RateLimiter::new(4),
        }))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
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
                    display_name: model.display_name.clone(),
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

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
    }
}

pub struct GoogleLanguageModel {
    id: LanguageModelId,
    model: google_ai::Model,
    state: gpui::Entity<State>,
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
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        let request = into_google(request, self.model.id().to_string());
        let http_client = self.http_client.clone();
        let api_key = self.state.read(cx).api_key.clone();

        let settings = &AllLanguageModelSettings::get_global(cx).google;
        let api_url = settings.api_url.clone();

        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("Missing Google API key"))?;
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
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<LanguageModelCompletionEvent>>>,
    > {
        let request = into_google(request, self.model.id().to_string());

        let http_client = self.http_client.clone();
        let Ok((api_key, api_url)) = cx.read_entity(&self.state, |state, cx| {
            let settings = &AllLanguageModelSettings::get_global(cx).google;
            (state.api_key.clone(), settings.api_url.clone())
        }) else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        let future = self.rate_limiter.stream(async move {
            let api_key = api_key.ok_or_else(|| anyhow!("Missing Google API Key"))?;
            let response =
                stream_generate_content(http_client.as_ref(), &api_url, &api_key, request);
            let events = response.await?;
            Ok(google_ai::extract_text_from_events(events).boxed())
        });
        async move {
            Ok(future
                .await?
                .map(|result| result.map(LanguageModelCompletionEvent::Text))
                .boxed())
        }
        .boxed()
    }

    fn use_any_tool(
        &self,
        _request: LanguageModelRequest,
        _name: String,
        _description: String,
        _schema: serde_json::Value,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<String>>>> {
        future::ready(Err(anyhow!("not implemented"))).boxed()
    }
}

pub fn into_google(
    request: LanguageModelRequest,
    model: String,
) -> google_ai::GenerateContentRequest {
    google_ai::GenerateContentRequest {
        model,
        contents: request
            .messages
            .into_iter()
            .map(|msg| google_ai::Content {
                parts: vec![google_ai::Part::TextPart(google_ai::TextPart {
                    text: msg.string_contents(),
                })],
                role: match msg.role {
                    Role::User => google_ai::Role::User,
                    Role::Assistant => google_ai::Role::Model,
                    Role::System => google_ai::Role::User, // Google AI doesn't have a system role
                },
            })
            .collect(),
        generation_config: Some(google_ai::GenerationConfig {
            candidate_count: Some(1),
            stop_sequences: Some(request.stop),
            max_output_tokens: None,
            temperature: request.temperature.map(|t| t as f64).or(Some(1.0)),
            top_p: None,
            top_k: None,
        }),
        safety_settings: None,
    }
}

pub fn count_google_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<usize>> {
    // We couldn't use the GoogleLanguageModelProvider to count tokens because the github copilot doesn't have the access to google_ai directly.
    // So we have to use tokenizer from tiktoken_rs to count tokens.
    cx.background_spawn(async move {
        let messages = request
            .messages
            .into_iter()
            .map(|message| tiktoken_rs::ChatCompletionRequestMessage {
                role: match message.role {
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::System => "system".into(),
                },
                content: Some(message.string_contents()),
                name: None,
                function_call: None,
            })
            .collect::<Vec<_>>();

        // Tiktoken doesn't yet support these models, so we manually use the
        // same tokenizer as GPT-4.
        tiktoken_rs::num_tokens_from_messages("gpt-4", &messages)
    })
    .boxed()
}

struct ConfigurationView {
    api_key_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            |this, mut cx| async move {
                if let Some(task) = state
                    .update(&mut cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(&mut cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("AIzaSy...", cx);
                editor
            }),
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let state = self.state.clone();
        cx.spawn_in(window, |_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.set_api_key(api_key, cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, |_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.reset_api_key(cx))?
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn render_api_key_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            white_space: WhiteSpace::Normal,
            ..Default::default()
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

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_from_env;

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's assistant with Google AI, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(InstructionListItem::new(
                            "Create one by visiting",
                            Some("Google AI's console"),
                            Some("https://aistudio.google.com/app/apikey"),
                        ))
                        .child(InstructionListItem::text_only(
                            "Paste your API key below and hit enter to start using the assistant",
                        )),
                )
                .child(
                    h_flex()
                        .w_full()
                        .my_2()
                        .px_2()
                        .py_1()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().border_variant)
                        .rounded_md()
                        .child(self.render_api_key_editor(cx)),
                )
                .child(
                    Label::new(
                        format!("You can also assign the {GOOGLE_AI_API_KEY_VAR} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .size_full()
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("API key set in {GOOGLE_AI_API_KEY_VAR} environment variable.")
                        } else {
                            "API key configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset key")
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your API key, unset the {GOOGLE_AI_API_KEY_VAR} environment variable.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                )
                .into_any()
        }
    }
}
