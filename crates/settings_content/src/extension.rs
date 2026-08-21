use std::sync::Arc;

use collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// Settings for installing and updating extensions.
#[with_fallible_options]
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
    /// The extensions that should be automatically updated by Zed.
    ///
    /// Default: {}
    #[serde(default)]
    pub auto_update_extensions: HashMap<Arc<str>, bool>,
    /// The capabilities granted to extensions.
    pub granted_extension_capabilities: Option<Vec<ExtensionCapabilityContent>>,
}

impl ExtensionSettingsContent {
    /// The Zed default values for this type.
    pub fn defaults() -> Self {
        Self {
            auto_install_extensions: HashMap::from_iter([(Arc::from("html"), true)]),
            auto_update_extensions: HashMap::default(),
            granted_extension_capabilities: Some(vec![
                ExtensionCapabilityContent::ProcessExec {
                    command: String::from("*"),
                    args: vec![String::from("**")],
                },
                ExtensionCapabilityContent::DownloadFile {
                    host: String::from("*"),
                    path: vec![String::from("**")],
                },
                ExtensionCapabilityContent::NpmInstallPackage {
                    package: String::from("*"),
                },
            ]),
        }
    }
}

/// A capability for an extension.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExtensionCapabilityContent {
    /// The capability to execute a process.
    #[serde(rename = "process:exec")]
    ProcessExec {
        /// The command to execute.
        command: String,
        /// The arguments to pass to the command. Use `*` for a single wildcard argument.
        /// If the last element is `**`, then any trailing arguments are allowed.
        args: Vec<String>,
    },
    /// The capability to download a file.
    DownloadFile {
        /// The host from which files may be downloaded.
        host: String,
        /// The allowed path components. Use `*` for a single wildcard component.
        /// If the last element is `**`, then any trailing components are allowed.
        path: Vec<String>,
    },
    /// The capability to install an npm package.
    #[serde(rename = "npm:install")]
    NpmInstallPackage {
        /// The package to install. Use `*` to allow any package.
        package: String,
    },
}
