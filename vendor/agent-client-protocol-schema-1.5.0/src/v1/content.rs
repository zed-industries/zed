//! Content blocks for representing various types of information in the Agent Client Protocol.
//!
//! This module defines the core content types used throughout the protocol for communication
//! between agents and clients. Content blocks provide a flexible, extensible way to represent
//! text, images, audio, and other resources in prompts, responses, and tool call results.
//!
//! The content block structure is designed to be compatible with the Model Context Protocol (MCP),
//! allowing seamless integration between ACP and MCP-based tools.
//!
//! See: [Content](https://agentclientprotocol.com/protocol/content)

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use crate::{IntoOption, SkipListener};

use super::Meta;

/// Content blocks represent displayable information in the Agent Client Protocol.
///
/// They provide a structured way to handle various types of user-facing content—whether
/// it's text from language models, images for analysis, or embedded resources for context.
///
/// Content blocks appear in:
/// - User prompts sent via `session/prompt`
/// - Language model output streamed through `session/update` notifications
/// - Progress updates and results from tool calls
///
/// This structure is compatible with the Model Context Protocol (MCP), enabling
/// agents to seamlessly forward content from MCP tool outputs without transformation.
///
/// See protocol docs: [Content](https://agentclientprotocol.com/protocol/content)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schemars(extend("discriminator" = {"propertyName": "type"}))]
#[non_exhaustive]
pub enum ContentBlock {
    /// Text content. May be plain text or formatted with Markdown.
    ///
    /// All agents MUST support text content blocks in prompts.
    /// Clients SHOULD render this text as Markdown.
    Text(TextContent),
    /// Images for visual context or analysis.
    ///
    /// Requires the `image` prompt capability when included in prompts.
    Image(ImageContent),
    /// Audio data for transcription or analysis.
    ///
    /// Requires the `audio` prompt capability when included in prompts.
    Audio(AudioContent),
    /// References to resources that the agent can access.
    ///
    /// All agents MUST support resource links in prompts.
    ResourceLink(ResourceLink),
    /// Complete resource contents embedded directly in the message.
    ///
    /// Preferred for including context as it avoids extra round-trips.
    ///
    /// Requires the `embeddedContext` prompt capability when included in prompts.
    Resource(EmbeddedResource),
}

/// Text provided to or from an LLM.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[non_exhaustive]
pub struct TextContent {
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
    /// Text payload carried by this content block.
    pub text: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl TextContent {
    /// Builds [`TextContent`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            annotations: None,
            text: text.into(),
            meta: None,
        }
    }

    /// Sets or clears the optional `annotations` field.
    #[must_use]
    pub fn annotations(mut self, annotations: impl IntoOption<Annotations>) -> Self {
        self.annotations = annotations.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

impl<T: Into<String>> From<T> for ContentBlock {
    fn from(value: T) -> Self {
        Self::Text(TextContent::new(value))
    }
}

/// An image provided to or from an LLM.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ImageContent {
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
    /// Base64-encoded media payload.
    pub data: String,
    /// MIME type describing the encoded media payload.
    pub mime_type: String,
    /// URI associated with this resource or media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub uri: Option<String>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl ImageContent {
    /// Builds [`ImageContent`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(data: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            annotations: None,
            data: data.into(),
            mime_type: mime_type.into(),
            uri: None,
            meta: None,
        }
    }

    /// Sets or clears the optional `annotations` field.
    #[must_use]
    pub fn annotations(mut self, annotations: impl IntoOption<Annotations>) -> Self {
        self.annotations = annotations.into_option();
        self
    }

    /// Sets or clears the optional `uri` field.
    #[must_use]
    pub fn uri(mut self, uri: impl IntoOption<String>) -> Self {
        self.uri = uri.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Audio provided to or from an LLM.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AudioContent {
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
    /// Base64-encoded media payload.
    pub data: String,
    /// MIME type describing the encoded media payload.
    pub mime_type: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl AudioContent {
    /// Builds [`AudioContent`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(data: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            annotations: None,
            data: data.into(),
            mime_type: mime_type.into(),
            meta: None,
        }
    }

    /// Sets or clears the optional `annotations` field.
    #[must_use]
    pub fn annotations(mut self, annotations: impl IntoOption<Annotations>) -> Self {
        self.annotations = annotations.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// The contents of a resource, embedded into a prompt or tool call result.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[non_exhaustive]
pub struct EmbeddedResource {
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
    /// Embedded resource payload, either text or binary data.
    pub resource: EmbeddedResourceResource,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl EmbeddedResource {
    /// Builds [`EmbeddedResource`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(resource: EmbeddedResourceResource) -> Self {
        Self {
            annotations: None,
            resource,
            meta: None,
        }
    }

    /// Sets or clears the optional `annotations` field.
    #[must_use]
    pub fn annotations(mut self, annotations: impl IntoOption<Annotations>) -> Self {
        self.annotations = annotations.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Resource content that can be embedded in a message.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EmbeddedResourceResource {
    /// Text resource contents embedded directly in the message.
    TextResourceContents(TextResourceContents),
    /// Binary resource contents embedded directly in the message.
    BlobResourceContents(BlobResourceContents),
}

/// Text-based resource contents.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TextResourceContents {
    /// MIME type describing the encoded media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<String>,
    /// Text payload carried by this content block.
    pub text: String,
    /// URI associated with this resource or media payload.
    pub uri: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl TextResourceContents {
    /// Builds [`TextResourceContents`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(text: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            mime_type: None,
            text: text.into(),
            uri: uri.into(),
            meta: None,
        }
    }

    /// Sets or clears the optional `mimeType` field.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl IntoOption<String>) -> Self {
        self.mime_type = mime_type.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Binary resource contents.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlobResourceContents {
    /// Base64-encoded bytes for a binary resource payload.
    pub blob: String,
    /// MIME type describing the encoded media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<String>,
    /// URI associated with this resource or media payload.
    pub uri: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl BlobResourceContents {
    /// Builds [`BlobResourceContents`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(blob: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            blob: blob.into(),
            mime_type: None,
            uri: uri.into(),
            meta: None,
        }
    }

    /// Sets or clears the optional `mimeType` field.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl IntoOption<String>) -> Self {
        self.mime_type = mime_type.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// A resource that the server is capable of reading, included in a prompt or tool call result.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResourceLink {
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
    /// Optional human-readable details shown with this protocol object.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// MIME type describing the encoded media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<String>,
    /// Human-readable name shown for this protocol object.
    pub name: String,
    /// Optional size of the linked resource in bytes, if known.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub size: Option<i64>,
    /// Optional display title for end-user UI.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// URI associated with this resource or media payload.
    pub uri: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl ResourceLink {
    /// Builds [`ResourceLink`] with its required content payload; optional annotations and metadata start unset.
    #[must_use]
    pub fn new(name: impl Into<String>, uri: impl Into<String>) -> Self {
        Self {
            annotations: None,
            description: None,
            mime_type: None,
            name: name.into(),
            size: None,
            title: None,
            uri: uri.into(),
            meta: None,
        }
    }

    /// Sets or clears the optional `annotations` field.
    #[must_use]
    pub fn annotations(mut self, annotations: impl IntoOption<Annotations>) -> Self {
        self.annotations = annotations.into_option();
        self
    }

    /// Sets or clears the optional `description` field.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Sets or clears the optional `mimeType` field.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl IntoOption<String>) -> Self {
        self.mime_type = mime_type.into_option();
        self
    }

    /// Sets or clears the optional `size` field.
    #[must_use]
    pub fn size(mut self, size: impl IntoOption<i64>) -> Self {
        self.size = size.into_option();
        self
    }

    /// Sets or clears the optional `title` field.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Optional annotations for the client. The client can use annotations to inform how objects are used or displayed
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Annotations {
    /// Intended recipients for this content, such as the user or assistant.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub audience: Option<Vec<Role>>,
    /// Timestamp indicating when the underlying resource was last modified.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub last_modified: Option<String>,
    /// Relative importance of this content when clients choose what to surface.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub priority: Option<f64>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl Annotations {
    /// Creates annotations with no audience, priority, or timestamp hints set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets or clears the optional `audience` field.
    #[must_use]
    pub fn audience(mut self, audience: impl IntoOption<Vec<Role>>) -> Self {
        self.audience = audience.into_option();
        self
    }

    /// Sets or clears the optional `lastModified` field.
    #[must_use]
    pub fn last_modified(mut self, last_modified: impl IntoOption<String>) -> Self {
        self.last_modified = last_modified.into_option();
        self
    }

    /// Sets or clears the optional `priority` field.
    #[must_use]
    pub fn priority(mut self, priority: impl IntoOption<f64>) -> Self {
        self.priority = priority.into_option();
        self
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// The sender or recipient of messages and data in a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Role {
    /// The assistant side of a conversation.
    Assistant,
    /// The user side of a conversation.
    User,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_content_roundtrip() {
        let content = TextContent::new("hello world");
        let json = serde_json::to_value(&content).unwrap();
        let parsed: TextContent = serde_json::from_value(json).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_text_content_omits_optional_fields() {
        let content = TextContent::new("hello");
        let json = serde_json::to_value(&content).unwrap();
        assert!(!json.as_object().unwrap().contains_key("annotations"));
        assert!(!json.as_object().unwrap().contains_key("meta"));
    }

    #[test]
    fn test_text_content_meta_defaults_on_missing_or_malformed_value() {
        let missing: TextContent = serde_json::from_value(serde_json::json!({
            "text": "hello"
        }))
        .unwrap();
        assert_eq!(missing.meta, None);

        let malformed: TextContent = serde_json::from_value(serde_json::json!({
            "text": "hello",
            "_meta": false
        }))
        .unwrap();
        assert_eq!(malformed.meta, None);
    }

    #[test]
    fn test_text_content_from_string() {
        let block: ContentBlock = "hello".into();
        match block {
            ContentBlock::Text(c) => assert_eq!(c.text, "hello"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_image_content_roundtrip() {
        let content = ImageContent::new("base64data", "image/png");
        let json = serde_json::to_value(&content).unwrap();
        let parsed: ImageContent = serde_json::from_value(json).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_image_content_omits_optional_fields() {
        let content = ImageContent::new("data", "image/png");
        let json = serde_json::to_value(&content).unwrap();
        assert!(!json.as_object().unwrap().contains_key("uri"));
        assert!(!json.as_object().unwrap().contains_key("annotations"));
        assert!(!json.as_object().unwrap().contains_key("meta"));
    }

    #[test]
    fn test_image_content_with_uri() {
        let content = ImageContent::new("data", "image/png").uri("https://example.com/image.png");
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["uri"], "https://example.com/image.png");
    }

    #[test]
    fn test_audio_content_roundtrip() {
        let content = AudioContent::new("base64audio", "audio/mp3");
        let json = serde_json::to_value(&content).unwrap();
        let parsed: AudioContent = serde_json::from_value(json).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_audio_content_omits_optional_fields() {
        let content = AudioContent::new("data", "audio/mp3");
        let json = serde_json::to_value(&content).unwrap();
        assert!(!json.as_object().unwrap().contains_key("annotations"));
        assert!(!json.as_object().unwrap().contains_key("meta"));
    }
}
