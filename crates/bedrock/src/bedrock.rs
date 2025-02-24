mod models;

use std::fmt::Display;
use std::pin::Pin;

use anyhow::{anyhow, Context, Error, Result};
use aws_sdk_bedrockruntime as bedrock;
pub use aws_sdk_bedrockruntime as bedrock_client;
pub use aws_sdk_bedrockruntime::types::ContentBlock as BedrockInnerContent;
pub use aws_sdk_bedrockruntime::types::{
    SpecificToolChoice as BedrockSpecificTool, ToolChoice as BedrockToolChoice,
    ToolInputSchema as BedrockToolInputSchema, ToolSpecification as BedrockTool,
};
pub use bedrock::operation::converse_stream::ConverseStreamInput as BedrockStreamingRequest;
pub use bedrock::types::{
    ContentBlock as BedrockRequestContent, ConversationRole as BedrockRole,
    ConverseOutput as BedrockResponse, ConverseStreamOutput as BedrockStreamingResponse,
    Message as BedrockMessage, ResponseStream as BedrockResponseStream,
};
use futures::stream::BoxStream;
use futures::{stream, Stream};
pub use models::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub async fn complete(
    client: &bedrock::Client,
    request: Request,
) -> Result<BedrockResponse, BedrockError> {
    let response = bedrock::Client::converse(client)
        .model_id(request.model.clone())
        .set_messages(request.messages.into())
        .send()
        .await
        .context("Failed to send request to Bedrock");

    match response {
        Ok(output) => output
            .output
            .ok_or_else(|| BedrockError::Other(anyhow!("no output"))),
        Err(err) => Err(BedrockError::Other(err)),
    }
}

pub async fn stream_completion(
    client: bedrock::Client,
    request: Request,
    handle: tokio::runtime::Handle,
) -> Result<BoxStream<'static, Result<BedrockStreamingResponse, BedrockError>>, Error> {
    handle
        .spawn(async move {
            let response = bedrock::Client::converse_stream(&client)
                .model_id(request.model.clone())
                .set_messages(request.messages.into())
                .send()
                .await;

            match response {
                Ok(output) => {
                    let stream: Pin<
                        Box<
                            dyn Stream<Item = Result<BedrockStreamingResponse, BedrockError>>
                                + Send,
                        >,
                    > = Box::pin(stream::unfold(output.stream, |mut stream| async move {
                        match stream.recv().await {
                            Ok(Some(output)) => Some((Ok(output), stream)),
                            Ok(None) => None,
                            Err(e) => {
                                Some((
                                    // TODO: Figure out how we can capture Throttling Exceptions
                                    Err(BedrockError::ClientError(anyhow!(
                                        "{:?}",
                                        aws_sdk_bedrockruntime::error::DisplayErrorContext(e)
                                    ))),
                                    stream,
                                ))
                            }
                        }
                    }));
                    Ok(stream)
                }
                Err(e) => Err(anyhow!(
                    "{:?}",
                    aws_sdk_bedrockruntime::error::DisplayErrorContext(e)
                )),
            }
        })
        .await
        .map_err(|e| anyhow!("Failed to spawn task: {:?}", e))?
}

use aws_smithy_types::Document;
use aws_smithy_types::Number as AwsNumber;
use serde_json::{Number, Value};

pub fn aws_document_to_value(doc: &Document) -> Value {
    match doc {
        Document::Null => Value::Null,
        Document::Bool(b) => Value::Bool(*b),
        Document::Number(n) => match n {
            AwsNumber::PosInt(i) => Value::Number(Number::from(*i)),
            AwsNumber::NegInt(i) => Value::Number(Number::from(*i)),
            AwsNumber::Float(f) => Value::Number(Number::from_f64(*f).unwrap()),
        },
        Document::String(s) => Value::String(s.clone()),
        Document::Array(arr) => Value::Array(arr.iter().map(aws_document_to_value).collect()),
        Document::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), aws_document_to_value(v)))
                .collect(),
        ),
    }
}

pub fn value_to_aws_document(value: &Value) -> Document {
    match value {
        Value::Null => Document::Null,
        Value::Bool(b) => Document::Bool(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_u64() {
                Document::Number(AwsNumber::PosInt(i))
            } else if let Some(i) = n.as_i64() {
                Document::Number(AwsNumber::NegInt(i))
            } else if let Some(f) = n.as_f64() {
                Document::Number(AwsNumber::Float(f))
            } else {
                Document::Null
            }
        }
        Value::String(s) => Document::String(s.clone()),
        Value::Array(arr) => Document::Array(arr.iter().map(value_to_aws_document).collect()),
        Value::Object(map) => Document::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), value_to_aws_document(v)))
                .collect(),
        ),
    }
}

#[derive(Debug)]
pub struct Request {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<BedrockMessage>,
    pub tools: Vec<BedrockTool>,
    pub tool_choice: Option<BedrockToolChoice>,
    pub system: Option<String>,
    pub metadata: Option<Metadata>,
    pub stop_sequences: Vec<String>,
    pub temperature: Option<f32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub user_id: Option<String>,
}

#[derive(Error, Debug)]
pub enum BedrockError {
    ClientError(anyhow::Error),
    ExtensionError(anyhow::Error),
    Other(anyhow::Error),
}

impl Display for BedrockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BedrockError::ClientError(e) => write!(f, "ClientError: {}", e),
            BedrockError::ExtensionError(e) => write!(f, "ExtensionError: {}", e),
            BedrockError::Other(e) => write!(f, "Other: {}", e),
        }
    }
}
