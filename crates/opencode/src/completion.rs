//! Completion dispatch for OpenCode's advertised API protocols.
//!
//! Callers supply credentials, provider identity, and transport configuration,
//! keeping application settings and UI dependencies outside this crate.

use anthropic::completion::{AnthropicEventMapper, AnthropicPromptCacheMode, into_anthropic};
use futures::StreamExt;
use google_ai::completion::{GoogleEventMapper, into_google};
use http_client::{CustomHeaders, HttpClient};
use language_model_core::{
    LanguageModelCompletionError, LanguageModelCompletionStream, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelRequest, ReasoningEffort,
    chat_completion::ChatCompletionEventMapper,
};
use open_ai::completion::{
    ChatCompletionMaxTokensParameter, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response,
};

use crate::{
    ApiProtocol,
    catalog::{DiscoveredModel, normalize_reasoning_effort},
};

/// Transport and identity for a completion, independent of application settings.
pub struct CompletionConfig<'a> {
    pub model: &'a DiscoveredModel,
    pub api_url: &'a str,
    pub api_key: &'a str,
    pub provider_id: LanguageModelProviderId,
    pub provider_name: LanguageModelProviderName,
    pub custom_headers: &'a CustomHeaders,
}

/// Converts a request and streams it using the model's advertised protocol.
///
/// Errors retain provider details for the caller's presentation and retry policy.
pub async fn stream_completion(
    client: &dyn HttpClient,
    config: CompletionConfig<'_>,
    mut request: LanguageModelRequest,
) -> Result<LanguageModelCompletionStream, LanguageModelCompletionError> {
    let model = &config.model.model;
    let api_url = config.api_url.trim_end_matches('/');
    match model.protocol() {
        ApiProtocol::Anthropic => {
            let mode = if config.model.supports_thinking && request.thinking_allowed {
                anthropic::AnthropicModelMode::AdaptiveThinking
            } else {
                anthropic::AnthropicModelMode::Default
            };
            let mut request = into_anthropic(
                request,
                model.id().to_string(),
                1.0,
                model.max_output_tokens().unwrap_or(8192),
                mode,
                AnthropicPromptCacheMode::Automatic,
                &config.provider_id,
            )?;
            // OpenCode's Anthropic endpoint rejects the thinking-binding beta option.
            if let Some(anthropic::Thinking::Adaptive { block_binding, .. }) = &mut request.thinking
            {
                *block_binding = None;
            }
            let stream = anthropic::stream_completion(
                client,
                api_url,
                config.api_key,
                request,
                None,
                config.custom_headers,
            )
            .await?;
            Ok(
                AnthropicEventMapper::new(config.provider_name, config.provider_id)
                    .map_stream(stream.boxed())
                    .boxed(),
            )
        }
        ApiProtocol::OpenAiChat => {
            let effort = request
                .thinking_allowed
                .then(|| {
                    request
                        .thinking_effort
                        .as_deref()
                        .and_then(normalize_reasoning_effort)
                })
                .flatten();
            let mut request = into_open_ai(
                request,
                model.id(),
                true,
                false,
                model.max_output_tokens(),
                ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                effort,
                model.interleaved_reasoning(),
            )?;
            prepare_deepseek_reasoning_content(&mut request, model.interleaved_reasoning());
            let stream = open_ai::stream_completion(
                client,
                config.provider_name.0.as_ref(),
                &format!("{api_url}/v1"),
                config.api_key,
                request,
                config.custom_headers,
            )
            .await?;
            Ok(ChatCompletionEventMapper::new()
                .map_stream(stream.boxed())
                .boxed())
        }
        ApiProtocol::OpenAiResponses => {
            let supports_none = model
                .supported_reasoning_effort_levels()
                .is_some_and(|levels| levels.contains(&ReasoningEffort::None));
            let request = into_open_ai_response(
                request,
                model.id(),
                true,
                false,
                model.max_output_tokens(),
                None,
                supports_none,
                &config.provider_id,
            )?;
            let stream = open_ai::responses::stream_response(
                client,
                config.provider_name.0.as_ref(),
                &format!("{api_url}/v1"),
                config.api_key,
                request,
                config.custom_headers,
            )
            .await?;
            Ok(OpenAiResponseEventMapper::new(config.provider_id)
                .map_stream(stream.boxed())
                .boxed())
        }
        ApiProtocol::Google => {
            if request.max_output_tokens.is_some() {
                request.max_output_tokens =
                    request.effective_max_output_tokens(model.max_output_tokens());
            }
            let mode = if config.model.supports_thinking && request.thinking_allowed {
                google_ai::GoogleModelMode::Thinking {
                    budget_tokens: None,
                }
            } else {
                google_ai::GoogleModelMode::Default
            };
            let request = into_google(request, model.id().to_string(), mode)?;
            let stream = crate::stream_generate_content(
                client,
                api_url,
                config.api_key,
                request,
                config.custom_headers,
            )
            .await?;
            Ok(GoogleEventMapper::new().map_stream(stream.boxed()).boxed())
        }
    }
}

/// Keeps DeepSeek's required reasoning field on every assistant message.
///
/// Empty reasoning is not retained in conversation history, but DeepSeek still
/// requires the field when replaying an assistant tool call.
pub fn prepare_deepseek_reasoning_content(
    request: &mut open_ai::Request,
    interleaved_reasoning: bool,
) {
    if !interleaved_reasoning || !request.model.starts_with("deepseek-") {
        return;
    }
    for message in &mut request.messages {
        if let open_ai::RequestMessage::Assistant {
            reasoning_content, ..
        } = message
        {
            reasoning_content.get_or_insert_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use futures::{AsyncReadExt, TryStreamExt};
    use http_client::{AsyncBody, FakeHttpClient, Response, http};
    use language_model_core::{
        LanguageModelCompletionEvent, LanguageModelRequestMessage, MessageContent, Role,
    };

    #[test]
    fn completion_dispatches_protocols_and_maps_streamed_text() -> Result<()> {
        for (protocol, model_id, path, response) in [
            (
                ApiProtocol::Anthropic,
                "claude-fable-5-1",
                "/zen/go/v1/messages",
                "event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"Hello\"}}\n\n",
            ),
            (
                ApiProtocol::OpenAiChat,
                "model",
                "/zen/go/v1/chat/completions",
                "data: {\"id\":\"chat-1\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"model\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\ndata: [DONE]\n\n",
            ),
            (
                ApiProtocol::OpenAiResponses,
                "model",
                "/zen/go/v1/responses",
                "event: response.output_text.delta\ndata: {\"type\":\"response.output_text.delta\",\"item_id\":\"msg-1\",\"output_index\":0,\"content_index\":0,\"delta\":\"Hello\"}\n\n",
            ),
            (
                ApiProtocol::Google,
                "model",
                "/zen/go/v1/models/model:streamGenerateContent",
                "data: {\"candidates\":[{\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"Hello\"}]},\"finishReason\":\"STOP\"}]}\n\n",
            ),
        ] {
            futures::executor::block_on(async {
                let client = FakeHttpClient::create(move |mut request| async move {
                    assert_eq!(request.uri().path(), path);
                    assert_eq!(
                        request.headers().get("x-custom"),
                        Some(&http::HeaderValue::from_static("value"))
                    );
                    let mut body = String::new();
                    request.body_mut().read_to_string(&mut body).await?;
                    let body: serde_json::Value = serde_json::from_str(&body)?;
                    if protocol != ApiProtocol::Google {
                        assert_eq!(body["model"], model_id);
                    }
                    if protocol == ApiProtocol::Anthropic {
                        assert_eq!(body["thinking"]["type"], "adaptive");
                        assert!(body.pointer("/thinking/block_binding").is_none());
                    }
                    Ok(Response::builder()
                        .status(200)
                        .body(AsyncBody::from(response))?)
                });
                let model = DiscoveredModel {
                    model: crate::Model::new(
                        model_id.into(),
                        None,
                        1000,
                        Some(100),
                        protocol,
                        None,
                        None,
                        false,
                    ),
                    supports_images: false,
                    supports_thinking: protocol == ApiProtocol::Anthropic,
                };
                let headers = CustomHeaders::new(vec![(
                    http::HeaderName::from_static("x-custom"),
                    http::HeaderValue::from_static("value"),
                )]);
                let request = LanguageModelRequest {
                    thinking_allowed: true,
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![MessageContent::Text("Hi".into())],
                        cache: false,
                        reasoning_details: None,
                    }],
                    ..Default::default()
                };
                let events = stream_completion(
                    client.as_ref(),
                    CompletionConfig {
                        model: &model,
                        api_url: "https://opencode.ai/zen/go/",
                        api_key: "test-key",
                        provider_id: LanguageModelProviderId::new("opencode-go"),
                        provider_name: LanguageModelProviderName::new("OpenCode Go"),
                        custom_headers: &headers,
                    },
                    request,
                )
                .await?
                .try_collect::<Vec<_>>()
                .await?;
                let text = events
                    .into_iter()
                    .filter_map(|event| match event {
                        LanguageModelCompletionEvent::Text(text) => Some(text),
                        _ => None,
                    })
                    .collect::<String>();
                assert_eq!(text, "Hello", "{protocol:?}");
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }
}
