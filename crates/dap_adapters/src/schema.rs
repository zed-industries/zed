use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonDebugConfiguration {
    /// Name of the module to be debugged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,

    /// Absolute path to the program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,

    /// Code to execute in string form. Example: "import debugpy;print(debugpy.__version__)"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Path python executable and interpreter arguments. Example: ["/usr/bin/python", "-E"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python: Option<Vec<String>>,

    /// Command line arguments passed to the program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Sets where to launch the debug target. Default is "integratedTerminal".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<ConsoleType>,

    /// Absolute path to the working directory of the program being debugged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,

    /// Environment variables defined as a key value pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// When true enables Django templates. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub django: Option<bool>,

    /// When true enables debugging of gevent monkey-patched code. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gevent: Option<bool>,

    /// When true enables Jinja2 template debugging (e.g. Flask). Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jinja: Option<bool>,

    /// When true debug only user-written code. To debug standard library or anything outside of "cwd" use false. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub just_my_code: Option<bool>,

    /// When true enables logging of debugger events to a log file(s). Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_to_file: Option<bool>,

    /// Map of local and remote paths. Example: [{"localRoot": "local path", "remoteRoot": "remote path"}, ...]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_mappings: Option<Vec<PathMapping>>,

    /// When true enables debugging Pyramid applications. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pyramid: Option<bool>,

    /// When true redirects output to debug console. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_output: Option<bool>,

    /// Shows return value of functions when stepping. The return value is added to the response to Variables Request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_return_value: Option<bool>,

    /// When true debugger stops at first line of user code. When false debugger does not stop until breakpoint, exception or pause.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_on_entry: Option<bool>,

    /// When true enables debugging multiprocess applications. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_process: Option<bool>,

    /// When true runs program under elevated permissions (on Unix). Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sudo: Option<bool>,

    /// The name of the debug configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// A task to run prior to spawning the debuggee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildTask>,

    /// TCP connection information for connecting to an externally started debug adapter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_connection: Option<TcpConnectionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsoleType {
    #[serde(rename = "internalConsole")]
    InternalConsole,
    #[serde(rename = "integratedTerminal")]
    IntegratedTerminal,
    #[serde(rename = "externalTerminal")]
    ExternalTerminal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathMapping {
    pub local_root: String,
    pub remote_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTask {
    String(String),
    Object(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConnectionInfo {
    /// The port that the debug adapter is listening on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// The host that the debug adapter is listening to (e.g. 127.0.0.1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// Timeout in milliseconds to connect to the debug adapter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

impl PythonDebugConfiguration {
    /// Validates that the configuration has at least one of the required fields
    pub fn is_valid(&self) -> bool {
        self.module.is_some() || self.program.is_some() || self.code.is_some()
    }
}
