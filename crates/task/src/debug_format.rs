use dap_types::StartDebuggingRequestArguments;
use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use util::ResultExt;

use crate::{TaskTemplate, TaskTemplates, TaskType};

impl Default for DebugConnectionType {
    fn default() -> Self {
        DebugConnectionType::TCP(TCPHost::default())
    }
}

/// Represents the host information of the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct TCPHost {
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

impl TCPHost {
    /// Get the host or fallback to the default host
    pub fn host(&self) -> Ipv4Addr {
        self.host.unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1))
    }
}

/// Represents the attach request information of the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct AttachConfig {
    /// The processId to attach to, if left empty we will show a process picker
    #[serde(default)]
    pub process_id: Option<u32>,
}

/// Represents the launch request information of the debug adapter
#[derive(Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct LaunchConfig {
    /// The program that you trying to debug
    pub program: String,
    /// The current working directory of your project
    pub cwd: Option<PathBuf>,
}

/// Represents the type that will determine which request to call on the debug adapter
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "lowercase", untagged)]
pub enum DebugRequestType {
    /// Call the `launch` request on the debug adapter
    Launch(LaunchConfig),
    /// Call the `attach` request on the debug adapter
    Attach(AttachConfig),
}

/// Represents a request for starting the debugger.
/// Contrary to `DebugRequestType`, `DebugRequestDisposition` is not Serializable.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DebugRequestDisposition {
    /// Debug session configured by the user.
    UserConfigured(DebugRequestType),
    /// Debug session configured by the debug adapter
    ReverseRequest(StartDebuggingRequestArguments),
}

impl DebugRequestDisposition {
    /// Get the current working directory from request if it's a launch request and exits
    pub fn cwd(&self) -> Option<PathBuf> {
        match self {
            Self::UserConfigured(DebugRequestType::Launch(launch_config)) => {
                launch_config.cwd.clone()
            }
            _ => None,
        }
    }
}
/// Represents the configuration for the debug adapter
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DebugAdapterConfig {
    /// Name of the debug task
    pub label: String,
    /// The type of adapter you want to use
    pub adapter: String,
    /// The type of request that should be called on the debug adapter
    pub request: DebugRequestDisposition,
    /// Additional initialization arguments to be sent on DAP initialization
    pub initialize_args: Option<serde_json::Value>,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    pub tcp_connection: Option<TCPHost>,
}

impl From<DebugTaskDefinition> for DebugAdapterConfig {
    fn from(def: DebugTaskDefinition) -> Self {
        Self {
            label: def.label,
            adapter: def.adapter,
            request: DebugRequestDisposition::UserConfigured(def.request),
            initialize_args: def.initialize_args,
            tcp_connection: def.tcp_connection,
        }
    }
}

impl TryFrom<DebugAdapterConfig> for DebugTaskDefinition {
    type Error = ();
    fn try_from(def: DebugAdapterConfig) -> Result<Self, Self::Error> {
        let request = match def.request {
            DebugRequestDisposition::UserConfigured(debug_request_type) => debug_request_type,
            DebugRequestDisposition::ReverseRequest(_) => return Err(()),
        };

        Ok(Self {
            label: def.label,
            adapter: def.adapter,
            request,
            initialize_args: def.initialize_args,
            tcp_connection: def.tcp_connection,
        })
    }
}

impl DebugTaskDefinition {
    /// Translate from debug definition to a task template
    pub fn to_zed_format(self) -> anyhow::Result<TaskTemplate> {
        let command = "".to_string();

        let cwd = if let DebugRequestType::Launch(ref launch) = self.request {
            launch
                .cwd
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned())
        } else {
            None
        };
        let label = self.label.clone();
        let task_type = TaskType::Debug(self);

        Ok(TaskTemplate {
            label,
            command,
            args: vec![],
            task_type,
            cwd,
            ..Default::default()
        })
    }
}
/// Represents the type of the debugger adapter connection
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "connection")]
pub enum DebugConnectionType {
    /// Connect to the debug adapter via TCP
    TCP(TCPHost),
    /// Connect to the debug adapter via STDIO
    STDIO,
}

/// This struct represent a user created debug task
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugTaskDefinition {
    /// The adapter to run
    pub adapter: String,
    /// The type of request that should be called on the debug adapter
    #[serde(flatten)]
    pub request: DebugRequestType,
    /// Name of the debug task
    pub label: String,
    /// Additional initialization arguments to be sent on DAP initialization
    pub initialize_args: Option<serde_json::Value>,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    pub tcp_connection: Option<TCPHost>,
}

/// A group of Debug Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct DebugTaskFile(pub Vec<DebugTaskDefinition>);

impl DebugTaskFile {
    /// Generates JSON schema of Tasks JSON template format.
    pub fn generate_json_schema() -> serde_json_lenient::Value {
        let schema = SchemaSettings::draft07()
            .with(|settings| settings.option_add_null_type = false)
            .into_generator()
            .into_root_schema_for::<Self>();

        serde_json_lenient::to_value(schema).unwrap()
    }
}

impl TryFrom<DebugTaskFile> for TaskTemplates {
    type Error = anyhow::Error;

    fn try_from(value: DebugTaskFile) -> Result<Self, Self::Error> {
        let templates = value
            .0
            .into_iter()
            .filter_map(|debug_definition| debug_definition.to_zed_format().log_err())
            .collect();

        Ok(Self(templates))
    }
}
