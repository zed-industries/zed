use anyhow::{Result, Context as AnyhowContext};
use fs::Fs;
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::{AsyncBody, HttpClient, Method};
use serde::Deserialize;
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolSchemaFormat, RateLimiter,
    Role, env_var,
};
use open_ai::ResponseStreamEvent;
pub use settings::ZaiAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore, update_settings_file};
use std::sync::{Arc, LazyLock};
use ui::{ButtonLink, ConfiguredApiCard, ElevationIndex, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("z_ai");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Z-AI");

const API_KEY_ENV_VAR_NAME: &str = "ZAI_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

const DEFAULT_API_URL: &str = "https://api.z.ai/api/paas/v4";

// Известные значения max_tokens для GLM моделей
fn get_model_max_tokens(model_id: &str) -> u64 {
    match model_id {
        "glm-4.5" => 128_000,
        "glm-4.5-air" => 128_000,
        "glm-4.6" => 128_000,
        "glm-4.7" => 128_000,
        _ => 128_000, // Дефолтное значение для GLM моделей
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZAiSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Debug, Deserialize)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    id: String,
    #[serde(skip)]
    _object: (),
    #[serde(skip)]
    _created: Option<u64>,
    #[serde(skip)]
    _owned_by: Option<String>,
}

pub struct ZAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    http_client: Arc<dyn HttpClient>,
    api_key_state: ApiKeyState,
    available_models: Vec<String>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = ZAiLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = ZAiLanguageModelProvider::api_url(cx);
        let auth_task = self.api_key_state
            .load_if_needed(api_url.clone(), |this| &mut this.api_key_state, cx);
        
        // Если аутентификация успешна, загружаем модели
        if self.api_key_state.has_key() {
            let fetch_task = self.fetch_models(cx);
            return cx.spawn(async move |_this, _cx| {
                auth_task.await?;
                fetch_task.await.log_err();
                Ok(())
            });
        }
        
        auth_task
    }

    fn fetch_models(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = ZAiLanguageModelProvider::api_url(cx);
        let api_key = self.api_key_state.key(&api_url);
        
        let http_client = self.http_client.clone();
        cx.spawn(async move |this, cx| {
            let Some(api_key) = api_key else {
                anyhow::bail!("No API key available");
            };
            
            log::debug!("[Z-AI] Fetching models from API: {}", api_url);
            let model_ids = ZAiLanguageModelProvider::fetch_models_from_api(
                http_client.as_ref(),
                &api_url,
                &api_key,
            ).await?;
            
            // Логируем детали каждой модели в момент получения
            for (idx, model_id) in model_ids.iter().enumerate() {
                let max_tokens = get_model_max_tokens(model_id);
                log::debug!(
                    "[Z-AI] Model {}: id={}, max_tokens={}",
                    idx + 1,
                    model_id,
                    max_tokens
                );
            }
            
            this.update(cx, |this, cx| {
                this.available_models = model_ids;
                cx.notify();
            })?;
            Ok(())
        })
    }
}

impl ZAiLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let http_client_clone = http_client.clone();
        let state = cx.new(move |cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let settings = Self::settings(cx);
                log::debug!(
                    "[Z-AI] Settings loaded: {} models configured, api_url={}",
                    settings.available_models.len(),
                    settings.api_url
                );
                let api_url = Self::api_url(cx);
                this.api_key_state
                    .handle_url_change(api_url, |this| &mut this.api_key_state, cx);
                cx.notify();
            })
            .detach();
            let initial_settings = Self::settings(cx);
            log::info!(
                "[Z-AI] Provider initialized: {} models in settings, api_url={}",
                initial_settings.available_models.len(),
                initial_settings.api_url
            );
            State {
                http_client: http_client_clone,
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
                available_models: Vec::new(),
            }
        });

        // Если API ключ уже есть, загружаем модели сразу
        let provider = Self { http_client, state };
        provider.state.update(cx, |state, cx| {
            if state.api_key_state.has_key() {
                let _ = state.fetch_models(cx);
            }
        });
        provider
    }

    async fn fetch_models_from_api(
        http_client: &dyn HttpClient,
        api_url: &str,
        api_key: &str,
    ) -> Result<Vec<String>> {
        let uri = format!("{}/models", api_url);
        log::debug!("[Z-AI] Fetching models from API: {}", uri);
        
        let request = http_client::Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(AsyncBody::default())?;

        let mut response = http_client
            .send(request)
            .await
            .context("failed to send list models request")?;

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        log::debug!("[Z-AI] API response status: {}, body: {}", response.status(), body);

        if response.status().is_success() {
            let models_response: ModelsResponse = serde_json::from_str(&body)
                .context("failed to parse models response")?;
            
            let model_ids: Vec<String> = models_response.data.into_iter().map(|m| m.id).collect();
            log::info!("[Z-AI] Successfully fetched {} models from API: {:?}", model_ids.len(), model_ids);
            Ok(model_ids)
        } else {
            anyhow::bail!(
                "error listing models.\nStatus: {:?}\nBody: {}",
                response.status(),
                body,
            );
        }
    }

    fn create_language_model(&self, model: CustomModel) -> Arc<dyn LanguageModel> {
        Arc::new(ZAiLanguageModel {
            id: LanguageModelId::from(model.id.clone()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    fn settings(cx: &App) -> &ZAiSettings {
        let settings = &crate::AllLanguageModelSettings::get_global(cx).z_ai;
        log::debug!(
            "[Z-AI] Reading settings: api_url={}, available_models.len()={}",
            settings.api_url,
            settings.available_models.len()
        );
        settings
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            DEFAULT_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

#[derive(Clone, Debug)]
struct CustomModel {
    id: String,
    display_name: String,
    max_tokens: u64,
    max_output_tokens: Option<u64>,
    supports_images: bool,
    supports_tools: bool,
    parallel_tool_calls: bool,
}

impl CustomModel {
    fn from_available_model(model: &AvailableModel) -> Self {
        Self {
            id: model.name.clone(),
            display_name: model.display_name.clone().unwrap_or_else(|| model.name.clone()),
            max_tokens: model.max_tokens,
            max_output_tokens: model.max_output_tokens,
            supports_images: model.supports_images.unwrap_or(false),
            supports_tools: model.supports_tools.unwrap_or(true),
            parallel_tool_calls: model.parallel_tool_calls.unwrap_or(false),
        }
    }
}

impl LanguageModelProviderState for ZAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for ZAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAi)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let models = self.provided_models(cx);
        models.first().cloned()
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        self.default_model(cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let settings = Self::settings(cx);
        let state = self.state.read(cx);

        let mut models = Vec::new();

        // Сначала используем модели из настроек (приоритет)
        if !settings.available_models.is_empty() {
            for available_model in settings.available_models.iter() {
                let custom_model = CustomModel::from_available_model(available_model);
                models.push(self.create_language_model(custom_model));
            }
        } else if !state.available_models.is_empty() {
            // Если моделей нет в настройках, используем модели из API
            for model_id in state.available_models.iter() {
                let max_tokens = get_model_max_tokens(model_id);
                // Создаем модель с параметрами на основе известных значений для GLM
                let custom_model = CustomModel {
                    id: model_id.clone(),
                    display_name: model_id.clone(),
                    max_tokens,
                    max_output_tokens: Some(8192), // Разумное значение для output
                    supports_images: false,
                    supports_tools: true,
                    parallel_tool_calls: false,
                };
                models.push(self.create_language_model(custom_model));
            }
        } else {
            log::debug!("[Z-AI] No models available - neither in settings nor from API");
        }

        models
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

pub struct ZAiLanguageModel {
    id: LanguageModelId,
    model: CustomModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl ZAiLanguageModel {
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

        let (api_key, api_url) = self.state.read_with(cx, |state, cx| {
            let api_url = ZAiLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        let future = self.request_limiter.stream(async move {
            let provider = PROVIDER_NAME;
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = open_ai::stream_completion(
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

impl LanguageModel for ZAiLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name.clone())
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

    fn supports_images(&self) -> bool {
        self.model.supports_images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchema
    }

    fn telemetry_id(&self) -> String {
        format!("z_ai/{}", self.model.id)
    }

    fn max_token_count(&self) -> u64 {
        let max_tokens = self.model.max_tokens;
        log::debug!(
            "[Z-AI] max_token_count for model {}: {}",
            self.model.id,
            max_tokens
        );
        max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        let max_output = self.model.max_output_tokens;
        log::debug!(
            "[Z-AI] max_output_tokens for model {}: {:?}",
            self.model.id,
            max_output
        );
        max_output
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        count_zai_tokens(request, cx)
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
        let request = crate::provider::open_ai::into_open_ai(
            request,
            &self.model.id,
            self.model.parallel_tool_calls,
            false,
            self.max_output_tokens(),
            None,
        );
        let completions = self.stream_completion(request, cx);
        async move {
            let mapper = crate::provider::open_ai::OpenAiEventMapper::new();
            Ok(mapper.map_stream(completions.await?).boxed())
        }
        .boxed()
    }
}

pub fn count_zai_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
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

        let model_name = "gpt-4";
        tiktoken_rs::num_tokens_from_messages(model_name, &messages).map(|tokens| tokens as u64)
    })
    .boxed()
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    api_url_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "000000000000000000000000000000000000000000000000",
            )
            .label("API key")
        });

        let api_url_editor = cx.new(|cx| {
            let input = InputField::new(window, cx, DEFAULT_API_URL).label("API URL");
            input.set_text(ZAiLanguageModelProvider::api_url(cx), window, cx);
            input
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
            api_url_editor,
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
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }

    fn save_api_url(&self, cx: &mut Context<Self>) {
        let api_url = self.api_url_editor.read(cx).text(cx).trim().to_string();
        let current_url = ZAiLanguageModelProvider::api_url(cx).to_string();
        if !api_url.is_empty() && api_url != current_url {
            let fs = <dyn Fs>::global(cx);
            update_settings_file(fs, cx, move |settings, _| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .z_ai
                    .get_or_insert_default()
                    .api_url = Some(api_url);
            });
        }
    }

    fn reset_api_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_url_editor
            .update(cx, |input, cx| input.set_text(DEFAULT_API_URL, window, cx));
        let fs = <dyn Fs>::global(cx);
        update_settings_file(fs, cx, |settings, _cx| {
            if let Some(settings) = settings
                .language_models
                .as_mut()
                .and_then(|models| models.z_ai.as_mut())
            {
                settings.api_url = Some(DEFAULT_API_URL.to_string());
            }
        });
        cx.notify();
    }

    fn render_api_url_editor(&self, cx: &Context<Self>) -> impl IntoElement {
        let api_url = ZAiLanguageModelProvider::api_url(cx);
        let custom_api_url_set = api_url.as_str() != DEFAULT_API_URL;

        v_flex()
            .gap_2()
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                this.save_api_url(cx);
                cx.notify();
            }))
            .child(self.api_url_editor.clone())
            .when(custom_api_url_set, |this| {
                this.child(
                    h_flex()
                        .p_2()
                        .justify_between()
                        .rounded_md()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().elevated_surface_background)
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Icon::new(IconName::Check).color(Color::Success))
                                .child(Label::new(format!("Using custom URL: {}", api_url)).size(LabelSize::Small)),
                        )
                        .child(
                            Button::new("reset-api-url", "Reset")
                                .label_size(LabelSize::Small)
                                .icon(IconName::Undo)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .layer(ElevationIndex::ModalSurface)
                                .on_click(
                                    cx.listener(|this, _, window, cx| this.reset_api_url(window, cx)),
                                ),
                        ),
                )
            })
            .into_any_element()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!("API key set in {API_KEY_ENV_VAR_NAME} environment variable")
        } else {
            let api_url = ZAiLanguageModelProvider::api_url(cx);
            if api_url == DEFAULT_API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new("To use Zed's agent with Z-AI, you need to add an API key. Follow these steps:"))
                .child(
                    List::new()
                        .child(
                            ListBulletItem::new("")
                                .child(Label::new("Create one by visiting"))
                                .child(ButtonLink::new("Z-AI console", "https://z.ai/manage-apikey/apikey-list"))
                        )
                        .child(
                            ListBulletItem::new("Paste your API key below and hit enter to start using the agent")
                        ),
                )
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(format!(
                        "You can also set the {API_KEY_ENV_VAR_NAME} environment variable and restart Zed."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .child(
                    Label::new("Note that Z-AI is an OpenAI-compatible provider.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .when(env_var_set, |this| {
                    this.tooltip_label(format!("To reset your API key, unset the {API_KEY_ENV_VAR_NAME} environment variable."))
                })
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .into_any_element()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex()
                .size_full()
                .gap_2()
                .child(self.render_api_url_editor(cx))
                .child(api_key_section)
                .into_any()
        }
    }
}

