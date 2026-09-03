use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, de};
use util::command::Command;

use crate::{
    DevContainerHost,
    command_json::{deserialize_json_output, deserialize_yaml_output},
    devcontainer_api::DevContainerError,
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
    #[serde(default)]
    pub(crate) started_at: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) init: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_compose_volumes"
    )]
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
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "deserialize_environment"
    )]
    pub(crate) environment: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeVolume {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub(crate) struct DockerComposeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    pub(crate) services: HashMap<String, DockerComposeService>,
    #[serde(default, deserialize_with = "deserialize_compose_top_level_volumes")]
    pub(crate) volumes: HashMap<String, DockerComposeVolume>,
}

pub(crate) struct Docker {
    host: DevContainerHost,
    docker_cli: String,
    has_buildx: bool,
}

impl DockerInspect {
    pub(crate) fn is_running(&self) -> bool {
        self.state.as_ref().map_or(false, |s| s.running)
    }
}

impl Docker {
    pub(crate) async fn new(
        host: DevContainerHost,
        docker_cli: &str,
        use_buildkit: Option<bool>,
    ) -> Self {
        let has_buildx = if docker_cli == "podman" {
            false
        } else if let Some(use_buildkit) = use_buildkit {
            // Honor the explicit `dev_container_use_buildkit` setting. Setting it
            // to `false` forces the classic Docker builder for Docker-compatible
            // engines that lack an integrated BuildKit (e.g. Apple Container via
            // a Docker-API bridge), where BuildKit builds cannot resolve
            // locally-built images. The classic builder builds the feature
            // content as an image and references it with an ordinary
            // multi-stage `FROM`.
            use_buildkit
        } else {
            let probe = host.command(docker_cli, &buildx_version_args(), &no_env(), None);
            match probe {
                Ok(mut command) => command
                    .output()
                    .await
                    .map(|output| output.status.success())
                    .unwrap_or(false),
                Err(_) => false,
            }
        };
        if !has_buildx && docker_cli != "podman" {
            log::info!(
                "Using the classic Docker builder for dev container builds (BuildKit unavailable or disabled)"
            );
        }
        Self {
            host,
            docker_cli: docker_cli.to_string(),
            has_buildx,
        }
    }

    /// Runs an engine invocation on this client's host and waits for it to
    /// exit. Every command the client issues goes through here, so the host
    /// is applied in exactly one place.
    async fn run(
        &self,
        args: Vec<String>,
        env: HashMap<String, String>,
    ) -> Result<std::process::Output, DevContainerError> {
        let mut command = self.host.command(&self.docker_cli, &args, &env, None)?;
        log::debug!("Running `{} {}`", self.docker_cli, args.join(" "));
        command.output().await.map_err(|e| {
            log::error!(
                "Error running `{} {}`: {e}",
                self.docker_cli,
                args.join(" ")
            );
            DevContainerError::CommandFailed(self.docker_cli.clone())
        })
    }

    async fn pull_image(&self, image: &str) -> Result<(), DevContainerError> {
        let output = self
            .run(pull_args(image), no_env())
            .await
            .map_err(|_| DevContainerError::ResourceFetchFailed)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success result from docker pull: {stderr}");
            return Err(DevContainerError::ResourceFetchFailed);
        }
        Ok(())
    }
}

fn no_env() -> HashMap<String, String> {
    HashMap::new()
}

fn buildx_version_args() -> Vec<String> {
    vec!["buildx".to_string(), "version".to_string()]
}

fn pull_args(image: &str) -> Vec<String> {
    vec!["pull".to_string(), "--".to_string(), image.to_string()]
}

fn query_containers_args(filters: &[String]) -> Vec<String> {
    let mut args = vec!["ps".to_string(), "-a".to_string()];
    for filter in filters {
        args.push("--filter".to_string());
        args.push(filter.clone());
    }
    args.push("--format={{ json . }}".to_string());
    args
}

fn inspect_args(id: &str) -> Vec<String> {
    vec![
        "inspect".to_string(),
        "--format={{json . }}".to_string(),
        id.to_string(),
    ]
}

fn compose_config_args(config_files: &[PathBuf]) -> Vec<String> {
    let mut args = vec!["compose".to_string()];
    for file_path in config_files {
        args.push("-f".to_string());
        args.push(file_path.display().to_string());
    }
    args.push("config".to_string());
    args
}

fn compose_build_args(
    config_files: &[PathBuf],
    project_name: &str,
    services: Option<&Vec<String>>,
) -> Vec<String> {
    let mut args = vec![
        "compose".to_string(),
        "--project-name".to_string(),
        project_name.to_string(),
    ];
    for file_path in config_files {
        args.push("-f".to_string());
        args.push(file_path.display().to_string());
    }
    args.push("build".to_string());
    if let Some(services) = services {
        args.extend(services.iter().cloned());
    }
    args
}

/// The builder selection passed to `docker compose build`. Without a usable
/// BuildKit the classic builder is forced, because the feature content image
/// is consumed by a later multi-stage `FROM` and only resolves from the
/// daemon's image store.
fn compose_build_env(is_podman: bool, has_buildx: bool) -> HashMap<String, String> {
    let mut env = HashMap::new();
    if is_podman {
        return env;
    }
    if has_buildx {
        env.insert("DOCKER_BUILDKIT".to_string(), "1".to_string());
    } else {
        env.insert("DOCKER_BUILDKIT".to_string(), "0".to_string());
        env.insert("COMPOSE_DOCKER_CLI_BUILD".to_string(), "0".to_string());
    }
    env
}

fn start_container_args(id: &str) -> Vec<String> {
    vec!["start".to_string(), id.to_string()]
}

fn exec_args(
    container_id: &str,
    remote_folder: &str,
    user: &str,
    env: &HashMap<String, String>,
    inner_command: &Command,
) -> Vec<String> {
    let mut args = vec![
        "exec".to_string(),
        "-w".to_string(),
        remote_folder.to_string(),
        "-u".to_string(),
        user.to_string(),
    ];
    for (key, value) in env.iter() {
        args.push("-e".to_string());
        args.push(format!("{key}={value}"));
    }
    args.push(container_id.to_string());
    args.push("sh".to_string());

    let mut inner_script: Vec<String> = vec![inner_command.get_program().display().to_string()];
    inner_script.extend(
        inner_command
            .get_args()
            .map(|arg| arg.display().to_string()),
    );
    args.push("-c".to_string());
    args.push(inner_script.join(" "));
    args
}

#[async_trait]
impl DockerClient for Docker {
    async fn inspect(&self, id: &String) -> Result<DockerInspect, DevContainerError> {
        // Always try inspect first — avoid pulling unless necessary.
        if let Ok(output) = self.run(inspect_args(id), no_env()).await
            && let Ok(Some(docker_inspect)) = deserialize_json_output::<DockerInspect>(output)
        {
            return Ok(docker_inspect);
        }

        // Inspect failed — try pulling and retry.
        self.pull_image(id).await.ok();

        let output = self.run(inspect_args(id), no_env()).await?;
        let docker_inspect: Option<DockerInspect> =
            deserialize_json_output(output).map_err(|e| {
                log::error!("Error reading docker inspect output: {e}");
                DevContainerError::CommandFailed(self.docker_cli.clone())
            })?;
        let Some(docker_inspect) = docker_inspect else {
            log::error!("Docker inspect produced no deserializable output");
            return Err(DevContainerError::CommandFailed(self.docker_cli.clone()));
        };
        Ok(docker_inspect)
    }

    async fn get_docker_compose_config(
        &self,
        config_files: &Vec<PathBuf>,
    ) -> Result<Option<DockerComposeConfig>, DevContainerError> {
        let output = self
            .run(compose_config_args(config_files), no_env())
            .await?;
        deserialize_yaml_output(output).map_err(|e| {
            log::error!("Error reading docker compose config output: {e}");
            DevContainerError::CommandFailed(self.docker_cli.clone())
        })
    }

    async fn docker_compose_build(
        &self,
        config_files: &Vec<PathBuf>,
        project_name: &str,
        services: Option<&Vec<String>>,
    ) -> Result<(), DevContainerError> {
        let output = self
            .run(
                compose_build_args(config_files, project_name, services),
                compose_build_env(self.is_podman(), self.has_buildx),
            )
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker compose build: {}", stderr);
            return Err(DevContainerError::CommandFailed(self.docker_cli.clone()));
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
        let output = self
            .run(
                exec_args(container_id, remote_folder, user, env, &inner_command),
                no_env(),
            )
            .await
            .map_err(|_| DevContainerError::ContainerNotValid(container_id.to_string()))?;
        let std_out = String::from_utf8_lossy(&output.stdout);
        log::debug!("Command output:\n {std_out}");
        if !output.status.success() {
            let std_err = String::from_utf8_lossy(&output.stderr);
            log::error!("Command produced a non-successful output. StdErr: {std_err}");
            return Err(DevContainerError::DevContainerScriptsFailed);
        }

        Ok(())
    }
    async fn start_container(&self, id: &str) -> Result<(), DevContainerError> {
        let output = self.run(start_container_args(id), no_env()).await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker start: {stderr}");
            return Err(DevContainerError::CommandFailed(self.docker_cli.clone()));
        }

        Ok(())
    }

    async fn find_process_by_filters(
        &self,
        filters: Vec<String>,
    ) -> Result<Option<DockerPs>, DevContainerError> {
        let output = self.run(query_containers_args(&filters), no_env()).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from docker ps: {stderr}");
            return Err(DevContainerError::CommandFailed(self.docker_cli.clone()));
        }
        let raw = String::from_utf8_lossy(&output.stdout);
        parse_find_process_output(&raw).map_err(|e| {
            // Preserve the dedicated multi-match error; log and re-wrap other parse failures.
            if let DevContainerError::MultipleMatchingContainers(_) = &e {
                e
            } else {
                log::error!("Error parsing docker ps output: {e}");
                DevContainerError::CommandFailed(self.docker_cli.clone())
            }
        })
    }

    fn new_command(&self) -> Command {
        Command::new(&self.docker_cli)
    }

    fn deploy(&self, command: Command) -> Result<Command, DevContainerError> {
        if matches!(self.host, DevContainerHost::Local) {
            return Ok(command);
        }
        let args: Vec<String> = command
            .get_args()
            .map(|arg| arg.display().to_string())
            .collect();
        let env: HashMap<String, String> = command
            .get_envs()
            .filter_map(|(key, value)| {
                Some((key.display().to_string(), value?.display().to_string()))
            })
            .collect();
        self.host.command(&self.docker_cli, &args, &env, None)
    }

    fn is_podman(&self) -> bool {
        self.docker_cli == "podman"
    }

    fn docker_cli(&self) -> String {
        self.docker_cli.clone()
    }

    fn supports_compose_buildkit(&self) -> bool {
        self.has_buildx
    }
}

/// Parses output of `docker ps -a --format={{ json . }}`. When a single
/// container matches the label filters, docker emits one JSON object; when
/// multiple match, it emits newline-delimited JSON (one object per line).
///
/// Returns `Ok(None)` for no matches, `Ok(Some(_))` for exactly one match,
/// and `DevContainerError::MultipleMatchingContainers` for ≥2 matches — the
/// spec expects identifying labels to be unique per project, so the caller
/// can't silently pick one.
fn parse_find_process_output(raw: &str) -> Result<Option<DockerPs>, DevContainerError> {
    if raw.trim().is_empty() {
        return Ok(None);
    }
    let containers: Vec<DockerPs> = serde_json_lenient::Deserializer::from_str(raw)
        .into_iter::<DockerPs>()
        .collect::<Result<_, _>>()
        .map_err(|e| {
            DevContainerError::CommandFailed(format!("failed to parse docker ps output: {e}"))
        })?;
    match containers.len() {
        0 => Ok(None),
        1 => Ok(containers.into_iter().next()),
        _ => Err(DevContainerError::MultipleMatchingContainers(
            containers.into_iter().map(|c| c.id).collect(),
        )),
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
        services: Option<&Vec<String>>,
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
    /// Creates a command targeting this client's container engine, for callers
    /// that build an invocation the trait does not model. Going through the
    /// client rather than naming the engine directly keeps the decision of
    /// *where* a command runs with the implementation.
    ///
    /// The result is only an argument holder: pass it to [`Self::deploy`]
    /// before running it, or it will run against the local engine.
    fn new_command(&self) -> Command;
    /// Rewrites a command built with [`Self::new_command`] so that it runs on
    /// the machine whose engine this client drives, wrapping it in the host's
    /// transport when that machine is not this one.
    fn deploy(&self, command: Command) -> Result<Command, DevContainerError>;
    fn is_podman(&self) -> bool;
    /// The engine's program name, for diagnostics. Prefer [`Self::new_command`]
    /// when building an invocation.
    fn docker_cli(&self) -> String;
}

fn deserialize_environment<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let Some(value) = Option::<serde_json_lenient::Value>::deserialize(deserializer)? else {
        return Ok(None);
    };

    match value {
        serde_json_lenient::Value::Object(object) => Ok(Some(
            object
                .into_iter()
                .filter_map(|(key, value)| match value {
                    serde_json_lenient::Value::Null => None,
                    serde_json_lenient::Value::String(value) => Some((key, value)),
                    other => Some((key, other.to_string())),
                })
                .collect(),
        )),
        serde_json_lenient::Value::Array(values) => Ok(Some(
            values
                .into_iter()
                .filter_map(|value| {
                    let value = value.as_str()?;
                    let (key, value) = value.split_once('=').unwrap_or((value, ""));
                    Some((key.to_string(), value.to_string()))
                })
                .collect(),
        )),
        _ => Ok(None),
    }
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

fn deserialize_compose_volumes<'de, D>(deserializer: D) -> Result<Vec<MountDefinition>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum VolumeItem {
        Object(MountDefinition),
        String(String),
    }

    let items = Vec::<VolumeItem>::deserialize(deserializer)?;
    items
        .into_iter()
        .map(|item| match item {
            VolumeItem::Object(mount) => Ok(mount),
            VolumeItem::String(s) => parse_compose_volume_string(&s)
                .ok_or_else(|| de::Error::custom(format!("invalid volume string: {s}"))),
        })
        .collect()
}

/// Parses Docker Compose short volume syntax: `[SOURCE:]TARGET[:MODE]`.
/// A leading drive letter (e.g. `C:`) on the source is treated as part of the
/// path rather than as a source/target separator.
fn parse_compose_volume_string(s: &str) -> Option<MountDefinition> {
    let bytes = s.as_bytes();

    // Find the colon that separates source from target, skipping a possible
    // Windows drive-letter prefix (single ASCII letter followed by `:`).
    let separator_start = if bytes.len() >= 2
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && bytes.get(2).map_or(false, |&b| b == b'/' || b == b'\\')
    {
        // Skip past the drive letter prefix (e.g. "C:\")
        3
    } else {
        0
    };

    if let Some(colon_pos) = s[separator_start..].find(':') {
        let colon_pos = colon_pos + separator_start;
        let source = &s[..colon_pos];

        let rest = &s[colon_pos + 1..];

        // `rest` may itself start with a Windows drive letter, so skip past
        // that before looking for a second colon that would delimit the mode.
        let mode_search_start = if rest.len() >= 2
            && rest.as_bytes()[0].is_ascii_alphabetic()
            && rest.as_bytes()[1] == b':'
        {
            2
        } else {
            0
        };

        let (target, _mode) = if let Some(pos) = rest[mode_search_start..].find(':') {
            let pos = pos + mode_search_start;
            (&rest[..pos], Some(&rest[pos + 1..]))
        } else {
            (rest, None)
        };

        if target.is_empty() {
            return None;
        }

        Some(MountDefinition {
            source: Some(source.to_string()),
            target: target.to_string(),
            mount_type: None,
        })
    } else {
        // No colon at all — anonymous volume with only a target path
        if s.is_empty() {
            return None;
        }
        Some(MountDefinition {
            source: None,
            target: s.to_string(),
            mount_type: None,
        })
    }
}

fn deserialize_compose_top_level_volumes<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, DockerComposeVolume>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, Option<DockerComposeVolume>> = HashMap::deserialize(deserializer)?;
    Ok(map
        .into_iter()
        .map(|(key, value)| (key, value.unwrap_or_default()))
        .collect())
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
        path::PathBuf,
        process::{ExitStatus, Output},
        sync::Arc,
    };

    use crate::{
        DevContainerHost, FakeRemoteConnection,
        command_json::deserialize_json_output,
        devcontainer_api::DevContainerError,
        devcontainer_json::MountDefinition,
        docker::{
            Docker, DockerClient, DockerComposeConfig, DockerComposeService,
            DockerComposeServicePort, DockerComposeVolume, DockerInspect, DockerPs,
            compose_build_args, compose_build_env, compose_config_args, exec_args, inspect_args,
            parse_find_process_output, query_containers_args, start_container_args,
        },
    };
    use util::command::Command;

    #[test]
    fn use_buildkit_setting_overrides_buildx_detection() {
        // `Some(_)` short-circuits the `buildx version` probe, so these run
        // without invoking docker.
        let forced_off = futures::executor::block_on(Docker::new(
            DevContainerHost::Local,
            "docker",
            Some(false),
        ));
        assert!(
            !forced_off.supports_compose_buildkit(),
            "use_buildkit=false must force the classic builder"
        );

        let forced_on =
            futures::executor::block_on(Docker::new(DevContainerHost::Local, "docker", Some(true)));
        assert!(
            forced_on.supports_compose_buildkit(),
            "use_buildkit=true must enable BuildKit"
        );

        // podman never supports the BuildKit/buildx path, regardless of the setting.
        let podman =
            futures::executor::block_on(Docker::new(DevContainerHost::Local, "podman", Some(true)));
        assert!(!podman.supports_compose_buildkit());
    }

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
        let given_id = "given_docker_id";

        assert_eq!(
            inspect_args(given_id),
            vec!["inspect", "--format={{json . }}", given_id]
        )
    }

    /// The argument vectors are shared by every host, so these lock down the
    /// shape that both a local engine and a transport-wrapped one receive.
    #[test]
    fn should_build_engine_argument_vectors() {
        assert_eq!(
            query_containers_args(&["label=a".to_string(), "label=b".to_string()]),
            vec![
                "ps",
                "-a",
                "--filter",
                "label=a",
                "--filter",
                "label=b",
                "--format={{ json . }}"
            ]
        );

        assert_eq!(start_container_args("abc"), vec!["start", "abc"]);

        let config_files = vec![PathBuf::from("/project/compose.yml")];
        assert_eq!(
            compose_config_args(&config_files),
            vec!["compose", "-f", "/project/compose.yml", "config"]
        );
        assert_eq!(
            compose_build_args(&config_files, "project", Some(&vec!["app".to_string()])),
            vec![
                "compose",
                "--project-name",
                "project",
                "-f",
                "/project/compose.yml",
                "build",
                "app"
            ]
        );

        // BuildKit selection is engine-dependent and must not leak into podman.
        assert!(compose_build_env(true, false).is_empty());
        assert_eq!(
            compose_build_env(false, true).get("DOCKER_BUILDKIT"),
            Some(&"1".to_string())
        );
        let classic = compose_build_env(false, false);
        assert_eq!(classic.get("DOCKER_BUILDKIT"), Some(&"0".to_string()));
        assert_eq!(
            classic.get("COMPOSE_DOCKER_CLI_BUILD"),
            Some(&"0".to_string())
        );

        let mut inner = Command::new("echo");
        inner.args(["hello", "world"]);
        assert_eq!(
            exec_args(
                "container",
                "/workspace",
                "root",
                &HashMap::from([("KEY".to_string(), "value".to_string())]),
                &inner,
            ),
            vec![
                "exec",
                "-w",
                "/workspace",
                "-u",
                "root",
                "-e",
                "KEY=value",
                "container",
                "sh",
                "-c",
                "echo hello world"
            ]
        );
    }

    /// A remote host must hand the whole invocation to the connection so the
    /// transport quotes it, and must not spawn the engine locally.
    #[test]
    fn should_route_commands_through_the_host_connection() {
        let connection = Arc::new(FakeRemoteConnection::default());
        let host = DevContainerHost::Remote(connection);
        let command = host
            .command(
                "docker",
                &inspect_args("id with spaces"),
                &HashMap::from([("DOCKER_BUILDKIT".to_string(), "0".to_string())]),
                None,
            )
            .expect("building a remote invocation should succeed");

        assert_eq!(command.get_program().display().to_string(), "ssh");
        assert_eq!(
            command
                .get_args()
                .map(|arg| arg.display().to_string())
                .collect::<Vec<_>>(),
            vec![
                "host".to_string(),
                "--".to_string(),
                "DOCKER_BUILDKIT=0".to_string(),
                "'docker'".to_string(),
                "'inspect'".to_string(),
                "'--format={{json . }}'".to_string(),
                "'id with spaces'".to_string(),
            ]
        );
    }

    /// Commands built outside the trait's modeled methods still have to reach
    /// the same host, or a remote dev container would be provisioned by the
    /// local engine.
    #[test]
    fn should_deploy_escape_hatch_commands_to_the_host() {
        let local = Docker {
            host: DevContainerHost::Local,
            docker_cli: "docker".to_string(),
            has_buildx: false,
        };
        let mut command = local.new_command();
        command.args(["build", "."]);
        let deployed = local.deploy(command).expect("local deploy cannot fail");
        assert_eq!(deployed.get_program().display().to_string(), "docker");

        let remote = Docker {
            host: DevContainerHost::Remote(Arc::new(FakeRemoteConnection::default())),
            docker_cli: "docker".to_string(),
            has_buildx: false,
        };
        let mut command = remote.new_command();
        command.args(["build", "."]);
        command.env("DOCKER_BUILDKIT", "0");
        let deployed = remote
            .deploy(command)
            .expect("building a remote invocation should succeed");
        assert_eq!(deployed.get_program().display().to_string(), "ssh");
        assert_eq!(
            deployed
                .get_args()
                .map(|arg| arg.display().to_string())
                .collect::<Vec<_>>(),
            vec![
                "host".to_string(),
                "--".to_string(),
                "DOCKER_BUILDKIT=0".to_string(),
                "'docker'".to_string(),
                "'build'".to_string(),
                "'.'".to_string(),
            ]
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn docker_exec_returns_error_on_nonzero_exit() {
        let docker = Docker {
            host: DevContainerHost::Local,
            docker_cli: "false".to_string(),
            has_buildx: false,
        };

        let result = gpui::block_on(docker.run_docker_exec(
            "container",
            "/workspace",
            "root",
            &HashMap::new(),
            Command::new("true"),
        ));

        assert!(matches!(
            result,
            Err(DevContainerError::DevContainerScriptsFailed)
        ));
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
    fn parse_find_process_output_none() {
        assert!(matches!(parse_find_process_output(""), Ok(None)));
        assert!(matches!(parse_find_process_output("   \n\n"), Ok(None)));
    }

    #[test]
    fn parse_find_process_output_single() {
        let raw = r#"{"ID":"abc123"}"#;
        let result = parse_find_process_output(raw).expect("single match must parse");
        assert_eq!(result.unwrap().id, "abc123");
    }

    #[test]
    fn parse_find_process_output_multiple_errors() {
        // `docker ps --format={{ json . }}` emits newline-delimited JSON when
        // multiple containers match the filters. The spec expects the
        // identifying labels to be unique per project, so this is an error.
        let raw = "{\"ID\":\"abc\"}\n{\"ID\":\"def\"}\n";
        match parse_find_process_output(raw) {
            Err(DevContainerError::MultipleMatchingContainers(ids)) => {
                assert_eq!(ids, vec!["abc".to_string(), "def".to_string()]);
            }
            other => panic!("expected MultipleMatchingContainers, got {other:?}"),
        }
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
                        environment: Some(HashMap::from([
                            ("POSTGRES_DB".to_string(), "postgres".to_string()),
                            ("POSTGRES_HOSTNAME".to_string(), "localhost".to_string()),
                            ("POSTGRES_PASSWORD".to_string(), "postgres".to_string()),
                            ("POSTGRES_PORT".to_string(), "5432".to_string()),
                            ("POSTGRES_USER".to_string(), "postgres".to_string()),
                        ])),
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
                        environment: Some(HashMap::from([
                            ("POSTGRES_DB".to_string(), "postgres".to_string()),
                            ("POSTGRES_HOSTNAME".to_string(), "localhost".to_string()),
                            ("POSTGRES_PASSWORD".to_string(), "postgres".to_string()),
                            ("POSTGRES_PORT".to_string(), "5432".to_string()),
                            ("POSTGRES_USER".to_string(), "postgres".to_string()),
                        ])),
                        ..Default::default()
                    },
                ),
            ]),
            volumes: HashMap::from([(
                "postgres-data".to_string(),
                DockerComposeVolume {
                    name: Some("devcontainer_postgres-data".to_string()),
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
    fn should_deserialize_compose_environment_key_only_entries() {
        let given_config = r#"
        {
            "name": "devcontainer",
            "services": {
                "array": {
                    "image": "node:22-alpine",
                    "environment": ["USER_INPUT", "DEFINED=value"]
                },
                "map": {
                    "image": "node:22-alpine",
                    "environment": {
                        "USER_INPUT": null,
                        "DEFINED": "value"
                    }
                }
            }
        }
        "#;

        let config: DockerComposeConfig = serde_json_lenient::from_str(given_config).unwrap();
        assert_eq!(
            config.services.get("array").unwrap().environment,
            Some(HashMap::from([
                ("DEFINED".to_string(), "value".to_string()),
                ("USER_INPUT".to_string(), "".to_string()),
            ]))
        );
        assert_eq!(
            config.services.get("map").unwrap().environment,
            Some(HashMap::from([(
                "DEFINED".to_string(),
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
    fn should_deserialize_compose_inline_volume_strings() {
        let given_yaml = indoc::indoc! {r#"
            name: devcontainer
            services:
              app:
                image: node:18
                volumes:
                  - postgres-data:/var/lib/postgresql/data
                  - /host/path:/container/path
                  - /anonymous/volume
                  - type: bind
                    source: /explicit
                    target: /mnt/explicit
            volumes:
              postgres-data:
                name: devcontainer_postgres-data
        "#};

        let config: DockerComposeConfig = serde_yaml::from_str(given_yaml).unwrap();
        let service = config.services.get("app").unwrap();
        assert_eq!(service.volumes.len(), 4);

        assert_eq!(service.volumes[0].source, Some("postgres-data".to_string()));
        assert_eq!(service.volumes[0].target, "/var/lib/postgresql/data");
        assert_eq!(service.volumes[0].mount_type, None);

        assert_eq!(service.volumes[1].source, Some("/host/path".to_string()));
        assert_eq!(service.volumes[1].target, "/container/path");

        assert_eq!(service.volumes[2].source, None);
        assert_eq!(service.volumes[2].target, "/anonymous/volume");

        assert_eq!(service.volumes[3].source, Some("/explicit".to_string()));
        assert_eq!(service.volumes[3].target, "/mnt/explicit");
        assert_eq!(service.volumes[3].mount_type, Some("bind".to_string()));
    }

    #[test]
    fn should_deserialize_compose_top_level_volumes_with_null_value() {
        let given_yaml = indoc::indoc! {r#"
            name: devcontainer
            services:
              app:
                image: node:18
            volumes:
              postgres-data:
              named-vol:
                name: custom-name
        "#};

        let config: DockerComposeConfig = serde_yaml::from_str(given_yaml).unwrap();
        assert_eq!(config.volumes.len(), 2);

        let bare = config
            .volumes
            .get("postgres-data")
            .expect("bare volume should exist");
        assert_eq!(bare.name, None);

        let named = config
            .volumes
            .get("named-vol")
            .expect("named volume should exist");
        assert_eq!(named.name, Some("custom-name".to_string()));
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
