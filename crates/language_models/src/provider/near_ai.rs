use anyhow::{Context as _, Result};
use credentials_provider::CredentialsProvider;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolSchemaFormat, RateLimiter,
};
use menu;
use open_ai::{ResponseStreamEvent, stream_completion};
use serde::Deserialize;
use settings::{Settings, SettingsStore};
use std::collections::HashMap;
use std::sync::Arc;
use ui::{ButtonLink, ConfiguredApiCard, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::open_ai::{OpenAiEventMapper, into_open_ai};

pub use settings::NearAiAvailableModel as AvailableModel;

const NEAR_AI_API_URL: &str = "https://cloud-api.near.ai/v1";
const NEAR_AI_DASHBOARD_URL: &str = "https://cloud.near.ai/dashboard";

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("near_ai");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("NEAR AI");

const API_KEY_ENV_VAR_NAME: &str = "NEAR_AI_API_KEY";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct NearAiSettings {
    pub api_url: String,
    pub auto_discover: bool,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct NearAiModel {
    pub id: String,
    pub object: String,
    #[serde(default)]
    pub context_length: Option<u64>,
    #[serde(default)]
    pub architecture: Option<ModelArchitecture>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelArchitecture {
    #[serde(default)]
    pub input_modalities: Option<Vec<String>>,
    #[serde(default)]
    pub output_modalities: Option<Vec<String>>,
}

impl NearAiModel {
    fn supports_tools(&self) -> bool {
        // Most text models support tools, but embedding/reranker/image models don't
        let is_embedding = self.id.contains("Embedding") || self.id.contains("Reranker");
        let is_image_gen = self
            .architecture
            .as_ref()
            .and_then(|a| a.output_modalities.as_ref())
            .map(|m| m.contains(&"image".to_string()))
            .unwrap_or(false);
        let is_audio = self
            .architecture
            .as_ref()
            .and_then(|a| a.input_modalities.as_ref())
            .map(|m| m.contains(&"audio".to_string()))
            .unwrap_or(false);
        !is_embedding && !is_image_gen && !is_audio
    }

    fn supports_images(&self) -> bool {
        self.id.contains("VL")
            || self.id.contains("vision")
            || self
                .architecture
                .as_ref()
                .and_then(|a| a.input_modalities.as_ref())
                .map(|m| m.contains(&"image".to_string()))
                .unwrap_or(false)
    }

    fn max_tokens(&self) -> u64 {
        match self.context_length {
            Some(0) | None => 128000,
            Some(n) => n,
        }
    }
}

#[derive(Deserialize)]
pub struct ListModelsResponse {
    pub data: Vec<NearAiModel>,
}

pub struct NearAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
    http_client: Arc<dyn HttpClient>,
    fetched_models: Vec<NearAiModel>,
    fetch_model_task: Option<Task<Result<()>>>,
}

impl State {
    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = NearAiLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        self.fetched_models.clear();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = NearAiLanguageModelProvider::api_url(cx);
        let task = self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        );

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| this.restart_fetch_models_task(cx))
                .ok();
            result
        })
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let http_client = Arc::clone(&self.http_client);
        let api_url = NearAiLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);

        cx.spawn(async move |this, cx| {
            let models = get_models(http_client.as_ref(), &api_url, api_key.as_deref()).await?;

            let models: Vec<NearAiModel> = models
                .into_iter()
                .filter(|model| {
                    let is_embedding =
                        model.id.contains("Embedding") || model.id.contains("Reranker");
                    let is_audio = model
                        .architecture
                        .as_ref()
                        .and_then(|a| a.input_modalities.as_ref())
                        .map(|m| m.contains(&"audio".to_string()))
                        .unwrap_or(false);
                    let is_image_gen = model
                        .architecture
                        .as_ref()
                        .and_then(|a| a.output_modalities.as_ref())
                        .map(|m| m.contains(&"image".to_string()))
                        .unwrap_or(false);
                    !is_embedding && !is_audio && !is_image_gen
                })
                .collect();

            this.update(cx, |this, cx| {
                this.fetched_models = models;
                cx.notify();
            })
        })
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        let settings = NearAiLanguageModelProvider::settings(cx);
        if settings.auto_discover {
            let task = self.fetch_models(cx);
            self.fetch_model_task.replace(task);
        }
    }
}

impl NearAiLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        let this = Self {
            http_client: http_client.clone(),
            state: cx.new(|cx| {
                cx.observe_global::<SettingsStore>({
                    let mut last_settings = NearAiLanguageModelProvider::settings(cx).clone();
                    move |this: &mut State, cx| {
                        let current_settings = NearAiLanguageModelProvider::settings(cx);
                        let settings_changed = current_settings != &last_settings;
                        if settings_changed {
                            let url_changed = last_settings.api_url != current_settings.api_url;
                            last_settings = current_settings.clone();
                            if url_changed {
                                let credentials_provider = this.credentials_provider.clone();
                                let api_url = Self::api_url(cx);
                                this.api_key_state.handle_url_change(
                                    api_url,
                                    |this| &mut this.api_key_state,
                                    credentials_provider,
                                    cx,
                                );
                                this.fetched_models.clear();
                                this.authenticate(cx).detach();
                            }
                            cx.notify();
                        }
                    }
                })
                .detach();

                State {
                    http_client,
                    fetched_models: Default::default(),
                    fetch_model_task: None,
                    api_key_state: ApiKeyState::new(
                        Self::api_url(cx),
                        EnvVar::new(API_KEY_ENV_VAR_NAME.into()),
                    ),
                    credentials_provider,
                }
            }),
        };
        this
    }

    fn settings(cx: &App) -> &NearAiSettings {
        &crate::AllLanguageModelSettings::get_global(cx).near_ai
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            NEAR_AI_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }

    fn create_language_model(
        &self,
        model: &NearAiModel,
        display_name: Option<String>,
        cx: &App,
    ) -> Arc<dyn LanguageModel> {
        let settings_model = Self::settings(cx)
            .available_models
            .iter()
            .find(|m| m.name == model.id);

        let (max_tokens, supports_tools, supports_images) = if let Some(sm) = settings_model {
            (
                sm.max_tokens,
                sm.supports_tools.unwrap_or(model.supports_tools()),
                sm.supports_images.unwrap_or(model.supports_images()),
            )
        } else {
            (
                model.max_tokens(),
                model.supports_tools(),
                model.supports_images(),
            )
        };

        Arc::new(NearAiLanguageModel {
            id: LanguageModelId::from(model.id.clone()),
            model: NearAiModelData {
                id: model.id.clone(),
                display_name,
                max_tokens,
                supports_tools,
                supports_images,
            },
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for NearAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for NearAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiNearAi)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let settings = Self::settings(cx);
        if let Some(first) = settings.available_models.first() {
            let fetched = self
                .state
                .read(cx)
                .fetched_models
                .iter()
                .find(|m| m.id == first.name);
            if let Some(model) = fetched {
                return Some(self.create_language_model(model, first.display_name.clone(), cx));
            }
            let fallback = NearAiModel {
                id: first.name.clone(),
                object: "model".to_string(),
                context_length: Some(first.max_tokens),
                architecture: None,
            };
            return Some(self.create_language_model(&fallback, first.display_name.clone(), cx));
        }
        self.state
            .read(cx)
            .fetched_models
            .first()
            .map(|m| self.create_language_model(m, None, cx))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models: HashMap<String, Arc<dyn LanguageModel>> = HashMap::new();
        let settings = Self::settings(cx);

        for model in self.state.read(cx).fetched_models.iter() {
            let lm = self.create_language_model(model, None, cx);
            models.insert(model.id.clone(), lm);
        }

        for settings_model in &settings.available_models {
            let fetched = self
                .state
                .read(cx)
                .fetched_models
                .iter()
                .find(|m| m.id == settings_model.name);
            if let Some(model) = fetched {
                let lm = self.create_language_model(model, settings_model.display_name.clone(), cx);
                models.insert(settings_model.name.clone(), lm);
            } else {
                let fallback = NearAiModel {
                    id: settings_model.name.clone(),
                    object: "model".to_string(),
                    context_length: Some(settings_model.max_tokens),
                    architecture: None,
                };
                let lm =
                    self.create_language_model(&fallback, settings_model.display_name.clone(), cx);
                models.insert(settings_model.name.clone(), lm);
            }
        }

        let mut models: Vec<_> = models.into_values().collect();
        models.sort_by_key(|m| m.name().0);
        models
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        let state = self.state.read(cx);
        if Self::settings(cx).auto_discover {
            state.api_key_state.has_key() && !state.fetched_models.is_empty()
        } else {
            state.api_key_state.has_key()
        }
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
        let state = self.state.clone();
        cx.new(|cx| ConfigurationView::new(state, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct NearAiModelData {
    pub id: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub supports_tools: bool,
    pub supports_images: bool,
}

pub struct NearAiLanguageModel {
    id: LanguageModelId,
    model: NearAiModelData,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl NearAiLanguageModel {
    fn stream_completion(
        &self,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, app| {
            let api_url = NearAiLanguageModelProvider::api_url(app);
            (state.api_key_state.key(&api_url), api_url)
        });

        let provider = PROVIDER_NAME.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_completion(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for NearAiLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(
            self.model
                .display_name
                .clone()
                .unwrap_or_else(|| self.model.id.clone()),
        )
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn supports_tools(&self) -> bool {
        self.model.supports_tools
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model.supports_tools,
            LanguageModelToolChoice::Any => self.model.supports_tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn supports_split_token_display(&self) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        format!("near_ai/{}", self.model.id)
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        None
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let request = into_open_ai(
            request,
            &self.model.id,
            false,
            false,
            self.max_output_tokens(),
            None,
            false,
        );
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

pub async fn get_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<NearAiModel>> {
    let uri = format!("{api_url}/models");
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");

    if let Some(api_key) = api_key {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
    }

    let request = request_builder.body(AsyncBody::default())?;

    let mut response = client.send(request).await?;

    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to connect to NEAR AI API: {} {}",
        response.status(),
        body,
    );

    let response: ListModelsResponse =
        serde_json::from_str(&body).context("Unable to parse NEAR AI models response")?;
    Ok(response.data)
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor =
            cx.new(|cx| InputField::new(window, cx, "sk-000000000000000000000000000000000"));

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
            .update(cx, |input, cx| input.set_text("", window, cx));

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
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }

    fn render_instructions(_cx: &App) -> Div {
        v_flex()
            .gap_2()
            .child(Label::new(
                "Use NEAR AI Cloud to access a variety of AI models with an OpenAI-compatible API.",
            ))
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Sign up and get an API key from"))
                            .child(ButtonLink::new(
                                "NEAR AI Cloud Dashboard",
                                NEAR_AI_DASHBOARD_URL,
                            )),
                    )
                    .child(ListBulletItem::new(
                        "Paste your API key below and hit enter to start using the agent",
                    )),
            )
    }

    fn render_api_key_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable.")
        } else {
            "API key configured".to_string()
        };

        if !state.api_key_state.has_key() {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed.")
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."))
                })
                .into_any_element()
        }
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.load_credentials_task.is_some() {
            return div()
                .child(Label::new("Loading credentials..."))
                .into_any_element();
        }

        v_flex()
            .gap_2()
            .child(Self::render_instructions(cx))
            .child(self.render_api_key_editor(cx))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_models(json: &str) -> Vec<NearAiModel> {
        let response: ListModelsResponse = serde_json::from_str(json).unwrap();
        response.data
    }

    #[test]
    fn parses_camel_case_modalities() {
        // The NEAR AI API returns camelCase keys (`inputModalities`, `outputModalities`).
        // If the struct doesn't rename, audio/image filtering silently breaks and
        // models like Whisper and FLUX leak into the chat model list.
        let json = r#"{"data":[{
            "id":"openai/whisper-large-v3","object":"model",
            "context_length":448,
            "architecture":{"inputModalities":["audio"],"outputModalities":["text"]}
        }]}"#;
        let model = &parse_models(json)[0];
        let arch = model.architecture.as_ref().unwrap();
        assert_eq!(
            arch.input_modalities.as_deref(),
            Some(&["audio".to_string()][..])
        );
        assert_eq!(
            arch.output_modalities.as_deref(),
            Some(&["text".to_string()][..])
        );
    }

    #[test]
    fn filters_non_chat_models() {
        let json = r#"{"data":[
            {"id":"anthropic/claude-sonnet-4-5","object":"model","context_length":200000},
            {"id":"black-forest-labs/FLUX.2-klein-4B","object":"model","context_length":128000,
             "architecture":{"inputModalities":["text"],"outputModalities":["image"]}},
            {"id":"openai/whisper-large-v3","object":"model","context_length":448,
             "architecture":{"inputModalities":["audio"],"outputModalities":["text"]}},
            {"id":"Qwen/Qwen3-Embedding-0.6B","object":"model","context_length":40960},
            {"id":"Qwen/Qwen3-Reranker-0.6B","object":"model","context_length":40960}
        ]}"#;

        let kept: Vec<_> = parse_models(json)
            .into_iter()
            .filter(|m| {
                let is_embedding = m.id.contains("Embedding") || m.id.contains("Reranker");
                let is_audio = m
                    .architecture
                    .as_ref()
                    .and_then(|a| a.input_modalities.as_ref())
                    .map(|x| x.contains(&"audio".to_string()))
                    .unwrap_or(false);
                let is_image_gen = m
                    .architecture
                    .as_ref()
                    .and_then(|a| a.output_modalities.as_ref())
                    .map(|x| x.contains(&"image".to_string()))
                    .unwrap_or(false);
                !is_embedding && !is_audio && !is_image_gen
            })
            .map(|m| m.id)
            .collect();

        assert_eq!(kept, vec!["anthropic/claude-sonnet-4-5".to_string()]);
    }

    #[test]
    fn max_tokens_handles_zero_and_missing() {
        let zero = NearAiModel {
            id: "x".into(),
            object: "model".into(),
            context_length: Some(0),
            architecture: None,
        };
        let missing = NearAiModel {
            id: "x".into(),
            object: "model".into(),
            context_length: None,
            architecture: None,
        };
        let real = NearAiModel {
            id: "x".into(),
            object: "model".into(),
            context_length: Some(200_000),
            architecture: None,
        };
        assert_eq!(zero.max_tokens(), 128_000);
        assert_eq!(missing.max_tokens(), 128_000);
        assert_eq!(real.max_tokens(), 200_000);
    }
}
