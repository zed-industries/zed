use anyhow::{Context as _, Result};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use google_ai::GenerateContentResponse;
pub use google_ai::completion::{GoogleEventMapper, into_google};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::{
    ApiKeyConfiguration, AuthenticateError, EnvVar, LanguageModelCompletionError,
    LanguageModelCompletionStream, LanguageModelToolChoiceSupport, unavailable_error,
};
use language_model::{
    GOOGLE_PROVIDER_ID, GOOGLE_PROVIDER_NAME, IconOrSvg, LanguageModel, LanguageModelClient,
    LanguageModelEffortLevel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, ModelRateLimiters, ProviderSettingsView,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use settings::GoogleAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::IconName;

use language_model::ApiKeyState;

const PROVIDER_ID: LanguageModelProviderId = GOOGLE_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = GOOGLE_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        /// The maximum number of tokens to use for reasoning. Must be lower than the model's `max_output_tokens`.
        budget_tokens: Option<u32>,
    },
}

pub struct GoogleLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    request_limiters: ModelRateLimiters,
}

pub struct State {
    api_key_state: ApiKeyState,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

const GEMINI_API_KEY_VAR_NAME: &str = "GEMINI_API_KEY";
const GOOGLE_AI_API_KEY_VAR_NAME: &str = "GOOGLE_AI_API_KEY";

static API_KEY_ENV_VAR: LazyLock<EnvVar> = LazyLock::new(|| {
    // Try GEMINI_API_KEY first as primary, fallback to GOOGLE_AI_API_KEY
    EnvVar::new(GEMINI_API_KEY_VAR_NAME.into()).or(EnvVar::new(GOOGLE_AI_API_KEY_VAR_NAME.into()))
});

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = self.credentials_provider.clone();
        let api_url = GoogleLanguageModelProvider::api_url(cx);
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
        let api_url = GoogleLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl GoogleLanguageModelProvider {
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

        Self {
            http_client,
            state,
            request_limiters: ModelRateLimiters::default(),
        }
    }

    /// Every model this provider offers, keyed by id: the built-in models,
    /// with settings entries added or overriding built-in ones.
    fn google_models(&self, cx: &App) -> BTreeMap<String, google_ai::Model> {
        let mut models = BTreeMap::default();

        for model in google_ai::Model::iter() {
            if !matches!(model, google_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        for model in &GoogleLanguageModelProvider::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                google_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    mode: model.mode.unwrap_or_default(),
                },
            );
        }

        models
    }

    /// The current configuration of `model`, if this provider still offers it.
    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<google_ai::Model, LanguageModelCompletionError> {
        self.google_models(cx)
            .remove(model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))
    }

    fn stream_google_request(
        &self,
        request: google_ai::GenerateContentRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<GenerateContentResponse>>>,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = GoogleLanguageModelProvider::api_url(cx);
            let extra_headers = GoogleLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        async move {
            let api_key = api_key.context("Missing Google API key")?;
            let request = google_ai::stream_generate_content(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
                &extra_headers,
            );
            request.await.context("failed to stream completion")
        }
        .boxed()
    }

    fn settings(cx: &App) -> &GoogleSettings {
        &crate::AllLanguageModelSettings::get_global(cx).google
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            google_ai::API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for GoogleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for GoogleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiGoogle)
    }

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.google_models(cx)
            .get(google_ai::Model::default().id())
            .map(language_model)
    }

    fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        self.google_models(cx)
            .get(google_ai::Model::default_fast().id())
            .map(language_model)
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.google_models(cx)
            .values()
            .map(language_model)
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn settings_view(&self, cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.read(cx);
        Some(ProviderSettingsView::ApiKey(ApiKeyConfiguration::new(
            state.api_key_state.has_key(),
            state.api_key_state.is_from_env_var(),
            state.api_key_state.env_var_name().clone(),
            "https://aistudio.google.com/app/apikey".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }
}

impl LanguageModelClient for GoogleLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        let mut request = request;
        if request.max_output_tokens.is_some() {
            request.max_output_tokens =
                request.effective_max_output_tokens(config.max_output_tokens());
        }
        let request = match into_google(request, config.request_id().to_string(), config.mode()) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let request = self.stream_google_request(request, cx);
        let future = request_limiter.stream(async move {
            let response = request.await.map_err(LanguageModelCompletionError::from)?;
            Ok(GoogleEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

fn language_model(model: &google_ai::Model) -> LanguageModel {
    let default_level = model.default_thinking_level();
    LanguageModel {
        supports_tools: model.supports_tools(),
        supports_images: model.supports_images(),
        supports_thinking: model.supports_thinking(),
        supported_effort_levels: model
            .supported_thinking_levels()
            .iter()
            .map(|level| LanguageModelEffortLevel {
                name: level.name().into(),
                value: level.value().into(),
                is_default: Some(*level) == default_level,
            })
            .collect(),
        tool_choice_support: LanguageModelToolChoiceSupport::ALL,
        max_output_tokens: model.max_output_tokens(),
        ..LanguageModel::new(
            LanguageModelId::from(model.id().to_string()),
            LanguageModelName::from(model.display_name().to_string()),
            PROVIDER_ID,
            PROVIDER_NAME,
            format!("google/{}", model.request_id()),
            model.max_token_count(),
        )
    }
}
