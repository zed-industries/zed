mod models;

use std::time::Duration;
use std::{pin::Pin, str::FromStr};
use std::any::Any;
use anyhow::{anyhow, Context, Error, Result};
use aws_sdk_bedrockruntime::types::{ContentBlockDelta, ContentBlockDeltaEvent, ContentBlockStopEvent, ConverseStreamOutput};
use chrono::{DateTime, Utc};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use thiserror::Error;

pub use aws_sdk_bedrockruntime as bedrock;
use aws_sdk_bedrockruntime::config::http::HttpResponse;
use aws_sdk_bedrockruntime::operation::converse::{ConverseError, ConverseOutput};
pub use bedrock::operation::converse_stream::ConverseStreamInput as StreamingRequest;
pub use bedrock::types::ContentBlock as RequestContent;
pub use bedrock::types::ConverseOutput as Response;
pub use bedrock::types::Message;
pub use bedrock::types::ConversationRole;
pub use bedrock::types::ResponseStream;

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
            Ok(output.into())
        }
        Err(err) => {
            Err(anyhow!(err))
        }
    }
}

pub async fn stream_completion(
    client: &bedrock::Client,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<BedrockEvent, BedrockError>>, BedrockError> { // There is no generic Bedrock event Type?

    let response = bedrock::Client::converse_stream(client)
        .model_id(request.model)
        .set_messages(request.messages.into()).send().await;

    let mut stream = match response {
        Ok(output) => Ok(output.stream),
        Err(e) => {
            // TODO: Figure this out

            unimplemented!();
        }
    };

    if stream.is_ok() {
        let reader = BufReader::new(stream);
        let stream = reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let event: bedrock = get_converse_output_text(line);
                        Some(Ok(event))
                    }
                    Err(e) => Some(Err(e.into())),
                }
            }).boxed();

        Ok(stream)
    }
}

fn get_converse_output_text(
    output: ConverseStreamOutput,
) -> Result<String, bedrock::operation::converse_stream::ConverseStreamError> {
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
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CacheControlType {
    Ephemeral,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: CacheControlType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseContent {
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<u32>,
}

#[derive(Error, Debug)]
pub enum BedrockError {
    SdkError(bedrock::Error),
    Other(anyhow::Error)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}
