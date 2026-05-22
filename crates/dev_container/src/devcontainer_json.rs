use std::{collections::HashMap, fmt::Display, path::Path, sync::Arc};

use crate::{command_json::CommandRunner, devcontainer_api::DevContainerError};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json_lenient::Value;
use util::command::Command;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum ForwardPort {
    Number(u16),
    String(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PortAttributeProtocol {
    Https,
    Http,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum OnAutoForward {
    #[default]
    Notify,
    OpenBrowser,
    OpenBrowserOnce,
    OpenPreview,
    Silent,
    Ignore,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PortAttributes {
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    on_auto_forward: OnAutoForward,
    #[serde(default)]
    elevate_if_needed: bool,
    #[serde(default)]
    require_local_port: bool,
    #[serde(default)]
    protocol: Option<PortAttributeProtocol>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum UserEnvProbe {
    None,
    InteractiveShell,
    LoginShell,
    LoginInteractiveShell,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MountDefinition {
    #[serde(default)]
    pub(crate) source: Option<String>,
    pub(crate) target: String,
    #[serde(rename = "type")]
    pub(crate) mount_type: Option<String>,
}

impl Display for MountDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mount_type = self.mount_type.clone().unwrap_or_else(|| {
            if let Some(source) = &self.source {
                if source.starts_with('/')
                    || source.starts_with("\\\\")
                    || source.get(1..3) == Some(":\\")
                    || source.get(1..3) == Some(":/")
                {
                    return "bind".to_string();
                }
            }
            "volume".to_string()
        });
        write!(f, "type={}", mount_type)?;
        if let Some(source) = &self.source {
            write!(f, ",source={}", source)?;
        }
        write!(f, ",target={},consistency=cached", self.target)
    }
}

/// Represents the value associated with a feature ID in the `features` map of devcontainer.json.
///
/// Per the spec, the value can be:
/// - A boolean (`true` to enable with defaults)
/// - A string (shorthand for `{"version": "<value>"}`)
/// - An object mapping option names to string or boolean values
///
/// See: https://containers.dev/implementors/features/#devcontainerjson-properties
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum FeatureOptions {
    Bool(bool),
    String(String),
    Options(HashMap<String, FeatureOptionValue>),
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub(crate) enum FeatureOptionValue {
    Bool(bool),
    String(String),
}
impl std::fmt::Display for FeatureOptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureOptionValue::Bool(b) => write!(f, "{}", b),
            FeatureOptionValue::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, Serialize, Eq, PartialEq, Default)]
pub(crate) struct ZedCustomizationsWrapper {
    pub(crate) zed: ZedCustomization,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct ZedCustomization {
    #[serde(default)]
    pub(crate) extensions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContainerBuild {
    pub(crate) dockerfile: String,
    pub(crate) context: Option<String>,
    pub(crate) args: Option<HashMap<String, String>>,
    pub(crate) options: Option<Vec<String>>,
    pub(crate) target: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_array")]
    pub(crate) cache_from: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
struct LifecycleScriptInternal {
    command: Option<String>,
    args: Vec<String>,
}

impl LifecycleScriptInternal {
    fn from_args(args: Vec<String>) -> Self {
        let command = args.get(0).map(|a| a.to_string());
        let remaining = args.iter().skip(1).map(|a| a.to_string()).collect();
        Self {
            command,
            args: remaining,
        }
    }
}

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct LifecycleScript {
    scripts: HashMap<String, LifecycleScriptInternal>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HostRequirements {
    cpus: Option<u16>,
    memory: Option<String>,
    storage: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum LifecycleCommand {
    InitializeCommand,
    OnCreateCommand,
    UpdateContentCommand,
    PostCreateCommand,
    PostStartCommand,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DevContainerBuildType {
    Image(String),
    Dockerfile(ContainerBuild),
    DockerCompose,
    None,
}
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainer {
    pub(crate) image: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) remote_user: Option<String>,
    pub(crate) forward_ports: Option<Vec<ForwardPort>>,
    pub(crate) ports_attributes: Option<HashMap<String, PortAttributes>>,
    pub(crate) other_ports_attributes: Option<PortAttributes>,
    pub(crate) container_env: Option<HashMap<String, String>>,
    pub(crate) remote_env: Option<HashMap<String, String>>,
    pub(crate) container_user: Option<String>,
    #[serde(rename = "updateRemoteUserUID")]
    pub(crate) update_remote_user_uid: Option<bool>,
    user_env_probe: Option<UserEnvProbe>,
    override_command: Option<bool>,
    shutdown_action: Option<ShutdownAction>,
    init: Option<bool>,
    pub(crate) privileged: Option<bool>,
    cap_add: Option<Vec<String>>,
    security_opt: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_mount_definitions")]
    pub(crate) mounts: Option<Vec<MountDefinition>>,
    pub(crate) features: Option<HashMap<String, FeatureOptions>>,
    pub(crate) override_feature_install_order: Option<Vec<String>>,
    pub(crate) customizations: Option<ZedCustomizationsWrapper>,
    pub(crate) build: Option<ContainerBuild>,
    #[serde(default, deserialize_with = "deserialize_app_port")]
    pub(crate) app_port: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_mount_definition")]
    pub(crate) workspace_mount: Option<MountDefinition>,
    pub(crate) workspace_folder: Option<String>,
    pub(crate) run_args: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_string_or_array")]
    pub(crate) docker_compose_file: Option<Vec<String>>,
    pub(crate) service: Option<String>,
    run_services: Option<Vec<String>>,
    pub(crate) initialize_command: Option<LifecycleScript>,
    pub(crate) on_create_command: Option<LifecycleScript>,
    pub(crate) update_content_command: Option<LifecycleScript>,
    pub(crate) post_create_command: Option<LifecycleScript>,
    pub(crate) post_start_command: Option<LifecycleScript>,
    pub(crate) post_attach_command: Option<LifecycleScript>,
    wait_for: Option<LifecycleCommand>,
    host_requirements: Option<HostRequirements>,
}

pub(crate) fn deserialize_devcontainer_json_to_value(
    json: &str,
) -> Result<serde_json_lenient::Value, DevContainerError> {
    serde_json_lenient::from_str(json).map_err(|e| {
        log::error!("Unable to deserialize json values: {e}");
        DevContainerError::DevContainerParseFailed
    })
}

pub(crate) fn deserialize_devcontainer_json_from_value(
    json: serde_json_lenient::Value,
) -> Result<DevContainer, DevContainerError> {
    serde_json_lenient::from_value(json).map_err(|e| {
        log::error!("Unable to deserialize devcontainer from json values: {e}");
        DevContainerError::DevContainerParseFailed
    })
}

pub(crate) fn deserialize_devcontainer_json(json: &str) -> Result<DevContainer, DevContainerError> {
    deserialize_devcontainer_json_to_value(json).and_then(deserialize_devcontainer_json_from_value)
}

impl DevContainer {
    pub(crate) fn build_type(&self) -> DevContainerBuildType {
        if let Some(image) = &self.image {
            DevContainerBuildType::Image(image.clone())
        } else if self.docker_compose_file.is_some() {
            DevContainerBuildType::DockerCompose
        } else if let Some(build) = &self.build {
            DevContainerBuildType::Dockerfile(build.clone())
        } else {
            DevContainerBuildType::None
        }
    }

    pub(crate) fn validate_devcontainer_contents(&self) -> Result<(), DevContainerError> {
        match self.build_type() {
            DevContainerBuildType::Image(_) => Ok(()),
            DevContainerBuildType::Dockerfile(_) => {
                if (self.workspace_folder.is_some() && self.workspace_mount.is_none())
                    || (self.workspace_folder.is_none() && self.workspace_mount.is_some())
                {
                    return Err(DevContainerError::DevContainerValidationFailed(
                        "workspaceMount and workspaceFolder must both be defined, or neither defined"
                            .to_string(),
                    ));
                }
                Ok(())
            }
            DevContainerBuildType::DockerCompose => {
                if self.service.is_none() {
                    return Err(DevContainerError::DevContainerValidationFailed(
                        "must specify a connecting service for docker-compose".to_string(),
                    ));
                }
                Ok(())
            }
            DevContainerBuildType::None => Ok(()),
        }
    }
}

// Custom deserializer that parses the entire customizations object as a
// serde_json_lenient::Value first, then extracts the "zed" portion.
// This avoids a bug in serde_json_lenient's `ignore_value` codepath which
// does not handle trailing commas in skipped values.
impl<'de> Deserialize<'de> for ZedCustomizationsWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let zed = value
            .get("zed")
            .map(|zed_value| serde_json_lenient::from_value::<ZedCustomization>(zed_value.clone()))
            .transpose()
            .map_err(serde::de::Error::custom)?
            .unwrap_or_default();
        Ok(ZedCustomizationsWrapper { zed })
    }
}

impl LifecycleScript {
    fn from_map(args: HashMap<String, Vec<String>>) -> Self {
        Self {
            scripts: args
                .into_iter()
                .map(|(k, v)| (k, LifecycleScriptInternal::from_args(v)))
                .collect(),
        }
    }
    fn from_str(args: &str) -> Self {
        let script: Vec<String> = args.split(" ").map(|a| a.to_string()).collect();

        Self::from_args(script)
    }
    fn from_args(args: Vec<String>) -> Self {
        Self::from_map(HashMap::from([("default".to_string(), args)]))
    }
    pub fn script_commands(&self) -> HashMap<String, Command> {
        self.scripts
            .iter()
            .filter_map(|(k, v)| {
                if let Some(inner_command) = &v.command {
                    let mut command = Command::new(inner_command);
                    command.args(&v.args);
                    Some((k.clone(), command))
                } else {
                    log::warn!(
                        "Lifecycle script command {k}, value {:?} has no program to run. Skipping",
                        v
                    );
                    None
                }
            })
            .collect()
    }

    pub async fn run(
        &self,
        command_runnder: &Arc<dyn CommandRunner>,
        working_directory: &Path,
    ) -> Result<(), DevContainerError> {
        for (command_name, mut command) in self.script_commands() {
            log::debug!("Running script {command_name}");

            command.current_dir(working_directory);

            let output = command_runnder
                .run_command(&mut command)
                .await
                .map_err(|e| {
                    log::error!("Error running command {command_name}: {e}");
                    DevContainerError::CommandFailed(command_name.clone())
                })?;
            if !output.status.success() {
                let std_err = String::from_utf8_lossy(&output.stderr);
                log::error!(
                    "Command {command_name} produced a non-successful output. StdErr: {std_err}"
                );
            }
            let std_out = String::from_utf8_lossy(&output.stdout);
            log::debug!("Command {command_name} output:\n {std_out}");
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for LifecycleScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct LifecycleScriptVisitor;

        impl<'de> Visitor<'de> for LifecycleScriptVisitor {
            type Value = LifecycleScript;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string, an array of strings, or a map of arrays")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LifecycleScript::from_str(value))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut array = Vec::new();
                while let Some(elem) = seq.next_element()? {
                    array.push(elem);
                }
                Ok(LifecycleScript::from_args(array))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut result = HashMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    let value: Value = map.next_value()?;
                    let script_args = match value {
                        Value::String(s) => {
                            s.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()
                        }
                        Value::Array(arr) => {
                            let strings: Vec<String> = arr
                                .into_iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                            strings
                        }
                        _ => continue,
                    };
                    result.insert(key, script_args);
                }
                Ok(LifecycleScript::from_map(result))
            }
        }

        deserializer.deserialize_any(LifecycleScriptVisitor)
    }
}

fn deserialize_mount_definition<'de, D>(
    deserializer: D,
) -> Result<Option<MountDefinition>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MountItem {
        Object(MountDefinition),
        String(String),
    }

    let item = MountItem::deserialize(deserializer)?;

    let mount = match item {
        MountItem::Object(mount) => mount,
        MountItem::String(s) => {
            let mut source = None;
            let mut target = None;
            let mut mount_type = None;

            for part in s.split(',') {
                let part = part.trim();
                if let Some((key, value)) = part.split_once('=') {
                    match key.trim() {
                        "source" => source = Some(value.trim().to_string()),
                        "target" => target = Some(value.trim().to_string()),
                        "type" => mount_type = Some(value.trim().to_string()),
                        _ => {} // Ignore unknown keys
                    }
                }
            }

            let target = target
                .ok_or_else(|| D::Error::custom(format!("mount string missing 'target': {}", s)))?;

            MountDefinition {
                source,
                target,
                mount_type,
            }
        }
    };

    Ok(Some(mount))
}

fn deserialize_mount_definitions<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<MountDefinition>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MountItem {
        Object(MountDefinition),
        String(String),
    }

    let items = Vec::<MountItem>::deserialize(deserializer)?;
    let mut mounts = Vec::new();

    for item in items {
        match item {
            MountItem::Object(mount) => mounts.push(mount),
            MountItem::String(s) => {
                let mut source = None;
                let mut target = None;
                let mut mount_type = None;

                for part in s.split(',') {
                    let part = part.trim();
                    if let Some((key, value)) = part.split_once('=') {
                        match key.trim() {
                            "source" => source = Some(value.trim().to_string()),
                            "target" => target = Some(value.trim().to_string()),
                            "type" => mount_type = Some(value.trim().to_string()),
                            _ => {} // Ignore unknown keys
                        }
                    }
                }

                let target = target.ok_or_else(|| {
                    D::Error::custom(format!("mount string missing 'target': {}", s))
                })?;

                mounts.push(MountDefinition {
                    source,
                    target,
                    mount_type,
                });
            }
        }
    }

    Ok(Some(mounts))
}

fn deserialize_app_port<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(u32),
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AppPort {
        Array(Vec<StringOrInt>),
        Single(StringOrInt),
    }

    fn normalize_port(value: StringOrInt) -> String {
        match value {
            StringOrInt::String(s) => {
                if s.contains(':') {
                    s
                } else {
                    format!("{s}:{s}")
                }
            }
            StringOrInt::Int(n) => format!("{n}:{n}"),
        }
    }

    match AppPort::deserialize(deserializer)? {
        AppPort::Single(value) => Ok(vec![normalize_port(value)]),
        AppPort::Array(values) => Ok(values.into_iter().map(normalize_port).collect()),
    }
}

fn deserialize_string_or_array<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrArray {
        String(String),
        Array(Vec<String>),
    }

    match StringOrArray::deserialize(deserializer)? {
        StringOrArray::String(s) => Ok(Some(vec![s])),
        StringOrArray::Array(b) => Ok(Some(b)),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        devcontainer_api::DevContainerError,
        devcontainer_json::{
            ContainerBuild, DevContainer, DevContainerBuildType, FeatureOptions, ForwardPort,
            HostRequirements, LifecycleCommand, LifecycleScript, MountDefinition, OnAutoForward,
            PortAttributeProtocol, PortAttributes, ShutdownAction, UserEnvProbe, ZedCustomization,
            ZedCustomizationsWrapper, deserialize_devcontainer_json,
        },
    };

    #[test]
    fn should_deserialize_customizations_with_unknown_keys() {
        let json_with_other_customizations = r#"
            {
                "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
                "customizations": {
                  "vscode": {
                    "extensions": [
                      "dbaeumer.vscode-eslint",
                      "GitHub.vscode-pull-request-github",
                    ],
                  },
                  "zed": {
                    "extensions": ["vue", "ruby"],
                  },
                  "codespaces": {
                    "repositories": {
                      "devcontainers/features": {
                        "permissions": {
                          "contents": "write",
                          "workflows": "write",
                        },
                      },
                    },
                  },
                },
            }
        "#;

        let result = deserialize_devcontainer_json(json_with_other_customizations);

        assert!(
            result.is_ok(),
            "Should ignore unknown customization keys, but got: {:?}",
            result.err()
        );
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer.customizations,
            Some(ZedCustomizationsWrapper {
                zed: ZedCustomization {
                    extensions: vec!["vue".to_string(), "ruby".to_string()]
                }
            })
        );
    }

    #[test]
    fn should_deserialize_customizations_without_zed_key() {
        let json_without_zed = r#"
            {
                "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
                "customizations": {
                    "vscode": {
                        "extensions": ["dbaeumer.vscode-eslint"]
                    }
                }
            }
        "#;

        let result = deserialize_devcontainer_json(json_without_zed);

        assert!(
            result.is_ok(),
            "Should handle missing zed key in customizations, but got: {:?}",
            result.err()
        );
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer.customizations,
            Some(ZedCustomizationsWrapper {
                zed: ZedCustomization { extensions: vec![] }
            })
        );
    }

    #[test]
    fn should_deserialize_simple_devcontainer_json() {
        let given_bad_json = "{ \"image\": 123 }";

        let result = deserialize_devcontainer_json(given_bad_json);

        assert!(result.is_err());
        assert_eq!(
            result.expect_err("err"),
            DevContainerError::DevContainerParseFailed
        );

        let given_image_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
                "name": "myDevContainer",
                "remoteUser": "root",
                "forwardPorts": [
                    "db:5432",
                    3000
                ],
                "portsAttributes": {
                    "3000": {
                        "label": "This Port",
                        "onAutoForward": "notify",
                        "elevateIfNeeded": false,
                        "requireLocalPort": true,
                        "protocol": "https"
                    },
                    "db:5432": {
                        "label": "This Port too",
                        "onAutoForward": "silent",
                        "elevateIfNeeded": true,
                        "requireLocalPort": false,
                        "protocol": "http"
                    }
                },
                "otherPortsAttributes": {
                    "label": "Other Ports",
                    "onAutoForward": "openBrowser",
                    "elevateIfNeeded": true,
                    "requireLocalPort": true,
                    "protocol": "https"
                },
                "updateRemoteUserUID": true,
                "remoteEnv": {
                    "MYVAR1": "myvarvalue",
                    "MYVAR2": "myvarothervalue"
                },
                "initializeCommand": ["echo", "initialize_command"],
                "onCreateCommand": "echo on_create_command",
                "updateContentCommand": {
                    "first": "echo update_content_command",
                    "second": ["echo", "update_content_command"]
                },
                "postCreateCommand": ["echo", "post_create_command"],
                "postStartCommand": "echo post_start_command",
                "postAttachCommand": {
                    "something": "echo post_attach_command",
                    "something1": "echo something else",
                },
                "waitFor": "postStartCommand",
                "userEnvProbe": "loginShell",
                "features": {
              		"ghcr.io/devcontainers/features/aws-cli:1": {},
              		"ghcr.io/devcontainers/features/anaconda:1": {}
               	},
                "overrideFeatureInstallOrder": [
                    "ghcr.io/devcontainers/features/anaconda:1",
                    "ghcr.io/devcontainers/features/aws-cli:1"
                ],
                "hostRequirements": {
                    "cpus": 2,
                    "memory": "8gb",
                    "storage": "32gb",
                    // Note that we're not parsing this currently
                    "gpu": true,
                },
                "appPort": 8081,
                "containerEnv": {
                    "MYVAR3": "myvar3",
                    "MYVAR4": "myvar4"
                },
                "containerUser": "myUser",
                "mounts": [
                    {
                        "source": "/localfolder/app",
                        "target": "/workspaces/app",
                        "type": "volume"
                    }
                ],
                "runArgs": [
                    "-c",
                    "some_command"
                ],
                "shutdownAction": "stopContainer",
                "overrideCommand": true,
                "workspaceFolder": "/workspaces",
                "workspaceMount": "source=/app,target=/workspaces/app,type=bind,consistency=cached",
                "customizations": {
                    "vscode": {
                        // Just confirm that this can be included and ignored
                    },
                    "zed": {
                        "extensions": [
                            "html"
                        ]
                    }
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_image_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer,
            DevContainer {
                image: Some(String::from("mcr.microsoft.com/devcontainers/base:ubuntu")),
                name: Some(String::from("myDevContainer")),
                remote_user: Some(String::from("root")),
                forward_ports: Some(vec![
                    ForwardPort::String("db:5432".to_string()),
                    ForwardPort::Number(3000),
                ]),
                ports_attributes: Some(HashMap::from([
                    (
                        "3000".to_string(),
                        PortAttributes {
                            label: Some("This Port".to_string()),
                            on_auto_forward: OnAutoForward::Notify,
                            elevate_if_needed: false,
                            require_local_port: true,
                            protocol: Some(PortAttributeProtocol::Https)
                        }
                    ),
                    (
                        "db:5432".to_string(),
                        PortAttributes {
                            label: Some("This Port too".to_string()),
                            on_auto_forward: OnAutoForward::Silent,
                            elevate_if_needed: true,
                            require_local_port: false,
                            protocol: Some(PortAttributeProtocol::Http)
                        }
                    )
                ])),
                other_ports_attributes: Some(PortAttributes {
                    label: Some("Other Ports".to_string()),
                    on_auto_forward: OnAutoForward::OpenBrowser,
                    elevate_if_needed: true,
                    require_local_port: true,
                    protocol: Some(PortAttributeProtocol::Https)
                }),
                update_remote_user_uid: Some(true),
                remote_env: Some(HashMap::from([
                    ("MYVAR1".to_string(), "myvarvalue".to_string()),
                    ("MYVAR2".to_string(), "myvarothervalue".to_string())
                ])),
                initialize_command: Some(LifecycleScript::from_args(vec![
                    "echo".to_string(),
                    "initialize_command".to_string()
                ])),
                on_create_command: Some(LifecycleScript::from_str("echo on_create_command")),
                update_content_command: Some(LifecycleScript::from_map(HashMap::from([
                    (
                        "first".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    ),
                    (
                        "second".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    )
                ]))),
                post_create_command: Some(LifecycleScript::from_str("echo post_create_command")),
                post_start_command: Some(LifecycleScript::from_args(vec![
                    "echo".to_string(),
                    "post_start_command".to_string()
                ])),
                post_attach_command: Some(LifecycleScript::from_map(HashMap::from([
                    (
                        "something".to_string(),
                        vec!["echo".to_string(), "post_attach_command".to_string()]
                    ),
                    (
                        "something1".to_string(),
                        vec![
                            "echo".to_string(),
                            "something".to_string(),
                            "else".to_string()
                        ]
                    )
                ]))),
                wait_for: Some(LifecycleCommand::PostStartCommand),
                user_env_probe: Some(UserEnvProbe::LoginShell),
                features: Some(HashMap::from([
                    (
                        "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    ),
                    (
                        "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    )
                ])),
                override_feature_install_order: Some(vec![
                    "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                    "ghcr.io/devcontainers/features/aws-cli:1".to_string()
                ]),
                host_requirements: Some(HostRequirements {
                    cpus: Some(2),
                    memory: Some("8gb".to_string()),
                    storage: Some("32gb".to_string()),
                }),
                app_port: vec!["8081:8081".to_string()],
                container_env: Some(HashMap::from([
                    ("MYVAR3".to_string(), "myvar3".to_string()),
                    ("MYVAR4".to_string(), "myvar4".to_string())
                ])),
                container_user: Some("myUser".to_string()),
                mounts: Some(vec![MountDefinition {
                    source: Some("/localfolder/app".to_string()),
                    target: "/workspaces/app".to_string(),
                    mount_type: Some("volume".to_string()),
                }]),
                run_args: Some(vec!["-c".to_string(), "some_command".to_string()]),
                shutdown_action: Some(ShutdownAction::StopContainer),
                override_command: Some(true),
                workspace_folder: Some("/workspaces".to_string()),
                workspace_mount: Some(MountDefinition {
                    source: Some("/app".to_string()),
                    target: "/workspaces/app".to_string(),
                    mount_type: Some("bind".to_string())
                }),
                customizations: Some(ZedCustomizationsWrapper {
                    zed: ZedCustomization {
                        extensions: vec!["html".to_string()]
                    }
                }),
                ..Default::default()
            }
        );

        assert_eq!(
            devcontainer.build_type(),
            DevContainerBuildType::Image(String::from(
                "mcr.microsoft.com/devcontainers/base:ubuntu"
            ))
        );
    }

    #[test]
    fn should_deserialize_docker_compose_devcontainer_json() {
        let given_docker_compose_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "name": "myDevContainer",
                "remoteUser": "root",
                "forwardPorts": [
                    "db:5432",
                    3000
                ],
                "portsAttributes": {
                    "3000": {
                        "label": "This Port",
                        "onAutoForward": "notify",
                        "elevateIfNeeded": false,
                        "requireLocalPort": true,
                        "protocol": "https"
                    },
                    "db:5432": {
                        "label": "This Port too",
                        "onAutoForward": "silent",
                        "elevateIfNeeded": true,
                        "requireLocalPort": false,
                        "protocol": "http"
                    }
                },
                "otherPortsAttributes": {
                    "label": "Other Ports",
                    "onAutoForward": "openBrowser",
                    "elevateIfNeeded": true,
                    "requireLocalPort": true,
                    "protocol": "https"
                },
                "updateRemoteUserUID": true,
                "remoteEnv": {
                    "MYVAR1": "myvarvalue",
                    "MYVAR2": "myvarothervalue"
                },
                "initializeCommand": ["echo", "initialize_command"],
                "onCreateCommand": "echo on_create_command",
                "updateContentCommand": {
                    "first": "echo update_content_command",
                    "second": ["echo", "update_content_command"]
                },
                "postCreateCommand": ["echo", "post_create_command"],
                "postStartCommand": "echo post_start_command",
                "postAttachCommand": {
                    "something": "echo post_attach_command",
                    "something1": "echo something else",
                },
                "waitFor": "postStartCommand",
                "userEnvProbe": "loginShell",
                "features": {
              		"ghcr.io/devcontainers/features/aws-cli:1": {},
              		"ghcr.io/devcontainers/features/anaconda:1": {}
               	},
                "overrideFeatureInstallOrder": [
                    "ghcr.io/devcontainers/features/anaconda:1",
                    "ghcr.io/devcontainers/features/aws-cli:1"
                ],
                "hostRequirements": {
                    "cpus": 2,
                    "memory": "8gb",
                    "storage": "32gb",
                    // Note that we're not parsing this currently
                    "gpu": true,
                },
                "dockerComposeFile": "docker-compose.yml",
                "service": "myService",
                "runServices": [
                    "myService",
                    "mySupportingService"
                ],
                "workspaceFolder": "/workspaces/thing",
                "shutdownAction": "stopCompose",
                "overrideCommand": true
            }
            "#;
        let result = deserialize_devcontainer_json(given_docker_compose_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer,
            DevContainer {
                name: Some(String::from("myDevContainer")),
                remote_user: Some(String::from("root")),
                forward_ports: Some(vec![
                    ForwardPort::String("db:5432".to_string()),
                    ForwardPort::Number(3000),
                ]),
                ports_attributes: Some(HashMap::from([
                    (
                        "3000".to_string(),
                        PortAttributes {
                            label: Some("This Port".to_string()),
                            on_auto_forward: OnAutoForward::Notify,
                            elevate_if_needed: false,
                            require_local_port: true,
                            protocol: Some(PortAttributeProtocol::Https)
                        }
                    ),
                    (
                        "db:5432".to_string(),
                        PortAttributes {
                            label: Some("This Port too".to_string()),
                            on_auto_forward: OnAutoForward::Silent,
                            elevate_if_needed: true,
                            require_local_port: false,
                            protocol: Some(PortAttributeProtocol::Http)
                        }
                    )
                ])),
                other_ports_attributes: Some(PortAttributes {
                    label: Some("Other Ports".to_string()),
                    on_auto_forward: OnAutoForward::OpenBrowser,
                    elevate_if_needed: true,
                    require_local_port: true,
                    protocol: Some(PortAttributeProtocol::Https)
                }),
                update_remote_user_uid: Some(true),
                remote_env: Some(HashMap::from([
                    ("MYVAR1".to_string(), "myvarvalue".to_string()),
                    ("MYVAR2".to_string(), "myvarothervalue".to_string())
                ])),
                initialize_command: Some(LifecycleScript::from_args(vec![
                    "echo".to_string(),
                    "initialize_command".to_string()
                ])),
                on_create_command: Some(LifecycleScript::from_str("echo on_create_command")),
                update_content_command: Some(LifecycleScript::from_map(HashMap::from([
                    (
                        "first".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    ),
                    (
                        "second".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    )
                ]))),
                post_create_command: Some(LifecycleScript::from_str("echo post_create_command")),
                post_start_command: Some(LifecycleScript::from_args(vec![
                    "echo".to_string(),
                    "post_start_command".to_string()
                ])),
                post_attach_command: Some(LifecycleScript::from_map(HashMap::from([
                    (
                        "something".to_string(),
                        vec!["echo".to_string(), "post_attach_command".to_string()]
                    ),
                    (
                        "something1".to_string(),
                        vec![
                            "echo".to_string(),
                            "something".to_string(),
                            "else".to_string()
                        ]
                    )
                ]))),
                wait_for: Some(LifecycleCommand::PostStartCommand),
                user_env_probe: Some(UserEnvProbe::LoginShell),
                features: Some(HashMap::from([
                    (
                        "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    ),
                    (
                        "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    )
                ])),
                override_feature_install_order: Some(vec![
                    "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                    "ghcr.io/devcontainers/features/aws-cli:1".to_string()
                ]),
                host_requirements: Some(HostRequirements {
                    cpus: Some(2),
                    memory: Some("8gb".to_string()),
                    storage: Some("32gb".to_string()),
                }),
                docker_compose_file: Some(vec!["docker-compose.yml".to_string()]),
                service: Some("myService".to_string()),
                run_services: Some(vec![
                    "myService".to_string(),
                    "mySupportingService".to_string(),
                ]),
                workspace_folder: Some("/workspaces/thing".to_string()),
                shutdown_action: Some(ShutdownAction::StopCompose),
                override_command: Some(true),
                ..Default::default()
            }
        );

        assert_eq!(
            devcontainer.build_type(),
            DevContainerBuildType::DockerCompose
        );
    }

    #[test]
    fn should_deserialize_dockerfile_devcontainer_json() {
        let given_dockerfile_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "name": "myDevContainer",
                "remoteUser": "root",
                "forwardPorts": [
                    "db:5432",
                    3000
                ],
                "portsAttributes": {
                    "3000": {
                        "label": "This Port",
                        "onAutoForward": "notify",
                        "elevateIfNeeded": false,
                        "requireLocalPort": true,
                        "protocol": "https"
                    },
                    "db:5432": {
                        "label": "This Port too",
                        "onAutoForward": "silent",
                        "elevateIfNeeded": true,
                        "requireLocalPort": false,
                        "protocol": "http"
                    }
                },
                "otherPortsAttributes": {
                    "label": "Other Ports",
                    "onAutoForward": "openBrowser",
                    "elevateIfNeeded": true,
                    "requireLocalPort": true,
                    "protocol": "https"
                },
                "updateRemoteUserUID": true,
                "remoteEnv": {
                    "MYVAR1": "myvarvalue",
                    "MYVAR2": "myvarothervalue"
                },
                "initializeCommand": ["echo", "initialize_command"],
                "onCreateCommand": "echo on_create_command",
                "updateContentCommand": {
                    "first": "echo update_content_command",
                    "second": ["echo", "update_content_command"]
                },
                "postCreateCommand": ["echo", "post_create_command"],
                "postStartCommand": "echo post_start_command",
                "postAttachCommand": {
                    "something": "echo post_attach_command",
                    "something1": "echo something else",
                },
                "waitFor": "postStartCommand",
                "userEnvProbe": "loginShell",
                "features": {
              		"ghcr.io/devcontainers/features/aws-cli:1": {},
              		"ghcr.io/devcontainers/features/anaconda:1": {}
               	},
                "overrideFeatureInstallOrder": [
                    "ghcr.io/devcontainers/features/anaconda:1",
                    "ghcr.io/devcontainers/features/aws-cli:1"
                ],
                "hostRequirements": {
                    "cpus": 2,
                    "memory": "8gb",
                    "storage": "32gb",
                    // Note that we're not parsing this currently
                    "gpu": true,
                },
                "appPort": 8081,
                "containerEnv": {
                    "MYVAR3": "myvar3",
                    "MYVAR4": "myvar4"
                },
                "containerUser": "myUser",
                "mounts": [
                    {
                        "source": "/localfolder/app",
                        "target": "/workspaces/app",
                        "type": "volume"
                    },
                    "source=dev-containers-cli-bashhistory,target=/home/node/commandhistory",
                ],
                "runArgs": [
                    "-c",
                    "some_command"
                ],
                "shutdownAction": "stopContainer",
                "overrideCommand": true,
                "workspaceFolder": "/workspaces",
                "workspaceMount": "source=/folder,target=/workspace,type=bind,consistency=cached",
                "build": {
                   	"dockerfile": "DockerFile",
                   	"context": "..",
                   	"args": {
                   	    "MYARG": "MYVALUE"
                   	},
                   	"options": [
                   	    "--some-option",
                   	    "--mount"
                   	],
                   	"target": "development",
                   	"cacheFrom": "some_image"
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_dockerfile_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");
        assert_eq!(
            devcontainer,
            DevContainer {
                name: Some(String::from("myDevContainer")),
                remote_user: Some(String::from("root")),
                forward_ports: Some(vec![
                    ForwardPort::String("db:5432".to_string()),
                    ForwardPort::Number(3000),
                ]),
                ports_attributes: Some(HashMap::from([
                    (
                        "3000".to_string(),
                        PortAttributes {
                            label: Some("This Port".to_string()),
                            on_auto_forward: OnAutoForward::Notify,
                            elevate_if_needed: false,
                            require_local_port: true,
                            protocol: Some(PortAttributeProtocol::Https)
                        }
                    ),
                    (
                        "db:5432".to_string(),
                        PortAttributes {
                            label: Some("This Port too".to_string()),
                            on_auto_forward: OnAutoForward::Silent,
                            elevate_if_needed: true,
                            require_local_port: false,
                            protocol: Some(PortAttributeProtocol::Http)
                        }
                    )
                ])),
                other_ports_attributes: Some(PortAttributes {
                    label: Some("Other Ports".to_string()),
                    on_auto_forward: OnAutoForward::OpenBrowser,
                    elevate_if_needed: true,
                    require_local_port: true,
                    protocol: Some(PortAttributeProtocol::Https)
                }),
                update_remote_user_uid: Some(true),
                remote_env: Some(HashMap::from([
                    ("MYVAR1".to_string(), "myvarvalue".to_string()),
                    ("MYVAR2".to_string(), "myvarothervalue".to_string())
                ])),
                initialize_command: Some(LifecycleScript::from_args(vec![
                    "echo".to_string(),
                    "initialize_command".to_string()
                ])),
                on_create_command: Some(LifecycleScript::from_str("echo on_create_command")),
                update_content_command: Some(LifecycleScript::from_map(HashMap::from([
                    (
                        "first".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    ),
                    (
                        "second".to_string(),
                        vec!["echo".to_string(), "update_content_command".to_string()]
                    )
                ]))),
                post_create_command: Some(LifecycleScript::from_str("echo post_create_command")),
                post_start_command: Some(LifecycleScript::from_args(vec![
                    "echo".to_string(),
                    "post_start_command".to_string()
                ])),
                post_attach_command: Some(LifecycleScript::from_map(HashMap::from([
                    (
                        "something".to_string(),
                        vec!["echo".to_string(), "post_attach_command".to_string()]
                    ),
                    (
                        "something1".to_string(),
                        vec![
                            "echo".to_string(),
                            "something".to_string(),
                            "else".to_string()
                        ]
                    )
                ]))),
                wait_for: Some(LifecycleCommand::PostStartCommand),
                user_env_probe: Some(UserEnvProbe::LoginShell),
                features: Some(HashMap::from([
                    (
                        "ghcr.io/devcontainers/features/aws-cli:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    ),
                    (
                        "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                        FeatureOptions::Options(HashMap::new())
                    )
                ])),
                override_feature_install_order: Some(vec![
                    "ghcr.io/devcontainers/features/anaconda:1".to_string(),
                    "ghcr.io/devcontainers/features/aws-cli:1".to_string()
                ]),
                host_requirements: Some(HostRequirements {
                    cpus: Some(2),
                    memory: Some("8gb".to_string()),
                    storage: Some("32gb".to_string()),
                }),
                app_port: vec!["8081:8081".to_string()],
                container_env: Some(HashMap::from([
                    ("MYVAR3".to_string(), "myvar3".to_string()),
                    ("MYVAR4".to_string(), "myvar4".to_string())
                ])),
                container_user: Some("myUser".to_string()),
                mounts: Some(vec![
                    MountDefinition {
                        source: Some("/localfolder/app".to_string()),
                        target: "/workspaces/app".to_string(),
                        mount_type: Some("volume".to_string()),
                    },
                    MountDefinition {
                        source: Some("dev-containers-cli-bashhistory".to_string()),
                        target: "/home/node/commandhistory".to_string(),
                        mount_type: None,
                    }
                ]),
                run_args: Some(vec!["-c".to_string(), "some_command".to_string()]),
                shutdown_action: Some(ShutdownAction::StopContainer),
                override_command: Some(true),
                workspace_folder: Some("/workspaces".to_string()),
                workspace_mount: Some(MountDefinition {
                    source: Some("/folder".to_string()),
                    target: "/workspace".to_string(),
                    mount_type: Some("bind".to_string())
                }),
                build: Some(ContainerBuild {
                    dockerfile: "DockerFile".to_string(),
                    context: Some("..".to_string()),
                    args: Some(HashMap::from([(
                        "MYARG".to_string(),
                        "MYVALUE".to_string()
                    )])),
                    options: Some(vec!["--some-option".to_string(), "--mount".to_string()]),
                    target: Some("development".to_string()),
                    cache_from: Some(vec!["some_image".to_string()]),
                }),
                ..Default::default()
            }
        );

        assert_eq!(
            devcontainer.build_type(),
            DevContainerBuildType::Dockerfile(ContainerBuild {
                dockerfile: "DockerFile".to_string(),
                context: Some("..".to_string()),
                args: Some(HashMap::from([(
                    "MYARG".to_string(),
                    "MYVALUE".to_string()
                )])),
                options: Some(vec!["--some-option".to_string(), "--mount".to_string()]),
                target: Some("development".to_string()),
                cache_from: Some(vec!["some_image".to_string()]),
            })
        );
    }

    #[test]
    fn should_deserialize_app_port_array() {
        let given_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "name": "myDevContainer",
                "remoteUser": "root",
                "appPort": [
                    "8081:8083",
                    "9001",
                ],
                "build": {
                   	"dockerfile": "DockerFile",
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");

        assert_eq!(
            devcontainer.app_port,
            vec!["8081:8083".to_string(), "9001:9001".to_string()]
        )
    }

    #[test]
    fn mount_definition_should_use_bind_type_for_unix_absolute_paths() {
        let mount = MountDefinition {
            source: Some("/home/user/project".to_string()),
            target: "/workspaces/project".to_string(),
            mount_type: None,
        };

        let rendered = mount.to_string();

        assert!(
            rendered.starts_with("type=bind,"),
            "Expected mount type 'bind' for Unix absolute path, but got: {rendered}"
        );
    }

    #[test]
    fn mount_definition_should_use_bind_type_for_windows_unc_paths() {
        let mount = MountDefinition {
            source: Some("\\\\server\\share\\project".to_string()),
            target: "/workspaces/project".to_string(),
            mount_type: None,
        };

        let rendered = mount.to_string();

        assert!(
            rendered.starts_with("type=bind,"),
            "Expected mount type 'bind' for Windows UNC path, but got: {rendered}"
        );
    }

    #[test]
    fn mount_definition_should_use_bind_type_for_windows_absolute_paths() {
        let mount = MountDefinition {
            source: Some("C:\\Users\\mrg\\cli".to_string()),
            target: "/workspaces/cli".to_string(),
            mount_type: None,
        };

        let rendered = mount.to_string();

        assert!(
            rendered.starts_with("type=bind,"),
            "Expected mount type 'bind' for Windows absolute path, but got: {rendered}"
        );
    }

    #[test]
    fn mount_definition_should_omit_source_when_none() {
        let mount = MountDefinition {
            source: None,
            target: "/tmp".to_string(),
            mount_type: Some("tmpfs".to_string()),
        };

        let rendered = mount.to_string();

        assert_eq!(rendered, "type=tmpfs,target=/tmp,consistency=cached");
    }

    #[test]
    fn should_deserialize_port_attributes_with_missing_optional_fields() {
        let json = r#"
        {
            "image": "nginx",
            "portsAttributes": {
                "8080": {
                    "label": "app",
                    "onAutoForward": "silent"
                }
            }
        }
        "#;

        let result = deserialize_devcontainer_json(json);
        assert!(
            result.is_ok(),
            "Expected deserialization to succeed with partial portsAttributes, got: {:?}",
            result.err()
        );

        let devcontainer = result.unwrap();
        let port_attrs = devcontainer.ports_attributes.unwrap();
        let attrs = port_attrs.get("8080").unwrap();
        assert_eq!(attrs.elevate_if_needed, false);
        assert_eq!(attrs.require_local_port, false);
    }

    #[test]
    fn should_deserialize_port_attributes_with_all_fields_omitted() {
        let json = r#"
        {
            "image": "nginx",
            "portsAttributes": {
                "3000": {}
            }
        }
        "#;

        let result = deserialize_devcontainer_json(json);
        assert!(
            result.is_ok(),
            "Expected deserialization to succeed with empty portsAttributes, got: {:?}",
            result.err()
        );

        let devcontainer = result.unwrap();
        let port_attrs = devcontainer.ports_attributes.unwrap();
        let attrs = port_attrs.get("3000").unwrap();
        assert_eq!(attrs.on_auto_forward, OnAutoForward::Notify);
        assert_eq!(attrs.elevate_if_needed, false);
        assert_eq!(attrs.require_local_port, false);
    }

    #[test]
    fn should_fail_validation_with_workspace_mount_only() {
        let given_image_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "build": {
                    "dockerfile": "Dockerfile",
                },
                "name": "myDevContainer",
                "workspaceMount": "source=/app,target=/workspaces/app,type=bind,consistency=cached",
                "customizations": {
                    "vscode": {
                        // Just confirm that this can be included and ignored
                    },
                    "zed": {
                        "extensions": [
                            "html"
                        ]
                    }
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_image_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");

        assert_eq!(
            devcontainer.validate_devcontainer_contents(),
            Err(DevContainerError::DevContainerValidationFailed(
                "workspaceMount and workspaceFolder must both be defined, or neither defined"
                    .to_string()
            ))
        );
    }

    #[test]
    fn should_fail_validation_with_workspace_folder_only() {
        let given_image_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "build": {
                    "dockerfile": "Dockerfile",
                },
                "name": "myDevContainer",
                "workspaceFolder": "/workspaces",
                "customizations": {
                    "vscode": {
                        // Just confirm that this can be included and ignored
                    },
                    "zed": {
                        "extensions": [
                            "html"
                        ]
                    }
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_image_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");

        assert_eq!(
            devcontainer.validate_devcontainer_contents(),
            Err(DevContainerError::DevContainerValidationFailed(
                "workspaceMount and workspaceFolder must both be defined, or neither defined"
                    .to_string()
            ))
        );
    }

    #[test]
    fn should_pass_validation_with_workspace_folder_for_docker_compose() {
        let given_image_container_json = r#"
            // These are some external comments. serde_lenient should handle them
            {
                // These are some internal comments
                "dockerComposeFile": "docker-compose-plain.yml",
                "service": "app",
                "name": "myDevContainer",
                "workspaceFolder": "/workspaces",
                "customizations": {
                    "vscode": {
                        // Just confirm that this can be included and ignored
                    },
                    "zed": {
                        "extensions": [
                            "html"
                        ]
                    }
                }
            }
            "#;

        let result = deserialize_devcontainer_json(given_image_container_json);

        assert!(result.is_ok());
        let devcontainer = result.expect("ok");

        assert!(devcontainer.validate_devcontainer_contents().is_ok());
    }
}
