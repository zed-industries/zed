mod models;

use anyhow::{Result, anyhow};
use aws_sdk_bedrockruntime as bedrock;
pub use aws_sdk_bedrockruntime as bedrock_client;
pub use aws_sdk_bedrockruntime::types::{
    AnyToolChoice as BedrockAnyToolChoice, AutoToolChoice as BedrockAutoToolChoice,
    ContentBlock as BedrockInnerContent, Tool as BedrockTool, ToolChoice as BedrockToolChoice,
    ToolConfiguration as BedrockToolConfig, ToolInputSchema as BedrockToolInputSchema,
    ToolSpecification as BedrockToolSpec,
};
use aws_sdk_bedrockruntime::types::{GuardrailStreamConfiguration, InferenceConfiguration};
pub use aws_smithy_types::Blob as BedrockBlob;
use aws_smithy_types::{Document, Number as AwsNumber};
pub use bedrock::operation::converse_stream::ConverseStreamInput as BedrockStreamingRequest;
pub use bedrock::types::{
    ContentBlock as BedrockRequestContent, ConversationRole as BedrockRole,
    ConverseOutput as BedrockResponse, ConverseStreamOutput as BedrockStreamingResponse,
    ImageBlock as BedrockImageBlock, ImageFormat as BedrockImageFormat,
    ImageSource as BedrockImageSource, Message as BedrockMessage,
    ReasoningContentBlock as BedrockThinkingBlock, ReasoningTextBlock as BedrockThinkingTextBlock,
    ResponseStream as BedrockResponseStream, SystemContentBlock as BedrockSystemContentBlock,
    ToolResultBlock as BedrockToolResultBlock,
    ToolResultContentBlock as BedrockToolResultContentBlock,
    ToolResultStatus as BedrockToolResultStatus, ToolUseBlock as BedrockToolUseBlock,
};
use futures::stream::{self, BoxStream};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::HashMap;
use thiserror::Error;

pub use crate::models::*;

pub async fn stream_completion(
    client: bedrock::Client,
    request: Request,
    extra_headers: http_client::CustomHeaders,
) -> Result<BoxStream<'static, Result<BedrockStreamingResponse, anyhow::Error>>, BedrockError> {
    let mut response = bedrock::Client::converse_stream(&client)
        .model_id(request.model.clone())
        .set_messages(request.messages.into());

    let mut additional_fields: HashMap<String, Document> = HashMap::new();

    if let Some(thinking) = &request.thinking {
        additional_fields.extend(thinking_request_fields(thinking));
    }

    if !additional_fields.is_empty() {
        response = response.additional_model_request_fields(Document::Object(additional_fields));
    }

    if request.tools.as_ref().is_some_and(|t| !t.tools.is_empty()) {
        response = response.set_tool_config(request.tools);
    }

    let inference_config = InferenceConfiguration::builder()
        .max_tokens(request.max_tokens as i32)
        .set_temperature(request.temperature)
        .set_top_p(request.top_p)
        .build();

    response = response.inference_config(inference_config);

    for system_block in request.system {
        response = response.system(system_block);
    }

    if let Some(guardrail_id) = &request.guardrail_identifier {
        let version = request.guardrail_version.as_deref().unwrap_or("DRAFT");

        response = response.guardrail_config(
            GuardrailStreamConfiguration::builder()
                .guardrail_identifier(guardrail_id)
                .guardrail_version(version)
                .build(),
        );
    }

    let output = response
        .customize()
        .mutate_request(move |http_request| {
            let headers = http_request.headers_mut();
            for (name, value) in extra_headers.iter() {
                headers.insert(
                    name.as_str().to_owned(),
                    value.to_str().unwrap_or("").to_owned(),
                );
            }
        })
        .send()
        .await
        .map_err(|err| match err {
            bedrock::error::SdkError::ServiceError(ctx) => {
                use bedrock::operation::converse_stream::ConverseStreamError;
                let err = ctx.into_err();
                match &err {
                    ConverseStreamError::ValidationException(e) => BedrockError::Validation(
                        e.message().unwrap_or("validation error").to_string(),
                    ),
                    ConverseStreamError::ThrottlingException(_) => BedrockError::RateLimited,
                    ConverseStreamError::ServiceUnavailableException(_)
                    | ConverseStreamError::ModelNotReadyException(_) => {
                        BedrockError::ServiceUnavailable
                    }
                    ConverseStreamError::AccessDeniedException(e) => BedrockError::AccessDenied(
                        e.message().unwrap_or("access denied").to_string(),
                    ),
                    ConverseStreamError::InternalServerException(e) => {
                        BedrockError::InternalServer(
                            e.message().unwrap_or("internal server error").to_string(),
                        )
                    }
                    _ => BedrockError::Other(err.into()),
                }
            }
            other => BedrockError::Other(other.into()),
        });

    let stream = Box::pin(stream::unfold(
        output?.stream,
        move |mut stream| async move {
            match stream.recv().await {
                Ok(Some(output)) => Some((Ok(output), stream)),
                Ok(None) => None,
                Err(err) => Some((
                    Err(anyhow!(
                        "{}",
                        aws_sdk_bedrockruntime::error::DisplayErrorContext(err)
                    )),
                    stream,
                )),
            }
        },
    ));

    Ok(stream)
}

pub fn aws_document_to_value(document: &Document) -> Value {
    match document {
        Document::Null => Value::Null,
        Document::Bool(value) => Value::Bool(*value),
        Document::Number(value) => match *value {
            AwsNumber::PosInt(value) => Value::Number(Number::from(value)),
            AwsNumber::NegInt(value) => Value::Number(Number::from(value)),
            AwsNumber::Float(value) => Value::Number(Number::from_f64(value).unwrap()),
        },
        Document::String(value) => Value::String(value.clone()),
        Document::Array(array) => Value::Array(array.iter().map(aws_document_to_value).collect()),
        Document::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| (key.clone(), aws_document_to_value(value)))
                .collect(),
        ),
    }
}

pub fn value_to_aws_document(value: &Value) -> Document {
    match value {
        Value::Null => Document::Null,
        Value::Bool(value) => Document::Bool(*value),
        Value::Number(value) => {
            if let Some(value) = value.as_u64() {
                Document::Number(AwsNumber::PosInt(value))
            } else if let Some(value) = value.as_i64() {
                Document::Number(AwsNumber::NegInt(value))
            } else if let Some(value) = value.as_f64() {
                Document::Number(AwsNumber::Float(value))
            } else {
                Document::Null
            }
        }
        Value::String(value) => Document::String(value.clone()),
        Value::Array(array) => Document::Array(array.iter().map(value_to_aws_document).collect()),
        Value::Object(map) => Document::Object(
            map.iter()
                .map(|(key, value)| (key.clone(), value_to_aws_document(value)))
                .collect(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Thinking {
    Enabled {
        budget_tokens: Option<u64>,
    },
    Adaptive {
        effort: BedrockAdaptiveThinkingEffort,
    },
    /// Explicitly turns thinking off. Required by Claude Opus 5, where
    /// adaptive thinking runs by default when the `thinking` field is
    /// omitted; only accepted at effort `high` or below.
    ///
    /// <https://docs.aws.amazon.com/bedrock/latest/userguide/model-card-anthropic-claude-opus-5.html>
    Disabled,
}

/// Converts the request's thinking configuration into the
/// `additionalModelRequestFields` entries understood by Anthropic models on
/// the Converse API.
fn thinking_request_fields(thinking: &Thinking) -> HashMap<String, Document> {
    let mut fields = HashMap::new();
    match thinking {
        Thinking::Enabled {
            budget_tokens: Some(budget_tokens),
        } => {
            fields.insert(
                "thinking".to_string(),
                Document::from(HashMap::from([
                    ("type".to_string(), Document::String("enabled".to_string())),
                    (
                        "budget_tokens".to_string(),
                        Document::Number(AwsNumber::PosInt(*budget_tokens)),
                    ),
                ])),
            );
        }
        Thinking::Enabled {
            budget_tokens: None,
        } => {}
        Thinking::Adaptive { effort: _ } => {
            fields.insert(
                "thinking".to_string(),
                Document::from(HashMap::from([
                    ("type".to_string(), Document::String("adaptive".to_string())),
                    (
                        "display".to_string(),
                        Document::String("summarized".to_string()),
                    ),
                ])),
            );
        }
        Thinking::Disabled => {
            // On Claude Opus 5 omitting the `thinking` field means adaptive
            // thinking runs by default, so turning it off requires this
            // explicit opt-out. No effort is attached: `disabled` combined
            // with effort `xhigh`/`max` is rejected with a 400.
            fields.insert(
                "thinking".to_string(),
                Document::from(HashMap::from([(
                    "type".to_string(),
                    Document::String("disabled".to_string()),
                )])),
            );
        }
    }
    fields
}

#[derive(Debug)]
pub struct Request {
    pub model: String,
    pub max_tokens: u64,
    pub messages: Vec<BedrockMessage>,
    pub tools: Option<BedrockToolConfig>,
    pub thinking: Option<Thinking>,
    /// System content blocks in prefix order. Typically `[Text(...)]` or, when
    /// the model supports prompt caching, `[Text(...), CachePoint(...)]` so the
    /// system prompt anchors its own cache prefix independent of tools and
    /// messages.
    pub system: Vec<BedrockSystemContentBlock>,
    pub metadata: Option<Metadata>,
    pub stop_sequences: Vec<String>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub guardrail_identifier: Option<String>,
    pub guardrail_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub user_id: Option<String>,
}

#[derive(Error, Debug)]
pub enum BedrockError {
    #[error("{0}")]
    Validation(String),
    #[error("rate limited")]
    RateLimited,
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error("{0}")]
    AccessDenied(String),
    #[error("{0}")]
    InternalServer(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_field<'a>(document: &'a Document, key: &str) -> Option<&'a str> {
        match document {
            Document::Object(map) => match map.get(key) {
                Some(Document::String(value)) => Some(value.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    #[test]
    fn test_disabled_thinking_serializes_opt_out_without_effort() {
        let fields = thinking_request_fields(&Thinking::Disabled);

        let thinking = fields.get("thinking").expect("thinking field");
        assert_eq!(string_field(thinking, "type"), Some("disabled"));
        // `disabled` combined with an effort of `xhigh`/`max` is a 400, so no
        // output_config may accompany the opt-out.
        assert!(!fields.contains_key("output_config"));
    }

    #[test]
    fn test_enabled_thinking_serializes_budget_tokens() {
        let fields = thinking_request_fields(&Thinking::Enabled {
            budget_tokens: Some(4_096),
        });

        let thinking = fields.get("thinking").expect("thinking field");
        assert_eq!(string_field(thinking, "type"), Some("enabled"));
        match thinking {
            Document::Object(map) => assert_eq!(
                map.get("budget_tokens"),
                Some(&Document::Number(AwsNumber::PosInt(4_096)))
            ),
            _ => panic!("thinking field should be an object"),
        }

        let fields = thinking_request_fields(&Thinking::Enabled {
            budget_tokens: None,
        });
        assert!(fields.is_empty());
    }
}
