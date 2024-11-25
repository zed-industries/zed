mod models;

use std::{str::FromStr};
use std::any::Any;
use std::io::BufReader;
use anyhow::{anyhow, Context, Result};
use aws_sdk_bedrockruntime::types::{ContentBlockDeltaEvent, ContentBlockStart, ContentBlockStartEvent, ConverseStreamMetadataEvent, ConverseStreamOutput, Message, MessageStartEvent, MessageStopEvent};
use futures::{stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use aws_sdk_bedrockruntime as bedrock;
pub use aws_sdk_bedrockruntime as bedrock_client;
use aws_sdk_bedrockruntime::types::error::ConverseStreamOutputError;
pub use bedrock::types::ConverseStreamOutput as BedrockStreamingResponse;
pub use bedrock::operation::converse_stream::ConverseStreamInput as BedrockStreamingRequest;
pub use bedrock::types::ContentBlock as BedrockRequestContent;
use bedrock::types::ConverseOutput as Response;
pub use bedrock::types::Message as BedrockMessage;
pub use bedrock::types::ConversationRole as BedrockRole;
pub use bedrock::types::ResponseStream as BedrockResponseStream;

//TODO: Re-export the Bedrock stuff
// https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html

pub use models::*;

pub async fn complete(
    client: &bedrock::Client,
    request: Request,
) -> Result<Response, BedrockError> {
    let mut response = bedrock::Client::converse(client)
        .model_id(request.model.clone())
        .set_messages(request.messages.into())
        .send().await.context("Failed to send request to Bedrock");

    match response {
        Ok(output) => {
            Ok(output.output.unwrap())
        }
        Err(err) => {
            Err(BedrockError::Other(err))
        }
    }
}

pub async fn stream_completion(
    client: &bedrock::Client,
    request: Request,
) -> Result<BoxStream<'static, Result<BedrockStreamingResponse, BedrockError>>, BedrockError> { // There is no generic Bedrock event Type?

    let response = bedrock::Client::converse_stream(client)
        .model_id(request.model)
        .set_messages(request.messages.into()).send().await;


    let mut stream = match response {
        Ok(output) => Ok(output.stream()),
        Err(e) => Err(
            BedrockError::ClientError(anyhow!(e))
        ),
    }?;



    Ok()
}

fn get_converse_output_text(
    output: ConverseStreamOutput,
) -> Result<String, BedrockError> {
    Ok(match output {
        ConverseStreamOutput::ContentBlockDelta(c) => {
            match c.delta() {
                Some(delta) => delta.as_text().cloned().unwrap_or_else(|_| "".into()),
                None => "".into(),
            }
        }
        _ => {
            String::from("")
        }
    })
}
//TODO: A LOT of these types need to re-export the Bedrock types instead of making custom ones

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    Other(anyhow::Error)
}
