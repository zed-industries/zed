mod models;

use anyhow::{anyhow, Context, Error, Result};
use futures::{stream, Stream};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use aws_sdk_bedrockruntime as bedrock;
pub use aws_sdk_bedrockruntime as bedrock_client;
pub use aws_sdk_bedrockruntime::types::ContentBlock as BedrockInnerContent;
use aws_sdk_bedrockruntime::types::{ContentBlockDelta, ContentBlockStopEvent};
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
    handle: tokio::runtime::Handle,
) -> Result<BoxStream<'static, Result<BedrockStreamingResponse, BedrockError>>, Error> {
    handle.spawn(async move {
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
                        Ok(None) => Some((Ok(BedrockStreamingResponse::ContentBlockStop(ContentBlockStopEvent::builder().build().unwrap())), stream)),
                        Err(e) => {
                            Some((Err(BedrockError::Other(anyhow!("{:?}", aws_sdk_bedrockruntime::error::DisplayErrorContext(e)))), stream))
                        }
                    }
                }));
                Ok(stream)
            }
            Err(e) => Err(anyhow!("{:?}", aws_sdk_bedrockruntime::error::DisplayErrorContext(e))),
        }
    }).await.map_err(|e| anyhow!("Failed to spawn task: {:?}", e))?
}


fn map_stream_output(output: BedrockStreamingResponse) -> Option<Result<String, BedrockError>> {
    match output {
        BedrockStreamingResponse::ContentBlockDelta(event) => {
            if let Some(ContentBlockDelta::Text(text)) = event.delta {
                Some(Ok(text))
            } else {
                Some(Err(BedrockError::Other(anyhow!(
                    "Received non-text content block delta"
                ))))
            }
        }
        BedrockStreamingResponse::ContentBlockStart(event) => {
            //Todo: Implement tool use
            Some(Err(BedrockError::Other(anyhow!(
                "Received tool use event"
            ))))
        }
        BedrockStreamingResponse::ContentBlockStop(_) => {
            Some(Err(BedrockError::Other(anyhow!(
                "Received tool use content block stop event"
            ))))
        }
        BedrockStreamingResponse::MessageStart(_) |
        BedrockStreamingResponse::MessageStop(_) |
        BedrockStreamingResponse::Metadata(_) => {
            // Todo: This shouldn't happen, but we'll see
            Some(Err(BedrockError::Other(anyhow!(
                "Received unexpected event"
            ))))
        }
        _ => {
            Some(Err(BedrockError::Other(anyhow!(
                "Received unknown non-exhaustive event."
            ))))
        }
    }
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
