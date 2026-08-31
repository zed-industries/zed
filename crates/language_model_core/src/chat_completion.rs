//! Wire types for streaming responses of OpenAI-compatible Chat Completions
//! APIs.
//!
//! Multiple providers (OpenAI, OpenRouter, LM Studio, llama.cpp, and various
//! OpenAI-compatible proxies) share this format, so the types are deliberately
//! lenient: every field a consumer does not strictly require is optional or
//! defaulted, because real-world providers routinely omit fields or send
//! explicit `null`s where the OpenAI reference implementation would not.

use crate::TokenUsage;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    /// Usage-only chunks from some providers omit `choices` entirely instead
    /// of sending an empty array.
    #[serde(default)]
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    #[serde(default)]
    pub index: u32,
    pub delta: Option<ResponseMessageDelta>,
    pub finish_reason: Option<String>,
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
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
