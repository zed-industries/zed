use anyhow::Result;
use gpui::SharedString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a schema for a specific adapter
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

impl AdapterSchemas {
    pub fn generate_json_schema(&self) -> Result<serde_json_lenient::Value> {
        let adapter_conditions = self
            .0
            .iter()
            .map(|adapter_schema| {
                let adapter_name = adapter_schema.adapter.to_string();
                json!({
                    "if": {
                        "properties": {
                            "adapter": { "const": adapter_name }
                        }
                    },
                    "then": adapter_schema.schema
                })
            })
            .collect::<Vec<_>>();

        let schema = serde_json_lenient::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "Debug Adapter Configurations",
            "description": "Configuration for debug adapters. Schema changes based on the selected adapter.",
            "type": "array",
            "items": {
                "type": "object",
                "required": ["adapter", "label"],
                "properties": {
                    "adapter": {
                        "type": "string",
                        "description": "The name of the debug adapter"
                    },
                    "label": {
                        "type": "string",
                        "description": "The name of the debug configuration"
                    },
                },
                "allOf": adapter_conditions
            }
        });

        Ok(serde_json_lenient::to_value(schema)?)
    }
}
