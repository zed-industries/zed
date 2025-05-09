use anyhow::Result;
use collections::FxHashMap;
use gpui::SharedString;
use schemars::{JsonSchema, r#gen::SchemaSettings};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{net::Ipv4Addr, path::Path};

use crate::TaskTemplate;

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
#[derive(Default, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct AttachRequest {
    /// The processId to attach to, if left empty we will show a process picker
    pub process_id: Option<u32>,
}

impl<'de> Deserialize<'de> for AttachRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            process_id: Option<u32>,
        }

        let helper = Helper::deserialize(deserializer)?;

        // Skip creating an AttachRequest if process_id is None
        if helper.process_id.is_none() {
            return Err(serde::de::Error::custom("process_id is required"));
        }

        Ok(AttachRequest {
            process_id: helper.process_id,
        })
    }
}

/// Represents the launch request information of the debug adapter
#[derive(Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema, Clone, Debug)]
pub struct LaunchRequest {
    /// The program that you trying to debug
    pub program: String,
    /// The current working directory of your project
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Arguments to pass to a debuggee
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: FxHashMap<String, String>,
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

impl DebugRequest {
    pub fn to_proto(&self) -> proto::DebugRequest {
        match self {
            DebugRequest::Launch(launch_request) => proto::DebugRequest {
                request: Some(proto::debug_request::Request::DebugLaunchRequest(
                    proto::DebugLaunchRequest {
                        program: launch_request.program.clone(),
                        cwd: launch_request
                            .cwd
                            .as_ref()
                            .map(|cwd| cwd.to_string_lossy().into_owned()),
                        args: launch_request.args.clone(),
                        env: launch_request
                            .env
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                    },
                )),
            },
            DebugRequest::Attach(attach_request) => proto::DebugRequest {
                request: Some(proto::debug_request::Request::DebugAttachRequest(
                    proto::DebugAttachRequest {
                        process_id: attach_request
                            .process_id
                            .expect("The process ID to be already filled out."),
                    },
                )),
            },
        }
    }

    pub fn from_proto(val: proto::DebugRequest) -> Result<DebugRequest> {
        let request = val
            .request
            .ok_or_else(|| anyhow::anyhow!("Missing debug request"))?;
        match request {
            proto::debug_request::Request::DebugLaunchRequest(proto::DebugLaunchRequest {
                program,
                cwd,
                args,
                env,
            }) => Ok(DebugRequest::Launch(LaunchRequest {
                program,
                cwd: cwd.map(From::from),
                args,
                env: env.into_iter().collect(),
            })),

            proto::debug_request::Request::DebugAttachRequest(proto::DebugAttachRequest {
                process_id,
            }) => Ok(DebugRequest::Attach(AttachRequest {
                process_id: Some(process_id),
            })),
        }
    }
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

#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum BuildTaskDefinition {
    ByName(SharedString),
    Template {
        #[serde(flatten)]
        task_template: TaskTemplate,
        #[serde(skip)]
        locator_name: Option<SharedString>,
    },
}
/// This struct represent a user created debug task
#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DebugScenario {
    pub adapter: SharedString,
    /// Name of the debug task
    pub label: SharedString,
    /// A task to run prior to spawning the debuggee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildTaskDefinition>,
    #[serde(flatten)]
    pub request: Option<DebugRequest>,
    /// Additional initialization arguments to be sent on DAP initialization
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialize_args: Option<serde_json::Value>,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp_connection: Option<TcpArgumentsTemplate>,
    /// Whether to tell the debug adapter to stop on entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_on_entry: Option<bool>,
}

impl DebugScenario {
    pub fn cwd(&self) -> Option<&Path> {
        if let Some(DebugRequest::Launch(config)) = &self.request {
            config.cwd.as_ref().map(Path::new)
        } else {
            None
        }
    }
}

/// A group of Debug Tasks defined in a JSON file.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct DebugTaskFile(pub Vec<DebugScenario>);

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
    use crate::{DebugRequest, DebugScenario, LaunchRequest};

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

    #[test]
    fn test_empty_scenario_has_none_request() {
        let json = r#"{
            "label": "Build & debug rust",
            "build": "rust",
            "adapter": "CodeLLDB"
        }"#;

        let deserialized: DebugScenario = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.request, None);
    }

    #[test]
    fn test_launch_scenario_deserialization() {
        let json = r#"{
            "label": "Launch program",
            "adapter": "CodeLLDB",
            "program": "target/debug/myapp",
            "args": ["--test"]
        }"#;

        let deserialized: DebugScenario = serde_json::from_str(json).unwrap();
        match deserialized.request {
            Some(DebugRequest::Launch(launch)) => {
                assert_eq!(launch.program, "target/debug/myapp");
                assert_eq!(launch.args, vec!["--test"]);
            }
            _ => panic!("Expected Launch request"),
        }
    }

    #[test]
    fn test_attach_scenario_deserialization() {
        let json = r#"{
            "label": "Attach to process",
            "adapter": "CodeLLDB",
            "process_id": 1234
        }"#;

        let deserialized: DebugScenario = serde_json::from_str(json).unwrap();
        match deserialized.request {
            Some(DebugRequest::Attach(attach)) => {
                assert_eq!(attach.process_id, Some(1234));
            }
            _ => panic!("Expected Attach request"),
        }
    }
}
