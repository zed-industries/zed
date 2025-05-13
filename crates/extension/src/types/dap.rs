use std::{net::Ipv4Addr, path::PathBuf};

use collections::{FxHashMap, HashMap};
use serde::{Deserialize, Serialize};

pub enum DebugRequest {
    Launch(LaunchRequest),
    Attach(AttachRequest),
}

/// Represents the attach request information of the debug adapter
#[derive(Default, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct AttachRequest {
    /// The processId to attach to, if left empty we will show a process picker
    pub process_id: Option<u32>,
}

/// Represents the launch request information of the debug adapter
#[derive(Deserialize, Serialize, Default, PartialEq, Eq, Clone, Debug)]
pub struct LaunchRequest {
    /// The program that you trying to debug
    pub program: String,
    /// The current working directory of your project
    #[serde(default)]
    pub cwd: Option<String>,
    /// Arguments to pass to a debuggee
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: FxHashMap<String, String>,
}

/// Represents the host information of the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct TcpArgumentsTemplate {
    /// The port that the debug adapter is listening on
    ///
    /// Default: We will try to find an open port
    pub port: Option<u16>,
    /// The host that the debug adapter is listening too
    ///
    /// Default: 127.0.0.1
    pub host: Option<Ipv4Addr>,
    /// The max amount of time in milliseconds to connect to a tcp DAP before returning an error
    ///
    /// Default: 2000ms
    pub timeout: Option<u64>,
}

pub struct DebugTaskDefinition {
    pub label: String,
    pub adapter: String,
    pub request: DebugRequest,
    /// Additional initialization arguments to be sent on DAP initialization
    pub initialize_args: Option<serde_json::Value>,
    /// Whether to tell the debug adapter to stop on entry
    pub stop_on_entry: Option<bool>,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new debug adapter process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    pub tcp_connection: Option<TcpArgumentsTemplate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TcpArguments {
    pub host: Ipv4Addr,
    pub port: u16,
    pub timeout: Option<u64>,
}

/// Created from a [DebugTaskDefinition], this struct describes how to spawn the debugger to create a previously-configured debug session.
#[derive(Debug, Clone, PartialEq)]
pub struct DebugAdapterBinary {
    pub command: String,
    pub arguments: Vec<String>,
    pub envs: HashMap<String, String>,
    pub cwd: Option<PathBuf>,
    pub connection: Option<TcpArguments>,
    pub request_args: StartDebuggingRequestArguments,
}

/// Indicates whether the new debug session should be started with a `launch` or `attach` request.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum StartDebuggingRequestArgumentsRequest {
    #[serde(rename = "launch")]
    Launch,
    #[serde(rename = "attach")]
    Attach,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct StartDebuggingRequestArguments {
    pub configuration: serde_json::Value,
    pub request: StartDebuggingRequestArgumentsRequest,
}
