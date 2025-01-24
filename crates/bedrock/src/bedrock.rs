mod models;

use anyhow::{anyhow, Context, Result};
use futures::{stream, Stream};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use aws_sdk_bedrockruntime as bedrock;
pub use aws_sdk_bedrockruntime as bedrock_client;
use aws_sdk_bedrockruntime::types::ContentBlockStopEvent;
use aws_sdk_bedrockruntime::types::ConverseStreamOutput::ContentBlockStop;
pub use bedrock::operation::converse_stream::ConverseStreamInput as BedrockStreamingRequest;
pub use bedrock::types::ContentBlock as BedrockRequestContent;
pub use bedrock::types::ConversationRole as BedrockRole;
use bedrock::types::ConverseOutput as Response;
pub use bedrock::types::ConverseStreamOutput as BedrockStreamingResponse;
pub use bedrock::types::Message as BedrockMessage;
pub use bedrock::types::ResponseStream as BedrockResponseStream;
use futures::stream::BoxStream;
pub use models::*;
use strum::Display;
use thiserror::Error;

pub async fn complete(
    client: &bedrock::Client,
    request: Request,
) -> Result<Response, BedrockError> {
    let mut response = bedrock::Client::converse(client)
        .model_id(request.model.clone())
        .set_messages(request.messages.into())
        .send()
        .await
        .context("Failed to send request to Bedrock");

    match response {
        Ok(output) => Ok(output.output.unwrap()),
        Err(err) => Err(BedrockError::Other(err)),
    }
}

pub async fn stream_completion(
    client: bedrock::Client,
    request: Request,
) -> Result<BoxStream<'static, Result<BedrockStreamingResponse, BedrockError>>> {
    // TODO: Make this use background executor rather than Tokio runtime
    async move {
        let response = bedrock::Client::converse_stream(&client)
            .model_id(request.model.clone())
            .set_messages(request.messages.into())
            .send()
            .await;

        match response {
            Ok(output) => {
                let stream: Pin<Box<dyn Stream<Item=Result<BedrockStreamingResponse, BedrockError>> + Send>> = Box::pin(stream::unfold(output.stream, |mut stream| async move {
                    match stream.recv().await {
                        Ok(Some(output)) => Some((Ok(output), stream)),
                        Ok(None) => Some((Ok(ContentBlockStop(ContentBlockStopEvent::builder().build().unwrap())), stream)),
                        Err(e) => {
                            Some((Err(BedrockError::Other(anyhow!("{:?}", aws_sdk_bedrockruntime::error::DisplayErrorContext(e)))), stream))
                        }
                    }
                }));
                Ok(stream)
            }
            Err(e) => Err(anyhow!("{:?}", aws_sdk_bedrockruntime::error::DisplayErrorContext(e))),
        }
    }.await
}

#[derive(Debug)]
pub struct Request {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<BedrockMessage>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub user_id: Option<String>,
}

#[derive(Error, Debug, Display)]
pub enum BedrockError {
    ClientError(anyhow::Error),
    ExtensionError(anyhow::Error),
    Other(anyhow::Error),
}
