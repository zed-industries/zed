use std::io::{Cursor, Write};
use std::sync::Arc;

use crate::role::Role;
use crate::{LanguageModelToolUse, LanguageModelToolUseId};
use anyhow::Result;
use base64::write::EncoderWriter;
use gpui::{
    App, AppContext as _, DevicePixels, Image, ImageFormat, ObjectFit, SharedString, Size, Task,
    point, px, size,
};
use image::codecs::png::PngEncoder;
use serde::{Deserialize, Serialize};
use util::ResultExt;
use zed_llm_client::CompletionMode;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct LanguageModelImage {
    /// A base64-encoded PNG image.
    pub source: SharedString,
    size: Size<DevicePixels>,
}

impl std::fmt::Debug for LanguageModelImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageModelImage")
            .field("source", &format!("<{} bytes>", self.source.len()))
            .field("size", &self.size)
            .finish()
    }
}

/// Anthropic wants uploaded images to be smaller than this in both dimensions.
const ANTHROPIC_SIZE_LIMT: f32 = 1568.;

impl LanguageModelImage {
    pub fn empty() -> Self {
        Self {
            source: "".into(),
            size: size(DevicePixels(0), DevicePixels(0)),
        }
    }

    pub fn from_image(data: Arc<Image>, cx: &mut App) -> Task<Option<Self>> {
        cx.background_spawn(async move {
            let image_bytes = Cursor::new(data.bytes());
            let dynamic_image = match data.format() {
                ImageFormat::Png => image::codecs::png::PngDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Jpeg => image::codecs::jpeg::JpegDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Webp => image::codecs::webp::WebPDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Gif => image::codecs::gif::GifDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                _ => return None,
            }
            .log_err()?;

            let width = dynamic_image.width();
            let height = dynamic_image.height();
            let image_size = size(DevicePixels(width as i32), DevicePixels(height as i32));

            let base64_image = {
                if image_size.width.0 > ANTHROPIC_SIZE_LIMT as i32
                    || image_size.height.0 > ANTHROPIC_SIZE_LIMT as i32
                {
                    let new_bounds = ObjectFit::ScaleDown.get_bounds(
                        gpui::Bounds {
                            origin: point(px(0.0), px(0.0)),
                            size: size(px(ANTHROPIC_SIZE_LIMT), px(ANTHROPIC_SIZE_LIMT)),
                        },
                        image_size,
                    );
                    let resized_image = dynamic_image.resize(
                        new_bounds.size.width.0 as u32,
                        new_bounds.size.height.0 as u32,
                        image::imageops::FilterType::Triangle,
                    );

                    encode_as_base64(data, resized_image)
                } else {
                    encode_as_base64(data, dynamic_image)
                }
            }
            .log_err()?;

            // SAFETY: The base64 encoder should not produce non-UTF8.
            let source = unsafe { String::from_utf8_unchecked(base64_image) };

            Some(LanguageModelImage {
                size: image_size,
                source: source.into(),
            })
        })
    }

    pub fn estimate_tokens(&self) -> usize {
        let width = self.size.width.0.unsigned_abs() as usize;
        let height = self.size.height.0.unsigned_abs() as usize;

        // From: https://docs.anthropic.com/en/docs/build-with-claude/vision#calculate-image-costs
        // Note that are a lot of conditions on Anthropic's API, and OpenAI doesn't use this,
        // so this method is more of a rough guess.
        (width * height) / 750
    }
}

fn encode_as_base64(data: Arc<Image>, image: image::DynamicImage) -> Result<Vec<u8>> {
    let mut base64_image = Vec::new();
    {
        let mut base64_encoder = EncoderWriter::new(
            Cursor::new(&mut base64_image),
            &base64::engine::general_purpose::STANDARD,
        );
        if data.format() == ImageFormat::Png {
            base64_encoder.write_all(data.bytes())?;
        } else {
            let mut png = Vec::new();
            image.write_with_encoder(PngEncoder::new(&mut png))?;

            base64_encoder.write_all(png.as_slice())?;
        }
    }
    Ok(base64_image)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LanguageModelToolResult {
    pub tool_use_id: LanguageModelToolUseId,
    pub tool_name: Arc<str>,
    pub is_error: bool,
    pub content: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum MessageContent {
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking(Vec<u8>),
    Image(LanguageModelImage),
    ToolUse(LanguageModelToolUse),
    ToolResult(LanguageModelToolResult),
}

impl From<String> for MessageContent {
    fn from(value: String) -> Self {
        MessageContent::Text(value)
    }
}

impl From<&str> for MessageContent {
    fn from(value: &str) -> Self {
        MessageContent::Text(value.to_string())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
    pub cache: bool,
}

impl LanguageModelRequestMessage {
    pub fn string_contents(&self) -> String {
        let mut buffer = String::new();
        for string in self.content.iter().filter_map(|content| match content {
            MessageContent::Text(text) => Some(text.as_str()),
            MessageContent::Thinking { text, .. } => Some(text.as_str()),
            MessageContent::RedactedThinking(_) => None,
            MessageContent::ToolResult(tool_result) => Some(tool_result.content.as_ref()),
            MessageContent::ToolUse(_) | MessageContent::Image(_) => None,
        }) {
            buffer.push_str(string);
        }

        buffer
    }

    pub fn contents_empty(&self) -> bool {
        self.content.iter().all(|content| match content {
            MessageContent::Text(text) => text.chars().all(|c| c.is_whitespace()),
            MessageContent::Thinking { text, .. } => text.chars().all(|c| c.is_whitespace()),
            MessageContent::ToolResult(tool_result) => {
                tool_result.content.chars().all(|c| c.is_whitespace())
            }
            MessageContent::RedactedThinking(_)
            | MessageContent::ToolUse(_)
            | MessageContent::Image(_) => false,
        })
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelRequestTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelRequest {
    pub thread_id: Option<String>,
    pub prompt_id: Option<String>,
    pub mode: Option<CompletionMode>,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub tools: Vec<LanguageModelRequestTool>,
    pub stop: Vec<String>,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}
