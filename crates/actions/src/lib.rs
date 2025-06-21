//! Actions crate that generates actions.json at build time.
//!
//! This crate uses build.rs to automatically regenerate the actions.json file
//! whenever the crate is built, eliminating the need for manual updates.

use serde::{Deserialize, Serialize};

/// Represents an action definition that can be serialized to/from JSON.
/// This struct is shared between the build script that generates actions.json
/// and other crates that need to read action definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDef {
    pub name: String,
    pub human_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deprecated_aliases: Vec<String>,
}
