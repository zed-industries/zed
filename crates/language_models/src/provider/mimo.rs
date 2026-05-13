use anyhow::Result;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, TaskExt, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelEffortLevel, LanguageModelId, LanguageModelName,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, LanguageModelToolChoice, RateLimiter,
    ReasoningEffort, env_var,
};
use open_ai::stream_completion;
pub use settings::MimoAvailableModel as AvailableModel;
use settings::{MimoRegion, Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};

use crate::provider::open_ai::{
    OpenAiEventMapper, into_open_ai, strip_openai_extensions_from_request,
};

use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("mimo");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Xiaomi MiMo");

const API_KEY_ENV_VAR_NAME: &str = "MIMO_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct MimoSettings {
    pub api_url: String,
    pub region: settings::MimoRegion,
    pub available_models: Vec<AvailableModel>,
}

pub struct MimoLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = MimoLanguageModelProvider::api_url(cx);
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = MimoLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl MimoLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let credentials_provider = this.credentials_provider.clone();
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    |this| &mut this.api_key_state,
                    credentials_provider,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                credentials_provider,
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: MimoModel) -> Arc<dyn LanguageModel> {
        Arc::new(MimoLanguageModel {
            id: LanguageModelId::from(format!("{}/{}", PROVIDER_ID, model.id())),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &MimoSettings {
        &crate::AllLanguageModelSettings::get_global(cx).mimo
    }

    fn api_url(cx: &App) -> SharedString {
        let settings = Self::settings(cx);
        if !settings.api_url.is_empty() {
            SharedString::new(settings.api_url.as_str())
        } else {
            settings.region.api_url().into()
        }
    }
}

impl LanguageModelProviderState for MimoLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for MimoLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(MimoModel::V2_5Pro))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(MimoModel::V2_5))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = Vec::new();

        models.push(MimoModel::V2_5Pro);
        models.push(MimoModel::V2_5);

        for available_model in &Self::settings(cx).available_models {
            models.push(MimoModel::Custom {
                name: available_model.name.clone(),
                display_name: available_model.display_name.clone(),
                max_tokens: available_model.max_tokens,
                max_output_tokens: available_model.max_output_tokens,
            });
        }

        models
            .into_iter()
            .map(|model| self.create_language_model(model))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct MimoLanguageModel {
    id: LanguageModelId,
    model: MimoModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MimoModel {
    V2_5Pro,
    V2_5,
    Custom {
        name: String,
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
    },
}

impl MimoModel {
    pub fn id(&self) -> &str {
        match self {
            Self::V2_5Pro => "mimo-v2.5-pro",
            Self::V2_5 => "mimo-v2.5",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::V2_5Pro => "MiMo V2.5 Pro",
            Self::V2_5 => "MiMo V2.5",
            Self::Custom {
                display_name, name, ..
            } => display_name.as_deref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::V2_5Pro | Self::V2_5 => 1_048_576,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::V2_5Pro | Self::V2_5 => Some(131_072),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }
}

impl MimoLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<open_ai::ResponseStreamEvent>>>> {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = MimoLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                });
            };
            let provider_name = PROVIDER_NAME.0.to_string();
            let request =
                stream_completion(http_client.as_ref(), &provider_name, &api_url, &api_key, request);
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for MimoLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_thinking(&self) -> bool {
        true
    }

    fn supported_effort_levels(&self) -> Vec<LanguageModelEffortLevel> {
        vec![
            LanguageModelEffortLevel {
                name: "Low".into(),
                value: "low".into(),
                is_default: false,
            },
            LanguageModelEffortLevel {
                name: "Medium".into(),
                value: "medium".into(),
                is_default: false,
            },
            LanguageModelEffortLevel {
                name: "High".into(),
                value: "high".into(),
                is_default: true,
            },
        ]
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        true
    }

    fn supports_images(&self) -> bool {
        matches!(self.model, MimoModel::V2_5 | MimoModel::Custom { .. })
    }

    fn telemetry_id(&self) -> String {
        format!("mimo/{}", self.model.id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let thinking_allowed = request.thinking_allowed;

        let reasoning_effort = if thinking_allowed {
            request
                .thinking_effort
                .as_deref()
                .and_then(|effort| match effort.trim().to_ascii_lowercase().as_str() {
                    "low" => Some(ReasoningEffort::Low),
                    "medium" => Some(ReasoningEffort::Medium),
                    "high" | "" => Some(ReasoningEffort::High),
                    _ => Some(ReasoningEffort::High),
                })
                .or(Some(ReasoningEffort::High))
        } else {
            None
        };

        let mut openai_request = into_open_ai(
            request,
            self.model.id(),
            false,
            false,
            self.model.max_output_tokens(),
            reasoning_effort,
            true,
        );

        strip_openai_extensions_from_request(&mut openai_request);

        if thinking_allowed {
            openai_request.extra_body = Some(serde_json::json!({
                "thinking": { "type": "enabled" }
            }));
        }

        let stream = self.stream_completion(openai_request, cx);

        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(stream.await?).boxed())
        }
        .boxed()
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(window, cx, "").label("API key")
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn set_region(&self, region: MimoRegion, cx: &mut Context<Self>) {
        let fs = <dyn fs::Fs>::global(cx);
        let region_clone = region.clone();
        update_settings_file(fs, cx, move |settings, _| {
            settings
                .language_models
                .get_or_insert_default()
                .mimo
                .get_or_insert_default()
                .region = Some(region_clone.clone());
        });
        cx.notify();
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let current_region = MimoLanguageModelProvider::settings(cx).region.clone();

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(
                    Label::new("To use Xiaomi MiMo models in Zed, you need an API key:")
                )
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Get your key at "))
                                .child(ButtonLink::new(
                                    "platform.xiaomimimo.com",
                                    "https://platform.xiaomimimo.com",
                                )),
                        )
                        .child(ListBulletItem::new(
                            "Paste your API key below and hit enter",
                        )),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(format!(
                "API key configured{}",
                if env_var_set {
                    format!(" (via {API_KEY_ENV_VAR_NAME})")
                } else {
                    String::new()
                }
            ))
            .disabled(env_var_set)
            .when(env_var_set, |this| {
                this.tooltip_label(format!(
                    "To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."
                ))
            })
            .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
            .into_any_element()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else {
            let regions = [
                (MimoRegion::Singapore, "Singapore"),
                (MimoRegion::China, "China"),
                (MimoRegion::Europe, "Europe"),
                (MimoRegion::PayAsYouGo, "Pay-as-you-go"),
            ];

            v_flex()
                .size_full()
                .gap_2()
                .child(api_key_section)
                .child(
                    v_flex()
                        .gap_1()
                        .child(Label::new("API Region").color(Color::Muted))
                        .child(
                            h_flex()
                                .gap_1()
                                .children(regions.map(|(region, label)| {
                                    let is_selected = current_region == region;
                                    let region_clone = region.clone();
                                    Button::new(
                                        format!("mimo-region-{label}"),
                                        label,
                                    )
                                    .toggle_state(is_selected)
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.set_region(region_clone.clone(), cx);
                                    }))
                                })),
                        ),
                )
                .into_any()
        }
    }
}
