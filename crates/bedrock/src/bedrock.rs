mod models;

use std::pin::Pin;

use anyhow::{anyhow, Context, Error, Result};
use aws_sdk_bedrockruntime as bedrock;
pub use aws_sdk_bedrockruntime as bedrock_client;
pub use aws_sdk_bedrockruntime::types::ContentBlock as BedrockInnerContent;
pub use bedrock::operation::converse_stream::ConverseStreamInput as BedrockStreamingRequest;
use bedrock::types::ConverseOutput as Response;
pub use bedrock::types::{
    ContentBlock as BedrockRequestContent, ConversationRole as BedrockRole,
    ConverseStreamOutput as BedrockStreamingResponse, Message as BedrockMessage,
    ResponseStream as BedrockResponseStream,
};
use futures::stream::BoxStream;
use futures::{stream, Stream};
pub use models::*;
use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error;

pub async fn complete(
    client: &bedrock::Client,
    request: Request,
) -> Result<Response, BedrockError> {
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
                            Err(e) => Some((
                                Err(BedrockError::Other(anyhow!(
                                    "{:?}",
                                    aws_sdk_bedrockruntime::error::DisplayErrorContext(e)
                                ))),
                                stream,
                            )),
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

#[derive(Debug)]
pub struct Request {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<BedrockMessage>,
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

#[derive(Error, Debug, Display)]
pub enum BedrockError {
    ClientError(anyhow::Error),
    ExtensionError(anyhow::Error),
    Other(anyhow::Error),
}
