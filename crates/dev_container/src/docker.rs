use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, de};
use util::command::Command;

use crate::{
    command_json::evaluate_json_command, devcontainer_api::DevContainerError,
    devcontainer_json::MountDefinition,
};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerPs {
    #[serde(alias = "ID")]
    pub(crate) id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerState {
    pub(crate) running: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspect {
    pub(crate) id: String,
    pub(crate) config: DockerInspectConfig,
    pub(crate) mounts: Option<Vec<DockerInspectMount>>,
    pub(crate) state: Option<DockerState>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerConfigLabels {
    #[serde(
        default,
        rename = "devcontainer.metadata",
        deserialize_with = "deserialize_metadata"
    )]
    pub(crate) metadata: Option<Vec<HashMap<String, serde_json_lenient::Value>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspectConfig {
    #[serde(default, deserialize_with = "deserialize_nullable_labels")]
    pub(crate) labels: DockerConfigLabels,
    #[serde(rename = "User")]
    pub(crate) image_user: Option<String>,
    #[serde(default)]
    pub(crate) env: Vec<String>,
}

impl DockerInspectConfig {
    pub(crate) fn env_as_map(&self) -> Result<HashMap<String, String>, DevContainerError> {
        let mut map = HashMap::new();
        for env_var in &self.env {
            let Some((key, value)) = env_var.split_once('=') else {
                log::warn!("Skipping environment variable without a value: {env_var}");
                continue;
            };
            map.insert(key.to_string(), value.to_string());
        }
        Ok(map)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspectMount {
    pub(crate) source: String,
    pub(crate) destination: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeServiceBuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dockerfile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) args: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) additional_contexts: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeServicePort {
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub(crate) target: String,
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub(crate) published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) host_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) app_protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
}

fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
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

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => Ok(s),
        StringOrInt::Int(b) => Ok(b.to_string()),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeService {
    pub(crate) image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cap_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) security_opt: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_labels"
    )]
    pub(crate) labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) build: Option<DockerComposeServiceBuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) privileged: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) volumes: Vec<MountDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) env_file: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) ports: Vec<DockerComposeServicePort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) network_mode: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_nullable_vec"
    )]
    pub(crate) command: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeVolume {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    pub(crate) services: HashMap<String, DockerComposeService>,
    #[serde(default)]
    pub(crate) volumes: HashMap<String, DockerComposeVolume>,
}

pub(crate) struct Docker {
    docker_cli: String,
    has_buildx: bool,
}

impl DockerInspect {
    pub(crate) fn is_running(&self) -> bool {
        self.state.as_ref().map_or(false, |s| s.running)
    }
}

impl Docker {
    pub(crate) async fn new(docker_cli: &str) -> Self {
        let has_buildx = if docker_cli == "podman" {
            false
        } else {
            let output = Command::new(docker_cli)
                .args(["buildx", "version"])
                .output()
                .await;
            output.map(|o| o.status.success()).unwrap_or(false)
        };
        if !has_buildx && docker_cli != "podman" {
            log::info!(
                "docker buildx not found; dev container builds will use the scratch-image fallback"
            );
        }
        Self {
            docker_cli: docker_cli.to_string(),
            has_buildx,
        }
    }

    fn is_podman(&self) -> bool {
        self.docker_cli == "podman"
    }

    async fn pull_image(&self, image: &String) -> Result<(), DevContainerError> {
        let mut command = Command::new(&self.docker_cli);
        command.args(&["pull", "--", image]);

        let output = command.output().await.map_err(|e| {
            log::error!("Error pulling image: {e}");
            DevContainerError::ResourceFetchFailed
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success result from docker pull: {stderr}");
            return Err(DevContainerError::ResourceFetchFailed);
        }
        Ok(())
    }

    fn create_docker_query_containers(&self, filters: Vec<String>) -> Command {
        let mut command = Command::new(&self.docker_cli);
        command.args(&["ps", "-a"]);

        for filter in filters {
            command.arg("--filter");
            command.arg(filter);
        }
        command.arg("--format={{ json . }}");
        command
    }

    fn create_docker_inspect(&self, id: &str) -> Command {
        let mut command = Command::new(&self.docker_cli);
        command.args(&["inspect", "--format={{json . }}", id]);
        command
    }

    fn create_docker_compose_config_command(&self, config_files: &Vec<PathBuf>) -> Command {
        let mut command = Command::new(&self.docker_cli);
        command.arg("compose");
        for file_path in config_files {
            command.args(&["-f", &file_path.display().to_string()]);
        }
        command.args(&["config", "--format", "json"]);
        command
    }
}

#[async_trait]
impl DockerClient for Docker {
    async fn inspect(&self, id: &String) -> Result<DockerInspect, DevContainerError> {
        // Try to pull the image, continue on failure; Image may be local only, id a reference to a running container
        self.pull_image(id).await.ok();

        let command = self.create_docker_inspect(id);

        let Some(docker_inspect): Option<DockerInspect> = evaluate_json_command(command).await?
        else {
            log::error!("Docker inspect produced no deserializable output");
            return Err(DevContainerError::CommandFailed(self.docker_cli.clone()));
        };
        Ok(docker_inspect)
    }

    async fn get_docker_compose_config(
        &self,
        config_files: &Vec<PathBuf>,
    ) -> Result<Option<DockerComposeConfig>, DevContainerError> {
        let command = self.create_docker_compose_config_command(config_files);
        evaluate_json_command(command).await
    }

    async fn docker_compose_build(
        &self,
        config_files: &Vec<PathBuf>,
        project_name: &str,
    ) -> Result<(), DevContainerError> {
        let mut command = Command::new(&self.docker_cli);
        if !self.is_podman() {
            command.env("DOCKER_BUILDKIT", "1");
        }
        command.args(&["compose", "--project-name", project_name]);
        for docker_compose_file in config_files {
            command.args(&["-f", &docker_compose_file.display().to_string()]);
        }
        command.arg("build");

        let output = command.output().await.map_err(|e| {
            log::error!("Error running docker compose up: {e}");
            DevContainerError::CommandFailed(command.get_program().display().to_string())
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker compose up: {}", stderr);
            return Err(DevContainerError::CommandFailed(
                command.get_program().display().to_string(),
            ));
        }

        Ok(())
    }
    async fn run_docker_exec(
        &self,
        container_id: &str,
        remote_folder: &str,
        user: &str,
        env: &HashMap<String, String>,
        inner_command: Command,
    ) -> Result<(), DevContainerError> {
        let mut command = Command::new(&self.docker_cli);

        command.args(&["exec", "-w", remote_folder, "-u", user]);

        for (k, v) in env.iter() {
            command.arg("-e");
            let env_declaration = format!("{}={}", k, v);
            command.arg(&env_declaration);
        }

        command.arg(container_id);

        command.arg("sh");

        let mut inner_program_script: Vec<String> =
            vec![inner_command.get_program().display().to_string()];
        let mut args: Vec<String> = inner_command
            .get_args()
            .map(|arg| arg.display().to_string())
            .collect();
        inner_program_script.append(&mut args);
        command.args(&["-c", &inner_program_script.join(" ")]);

        let output = command.output().await.map_err(|e| {
            log::error!("Error running command {e} in container exec");
            DevContainerError::ContainerNotValid(container_id.to_string())
        })?;
        if !output.status.success() {
            let std_err = String::from_utf8_lossy(&output.stderr);
            log::error!("Command produced a non-successful output. StdErr: {std_err}");
        }
        let std_out = String::from_utf8_lossy(&output.stdout);
        log::debug!("Command output:\n {std_out}");

        Ok(())
    }
    async fn start_container(&self, id: &str) -> Result<(), DevContainerError> {
        let mut command = Command::new(&self.docker_cli);

        command.args(&["start", id]);

        let output = command.output().await.map_err(|e| {
            log::error!("Error running docker start: {e}");
            DevContainerError::CommandFailed(command.get_program().display().to_string())
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker start: {stderr}");
            return Err(DevContainerError::CommandFailed(
                command.get_program().display().to_string(),
            ));
        }

        Ok(())
    }

    async fn find_process_by_filters(
        &self,
        filters: Vec<String>,
    ) -> Result<Option<DockerPs>, DevContainerError> {
        let command = self.create_docker_query_containers(filters);
        evaluate_json_command(command).await
    }

    fn docker_cli(&self) -> String {
        self.docker_cli.clone()
    }

    fn supports_compose_buildkit(&self) -> bool {
        self.has_buildx
    }
}

#[async_trait]
pub(crate) trait DockerClient {
    async fn inspect(&self, id: &String) -> Result<DockerInspect, DevContainerError>;
    async fn get_docker_compose_config(
        &self,
        config_files: &Vec<PathBuf>,
    ) -> Result<Option<DockerComposeConfig>, DevContainerError>;
    async fn docker_compose_build(
        &self,
        config_files: &Vec<PathBuf>,
        project_name: &str,
    ) -> Result<(), DevContainerError>;
    async fn run_docker_exec(
        &self,
        container_id: &str,
        remote_folder: &str,
        user: &str,
        env: &HashMap<String, String>,
        inner_command: Command,
    ) -> Result<(), DevContainerError>;
    async fn start_container(&self, id: &str) -> Result<(), DevContainerError>;
    async fn find_process_by_filters(
        &self,
        filters: Vec<String>,
    ) -> Result<Option<DockerPs>, DevContainerError>;
    fn supports_compose_buildkit(&self) -> bool;
    /// This operates as an escape hatch for more custom uses of the docker API.
    /// See DevContainerManifest::create_docker_build as an example
    fn docker_cli(&self) -> String;
}

fn deserialize_labels<'de, D>(deserializer: D) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct LabelsVisitor;

    impl<'de> de::Visitor<'de> for LabelsVisitor {
        type Value = Option<HashMap<String, String>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of strings or a map of string key-value pairs")
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let values = Vec::<String>::deserialize(de::value::SeqAccessDeserializer::new(seq))?;

            Ok(Some(
                values
                    .iter()
                    .filter_map(|v| {
                        let (key, value) = v.split_once('=')?;
                        Some((key.to_string(), value.to_string()))
                    })
                    .collect(),
            ))
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            HashMap::<String, String>::deserialize(de::value::MapAccessDeserializer::new(map))
                .map(|v| Some(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(LabelsVisitor)
}

fn deserialize_nullable_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(|opt| opt.unwrap_or_default())
}

fn deserialize_nullable_labels<'de, D>(deserializer: D) -> Result<DockerConfigLabels, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<DockerConfigLabels>::deserialize(deserializer).map(|opt| opt.unwrap_or_default())
}

fn deserialize_metadata<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<HashMap<String, serde_json_lenient::Value>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(json_string) => {
            // The devcontainer metadata label can be either a JSON array (e.g. from
            // image-based devcontainers) or a single JSON object (e.g. from
            // docker-compose-based devcontainers created by the devcontainer CLI).
            // Handle both formats.
            let parsed: Vec<HashMap<String, serde_json_lenient::Value>> =
                serde_json_lenient::from_str(&json_string).or_else(|_| {
                    let single: HashMap<String, serde_json_lenient::Value> =
                        serde_json_lenient::from_str(&json_string).map_err(|e| {
                            log::error!("Error deserializing metadata: {e}");
                            serde::de::Error::custom(e)
                        })?;
                    Ok(vec![single])
                })?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        ffi::OsStr,
        process::{ExitStatus, Output},
    };

    use crate::{
        command_json::deserialize_json_output,
        devcontainer_json::MountDefinition,
        docker::{
            Docker, DockerComposeConfig, DockerComposeService, DockerComposeServicePort,
            DockerComposeVolume, DockerInspect, DockerPs,
        },
    };

    #[test]
    fn should_parse_simple_env_var() {
        let config = super::DockerInspectConfig {
            labels: super::DockerConfigLabels { metadata: None },
            image_user: None,
            env: vec!["KEY=value".to_string()],
        };

        let map = config.env_as_map().unwrap();
        assert_eq!(map.get("KEY").unwrap(), "value");
    }

    #[test]
    fn should_parse_env_var_with_equals_in_value() {
        let config = super::DockerInspectConfig {
            labels: super::DockerConfigLabels { metadata: None },
            image_user: None,
            env: vec!["COMPLEX=key=val other>=1.0".to_string()],
        };

        let map = config.env_as_map().unwrap();
        assert_eq!(map.get("COMPLEX").unwrap(), "key=val other>=1.0");
    }

    #[test]
    fn should_parse_database_url_with_equals_in_query_string() {
        let config = super::DockerInspectConfig {
            labels: super::DockerConfigLabels { metadata: None },
            image_user: None,
            env: vec![
                "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".to_string(),
                "TEST_DATABASE_URL=postgres://postgres:postgres@db:5432/mydb?sslmode=disable"
                    .to_string(),
            ],
        };

        let map = config.env_as_map().unwrap();
        assert_eq!(
            map.get("TEST_DATABASE_URL").unwrap(),
            "postgres://postgres:postgres@db:5432/mydb?sslmode=disable"
        );
    }

    #[test]
    fn should_skip_env_var_without_equals() {
        let config = super::DockerInspectConfig {
            labels: super::DockerConfigLabels { metadata: None },
            image_user: None,
            env: vec![
                "VALID_KEY=valid_value".to_string(),
                "NO_EQUALS_VAR".to_string(),
                "ANOTHER_VALID=value".to_string(),
            ],
        };

        let map = config.env_as_map().unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("VALID_KEY").unwrap(), "valid_value");
        assert_eq!(map.get("ANOTHER_VALID").unwrap(), "value");
        assert!(!map.contains_key("NO_EQUALS_VAR"));
    }

    #[test]
    fn should_parse_simple_label() {
        let json = r#"{"volumes": [], "labels": ["com.example.key=value"]}"#;
        let service: DockerComposeService = serde_json_lenient::from_str(json).unwrap();
        let labels = service.labels.unwrap();
        assert_eq!(labels.get("com.example.key").unwrap(), "value");
    }

    #[test]
    fn should_parse_label_with_equals_in_value() {
        let json = r#"{"volumes": [], "labels": ["com.example.key=value=with=equals"]}"#;
        let service: DockerComposeService = serde_json_lenient::from_str(json).unwrap();
        let labels = service.labels.unwrap();
        assert_eq!(labels.get("com.example.key").unwrap(), "value=with=equals");
    }

    #[test]
    fn should_create_docker_inspect_command() {
        let docker = Docker {
            docker_cli: "docker".to_string(),
            has_buildx: false,
        };
        let given_id = "given_docker_id";

        let command = docker.create_docker_inspect(given_id);

        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            vec![
                OsStr::new("inspect"),
                OsStr::new("--format={{json . }}"),
                OsStr::new(given_id)
            ]
        )
    }

    #[test]
    fn should_deserialize_docker_ps_with_filters() {
        // First, deserializes empty
        let empty_output = Output {
            status: ExitStatus::default(),
            stderr: vec![],
            stdout: String::from("").into_bytes(),
        };

        let result: Option<DockerPs> = deserialize_json_output(empty_output).unwrap();

        assert!(result.is_none());

        let full_output = Output {
                status: ExitStatus::default(),
                stderr: vec![],
                stdout: String::from(r#"
    {
        "Command": "\"/bin/sh -c 'echo Co…\"",
        "CreatedAt": "2026-02-04 15:44:21 -0800 PST",
        "ID": "abdb6ab59573",
        "Image": "mcr.microsoft.com/devcontainers/base:ubuntu",
        "Labels": "desktop.docker.io/mounts/0/Source=/somepath/cli,desktop.docker.io/mounts/0/SourceKind=hostFile,desktop.docker.io/mounts/0/Target=/workspaces/cli,desktop.docker.io/ports.scheme=v2,dev.containers.features=common,dev.containers.id=base-ubuntu,dev.containers.release=v0.4.24,dev.containers.source=https://github.com/devcontainers/images,dev.containers.timestamp=Fri, 30 Jan 2026 16:52:34 GMT,dev.containers.variant=noble,devcontainer.config_file=/somepath/cli/.devcontainer/dev_container_2/devcontainer.json,devcontainer.local_folder=/somepath/cli,devcontainer.metadata=[{\"id\":\"ghcr.io/devcontainers/features/common-utils:2\"},{\"id\":\"ghcr.io/devcontainers/features/git:1\",\"customizations\":{\"vscode\":{\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`.\"}]}}}},{\"remoteUser\":\"vscode\"}],org.opencontainers.image.ref.name=ubuntu,org.opencontainers.image.version=24.04,version=2.1.6",
        "LocalVolumes": "0",
        "Mounts": "/host_mnt/User…",
        "Names": "objective_haslett",
        "Networks": "bridge",
        "Platform": {
        "architecture": "arm64",
        "os": "linux"
        },
        "Ports": "",
        "RunningFor": "47 hours ago",
        "Size": "0B",
        "State": "running",
        "Status": "Up 47 hours"
    }
                    "#).into_bytes(),
            };

        let result: Option<DockerPs> = deserialize_json_output(full_output).unwrap();

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.id, "abdb6ab59573".to_string());

        // Podman variant (Id, not ID)
        let full_output = Output {
                status: ExitStatus::default(),
                stderr: vec![],
                stdout: String::from(r#"
    {
        "Command": "\"/bin/sh -c 'echo Co…\"",
        "CreatedAt": "2026-02-04 15:44:21 -0800 PST",
        "Id": "abdb6ab59573",
        "Image": "mcr.microsoft.com/devcontainers/base:ubuntu",
        "Labels": "desktop.docker.io/mounts/0/Source=/somepath/cli,desktop.docker.io/mounts/0/SourceKind=hostFile,desktop.docker.io/mounts/0/Target=/workspaces/cli,desktop.docker.io/ports.scheme=v2,dev.containers.features=common,dev.containers.id=base-ubuntu,dev.containers.release=v0.4.24,dev.containers.source=https://github.com/devcontainers/images,dev.containers.timestamp=Fri, 30 Jan 2026 16:52:34 GMT,dev.containers.variant=noble,devcontainer.config_file=/somepath/cli/.devcontainer/dev_container_2/devcontainer.json,devcontainer.local_folder=/somepath/cli,devcontainer.metadata=[{\"id\":\"ghcr.io/devcontainers/features/common-utils:2\"},{\"id\":\"ghcr.io/devcontainers/features/git:1\",\"customizations\":{\"vscode\":{\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`.\"}]}}}},{\"remoteUser\":\"vscode\"}],org.opencontainers.image.ref.name=ubuntu,org.opencontainers.image.version=24.04,version=2.1.6",
        "LocalVolumes": "0",
        "Mounts": "/host_mnt/User…",
        "Names": "objective_haslett",
        "Networks": "bridge",
        "Platform": {
        "architecture": "arm64",
        "os": "linux"
        },
        "Ports": "",
        "RunningFor": "47 hours ago",
        "Size": "0B",
        "State": "running",
        "Status": "Up 47 hours"
    }
                    "#).into_bytes(),
            };

        let result: Option<DockerPs> = deserialize_json_output(full_output).unwrap();

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.id, "abdb6ab59573".to_string());
    }

    #[test]
    fn should_deserialize_object_metadata_from_docker_compose_container() {
        // The devcontainer CLI writes metadata as a bare JSON object (not an array)
        // when there is only one metadata entry (e.g. docker-compose with no features).
        // See https://github.com/devcontainers/cli/issues/1054
        let given_config = r#"
    {
      "Id": "dc4e7b8ff4bf",
      "Config": {
        "Labels": {
          "devcontainer.metadata": "{\"remoteUser\":\"ubuntu\"}"
        }
      }
    }
                "#;
        let config = serde_json_lenient::from_str::<DockerInspect>(given_config).unwrap();

        assert!(config.config.labels.metadata.is_some());
        let metadata = config.config.labels.metadata.unwrap();
        assert_eq!(metadata.len(), 1);
        assert!(metadata[0].contains_key("remoteUser"));
        assert_eq!(metadata[0]["remoteUser"], "ubuntu");
    }

    #[test]
    fn should_deserialize_docker_compose_config() {
        let given_config = r#"
    {
        "name": "devcontainer",
        "networks": {
        "default": {
            "name": "devcontainer_default",
            "ipam": {}
        }
        },
        "services": {
            "app": {
                "command": [
                "sleep",
                "infinity"
                ],
                "depends_on": {
                "db": {
                    "condition": "service_started",
                    "restart": true,
                    "required": true
                }
                },
                "entrypoint": null,
                "environment": {
                "POSTGRES_DB": "postgres",
                "POSTGRES_HOSTNAME": "localhost",
                "POSTGRES_PASSWORD": "postgres",
                "POSTGRES_PORT": "5432",
                "POSTGRES_USER": "postgres"
                },
                "ports": [
                    {
                        "target": "5443",
                        "published": "5442"
                    },
                    {
                        "name": "custom port",
                        "protocol": "udp",
                        "host_ip": "127.0.0.1",
                        "app_protocol": "http",
                        "mode": "host",
                        "target": "8081",
                        "published": "8083"

                    }
                ],
                "image": "mcr.microsoft.com/devcontainers/rust:2-1-bookworm",
                "network_mode": "service:db",
                "volumes": [
                {
                    "type": "bind",
                    "source": "/path/to",
                    "target": "/workspaces",
                    "bind": {
                    "create_host_path": true
                    }
                }
                ]
            },
            "db": {
                "command": null,
                "entrypoint": null,
                "environment": {
                "POSTGRES_DB": "postgres",
                "POSTGRES_HOSTNAME": "localhost",
                "POSTGRES_PASSWORD": "postgres",
                "POSTGRES_PORT": "5432",
                "POSTGRES_USER": "postgres"
                },
                "image": "postgres:14.1",
                "networks": {
                "default": null
                },
                "restart": "unless-stopped",
                "volumes": [
                {
                    "type": "volume",
                    "source": "postgres-data",
                    "target": "/var/lib/postgresql/data",
                    "volume": {}
                }
                ]
            }
        },
        "volumes": {
        "postgres-data": {
            "name": "devcontainer_postgres-data"
        }
        }
    }
                "#;

        let docker_compose_config: DockerComposeConfig =
            serde_json_lenient::from_str(given_config).unwrap();

        let expected_config = DockerComposeConfig {
            name: Some("devcontainer".to_string()),
            services: HashMap::from([
                (
                    "app".to_string(),
                    DockerComposeService {
                        command: vec!["sleep".to_string(), "infinity".to_string()],
                        image: Some(
                            "mcr.microsoft.com/devcontainers/rust:2-1-bookworm".to_string(),
                        ),
                        volumes: vec![MountDefinition {
                            mount_type: Some("bind".to_string()),
                            source: Some("/path/to".to_string()),
                            target: "/workspaces".to_string(),
                        }],
                        network_mode: Some("service:db".to_string()),

                        ports: vec![
                            DockerComposeServicePort {
                                target: "5443".to_string(),
                                published: "5442".to_string(),
                                ..Default::default()
                            },
                            DockerComposeServicePort {
                                target: "8081".to_string(),
                                published: "8083".to_string(),
                                mode: Some("host".to_string()),
                                protocol: Some("udp".to_string()),
                                host_ip: Some("127.0.0.1".to_string()),
                                app_protocol: Some("http".to_string()),
                                name: Some("custom port".to_string()),
                            },
                        ],
                        ..Default::default()
                    },
                ),
                (
                    "db".to_string(),
                    DockerComposeService {
                        image: Some("postgres:14.1".to_string()),
                        volumes: vec![MountDefinition {
                            mount_type: Some("volume".to_string()),
                            source: Some("postgres-data".to_string()),
                            target: "/var/lib/postgresql/data".to_string(),
                        }],
                        ..Default::default()
                    },
                ),
            ]),
            volumes: HashMap::from([(
                "postgres-data".to_string(),
                DockerComposeVolume {
                    name: "devcontainer_postgres-data".to_string(),
                },
            )]),
        };

        assert_eq!(docker_compose_config, expected_config);
    }

    #[test]
    fn should_deserialize_compose_labels_as_map() {
        let given_config = r#"
        {
            "name": "devcontainer",
            "services": {
                "app": {
                    "image": "node:22-alpine",
                    "volumes": [],
                    "labels": {
                        "com.example.test": "value",
                        "another.label": "another-value"
                    }
                }
            }
        }
        "#;

        let config: DockerComposeConfig = serde_json_lenient::from_str(given_config).unwrap();
        let service = config.services.get("app").unwrap();
        let labels = service.labels.clone().unwrap();
        assert_eq!(
            labels,
            HashMap::from([
                ("another.label".to_string(), "another-value".to_string()),
                ("com.example.test".to_string(), "value".to_string())
            ])
        );
    }

    #[test]
    fn should_deserialize_compose_labels_as_array() {
        let given_config = r#"
        {
            "name": "devcontainer",
            "services": {
                "app": {
                    "image": "node:22-alpine",
                    "volumes": [],
                    "labels": ["com.example.test=value"]
                }
            }
        }
        "#;

        let config: DockerComposeConfig = serde_json_lenient::from_str(given_config).unwrap();
        let service = config.services.get("app").unwrap();
        assert_eq!(
            service.labels,
            Some(HashMap::from([(
                "com.example.test".to_string(),
                "value".to_string()
            )]))
        );
    }

    #[test]
    fn should_deserialize_compose_without_volumes() {
        let given_config = r#"
        {
            "name": "devcontainer",
            "services": {
                "app": {
                    "image": "node:22-alpine",
                    "volumes": []
                }
            }
        }
        "#;

        let config: DockerComposeConfig = serde_json_lenient::from_str(given_config).unwrap();
        assert!(config.volumes.is_empty());
    }

    #[test]
    fn should_deserialize_compose_with_missing_volumes_field() {
        let given_config = r#"
        {
            "name": "devcontainer",
            "services": {
                "sidecar": {
                    "image": "ubuntu:24.04"
                }
            }
        }
        "#;

        let config: DockerComposeConfig = serde_json_lenient::from_str(given_config).unwrap();
        let service = config.services.get("sidecar").unwrap();
        assert!(service.volumes.is_empty());
    }

    #[test]
    fn should_deserialize_compose_volume_without_source() {
        let given_config = r#"
        {
            "name": "devcontainer",
            "services": {
                "app": {
                    "image": "ubuntu:24.04",
                    "volumes": [
                        {
                            "type": "tmpfs",
                            "target": "/tmp"
                        }
                    ]
                }
            }
        }
        "#;

        let config: DockerComposeConfig = serde_json_lenient::from_str(given_config).unwrap();
        let service = config.services.get("app").unwrap();
        assert_eq!(service.volumes.len(), 1);
        assert_eq!(service.volumes[0].source, None);
        assert_eq!(service.volumes[0].target, "/tmp");
        assert_eq!(service.volumes[0].mount_type, Some("tmpfs".to_string()));
    }

    #[test]
    fn should_deserialize_inspect_without_labels() {
        let given_config = r#"
        {
            "Id": "sha256:abc123",
            "Config": {
                "Env": ["PATH=/usr/bin"],
                "Cmd": ["node"],
                "WorkingDir": "/"
            }
        }
        "#;

        let inspect: DockerInspect = serde_json_lenient::from_str(given_config).unwrap();
        assert!(inspect.config.labels.metadata.is_none());
        assert!(inspect.config.image_user.is_none());
    }

    #[test]
    fn should_deserialize_inspect_with_null_labels() {
        let given_config = r#"
        {
            "Id": "sha256:abc123",
            "Config": {
                "Labels": null,
                "Env": ["PATH=/usr/bin"]
            }
        }
        "#;

        let inspect: DockerInspect = serde_json_lenient::from_str(given_config).unwrap();
        assert!(inspect.config.labels.metadata.is_none());
    }

    #[test]
    fn should_deserialize_inspect_with_labels_but_no_metadata() {
        let given_config = r#"
        {
            "Id": "sha256:abc123",
            "Config": {
                "Labels": {
                    "com.example.test": "value"
                },
                "Env": ["PATH=/usr/bin"]
            }
        }
        "#;

        let inspect: DockerInspect = serde_json_lenient::from_str(given_config).unwrap();
        assert!(inspect.config.labels.metadata.is_none());
    }
}
