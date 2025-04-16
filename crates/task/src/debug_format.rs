use dap_types::StartDebuggingRequestArguments;
use schemars::{JsonSchema, r#gen::SchemaSettings};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::path::PathBuf;
use util::ResultExt;

use crate::{TaskTemplate, TaskTemplates, TaskType, task_template::DebugArgs};

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
    pub process_id: Option<u32>,
}

/// Represents the launch request information of the debug adapter
#[derive(Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct LaunchConfig {
    /// The program that you trying to debug
    pub program: String,
    /// The current working directory of your project
    pub cwd: Option<PathBuf>,
    /// Arguments to pass to a debuggee
    #[serde(default)]
    pub args: Vec<String>,
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

impl From<LaunchConfig> for DebugRequestType {
    fn from(launch_config: LaunchConfig) -> Self {
        DebugRequestType::Launch(launch_config)
    }
}

impl From<AttachConfig> for DebugRequestType {
    fn from(attach_config: AttachConfig) -> Self {
        DebugRequestType::Attach(attach_config)
    }
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

impl TryFrom<TaskTemplate> for DebugTaskDefinition {
    type Error = ();

    fn try_from(value: TaskTemplate) -> Result<Self, Self::Error> {
        let TaskType::Debug(debug_args) = value.task_type else {
            return Err(());
        };

        let request = match debug_args.request {
            crate::DebugArgsRequest::Launch => DebugRequestType::Launch(LaunchConfig {
                program: value.command,
                cwd: value.cwd.map(PathBuf::from),
                args: value.args,
            }),
            crate::DebugArgsRequest::Attach(attach_config) => {
                DebugRequestType::Attach(attach_config)
            }
        };

        Ok(DebugTaskDefinition {
            adapter: debug_args.adapter,
            request,
            label: value.label,
            initialize_args: debug_args.initialize_args,
            tcp_connection: debug_args.tcp_connection,
            locator: debug_args.locator,
            stop_on_entry: debug_args.stop_on_entry,
        })
    }
}

impl DebugTaskDefinition {
    /// Translate from debug definition to a task template
    pub fn to_zed_format(self) -> anyhow::Result<TaskTemplate> {
        let (command, cwd, request) = match self.request {
            DebugRequestType::Launch(launch_config) => (
                launch_config.program,
                launch_config
                    .cwd
                    .map(|cwd| cwd.to_string_lossy().to_string()),
                crate::task_template::DebugArgsRequest::Launch,
            ),
            DebugRequestType::Attach(attach_config) => (
                "".to_owned(),
                None,
                crate::task_template::DebugArgsRequest::Attach(attach_config),
            ),
        };

        let task_type = TaskType::Debug(DebugArgs {
            adapter: self.adapter,
            request,
            initialize_args: self.initialize_args,
            locator: self.locator,
            tcp_connection: self.tcp_connection,
            stop_on_entry: self.stop_on_entry,
        });

        let label = self.label.clone();

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
    /// Locator to use
    /// -- cargo
    pub locator: Option<String>,
    /// Whether to tell the debug adapter to stop on entry
    pub stop_on_entry: Option<bool>,
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

#[cfg(test)]
mod tests {
    use crate::{DebugRequestType, LaunchConfig};

    #[test]
    fn test_can_deserialize_non_attach_task() {
        let deserialized: DebugRequestType =
            serde_json::from_str(r#"{"program": "cafebabe"}"#).unwrap();
        assert_eq!(
            deserialized,
            DebugRequestType::Launch(LaunchConfig {
                program: "cafebabe".to_owned(),
                ..Default::default()
            })
        );
    }
}
