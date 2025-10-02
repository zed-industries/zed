use std::io::{Cursor, Write};
use std::sync::Arc;

use anyhow::Result;
use base64::write::EncoderWriter;
use cloud_llm_client::{CompletionIntent, CompletionMode};
use gpui::{
    App, AppContext as _, DevicePixels, Image, ImageFormat, ObjectFit, SharedString, Size, Task,
    point, px, size,
};
use image::codecs::png::PngEncoder;
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::role::Role;
use crate::{LanguageModelToolUse, LanguageModelToolUseId};

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

    // Parse Self from a JSON object with case-insensitive field names
    pub fn from_json(obj: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        let mut source = None;
        let mut size_obj = None;

        // Find source and size fields (case-insensitive)
        for (k, v) in obj.iter() {
            match k.to_lowercase().as_str() {
                "source" => source = v.as_str(),
                "size" => size_obj = v.as_object(),
                _ => {}
            }
        }

        let source = source?;
        let size_obj = size_obj?;

        let mut width = None;
        let mut height = None;

        // Find width and height in size object (case-insensitive)
        for (k, v) in size_obj.iter() {
            match k.to_lowercase().as_str() {
                "width" => width = v.as_i64().map(|w| w as i32),
                "height" => height = v.as_i64().map(|h| h as i32),
                _ => {}
            }
        }

        Some(Self {
            size: size(DevicePixels(width?), DevicePixels(height?)),
            source: SharedString::from(source.to_string()),
        })
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
                        new_bounds.size.width.into(),
                        new_bounds.size.height.into(),
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

        // 2. Try as object
        if let Some(obj) = value.as_object() {
            // get a JSON field case-insensitively
            fn get_field<'a>(
                obj: &'a serde_json::Map<String, serde_json::Value>,
                field: &str,
            ) -> Option<&'a serde_json::Value> {
                obj.iter()
                    .find(|(k, _)| k.to_lowercase() == field.to_lowercase())
                    .map(|(_, v)| v)
            }

            // Accept wrapped text format: { "type": "text", "text": "..." }
            if let (Some(type_value), Some(text_value)) =
                (get_field(obj, "type"), get_field(obj, "text"))
                && let Some(type_str) = type_value.as_str()
                && type_str.to_lowercase() == "text"
                && let Some(text) = text_value.as_str()
            {
                return Ok(Self::Text(Arc::from(text)));
            }

            // Check for wrapped Text variant: { "text": "..." }
            if let Some((_key, value)) = obj.iter().find(|(k, _)| k.to_lowercase() == "text")
                && obj.len() == 1
            {
                // Only one field, and it's "text" (case-insensitive)
                if let Some(text) = value.as_str() {
                    return Ok(Self::Text(Arc::from(text)));
                }
            }

            // Check for wrapped Image variant: { "image": { "source": "...", "size": ... } }
            if let Some((_key, value)) = obj.iter().find(|(k, _)| k.to_lowercase() == "image")
                && obj.len() == 1
            {
                // Only one field, and it's "image" (case-insensitive)
                // Try to parse the nested image object
                if let Some(image_obj) = value.as_object()
                    && let Some(image) = LanguageModelImage::from_json(image_obj)
                {
                    return Ok(Self::Image(image));
                }
            }

            // Try as direct Image (object with "source" and "size" fields)
            if let Some(image) = LanguageModelImage::from_json(obj) {
                return Ok(Self::Image(image));
            }
        }

        // If none of the variants match, return an error with the problematic JSON
        Err(D::Error::custom(format!(
            "data did not match any variant of LanguageModelToolResultContent. Expected either a string, \
             an object with 'type': 'text', a wrapped variant like {{\"Text\": \"...\"}}, or an image object. Got: {}",
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
        )))
    }
}

impl LanguageModelToolResultContent {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
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

impl From<LanguageModelImage> for LanguageModelToolResultContent {
    fn from(image: LanguageModelImage) -> Self {
        Self::Image(image)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum MessageContent {
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking(String),
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
    pub intent: Option<CompletionIntent>,
    pub mode: Option<CompletionMode>,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub tools: Vec<LanguageModelRequestTool>,
    pub tool_choice: Option<LanguageModelToolChoice>,
    pub stop: Vec<String>,
    pub temperature: Option<f32>,
    pub thinking_allowed: bool,
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
        let json = r#""This is plain text""#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("This is plain text".into())
        );

        let json = r#"{"type": "text", "text": "This is wrapped text"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("This is wrapped text".into())
        );

        let json = r#"{"Type": "TEXT", "TEXT": "Case insensitive"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("Case insensitive".into())
        );

        let json = r#"{"Text": "Wrapped variant"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("Wrapped variant".into())
        );

        let json = r#"{"text": "Lowercase wrapped"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("Lowercase wrapped".into())
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

        // Test wrapped Image variant
        let json = r#"{
            "Image": {
                "source": "wrappedimagedata",
                "size": {
                    "width": 50,
                    "height": 75
                }
            }
        }"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        match result {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "wrappedimagedata");
                assert_eq!(image.size.width.0, 50);
                assert_eq!(image.size.height.0, 75);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test wrapped Image variant with case insensitive
        let json = r#"{
            "image": {
                "Source": "caseinsensitive",
                "SIZE": {
                    "width": 30,
                    "height": 40
                }
            }
        }"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        match result {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "caseinsensitive");
                assert_eq!(image.size.width.0, 30);
                assert_eq!(image.size.height.0, 40);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test that wrapped text with wrong type fails
        let json = r#"{"type": "blahblah", "text": "This should fail"}"#;
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

        // Test direct image with case-insensitive fields
        let json = r#"{
            "SOURCE": "directimage",
            "Size": {
                "width": 200,
                "height": 300
            }
        }"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        match result {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "directimage");
                assert_eq!(image.size.width.0, 200);
                assert_eq!(image.size.height.0, 300);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test that multiple fields prevent wrapped variant interpretation
        let json = r#"{"Text": "not wrapped", "extra": "field"}"#;
        let result: Result<LanguageModelToolResultContent, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Test wrapped text with uppercase TEXT variant
        let json = r#"{"TEXT": "Uppercase variant"}"#;
        let result: LanguageModelToolResultContent = serde_json::from_str(json).unwrap();
        assert_eq!(
            result,
            LanguageModelToolResultContent::Text("Uppercase variant".into())
        );

        // Test that numbers and other JSON values fail gracefully
        let json = r#"123"#;
        let result: Result<LanguageModelToolResultContent, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#"null"#;
        let result: Result<LanguageModelToolResultContent, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#"[1, 2, 3]"#;
        let result: Result<LanguageModelToolResultContent, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
