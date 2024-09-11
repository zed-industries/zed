use schemars::{gen::SchemaSettings, JsonSchema};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
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
    pub port: Option<u16>,
    /// The host that the debug adapter is listening too
    pub host: Option<Ipv4Addr>,
    /// The delay in ms between starting and connecting to the debug adapter
    pub delay: Option<u64>,
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
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DebugAdapterKind {
    /// Manually setup starting a debug adapter
    #[default]
    Custom,
    /// Use debugpy
    Python,
    /// Use vscode-php-debug
    PHP,
    /// Use lldb
    Lldb,
}

/// Represents the configuration for the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugAdapterConfig {
    /// Unique id of for the debug adapter,
    /// that will be send with the `initialize` request
    pub kind: DebugAdapterKind,
    /// The type of connection the adapter should use
    /// The type of request that should be called on the debug adapter
    #[serde(default)]
    pub request: DebugRequestType,
    /// The configuration options that are send with the `launch` or `attach` request
    /// to the debug adapter
    // pub request_args: Option<DebugRequestArgs>,
    pub program: String,
    /// The path to the adapter
    pub adapter_path: Option<String>,
}

/// Represents the configuration for the debug adapter that is send with the launch request
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(transparent)]
pub struct DebugRequestArgs {
    pub args: serde_json::Value,
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

#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugTaskDefinition {
    /// Name of the debug tasks
    label: String,
    /// Program to run the debugger on
    program: String,
    /// Launch | Request depending on the session the adapter should be ran as
    #[serde(default)]
    session_type: DebugRequestType,
    /// The adapter to run
    adapter: DebugAdapterKind,
}

impl DebugTaskDefinition {
    fn to_zed_format(self) -> anyhow::Result<TaskTemplate> {
        let command = "".to_string();
        let task_type = TaskType::Debug(DebugAdapterConfig {
            kind: self.adapter,
            request: self.session_type,
            program: self.program,
            adapter_path: None,
        });

        let args: Vec<String> = Vec::new();

        Ok(TaskTemplate {
            label: self.label,
            command,
            args,
            task_type,
            ..Default::default()
        })
    }
}

/// A group of Debug Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
