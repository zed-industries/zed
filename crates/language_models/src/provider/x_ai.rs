use anyhow::Result;
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, StreamExt, future::BoxFuture};
use gpui::{App, AppContext, AsyncApp, Context, Entity, SharedString, Task};
use http_client::{CustomHeaders, HttpClient};
use language_model::chat_completion::{ChatCompletionEventMapper, ResponseStreamEvent};
use language_model::{
    ApiKeyConfiguration, ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel,
    LanguageModelCompletionError, LanguageModelCompletionStream, LanguageModelEffortLevel,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoiceSupport, ModelRateLimiters, ProviderSettingsView, RateLimiter, env_var,
    unavailable_error,
};
pub use settings::XaiAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};
use strum::IntoEnumIterator;
use ui::IconName;
use x_ai::XAI_API_URL;

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("x_ai");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("xAI");

const API_KEY_ENV_VAR_NAME: &str = "XAI_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct XAiSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
    pub custom_headers: CustomHeaders,
}

pub struct XAiLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    request_limiters: ModelRateLimiters,
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
        let api_url = XAiLanguageModelProvider::api_url(cx);
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
        let api_url = XAiLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            credentials_provider,
            cx,
        )
    }
}

impl XAiLanguageModelProvider {
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
    fn x_ai_models(&self, cx: &App) -> BTreeMap<String, x_ai::Model> {
        let mut models = BTreeMap::default();

        for model in x_ai::Model::iter() {
            if !matches!(model, x_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        for model in &Self::settings(cx).available_models {
            models.insert(
                model.name.clone(),
                x_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    max_completion_tokens: model.max_completion_tokens,
                    supports_images: model.supports_images,
                    supports_tools: model.supports_tools,
                    parallel_tool_calls: model.parallel_tool_calls,
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
    ) -> Result<x_ai::Model, LanguageModelCompletionError> {
        self.x_ai_models(cx)
            .remove(model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))
    }

    fn stream_chat_completion(
        &self,
        request_limiter: &RateLimiter,
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

        let (api_key, api_url, extra_headers) = self.state.read_with(cx, |state, cx| {
            let api_url = XAiLanguageModelProvider::api_url(cx);
            let extra_headers = XAiLanguageModelProvider::settings(cx)
                .custom_headers
                .clone();
            (state.api_key_state.key(&api_url), api_url, extra_headers)
        });

        let future = request_limiter.stream(async move {
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
                &extra_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn settings(cx: &App) -> &XAiSettings {
        &crate::AllLanguageModelSettings::get_global(cx).x_ai
    }

    fn api_url(cx: &App) -> SharedString {
        let api_url = &Self::settings(cx).api_url;
        if api_url.is_empty() {
            XAI_API_URL.into()
        } else {
            SharedString::new(api_url.as_str())
        }
    }
}

impl LanguageModelProviderState for XAiLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for XAiLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiXAi)
    }

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.x_ai_models(cx)
            .get(x_ai::Model::default().id())
            .map(language_model)
    }

    fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        self.x_ai_models(cx)
            .get(x_ai::Model::default_fast().id())
            .map(language_model)
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        self.x_ai_models(cx).values().map(language_model).collect()
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
            "https://console.x.ai/team/default/api-keys".into(),
        )))
    }

    fn set_api_key(&self, api_key: Option<String>, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(api_key, cx))
    }

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
        let reasoning_effort = reasoning_effort_for_request(&request, &config);
        let request = match crate::provider::open_ai::into_open_ai(
            request,
            config.id(),
            config.supports_parallel_tool_calls(),
            config.supports_prompt_cache_key(),
            config.max_output_tokens(),
            crate::provider::open_ai::ChatCompletionMaxTokensParameter::MaxCompletionTokens,
            reasoning_effort,
            false,
        ) {
            Ok(request) => request,
            Err(error) => return async move { Err(error.into()) }.boxed(),
        };
        let completions = self.stream_chat_completion(&request_limiter, request, cx);
        let executor = cx.background_executor().clone();
        async move {
            let mapper = ChatCompletionEventMapper::new();
            Ok(language_model::stream_in_background(
                mapper.map_stream(completions.await?).boxed(),
                executor,
            ))
        }
        .boxed()
    }
}

fn x_ai_reasoning_efforts(model: &x_ai::Model) -> &'static [open_ai::ReasoningEffort] {
    match model {
        x_ai::Model::Grok43 => &[
            open_ai::ReasoningEffort::None,
            open_ai::ReasoningEffort::Low,
            open_ai::ReasoningEffort::Medium,
            open_ai::ReasoningEffort::High,
        ],
        x_ai::Model::Grok45 => &[
            open_ai::ReasoningEffort::Low,
            open_ai::ReasoningEffort::Medium,
            open_ai::ReasoningEffort::High,
        ],
        x_ai::Model::Grok46 | x_ai::Model::Grok47 => &[
            open_ai::ReasoningEffort::Low,
            open_ai::ReasoningEffort::Medium,
            open_ai::ReasoningEffort::High,
            open_ai::ReasoningEffort::XHigh,
        ],
        _ => &[],
    }
}

fn default_thinking_reasoning_effort(model: &x_ai::Model) -> Option<open_ai::ReasoningEffort> {
    match model {
        x_ai::Model::Grok43 => Some(open_ai::ReasoningEffort::Low),
        x_ai::Model::Grok45 | x_ai::Model::Grok46 | x_ai::Model::Grok47 => {
            Some(open_ai::ReasoningEffort::High)
        }
        _ => None,
    }
}

fn reasoning_effort_for_request(
    request: &LanguageModelRequest,
    model: &x_ai::Model,
) -> Option<open_ai::ReasoningEffort> {
    let supported_efforts = x_ai_reasoning_efforts(model);
    if supported_efforts.is_empty() {
        return None;
    }

    if request.thinking_allowed {
        request
            .thinking_effort
            .as_deref()
            .and_then(|effort| effort.parse::<open_ai::ReasoningEffort>().ok())
            .filter(|effort| supported_efforts.contains(effort))
            .filter(|effort| *effort != open_ai::ReasoningEffort::None)
            .or_else(|| default_thinking_reasoning_effort(model))
    } else if supported_efforts.contains(&open_ai::ReasoningEffort::None) {
        Some(open_ai::ReasoningEffort::None)
    } else {
        None
    }
}

fn supported_thinking_effort_levels(model: &x_ai::Model) -> Vec<LanguageModelEffortLevel> {
    let default_effort = default_thinking_reasoning_effort(model);
    x_ai_reasoning_efforts(model)
        .iter()
        .copied()
        .filter_map(|effort| {
            let (name, value) = match effort {
                open_ai::ReasoningEffort::None => return None,
                open_ai::ReasoningEffort::Minimal => ("Minimal", "minimal"),
                open_ai::ReasoningEffort::Low => ("Low", "low"),
                open_ai::ReasoningEffort::Medium => ("Medium", "medium"),
                open_ai::ReasoningEffort::High => ("High", "high"),
                open_ai::ReasoningEffort::XHigh => ("Extra High", "xhigh"),
                open_ai::ReasoningEffort::Max => return None, // Not supported by any xAI models
            };

            Some(LanguageModelEffortLevel {
                name: name.into(),
                value: value.into(),
                is_default: Some(effort) == default_effort,
            })
        })
        .collect()
}

fn language_model(model: &x_ai::Model) -> LanguageModel {
    LanguageModel {
        supports_tools: model.supports_tool(),
        supports_images: model.supports_images(),
        supports_streaming_tools: true,
        tool_choice_support: LanguageModelToolChoiceSupport::ALL,
        supports_thinking: model.supports_reasoning_effort(),
        supported_effort_levels: supported_thinking_effort_levels(model).into(),
        max_output_tokens: model.max_output_tokens(),
        supports_split_token_display: true,
        ..LanguageModel::new(
            LanguageModelId::from(model.id().to_string()),
            LanguageModelName::from(model.display_name().to_string()),
            PROVIDER_ID,
            PROVIDER_NAME,
            format!("x_ai/{}", model.id()),
            model.max_token_count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grok_43_supports_selectable_thinking_effort_levels() {
        let effort_levels = supported_thinking_effort_levels(&x_ai::Model::Grok43);
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["low", "medium", "high"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("low")
        );
    }

    #[test]
    fn grok_43_request_uses_selected_reasoning_effort() {
        let request = LanguageModelRequest {
            thinking_allowed: true,
            thinking_effort: Some("high".to_string()),
            ..Default::default()
        };

        assert_eq!(
            reasoning_effort_for_request(&request, &x_ai::Model::Grok43),
            Some(open_ai::ReasoningEffort::High)
        );
    }

    #[test]
    fn grok_43_request_uses_none_when_thinking_is_disabled() {
        let request = LanguageModelRequest {
            thinking_allowed: false,
            ..Default::default()
        };

        assert_eq!(
            reasoning_effort_for_request(&request, &x_ai::Model::Grok43),
            Some(open_ai::ReasoningEffort::None)
        );
    }
}
