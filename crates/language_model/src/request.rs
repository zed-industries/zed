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
    pub size: Size<DevicePixels>,
}

impl LanguageModelImage {
    pub fn len(&self) -> usize {
        self.source.len()
    }

    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }
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

    pub fn to_base64_url(&self) -> String {
        format!("data:image/png;base64,{}", self.source)
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
    pub content: LanguageModelToolResultContent,
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
pub enum LanguageModelToolResultContent {
    Text(Arc<str>),
    Image(LanguageModelImage),
}

impl<'de> Deserialize<'de> for LanguageModelToolResultContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let value = serde_json::Value::deserialize(deserializer)?;

        // Models can provide these responses in several styles. Try each in order.

        // 1. Try as plain string
        if let Ok(text) = serde_json::from_value::<String>(value.clone()) {
            return Ok(Self::Text(Arc::from(text)));
        }

        // 2. Try as object with fields including "type": "text" as well as "text": "..."
        if let Some(obj) = value.as_object() {
            if let (Some(type_value), Some(text_value)) = (obj.get("type"), obj.get("text")) {
                if type_value.as_str() == Some("text") {
                    if let Some(text) = text_value.as_str() {
                        return Ok(Self::Text(Arc::from(text)));
                    }
                }
            }

            // 3. Try as Image (object with "source" and "size" fields)
            if let Ok(image) = serde_json::from_value::<LanguageModelImage>(value.clone()) {
                return Ok(Self::Image(image));
            }
        }

        // If none of the variants match, return an error with the problematic JSON
        Err(D::Error::custom(format!(
            "Unable to deserialize LanguageModelToolResultContent from JSON. Expected the JSON to be either a string, an object with \"type\" and \"text\" fields, or an image object with \"source\" and \"size\" fields. Got: {}",
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
        )))
    }
}

impl LanguageModelToolResultContent {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(&text),
            Self::Image(_) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Text(text) => text.chars().all(|c| c.is_whitespace()),
            Self::Image(_) => false,
        }
    }
}

impl From<&str> for LanguageModelToolResultContent {
    fn from(value: &str) -> Self {
        Self::Text(Arc::from(value))
    }
}

impl From<String> for LanguageModelToolResultContent {
    fn from(value: String) -> Self {
        Self::Text(Arc::from(value))
    }
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

impl MessageContent {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            MessageContent::Text(text) => Some(text.as_str()),
            MessageContent::Thinking { text, .. } => Some(text.as_str()),
            MessageContent::RedactedThinking(_) => None,
            MessageContent::ToolResult(tool_result) => tool_result.content.to_str(),
            MessageContent::ToolUse(_) | MessageContent::Image(_) => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MessageContent::Text(text) => text.chars().all(|c| c.is_whitespace()),
            MessageContent::Thinking { text, .. } => text.chars().all(|c| c.is_whitespace()),
            MessageContent::ToolResult(tool_result) => tool_result.content.is_empty(),
            MessageContent::RedactedThinking(_)
            | MessageContent::ToolUse(_)
            | MessageContent::Image(_) => false,
        }
    }
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
        for string in self.content.iter().filter_map(|content| content.to_str()) {
            buffer.push_str(string);
        }

        buffer
    }

    pub fn contents_empty(&self) -> bool {
        self.content.iter().all(|content| content.is_empty())
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelRequestTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum LanguageModelToolChoice {
    Auto,
    Any,
    None,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelRequest {
    pub thread_id: Option<String>,
    pub prompt_id: Option<String>,
    pub mode: Option<CompletionMode>,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub tools: Vec<LanguageModelRequestTool>,
    pub tool_choice: Option<LanguageModelToolChoice>,
    pub stop: Vec<String>,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_model_tool_result_content_deserialization() {
        // Test plain string deserialization
        let json = r#""This is plain text""#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("This is plain text".into())
        );

        // Test wrapped text with type "text"
        let json = r#"{"type": "text", "text": "This is wrapped text"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("This is wrapped text".into())
        );

        // Test image deserialization
        let json = r#"{
            "source": "base64encodedimagedata",
            "size": {
                "width": 100,
                "height": 200
            }
        }"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        match result {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "base64encodedimagedata");
                assert_eq!(image.size.width.0, 100);
                assert_eq!(image.size.height.0, 200);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test that wrapped text with wrong type fails
        let json = r#"{"type": "foobarbaz", "text": "This should fail"}"#;
        let result: Result<LanguageModelToolResultContent, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Test that malformed JSON fails
        let json = r#"{"invalid": "structure"}"#;
        let result: Result<LanguageModelToolResultContent, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Test edge cases
        let json = r#""""#; // Empty string
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(result, LanguageModelToolResultContent::Text("".into()));

        // Test with extra fields in wrapped text (should be ignored)
        let json = r#"{"type": "text", "text": "Hello", "extra": "field"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(result, LanguageModelToolResultContent::Text("Hello".into()));
    }

    #[test]
    fn test_language_model_tool_result_content_methods() {
        // Test to_str()
        let text_content = LanguageModelToolResultContent::Text("Hello".into());
        assert_eq!(text_content.to_str(), Some("Hello"));

        let image_content = LanguageModelToolResultContent::Image(LanguageModelImage::empty());
        assert_eq!(image_content.to_str(), None);

        // Test is_empty()
        let empty_text = LanguageModelToolResultContent::Text("   \n\t  ".into());
        assert!(empty_text.is_empty());

        let non_empty_text = LanguageModelToolResultContent::Text("Hello".into());
        assert!(!non_empty_text.is_empty());

        let image = LanguageModelToolResultContent::Image(LanguageModelImage::empty());
        assert!(!image.is_empty());
    }

    #[test]
    fn test_language_model_tool_result_serialization() {
        // Note: LanguageModelToolResultContent has asymmetric serialization/deserialization
        // It serializes as {"Text": "..."} but deserializes from plain strings or wrapped format

        // Test deserialization of a LanguageModelToolResult with plain text content
        let json = r#"{
            "tool_use_id": "test-id",
            "tool_name": "test-tool",
            "is_error": false,
            "content": "Result text",
            "output": {"key": "value"}
        }"#;

        let deserialized: LanguageModelToolResult = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.tool_use_id.to_string(), "test-id");
        assert_eq!(deserialized.tool_name.as_ref(), "test-tool");
        assert_eq!(deserialized.is_error, false);
        assert_eq!(
            deserialized.content,
            LanguageModelToolResultContent::Text("Result text".into())
        );
        assert_eq!(
            deserialized.output,
            Some(serde_json::json!({"key": "value"}))
        );

        // Test deserialization with wrapped text format
        let json_wrapped = r#"{
            "tool_use_id": "test-id2",
            "tool_name": "test-tool2",
            "is_error": true,
            "content": {"type": "text", "text": "Wrapped result"},
            "output": null
        }"#;

        let deserialized: LanguageModelToolResult = serde_json::from_str(json_wrapped).unwrap();
        assert_eq!(
            deserialized.content,
            LanguageModelToolResultContent::Text("Wrapped result".into())
        );
        assert_eq!(deserialized.is_error, true);
        assert_eq!(deserialized.output, None);
    }
}
