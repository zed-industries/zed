pub use ipc_channel::ipc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct IpcHandshake {
    pub requests: ipc::IpcSender<CliRequest>,
    pub responses: ipc::IpcReceiver<CliResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliRequest {
    Open { paths: Vec<PathBuf>, wait: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliResponse {
    Stdout { message: String },
    Stderr { message: String },
    Exit { status: i32 },
}
