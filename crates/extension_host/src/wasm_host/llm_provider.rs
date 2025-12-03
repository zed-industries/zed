use crate::wasm_host::WasmExtension;

use crate::wasm_host::wit::{
    LlmCompletionEvent, LlmCompletionRequest, LlmImageData, LlmMessageContent, LlmMessageRole,
    LlmModelInfo, LlmProviderInfo, LlmRequestMessage, LlmStopReason, LlmThinkingContent,
    LlmToolChoice, LlmToolDefinition, LlmToolInputFormat, LlmToolResult, LlmToolResultContent,
    LlmToolUse,
};
use anyhow::{Result, anyhow};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use gpui::{AnyView, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Task, Window};
use language_model::tool_schema::LanguageModelToolSchemaFormat;
use language_model::{
    AuthenticateError, ConfigurationViewTargetAgent, LanguageModel,
    LanguageModelCacheConfiguration, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice, LanguageModelToolUse, LanguageModelToolUseId, StopReason, TokenUsage,
};
use std::sync::Arc;

/// An extension-based language model provider.
pub struct ExtensionLanguageModelProvider {
    pub extension: WasmExtension,
    pub provider_info: LlmProviderInfo,
    state: Entity<ExtensionLlmProviderState>,
}

pub struct ExtensionLlmProviderState {
    is_authenticated: bool,
    available_models: Vec<LlmModelInfo>,
}

impl EventEmitter<()> for ExtensionLlmProviderState {}

impl ExtensionLanguageModelProvider {
    pub fn new(
        extension: WasmExtension,
        provider_info: LlmProviderInfo,
        models: Vec<LlmModelInfo>,
        is_authenticated: bool,
        cx: &mut App,
    ) -> Self {
        let state = cx.new(|_| ExtensionLlmProviderState {
            is_authenticated,
            available_models: models,
        });

        Self {
            extension,
            provider_info,
            state,
        }
    }

    fn provider_id_string(&self) -> String {
        format!("{}:{}", self.extension.manifest.id, self.provider_info.id)
    }
}

impl LanguageModelProvider for ExtensionLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        let id = LanguageModelProviderId::from(self.provider_id_string());
        eprintln!("ExtensionLanguageModelProvider::id() -> {:?}", id);
        id
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName::from(self.provider_info.name.clone())
    }

    fn icon(&self) -> ui::IconName {
        ui::IconName::ZedAssistant
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        state
            .available_models
            .iter()
            .find(|m| m.is_default)
            .or_else(|| state.available_models.first())
            .map(|model_info| {
                Arc::new(ExtensionLanguageModel {
                    extension: self.extension.clone(),
                    model_info: model_info.clone(),
                    provider_id: self.id(),
                    provider_name: self.name(),
                    provider_info: self.provider_info.clone(),
                }) as Arc<dyn LanguageModel>
            })
    }

    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        state
            .available_models
            .iter()
            .find(|m| m.is_default_fast)
            .or_else(|| state.available_models.iter().find(|m| m.is_default))
            .or_else(|| state.available_models.first())
            .map(|model_info| {
                Arc::new(ExtensionLanguageModel {
                    extension: self.extension.clone(),
                    model_info: model_info.clone(),
                    provider_id: self.id(),
                    provider_name: self.name(),
                    provider_info: self.provider_info.clone(),
                }) as Arc<dyn LanguageModel>
            })
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        eprintln!(
            "ExtensionLanguageModelProvider::provided_models called for {}, returning {} models",
            self.provider_info.name,
            state.available_models.len()
        );
        state
            .available_models
            .iter()
            .map(|model_info| {
                eprintln!("  - model: {}", model_info.name);
                Arc::new(ExtensionLanguageModel {
                    extension: self.extension.clone(),
                    model_info: model_info.clone(),
                    provider_id: self.id(),
                    provider_name: self.name(),
                    provider_info: self.provider_info.clone(),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let state = self.state.clone();

        cx.spawn(async move |cx| {
            let result = extension
                .call(|extension, store| {
                    async move {
                        extension
                            .call_llm_provider_authenticate(store, &provider_id)
                            .await
                    }
                    .boxed()
                })
                .await;

            match result {
                Ok(Ok(Ok(()))) => {
                    cx.update(|cx| {
                        state.update(cx, |state, _| {
                            state.is_authenticated = true;
                        });
                    })?;
                    Ok(())
                }
                Ok(Ok(Err(e))) => Err(AuthenticateError::Other(anyhow!("{}", e))),
                Ok(Err(e)) => Err(AuthenticateError::Other(e)),
                Err(e) => Err(AuthenticateError::Other(e)),
            }
        })
    }

    fn configuration_view(
        &self,
        _target_agent: ConfigurationViewTargetAgent,
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|_| EmptyConfigView).into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let state = self.state.clone();

        cx.spawn(async move |cx| {
            let result = extension
                .call(|extension, store| {
                    async move {
                        extension
                            .call_llm_provider_reset_credentials(store, &provider_id)
                            .await
                    }
                    .boxed()
                })
                .await;

            match result {
                Ok(Ok(Ok(()))) => {
                    cx.update(|cx| {
                        state.update(cx, |state, _| {
                            state.is_authenticated = false;
                        });
                    })?;
                    Ok(())
                }
                Ok(Ok(Err(e))) => Err(anyhow!("{}", e)),
                Ok(Err(e)) => Err(e),
                Err(e) => Err(e),
            }
        })
    }
}

impl LanguageModelProviderState for ExtensionLanguageModelProvider {
    type ObservableEntity = ExtensionLlmProviderState;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }

    fn subscribe<T: 'static>(
        &self,
        cx: &mut Context<T>,
        callback: impl Fn(&mut T, &mut Context<T>) + 'static,
    ) -> Option<gpui::Subscription> {
        Some(cx.subscribe(&self.state, move |this, _, _, cx| callback(this, cx)))
    }
}

struct EmptyConfigView;

impl gpui::Render for EmptyConfigView {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::Empty
    }
}

/// An extension-based language model.
pub struct ExtensionLanguageModel {
    extension: WasmExtension,
    model_info: LlmModelInfo,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    provider_info: LlmProviderInfo,
}

impl LanguageModel for ExtensionLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId::from(format!("{}:{}", self.provider_id.0, self.model_info.id))
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model_info.name.clone())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn telemetry_id(&self) -> String {
        format!("extension:{}", self.model_info.id)
    }

    fn supports_images(&self) -> bool {
        self.model_info.capabilities.supports_images
    }

    fn supports_tools(&self) -> bool {
        self.model_info.capabilities.supports_tools
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model_info.capabilities.supports_tool_choice_auto,
            LanguageModelToolChoice::Any => self.model_info.capabilities.supports_tool_choice_any,
            LanguageModelToolChoice::None => self.model_info.capabilities.supports_tool_choice_none,
        }
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        match self.model_info.capabilities.tool_input_format {
            LlmToolInputFormat::JsonSchema => LanguageModelToolSchemaFormat::JsonSchema,
            LlmToolInputFormat::Simplified => LanguageModelToolSchemaFormat::JsonSchema,
        }
    }

    fn max_token_count(&self) -> u64 {
        self.model_info.max_token_count
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model_info.max_output_tokens
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        _cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let model_id = self.model_info.id.clone();

        async move {
            let wit_request = convert_request_to_wit(&request);

            let result = extension
                .call(|ext, store| {
                    async move {
                        ext.call_llm_count_tokens(store, &provider_id, &model_id, &wit_request)
                            .await
                    }
                    .boxed()
                })
                .await?;

            match result {
                Ok(Ok(count)) => Ok(count),
                Ok(Err(e)) => Err(anyhow!("{}", e)),
                Err(e) => Err(e),
            }
        }
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    > {
        let extension = self.extension.clone();
        let provider_id = self.provider_info.id.clone();
        let model_id = self.model_info.id.clone();

        async move {
            let wit_request = convert_request_to_wit(&request);

            // Start the stream and get a stream ID
            let outer_result = extension
                .call(|ext, store| {
                    async move {
                        ext.call_llm_stream_completion_start(
                            store,
                            &provider_id,
                            &model_id,
                            &wit_request,
                        )
                        .await
                    }
                    .boxed()
                })
                .await
                .map_err(|e| LanguageModelCompletionError::Other(e))?;

            // Unwrap the inner Result<Result<String, String>>
            let inner_result =
                outer_result.map_err(|e| LanguageModelCompletionError::Other(anyhow!("{}", e)))?;

            // Get the stream ID
            let stream_id =
                inner_result.map_err(|e| LanguageModelCompletionError::Other(anyhow!("{}", e)))?;

            // Create a stream that polls for events
            let stream = futures::stream::unfold(
                (extension, stream_id, false),
                |(ext, stream_id, done)| async move {
                    if done {
                        return None;
                    }

                    let result = ext
                        .call({
                            let stream_id = stream_id.clone();
                            move |ext, store| {
                                async move {
                                    ext.call_llm_stream_completion_next(store, &stream_id).await
                                }
                                .boxed()
                            }
                        })
                        .await;

                    match result {
                        Ok(Ok(Ok(Some(event)))) => {
                            let converted = convert_completion_event(event);
                            Some((Ok(converted), (ext, stream_id, false)))
                        }
                        Ok(Ok(Ok(None))) => {
                            // Stream complete - close it
                            let _ = ext
                                .call({
                                    let stream_id = stream_id.clone();
                                    move |ext, store| {
                                        async move {
                                            ext.call_llm_stream_completion_close(store, &stream_id)
                                                .await
                                        }
                                        .boxed()
                                    }
                                })
                                .await;
                            None
                        }
                        Ok(Ok(Err(e))) => {
                            // Extension returned an error - close stream and return error
                            let _ = ext
                                .call({
                                    let stream_id = stream_id.clone();
                                    move |ext, store| {
                                        async move {
                                            ext.call_llm_stream_completion_close(store, &stream_id)
                                                .await
                                        }
                                        .boxed()
                                    }
                                })
                                .await;
                            Some((
                                Err(LanguageModelCompletionError::Other(anyhow!("{}", e))),
                                (ext, stream_id, true),
                            ))
                        }
                        Ok(Err(e)) => {
                            // WASM call error - close stream and return error
                            let _ = ext
                                .call({
                                    let stream_id = stream_id.clone();
                                    move |ext, store| {
                                        async move {
                                            ext.call_llm_stream_completion_close(store, &stream_id)
                                                .await
                                        }
                                        .boxed()
                                    }
                                })
                                .await;
                            Some((
                                Err(LanguageModelCompletionError::Other(e)),
                                (ext, stream_id, true),
                            ))
                        }
                        Err(e) => {
                            // Channel error - close stream and return error
                            let _ = ext
                                .call({
                                    let stream_id = stream_id.clone();
                                    move |ext, store| {
                                        async move {
                                            ext.call_llm_stream_completion_close(store, &stream_id)
                                                .await
                                        }
                                        .boxed()
                                    }
                                })
                                .await;
                            Some((
                                Err(LanguageModelCompletionError::Other(e)),
                                (ext, stream_id, true),
                            ))
                        }
                    }
                },
            );

            Ok(stream.boxed())
        }
        .boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        None
    }
}

fn convert_request_to_wit(request: &LanguageModelRequest) -> LlmCompletionRequest {
    let messages = request
        .messages
        .iter()
        .map(|msg| LlmRequestMessage {
            role: match msg.role {
                language_model::Role::User => LlmMessageRole::User,
                language_model::Role::Assistant => LlmMessageRole::Assistant,
                language_model::Role::System => LlmMessageRole::System,
            },
            content: msg
                .content
                .iter()
                .map(|content| match content {
                    language_model::MessageContent::Text(text) => {
                        LlmMessageContent::Text(text.clone())
                    }
                    language_model::MessageContent::Image(image) => {
                        LlmMessageContent::Image(LlmImageData {
                            source: image.source.to_string(),
                            width: Some(image.size.width.0 as u32),
                            height: Some(image.size.height.0 as u32),
                        })
                    }
                    language_model::MessageContent::ToolUse(tool_use) => {
                        LlmMessageContent::ToolUse(LlmToolUse {
                            id: tool_use.id.to_string(),
                            name: tool_use.name.to_string(),
                            input: tool_use.raw_input.clone(),
                            thought_signature: tool_use.thought_signature.clone(),
                        })
                    }
                    language_model::MessageContent::ToolResult(result) => {
                        LlmMessageContent::ToolResult(LlmToolResult {
                            tool_use_id: result.tool_use_id.to_string(),
                            tool_name: result.tool_name.to_string(),
                            is_error: result.is_error,
                            content: match &result.content {
                                language_model::LanguageModelToolResultContent::Text(t) => {
                                    LlmToolResultContent::Text(t.to_string())
                                }
                                language_model::LanguageModelToolResultContent::Image(img) => {
                                    LlmToolResultContent::Image(LlmImageData {
                                        source: img.source.to_string(),
                                        width: Some(img.size.width.0 as u32),
                                        height: Some(img.size.height.0 as u32),
                                    })
                                }
                            },
                        })
                    }
                    language_model::MessageContent::Thinking { text, signature } => {
                        LlmMessageContent::Thinking(LlmThinkingContent {
                            text: text.clone(),
                            signature: signature.clone(),
                        })
                    }
                    language_model::MessageContent::RedactedThinking(data) => {
                        LlmMessageContent::RedactedThinking(data.clone())
                    }
                })
                .collect(),
            cache: msg.cache,
        })
        .collect();

    let tools = request
        .tools
        .iter()
        .map(|tool| LlmToolDefinition {
            name: tool.name.clone(),
            description: tool.description.clone(),
            input_schema: serde_json::to_string(&tool.input_schema).unwrap_or_default(),
        })
        .collect();

    let tool_choice = request.tool_choice.as_ref().map(|choice| match choice {
        LanguageModelToolChoice::Auto => LlmToolChoice::Auto,
        LanguageModelToolChoice::Any => LlmToolChoice::Any,
        LanguageModelToolChoice::None => LlmToolChoice::None,
    });

    LlmCompletionRequest {
        messages,
        tools,
        tool_choice,
        stop_sequences: request.stop.clone(),
        temperature: request.temperature,
        thinking_allowed: request.thinking_allowed,
        max_tokens: None,
    }
}

fn convert_completion_event(event: LlmCompletionEvent) -> LanguageModelCompletionEvent {
    match event {
        LlmCompletionEvent::Started => LanguageModelCompletionEvent::Started,
        LlmCompletionEvent::Text(text) => LanguageModelCompletionEvent::Text(text),
        LlmCompletionEvent::Thinking(thinking) => LanguageModelCompletionEvent::Thinking {
            text: thinking.text,
            signature: thinking.signature,
        },
        LlmCompletionEvent::RedactedThinking(data) => {
            LanguageModelCompletionEvent::RedactedThinking { data }
        }
        LlmCompletionEvent::ToolUse(tool_use) => {
            LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                id: LanguageModelToolUseId::from(tool_use.id),
                name: tool_use.name.into(),
                raw_input: tool_use.input.clone(),
                input: serde_json::from_str(&tool_use.input).unwrap_or(serde_json::Value::Null),
                is_input_complete: true,
                thought_signature: tool_use.thought_signature,
            })
        }
        LlmCompletionEvent::ToolUseJsonParseError(error) => {
            LanguageModelCompletionEvent::ToolUseJsonParseError {
                id: LanguageModelToolUseId::from(error.id),
                tool_name: error.tool_name.into(),
                raw_input: error.raw_input.into(),
                json_parse_error: error.error,
            }
        }
        LlmCompletionEvent::Stop(reason) => LanguageModelCompletionEvent::Stop(match reason {
            LlmStopReason::EndTurn => StopReason::EndTurn,
            LlmStopReason::MaxTokens => StopReason::MaxTokens,
            LlmStopReason::ToolUse => StopReason::ToolUse,
            LlmStopReason::Refusal => StopReason::Refusal,
        }),
        LlmCompletionEvent::Usage(usage) => LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens.unwrap_or(0),
            cache_read_input_tokens: usage.cache_read_input_tokens.unwrap_or(0),
        }),
        LlmCompletionEvent::ReasoningDetails(json) => {
            LanguageModelCompletionEvent::ReasoningDetails(
                serde_json::from_str(&json).unwrap_or(serde_json::Value::Null),
            )
        }
    }
}
