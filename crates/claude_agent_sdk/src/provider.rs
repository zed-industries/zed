//! Provider module - LanguageModel and LanguageModelProvider implementation
//!
//! This module provides the Claude Agent SDK as a language model provider for Zed,
//! implementing the required traits to integrate with Zed's AI system.

use anthropic::{AnthropicModelMode, Event, ANTHROPIC_API_URL};
use anyhow::{Result, anyhow};
use futures::{FutureExt, StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::HttpClient;
use language_model::{
    ApiKeyState, AuthenticateError, ConfigurationViewTargetAgent, EnvVar, IconOrSvg,
    LanguageModel, LanguageModelCacheConfiguration, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolUse, RateLimiter,
    StopReason, TokenUsage, env_var,
};
use parking_lot::Mutex;
use settings::{Settings, SettingsStore};
use std::sync::{Arc, LazyLock};

use crate::{Agent, AgentConfig, CLAUDE_AGENT_PROVIDER_ID, CLAUDE_AGENT_PROVIDER_NAME};

const API_KEY_ENV_VAR_NAME: &str = "ANTHROPIC_API_KEY";
static API_KEY_ENV_VAR: LazyLock<EnvVar> = env_var!(API_KEY_ENV_VAR_NAME);

/// Settings for the Claude Agent provider
#[derive(Default, Clone, Debug, PartialEq)]
pub struct ClaudeAgentSettings {
    /// API URL (defaults to Anthropic's API)
    pub api_url: String,
    /// Agent configuration
    pub agent_config: AgentConfig,
}

/// State for the Claude Agent provider
pub struct ClaudeAgentState {
    api_key_state: ApiKeyState,
}

impl ClaudeAgentState {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let api_url = ClaudeAgentLanguageModelProvider::api_url(cx);
        self.api_key_state
            .store(api_url, api_key, |this| &mut this.api_key_state, cx)
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = ClaudeAgentLanguageModelProvider::api_url(cx);
        self.api_key_state
            .load_if_needed(api_url, |this| &mut this.api_key_state, cx)
    }
}

/// The Claude Agent Language Model Provider
pub struct ClaudeAgentLanguageModelProvider {
    http_client: Arc<dyn HttpClient>,
    state: Entity<ClaudeAgentState>,
}

impl ClaudeAgentLanguageModelProvider {
    /// Create a new Claude Agent provider
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut ClaudeAgentState, cx| {
                let api_url = Self::api_url(cx);
                this.api_key_state
                    .handle_url_change(api_url, |this| &mut this.api_key_state, cx);
                cx.notify();
            })
            .detach();
            ClaudeAgentState {
                api_key_state: ApiKeyState::new(Self::api_url(cx), (*API_KEY_ENV_VAR).clone()),
            }
        });

        Self { http_client, state }
    }

    fn create_language_model(&self, model: anthropic::Model) -> Arc<dyn LanguageModel> {
        Arc::new(ClaudeAgentModel {
            id: LanguageModelId::from(format!("agent-{}", model.id())),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
            agent: Arc::new(Mutex::new(Agent::new())),
        })
    }

    fn api_url(cx: &App) -> SharedString {
        // TODO: Add settings integration
        SharedString::new(ANTHROPIC_API_URL)
    }
}

impl LanguageModelProviderState for ClaudeAgentLanguageModelProvider {
    type ObservableEntity = ClaudeAgentState;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for ClaudeAgentLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        CLAUDE_AGENT_PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        CLAUDE_AGENT_PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(icons::IconName::AiAnthropic)
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::ClaudeSonnet4_5))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(self.create_language_model(anthropic::Model::ClaudeHaiku3_5))
    }

    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        vec![
            self.create_language_model(anthropic::Model::ClaudeOpus4_5),
            self.create_language_model(anthropic::Model::ClaudeSonnet4_5),
            self.create_language_model(anthropic::Model::ClaudeSonnet4_5Thinking),
        ]
    }

    fn provided_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        vec![
            self.create_language_model(anthropic::Model::ClaudeOpus4_5),
            self.create_language_model(anthropic::Model::ClaudeSonnet4_5),
            self.create_language_model(anthropic::Model::ClaudeSonnet4_5Thinking),
            self.create_language_model(anthropic::Model::ClaudeHaiku3_5),
        ]
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: ConfigurationViewTargetAgent,
        _window: &mut Window,
        _cx: &mut App,
    ) -> AnyView {
        // TODO: Implement configuration UI
        unimplemented!("Configuration view not yet implemented")
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

/// The Claude Agent Language Model
pub struct ClaudeAgentModel {
    id: LanguageModelId,
    model: anthropic::Model,
    state: Entity<ClaudeAgentState>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
    agent: Arc<Mutex<Agent>>,
}

impl LanguageModel for ClaudeAgentModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(format!("Claude Agent ({})", self.model.display_name()))
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        CLAUDE_AGENT_PROVIDER_ID
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        CLAUDE_AGENT_PROVIDER_NAME
    }

    fn telemetry_id(&self) -> String {
        format!("claude-agent-{}", self.model.id())
    }

    fn api_key(&self, cx: &App) -> Option<String> {
        let api_url = ClaudeAgentLanguageModelProvider::api_url(cx);
        self.state.read(cx).api_key_state.key(&api_url)
    }

    fn supports_images(&self) -> bool {
        true // Claude supports images
    }

    fn supports_tools(&self) -> bool {
        true // Agent SDK fully supports tools
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => true,
            LanguageModelToolChoice::Any => true,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_streaming_tools(&self) -> bool {
        true
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u64> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        // Use tiktoken for token estimation
        async move {
            // Simple estimation - in production, use the actual Anthropic token counting API
            let total_text: String = request
                .messages
                .iter()
                .map(|m| m.string_contents())
                .collect::<Vec<_>>()
                .join(" ");

            // Rough estimation: ~4 characters per token
            Ok((total_text.len() / 4) as u64)
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
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();
        let model = self.model.clone();
        let state = self.state.clone();

        let api_key_result = state.read_with(cx, |state, cx| {
            let api_url = ClaudeAgentLanguageModelProvider::api_url(cx);
            (state.api_key_state.key(&api_url), api_url)
        });

        async move {
            let (api_key, api_url) = api_key_result
                .map_err(|_| anyhow!("App state dropped"))?;

            let api_key = api_key.ok_or(LanguageModelCompletionError::NoApiKey {
                provider: CLAUDE_AGENT_PROVIDER_NAME,
            })?;

            // Convert request to Anthropic format
            let anthropic_request = convert_request_to_anthropic(&request, &model);

            // Stream completion from Anthropic
            let stream = anthropic::stream_completion(
                http_client.as_ref(),
                &api_url,
                &api_key,
                anthropic_request,
                model.beta_headers(),
            )
            .await
            .map_err(LanguageModelCompletionError::from)?;

            // Convert Anthropic events to LanguageModelCompletionEvents
            let event_stream = stream.map(|event| {
                event
                    .map(convert_anthropic_event)
                    .map_err(LanguageModelCompletionError::from)
            });

            Ok(event_stream.boxed())
        }
        .boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        self.model.cache_configuration().map(|config| {
            LanguageModelCacheConfiguration {
                max_cache_anchors: config.max_cache_anchors,
                should_speculate: config.should_speculate,
                min_total_token: config.min_total_token,
            }
        })
    }
}

/// Convert a LanguageModelRequest to an Anthropic Request
fn convert_request_to_anthropic(
    request: &LanguageModelRequest,
    model: &anthropic::Model,
) -> anthropic::Request {
    // This is a simplified conversion - the full implementation would handle
    // all message types, tools, etc.
    let mut messages = Vec::new();
    let mut system_message = String::new();

    for message in &request.messages {
        match message.role {
            language_model::Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.string_contents());
            }
            language_model::Role::User => {
                messages.push(anthropic::Message {
                    role: anthropic::Role::User,
                    content: vec![anthropic::RequestContent::Text {
                        text: message.string_contents(),
                        cache_control: None,
                    }],
                });
            }
            language_model::Role::Assistant => {
                messages.push(anthropic::Message {
                    role: anthropic::Role::Assistant,
                    content: vec![anthropic::RequestContent::Text {
                        text: message.string_contents(),
                        cache_control: None,
                    }],
                });
            }
        }
    }

    // Convert tools
    let tools: Vec<anthropic::Tool> = request
        .tools
        .iter()
        .map(|tool| anthropic::Tool {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: tool.input_schema.clone(),
        })
        .collect();

    anthropic::Request {
        model: model.id().to_string(),
        max_tokens: request.max_tokens.unwrap_or(8192) as u32,
        messages,
        system: if system_message.is_empty() {
            None
        } else {
            Some(anthropic::StringOrContents::String(system_message))
        },
        tools,
        tool_choice: request.tool_choice.map(|choice| match choice {
            LanguageModelToolChoice::Auto => anthropic::ToolChoice::Auto,
            LanguageModelToolChoice::Any => anthropic::ToolChoice::Any,
            LanguageModelToolChoice::None => anthropic::ToolChoice::None,
        }),
        thinking: None, // TODO: Support thinking mode
        temperature: request.temperature,
        stream: true,
        metadata: None,
    }
}

/// Convert an Anthropic Event to a LanguageModelCompletionEvent
fn convert_anthropic_event(event: Event) -> LanguageModelCompletionEvent {
    match event {
        Event::MessageStart { message } => {
            LanguageModelCompletionEvent::StartMessage {
                message_id: message.id,
            }
        }
        Event::ContentBlockStart { content_block, .. } => {
            match content_block {
                anthropic::ResponseContent::Text { text } => {
                    LanguageModelCompletionEvent::Text(text)
                }
                anthropic::ResponseContent::ToolUse { id, name, input } => {
                    LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                        id: id.into(),
                        name: name.into(),
                        input,
                        is_input_complete: false,
                    })
                }
                anthropic::ResponseContent::Thinking { thinking, signature } => {
                    LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature,
                    }
                }
                anthropic::ResponseContent::RedactedThinking { data } => {
                    LanguageModelCompletionEvent::RedactedThinking { data }
                }
            }
        }
        Event::ContentBlockDelta { delta, .. } => {
            match delta {
                anthropic::ContentDelta::TextDelta { text } => {
                    LanguageModelCompletionEvent::Text(text)
                }
                anthropic::ContentDelta::InputJsonDelta { partial_json } => {
                    // Tool input is being streamed
                    LanguageModelCompletionEvent::Text(partial_json)
                }
                anthropic::ContentDelta::ThinkingDelta { thinking } => {
                    LanguageModelCompletionEvent::Thinking {
                        text: thinking,
                        signature: None,
                    }
                }
                anthropic::ContentDelta::SignatureDelta { signature } => {
                    LanguageModelCompletionEvent::Thinking {
                        text: String::new(),
                        signature: Some(signature),
                    }
                }
            }
        }
        Event::ContentBlockStop { .. } => {
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn)
        }
        Event::MessageDelta { delta, usage } => {
            if let Some(reason) = delta.stop_reason {
                let stop_reason = match reason.as_str() {
                    "end_turn" => StopReason::EndTurn,
                    "max_tokens" => StopReason::MaxTokens,
                    "tool_use" => StopReason::ToolUse,
                    _ => StopReason::EndTurn,
                };
                LanguageModelCompletionEvent::Stop(stop_reason)
            } else {
                LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 0,
                    output_tokens: usage.output_tokens as u64,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })
            }
        }
        Event::MessageStop => {
            LanguageModelCompletionEvent::Stop(StopReason::EndTurn)
        }
        Event::Error { error } => {
            // Convert error to a text event for now
            LanguageModelCompletionEvent::Text(format!("Error: {}", error.message))
        }
        Event::Ping => {
            // Ignore ping events
            LanguageModelCompletionEvent::Text(String::new())
        }
    }
}
