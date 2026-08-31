//! Wire types and event mapping for streaming responses of OpenAI-compatible
//! Chat Completions APIs.
//!
//! Multiple providers (OpenAI, OpenRouter, LM Studio, llama.cpp, and various
//! OpenAI-compatible proxies) share this format, so the types are deliberately
//! lenient: every field a consumer does not strictly require is optional or
//! defaulted, because real-world providers routinely omit fields or send
//! explicit `null`s where the OpenAI reference implementation would not.

use crate::util::{fix_streamed_json, parse_tool_arguments};
use crate::{
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelToolUse,
    LanguageModelToolUseInput, StopReason, TokenUsage,
};
use collections::HashMap;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::pin::Pin;

/// A single decoded Chat Completions stream chunk: either an event or a
/// provider error envelope (`{"error": {...}}`).
///
/// The error payload is generic so providers with richer error envelopes
/// (e.g. OpenRouter) can preserve their extra fields.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseStreamResult<E = ResponseStreamError> {
    Ok(ResponseStreamEvent),
    Err { error: E },
}

/// `#[derive(Deserialize)]` with `#[serde(untagged)]` is avoided here because
/// untagged enums report unhelpful errors ("data did not match any variant")
/// and, with the lenient event type below, would silently swallow error
/// envelopes as empty events. Instead:
///
/// * A non-null top-level `error` field decodes as an error, taking
///   precedence over any event payload in the same chunk.
/// * A chunk with a `choices` or `usage` field decodes as an event,
///   preserving the underlying deserialization error message on failure.
/// * Anything else (e.g. non-standard error envelopes like `{"detail": ...}`
///   or `{"object": "error", ...}`) is rejected so it surfaces as a stream
///   error instead of being silently dropped as an empty event.
impl<'de, E> Deserialize<'de> for ResponseStreamResult<E>
where
    E: serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value.get("error") {
            Some(error) if !error.is_null() => {
                let error = E::deserialize(error).map_err(serde::de::Error::custom)?;
                Ok(ResponseStreamResult::Err { error })
            }
            _ => {
                if value.get("choices").is_none() && value.get("usage").is_none() {
                    return Err(serde::de::Error::custom(format!(
                        "unrecognized chat completion stream chunk: {value}"
                    )));
                }
                let event =
                    ResponseStreamEvent::deserialize(&value).map_err(serde::de::Error::custom)?;
                Ok(ResponseStreamResult::Ok(event))
            }
        }
    }
}

/// The error payload most OpenAI-compatible providers send inside an
/// `{"error": {...}}` envelope.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResponseStreamEvent {
    /// Usage-only chunks from some providers omit `choices` entirely or send
    /// an explicit `null` instead of an empty array.
    #[serde(default, deserialize_with = "null_as_default")]
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    #[serde(default, deserialize_with = "null_as_default")]
    pub index: u32,
    pub delta: Option<ResponseMessageDelta>,
    pub finish_reason: Option<String>,
}

/// Deserializes a missing field or an explicit `null` as the type's default,
/// honoring this module's leniency guarantee for fields that are not
/// themselves optional.
fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub content: Option<String>,
    /// Reasoning text as sent by OpenRouter and compatible providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    /// Reasoning text as sent by DeepSeek-style providers and `llama-server`
    /// (when started with a reasoning format, e.g. `--reasoning-format deepseek`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
    /// Provider-defined structured reasoning metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    /// Some providers omit the index; treating it as 0 merges the chunks into
    /// a single tool call, which is correct for the common single-call case.
    #[serde(default, deserialize_with = "null_as_default")]
    pub index: usize,
    pub id: Option<String>,

    // There is also an optional `type` field that would determine if a
    // function is there. Sometimes this streams in with the `function` before
    // it streams in the `type`
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
    /// Provider-defined metadata required to replay a reasoning tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Usage {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
    /// Prompt-cache usage when reported by the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Reports prompt-cache token usage from compatible providers.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PromptTokensDetails {
    /// Tokens read from a prompt cache.
    pub cached_tokens: Option<u64>,
    /// Tokens written to a prompt cache.
    pub cache_write_tokens: Option<u64>,
}

impl Usage {
    /// Converts to a [`TokenUsage`] update, splitting cache reads and writes
    /// out of `prompt_tokens` when the provider reports them.
    ///
    /// Returns `None` unless both `prompt_tokens` and `completion_tokens` are
    /// present, because a partial usage object carries no usable totals.
    pub fn token_usage(&self) -> Option<TokenUsage> {
        let prompt_tokens = self.prompt_tokens?;
        let completion_tokens = self.completion_tokens?;
        let details = self.prompt_tokens_details.as_ref();
        let cache_creation_input_tokens = details
            .and_then(|details| details.cache_write_tokens)
            .unwrap_or(0);
        let cache_read_input_tokens = details
            .and_then(|details| details.cached_tokens)
            .unwrap_or(0);
        Some(TokenUsage {
            input_tokens: prompt_tokens
                .saturating_sub(cache_creation_input_tokens)
                .saturating_sub(cache_read_input_tokens),
            output_tokens: completion_tokens,
            cache_creation_input_tokens,
            cache_read_input_tokens,
        })
    }
}

/// Accumulates structured reasoning metadata from compatible providers.
///
/// Array entries are matched by `index` and then `id`. Fragmented `text`,
/// `summary`, and `data` fields are concatenated while other non-null fields
/// replace their previous values.
///
/// # Examples
///
/// ```
/// use language_model_core::chat_completion::ReasoningDetailsAccumulator;
/// use serde_json::json;
///
/// let mut accumulator = ReasoningDetailsAccumulator::default();
/// accumulator.push(json!([{"index": 0, "text": "first "}]));
/// let details = accumulator
///     .push(json!([{"index": 0, "text": "second"}]))
///     .expect("non-empty reasoning details");
///
/// assert_eq!(details[0]["text"], "first second");
/// ```
#[derive(Debug, Default)]
pub struct ReasoningDetailsAccumulator {
    accumulated: Option<Value>,
}

impl ReasoningDetailsAccumulator {
    /// Merges `chunk` and returns the updated metadata snapshot.
    ///
    /// `null` and empty arrays do not replace previously accumulated metadata
    /// and return `None`.
    pub fn push(&mut self, chunk: Value) -> Option<Value> {
        match chunk {
            Value::Null => None,
            Value::Array(chunks) if chunks.is_empty() => None,
            Value::Array(chunks) => {
                let mut details = match self.accumulated.take() {
                    Some(Value::Array(details)) => details,
                    _ => Vec::new(),
                };
                for chunk in chunks {
                    merge_reasoning_detail(&mut details, chunk);
                }
                let accumulated = Value::Array(details);
                self.accumulated = Some(accumulated.clone());
                Some(accumulated)
            }
            chunk => {
                self.accumulated = Some(chunk.clone());
                Some(chunk)
            }
        }
    }
}

fn merge_reasoning_detail(details: &mut Vec<Value>, chunk: Value) {
    let index = chunk.get("index").and_then(Value::as_u64);
    let target_index = index
        .and_then(|index| {
            details
                .iter()
                .position(|detail| detail.get("index").and_then(Value::as_u64) == Some(index))
        })
        .or_else(|| {
            let id = chunk.get("id").and_then(Value::as_str)?;
            details
                .iter()
                .position(|detail| detail.get("id").and_then(Value::as_str) == Some(id))
        });
    let Some(target_index) = target_index else {
        details.push(chunk);
        return;
    };
    let (Some(target), Some(chunk)) = (details[target_index].as_object_mut(), chunk.as_object())
    else {
        return;
    };
    for (key, value) in chunk {
        if matches!(key.as_str(), "text" | "summary" | "data")
            && let Some(fragment) = value.as_str()
            && let Some(existing) = target.get(key).and_then(Value::as_str)
        {
            target.insert(key.clone(), Value::String(format!("{existing}{fragment}")));
        } else if !value.is_null() {
            target.insert(key.clone(), value.clone());
        }
    }
}

#[derive(Default)]
struct RawToolCall {
    id: String,
    name: String,
    arguments: String,
    thought_signature: Option<String>,
}

/// Maps a stream of Chat Completions chunks to [`LanguageModelCompletionEvent`]s.
///
/// This is shared by every provider that speaks an OpenAI-compatible Chat
/// Completions dialect (OpenAI, OpenAI-compatible endpoints, OpenRouter,
/// LM Studio, llama.cpp, Bedrock/Mantle, and others), so its behavior must
/// stay provider-neutral: any provider-specific interpretation belongs in the
/// provider's own adapter before or after this mapping.
pub struct ChatCompletionEventMapper {
    tool_calls_by_index: HashMap<usize, RawToolCall>,
    reasoning_details: ReasoningDetailsAccumulator,
}

impl ChatCompletionEventMapper {
    pub fn new() -> Self {
        Self {
            tool_calls_by_index: HashMap::default(),
            reasoning_details: ReasoningDetailsAccumulator::default(),
        }
    }

    pub fn map_stream<E>(
        mut self,
        events: Pin<Box<dyn Send + Stream<Item = Result<ResponseStreamEvent, E>>>>,
    ) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>
    where
        E: Into<LanguageModelCompletionError>,
    {
        events.flat_map(move |event| {
            futures::stream::iter(match event {
                Ok(event) => self.map_event(event),
                Err(error) => vec![Err(error.into())],
            })
        })
    }

    pub fn map_event(
        &mut self,
        event: ResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        if let Some(token_usage) = event.usage.as_ref().and_then(|usage| usage.token_usage()) {
            events.push(Ok(LanguageModelCompletionEvent::UsageUpdate(token_usage)));
        }

        let Some(choice) = event.choices.first() else {
            return events;
        };

        if let Some(delta) = choice.delta.as_ref() {
            if let Some(reasoning_details) = delta.reasoning_details.clone()
                && let Some(reasoning_details) = self.reasoning_details.push(reasoning_details)
            {
                events.push(Ok(LanguageModelCompletionEvent::ReasoningDetails(
                    reasoning_details,
                )));
            }
            if let Some(reasoning) = delta.reasoning.clone() {
                push_thinking_event(reasoning, &mut events);
            }
            if let Some(reasoning_content) = delta.reasoning_content.clone() {
                push_thinking_event(reasoning_content, &mut events);
            }
            if let Some(content) = delta.content.clone() {
                if !content.is_empty() {
                    events.push(Ok(LanguageModelCompletionEvent::Text(content)));
                }
            }

            if let Some(tool_calls) = delta.tool_calls.as_ref() {
                for tool_call in tool_calls {
                    let entry = self.tool_calls_by_index.entry(tool_call.index).or_default();

                    if let Some(tool_id) = tool_call.id.clone()
                        && !tool_id.is_empty()
                    {
                        entry.id = tool_id;
                    }

                    if let Some(function) = tool_call.function.as_ref() {
                        if let Some(name) = function.name.clone()
                            && !name.is_empty()
                        {
                            entry.name = name;
                        }

                        if let Some(arguments) = function.arguments.clone() {
                            entry.arguments.push_str(&arguments);
                        }

                        if let Some(thought_signature) = function.thought_signature.clone() {
                            entry.thought_signature = Some(thought_signature);
                        }
                    }

                    if !entry.id.is_empty() && !entry.name.is_empty() {
                        if let Ok(input) =
                            serde_json::from_str::<Value>(&fix_streamed_json(&entry.arguments))
                        {
                            events.push(Ok(LanguageModelCompletionEvent::ToolUse(
                                LanguageModelToolUse {
                                    id: entry.id.clone().into(),
                                    name: entry.name.as_str().into(),
                                    is_input_complete: false,
                                    input: LanguageModelToolUseInput::Json(input),
                                    raw_input: entry.arguments.clone(),
                                    thought_signature: entry.thought_signature.clone(),
                                },
                            )));
                        }
                    }
                }
            }
        }

        match choice.finish_reason.as_deref() {
            Some("stop") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            Some("tool_calls") => {
                events.extend(self.tool_calls_by_index.drain().map(|(_, tool_call)| {
                    match parse_tool_arguments(&tool_call.arguments) {
                        Ok(input) => Ok(LanguageModelCompletionEvent::ToolUse(
                            LanguageModelToolUse {
                                id: tool_call.id.clone().into(),
                                name: tool_call.name.as_str().into(),
                                is_input_complete: true,
                                input: LanguageModelToolUseInput::Json(input),
                                raw_input: tool_call.arguments.clone(),
                                thought_signature: tool_call.thought_signature.clone(),
                            },
                        )),
                        Err(error) => Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                            id: tool_call.id.into(),
                            tool_name: tool_call.name.into(),
                            raw_input: tool_call.arguments.clone().into(),
                            json_parse_error: error.to_string(),
                        }),
                    }
                }));

                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)));
            }
            Some("length") => {
                events.push(Ok(LanguageModelCompletionEvent::Stop(
                    StopReason::MaxTokens,
                )));
            }
            Some(stop_reason) => {
                log::error!("Unexpected chat completion stop_reason: {stop_reason:?}",);
                events.push(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)));
            }
            None => {}
        }

        events
    }
}

fn push_thinking_event(
    text: String,
    events: &mut Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
) {
    if !text.is_empty() {
        events.push(Ok(LanguageModelCompletionEvent::Thinking {
            text,
            signature: None,
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LanguageModelToolUse;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    fn parse(chunk: &str) -> ResponseStreamResult {
        serde_json::from_str::<ResponseStreamResult>(chunk).unwrap()
    }

    fn expect_event(chunk: &str) -> ResponseStreamEvent {
        match parse(chunk) {
            ResponseStreamResult::Ok(event) => event,
            ResponseStreamResult::Err { .. } => panic!("expected an event, got an error: {chunk}"),
        }
    }

    #[test]
    fn parses_usage_only_chunk_with_null_prompt_cache_tokens() {
        let event = expect_event(
            r#"{"choices":[],"usage":{"prompt_tokens":5,"completion_tokens":3,"total_tokens":8,"prompt_tokens_details":{"cached_tokens":0,"cache_write_tokens":null}}}"#,
        );
        let details = event.usage.unwrap().prompt_tokens_details.unwrap();
        assert_eq!(details.cached_tokens, Some(0));
        assert_eq!(details.cache_write_tokens, None);
    }

    #[test]
    fn parses_chunk_without_choices() {
        let event = expect_event(r#"{"usage":{"total_tokens":8}}"#);
        assert!(event.choices.is_empty());
        assert_eq!(event.usage.unwrap().total_tokens, Some(8));
    }

    #[test]
    fn parses_explicit_nulls_for_non_optional_fields() {
        let event = expect_event(
            r#"{"choices":[{"index":null,"delta":{"tool_calls":[{"index":null,"id":"call_1","function":null}]},"finish_reason":null}],"usage":null}"#,
        );
        let choice = &event.choices[0];
        assert_eq!(choice.index, 0);
        let tool_call = &choice.delta.as_ref().unwrap().tool_calls.as_ref().unwrap()[0];
        assert_eq!(tool_call.index, 0);
        assert_eq!(tool_call.id.as_deref(), Some("call_1"));

        let event = expect_event(r#"{"choices":null,"usage":{"total_tokens":8}}"#);
        assert!(event.choices.is_empty());
        assert_eq!(event.usage.unwrap().total_tokens, Some(8));
    }

    #[test]
    fn parses_empty_usage_object() {
        let event = expect_event(r#"{"choices":[],"usage":{}}"#);
        let usage = event.usage.unwrap();
        assert_eq!(usage.prompt_tokens, None);
        assert_eq!(usage.total_tokens, None);
    }

    #[test]
    fn parses_tool_call_chunk_without_index() {
        let event = expect_event(
            r#"{"choices":[{"index":0,"delta":{"role":"assistant","tool_calls":[{"id":"call_1","type":"function","function":{"name":"edit_file","arguments":""}}]}}]}"#,
        );
        let mut choices = event.choices;
        let delta = choices.remove(0).delta.unwrap();
        let tool_call = delta.tool_calls.unwrap().remove(0);
        assert_eq!(tool_call.index, 0);
        assert_eq!(tool_call.id.as_deref(), Some("call_1"));
        assert_eq!(
            tool_call.function.unwrap().name.as_deref(),
            Some("edit_file")
        );
    }

    #[test]
    fn parses_error_envelope() {
        match parse(r#"{"error":{"message":"quota exceeded"}}"#) {
            ResponseStreamResult::Err { error } => assert_eq!(error.message, "quota exceeded"),
            ResponseStreamResult::Ok(_) => panic!("expected an error"),
        }
    }

    #[test]
    fn parses_custom_error_payload() {
        #[derive(Deserialize)]
        struct CustomError {
            code: u16,
            message: String,
        }

        let chunk = r#"{"error":{"code":429,"message":"slow down"}}"#;
        match serde_json::from_str::<ResponseStreamResult<CustomError>>(chunk).unwrap() {
            ResponseStreamResult::Err { error } => {
                assert_eq!(error.code, 429);
                assert_eq!(error.message, "slow down");
            }
            ResponseStreamResult::Ok(_) => panic!("expected an error"),
        }
    }

    #[test]
    fn error_takes_precedence_over_event_payload() {
        match parse(r#"{"error":{"message":"boom"},"choices":[{"delta":{"content":"hi"}}]}"#) {
            ResponseStreamResult::Err { error } => assert_eq!(error.message, "boom"),
            ResponseStreamResult::Ok(_) => panic!("expected an error"),
        }
    }

    #[test]
    fn rejects_unrecognized_chunks() {
        for chunk in [
            r#"{"detail":"Internal Server Error"}"#,
            r#"{"object":"error","message":"engine overloaded","code":50302}"#,
            r#""catastrophe""#,
        ] {
            let error = serde_json::from_str::<ResponseStreamResult>(chunk).unwrap_err();
            assert!(
                error.to_string().contains("unrecognized"),
                "expected {chunk} to be rejected, got: {error}"
            );
        }
    }

    #[test]
    fn null_error_field_is_not_an_error() {
        let event = expect_event(r#"{"error":null,"choices":[{"delta":{"content":"hi"}}]}"#);
        assert_eq!(
            event.choices[0].delta.as_ref().unwrap().content.as_deref(),
            Some("hi")
        );
    }

    #[test]
    fn token_usage_requires_both_totals() {
        let usage = Usage {
            prompt_tokens: Some(10),
            completion_tokens: None,
            ..Default::default()
        };
        assert!(usage.token_usage().is_none());
    }

    #[test]
    fn token_usage_splits_cache_tokens_out_of_prompt_tokens() {
        let usage = Usage {
            prompt_tokens: Some(12),
            completion_tokens: Some(7),
            total_tokens: Some(19),
            prompt_tokens_details: Some(PromptTokensDetails {
                cached_tokens: Some(5),
                cache_write_tokens: Some(3),
            }),
        };
        let token_usage = usage.token_usage().unwrap();
        assert_eq!(token_usage.input_tokens, 4);
        assert_eq!(token_usage.output_tokens, 7);
        assert_eq!(token_usage.cache_creation_input_tokens, 3);
        assert_eq!(token_usage.cache_read_input_tokens, 5);
    }

    #[test]
    fn reports_the_underlying_field_error() {
        let error = serde_json::from_str::<ResponseStreamResult>(
            r#"{"choices":[{"index":0,"delta":{"content":42}}]}"#,
        )
        .unwrap_err();
        let message = error.to_string();
        assert!(
            !message.contains("did not match any variant"),
            "expected a specific error, got: {message}"
        );
        assert!(
            message.contains("invalid type"),
            "expected the underlying field error, got: {message}"
        );
    }

    fn map_completion_events(
        events: Vec<ResponseStreamEvent>,
    ) -> Vec<LanguageModelCompletionEvent> {
        let mut mapper = ChatCompletionEventMapper::new();
        let mut all_events = Vec::new();
        for event in events {
            all_events.extend(mapper.map_event(event));
        }
        all_events.into_iter().filter_map(|e| e.ok()).collect()
    }

    #[test]
    fn stream_maps_reasoning() {
        let events = map_completion_events(vec![ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Some(ResponseMessageDelta {
                    content: None,
                    reasoning: Some("thinking".into()),
                    tool_calls: None,
                    reasoning_content: None,
                    reasoning_details: None,
                }),
                finish_reason: None,
            }],
            usage: None,
        }]);

        assert_eq!(
            events,
            vec![LanguageModelCompletionEvent::Thinking {
                text: "thinking".into(),
                signature: None,
            }]
        );
    }

    #[test]
    fn stream_maps_length_finish_reason_to_max_tokens_stop() {
        let events = map_completion_events(vec![ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: None,
                finish_reason: Some("length".into()),
            }],
            usage: None,
        }]);

        assert_eq!(
            events,
            vec![LanguageModelCompletionEvent::Stop(StopReason::MaxTokens)]
        );
    }

    #[test]
    fn chunk_without_choices_or_usage_maps_to_no_events() {
        let mut mapper = ChatCompletionEventMapper::new();
        assert!(mapper.map_event(ResponseStreamEvent::default()).is_empty());
    }

    #[test]
    fn usage_update_precedes_text_and_stop_events_from_the_same_chunk() {
        let mut mapper = ChatCompletionEventMapper::new();
        let events = mapper.map_event(ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Some(ResponseMessageDelta {
                    content: Some("Hello!".to_string()),
                    ..Default::default()
                }),
                finish_reason: Some("stop".to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: Some(11),
                completion_tokens: Some(7),
                total_tokens: Some(18),
                prompt_tokens_details: None,
            }),
        });

        assert!(matches!(
            events.as_slice(),
            [
                Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 11,
                    output_tokens: 7,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })),
                Ok(LanguageModelCompletionEvent::Text(text)),
                Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn)),
            ] if text == "Hello!"
        ));
    }

    #[test]
    fn usage_update_precedes_tool_use_and_stop_events_from_the_same_chunk() {
        let mut mapper = ChatCompletionEventMapper::new();
        let events = mapper.map_event(ResponseStreamEvent {
            choices: vec![ChoiceDelta {
                index: 0,
                delta: Some(ResponseMessageDelta {
                    tool_calls: Some(vec![ToolCallChunk {
                        index: 0,
                        id: Some("tool-call-id".to_string()),
                        function: Some(FunctionChunk {
                            name: Some("test_tool".to_string()),
                            arguments: Some(r#"{"value":1}"#.to_string()),
                            thought_signature: None,
                        }),
                    }]),
                    ..Default::default()
                }),
                finish_reason: Some("tool_calls".to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: Some(13),
                completion_tokens: Some(5),
                total_tokens: Some(18),
                prompt_tokens_details: None,
            }),
        });

        assert!(matches!(
            events.as_slice(),
            [
                Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 13,
                    output_tokens: 5,
                    cache_creation_input_tokens: 0,
                    cache_read_input_tokens: 0,
                })),
                Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    is_input_complete: false,
                    ..
                })),
                Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                    id,
                    name,
                    is_input_complete: true,
                    ..
                })),
                Ok(LanguageModelCompletionEvent::Stop(StopReason::ToolUse)),
            ] if id.to_string() == "tool-call-id" && name.as_ref() == "test_tool"
        ));
    }

    #[test]
    fn stream_merges_reasoning_details_and_maps_compatible_usage_and_signatures() {
        let response_events = serde_json::from_value(json!([
            {
                "choices": [{
                    "index": 0,
                    "delta": {
                        "reasoning_details": [{
                            "id": "reasoning-1",
                            "index": 0,
                            "type": "reasoning.text",
                            "text": "first "
                        }],
                        "tool_calls": [{
                            "index": 0,
                            "id": "call-1",
                            "function": {
                                "name": "search",
                                "arguments": "{",
                                "thought_signature": "signature"
                            }
                        }]
                    },
                    "finish_reason": null
                }],
                "usage": null
            },
            {
                "choices": [{
                    "index": 0,
                    "delta": {
                        "reasoning_details": [{
                            "id": "reasoning-1",
                            "index": 0,
                            "text": "second"
                        }],
                        "tool_calls": [{
                            "index": 0,
                            "function": {
                                "arguments": "}"
                            }
                        }]
                    },
                    "finish_reason": "tool_calls"
                }],
                "usage": null
            },
            {
                "choices": [],
                "usage": {
                    "prompt_tokens": 10000,
                    "completion_tokens": 500,
                    "total_tokens": 10500,
                    "prompt_tokens_details": {
                        "cached_tokens": 6000,
                        "cache_write_tokens": 1000
                    }
                }
            }
        ]))
        .expect("valid compatible Chat Completions events");
        let events = map_completion_events(response_events);

        assert!(events.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::ReasoningDetails(details)
                    if details[0]["text"] == "first second"
            )
        }));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::ToolUse(tool_use)
                    if tool_use.is_input_complete
                        && tool_use.thought_signature.as_deref() == Some("signature")
            )
        }));
        assert!(events.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                    input_tokens: 3_000,
                    output_tokens: 500,
                    cache_creation_input_tokens: 1_000,
                    cache_read_input_tokens: 6_000,
                })
            )
        }));
    }

    #[test]
    fn reasoning_details_accumulator_replaces_an_incompatible_previous_shape() {
        let mut accumulator = ReasoningDetailsAccumulator::default();
        assert_eq!(
            accumulator.push(json!({"summary": "provider-defined"})),
            Some(json!({"summary": "provider-defined"}))
        );
        assert_eq!(
            accumulator.push(json!([{"index": 0, "text": "reasoning"}])),
            Some(json!([{"index": 0, "text": "reasoning"}]))
        );
    }

    // OpenRouter sends an empty `reasoning_details` array in the finish chunk;
    // it must not wipe out details accumulated from earlier chunks.
    #[test]
    fn reasoning_details_accumulator_ignores_null_and_empty_array_chunks() {
        let mut accumulator = ReasoningDetailsAccumulator::default();
        assert!(
            accumulator
                .push(json!([{"index": 0, "type": "reasoning.text", "text": "thinking"}]))
                .is_some()
        );
        assert!(
            accumulator
                .push(
                    json!([{"index": 0, "type": "reasoning.encrypted", "data": "encrypted-blob"}])
                )
                .is_some()
        );

        assert_eq!(accumulator.push(json!([])), None);
        assert_eq!(accumulator.push(serde_json::Value::Null), None);

        let details = accumulator
            .push(json!([{"index": 0, "text": " more"}]))
            .expect("accumulated reasoning details");
        assert_eq!(details[0]["text"], "thinking more");
        assert_eq!(details[0]["data"], "encrypted-blob");
    }

    #[test]
    fn stream_maps_preserves_tool_id_and_name_across_empty_deltas() {
        // DashScope sends id="" and name="" in subsequent tool_calls delta
        // chunks after the first chunk. ChatCompletionEventMapper must not overwrite
        // the accumulated id and name with these empty strings.

        let events = vec![
            // First chunk: id and name are present
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Some(ResponseMessageDelta {
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("call_dashscope_test".into()),
                            function: Some(FunctionChunk {
                                name: Some("list_directory".into()),
                                arguments: Some("".into()),
                                thought_signature: None,
                            }),
                        }]),
                        reasoning_content: None,
                        reasoning_details: None,
                    }),
                    finish_reason: None,
                }],
                usage: None,
            },
            // Subsequent chunks: DashScope sends id="" and name=""
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Some(ResponseMessageDelta {
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("".into()),
                            function: Some(FunctionChunk {
                                name: Some("".into()),
                                arguments: Some("{\"path\": \"".into()),
                                thought_signature: None,
                            }),
                        }]),
                        reasoning_content: None,
                        reasoning_details: None,
                    }),
                    finish_reason: None,
                }],
                usage: None,
            },
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: Some(ResponseMessageDelta {
                        content: None,
                        reasoning: None,
                        tool_calls: Some(vec![ToolCallChunk {
                            index: 0,
                            id: Some("".into()),
                            function: Some(FunctionChunk {
                                name: Some("".into()),
                                arguments: Some("blog-scraper\"}".into()),
                                thought_signature: None,
                            }),
                        }]),
                        reasoning_content: None,
                        reasoning_details: None,
                    }),
                    finish_reason: None,
                }],
                usage: None,
            },
            // Final chunk: finish_reason = "tool_calls"
            ResponseStreamEvent {
                choices: vec![ChoiceDelta {
                    index: 0,
                    delta: None,
                    finish_reason: Some("tool_calls".into()),
                }],
                usage: None,
            },
        ];

        let mapped = map_completion_events(events);

        // Events emitted:
        //   1. Partial ToolUse from chunk 1 (fix_json("") → "{}", parseable)
        //   2. Partial ToolUse from chunk 3 (arguments fully assembled)
        //   3. Complete ToolUse from finish_reason="tool_calls" drain
        //   4. Stop(ToolUse)
        assert_eq!(mapped.len(), 4);

        // Verify the complete ToolUse event (from finish_reason drain)
        // has the correct id, name, and accumulated arguments.
        let complete_tool_use = mapped.iter().find_map(|event| {
            if let LanguageModelCompletionEvent::ToolUse(tool_use) = event {
                if tool_use.is_input_complete {
                    return Some(tool_use);
                }
            }
            None
        });
        assert!(
            complete_tool_use.is_some(),
            "expected a completed ToolUse event"
        );
        let tool_use = complete_tool_use.unwrap();
        assert_eq!(
            tool_use.id.to_string(),
            "call_dashscope_test",
            "id must survive empty-string overwrites"
        );
        assert_eq!(
            tool_use.name.as_ref(),
            "list_directory",
            "name must survive empty-string overwrites"
        );
        assert_eq!(
            tool_use.raw_input, "{\"path\": \"blog-scraper\"}",
            "arguments should accumulate across chunks"
        );

        // Verify the Stop event
        assert!(mapped.iter().any(|event| {
            matches!(
                event,
                LanguageModelCompletionEvent::Stop(StopReason::ToolUse)
            )
        }));
    }
}
