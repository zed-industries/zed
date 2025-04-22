use anyhow::Result;
use schemars::{JsonSchema, r#gen::SchemaSettings};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{net::Ipv4Addr, path::Path};

use crate::{TaskTemplate, TaskType, task_template::DebugArgs};

/// Represents the host information of the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
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

impl TcpArgumentsTemplate {
    /// Get the host or fallback to the default host
    pub fn host(&self) -> Ipv4Addr {
        self.host.unwrap_or_else(|| Ipv4Addr::new(127, 0, 0, 1))
    }

    pub fn from_proto(proto: proto::TcpHost) -> Result<Self> {
        Ok(Self {
            port: proto.port.map(|p| p.try_into()).transpose()?,
            host: proto.host.map(|h| h.parse()).transpose()?,
            timeout: proto.timeout,
        })
    }

    pub fn to_proto(&self) -> proto::TcpHost {
        proto::TcpHost {
            port: self.port.map(|p| p.into()),
            host: self.host.map(|h| h.to_string()),
            timeout: self.timeout,
        }
    }
}

/// Represents the attach request information of the debug adapter
#[derive(Default, Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct AttachRequest {
    /// The processId to attach to, if left empty we will show a process picker
    pub process_id: Option<u32>,
}

/// Represents the launch request information of the debug adapter
#[derive(Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct LaunchRequest {
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
pub enum DebugRequest {
    /// Call the `launch` request on the debug adapter
    Launch(LaunchRequest),
    /// Call the `attach` request on the debug adapter
    Attach(AttachRequest),
}

impl From<LaunchRequest> for DebugRequest {
    fn from(launch_config: LaunchRequest) -> Self {
        DebugRequest::Launch(launch_config)
    }
}

impl From<AttachRequest> for DebugRequest {
    fn from(attach_config: AttachRequest) -> Self {
        DebugRequest::Attach(attach_config)
    }
}

impl TryFrom<TaskTemplate> for DebugTaskTemplate {
    type Error = ();

    fn try_from(value: TaskTemplate) -> Result<Self, Self::Error> {
        let TaskType::Debug(debug_args) = value.task_type else {
            return Err(());
        };

        let request = match debug_args.request {
            crate::DebugArgsRequest::Launch => DebugRequest::Launch(LaunchRequest {
                program: value.command,
                cwd: value.cwd.map(PathBuf::from),
                args: value.args,
            }),
            crate::DebugArgsRequest::Attach(attach_config) => DebugRequest::Attach(attach_config),
        };

        Ok(DebugTaskTemplate {
            locator: debug_args.locator,
            definition: DebugTaskDefinition {
                adapter: debug_args.adapter,
                request,
                label: value.label,
                initialize_args: debug_args.initialize_args,
                tcp_connection: debug_args.tcp_connection,
                stop_on_entry: debug_args.stop_on_entry,
            },
        })
    }
}

impl DebugTaskTemplate {
    /// Translate from debug definition to a task template
    pub fn to_zed_format(self) -> TaskTemplate {
        let (command, cwd, request) = match self.definition.request {
            DebugRequest::Launch(launch_config) => (
                launch_config.program,
                launch_config
                    .cwd
                    .map(|cwd| cwd.to_string_lossy().to_string()),
                crate::task_template::DebugArgsRequest::Launch,
            ),
            DebugRequest::Attach(attach_config) => (
                "".to_owned(),
                None,
                crate::task_template::DebugArgsRequest::Attach(attach_config),
            ),
        };

        let task_type = TaskType::Debug(DebugArgs {
            adapter: self.definition.adapter,
            request,
            initialize_args: self.definition.initialize_args,
            locator: self.locator,
            tcp_connection: self.definition.tcp_connection,
            stop_on_entry: self.definition.stop_on_entry,
        });

        let label = self.definition.label.clone();

        TaskTemplate {
            label,
            command,
            args: vec![],
            task_type,
            cwd,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugTaskTemplate {
    pub locator: Option<String>,
    #[serde(flatten)]
    pub definition: DebugTaskDefinition,
}

/// This struct represent a user created debug task
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugTaskDefinition {
    /// The adapter to run
    pub adapter: String,
    /// The type of request that should be called on the debug adapter
    #[serde(flatten)]
    pub request: DebugRequest,
    /// Name of the debug task
    pub label: String,
    /// Additional initialization arguments to be sent on DAP initialization
    pub initialize_args: Option<serde_json::Value>,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    pub tcp_connection: Option<TcpArgumentsTemplate>,
    /// Whether to tell the debug adapter to stop on entry
    pub stop_on_entry: Option<bool>,
}

impl DebugTaskDefinition {
    pub fn cwd(&self) -> Option<&Path> {
        if let DebugRequest::Launch(config) = &self.request {
            config.cwd.as_deref()
        } else {
            None
        }
    }
    pub fn to_proto(&self) -> proto::DebugTaskDefinition {
        proto::DebugTaskDefinition {
            adapter: self.adapter.clone(),
            request: Some(match &self.request {
                DebugRequest::Launch(config) => {
                    proto::debug_task_definition::Request::DebugLaunchRequest(
                        proto::DebugLaunchRequest {
                            program: config.program.clone(),
                            cwd: config.cwd.as_ref().map(|c| c.to_string_lossy().to_string()),
                            args: config.args.clone(),
                        },
                    )
                }
                DebugRequest::Attach(attach_request) => {
                    proto::debug_task_definition::Request::DebugAttachRequest(
                        proto::DebugAttachRequest {
                            process_id: attach_request.process_id.unwrap_or_default(),
                        },
                    )
                }
            }),
            label: self.label.clone(),
            initialize_args: self.initialize_args.as_ref().map(|v| v.to_string()),
            tcp_connection: self.tcp_connection.as_ref().map(|t| t.to_proto()),
            stop_on_entry: self.stop_on_entry,
        }
    }

    pub fn from_proto(proto: proto::DebugTaskDefinition) -> Result<Self> {
        let request = proto
            .request
            .ok_or_else(|| anyhow::anyhow!("request is required"))?;
        Ok(Self {
            label: proto.label,
            initialize_args: proto.initialize_args.map(|v| v.into()),
            tcp_connection: proto
                .tcp_connection
                .map(TcpArgumentsTemplate::from_proto)
                .transpose()?,
            stop_on_entry: proto.stop_on_entry,
            adapter: proto.adapter.clone(),
            request: match request {
                proto::debug_task_definition::Request::DebugAttachRequest(config) => {
                    DebugRequest::Attach(AttachRequest {
                        process_id: Some(config.process_id),
                    })
                }

                proto::debug_task_definition::Request::DebugLaunchRequest(config) => {
                    DebugRequest::Launch(LaunchRequest {
                        program: config.program,
                        cwd: config.cwd.map(|cwd| cwd.into()),
                        args: config.args,
                    })
                }
            },
        })
    }
}

/// A group of Debug Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct DebugTaskFile(pub Vec<DebugTaskTemplate>);

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

#[cfg(test)]
mod tests {
    use crate::{DebugRequest, LaunchRequest};

    #[test]
    fn test_can_deserialize_non_attach_task() {
        let deserialized: DebugRequest =
            serde_json::from_str(r#"{"program": "cafebabe"}"#).unwrap();
        assert_eq!(
            deserialized,
            DebugRequest::Launch(LaunchRequest {
                program: "cafebabe".to_owned(),
                ..Default::default()
            })
        );
    }
}
