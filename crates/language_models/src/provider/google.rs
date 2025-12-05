use anyhow::{Context as _, Result, anyhow};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, Stream, StreamExt, future, future::BoxFuture};
use google_ai::{
    FunctionDeclaration, GenerateContentResponse, GoogleModelMode, Part, SystemInstruction,
    ThinkingConfig, UsageMetadata,
};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelToolChoice, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, LanguageModelToolUseId, MessageContent, StopReason,
};
use language_model::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, RateLimiter, Role,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use settings::GoogleAvailableModel as AvailableModel;
use settings::{Settings, SettingsStore};
use std::pin::Pin;
use std::sync::{
    Arc, LazyLock,
    atomic::{self, AtomicU64},
};
use strum::IntoEnumIterator;
use ui::{List, prelude::*};
use ui_input::InputField;
use util::ResultExt;
use zed_env_vars::EnvVar;

use crate::api_key::ApiKey;
use crate::api_key::ApiKeyState;
use crate::ui::{ConfiguredApiCard, InstructionListItem};

const PROVIDER_ID: LanguageModelProviderId = language_model::GOOGLE_PROVIDER_ID;
const PROVIDER_NAME: LanguageModelProviderName = language_model::GOOGLE_PROVIDER_NAME;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
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
}

pub struct State {
    api_key_state: ApiKeyState,
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
        let api_url = GoogleLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = GoogleLanguageModelProvider::api_url(cx);
        self.api_key_state.load_if_needed(
            api_url,
            &API_KEY_ENV_VAR,
            |this| &mut this.api_key_state,
            cx,
        )
    }
}

impl GoogleLanguageModelProvider {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let api_url = Self::api_url(cx);
                this.api_key_state.handle_url_change(
                    api_url,
                    &API_KEY_ENV_VAR,
                    |this| &mut this.api_key_state,
                    cx,
                );
                cx.notify();
            })
            .detach();
            State {
                api_key_state: ApiKeyState::new(Self::api_url(cx)),
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: google_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(GoogleLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }

    pub fn api_key_for_gemini_cli(cx: &mut App) -> Task<Result<String>> {
        if let Some(key) = API_KEY_ENV_VAR.value.clone() {
            return Task::ready(Ok(key));
        }
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        let api_url = Self::api_url(cx).to_string();
        cx.spawn(async move |cx| {
            Ok(
                ApiKey::load_from_system_keychain(&api_url, credentials_provider.as_ref(), cx)
                    .await?
                    .key()
                    .to_string(),
            )
        })
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

    fn icon(&self) -> IconName {
        IconName::AiGoogle
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(google_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(google_ai::Model::default_fast()))
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
            .into_values()
            .map(|model| {
                Arc::new(GoogleLanguageModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    state: self.state.clone(),
                    http_client: self.http_client.clone(),
                    request_limiter: RateLimiter::new(4),
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

    fn configuration_view(
        &self,
        target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), target_agent, window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct GoogleLanguageModel {
    id: LanguageModelId,
    model: google_ai::Model,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl GoogleLanguageModel {
    fn stream_completion(
        &self,
        request: google_ai::GenerateContentRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<GenerateContentResponse>>>,
    > {
        let http_client = self.http_client.clone();

        let Ok((api_key, api_url)) = self.state.read_with(cx, |state, cx| {
            let api_url = GoogleLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        }) else {
            return future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        async move {
            let api_key = api_key.context("Missing Google API key")?;
            let request = google_ai::stream_generate_content(
                http_client.as_ref(),
                &api_url,
                &api_key,
                request,
            );
            request.await.context("failed to stream completion")
        }
        .boxed()
    }
}

impl LanguageModel for GoogleLanguageModel {
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
        self.model.supports_tools()
    }

    fn supports_images(&self) -> bool {
        self.model.supports_images()
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto
            | LanguageModelToolChoice::Any
            | LanguageModelToolChoice::None => true,
        }
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn telemetry_id(&self) -> String {
        format!("google/{}", self.model.request_id())
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let model_id = self.model.request_id().to_string();
        let request = into_google(request, model_id, self.model.mode());
        let http_client = self.http_client.clone();
        let api_url = GoogleLanguageModelProvider::api_url(cx);
        let api_key = self.state.read(cx).api_key_state.key(&api_url);

        async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey {
                    provider: PROVIDER_NAME,
                }
                .into());
            };
            let response = google_ai::count_tokens(
                http_client.as_ref(),
                &api_url,
                &api_key,
                google_ai::CountTokensRequest {
                    generate_content_request: request,
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
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        let request = into_google(
            request,
            self.model.request_id().to_string(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.await.map_err(LanguageModelCompletionError::from)?;
            Ok(GoogleEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn into_google(
    mut request: LanguageModelRequest,
    model_id: String,
    mode: GoogleModelMode,
) -> google_ai::GenerateContentRequest {
    fn map_content(content: Vec<MessageContent>) -> Vec<Part> {
        content
            .into_iter()
            .flat_map(|content| match content {
                language_model::MessageContent::Text(text) => {
                    if !text.is_empty() {
                        vec![Part::TextPart(google_ai::TextPart { text })]
                    } else {
                        vec![]
                    }
                }
                language_model::MessageContent::Thinking {
                    text: _,
                    signature: Some(signature),
                } => {
                    if !signature.is_empty() {
                        vec![Part::ThoughtPart(google_ai::ThoughtPart {
                            thought: true,
                            thought_signature: signature,
                        })]
                    } else {
                        vec![]
                    }
                }
                language_model::MessageContent::Thinking { .. } => {
                    vec![]
                }
                language_model::MessageContent::RedactedThinking(_) => vec![],
                language_model::MessageContent::Image(image) => {
                    vec![Part::InlineDataPart(google_ai::InlineDataPart {
                        inline_data: google_ai::GenerativeContentBlob {
                            mime_type: "image/png".to_string(),
                            data: image.source.to_string(),
                        },
                    })]
                }
                language_model::MessageContent::ToolUse(tool_use) => {
                    // Normalize empty string signatures to None
                    let thought_signature = tool_use.thought_signature.filter(|s| !s.is_empty());

                    vec![Part::FunctionCallPart(google_ai::FunctionCallPart {
                        function_call: google_ai::FunctionCall {
                            name: tool_use.name.to_string(),
                            args: tool_use.input,
                        },
                        thought_signature,
                    })]
                }
                language_model::MessageContent::ToolResult(tool_result) => {
                    match tool_result.content {
                        language_model::LanguageModelToolResultContent::Text(text) => {
                            vec![Part::FunctionResponsePart(
                                google_ai::FunctionResponsePart {
                                    function_response: google_ai::FunctionResponse {
                                        name: tool_result.tool_name.to_string(),
                                        // The API expects a valid JSON object
                                        response: serde_json::json!({
                                            "output": text
                                        }),
                                    },
                                },
                            )]
                        }
                        language_model::LanguageModelToolResultContent::Image(image) => {
                            vec![
                                Part::FunctionResponsePart(google_ai::FunctionResponsePart {
                                    function_response: google_ai::FunctionResponse {
                                        name: tool_result.tool_name.to_string(),
                                        // The API expects a valid JSON object
                                        response: serde_json::json!({
                                            "output": "Tool responded with an image"
                                        }),
                                    },
                                }),
                                Part::InlineDataPart(google_ai::InlineDataPart {
                                    inline_data: google_ai::GenerativeContentBlob {
                                        mime_type: "image/png".to_string(),
                                        data: image.source.to_string(),
                                    },
                                }),
                            ]
                        }
                    }
                }
            })
            .collect()
    }

    let system_instructions = if request
        .messages
        .first()
        .is_some_and(|msg| matches!(msg.role, Role::System))
    {
        let message = request.messages.remove(0);
        Some(SystemInstruction {
            parts: map_content(message.content),
        })
    } else {
        None
    };

    google_ai::GenerateContentRequest {
        model: google_ai::ModelName { model_id },
        system_instruction: system_instructions,
        contents: request
            .messages
            .into_iter()
            .filter_map(|message| {
                let parts = map_content(message.content);
                if parts.is_empty() {
                    None
                } else {
                    Some(google_ai::Content {
                        parts,
                        role: match message.role {
                            Role::User => google_ai::Role::User,
                            Role::Assistant => google_ai::Role::Model,
                            Role::System => google_ai::Role::User, // Google AI doesn't have a system role
                        },
                    })
                }
            })
            .collect(),
        generation_config: Some(google_ai::GenerationConfig {
            candidate_count: Some(1),
            stop_sequences: Some(request.stop),
            max_output_tokens: None,
            temperature: request.temperature.map(|t| t as f64).or(Some(1.0)),
            thinking_config: match (request.thinking_allowed, mode) {
                (true, GoogleModelMode::Thinking { budget_tokens }) => {
                    budget_tokens.map(|thinking_budget| ThinkingConfig { thinking_budget })
                }
                _ => None,
            },
            top_p: None,
            top_k: None,
        }),
        safety_settings: None,
        tools: (!request.tools.is_empty()).then(|| {
            vec![google_ai::Tool {
                function_declarations: request
                    .tools
                    .into_iter()
                    .map(|tool| FunctionDeclaration {
                        name: tool.name,
                        description: tool.description,
                        parameters: tool.input_schema,
                    })
                    .collect(),
            }]
        }),
        tool_config: request.tool_choice.map(|choice| google_ai::ToolConfig {
            function_calling_config: google_ai::FunctionCallingConfig {
                mode: match choice {
                    LanguageModelToolChoice::Auto => google_ai::FunctionCallingMode::Auto,
                    LanguageModelToolChoice::Any => google_ai::FunctionCallingMode::Any,
                    LanguageModelToolChoice::None => google_ai::FunctionCallingMode::None,
                },
                allowed_function_names: None,
            },
        }),
    }
}

pub struct GoogleEventMapper {
    usage: UsageMetadata,
    stop_reason: StopReason,
}

impl GoogleEventMapper {
    pub fn new() -> Self {
        Self {
            usage: UsageMetadata::default(),
            stop_reason: StopReason::EndTurn,
        }
    }

    pub fn map_stream(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<GenerateContentResponse>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    {
        events
            .map(Some)
            .chain(futures::stream::once(async { None }))
            .flat_map(move |event| {
                futures::stream::iter(match event {
                    Some(Ok(event)) => self.map_event(event),
                    Some(Err(error)) => {
                        vec![Err(LanguageModelCompletionError::from(error))]
                    }
                    None => vec![Ok(LanguageModelCompletionEvent::Stop(self.stop_reason))],
                })
            })
    }

    pub fn map_event(
        &mut self,
        event: GenerateContentResponse,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        static TOOL_CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

        let mut events: Vec<_> = Vec::new();
        let mut wants_to_use_tool = false;
        if let Some(usage_metadata) = event.usage_metadata {
            update_usage(&mut self.usage, &usage_metadata);
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(
                convert_usage(&self.usage),
            )))
        }

        if let Some(prompt_feedback) = event.prompt_feedback
            && let Some(block_reason) = prompt_feedback.block_reason.as_deref()
        {
            self.stop_reason = match block_reason {
                "SAFETY" | "OTHER" | "BLOCKLIST" | "PROHIBITED_CONTENT" | "IMAGE_SAFETY" => {
                    StopReason::Refusal
                }
                _ => {
                    log::error!("Unexpected Google block_reason: {block_reason}");
                    StopReason::Refusal
                }
            };
            events.push(Ok(LanguageModelCompletionEvent::Stop(self.stop_reason)));

            return events;
        }

        if let Some(candidates) = event.candidates {
            for candidate in candidates {
                if let Some(finish_reason) = candidate.finish_reason.as_deref() {
                    self.stop_reason = match finish_reason {
                        "STOP" => StopReason::EndTurn,
                        "MAX_TOKENS" => StopReason::MaxTokens,
                        _ => {
                            log::error!("Unexpected google finish_reason: {finish_reason}");
                            StopReason::EndTurn
                        }
                    };
                }
                candidate
                    .content
                    .parts
                    .into_iter()
                    .for_each(|part| match part {
                        Part::TextPart(text_part) => {
                            events.push(Ok(LanguageModelCompletionEvent::Text(text_part.text)))
                        }
                        Part::InlineDataPart(_) => {}
                        Part::FunctionCallPart(function_call_part) => {
                            wants_to_use_tool = true;
                            let name: Arc<str> = function_call_part.function_call.name.into();
                            let next_tool_id =
                                TOOL_CALL_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
                            let id: LanguageModelToolUseId =
                                format!("{}-{}", name, next_tool_id).into();

                            // Normalize empty string signatures to None
                            let thought_signature = function_call_part
                                .thought_signature
                                .filter(|s| !s.is_empty());

                            events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id,
                                    name,
                                    is_input_complete: true,
                                    raw_input: function_call_part.function_call.args.to_string(),
                                    input: function_call_part.function_call.args,
                                    thought_signature,
                                },
                            )));
                        }
                        Part::FunctionResponsePart(_) => {}
                        Part::ThoughtPart(part) => {
                            events.push(Ok(LanguageModelCompletionEvent::Thinking {
                                text: "(Encrypted thought)".to_string(), // TODO: Can we populate this from thought summaries?
                                signature: Some(part.thought_signature),
                            }));
                        }
                    });
            }
        }

        // Even when Gemini wants to use a Tool, the API
        // responds with `finish_reason: STOP`
        if wants_to_use_tool {
            self.stop_reason = StopReason::ToolUse;
            events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
        }
        events
    }
}

pub fn count_google_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<u64>> {
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
        tiktoken_rs::num_tokens_from_messages("gpt-4", &messages).map(|tokens| tokens as u64)
    })
    .boxed()
}

fn update_usage(usage: &mut UsageMetadata, new: &UsageMetadata) {
    if let Some(prompt_token_count) = new.prompt_token_count {
        usage.prompt_token_count = Some(prompt_token_count);
    }
    if let Some(cached_content_token_count) = new.cached_content_token_count {
        usage.cached_content_token_count = Some(cached_content_token_count);
    }
    if let Some(candidates_token_count) = new.candidates_token_count {
        usage.candidates_token_count = Some(candidates_token_count);
    }
    if let Some(tool_use_prompt_token_count) = new.tool_use_prompt_token_count {
        usage.tool_use_prompt_token_count = Some(tool_use_prompt_token_count);
    }
    if let Some(thoughts_token_count) = new.thoughts_token_count {
        usage.thoughts_token_count = Some(thoughts_token_count);
    }
    if let Some(total_token_count) = new.total_token_count {
        usage.total_token_count = Some(total_token_count);
    }
}

fn convert_usage(usage: &UsageMetadata) -> language_model::TokenUsage {
    let prompt_tokens = usage.prompt_token_count.unwrap_or(0);
    let cached_tokens = usage.cached_content_token_count.unwrap_or(0);
    let input_tokens = prompt_tokens - cached_tokens;
    let output_tokens = usage.candidates_token_count.unwrap_or(0);

    language_model::TokenUsage {
        input_tokens,
        output_tokens,
        cache_read_input_tokens: cached_tokens,
        cache_creation_input_tokens: 0,
    }
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    target_agent: language_model::ConfigurationViewTargetAgent,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(
        state: Entity<State>,
        target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
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
            api_key_editor: cx.new(|cx| InputField::new(window, cx, "AIzaSy...")),
            target_agent,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))?
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
                .update(cx, |state, cx| state.set_api_key(None, cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let env_var_set = self.state.read(cx).api_key_state.is_from_env_var();
        let configured_card_label = if env_var_set {
            format!(
                "API key set in {} environment variable",
                API_KEY_ENV_VAR.name
            )
        } else {
            let api_url = GoogleLanguageModelProvider::api_url(cx);
            if api_url == google_ai::API_URL {
                "API key configured".to_string()
            } else {
                format!("API key configured for {}", api_url)
            }
        };

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Loading credentials..."))
                .into_any_element()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(format!("To use {}, you need to add an API key. Follow these steps:", match &self.target_agent {
                    ConfigurationViewTargetAgent::ZedAgent => "Zed's agent with Google AI".into(),
                    ConfigurationViewTargetAgent::Other(agent) => agent.clone(),
                })))
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
                .child(self.api_key_editor.clone())
                .child(
                    Label::new(
                        format!("You can also assign the {GEMINI_API_KEY_VAR_NAME} environment variable and restart Zed."),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any_element()
        } else {
            ConfiguredApiCard::new(configured_card_label)
                .disabled(env_var_set)
                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx)))
                .when(env_var_set, |this| {
                    this.tooltip_label(format!("To reset your API key, make sure {GEMINI_API_KEY_VAR_NAME} and {GOOGLE_AI_API_KEY_VAR_NAME} environment variables are unset."))
                })
                .into_any_element()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use google_ai::{
        Content, FunctionCall, FunctionCallPart, GenerateContentCandidate, GenerateContentResponse,
        Part, Role as GoogleRole, TextPart,
    };
    use language_model::{LanguageModelToolUseId, MessageContent, Role};
    use serde_json::json;

    #[test]
    fn test_function_call_with_signature_creates_tool_use_with_signature() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                        },
                        thought_signature: Some("test_signature_123".to_string()),
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        assert_eq!(events.len(), 2); // ToolUse event + Stop event

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(tool_use.name.as_ref(), "test_function");
            assert_eq!(
                tool_use.thought_signature.as_deref(),
                Some("test_signature_123")
            );
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_function_call_without_signature_has_none() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                        },
                        thought_signature: None,
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(tool_use.thought_signature, None);
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_empty_string_signature_normalized_to_none() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                        },
                        thought_signature: Some("".to_string()),
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(tool_use.thought_signature, None);
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_parallel_function_calls_preserve_signatures() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "function_1".to_string(),
                                args: json!({"arg": "value1"}),
                            },
                            thought_signature: Some("signature_1".to_string()),
                        }),
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "function_2".to_string(),
                                args: json!({"arg": "value2"}),
                            },
                            thought_signature: None,
                        }),
                    ],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        assert_eq!(events.len(), 3); // 2 ToolUse events + Stop event

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(tool_use.name.as_ref(), "function_1");
            assert_eq!(tool_use.thought_signature.as_deref(), Some("signature_1"));
        } else {
            panic!("Expected ToolUse event for function_1");
        }

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[1] {
            assert_eq!(tool_use.name.as_ref(), "function_2");
            assert_eq!(tool_use.thought_signature, None);
        } else {
            panic!("Expected ToolUse event for function_2");
        }
    }

    #[test]
    fn test_tool_use_with_signature_converts_to_function_call_part() {
        let tool_use = language_model::LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test_id"),
            name: "test_function".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: Some("test_signature_456".to_string()),
        };

        let request = super::into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        assert_eq!(request.contents[0].parts.len(), 1);
        if let Part::FunctionCallPart(fc_part) = &request.contents[0].parts[0] {
            assert_eq!(fc_part.function_call.name, "test_function");
            assert_eq!(
                fc_part.thought_signature.as_deref(),
                Some("test_signature_456")
            );
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_tool_use_without_signature_omits_field() {
        let tool_use = language_model::LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test_id"),
            name: "test_function".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: None,
        };

        let request = super::into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        assert_eq!(request.contents[0].parts.len(), 1);
        if let Part::FunctionCallPart(fc_part) = &request.contents[0].parts[0] {
            assert_eq!(fc_part.thought_signature, None);
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_empty_signature_in_tool_use_normalized_to_none() {
        let tool_use = language_model::LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test_id"),
            name: "test_function".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: Some("".to_string()),
        };

        let request = super::into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        if let Part::FunctionCallPart(fc_part) = &request.contents[0].parts[0] {
            assert_eq!(fc_part.thought_signature, None);
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_round_trip_preserves_signature() {
        let mut mapper = GoogleEventMapper::new();

        // Simulate receiving a response from Google with a signature
        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                        },
                        thought_signature: Some("round_trip_sig".to_string()),
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        let tool_use = if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            tool_use.clone()
        } else {
            panic!("Expected ToolUse event");
        };

        // Convert back to Google format
        let request = super::into_google(
            LanguageModelRequest {
                messages: vec![language_model::LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content: vec![MessageContent::ToolUse(tool_use)],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            },
            "gemini-2.5-flash".to_string(),
            GoogleModelMode::Default,
        );

        // Verify signature is preserved
        if let Part::FunctionCallPart(fc_part) = &request.contents[0].parts[0] {
            assert_eq!(fc_part.thought_signature.as_deref(), Some("round_trip_sig"));
        } else {
            panic!("Expected FunctionCallPart");
        }
    }

    #[test]
    fn test_mixed_text_and_function_call_with_signature() {
        let mut mapper = GoogleEventMapper::new();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![
                        Part::TextPart(TextPart {
                            text: "I'll help with that.".to_string(),
                        }),
                        Part::FunctionCallPart(FunctionCallPart {
                            function_call: FunctionCall {
                                name: "helper_function".to_string(),
                                args: json!({"query": "help"}),
                            },
                            thought_signature: Some("mixed_sig".to_string()),
                        }),
                    ],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        assert_eq!(events.len(), 3); // Text event + ToolUse event + Stop event

        if let Ok(LanguageModelCompletionEvent::Text(text)) = &events[0] {
            assert_eq!(text, "I'll help with that.");
        } else {
            panic!("Expected Text event");
        }

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[1] {
            assert_eq!(tool_use.name.as_ref(), "helper_function");
            assert_eq!(tool_use.thought_signature.as_deref(), Some("mixed_sig"));
        } else {
            panic!("Expected ToolUse event");
        }
    }

    #[test]
    fn test_special_characters_in_signature_preserved() {
        let mut mapper = GoogleEventMapper::new();

        let signature_with_special_chars = "sig<>\"'&%$#@!{}[]".to_string();

        let response = GenerateContentResponse {
            candidates: Some(vec![GenerateContentCandidate {
                index: Some(0),
                content: Content {
                    parts: vec![Part::FunctionCallPart(FunctionCallPart {
                        function_call: FunctionCall {
                            name: "test_function".to_string(),
                            args: json!({"arg": "value"}),
                        },
                        thought_signature: Some(signature_with_special_chars.clone()),
                    })],
                    role: GoogleRole::Model,
                },
                finish_reason: None,
                finish_message: None,
                safety_ratings: None,
                citation_metadata: None,
            }]),
            prompt_feedback: None,
            usage_metadata: None,
        };

        let events = mapper.map_event(response);

        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) = &events[0] {
            assert_eq!(
                tool_use.thought_signature.as_deref(),
                Some(signature_with_special_chars.as_str())
            );
        } else {
            panic!("Expected ToolUse event");
        }
    }
}
