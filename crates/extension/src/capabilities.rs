mod download_file_capability;
mod npm_install_package_capability;
mod process_exec_capability;

pub use download_file_capability::*;
pub use npm_install_package_capability::*;
pub use process_exec_capability::*;

use serde::{Deserialize, Serialize};

/// A capability for an extension.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExtensionCapability {
    #[serde(rename = "process:exec")]
    ProcessExec(ProcessExecCapability),
    DownloadFile(DownloadFileCapability),
    #[serde(rename = "npm:install")]
    NpmInstallPackage(NpmInstallPackageCapability),
}
