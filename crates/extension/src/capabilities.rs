mod download_file_capability;
mod npm_install_package_capability;
mod process_exec_capability;
mod terminal_close_capability;
mod terminal_create_capability;
mod terminal_input_capability;
mod terminal_read_capability;

pub use download_file_capability::*;
pub use npm_install_package_capability::*;
pub use process_exec_capability::*;
pub use terminal_close_capability::*;
pub use terminal_create_capability::*;
pub use terminal_input_capability::*;
pub use terminal_read_capability::*;

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
    #[serde(rename = "terminal:create")]
    TerminalCreate(TerminalCreateCapability),
    #[serde(rename = "terminal:input")]
    TerminalInput(TerminalInputCapability),
    #[serde(rename = "terminal:read")]
    TerminalRead(TerminalReadCapability),
    #[serde(rename = "terminal:close")]
    TerminalClose(TerminalCloseCapability),
}
