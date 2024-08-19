use super::open_ai::count_open_ai_tokens;
use crate::{
    settings::AllLanguageModelSettings, CloudModel, LanguageModel, LanguageModelCacheConfiguration,
    LanguageModelId, LanguageModelName, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter, ZedModel,
};
use anthropic::AnthropicError;
use anyhow::{anyhow, Result};
use client::{Client, PerformCompletionParams, UserStore, EXPIRED_LLM_TOKEN_HEADER_NAME};
use collections::BTreeMap;
use feature_flags::{FeatureFlagAppExt, ZedPro};
use futures::{
    future::BoxFuture, stream::BoxStream, AsyncBufReadExt, FutureExt, Stream, StreamExt,
    TryStreamExt as _,
};
use gpui::{
    AnyElement, AnyView, AppContext, AsyncAppContext, FontWeight, Model, ModelContext,
    Subscription, Task,
};
use http_client::{AsyncBody, HttpClient, Method, Response};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;
use settings::{Settings, SettingsStore};
use smol::{
    io::{AsyncReadExt, BufReader},
    lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard},
};
use std::{future, sync::Arc};
use strum::IntoEnumIterator;
use ui::prelude::*;

use crate::{LanguageModelAvailability, LanguageModelProvider};

use super::anthropic::count_anthropic_tokens;

pub const PROVIDER_ID: &str = "zed.dev";
pub const PROVIDER_NAME: &str = "Zed";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ZedDotDevSettings {
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AvailableProvider {
    Anthropic,
    OpenAi,
    Google,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    /// The provider of the language model.
    pub provider: AvailableProvider,
    /// The model's name in the provider's API. e.g. claude-3-5-sonnet-20240620
    pub name: String,
    /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
    pub display_name: Option<String>,
    /// The size of the context window, indicating the maximum number of tokens the model can process.
    pub max_tokens: usize,
    /// The maximum number of output tokens allowed by the model.
    pub max_output_tokens: Option<u32>,
    /// Override this model with a different Anthropic model for tool calls.
    pub tool_override: Option<String>,
    /// Indicates whether this custom model supports caching.
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
}

pub struct CloudLanguageModelProvider {
    client: Arc<Client>,
    llm_api_token: LlmApiToken,
    state: gpui::Model<State>,
    _maintain_client_status: Task<()>,
}

pub struct State {
    client: Arc<Client>,
    user_store: Model<UserStore>,
    status: client::Status,
    accept_terms: Option<Task<Result<()>>>,
    _subscription: Subscription,
}

impl State {
    fn is_signed_out(&self) -> bool {
        self.status.is_signed_out()
    }

    fn authenticate(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            client.authenticate_and_connect(true, &cx).await?;
            this.update(&mut cx, |_, cx| cx.notify())
        })
    }

    fn has_accepted_terms_of_service(&self, cx: &AppContext) -> bool {
        self.user_store
            .read(cx)
            .current_user_has_accepted_terms()
            .unwrap_or(false)
    }

    fn accept_terms_of_service(&mut self, cx: &mut ModelContext<Self>) {
        let user_store = self.user_store.clone();
        self.accept_terms = Some(cx.spawn(move |this, mut cx| async move {
            let _ = user_store
                .update(&mut cx, |store, cx| store.accept_terms_of_service(cx))?
                .await;
            this.update(&mut cx, |this, cx| {
                this.accept_terms = None;
                cx.notify()
            })
        }));
    }
}

impl CloudLanguageModelProvider {
    pub fn new(user_store: Model<UserStore>, client: Arc<Client>, cx: &mut AppContext) -> Self {
        let mut status_rx = client.status();
        let status = *status_rx.borrow();

        let state = cx.new_model(|cx| State {
            client: client.clone(),
            user_store,
            status,
            accept_terms: None,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        let state_ref = state.downgrade();
        let maintain_client_status = cx.spawn(|mut cx| async move {
            while let Some(status) = status_rx.next().await {
                if let Some(this) = state_ref.upgrade() {
                    _ = this.update(&mut cx, |this, cx| {
                        if this.status != status {
                            this.status = status;
                            cx.notify();
                        }
                    });
                } else {
                    break;
                }
            }
        });

        Self {
            client,
            state,
            llm_api_token: LlmApiToken::default(),
            _maintain_client_status: maintain_client_status,
        }
    }
}

impl LanguageModelProviderState for CloudLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Model<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for CloudLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn icon(&self) -> IconName {
        IconName::AiZed
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        if cx.is_staff() {
            for model in anthropic::Model::iter() {
                if !matches!(model, anthropic::Model::Custom { .. }) {
                    models.insert(model.id().to_string(), CloudModel::Anthropic(model));
                }
            }
            for model in open_ai::Model::iter() {
                if !matches!(model, open_ai::Model::Custom { .. }) {
                    models.insert(model.id().to_string(), CloudModel::OpenAi(model));
                }
            }
            for model in google_ai::Model::iter() {
                if !matches!(model, google_ai::Model::Custom { .. }) {
                    models.insert(model.id().to_string(), CloudModel::Google(model));
                }
            }
            for model in ZedModel::iter() {
                models.insert(model.id().to_string(), CloudModel::Zed(model));
            }

            // Override with available models from settings
            for model in &AllLanguageModelSettings::get_global(cx)
                .zed_dot_dev
                .available_models
            {
                let model = match model.provider {
                    AvailableProvider::Anthropic => {
                        CloudModel::Anthropic(anthropic::Model::Custom {
                            name: model.name.clone(),
                            display_name: model.display_name.clone(),
                            max_tokens: model.max_tokens,
                            tool_override: model.tool_override.clone(),
                            cache_configuration: model.cache_configuration.as_ref().map(|config| {
                                anthropic::AnthropicModelCacheConfiguration {
                                    max_cache_anchors: config.max_cache_anchors,
                                    should_speculate: config.should_speculate,
                                    min_total_token: config.min_total_token,
                                }
                            }),
                            max_output_tokens: model.max_output_tokens,
                        })
                    }
                    AvailableProvider::OpenAi => CloudModel::OpenAi(open_ai::Model::Custom {
                        name: model.name.clone(),
                        max_tokens: model.max_tokens,
                    }),
                    AvailableProvider::Google => CloudModel::Google(google_ai::Model::Custom {
                        name: model.name.clone(),
                        max_tokens: model.max_tokens,
                    }),
                };
                models.insert(model.id().to_string(), model.clone());
            }
        } else {
            models.insert(
                anthropic::Model::Claude3_5Sonnet.id().to_string(),
                CloudModel::Anthropic(anthropic::Model::Claude3_5Sonnet),
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(CloudLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    llm_api_token: self.llm_api_token.clone(),
                    client: self.client.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        !self.state.read(cx).is_signed_out()
    }

    fn authenticate(&self, _cx: &mut AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn configuration_view(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|_cx| ConfigurationView {
            state: self.state.clone(),
        })
        .into()
    }

    fn must_accept_terms(&self, cx: &AppContext) -> bool {
        !self.state.read(cx).has_accepted_terms_of_service(cx)
    }

    fn render_accept_terms(&self, cx: &mut WindowContext) -> Option<AnyElement> {
        let state = self.state.read(cx);

        let terms = [(
            "terms_of_service",
            "Terms of Service",
            "https://zed.dev/terms-of-service",
        )]
        .map(|(id, label, url)| {
            Button::new(id, label)
                .style(ButtonStyle::Subtle)
                .icon(IconName::ExternalLink)
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .on_click(move |_, cx| cx.open_url(url))
        });

        if state.has_accepted_terms_of_service(cx) {
            None
        } else {
            let disabled = state.accept_terms.is_some();
            Some(
                v_flex()
                    .gap_2()
                    .child(
                        v_flex()
                            .child(Label::new("Terms and Conditions").weight(FontWeight::MEDIUM))
                            .child(
                                Label::new(
                                    "Please read and accept our terms and conditions to continue.",
                                )
                                .size(LabelSize::Small),
                            ),
                    )
                    .child(v_flex().gap_1().children(terms))
                    .child(
                        h_flex().justify_end().child(
                            Button::new("accept_terms", "I've read it and accept it")
                                .disabled(disabled)
                                .on_click({
                                    let state = self.state.downgrade();
                                    move |_, cx| {
                                        state
                                            .update(cx, |state, cx| {
                                                state.accept_terms_of_service(cx)
                                            })
                                            .ok();
                                    }
                                }),
                        ),
                    )
                    .into_any(),
            )
        }
    }

    fn reset_credentials(&self, _cx: &mut AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}

pub struct CloudLanguageModel {
    id: LanguageModelId,
    model: CloudModel,
    llm_api_token: LlmApiToken,
    client: Arc<Client>,
    request_limiter: RateLimiter,
}

#[derive(Clone, Default)]
struct LlmApiToken(Arc<RwLock<Option<String>>>);

impl CloudLanguageModel {
    async fn perform_llm_completion(
        client: Arc<Client>,
        llm_api_token: LlmApiToken,
        body: PerformCompletionParams,
    ) -> Result<Response<AsyncBody>> {
        let http_client = &client.http_client();

        let mut token = llm_api_token.acquire(&client).await?;
        let mut did_retry = false;

        let response = loop {
            let request = http_client::Request::builder()
                .method(Method::POST)
                .uri(http_client.build_zed_llm_url("/completion", &[])?.as_ref())
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(serde_json::to_string(&body)?.into())?;
            let mut response = http_client.send(request).await?;
            if response.status().is_success() {
                break response;
            } else if !did_retry
                && response
                    .headers()
                    .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                    .is_some()
            {
                did_retry = true;
                token = llm_api_token.refresh(&client).await?;
            } else {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;
                break Err(anyhow!(
                    "cloud language model completion failed with status {}: {body}",
                    response.status()
                ))?;
            }
        };

        Ok(response)
    }
}

impl LanguageModel for CloudLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn icon(&self) -> Option<IconName> {
        self.model.icon()
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("zed.dev/{}", self.model.id())
    }

    fn availability(&self) -> LanguageModelAvailability {
        self.model.availability()
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        match self.model.clone() {
            CloudModel::Anthropic(_) => count_anthropic_tokens(request, cx),
            CloudModel::OpenAi(model) => count_open_ai_tokens(request, model, cx),
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = request.into_google(model.id().into());
                let request = google_ai::CountTokensRequest {
                    contents: request.contents,
                };
                async move {
                    let request = serde_json::to_string(&request)?;
                    let response = client
                        .request(proto::CountLanguageModelTokens {
                            provider: proto::LanguageModelProvider::Google as i32,
                            request,
                        })
                        .await?;
                    Ok(response.token_count as usize)
                }
                .boxed()
            }
            CloudModel::Zed(_) => {
                count_open_ai_tokens(request, open_ai::Model::ThreePointFiveTurbo, cx)
            }
        }
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        match &self.model {
            CloudModel::Anthropic(model) => {
                let request = request.into_anthropic(model.id().into(), model.max_output_tokens());
                let client = self.client.clone();
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::Anthropic,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(anthropic::extract_text_from_events(
                        response_lines(response).map_err(AnthropicError::Other),
                    ))
                });
                async move {
                    Ok(future
                        .await?
                        .map(|result| result.map_err(|err| anyhow!(err)))
                        .boxed())
                }
                .boxed()
            }
            CloudModel::OpenAi(model) => {
                let client = self.client.clone();
                let request = request.into_open_ai(model.id().into());
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::OpenAi,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(open_ai::extract_text_from_events(response_lines(response)))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            CloudModel::Google(model) => {
                let client = self.client.clone();
                let request = request.into_google(model.id().into());
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::Google,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(google_ai::extract_text_from_events(response_lines(
                        response,
                    )))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
            CloudModel::Zed(model) => {
                let client = self.client.clone();
                let mut request = request.into_open_ai(model.id().into());
                request.max_tokens = Some(4000);
                let llm_api_token = self.llm_api_token.clone();
                let future = self.request_limiter.stream(async move {
                    let response = Self::perform_llm_completion(
                        client.clone(),
                        llm_api_token,
                        PerformCompletionParams {
                            provider: client::LanguageModelProvider::Zed,
                            model: request.model.clone(),
                            provider_request: RawValue::from_string(serde_json::to_string(
                                &request,
                            )?)?,
                        },
                    )
                    .await?;
                    Ok(open_ai::extract_text_from_events(response_lines(response)))
                });
                async move { Ok(future.await?.boxed()) }.boxed()
            }
        }
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        tool_name: String,
        tool_description: String,
        input_schema: serde_json::Value,
        _cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let client = self.client.clone();
        let llm_api_token = self.llm_api_token.clone();

        match &self.model {
            CloudModel::Anthropic(model) => {
                let mut request =
                    request.into_anthropic(model.tool_model_id().into(), model.max_output_tokens());
                request.tool_choice = Some(anthropic::ToolChoice::Tool {
                    name: tool_name.clone(),
                });
                request.tools = vec![anthropic::Tool {
                    name: tool_name.clone(),
                    description: tool_description,
                    input_schema,
                }];

                self.request_limiter
                    .run(async move {
                        let response = Self::perform_llm_completion(
                            client.clone(),
                            llm_api_token,
                            PerformCompletionParams {
                                provider: client::LanguageModelProvider::Anthropic,
                                model: request.model.clone(),
                                provider_request: RawValue::from_string(serde_json::to_string(
                                    &request,
                                )?)?,
                            },
                        )
                        .await?;

                        Ok(anthropic::extract_tool_args_from_events(
                            tool_name,
                            Box::pin(response_lines(response)),
                        )
                        .await?
                        .boxed())
                    })
                    .boxed()
            }
            CloudModel::OpenAi(model) => {
                let mut request = request.into_open_ai(model.id().into());
                request.tool_choice = Some(open_ai::ToolChoice::Other(
                    open_ai::ToolDefinition::Function {
                        function: open_ai::FunctionDefinition {
                            name: tool_name.clone(),
                            description: None,
                            parameters: None,
                        },
                    },
                ));
                request.tools = vec![open_ai::ToolDefinition::Function {
                    function: open_ai::FunctionDefinition {
                        name: tool_name.clone(),
                        description: Some(tool_description),
                        parameters: Some(input_schema),
                    },
                }];

                self.request_limiter
                    .run(async move {
                        let response = Self::perform_llm_completion(
                            client.clone(),
                            llm_api_token,
                            PerformCompletionParams {
                                provider: client::LanguageModelProvider::OpenAi,
                                model: request.model.clone(),
                                provider_request: RawValue::from_string(serde_json::to_string(
                                    &request,
                                )?)?,
                            },
                        )
                        .await?;

                        Ok(open_ai::extract_tool_args_from_events(
                            tool_name,
                            Box::pin(response_lines(response)),
                        )
                        .await?
                        .boxed())
                    })
                    .boxed()
            }
            CloudModel::Google(_) => {
                future::ready(Err(anyhow!("tool use not implemented for Google AI"))).boxed()
            }
            CloudModel::Zed(model) => {
                // All Zed models are OpenAI-based at the time of writing.
                let mut request = request.into_open_ai(model.id().into());
                request.tool_choice = Some(open_ai::ToolChoice::Other(
                    open_ai::ToolDefinition::Function {
                        function: open_ai::FunctionDefinition {
                            name: tool_name.clone(),
                            description: None,
                            parameters: None,
                        },
                    },
                ));
                request.tools = vec![open_ai::ToolDefinition::Function {
                    function: open_ai::FunctionDefinition {
                        name: tool_name.clone(),
                        description: Some(tool_description),
                        parameters: Some(input_schema),
                    },
                }];

                self.request_limiter
                    .run(async move {
                        let response = Self::perform_llm_completion(
                            client.clone(),
                            llm_api_token,
                            PerformCompletionParams {
                                provider: client::LanguageModelProvider::Zed,
                                model: request.model.clone(),
                                provider_request: RawValue::from_string(serde_json::to_string(
                                    &request,
                                )?)?,
                            },
                        )
                        .await?;

                        Ok(open_ai::extract_tool_args_from_events(
                            tool_name,
                            Box::pin(response_lines(response)),
                        )
                        .await?
                        .boxed())
                    })
                    .boxed()
            }
        }
    }
}

fn response_lines<T: DeserializeOwned>(
    response: Response<AsyncBody>,
) -> impl Stream<Item = Result<T>> {
    futures::stream::try_unfold(
        (String::new(), BufReader::new(response.into_body())),
        move |(mut line, mut body)| async {
            match body.read_line(&mut line).await {
                Ok(0) => Ok(None),
                Ok(_) => {
                    let event: T = serde_json::from_str(&line)?;
                    line.clear();
                    Ok(Some((event, (line, body))))
                }
                Err(e) => Err(e.into()),
            }
        },
    )
}

impl LlmApiToken {
    async fn acquire(&self, client: &Arc<Client>) -> Result<String> {
        let lock = self.0.upgradable_read().await;
        if let Some(token) = lock.as_ref() {
            Ok(token.to_string())
        } else {
            Self::fetch(RwLockUpgradableReadGuard::upgrade(lock).await, &client).await
        }
    }

    async fn refresh(&self, client: &Arc<Client>) -> Result<String> {
        Self::fetch(self.0.write().await, &client).await
    }

    async fn fetch<'a>(
        mut lock: RwLockWriteGuard<'a, Option<String>>,
        client: &Arc<Client>,
    ) -> Result<String> {
        let response = client.request(proto::GetLlmToken {}).await?;
        *lock = Some(response.token.clone());
        Ok(response.token.clone())
    }
}

struct ConfigurationView {
    state: gpui::Model<State>,
}

impl ConfigurationView {
    fn authenticate(&mut self, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, cx| {
            state.authenticate(cx).detach_and_log_err(cx);
        });
        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const ZED_AI_URL: &str = "https://zed.dev/ai";
        const ACCOUNT_SETTINGS_URL: &str = "https://zed.dev/account";

        let is_connected = !self.state.read(cx).is_signed_out();
        let plan = self.state.read(cx).user_store.read(cx).current_plan();
        let must_accept_terms = !self.state.read(cx).has_accepted_terms_of_service(cx);

        let is_pro = plan == Some(proto::Plan::ZedPro);

        if is_connected {
            v_flex()
                .gap_3()
                .max_w_4_5()
                .when(must_accept_terms, |this| {
                    this.child(Label::new(
                        "You must accept the terms of service to use this provider.",
                    ))
                })
                .child(Label::new(
                    if is_pro {
                        "You have full access to Zed's hosted models from Anthropic, OpenAI, Google with faster speeds and higher limits through Zed Pro."
                    } else {
                        "You have basic access to models from Anthropic through the Zed AI Free plan."
                    }))
                .children(if is_pro {
                    Some(
                        h_flex().child(
                            Button::new("manage_settings", "Manage Subscription")
                                .style(ButtonStyle::Filled)
                                .on_click(
                                    cx.listener(|_, _, cx| cx.open_url(ACCOUNT_SETTINGS_URL)),
                                ),
                        ),
                    )
                } else if cx.has_flag::<ZedPro>() {
                    Some(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("learn_more", "Learn more")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(|_, _, cx| cx.open_url(ZED_AI_URL))),
                            )
                            .child(
                                Button::new("upgrade", "Upgrade")
                                    .style(ButtonStyle::Subtle)
                                    .color(Color::Accent)
                                    .on_click(
                                        cx.listener(|_, _, cx| cx.open_url(ACCOUNT_SETTINGS_URL)),
                                    ),
                            ),
                    )
                } else {
                    None
                })
        } else {
            v_flex()
                .gap_6()
                .child(Label::new("Use the zed.dev to access language models."))
                .child(
                    v_flex()
                        .gap_2()
                        .child(
                            Button::new("sign_in", "Sign in")
                                .icon_color(Color::Muted)
                                .icon(IconName::Github)
                                .icon_position(IconPosition::Start)
                                .style(ButtonStyle::Filled)
                                .full_width()
                                .on_click(cx.listener(move |this, _, cx| this.authenticate(cx))),
                        )
                        .child(
                            div().flex().w_full().items_center().child(
                                Label::new("Sign in to enable collaboration.")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                        ),
                )
        }
    }
}
