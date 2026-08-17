//! Elicitation types for structured user input.
//!
//! **UNSTABLE**: This module is not part of the spec yet, and may be removed or changed at any point.
//!
//! This module defines the types used for agent-initiated elicitation,
//! where the agent requests structured input from the user via forms or URLs.

use std::{collections::BTreeMap, sync::Arc};

use derive_more::{Display, From};
use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use super::{
    ELICITATION_COMPLETE_NOTIFICATION, ELICITATION_CREATE_METHOD_NAME, Meta, RequestId, SessionId,
    ToolCallId,
};
use crate::IntoOption;
use crate::SkipListener;

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Unique identifier for an elicitation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct ElicitationId(pub Arc<str>);

impl ElicitationId {
    /// Wraps a protocol string as a typed [`ElicitationId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// String format types for string properties in elicitation schemas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum StringFormat {
    /// Email address format.
    Email,
    /// URI format.
    Uri,
    /// Date format (YYYY-MM-DD).
    Date,
    /// Date-time format (RFC 3339).
    DateTime,
    /// Custom or future string format.
    ///
    /// Unknown formats are preserved. Implementations that do not understand a
    /// format should treat it as an annotation rather than rejecting the schema.
    #[serde(untagged)]
    Other(String),
}

/// Type discriminator for elicitation schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ElicitationSchemaType {
    /// Object schema type.
    #[default]
    Object,
}

/// A titled enum option with a const value, human-readable title, and optional description.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[non_exhaustive]
pub struct EnumOption {
    /// The constant value for this option.
    #[serde(rename = "const")]
    pub value: String,
    /// Human-readable title for this option.
    pub title: String,
    /// Human-readable description.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
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

impl EnumOption {
    /// Create a new enum option.
    #[must_use]
    pub fn new(value: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            description: None,
            meta: None,
        }
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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

/// Schema for string properties in an elicitation form.
///
/// When `enum` or `oneOf` is set, this represents a single-select enum
/// with `"type": "string"`.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StringPropertySchema {
    /// Optional title for the property.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Human-readable description.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Minimum string length.
    #[serde(default)]
    pub min_length: Option<u32>,
    /// Maximum string length.
    #[serde(default)]
    pub max_length: Option<u32>,
    /// Pattern the string must match.
    #[schemars(extend("format" = "regex"))]
    #[serde(default)]
    pub pattern: Option<String>,
    /// String format.
    #[serde(default)]
    pub format: Option<StringFormat>,
    /// Default value.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub default: Option<String>,
    /// Enum values for untitled single-select enums.
    /// Must contain at least one value when present.
    #[schemars(length(min = 1))]
    #[serde(default)]
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    /// Titled enum options for titled single-select enums.
    /// Must contain at least one option when present.
    #[schemars(length(min = 1))]
    #[serde(default)]
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<EnumOption>>,
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

impl StringPropertySchema {
    /// Create a new string property schema.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an email string property schema.
    #[must_use]
    pub fn email() -> Self {
        Self {
            format: Some(StringFormat::Email),
            ..Default::default()
        }
    }

    /// Create a URI string property schema.
    #[must_use]
    pub fn uri() -> Self {
        Self {
            format: Some(StringFormat::Uri),
            ..Default::default()
        }
    }

    /// Create a date string property schema.
    #[must_use]
    pub fn date() -> Self {
        Self {
            format: Some(StringFormat::Date),
            ..Default::default()
        }
    }

    /// Create a date-time string property schema.
    #[must_use]
    pub fn date_time() -> Self {
        Self {
            format: Some(StringFormat::DateTime),
            ..Default::default()
        }
    }

    /// Optional title for the property.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Minimum string length.
    #[must_use]
    pub fn min_length(mut self, min_length: impl IntoOption<u32>) -> Self {
        self.min_length = min_length.into_option();
        self
    }

    /// Maximum string length.
    #[must_use]
    pub fn max_length(mut self, max_length: impl IntoOption<u32>) -> Self {
        self.max_length = max_length.into_option();
        self
    }

    /// Pattern the string must match.
    #[must_use]
    pub fn pattern(mut self, pattern: impl IntoOption<String>) -> Self {
        self.pattern = pattern.into_option();
        self
    }

    /// String format.
    #[must_use]
    pub fn format(mut self, format: impl IntoOption<StringFormat>) -> Self {
        self.format = format.into_option();
        self
    }

    /// Default value.
    #[must_use]
    pub fn default_value(mut self, default: impl IntoOption<String>) -> Self {
        self.default = default.into_option();
        self
    }

    /// Enum values for untitled single-select enums.
    #[must_use]
    pub fn enum_values(mut self, enum_values: impl IntoOption<Vec<String>>) -> Self {
        self.enum_values = enum_values.into_option();
        self
    }

    /// Titled enum options for titled single-select enums.
    #[must_use]
    pub fn one_of(mut self, one_of: impl IntoOption<Vec<EnumOption>>) -> Self {
        self.one_of = one_of.into_option();
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

/// Schema for number (floating-point) properties in an elicitation form.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NumberPropertySchema {
    /// Optional title for the property.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Human-readable description.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Minimum value (inclusive).
    #[serde(default)]
    pub minimum: Option<f64>,
    /// Maximum value (inclusive).
    #[serde(default)]
    pub maximum: Option<f64>,
    /// Default value.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub default: Option<f64>,
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

impl NumberPropertySchema {
    /// Create a new number property schema.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Optional title for the property.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Minimum value (inclusive).
    #[must_use]
    pub fn minimum(mut self, minimum: impl IntoOption<f64>) -> Self {
        self.minimum = minimum.into_option();
        self
    }

    /// Maximum value (inclusive).
    #[must_use]
    pub fn maximum(mut self, maximum: impl IntoOption<f64>) -> Self {
        self.maximum = maximum.into_option();
        self
    }

    /// Default value.
    #[must_use]
    pub fn default_value(mut self, default: impl IntoOption<f64>) -> Self {
        self.default = default.into_option();
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

/// Schema for integer properties in an elicitation form.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IntegerPropertySchema {
    /// Optional title for the property.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Human-readable description.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Minimum value (inclusive).
    #[serde(default)]
    pub minimum: Option<i64>,
    /// Maximum value (inclusive).
    #[serde(default)]
    pub maximum: Option<i64>,
    /// Default value.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub default: Option<i64>,
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

impl IntegerPropertySchema {
    /// Create a new integer property schema.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Optional title for the property.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Minimum value (inclusive).
    #[must_use]
    pub fn minimum(mut self, minimum: impl IntoOption<i64>) -> Self {
        self.minimum = minimum.into_option();
        self
    }

    /// Maximum value (inclusive).
    #[must_use]
    pub fn maximum(mut self, maximum: impl IntoOption<i64>) -> Self {
        self.maximum = maximum.into_option();
        self
    }

    /// Default value.
    #[must_use]
    pub fn default_value(mut self, default: impl IntoOption<i64>) -> Self {
        self.default = default.into_option();
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

/// Schema for boolean properties in an elicitation form.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BooleanPropertySchema {
    /// Optional title for the property.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Human-readable description.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Default value.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub default: Option<bool>,
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

impl BooleanPropertySchema {
    /// Create a new boolean property schema.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Optional title for the property.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Default value.
    #[must_use]
    pub fn default_value(mut self, default: impl IntoOption<bool>) -> Self {
        self.default = default.into_option();
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

/// String item schema for multi-select enum properties.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[non_exhaustive]
pub struct StringMultiSelectItems {
    /// Allowed enum values. Must contain at least one value.
    #[schemars(length(min = 1))]
    #[serde(rename = "enum")]
    pub values: Vec<String>,
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

impl StringMultiSelectItems {
    /// Create new string multi-select items.
    #[must_use]
    pub fn new(values: Vec<String>) -> Self {
        Self { values, meta: None }
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

/// Items definition for titled multi-select enum properties.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[non_exhaustive]
pub struct TitledMultiSelectItems {
    /// Titled enum options. Must contain at least one option.
    #[schemars(length(min = 1))]
    #[serde(rename = "anyOf")]
    pub options: Vec<EnumOption>,
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

impl TitledMultiSelectItems {
    /// Create new titled multi-select items.
    #[must_use]
    pub fn new(options: Vec<EnumOption>) -> Self {
        Self {
            options,
            meta: None,
        }
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

/// Custom or future typed item schema for multi-select properties.
///
/// This preserves unknown item `type` values and the rest of the `items`
/// payload for clients that store, replay, proxy, or forward elicitation
/// requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[schemars(inline)]
#[schemars(transform = other_multi_select_items_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherMultiSelectItems {
    /// Custom or future multi-select item type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown item schema payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherMultiSelectItems {
    /// Builds [`OtherMultiSelectItems`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherMultiSelectItems {
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

        if is_known_multi_select_item_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known multi-select item type `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

const KNOWN_MULTI_SELECT_ITEM_TYPES: &[&str] = &["string"];

fn is_known_multi_select_item_type(type_: &str) -> bool {
    KNOWN_MULTI_SELECT_ITEM_TYPES.contains(&type_)
}

fn other_multi_select_items_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        KNOWN_MULTI_SELECT_ITEM_TYPES,
    );
}

/// Items for a multi-select (array) property schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum MultiSelectItems {
    /// Multi-select string items with plain string values.
    String(StringMultiSelectItems),
    /// Custom or future typed multi-select items.
    #[serde(untagged)]
    Other(OtherMultiSelectItems),
    /// Titled multi-select items with human-readable labels.
    #[serde(untagged)]
    Titled(TitledMultiSelectItems),
}

/// Schema for multi-select (array) properties in an elicitation form.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MultiSelectPropertySchema {
    /// Optional title for the property.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Human-readable description.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
    /// Minimum number of items to select.
    #[serde(default)]
    pub min_items: Option<u64>,
    /// Maximum number of items to select.
    #[serde(default)]
    pub max_items: Option<u64>,
    /// The items definition describing allowed values.
    pub items: MultiSelectItems,
    /// Default selected values.
    #[serde_as(deserialize_as = "DefaultOnError<Option<VecSkipError<_, SkipListener>>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    #[serde(default)]
    pub default: Option<Vec<String>>,
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

impl MultiSelectPropertySchema {
    /// Create a new untitled multi-select property schema.
    #[must_use]
    pub fn new(values: Vec<String>) -> Self {
        Self {
            title: None,
            description: None,
            min_items: None,
            max_items: None,
            items: MultiSelectItems::String(StringMultiSelectItems::new(values)),
            default: None,
            meta: None,
        }
    }

    /// Create a new titled multi-select property schema.
    #[must_use]
    pub fn titled(options: Vec<EnumOption>) -> Self {
        Self {
            title: None,
            description: None,
            min_items: None,
            max_items: None,
            items: MultiSelectItems::Titled(TitledMultiSelectItems {
                options,
                meta: None,
            }),
            default: None,
            meta: None,
        }
    }

    /// Optional title for the property.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
        self
    }

    /// Minimum number of items to select.
    #[must_use]
    pub fn min_items(mut self, min_items: impl IntoOption<u64>) -> Self {
        self.min_items = min_items.into_option();
        self
    }

    /// Maximum number of items to select.
    #[must_use]
    pub fn max_items(mut self, max_items: impl IntoOption<u64>) -> Self {
        self.max_items = max_items.into_option();
        self
    }

    /// Default selected values.
    #[must_use]
    pub fn default_value(mut self, default: impl IntoOption<Vec<String>>) -> Self {
        self.default = default.into_option();
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

/// Property schema for elicitation form fields.
///
/// Each variant corresponds to a JSON Schema `"type"` value.
/// Single-select enums use the `String` variant with `enum` or `oneOf` set.
/// Multi-select enums use the `Array` variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ElicitationPropertySchema {
    /// String property (or single-select enum when `enum`/`oneOf` is set).
    String(StringPropertySchema),
    /// Number (floating-point) property.
    Number(NumberPropertySchema),
    /// Integer property.
    Integer(IntegerPropertySchema),
    /// Boolean property.
    Boolean(BooleanPropertySchema),
    /// Multi-select array property.
    Array(MultiSelectPropertySchema),
    /// Custom or future elicitation property schema.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Clients that do not understand this property schema type should preserve
    /// the raw schema when storing, replaying, proxying, or forwarding
    /// elicitation requests. They MUST NOT render it as a known input control.
    #[serde(untagged)]
    Other(OtherElicitationPropertySchema),
}

/// Custom or future elicitation property schema payload.
///
/// This preserves the unknown `type` discriminator and the rest of the property
/// schema object for clients that store, replay, proxy, or forward elicitation
/// requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[schemars(inline)]
#[schemars(transform = other_elicitation_property_schema_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherElicitationPropertySchema {
    /// Custom or future elicitation property schema type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// Additional fields from the unknown property schema payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherElicitationPropertySchema {
    /// Builds [`OtherElicitationPropertySchema`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(type_: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("type");
        Self {
            type_: type_.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherElicitationPropertySchema {
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

        if is_known_elicitation_property_schema_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known elicitation property schema type `{type_}` did not match its schema"
            )));
        }

        Ok(Self { type_, fields })
    }
}

const KNOWN_ELICITATION_PROPERTY_SCHEMA_TYPES: &[&str] =
    &["string", "number", "integer", "boolean", "array"];

fn is_known_elicitation_property_schema_type(type_: &str) -> bool {
    KNOWN_ELICITATION_PROPERTY_SCHEMA_TYPES.contains(&type_)
}

fn other_elicitation_property_schema_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        KNOWN_ELICITATION_PROPERTY_SCHEMA_TYPES,
    );
}

impl From<StringPropertySchema> for ElicitationPropertySchema {
    fn from(schema: StringPropertySchema) -> Self {
        Self::String(schema)
    }
}

impl From<NumberPropertySchema> for ElicitationPropertySchema {
    fn from(schema: NumberPropertySchema) -> Self {
        Self::Number(schema)
    }
}

impl From<IntegerPropertySchema> for ElicitationPropertySchema {
    fn from(schema: IntegerPropertySchema) -> Self {
        Self::Integer(schema)
    }
}

impl From<BooleanPropertySchema> for ElicitationPropertySchema {
    fn from(schema: BooleanPropertySchema) -> Self {
        Self::Boolean(schema)
    }
}

impl From<MultiSelectPropertySchema> for ElicitationPropertySchema {
    fn from(schema: MultiSelectPropertySchema) -> Self {
        Self::Array(schema)
    }
}

fn default_object_type() -> ElicitationSchemaType {
    ElicitationSchemaType::Object
}

/// Type-safe elicitation schema for requesting structured user input.
///
/// This represents a JSON Schema object with primitive-typed properties,
/// as required by the elicitation specification.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationSchema {
    /// Type discriminator. Always `"object"`.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(rename = "type", default = "default_object_type")]
    pub type_: ElicitationSchemaType,
    /// Optional title for the schema.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub title: Option<String>,
    /// Property definitions (must be primitive types).
    #[serde(default)]
    pub properties: BTreeMap<String, ElicitationPropertySchema>,
    /// List of required property names.
    #[serde(default)]
    pub required: Option<Vec<String>>,
    /// Optional description of what this schema represents.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub description: Option<String>,
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

impl Default for ElicitationSchema {
    fn default() -> Self {
        Self {
            type_: default_object_type(),
            title: None,
            properties: BTreeMap::new(),
            required: None,
            description: None,
            meta: None,
        }
    }
}

impl ElicitationSchema {
    /// Create a new empty elicitation schema.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Optional title for the schema.
    #[must_use]
    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    /// Optional description of what this schema represents.
    #[must_use]
    pub fn description(mut self, description: impl IntoOption<String>) -> Self {
        self.description = description.into_option();
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

    /// Add a property to the schema.
    #[must_use]
    pub fn property<S>(mut self, name: impl Into<String>, schema: S, required: bool) -> Self
    where
        S: Into<ElicitationPropertySchema>,
    {
        let name = name.into();
        self.properties.insert(name.clone(), schema.into());

        if required {
            let required_fields = self.required.get_or_insert_with(Vec::new);
            if !required_fields.contains(&name) {
                required_fields.push(name);
            }
        } else if let Some(required_fields) = &mut self.required {
            required_fields.retain(|field| field != &name);

            if required_fields.is_empty() {
                self.required = None;
            }
        }

        self
    }

    /// Add a string property.
    #[must_use]
    pub fn string(self, name: impl Into<String>, required: bool) -> Self {
        self.property(name, StringPropertySchema::new(), required)
    }

    /// Add an email property.
    #[must_use]
    pub fn email(self, name: impl Into<String>, required: bool) -> Self {
        self.property(name, StringPropertySchema::email(), required)
    }

    /// Add a URI property.
    #[must_use]
    pub fn uri(self, name: impl Into<String>, required: bool) -> Self {
        self.property(name, StringPropertySchema::uri(), required)
    }

    /// Add a date property.
    #[must_use]
    pub fn date(self, name: impl Into<String>, required: bool) -> Self {
        self.property(name, StringPropertySchema::date(), required)
    }

    /// Add a date-time property.
    #[must_use]
    pub fn date_time(self, name: impl Into<String>, required: bool) -> Self {
        self.property(name, StringPropertySchema::date_time(), required)
    }

    /// Add a number property with range.
    #[must_use]
    pub fn number(self, name: impl Into<String>, min: f64, max: f64, required: bool) -> Self {
        self.property(
            name,
            NumberPropertySchema::new().minimum(min).maximum(max),
            required,
        )
    }

    /// Add an integer property with range.
    #[must_use]
    pub fn integer(self, name: impl Into<String>, min: i64, max: i64, required: bool) -> Self {
        self.property(
            name,
            IntegerPropertySchema::new().minimum(min).maximum(max),
            required,
        )
    }

    /// Add a boolean property.
    #[must_use]
    pub fn boolean(self, name: impl Into<String>, required: bool) -> Self {
        self.property(name, BooleanPropertySchema::new(), required)
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Elicitation capabilities supported by the client.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationCapabilities {
    /// Whether the client supports form-based elicitation.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the client supports form-based elicitation.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub form: Option<ElicitationFormCapabilities>,
    /// Whether the client supports URL-based elicitation.
    ///
    /// Optional. Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the client supports URL-based elicitation.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub url: Option<ElicitationUrlCapabilities>,
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

impl ElicitationCapabilities {
    /// Builds an empty [`ElicitationCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the client supports form-based elicitation.
    ///
    /// Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the client supports form-based elicitation.
    #[must_use]
    pub fn form(mut self, form: impl IntoOption<ElicitationFormCapabilities>) -> Self {
        self.form = form.into_option();
        self
    }

    /// Whether the client supports URL-based elicitation.
    ///
    /// Omitted or `null` both mean the client does not advertise support.
    /// Supplying `{}` means the client supports URL-based elicitation.
    #[must_use]
    pub fn url(mut self, url: impl IntoOption<ElicitationUrlCapabilities>) -> Self {
        self.url = url.into_option();
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Form-based elicitation capabilities.
///
/// Supplying `{}` means the client supports form-based elicitation.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationFormCapabilities {
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

impl ElicitationFormCapabilities {
    /// Builds an empty [`ElicitationFormCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// URL-based elicitation capabilities.
///
/// Supplying `{}` means the client supports URL-based elicitation.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationUrlCapabilities {
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

impl ElicitationUrlCapabilities {
    /// Builds an empty [`ElicitationUrlCapabilities`]; use builder methods to advertise supported sub-capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// The scope of an elicitation request, determining what context it's tied to.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ElicitationScope {
    /// Tied to a session, optionally to a specific tool call within that session.
    Session(ElicitationSessionScope),
    /// Tied to a specific JSON-RPC request outside of a session
    /// (e.g., during auth/configuration phases before any session is started).
    Request(ElicitationRequestScope),
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Session-scoped elicitation, optionally tied to a specific tool call.
///
/// When `tool_call_id` is set, the elicitation is tied to a specific tool call.
/// This is useful when an agent receives an elicitation from an MCP server
/// during a tool call and needs to redirect it to the user.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationSessionScope {
    /// The session this elicitation is tied to.
    pub session_id: SessionId,
    /// Optional tool call within the session.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    pub tool_call_id: Option<ToolCallId>,
}

impl ElicitationSessionScope {
    /// Builds [`ElicitationSessionScope`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(session_id: impl Into<SessionId>) -> Self {
        Self {
            session_id: session_id.into(),
            tool_call_id: None,
        }
    }

    /// Sets or clears the optional `toolCallId` field.
    #[must_use]
    pub fn tool_call_id(mut self, tool_call_id: impl IntoOption<ToolCallId>) -> Self {
        self.tool_call_id = tool_call_id.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request-scoped elicitation, tied to a specific JSON-RPC request outside of a session
/// (e.g., during auth/configuration phases before any session is started).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationRequestScope {
    /// The request this elicitation is tied to.
    pub request_id: RequestId,
}

impl ElicitationRequestScope {
    /// Builds [`ElicitationRequestScope`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(request_id: impl Into<RequestId>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}

impl From<ElicitationSessionScope> for ElicitationScope {
    fn from(scope: ElicitationSessionScope) -> Self {
        Self::Session(scope)
    }
}

impl From<ElicitationRequestScope> for ElicitationScope {
    fn from(scope: ElicitationRequestScope) -> Self {
        Self::Request(scope)
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Request from the agent to elicit structured user input.
///
/// The agent sends this to the client to request information from the user,
/// either via a form or by directing them to a URL.
/// Elicitations are tied to a session (optionally a tool call) or a request.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[schemars(extend("x-side" = "client", "x-method" = ELICITATION_CREATE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateElicitationRequest {
    /// The elicitation mode and its mode-specific fields.
    #[serde(flatten)]
    pub mode: ElicitationMode,
    /// A human-readable message describing what input is needed.
    pub message: String,
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

impl CreateElicitationRequest {
    /// Builds [`CreateElicitationRequest`] with the required request fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(mode: impl Into<ElicitationMode>, message: impl Into<String>) -> Self {
        Self {
            mode: mode.into(),
            message: message.into(),
            meta: None,
        }
    }

    /// Returns the scope this elicitation is tied to.
    #[must_use]
    pub fn scope(&self) -> &ElicitationScope {
        self.mode.scope()
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// The mode of elicitation, determining how user input is collected.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "mode", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ElicitationMode {
    /// Form-based elicitation where the client renders a form from the provided schema.
    Form(ElicitationFormMode),
    /// URL-based elicitation where the client directs the user to a URL.
    Url(ElicitationUrlMode),
    /// Custom or future elicitation mode.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Clients that do not understand this mode should preserve the raw payload
    /// when storing, replaying, proxying, or forwarding elicitation requests.
    /// They MUST NOT render it as a known elicitation mode.
    #[serde(untagged)]
    Other(OtherElicitationMode),
}

/// Custom or future elicitation mode payload.
///
/// This preserves the unknown `mode` discriminator and the rest of the mode
/// object for clients that store, replay, proxy, or forward elicitation
/// requests.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq)]
#[schemars(inline)]
#[schemars(transform = other_elicitation_mode_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherElicitationMode {
    /// Custom or future elicitation mode.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    pub mode: String,
    /// The scope this elicitation is tied to.
    #[serde(flatten)]
    pub scope: ElicitationScope,
    /// Additional fields from the unknown elicitation mode payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherElicitationMode {
    /// Builds [`OtherElicitationMode`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        mode: impl Into<String>,
        scope: impl Into<ElicitationScope>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("mode");
        remove_elicitation_scope_fields(&mut fields);
        Self {
            mode: mode.into(),
            scope: scope.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherElicitationMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let mode = fields
            .remove("mode")
            .ok_or_else(|| serde::de::Error::missing_field("mode"))?;
        let serde_json::Value::String(mode) = mode else {
            return Err(serde::de::Error::custom("`mode` must be a string"));
        };

        if is_known_elicitation_mode(&mode) {
            return Err(serde::de::Error::custom(format!(
                "known elicitation mode `{mode}` did not match its schema"
            )));
        }

        let scope = serde_json::from_value::<ElicitationScope>(serde_json::Value::Object(
            fields.clone().into_iter().collect(),
        ))
        .map_err(serde::de::Error::custom)?;
        remove_elicitation_scope_fields(&mut fields);

        Ok(Self {
            mode,
            scope,
            fields,
        })
    }
}

const KNOWN_ELICITATION_MODES: &[&str] = &["form", "url"];

fn is_known_elicitation_mode(mode: &str) -> bool {
    KNOWN_ELICITATION_MODES.contains(&mode)
}

fn remove_elicitation_scope_fields(fields: &mut BTreeMap<String, serde_json::Value>) {
    fields.remove("sessionId");
    fields.remove("toolCallId");
    fields.remove("requestId");
}

fn other_elicitation_mode_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(schema, "mode", KNOWN_ELICITATION_MODES);
}

impl From<ElicitationFormMode> for ElicitationMode {
    fn from(mode: ElicitationFormMode) -> Self {
        Self::Form(mode)
    }
}

impl From<ElicitationUrlMode> for ElicitationMode {
    fn from(mode: ElicitationUrlMode) -> Self {
        Self::Url(mode)
    }
}

impl From<OtherElicitationMode> for ElicitationMode {
    fn from(mode: OtherElicitationMode) -> Self {
        Self::Other(mode)
    }
}

impl ElicitationMode {
    /// Returns the scope this elicitation mode is tied to.
    #[must_use]
    pub fn scope(&self) -> &ElicitationScope {
        match self {
            Self::Form(f) => &f.scope,
            Self::Url(u) => &u.scope,
            Self::Other(other) => &other.scope,
        }
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Form-based elicitation mode where the client renders a form from the provided schema.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationFormMode {
    /// The scope this elicitation is tied to.
    #[serde(flatten)]
    pub scope: ElicitationScope,
    /// A JSON Schema describing the form fields to present to the user.
    pub requested_schema: ElicitationSchema,
}

impl ElicitationFormMode {
    /// Builds [`ElicitationFormMode`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(scope: impl Into<ElicitationScope>, requested_schema: ElicitationSchema) -> Self {
        Self {
            scope: scope.into(),
            requested_schema,
        }
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// URL-based elicitation mode where the client directs the user to a URL.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationUrlMode {
    /// The scope this elicitation is tied to.
    #[serde(flatten)]
    pub scope: ElicitationScope,
    /// The unique identifier for this elicitation.
    pub elicitation_id: ElicitationId,
    /// The URL to direct the user to.
    #[schemars(url)]
    pub url: String,
}

impl ElicitationUrlMode {
    /// Builds [`ElicitationUrlMode`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        scope: impl Into<ElicitationScope>,
        elicitation_id: impl Into<ElicitationId>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            scope: scope.into(),
            elicitation_id: elicitation_id.into(),
            url: url.into(),
        }
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Response from the client to an elicitation request.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[schemars(extend("x-side" = "client", "x-method" = ELICITATION_CREATE_METHOD_NAME))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateElicitationResponse {
    /// The user's action in response to the elicitation.
    #[serde(flatten)]
    pub action: ElicitationAction,
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

impl CreateElicitationResponse {
    /// Builds [`CreateElicitationResponse`] with the required response fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(action: impl Into<ElicitationAction>) -> Self {
        Self {
            action: action.into(),
            meta: None,
        }
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

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// The user's action in response to an elicitation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "action", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ElicitationAction {
    /// The user accepted and provided content.
    Accept(ElicitationAcceptAction),
    /// The user declined the elicitation.
    Decline,
    /// The elicitation was cancelled.
    Cancel,
    /// Custom or future elicitation action.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Agents that do not understand this action should preserve the raw
    /// payload when storing, replaying, proxying, or forwarding elicitation
    /// responses. They MUST NOT treat it as a known elicitation action.
    #[serde(untagged)]
    Other(OtherElicitationAction),
}

/// Custom or future elicitation action payload.
///
/// This preserves the unknown `action` discriminator and the rest of the
/// response object for agents that store, replay, proxy, or forward elicitation
/// responses.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq)]
#[schemars(inline)]
#[schemars(transform = other_elicitation_action_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherElicitationAction {
    /// Custom or future elicitation action.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    pub action: String,
    /// Additional fields from the unknown elicitation action payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherElicitationAction {
    /// Builds [`OtherElicitationAction`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(action: impl Into<String>, mut fields: BTreeMap<String, serde_json::Value>) -> Self {
        fields.remove("action");
        Self {
            action: action.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherElicitationAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let action = fields
            .remove("action")
            .ok_or_else(|| serde::de::Error::missing_field("action"))?;
        let serde_json::Value::String(action) = action else {
            return Err(serde::de::Error::custom("`action` must be a string"));
        };

        if is_known_elicitation_action(&action) {
            return Err(serde::de::Error::custom(format!(
                "known elicitation action `{action}` did not match its schema"
            )));
        }

        Ok(Self { action, fields })
    }
}

const KNOWN_ELICITATION_ACTIONS: &[&str] = &["accept", "decline", "cancel"];

fn is_known_elicitation_action(action: &str) -> bool {
    KNOWN_ELICITATION_ACTIONS.contains(&action)
}

fn other_elicitation_action_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "action",
        KNOWN_ELICITATION_ACTIONS,
    );
}

impl From<ElicitationAcceptAction> for ElicitationAction {
    fn from(action: ElicitationAcceptAction) -> Self {
        Self::Accept(action)
    }
}

impl From<OtherElicitationAction> for ElicitationAction {
    fn from(action: OtherElicitationAction) -> Self {
        Self::Other(action)
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// The user accepted the elicitation and provided content.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ElicitationAcceptAction {
    /// The user-provided content, if any, as an object matching the requested schema.
    #[serde(default)]
    pub content: Option<BTreeMap<String, ElicitationContentValue>>,
}

impl ElicitationAcceptAction {
    /// Builds [`ElicitationAcceptAction`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new() -> Self {
        Self { content: None }
    }

    /// The user-provided content as an object matching the requested schema.
    #[must_use]
    pub fn content(
        mut self,
        content: impl IntoOption<BTreeMap<String, ElicitationContentValue>>,
    ) -> Self {
        self.content = content.into_option();
        self
    }
}

/// Allowed wire representations for [`ElicitationContentValue`].
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ElicitationContentValue {
    /// String value accepted in elicitation response content.
    String(String),
    /// Integer value accepted in elicitation response content.
    Integer(i64),
    /// Number value accepted in elicitation response content.
    Number(f64),
    /// Boolean value accepted in elicitation response content.
    Boolean(bool),
    /// String array value accepted in elicitation response content.
    StringArray(Vec<String>),
}

impl From<String> for ElicitationContentValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ElicitationContentValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<i64> for ElicitationContentValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<i32> for ElicitationContentValue {
    fn from(value: i32) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<f64> for ElicitationContentValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for ElicitationContentValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<Vec<String>> for ElicitationContentValue {
    fn from(value: Vec<String>) -> Self {
        Self::StringArray(value)
    }
}

impl From<Vec<&str>> for ElicitationContentValue {
    fn from(value: Vec<&str>) -> Self {
        Self::StringArray(value.into_iter().map(str::to_string).collect())
    }
}

impl Default for ElicitationAcceptAction {
    fn default() -> Self {
        Self::new()
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Notification sent by the agent when a URL-based elicitation is complete.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[schemars(extend("x-side" = "client", "x-method" = ELICITATION_COMPLETE_NOTIFICATION))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CompleteElicitationNotification {
    /// The ID of the elicitation that completed.
    pub elicitation_id: ElicitationId,
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

impl CompleteElicitationNotification {
    /// Builds [`CompleteElicitationNotification`] with the required notification fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(elicitation_id: impl Into<ElicitationId>) -> Self {
        Self {
            elicitation_id: elicitation_id.into(),
            meta: None,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn form_mode_request_serialization() {
        let schema = ElicitationSchema::new().string("name", true);
        let req = CreateElicitationRequest::new(
            ElicitationFormMode::new(ElicitationSessionScope::new("sess_1"), schema),
            "Please enter your name",
        );

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["sessionId"], "sess_1");
        assert!(json.get("toolCallId").is_none());
        assert_eq!(json["mode"], "form");
        assert_eq!(json["message"], "Please enter your name");
        assert!(json["requestedSchema"].is_object());
        assert_eq!(json["requestedSchema"]["type"], "object");
        assert_eq!(
            json["requestedSchema"]["properties"]["name"]["type"],
            "string"
        );

        let roundtripped: CreateElicitationRequest = serde_json::from_value(json).unwrap();
        assert_eq!(
            *roundtripped.scope(),
            ElicitationSessionScope::new("sess_1").into()
        );
        assert_eq!(roundtripped.message, "Please enter your name");
        assert!(matches!(roundtripped.mode, ElicitationMode::Form(_)));
    }

    #[test]
    fn url_mode_request_serialization() {
        let req = CreateElicitationRequest::new(
            ElicitationUrlMode::new(
                ElicitationSessionScope::new("sess_2").tool_call_id("tc_1"),
                "elic_1",
                "https://example.com/auth",
            ),
            "Please authenticate",
        );

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["sessionId"], "sess_2");
        assert_eq!(json["toolCallId"], "tc_1");
        assert_eq!(json["mode"], "url");
        assert_eq!(json["elicitationId"], "elic_1");
        assert_eq!(json["url"], "https://example.com/auth");
        assert_eq!(json["message"], "Please authenticate");

        let roundtripped: CreateElicitationRequest = serde_json::from_value(json).unwrap();
        assert_eq!(
            *roundtripped.scope(),
            ElicitationSessionScope::new("sess_2")
                .tool_call_id("tc_1")
                .into()
        );
        assert!(matches!(roundtripped.mode, ElicitationMode::Url(_)));
    }

    #[test]
    fn response_accept_serialization() {
        let resp = CreateElicitationResponse::new(ElicitationAction::Accept(
            ElicitationAcceptAction::new().content(BTreeMap::from([(
                "name".to_string(),
                ElicitationContentValue::from("Alice"),
            )])),
        ));

        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["action"], "accept");
        assert_eq!(json["content"]["name"], "Alice");

        let roundtripped: CreateElicitationResponse = serde_json::from_value(json).unwrap();
        assert!(matches!(
            roundtripped.action,
            ElicitationAction::Accept(ElicitationAcceptAction {
                content: Some(_),
                ..
            })
        ));
    }

    #[test]
    fn response_decline_serialization() {
        let resp = CreateElicitationResponse::new(ElicitationAction::Decline);

        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["action"], "decline");

        let roundtripped: CreateElicitationResponse = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped.action, ElicitationAction::Decline));
    }

    #[test]
    fn response_cancel_serialization() {
        let resp = CreateElicitationResponse::new(ElicitationAction::Cancel);

        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["action"], "cancel");

        let roundtripped: CreateElicitationResponse = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped.action, ElicitationAction::Cancel));
    }

    #[test]
    fn unknown_action_response_serialization() {
        let json = json!({
            "action": "_defer",
            "reason": "waiting",
            "retryAfterMs": 1000
        });

        let resp: CreateElicitationResponse = serde_json::from_value(json.clone()).unwrap();
        let ElicitationAction::Other(other) = &resp.action else {
            panic!("expected unknown elicitation action");
        };

        assert_eq!(other.action, "_defer");
        assert_eq!(other.fields.get("reason"), Some(&json!("waiting")));
        assert_eq!(other.fields.get("retryAfterMs"), Some(&json!(1000)));
        assert_eq!(serde_json::to_value(&resp).unwrap(), json);
    }

    #[test]
    fn unknown_action_does_not_hide_known_action() {
        assert!(
            serde_json::from_value::<OtherElicitationAction>(json!({
                "action": "accept",
                "content": {}
            }))
            .is_err()
        );
        assert!(serde_json::from_value::<ElicitationAction>(json!({})).is_err());
    }

    #[test]
    fn url_mode_request_scope_serialization() {
        let req = CreateElicitationRequest::new(
            ElicitationUrlMode::new(
                ElicitationRequestScope::new(RequestId::Number(42)),
                "elic_2",
                "https://example.com/setup",
            ),
            "Please complete setup",
        );

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["requestId"], 42);
        assert!(json.get("sessionId").is_none());
        assert_eq!(json["mode"], "url");
        assert_eq!(json["elicitationId"], "elic_2");
        assert_eq!(json["url"], "https://example.com/setup");
        assert_eq!(json["message"], "Please complete setup");

        let roundtripped: CreateElicitationRequest = serde_json::from_value(json).unwrap();
        assert_eq!(
            *roundtripped.scope(),
            ElicitationRequestScope::new(RequestId::Number(42)).into()
        );
        assert!(matches!(roundtripped.mode, ElicitationMode::Url(_)));
    }

    #[test]
    fn unknown_mode_request_serialization() {
        let json = json!({
            "requestId": 42,
            "mode": "_browser",
            "message": "Open a browser window",
            "target": "login"
        });

        let req: CreateElicitationRequest = serde_json::from_value(json.clone()).unwrap();
        let ElicitationMode::Other(other) = &req.mode else {
            panic!("expected unknown elicitation mode");
        };

        assert_eq!(other.mode, "_browser");
        assert_eq!(
            other.scope,
            ElicitationRequestScope::new(RequestId::Number(42)).into()
        );
        assert_eq!(other.fields.get("target"), Some(&json!("login")));
        assert_eq!(
            *req.scope(),
            ElicitationRequestScope::new(RequestId::Number(42)).into()
        );
        assert_eq!(serde_json::to_value(&req).unwrap(), json);
    }

    #[test]
    fn unknown_mode_does_not_hide_malformed_known_mode() {
        let missing_requested_schema = json!({
            "requestId": 42,
            "mode": "form",
            "message": "Enter your name"
        });

        assert!(
            serde_json::from_value::<CreateElicitationRequest>(missing_requested_schema).is_err()
        );
        assert!(serde_json::from_value::<ElicitationMode>(json!({})).is_err());
    }

    #[test]
    fn request_scope_request_serialization() {
        let req = CreateElicitationRequest::new(
            ElicitationFormMode::new(
                ElicitationRequestScope::new(RequestId::Number(99)),
                ElicitationSchema::new().string("workspace", true),
            ),
            "Enter workspace name",
        );

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["requestId"], 99);
        assert!(json.get("sessionId").is_none());

        let roundtripped: CreateElicitationRequest = serde_json::from_value(json).unwrap();
        assert_eq!(
            *roundtripped.scope(),
            ElicitationRequestScope::new(RequestId::Number(99)).into()
        );
    }

    /// These tests verify that serialization through `ClientResponse` produces the
    /// correct flattened wire format and round-trips back via the concrete
    /// `CreateElicitationResponse` type.
    #[test]
    fn client_response_serialization_accept() {
        use crate::v2::ClientResponse;

        let resp =
            ClientResponse::CreateElicitationResponse(Box::new(CreateElicitationResponse::new(
                ElicitationAction::Accept(ElicitationAcceptAction::new().content(BTreeMap::from(
                    [("name".to_string(), ElicitationContentValue::from("Alice"))],
                ))),
            )));
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["action"], "accept");
        assert_eq!(json["content"]["name"], "Alice");

        // Round-trip back through the concrete type
        let roundtripped: CreateElicitationResponse = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped.action, ElicitationAction::Accept(_)));
    }

    #[test]
    fn client_response_serialization_decline() {
        use crate::v2::ClientResponse;

        let resp = ClientResponse::CreateElicitationResponse(Box::new(
            CreateElicitationResponse::new(ElicitationAction::Decline),
        ));
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["action"], "decline");

        let roundtripped: CreateElicitationResponse = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped.action, ElicitationAction::Decline));
    }

    #[test]
    fn client_response_serialization_cancel() {
        use crate::v2::ClientResponse;

        let resp = ClientResponse::CreateElicitationResponse(Box::new(
            CreateElicitationResponse::new(ElicitationAction::Cancel),
        ));
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["action"], "cancel");

        let roundtripped: CreateElicitationResponse = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped.action, ElicitationAction::Cancel));
    }

    /// Guard against serde regressions with the `flatten` + internally-tagged combination.
    /// Extra fields in the JSON must not cause deserialization failures.
    #[test]
    fn request_tolerates_extra_fields() {
        let json = json!({
            "sessionId": "sess_1",
            "mode": "form",
            "message": "Enter your name",
            "requestedSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "title": "Name" }
                },
                "required": ["name"]
            },
            "unknownStringField": "hello",
            "unknownNumberField": 42
        });

        let req: CreateElicitationRequest = serde_json::from_value(json).unwrap();
        assert_eq!(*req.scope(), ElicitationSessionScope::new("sess_1").into());
        assert_eq!(req.message, "Enter your name");
        assert!(matches!(req.mode, ElicitationMode::Form(_)));
    }

    #[test]
    fn completion_notification_serialization() {
        let notif = CompleteElicitationNotification::new("elic_1");

        let json = serde_json::to_value(&notif).unwrap();
        assert_eq!(json["elicitationId"], "elic_1");

        let roundtripped: CompleteElicitationNotification = serde_json::from_value(json).unwrap();
        assert_eq!(roundtripped.elicitation_id, ElicitationId::new("elic_1"));
    }

    #[test]
    fn capabilities_form_only() {
        let caps = ElicitationCapabilities::new().form(ElicitationFormCapabilities::new());

        let json = serde_json::to_value(&caps).unwrap();
        assert!(json["form"].is_object());
        assert!(json.get("url").is_none());

        let roundtripped: ElicitationCapabilities = serde_json::from_value(json).unwrap();
        assert!(roundtripped.form.is_some());
        assert!(roundtripped.url.is_none());
    }

    #[test]
    fn capabilities_url_only() {
        let caps = ElicitationCapabilities::new().url(ElicitationUrlCapabilities::new());

        let json = serde_json::to_value(&caps).unwrap();
        assert!(json.get("form").is_none());
        assert!(json["url"].is_object());

        let roundtripped: ElicitationCapabilities = serde_json::from_value(json).unwrap();
        assert!(roundtripped.form.is_none());
        assert!(roundtripped.url.is_some());
    }

    #[test]
    fn capabilities_both() {
        let caps = ElicitationCapabilities::new()
            .form(ElicitationFormCapabilities::new())
            .url(ElicitationUrlCapabilities::new());

        let json = serde_json::to_value(&caps).unwrap();
        assert!(json["form"].is_object());
        assert!(json["url"].is_object());

        let roundtripped: ElicitationCapabilities = serde_json::from_value(json).unwrap();
        assert!(roundtripped.form.is_some());
        assert!(roundtripped.url.is_some());
    }

    #[test]
    fn schema_default_sets_object_type() {
        let schema = ElicitationSchema::default();

        assert_eq!(schema.type_, ElicitationSchemaType::Object);
        assert!(schema.properties.is_empty());

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["type"], "object");
    }

    #[test]
    fn schema_builder_serialization() {
        let schema = ElicitationSchema::new()
            .string("name", true)
            .email("email", true)
            .integer("age", 0, 150, true)
            .boolean("newsletter", false)
            .description("User registration");

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["type"], "object");
        assert_eq!(json["description"], "User registration");
        assert_eq!(json["properties"]["name"]["type"], "string");
        assert_eq!(json["properties"]["email"]["type"], "string");
        assert_eq!(json["properties"]["email"]["format"], "email");
        assert_eq!(json["properties"]["age"]["type"], "integer");
        assert_eq!(json["properties"]["age"]["minimum"], 0);
        assert_eq!(json["properties"]["age"]["maximum"], 150);
        assert_eq!(json["properties"]["newsletter"]["type"], "boolean");

        let required = json["required"].as_array().unwrap();
        assert!(required.contains(&json!("name")));
        assert!(required.contains(&json!("email")));
        assert!(required.contains(&json!("age")));
        assert!(!required.contains(&json!("newsletter")));

        let roundtripped: ElicitationSchema = serde_json::from_value(json).unwrap();
        assert_eq!(roundtripped.properties.len(), 4);
        assert!(roundtripped.required.unwrap().contains(&"name".to_string()));
    }

    #[test]
    fn schema_string_enum_serialization() {
        let schema = ElicitationSchema::new().property(
            "color",
            StringPropertySchema::new().enum_values(vec![
                "red".into(),
                "green".into(),
                "blue".into(),
            ]),
            true,
        );

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["properties"]["color"]["type"], "string");
        let enum_vals = json["properties"]["color"]["enum"].as_array().unwrap();
        assert_eq!(enum_vals.len(), 3);

        let roundtripped: ElicitationSchema = serde_json::from_value(json).unwrap();
        if let ElicitationPropertySchema::String(s) = roundtripped.properties.get("color").unwrap()
        {
            assert_eq!(s.enum_values.as_ref().unwrap().len(), 3);
        } else {
            panic!("expected String variant");
        }
    }

    #[test]
    fn schema_multi_select_serialization() {
        let schema = ElicitationSchema::new().property(
            "colors",
            MultiSelectPropertySchema::new(vec!["red".into(), "green".into(), "blue".into()])
                .min_items(1)
                .max_items(3),
            false,
        );

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["properties"]["colors"]["type"], "array");
        assert_eq!(json["properties"]["colors"]["items"]["type"], "string");
        assert_eq!(json["properties"]["colors"]["minItems"], 1);
        assert_eq!(json["properties"]["colors"]["maxItems"], 3);

        let roundtripped: ElicitationSchema = serde_json::from_value(json).unwrap();
        let ElicitationPropertySchema::Array(array) =
            roundtripped.properties.get("colors").unwrap()
        else {
            panic!("expected Array variant");
        };
        let MultiSelectItems::String(items) = &array.items else {
            panic!("expected String multi-select items");
        };
        assert_eq!(items.values.len(), 3);
    }

    #[test]
    fn multi_select_titled_items_keep_mcp_shape() {
        let items = MultiSelectItems::Titled(TitledMultiSelectItems::new(vec![EnumOption::new(
            "#ff0000", "Red",
        )]));

        let json = serde_json::to_value(&items).unwrap();
        assert!(json.get("type").is_none());
        assert_eq!(json["anyOf"][0]["const"], "#ff0000");
        assert_eq!(json["anyOf"][0]["title"], "Red");

        let roundtripped: MultiSelectItems = serde_json::from_value(json).unwrap();
        assert!(matches!(roundtripped, MultiSelectItems::Titled(_)));
    }

    #[test]
    fn multi_select_items_preserve_unknown_type() {
        let json = json!({
            "type": "_token",
            "format": "workspace",
            "anyOf": [
                { "const": "repo", "title": "Repository" }
            ]
        });

        let items: MultiSelectItems = serde_json::from_value(json.clone()).unwrap();
        let MultiSelectItems::Other(other) = &items else {
            panic!("expected unknown multi-select items");
        };

        assert_eq!(other.type_, "_token");
        assert_eq!(other.fields.get("format"), Some(&json!("workspace")));
        assert_eq!(other.fields.get("anyOf"), Some(&json["anyOf"]));
        assert_eq!(serde_json::to_value(&items).unwrap(), json);
    }

    #[test]
    fn multi_select_items_unknown_does_not_hide_malformed_string_type() {
        assert!(
            serde_json::from_value::<MultiSelectItems>(json!({
                "type": "string"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<OtherMultiSelectItems>(json!({
                "type": "string",
                "format": "workspace"
            }))
            .is_err()
        );
    }

    #[test]
    fn property_schema_preserves_unknown_type() {
        let schema: ElicitationSchema = serde_json::from_value(json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "_location",
                    "title": "Location",
                    "precision": "city"
                }
            }
        }))
        .unwrap();

        let ElicitationPropertySchema::Other(unknown) = schema.properties.get("location").unwrap()
        else {
            panic!("expected unknown property schema");
        };

        assert_eq!(unknown.type_, "_location");
        assert_eq!(unknown.fields.get("title"), Some(&json!("Location")));
        assert_eq!(unknown.fields.get("precision"), Some(&json!("city")));
        assert_eq!(
            serde_json::to_value(ElicitationPropertySchema::Other(unknown.clone())).unwrap(),
            json!({
                "type": "_location",
                "title": "Location",
                "precision": "city"
            })
        );
    }

    #[test]
    fn property_schema_unknown_does_not_hide_malformed_known_type() {
        assert!(
            serde_json::from_value::<ElicitationPropertySchema>(json!({
                "type": "array"
            }))
            .is_err()
        );
        assert!(serde_json::from_value::<ElicitationPropertySchema>(json!({})).is_err());
    }

    #[test]
    fn schema_titled_enum_serialization() {
        let schema = ElicitationSchema::new().property(
            "country",
            StringPropertySchema::new().one_of(vec![
                EnumOption::new("us", "United States").description("Use US English spelling."),
                EnumOption::new("uk", "United Kingdom"),
            ]),
            true,
        );

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["properties"]["country"]["type"], "string");
        let one_of = json["properties"]["country"]["oneOf"].as_array().unwrap();
        assert_eq!(one_of.len(), 2);
        assert_eq!(one_of[0]["const"], "us");
        assert_eq!(one_of[0]["title"], "United States");
        assert_eq!(one_of[0]["description"], "Use US English spelling.");
        assert!(one_of[1].get("description").is_none());

        let roundtripped: ElicitationSchema = serde_json::from_value(json).unwrap();
        if let ElicitationPropertySchema::String(s) =
            roundtripped.properties.get("country").unwrap()
        {
            let one_of = s.one_of.as_ref().unwrap();
            assert_eq!(one_of.len(), 2);
            assert_eq!(
                one_of[0].description.as_deref(),
                Some("Use US English spelling.")
            );
            assert!(one_of[1].description.is_none());
        } else {
            panic!("expected String variant");
        }
    }

    #[test]
    fn schema_number_property_serialization() {
        let schema = ElicitationSchema::new().number("rating", 0.0, 5.0, true);

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["properties"]["rating"]["type"], "number");
        assert_eq!(json["properties"]["rating"]["minimum"], 0.0);
        assert_eq!(json["properties"]["rating"]["maximum"], 5.0);

        let roundtripped: ElicitationSchema = serde_json::from_value(json).unwrap();
        if let ElicitationPropertySchema::Number(n) = roundtripped.properties.get("rating").unwrap()
        {
            assert_eq!(n.minimum, Some(0.0));
            assert_eq!(n.maximum, Some(5.0));
        } else {
            panic!("expected Number variant");
        }
    }

    #[test]
    fn schema_string_format_serialization() {
        let schema = ElicitationSchema::new()
            .uri("website", true)
            .date("birthday", true)
            .date_time("updated_at", false);

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["properties"]["website"]["type"], "string");
        assert_eq!(json["properties"]["website"]["format"], "uri");
        assert_eq!(json["properties"]["birthday"]["type"], "string");
        assert_eq!(json["properties"]["birthday"]["format"], "date");
        assert_eq!(json["properties"]["updated_at"]["type"], "string");
        assert_eq!(json["properties"]["updated_at"]["format"], "date-time");

        let required = json["required"].as_array().unwrap();
        assert!(required.contains(&json!("website")));
        assert!(required.contains(&json!("birthday")));
        assert!(!required.contains(&json!("updated_at")));
    }

    #[test]
    fn schema_string_pattern_serialization() {
        let schema = ElicitationSchema::new().property(
            "name",
            StringPropertySchema::new()
                .min_length(1)
                .max_length(64)
                .pattern("^[a-zA-Z_][a-zA-Z0-9_]*$"),
            true,
        );

        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["properties"]["name"]["type"], "string");
        assert_eq!(
            json["properties"]["name"]["pattern"],
            "^[a-zA-Z_][a-zA-Z0-9_]*$"
        );

        let roundtripped: ElicitationSchema = serde_json::from_value(json).unwrap();
        if let ElicitationPropertySchema::String(s) = roundtripped.properties.get("name").unwrap() {
            assert_eq!(s.pattern.as_deref(), Some("^[a-zA-Z_][a-zA-Z0-9_]*$"));
        } else {
            panic!("expected String variant");
        }
    }

    #[test]
    fn schema_property_updates_required_state() {
        let schema = ElicitationSchema::new()
            .string("name", true)
            .email("name", false);

        let json = serde_json::to_value(&schema).unwrap();
        assert!(json.get("required").is_none());
        assert_eq!(json["properties"]["name"]["format"], "email");
    }

    #[test]
    fn schema_defaults_invalid_object_type() {
        let schema = serde_json::from_value::<ElicitationSchema>(json!({
            "type": "array",
            "properties": {
                "name": {
                    "type": "string"
                }
            }
        }))
        .unwrap();

        assert_eq!(schema.type_, ElicitationSchemaType::Object);
        assert!(schema.properties.contains_key("name"));
    }

    #[test]
    fn titled_multi_select_items_reject_one_of() {
        let err = serde_json::from_value::<TitledMultiSelectItems>(json!({
            "oneOf": [
                {
                    "const": "red",
                    "title": "Red"
                }
            ]
        }))
        .unwrap_err();

        assert!(err.to_string().contains("missing field `anyOf`"));
    }

    #[test]
    fn response_accept_rejects_non_object_content() {
        assert!(
            serde_json::from_value::<CreateElicitationResponse>(json!({
                "action": "accept",
                "content": "Alice"
            }))
            .is_err()
        );
    }

    #[test]
    fn response_accept_rejects_nested_object_content() {
        assert!(
            serde_json::from_value::<CreateElicitationResponse>(json!({
                "action": "accept",
                "content": {
                    "profile": {
                        "name": "Alice"
                    }
                }
            }))
            .is_err()
        );
    }

    #[test]
    fn response_accept_allows_primitive_and_string_array_content() {
        let response = CreateElicitationResponse::new(ElicitationAction::Accept(
            ElicitationAcceptAction::new().content(BTreeMap::from([
                ("name".to_string(), ElicitationContentValue::from("Alice")),
                ("age".to_string(), ElicitationContentValue::from(30_i32)),
                ("score".to_string(), ElicitationContentValue::from(9.5_f64)),
                (
                    "subscribed".to_string(),
                    ElicitationContentValue::from(true),
                ),
                (
                    "tags".to_string(),
                    ElicitationContentValue::from(vec!["rust", "acp"]),
                ),
            ])),
        ));

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["action"], "accept");
        assert_eq!(json["content"]["name"], "Alice");
        assert_eq!(json["content"]["age"], 30);
        assert_eq!(json["content"]["score"], 9.5);
        assert_eq!(json["content"]["subscribed"], true);
        assert_eq!(json["content"]["tags"][0], "rust");
        assert_eq!(json["content"]["tags"][1], "acp");
    }
}
