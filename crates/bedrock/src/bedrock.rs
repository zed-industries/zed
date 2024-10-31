use std::time::Duration;
use std::{pin::Pin, str::FromStr};

use anyhow::{anyhow, Context, Result};
use aws_sdk_bedrockruntime::types::ConverseStreamOutput;
use chrono::{DateTime, Utc};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, FutureExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use thiserror::Error;
use util::ResultExt as _;

pub use aws_sdk_bedrockruntime as bedrock;

pub use bedrock::operation::converse_stream::ConverseStreamInput as StreamingRequest;
pub use bedrock::Error as BedrockError;

//TODO: Re-export the Bedrock stuff
// https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BedrockModelCacheConfiguration {
    pub min_total_token: usize,
    pub should_speculate: bool,
    pub max_cache_anchors: usize,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "claude-3-5-sonnet", alias = "claude-3-5-sonnet-latest")]
    Claude3_5Sonnet,
    #[serde(rename = "claude-3-opus", alias = "claude-3-opus-latest")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet", alias = "claude-3-sonnet-latest")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku", alias = "claude-3-haiku-latest")]
    Claude3Haiku,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: usize,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        /// Indicates whether this custom model supports caching.
        cache_configuration: Option<BedrockModelCacheConfiguration>,
        max_output_tokens: Option<u32>,
        default_temperature: Option<f32>,
    },
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        if id.starts_with("claude-3-5-sonnet") {
            Ok(Self::Claude3_5Sonnet)
        } else if id.starts_with("claude-3-opus") {
            Ok(Self::Claude3Opus)
        } else if id.starts_with("claude-3-sonnet") {
            Ok(Self::Claude3Sonnet)
        } else if id.starts_with("claude-3-haiku") {
            Ok(Self::Claude3Haiku)
        } else {
            Err(anyhow!("invalid model id"))
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Model::Claude3_5Sonnet => "claude-3-5-sonnet-latest",
            Model::Claude3Opus => "claude-3-opus-latest",
            Model::Claude3Sonnet => "claude-3-sonnet-latest",
            Model::Claude3Haiku => "claude-3-haiku-latest",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn cache_configuration(&self) -> Option<BedrockModelCacheConfiguration> {
        match self {
            Self::Claude3_5Sonnet | Self::Claude3Haiku => Some(BedrockModelCacheConfiguration {
                min_total_token: 2_048,
                should_speculate: true,
                max_cache_anchors: 4,
            }),
            Self::Custom {
                cache_configuration,
                ..
            } => cache_configuration.clone(),
            _ => None,
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> u32 {
        match self {
            Self::Claude3Opus | Self::Claude3Sonnet | Self::Claude3Haiku => 4_096,
            Self::Claude3_5Sonnet => 8_192,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(4_096),
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
        }
    }
}

pub async fn complete(
    client: &bedrock::Client,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response, BedrockError> {
    todo!()
}

pub async fn stream_completion(
    client: &bedrock::Client,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<Event, BedrockError>>, BedrockError> {

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

pub use bedrock::types::Message;
pub use bedrock::types::ConversationRole;
pub use bedrock::types::ResponseStream;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestContent {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "image")]
    Image {
        source: ImageSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: Role,
    pub content: Vec<ResponseContent>,
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
    pub usage: Usage,
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
