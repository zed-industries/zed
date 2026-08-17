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

use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

use derive_more::{Display, From};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use super::Meta;
use crate::{IntoOption, SkipListener};

/// An Internet media type identifying the format of protocol content.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(Arc<str>, String, &str, &mut str, Box<str>, Cow<'_, str>)]
#[non_exhaustive]
pub struct MediaType(pub Arc<str>);

impl MediaType {
    /// Wraps a protocol string as a typed [`MediaType`].
    #[must_use]
    pub fn new(media_type: impl Into<Self>) -> Self {
        media_type.into()
    }
}

impl AsRef<str> for MediaType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&String> for MediaType {
    fn from(media_type: &String) -> Self {
        Self(media_type.as_str().into())
    }
}

macro_rules! impl_media_type_option_conversion {
    ($source:ty) => {
        impl IntoOption<MediaType> for $source {
            fn into_option(self) -> Option<MediaType> {
                Some(self.into())
            }
        }
    };
}

impl_media_type_option_conversion!(Arc<str>);
impl_media_type_option_conversion!(String);
impl_media_type_option_conversion!(&str);
impl_media_type_option_conversion!(&mut str);
impl_media_type_option_conversion!(&String);
impl_media_type_option_conversion!(Box<str>);
impl_media_type_option_conversion!(Cow<'_, str>);

/// Content blocks represent displayable information in the Agent Client Protocol.
///
/// They provide a structured way to handle various types of user-facing content—whether
/// it's text from language models, images for analysis, or embedded resources for context.
///
/// Content blocks appear in:
/// - User prompts sent via `session/prompt`
/// - Language model output reported through `session/update` notifications as
///   message updates or streamed chunks
/// - Progress updates and results from tool calls
///
/// This structure is compatible with the Model Context Protocol (MCP), enabling
/// agents to seamlessly forward content from MCP tool outputs without transformation.
///
/// See protocol docs: [Content](https://agentclientprotocol.com/protocol/content)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
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
    /// Custom or future content block.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this content block type should preserve
    /// the raw payload when storing, replaying, proxying, or forwarding content,
    /// and otherwise ignore it or display it generically.
    #[serde(untagged)]
    Other(OtherContentBlock),
}

/// Custom or future content block payload.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[schemars(inline)]
#[schemars(transform = other_content_block_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherContentBlock {
    /// Custom or future content block type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown content block payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherContentBlock {
    /// Builds [`OtherContentBlock`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherContentBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };

        if is_known_content_block_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known content block `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

fn is_known_content_block_type(type_: &str) -> bool {
    matches!(
        type_,
        "text" | "image" | "audio" | "resource_link" | "resource"
    )
}

fn other_content_block_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        &["text", "image", "audio", "resource_link", "resource"],
    );
}

/// Text provided to or from an LLM.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[non_exhaustive]
pub struct TextContent {
    /// Text payload carried by this content block.
    pub text: String,
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
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
    /// Base64-encoded media payload.
    #[schemars(extend("contentEncoding" = "base64"))]
    pub data: String,
    /// MIME type describing the encoded media payload.
    pub mime_type: MediaType,
    /// URI associated with this resource or media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[schemars(url)]
    #[serde(default)]
    pub uri: Option<String>,
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
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
    pub fn new(data: impl Into<String>, mime_type: impl Into<MediaType>) -> Self {
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
    /// Base64-encoded media payload.
    #[schemars(extend("contentEncoding" = "base64"))]
    pub data: String,
    /// MIME type describing the encoded media payload.
    pub mime_type: MediaType,
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
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
    pub fn new(data: impl Into<String>, mime_type: impl Into<MediaType>) -> Self {
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
    /// Embedded resource payload, either text or binary data.
    pub resource: EmbeddedResourceResource,
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
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
    /// Text payload carried by this content block.
    pub text: String,
    /// URI associated with this resource or media payload.
    #[schemars(url)]
    pub uri: String,
    /// MIME type describing the encoded media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<MediaType>,
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
    pub fn mime_type(mut self, mime_type: impl IntoOption<MediaType>) -> Self {
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
    #[schemars(extend("contentEncoding" = "base64"))]
    pub blob: String,
    /// URI associated with this resource or media payload.
    #[schemars(url)]
    pub uri: String,
    /// MIME type describing the encoded media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<MediaType>,
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
    pub fn mime_type(mut self, mime_type: impl IntoOption<MediaType>) -> Self {
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
    /// Human-readable name shown for this protocol object.
    pub name: String,
    /// URI associated with this resource or media payload.
    #[schemars(url)]
    pub uri: String,
    /// Optional display title for end-user UI.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Optional human-readable details shown with this protocol object.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Optional set of sized icons that the client can display in a user interface.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub icons: Option<Vec<Icon>>,
    /// MIME type describing the encoded media payload.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<MediaType>,
    /// Optional size of the linked resource in bytes, if known.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub size: Option<i64>,
    /// Optional annotations that help clients decide how to display or route this content.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub annotations: Option<Annotations>,
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
            icons: None,
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

    /// Sets or clears the optional `icons` field.
    #[must_use]
    pub fn icons(mut self, icons: impl IntoOption<Vec<Icon>>) -> Self {
        self.icons = icons.into_option();
        self
    }

    /// Sets or clears the optional `mimeType` field.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl IntoOption<MediaType>) -> Self {
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

/// An optionally-sized icon that can be displayed in a user interface.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Icon {
    /// A standard URI pointing to an icon resource.
    #[schemars(url)]
    pub src: String,
    /// Optional MIME type override if the source MIME type is missing or generic.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub mime_type: Option<MediaType>,
    /// Optional array of strings that specify sizes at which the icon can be used.
    /// Each string should be in `WxH` format (e.g., `"48x48"`, `"96x96"`) or
    /// `"any"` for scalable formats like SVG.
    ///
    /// If not provided, the client should assume that the icon can be used at any size.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub sizes: Option<Vec<String>>,
    /// Optional theme this icon is designed for.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub theme: Option<IconTheme>,
}

impl Icon {
    /// Builds [`Icon`] with the required source URI; optional display hints start unset.
    #[must_use]
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            mime_type: None,
            sizes: None,
            theme: None,
        }
    }

    /// Sets or clears the optional `mimeType` field.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl IntoOption<MediaType>) -> Self {
        self.mime_type = mime_type.into_option();
        self
    }

    /// Sets or clears the optional sizes at which the icon can be used.
    #[must_use]
    pub fn sizes(mut self, sizes: impl IntoOption<Vec<String>>) -> Self {
        self.sizes = sizes.into_option();
        self
    }

    /// Sets or clears the optional `theme` field.
    #[must_use]
    pub fn theme(mut self, theme: impl IntoOption<IconTheme>) -> Self {
        self.theme = theme.into_option();
        self
    }
}

/// Theme an icon is designed for.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum IconTheme {
    /// Icon designed for light backgrounds.
    Light,
    /// Icon designed for dark backgrounds.
    Dark,
    /// Custom or future icon theme.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
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
    ///
    /// Must be an RFC 3339 formatted string (e.g., "2025-01-12T15:00:58Z").
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "format" = "date-time"))]
    #[serde(default)]
    pub last_modified: Option<String>,
    /// Relative importance of this content when clients choose what to surface.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[schemars(range(min = 0, max = 1))]
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
    /// Custom or future role.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
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
    fn role_preserves_unknown_variant() {
        let role: Role = serde_json::from_str("\"critic\"").unwrap();
        assert_eq!(role, Role::Other("critic".to_string()));
        assert_eq!(serde_json::to_value(&role).unwrap(), "critic");
    }

    #[test]
    fn icon_theme_preserves_unknown_variant() {
        let theme: IconTheme = serde_json::from_str("\"contrast\"").unwrap();
        assert_eq!(theme, IconTheme::Other("contrast".to_string()));
        assert_eq!(serde_json::to_value(&theme).unwrap(), "contrast");
    }

    #[test]
    fn content_block_preserves_unknown_variant() {
        let block: ContentBlock = serde_json::from_value(serde_json::json!({
            "type": "_widget",
            "title": "Status",
            "state": {"ok": true}
        }))
        .unwrap();

        let ContentBlock::Other(unknown) = block else {
            panic!("expected unknown content block");
        };

        assert_eq!(unknown.type_, "_widget");
        assert_eq!(
            unknown.fields.get("title"),
            Some(&serde_json::json!("Status"))
        );
        assert_eq!(
            serde_json::to_value(ContentBlock::Other(unknown)).unwrap(),
            serde_json::json!({
                "type": "_widget",
                "title": "Status",
                "state": {"ok": true}
            })
        );
    }

    #[test]
    fn content_block_does_not_hide_malformed_known_variant() {
        assert!(
            serde_json::from_value::<ContentBlock>(serde_json::json!({
                "type": "text"
            }))
            .is_err()
        );
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

    #[test]
    fn resource_link_icons_roundtrip() {
        let icon = Icon::new("https://example.com/icon.png")
            .mime_type("image/png")
            .sizes(vec!["48x48".to_string(), "any".to_string()])
            .theme(IconTheme::Dark);
        let link = ResourceLink::new("Example", "file:///example.txt").icons(vec![icon]);

        let json = serde_json::to_value(&link).unwrap();
        assert_eq!(json["icons"][0]["src"], "https://example.com/icon.png");
        assert_eq!(json["icons"][0]["mimeType"], "image/png");
        assert_eq!(json["icons"][0]["sizes"][0], "48x48");
        assert_eq!(json["icons"][0]["theme"], "dark");

        let parsed: ResourceLink = serde_json::from_value(json).unwrap();
        assert_eq!(link, parsed);
    }

    #[test]
    fn annotations_priority_schema_matches_mcp_bounds() {
        let schema = schemars::schema_for!(Annotations);
        let json = serde_json::to_value(schema).unwrap();

        assert_eq!(json["properties"]["priority"]["minimum"], 0);
        assert_eq!(json["properties"]["priority"]["maximum"], 1);
        assert_eq!(json["properties"]["lastModified"]["format"], "date-time");
    }

    #[test]
    fn content_schema_uses_standard_string_annotations() {
        let image = serde_json::to_value(schemars::schema_for!(ImageContent)).unwrap();
        assert_eq!(image["properties"]["data"]["contentEncoding"], "base64");
        assert!(image["properties"]["data"].get("format").is_none());
        assert_eq!(image["properties"]["uri"]["format"], "uri");

        let audio = serde_json::to_value(schemars::schema_for!(AudioContent)).unwrap();
        assert_eq!(audio["properties"]["data"]["contentEncoding"], "base64");
        assert!(audio["properties"]["data"].get("format").is_none());

        let text_resource =
            serde_json::to_value(schemars::schema_for!(TextResourceContents)).unwrap();
        assert_eq!(text_resource["properties"]["uri"]["format"], "uri");

        let blob_resource =
            serde_json::to_value(schemars::schema_for!(BlobResourceContents)).unwrap();
        assert_eq!(
            blob_resource["properties"]["blob"]["contentEncoding"],
            "base64"
        );
        assert!(blob_resource["properties"]["blob"].get("format").is_none());
        assert_eq!(blob_resource["properties"]["uri"]["format"], "uri");

        let resource_link = serde_json::to_value(schemars::schema_for!(ResourceLink)).unwrap();
        assert_eq!(resource_link["properties"]["uri"]["format"], "uri");

        let icon = serde_json::to_value(schemars::schema_for!(Icon)).unwrap();
        assert_eq!(icon["properties"]["src"]["format"], "uri");
        assert_eq!(icon["properties"]["sizes"]["items"]["type"], "string");
        assert!(
            icon["properties"]["sizes"]["items"]
                .get("pattern")
                .is_none()
        );
    }
}
