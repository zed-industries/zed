use std::sync::Arc;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};
use util::serde::default_true;

#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ExtensionSettingsContent {
    /// The extensions that should be automatically installed by Zed.
    ///
    /// This is used to make functionality provided by extensions (e.g., language support)
    /// available out-of-the-box.
    ///
    /// Default: { "html": true }
    #[serde(default)]
    pub auto_install_extensions: HashMap<Arc<str>, bool>,
    #[serde(default)]
    pub auto_update_extensions: HashMap<Arc<str>, bool>,
    /// Whether Zed suggests installing extensions for the languages of the files you open.
    ///
    /// Default: true
    #[serde(default = "default_true")]
    pub extension_suggestions: bool,
    /// The capabilities granted to extensions.
    pub granted_extension_capabilities: Option<Vec<ExtensionCapabilityContent>>,
}

impl Default for ExtensionSettingsContent {
    fn default() -> Self {
        Self {
            auto_install_extensions: HashMap::default(),
            auto_update_extensions: HashMap::default(),
            extension_suggestions: true,
            granted_extension_capabilities: None,
        }
    }
}

/// A capability for an extension.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExtensionCapabilityContent {
    #[serde(rename = "process:exec")]
    ProcessExec {
        /// The command to execute.
        command: String,
        /// The arguments to pass to the command. Use `*` for a single wildcard argument.
        /// If the last element is `**`, then any trailing arguments are allowed.
        args: Vec<String>,
    },
    DownloadFile {
        host: String,
        path: Vec<String>,
    },
    #[serde(rename = "npm:install")]
    NpmInstallPackage {
        package: String,
    },
}
