use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::role::Role;
use crate::{LanguageModelToolUse, LanguageModelToolUseId, SharedString};

/// Dimensions of a `LanguageModelImage`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct LanguageModelImage {
    /// A base64-encoded PNG image.
    pub source: SharedString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
}

impl LanguageModelImage {
    pub fn len(&self) -> usize {
        self.source.len()
    }

    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    pub fn empty() -> Self {
        Self {
            source: "".into(),
            size: None,
        }
    }

    /// Parse Self from a JSON object with case-insensitive field names
    pub fn from_json(obj: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        let mut source = None;
        let mut size_obj = None;

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

        for (k, v) in size_obj.iter() {
            match k.to_lowercase().as_str() {
                "width" => width = v.as_i64().map(|w| w as i32),
                "height" => height = v.as_i64().map(|h| h as i32),
                _ => {}
            }
        }

        Some(Self {
            size: Some(ImageSize {
                width: width?,
                height: height?,
            }),
            source: SharedString::from(source.to_string()),
        })
    }

    pub fn estimate_tokens(&self) -> usize {
        let Some(size) = self.size.as_ref() else {
            return 0;
        };
        let width = size.width.unsigned_abs() as usize;
        let height = size.height.unsigned_abs() as usize;

        // From: https://docs.anthropic.com/en/docs/build-with-claude/vision#calculate-image-costs
        (width * height) / 750
    }

    pub fn to_base64_url(&self) -> String {
        format!("data:image/png;base64,{}", self.source)
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LanguageModelToolResult {
    pub tool_use_id: LanguageModelToolUseId,
    pub tool_name: Arc<str>,
    pub is_error: bool,
    /// The tool output formatted for presenting to the model
    pub content: LanguageModelToolResultContent,
    /// The raw tool output, if available, often for debugging or extra state for replay
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

        // 1. Try as plain string
        if let Ok(text) = serde_json::from_value::<String>(value.clone()) {
            return Ok(Self::Text(Arc::from(text)));
        }

        // 2. Try as object
        if let Some(obj) = value.as_object() {
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
                if let Some(text) = value.as_str() {
                    return Ok(Self::Text(Arc::from(text)));
                }
            }

            // Check for wrapped Image variant: { "image": { "source": "...", "size": ... } }
            if let Some((_key, value)) = obj.iter().find(|(k, _)| k.to_lowercase() == "image")
                && obj.len() == 1
            {
                if let Some(image_obj) = value.as_object()
                    && let Some(image) = LanguageModelImage::from_json(image_obj)
                {
                    return Ok(Self::Image(image));
                }
            }

            // Try as direct Image
            if let Some(image) = LanguageModelImage::from_json(obj) {
                return Ok(Self::Image(image));
            }
        }

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
    pub use_input_streaming: bool,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum LanguageModelToolChoice {
    Auto,
    Any,
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionIntent {
    UserPrompt,
    Subagent,
    ToolResults,
    ThreadSummarization,
    ThreadContextSummarization,
    CreateFile,
    EditFile,
    InlineAssist,
    TerminalInlineAssist,
    GenerateGitCommitMessage,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelRequest {
    pub thread_id: Option<String>,
    pub prompt_id: Option<String>,
    pub intent: Option<CompletionIntent>,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub tools: Vec<LanguageModelRequestTool>,
    pub tool_choice: Option<LanguageModelToolChoice>,
    pub stop: Vec<String>,
    pub temperature: Option<f32>,
    pub thinking_allowed: bool,
    pub thinking_effort: Option<String>,
    pub speed: Option<Speed>,
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    #[default]
    Standard,
    Fast,
}

impl Speed {
    pub fn toggle(self) -> Self {
        match self {
            Speed::Standard => Speed::Fast,
            Speed::Fast => Speed::Standard,
        }
    }
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
        // Test plain string
        let json = serde_json::json!("hello world");
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        assert_eq!(
            content,
            LanguageModelToolResultContent::Text(Arc::from("hello world"))
        );

        // Test wrapped text format: { "type": "text", "text": "..." }
        let json = serde_json::json!({"type": "text", "text": "hello"});
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        assert_eq!(
            content,
            LanguageModelToolResultContent::Text(Arc::from("hello"))
        );

        // Test single-field text object: { "text": "..." }
        let json = serde_json::json!({"text": "hello"});
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        assert_eq!(
            content,
            LanguageModelToolResultContent::Text(Arc::from("hello"))
        );

        // Test case-insensitive type field
        let json = serde_json::json!({"Type": "Text", "Text": "hello"});
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        assert_eq!(
            content,
            LanguageModelToolResultContent::Text(Arc::from("hello"))
        );

        // Test image object
        let json = serde_json::json!({
            "source": "base64encodedimagedata",
            "size": {"width": 100, "height": 200}
        });
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        match content {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "base64encodedimagedata");
                let size = image.size.expect("size");
                assert_eq!(size.width, 100);
                assert_eq!(size.height, 200);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test wrapped image: { "image": { "source": "...", "size": ... } }
        let json = serde_json::json!({
            "image": {
                "source": "wrappedimagedata",
                "size": {"width": 50, "height": 75}
            }
        });
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        match content {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "wrappedimagedata");
                let size = image.size.expect("size");
                assert_eq!(size.width, 50);
                assert_eq!(size.height, 75);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test case insensitive
        let json = serde_json::json!({
            "Source": "caseinsensitive",
            "Size": {"Width": 30, "Height": 40}
        });
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        match content {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "caseinsensitive");
                let size = image.size.expect("size");
                assert_eq!(size.width, 30);
                assert_eq!(size.height, 40);
            }
            _ => panic!("Expected Image variant"),
        }

        // Test direct image object
        let json = serde_json::json!({
            "source": "directimage",
            "size": {"width": 200, "height": 300}
        });
        let content: LanguageModelToolResultContent = serde_json::from_value(json).unwrap();
        match content {
            LanguageModelToolResultContent::Image(image) => {
                assert_eq!(image.source.as_ref(), "directimage");
                let size = image.size.expect("size");
                assert_eq!(size.width, 200);
                assert_eq!(size.height, 300);
            }
            _ => panic!("Expected Image variant"),
        }
    }
}
