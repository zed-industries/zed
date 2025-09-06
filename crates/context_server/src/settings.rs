//! Settings for context servers

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Settings for context server behavior and configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ContextServerSettings {
    /// Default timeout for context server operations in milliseconds
    #[serde(default = "default_timeout")]
    pub default_timeout: u64,

    /// Whether to automatically retry failed connections
    #[serde(default = "default_auto_retry")]
    pub auto_retry: bool,

    /// Maximum number of retry attempts
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

impl Default for ContextServerSettings {
    fn default() -> Self {
        Self {
            default_timeout: default_timeout(),
            auto_retry: default_auto_retry(),
            max_retries: default_max_retries(),
        }
    }
}

fn default_timeout() -> u64 {
    60000 // 60 seconds in milliseconds
}

fn default_auto_retry() -> bool {
    true
}

fn default_max_retries() -> u32 {
    3
}
