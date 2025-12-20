//! Flutter-specific DAP custom requests.
//!
//! These requests are extensions to the standard Debug Adapter Protocol
//! supported by Flutter's debug adapter. See:
//! https://github.com/flutter/flutter/blob/master/packages/flutter_tools/lib/src/debug_adapters/README.md

use dap_types::requests::Request;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Hot reload request - injects updated source code into the running VM
/// and rebuilds the widget tree without restarting the app.
#[derive(Debug, Clone, Copy)]
pub struct HotReload;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HotReloadArguments {
    /// The reason for the hot reload, typically "manual" or "save"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HotReloadResponse {}

impl Request for HotReload {
    type Arguments = HotReloadArguments;
    type Response = HotReloadResponse;
    const COMMAND: &'static str = "hotReload";
}

/// Hot restart request - updates code and performs a full restart
/// (does not preserve state).
#[derive(Debug, Clone, Copy)]
pub struct HotRestart;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HotRestartArguments {
    /// The reason for the hot restart, typically "manual" or "save"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HotRestartResponse {}

impl Request for HotRestart {
    type Arguments = HotRestartArguments;
    type Response = HotRestartResponse;
    const COMMAND: &'static str = "hotRestart";
}

/// Call service request - invokes a VM service extension.
/// Used for operations like debugDumpRenderTree, toggle debug painting, etc.
#[derive(Debug, Clone, Copy)]
pub struct CallService;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallServiceArguments {
    /// The service method to call (e.g., "ext.flutter.debugPaint")
    pub method: String,
    /// Optional parameters for the service call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CallServiceResponse {
    /// The result from the service call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

impl Request for CallService {
    type Arguments = CallServiceArguments;
    type Response = CallServiceResponse;
    const COMMAND: &'static str = "callService";
}
