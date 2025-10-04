use std::sync::Arc;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use settings_macros::MergeFrom;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
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
    /// The capabilities granted to extensions.
    #[serde(default)]
    pub granted_extension_capabilities: Option<Vec<ExtensionCapabilityContent>>,
}

/// A capability for an extension.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExtensionCapabilityContent {
    #[serde(rename = "process:exec")]
    ProcessExec(ProcessExecCapabilityContent),
    DownloadFile(DownloadFileCapabilityContent),
    #[serde(rename = "npm:install")]
    NpmInstallPackage(NpmInstallPackageCapabilityContent),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProcessExecCapabilityContent {
    /// The command to execute.
    pub command: String,
    /// The arguments to pass to the command. Use `*` for a single wildcard argument.
    /// If the last element is `**`, then any trailing arguments are allowed.
    pub args: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DownloadFileCapabilityContent {
    pub host: String,
    pub path: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NpmInstallPackageCapabilityContent {
    pub package: String,
}
