pub use ipc_channel::ipc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IpcHandshake {
    pub requests: ipc::IpcSender<CliRequest>,
    pub responses: ipc::IpcReceiver<CliResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliRequest {
    // The filed is named `path` for compatibility, but now CLI can request
    // opening a path at a certain row and/or column: `some/path:123` and `some/path:123:456`.
    //
    // Since Zed CLI has to be installed separately, there can be situations when old CLI is
    // querying new Zed editors, support both formats by using `String` here and parsing it on Zed side later.
    Open { paths: Vec<String>, wait: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliResponse {
    Ping,
    Stdout { message: String },
    Stderr { message: String },
    Exit { status: i32 },
}

/// When Zed started not as an *.app but as a binary (e.g. local development),
/// there's a possibility to tell it to behave "regularly".
pub const FORCE_CLI_MODE_ENV_VAR_NAME: &str = "ZED_FORCE_CLI_MODE";
