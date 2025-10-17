use anyhow::{Context as _, Result, anyhow};
use collections::BTreeMap;
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, Stream, StreamExt, future::BoxFuture};
use google_vertex_ai::{
    FunctionDeclaration, GenerateContentResponse, Part, SystemInstruction, ThinkingConfig,
    UsageMetadata,
};
use gpui::{AnyView, App, AsyncApp, Context, Subscription, Task};
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
pub use settings::GoogleVertexAvailableModel as AvailableModel;
use settings::{ModelMode, Settings, SettingsStore};
use std::pin::Pin;
use std::sync::{
    Arc,
    atomic::{self, AtomicU64},
};
use strum::IntoEnumIterator;
use ui::{Icon, IconName, List, Tooltip, prelude::*};
use util::ResultExt;

use crate::AllLanguageModelSettings;
use crate::ui::InstructionListItem;

const PROVIDER_ID: &str = "google-vertex-ai";
const PROVIDER_NAME: &str = "Google Vertex AI";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct GoogleVertexSettings {
    pub api_url: String,
    pub project_id: String,  // ADDED
    pub location_id: String, // ADDED
    pub available_models: Vec<AvailableModel>,
}

pub struct GoogleVertexLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: gpui::Entity<State>,
}

pub struct State {
    api_key: Option<String>,
    api_key_from_env: bool,
    _subscription: Subscription,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    fn reset_api_key(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        // Ensure api_url, project_id, and location_id are available for credentials deletion
        let settings = AllLanguageModelSettings::get_global(cx)
            .google_vertex
            .clone();

        cx.spawn(async move |this, cx| {
            credentials_provider
                .delete_credentials(&settings.api_url, &cx) // Use api_url
                .await
                .log_err();
            this.update(cx, |this, cx| {
                this.api_key = None;
                this.api_key_from_env = false;
                cx.notify();
            })
        })
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        log::info!("Authenticating Google Vertex AI...");

        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        // The Tokio runtime provided by `gpui::spawn` is not sufficient for `tokio::process`
        // or `tokio::task::spawn_blocking`. We must fall back to the standard library's threading
        // to run the synchronous `gcloud` command, and use a channel to communicate the
        // result back to our async context.
        cx.spawn(async move |this, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();

            std::thread::spawn(move || {
                let result = std::process::Command::new("gcloud")
                    .args(&["auth", "application-default", "print-access-token"])
                    .output()
                    .map_err(|e| {
                        AuthenticateError::Other(anyhow!("Failed to execute gcloud command: {}", e))
                    });

                // Send the result back to the async task, ignoring if the receiver was dropped.
                let _ = tx.send(result);
            });

            // Await the result from the channel.
            // First, explicitly handle the channel's `Canceled` error.
            // Then, use `?` to propagate the `AuthenticateError` from the command execution.
            let token_output = rx.await.map_err(|_cancelled| {
                AuthenticateError::Other(anyhow!("Authentication task was cancelled"))
            })??;

            // Retrieve the access token from the gcloud command output.
            // Ensure UTF-8 decoding and trim whitespace.
            let access_token = String::from_utf8(token_output.stdout)
                .map_err(|e| {
                    AuthenticateError::Other(anyhow!("Invalid UTF-8 in gcloud output: {}", e))
                })?
                .trim()
                .to_string();

            // Check the exit status of the gcloud command.
            if !token_output.status.success() {
                let stderr = String::from_utf8_lossy(&token_output.stderr).into_owned();
                return Err(AuthenticateError::Other(anyhow!(
                    "gcloud command failed: {}",
                    stderr
                )));
            }

            let api_key = access_token; // Use the retrieved token as the API key.
            let from_env = false; // This token is dynamically fetched, not from env or keychain.

            this.update(cx, |this, cx| {
                this.api_key = Some(api_key);
                this.api_key_from_env = from_env;
                cx.notify();
            })?;

            Ok(())
        })
    }
}

impl GoogleVertexLanguageModelProvider {
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

    fn create_language_model(&self, model: google_vertex_ai::Model) -> Arc<dyn LanguageModel> {
        Arc::new(GoogleVertexLanguageModel {
            id: LanguageModelId::from(model.id().to_string()),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for GoogleVertexLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for GoogleVertexLanguageModelProvider {
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
        Some(self.create_language_model(google_vertex_ai::Model::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(google_vertex_ai::Model::default_fast()))
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        // Add base models from google_vertex_ai::Model::iter()
        for model in google_vertex_ai::Model::iter() {
            if !matches!(model, google_vertex_ai::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .google_vertex
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                google_vertex_ai::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    mode: model.mode.clone().unwrap(),
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(GoogleVertexLanguageModel {
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
        _configuration_view_target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.reset_api_key(cx))
    }
}

pub struct GoogleVertexLanguageModel {
    id: LanguageModelId,
    model: google_vertex_ai::Model,
    state: gpui::Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl GoogleVertexLanguageModel {
    fn stream_completion(
        &self,
        request: google_vertex_ai::GenerateContentRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<futures::stream::BoxStream<'static, Result<GenerateContentResponse>>>,
    > {
        let http_client = self.http_client.clone();

        let Ok((access_token_option, api_url, project_id, location_id)) =
            cx.read_entity(&self.state, |state, cx| {
                let settings = &AllLanguageModelSettings::get_global(cx).google_vertex;
                (
                    state.api_key.clone(), // This is the access token for Vertex AI
                    settings.api_url.clone(),
                    settings.project_id.clone(),  // ADDED
                    settings.location_id.clone(), // ADDED
                )
            })
        else {
            return futures::future::ready(Err(anyhow!("App state dropped"))).boxed();
        };

        async move {
            let access_token =
                access_token_option.context("Missing Google API key (access token)")?;

            let request = google_vertex_ai::stream_generate_content(
                http_client.as_ref(),
                &api_url,
                &project_id,  // ADDED
                &location_id, // ADDED
                &access_token,
                request,
            );
            request.await.context("failed to stream completion")
        }
        .boxed()
    }
}

impl LanguageModel for GoogleVertexLanguageModel {
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
        format!("google_vertex/{}", self.model.request_id())
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
        let request = into_vertex_ai(request, model_id.clone(), self.model.mode());
        let http_client = self.http_client.clone();

        // Synchronously read the state and settings.
        // `read_entity` executes the closure and returns its result directly.
        let (access_token_option, api_url, project_id, location_id) =
            cx.read_entity(&self.state, |state, cx| {
                let settings = &AllLanguageModelSettings::get_global(cx).google_vertex;
                (
                    state.api_key.clone(), // This is the access token for Vertex AI (Option<String>)
                    settings.api_url.clone(), // String
                    settings.project_id.clone(), // String
                    settings.location_id.clone(), // String
                )
            }); // No .unwrap_or_default() here, as read_entity directly returns the tuple

        async move {
            // Check if the access token is present. If not, return an error.
            let access_token = access_token_option
                .context("Missing Google API key (access token). Please authenticate.")?;

            let response = google_vertex_ai::count_tokens(
                http_client.as_ref(),
                &api_url,
                &project_id,
                &location_id,
                &access_token,
                google_vertex_ai::CountTokensRequest {
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
        let request = into_vertex_ai(
            request,
            self.model.request_id().to_string(),
            self.model.mode(),
        );
        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request
                .await
                .map_err(|err| LanguageModelCompletionError::Other(anyhow!(err)))?;
            Ok(GoogleVertexEventMapper::new().map_stream(response))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

pub fn into_vertex_ai(
    mut request: LanguageModelRequest,
    model_id: String,
    mode: ModelMode,
) -> google_vertex_ai::GenerateContentRequest {
    fn map_content(content: Vec<MessageContent>) -> Vec<Part> {
        content
            .into_iter()
            .flat_map(|content| match content {
                language_model::MessageContent::Text(text) => {
                    if !text.is_empty() {
                        vec![Part::TextPart(google_vertex_ai::TextPart { text })]
                    } else {
                        vec![]
                    }
                }
                language_model::MessageContent::Thinking {
                    text: _,
                    signature: Some(signature),
                } => {
                    if !signature.is_empty() {
                        vec![Part::ThoughtPart(google_vertex_ai::ThoughtPart {
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
                    vec![Part::InlineDataPart(google_vertex_ai::InlineDataPart {
                        inline_data: google_vertex_ai::GenerativeContentBlob {
                            mime_type: "image/png".to_string(), // Assuming PNG for simplicity, could derive from format
                            data: image.source.to_string(), // Assuming base64 encoded for simplicity
                        },
                    })]
                }
                language_model::MessageContent::ToolUse(tool_use) => {
                    vec![Part::FunctionCallPart(google_vertex_ai::FunctionCallPart {
                        function_call: google_vertex_ai::FunctionCall {
                            name: tool_use.name.to_string(),
                            args: tool_use.input,
                        },
                    })]
                }
                language_model::MessageContent::ToolResult(tool_result) => {
                    match tool_result.content {
                        language_model::LanguageModelToolResultContent::Text(text) => {
                            vec![Part::FunctionResponsePart(
                                google_vertex_ai::FunctionResponsePart {
                                    function_response: google_vertex_ai::FunctionResponse {
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
                                Part::FunctionResponsePart(
                                    google_vertex_ai::FunctionResponsePart {
                                        function_response: google_vertex_ai::FunctionResponse {
                                            name: tool_result.tool_name.to_string(),
                                            // The API expects a valid JSON object
                                            response: serde_json::json!({
                                                "output": "Tool responded with an image"
                                            }),
                                        },
                                    },
                                ),
                                Part::InlineDataPart(google_vertex_ai::InlineDataPart {
                                    inline_data: google_vertex_ai::GenerativeContentBlob {
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
        .map_or(false, |msg| matches!(msg.role, Role::System))
    {
        let message = request.messages.remove(0);
        Some(SystemInstruction {
            parts: map_content(message.content),
        })
    } else {
        None
    };

    google_vertex_ai::GenerateContentRequest {
        model: google_vertex_ai::ModelName { model_id },
        system_instruction: system_instructions,
        contents: request
            .messages
            .into_iter()
            .filter_map(|message| {
                let parts = map_content(message.content);
                if parts.is_empty() {
                    None
                } else {
                    Some(google_vertex_ai::Content {
                        parts,
                        role: match message.role {
                            Role::User => google_vertex_ai::Role::User,
                            Role::Assistant => google_vertex_ai::Role::Model,
                            Role::System => google_vertex_ai::Role::User, // Google AI doesn't have a distinct system role; often maps to user for initial context
                        },
                    })
                }
            })
            .collect(),
        generation_config: Some(google_vertex_ai::GenerationConfig {
            candidate_count: Some(1),
            stop_sequences: Some(request.stop),
            max_output_tokens: None,
            temperature: request.temperature.map(|t| t as f64).or(Some(1.0)),
            thinking_config: match mode {
                ModelMode::Thinking { budget_tokens } => {
                    budget_tokens.map(|thinking_budget| ThinkingConfig { thinking_budget })
                }
                ModelMode::Default => None,
            },
            top_p: None,
            top_k: None,
        }),
        safety_settings: None, // Safety settings are handled at a different layer or can be configured.
        tools: (request.tools.len() > 0).then(|| {
            vec![google_vertex_ai::Tool {
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
        tool_config: request
            .tool_choice
            .map(|choice| google_vertex_ai::ToolConfig {
                function_calling_config: google_vertex_ai::FunctionCallingConfig {
                    mode: match choice {
                        LanguageModelToolChoice::Auto => {
                            google_vertex_ai::FunctionCallingMode::Auto
                        }
                        LanguageModelToolChoice::Any => google_vertex_ai::FunctionCallingMode::Any,
                        LanguageModelToolChoice::None => {
                            google_vertex_ai::FunctionCallingMode::None
                        }
                    },
                    allowed_function_names: None,
                },
            }),
    }
}

pub struct GoogleVertexEventMapper {
    usage: UsageMetadata,
    stop_reason: StopReason,
}

impl GoogleVertexEventMapper {
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
                        vec![Err(LanguageModelCompletionError::Other(anyhow!(error)))]
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
        if let Some(candidates) = event.candidates {
            for candidate in candidates {
                if let Some(finish_reason) = candidate.finish_reason.as_deref() {
                    self.stop_reason = match finish_reason {
                        "STOP" => StopReason::EndTurn,
                        "MAX_TOKENS" => StopReason::MaxTokens,
                        _ => {
                            log::error!("Unexpected google_vertex finish_reason: {finish_reason}");
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

                            events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id,
                                    name,
                                    is_input_complete: true,
                                    raw_input: function_call_part.function_call.args.to_string(),
                                    input: function_call_part.function_call.args,
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
            state,
            load_credentials_task,
        }
    }

    fn authenticate_gcloud(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Authenticating with gcloud...");

        let state = self.state.clone();
        self.load_credentials_task = Some(cx.spawn_in(window, {
            async move |this, cx| {
                if let Some(task) = state
                    .update(cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));
        cx.notify();
    }

    fn reset_gcloud_auth(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state.update(cx, |state, cx| state.reset_api_key(cx))?.await
        })
        .detach_and_log_err(cx);

        cx.notify();
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_authenticated = self.state.read(cx).is_authenticated();

        if self.load_credentials_task.is_some() {
            div()
                .child(Label::new("Attempting to authenticate with gcloud..."))
                .into_any()
        } else if !is_authenticated {
            v_flex()
                .size_full()
                .child(Label::new("Please authenticate with Google Cloud to use this provider."))
                .child(
                    List::new()
                        .child(InstructionListItem::text_only(
                            "1. Ensure Google Cloud SDK is installed and configured.",
                        ))
                        .child(InstructionListItem::text_only(
                            "2. Run 'gcloud auth application-default login' in your terminal.",
                        ))
                        .child(InstructionListItem::text_only(
                            "3. Configure your desired Google Cloud Project ID and Location ID in Zed's settings.json file under 'language_models.google_vertex'.",
                        ))
                )
                .child(
                    h_flex()
                        .w_full()
                        .my_2()
                        .child(
                            Button::new("authenticate-gcloud", "Authenticate with gcloud")
                                .label_size(LabelSize::Small)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(|this, _, window, cx| this.authenticate_gcloud(window, cx))),
                        ),
                )
                .child(
                    Label::new(
                        "This will attempt to acquire an access token using your
                        gcloud application-default credentials. You might need to run
                        'gcloud auth application-default login' manually first."
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                // .justify_between() // Removed, button is handled separately
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new("Authenticated with gcloud.")),
                )
                  .child(
                    Button::new("reset-gcloud-auth", "Clear Token")
                        .label_size(LabelSize::Small)
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .tooltip(Tooltip::text("Clear the in-memory access token. You will need to re-authenticate to use the provider."))
                        .on_click(cx.listener(|this, _, window, cx| this.reset_gcloud_auth(window, cx))),
                )
                .into_any()
        }
    }
}
