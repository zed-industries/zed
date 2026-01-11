use std::io::{Cursor, Write};
use std::sync::Arc;

use anyhow::Result;
use base64::write::EncoderWriter;
use cloud_llm_client::{CompletionIntent, CompletionMode};
use gpui::{
    App, AppContext as _, DevicePixels, Image, ImageFormat, ObjectFit, SharedString, Size, Task,
    point, px, size,
};
use image::GenericImageView as _;
use image::codecs::png::PngEncoder;
use serde::{Deserialize, Serialize};
use util::ResultExt;

use crate::role::Role;
use crate::{LanguageModelToolUse, LanguageModelToolUseId};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct LanguageModelImage {
    /// A base64-encoded PNG image.
    pub source: SharedString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<Size<DevicePixels>>,
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
            size: Some(size(DevicePixels(width?), DevicePixels(height?))),
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
const ANTHROPIC_SIZE_LIMIT: f32 = 1568.;

/// Default per-image hard limit (in bytes) for the encoded image payload we send upstream.
///
/// NOTE: `LanguageModelImage.source` is base64-encoded PNG bytes (without the `data:` prefix).
/// This limit is enforced on the encoded PNG bytes *before* base64 encoding.
const DEFAULT_IMAGE_MAX_BYTES: usize = 5 * 1024 * 1024;

/// Conservative cap on how many times we'll attempt to shrink/re-encode an image to fit
/// `DEFAULT_IMAGE_MAX_BYTES`.
const MAX_IMAGE_DOWNSCALE_PASSES: usize = 8;

impl LanguageModelImage {
    pub fn empty() -> Self {
        Self {
            source: "".into(),
            size: None,
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
                ImageFormat::Bmp => image::codecs::bmp::BmpDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                ImageFormat::Tiff => image::codecs::tiff::TiffDecoder::new(image_bytes)
                    .and_then(image::DynamicImage::from_decoder),
                _ => return None,
            }
            .log_err()?;

            let width = dynamic_image.width();
            let height = dynamic_image.height();
            let image_size = size(DevicePixels(width as i32), DevicePixels(height as i32));

            // First apply any provider-specific dimension constraints we know about (Anthropic).
            let mut processed_image = if image_size.width.0 > ANTHROPIC_SIZE_LIMIT as i32
                || image_size.height.0 > ANTHROPIC_SIZE_LIMIT as i32
            {
                let new_bounds = ObjectFit::ScaleDown.get_bounds(
                    gpui::Bounds {
                        origin: point(px(0.0), px(0.0)),
                        size: size(px(ANTHROPIC_SIZE_LIMIT), px(ANTHROPIC_SIZE_LIMIT)),
                    },
                    image_size,
                );
                dynamic_image.resize(
                    new_bounds.size.width.into(),
                    new_bounds.size.height.into(),
                    image::imageops::FilterType::Triangle,
                )
            } else {
                dynamic_image
            };

            // Then enforce a default per-image size cap on the encoded PNG bytes.
            //
            // We always send PNG bytes (either original PNG bytes, or re-encoded PNG) base64'd.
            // The upstream provider limit we want to respect is effectively on the binary image
            // payload size, so we enforce against the encoded PNG bytes before base64 encoding.
            let mut encoded_png = encode_png_bytes(&processed_image).log_err()?;
            for _pass in 0..MAX_IMAGE_DOWNSCALE_PASSES {
                if encoded_png.len() <= DEFAULT_IMAGE_MAX_BYTES {
                    break;
                }

                // Scale down geometrically to converge quickly. We don't know the final PNG size
                // as a function of pixels, so we iteratively shrink.
                let (w, h) = processed_image.dimensions();
                if w <= 1 || h <= 1 {
                    break;
                }

                // Shrink by ~15% each pass (0.85). This is a compromise between speed and
                // preserving image detail.
                let new_w = ((w as f32) * 0.85).round().max(1.0) as u32;
                let new_h = ((h as f32) * 0.85).round().max(1.0) as u32;

                processed_image =
                    processed_image.resize(new_w, new_h, image::imageops::FilterType::Triangle);
                encoded_png = encode_png_bytes(&processed_image).log_err()?;
            }

            if encoded_png.len() > DEFAULT_IMAGE_MAX_BYTES {
                // Still too large after multiple passes; treat as non-convertible for now.
                // (Provider-specific handling can be introduced later.)
                return None;
            }

            // Now base64 encode the PNG bytes.
            let base64_image = encode_bytes_as_base64(encoded_png.as_slice()).log_err()?;

            // SAFETY: The base64 encoder should not produce non-UTF8.
            let source = unsafe { String::from_utf8_unchecked(base64_image) };

            Some(LanguageModelImage {
                size: Some(image_size),
                source: source.into(),
            })
        })
    }

    pub fn estimate_tokens(&self) -> usize {
        let Some(size) = self.size.as_ref() else {
            return 0;
        };
        let width = size.width.0.unsigned_abs() as usize;
        let height = size.height.0.unsigned_abs() as usize;

        // From: https://docs.anthropic.com/en/docs/build-with-claude/vision#calculate-image-costs
        // Note that are a lot of conditions on Anthropic's API, and OpenAI doesn't use this,
        // so this method is more of a rough guess.
        (width * height) / 750
    }

    pub fn to_base64_url(&self) -> String {
        format!("data:image/png;base64,{}", self.source)
    }
}

fn encode_png_bytes(image: &image::DynamicImage) -> Result<Vec<u8>> {
    let mut png = Vec::new();
    image.write_with_encoder(PngEncoder::new(&mut png))?;
    Ok(png)
}

fn encode_bytes_as_base64(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut base64_image = Vec::new();
    {
        let mut base64_encoder = EncoderWriter::new(
            Cursor::new(&mut base64_image),
            &base64::engine::general_purpose::STANDARD,
        );
        base64_encoder.write_all(bytes)?;
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<serde_json::Value>,
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
    use base64::Engine as _;
    use gpui::TestAppContext;
    use image::ImageDecoder as _;

    fn base64_to_png_bytes(base64_png: &str) -> Vec<u8> {
        base64::engine::general_purpose::STANDARD
            .decode(base64_png.as_bytes())
            .expect("base64 should decode")
    }

    fn png_dimensions(png_bytes: &[u8]) -> (u32, u32) {
        let decoder =
            image::codecs::png::PngDecoder::new(Cursor::new(png_bytes)).expect("png should decode");
        decoder.dimensions()
    }

    fn make_noisy_png_bytes(width: u32, height: u32) -> Vec<u8> {
        // Create an RGBA image with per-pixel variance to avoid PNG compressing too well.
        let mut img = image::RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let r = ((x ^ y) & 0xFF) as u8;
                let g = ((x.wrapping_mul(31) ^ y.wrapping_mul(17)) & 0xFF) as u8;
                let b = ((x.wrapping_mul(131) ^ y.wrapping_mul(7)) & 0xFF) as u8;
                img.put_pixel(x, y, image::Rgba([r, g, b, 0xFF]));
            }
        }

        let mut out = Vec::new();
        image::DynamicImage::ImageRgba8(img)
            .write_with_encoder(PngEncoder::new(&mut out))
            .expect("png encoding should succeed");
        out
    }

    #[gpui::test]
    async fn test_from_image_downscales_to_default_5mb_limit(cx: &mut TestAppContext) {
        // Pick a size that reliably produces a PNG > 5MB when filled with noise.
        // If this fails (image is too small), bump dimensions.
        let original_png = make_noisy_png_bytes(4096, 4096);
        assert!(
            original_png.len() > DEFAULT_IMAGE_MAX_BYTES,
            "precondition failed: noisy PNG must exceed DEFAULT_IMAGE_MAX_BYTES"
        );

        let image = gpui::Image::from_bytes(ImageFormat::Png, original_png);
        let lm_image = cx
            .update(|cx| LanguageModelImage::from_image(Arc::new(image), cx))
            .await
            .expect("image conversion should succeed");

        let encoded_png = base64_to_png_bytes(lm_image.source.as_ref());
        assert!(
            encoded_png.len() <= DEFAULT_IMAGE_MAX_BYTES,
            "expected encoded PNG <= DEFAULT_IMAGE_MAX_BYTES, got {} bytes",
            encoded_png.len()
        );

        // Ensure we actually downscaled in pixels (not just re-encoded).
        let (w, h) = png_dimensions(&encoded_png);
        assert!(
            w < 4096 || h < 4096,
            "expected image to be downscaled in at least one dimension; got {w}x{h}"
        );
    }

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
                let size = image.size.expect("size");
                assert_eq!(size.width.0, 100);
                assert_eq!(size.height.0, 200);
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
                let size = image.size.expect("size");
                assert_eq!(size.width.0, 50);
                assert_eq!(size.height.0, 75);
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
                let size = image.size.expect("size");
                assert_eq!(size.width.0, 30);
                assert_eq!(size.height.0, 40);
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
                let size = image.size.expect("size");
                assert_eq!(size.width.0, 200);
                assert_eq!(size.height.0, 300);
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
