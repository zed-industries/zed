use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

/// Represents the type that will determine which request to call on the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DebugRequestType {
    /// Call the `launch` request on the debug adapter
    #[default]
    Launch,
    /// Call the `attach` request on the debug adapter
    Attach,
}

/// The Debug adapter to use
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum DebugAdapterKind {
    /// Manually setup starting a debug adapter
    /// The argument within is used to start the DAP
    Custom(CustomArgs),
    /// Use debugpy
    Python(TCPHost),
    /// Use vscode-php-debug
    PHP(TCPHost),
    /// Use vscode-js-debug
    Javascript(TCPHost),
    /// Use lldb
    Lldb,
}

impl DebugAdapterKind {
    /// Returns the display name for the adapter kind
    pub fn display_name(&self) -> &str {
        match self {
            Self::Custom(_) => "Custom",
            Self::Python(_) => "Python",
            Self::PHP(_) => "PHP",
            Self::Javascript(_) => "JavaScript",
            Self::Lldb => "LLDB",
        }
    }
}

/// Custom arguments used to setup a custom debugger
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct CustomArgs {
    /// The connection that a custom debugger should use
    #[serde(flatten)]
    pub connection: DebugConnectionType,
    /// The cli command used to start the debug adapter e.g. `python3`, `node` or the adapter binary
    pub command: String,
    /// The cli arguments used to start the debug adapter
    pub args: Option<Vec<String>>,
    /// The cli envs used to start the debug adapter
    pub envs: Option<HashMap<String, String>>,
}

/// Represents the configuration for the debug adapter
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugAdapterConfig {
    /// Unique id of for the debug adapter,
    /// that will be send with the `initialize` request
    #[serde(flatten)]
    pub kind: DebugAdapterKind,
    /// The type of connection the adapter should use
    /// The type of request that should be called on the debug adapter
    #[serde(default)]
    pub request: DebugRequestType,
    /// The program that you trying to debug
    pub program: Option<String>,
    /// The current working directory of your project
    pub cwd: Option<PathBuf>,
    /// Additional initialization arguments to be sent on DAP initialization
    pub initialize_args: Option<serde_json::Value>,
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

#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugTaskDefinition {
    /// Name of the debug tasks
    label: String,
    /// Program to run the debugger on
    program: Option<String>,
    /// The current working directory of your project
    cwd: Option<String>,
    /// Launch | Request depending on the session the adapter should be ran as
    #[serde(default)]
    session_type: DebugRequestType,
    /// The adapter to run
    adapter: DebugAdapterKind,
    /// Additional initialization arguments to be sent on DAP initialization
    initialize_args: Option<serde_json::Value>,
}

impl DebugTaskDefinition {
    fn to_zed_format(self) -> anyhow::Result<TaskTemplate> {
        let command = "".to_string();
        let task_type = TaskType::Debug(DebugAdapterConfig {
            kind: self.adapter,
            request: self.session_type,
            program: self.program,
            cwd: self.cwd.clone().map(PathBuf::from),
            initialize_args: self.initialize_args,
        });

        let args: Vec<String> = Vec::new();

        Ok(TaskTemplate {
            label: self.label,
            command,
            args,
            task_type,
            cwd: self.cwd,
            ..Default::default()
        })
    }
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
