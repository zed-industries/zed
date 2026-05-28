use gpui::SharedString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// JSON schema for a specific adapter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub struct AdapterSchema {
    /// The adapter name identifier
    pub adapter: SharedString,
    /// The JSON schema for this adapter's configuration
    pub schema: serde_json::Value,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct AdapterSchemas(pub Vec<AdapterSchema>);
