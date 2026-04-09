use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize};
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

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub(crate) struct DockerConfigLabels {
    #[serde(
        rename = "devcontainer.metadata",
        deserialize_with = "deserialize_metadata"
    )]
    pub(crate) metadata: Option<Vec<HashMap<String, serde_json_lenient::Value>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct DockerInspectConfig {
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
            let parts: Vec<&str> = env_var.split("=").collect();
            if parts.len() != 2 {
                log::error!("Unable to parse {env_var} into and environment key-value");
                return Err(DevContainerError::DevContainerParseFailed);
            }
            map.insert(parts[0].to_string(), parts[1].to_string());
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
    pub(crate) args: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) additional_contexts: Option<HashMap<String, String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) build: Option<DockerComposeServiceBuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) privileged: Option<bool>,
    pub(crate) volumes: Vec<MountDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) env_file: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) ports: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) network_mode: Option<String>,
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
    pub(crate) volumes: HashMap<String, DockerComposeVolume>,
}

pub(crate) struct Docker {
    docker_cli: String,
}

impl DockerInspect {
    pub(crate) fn is_running(&self) -> bool {
        self.state.as_ref().map_or(false, |s| s.running)
    }
}

impl Docker {
    pub(crate) fn new(docker_cli: &str) -> Self {
        Self {
            docker_cli: docker_cli.to_string(),
        }
    }

    fn is_podman(&self) -> bool {
        self.docker_cli == "podman"
    }

    async fn pull_image(&self, image: &String) -> Result<(), DevContainerError> {
        let mut command = Command::new(&self.docker_cli);
        command.args(&["pull", image]);

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
        !self.is_podman()
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

fn deserialize_metadata<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<HashMap<String, serde_json_lenient::Value>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(json_string) => {
            let parsed: Vec<HashMap<String, serde_json_lenient::Value>> =
                serde_json_lenient::from_str(&json_string).map_err(|e| {
                    log::error!("Error deserializing metadata: {e}");
                    serde::de::Error::custom(e)
                })?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

pub(crate) fn get_remote_dir_from_config(
    config: &DockerInspect,
    local_dir: String,
) -> Result<String, DevContainerError> {
    let local_path = PathBuf::from(&local_dir);

    let Some(mounts) = &config.mounts else {
        log::error!("No mounts defined for container");
        return Err(DevContainerError::ContainerNotValid(config.id.clone()));
    };

    for mount in mounts {
        // Sometimes docker will mount the local filesystem on host_mnt for system isolation
        let mount_source = PathBuf::from(&mount.source.trim_start_matches("/host_mnt"));
        if let Ok(relative_path_to_project) = local_path.strip_prefix(&mount_source) {
            let remote_dir = format!(
                "{}/{}",
                &mount.destination,
                relative_path_to_project.display()
            );
            return Ok(remote_dir);
        }
        if mount.source == local_dir {
            return Ok(mount.destination.clone());
        }
    }
    log::error!("No mounts to local folder");
    Err(DevContainerError::ContainerNotValid(config.id.clone()))
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
            Docker, DockerComposeConfig, DockerComposeService, DockerComposeVolume, DockerInspect,
            DockerPs, get_remote_dir_from_config,
        },
    };

    #[test]
    fn should_create_docker_inspect_command() {
        let docker = Docker::new("docker");
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
    fn should_get_target_dir_from_docker_inspect() {
        let given_config = r#"
    {
      "Id": "abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85",
      "Created": "2026-02-04T23:44:21.802688084Z",
      "Path": "/bin/sh",
      "Args": [
        "-c",
        "echo Container started\ntrap \"exit 0\" 15\n\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done",
        "-"
      ],
      "State": {
        "Status": "running",
        "Running": true,
        "Paused": false,
        "Restarting": false,
        "OOMKilled": false,
        "Dead": false,
        "Pid": 23087,
        "ExitCode": 0,
        "Error": "",
        "StartedAt": "2026-02-04T23:44:21.954875084Z",
        "FinishedAt": "0001-01-01T00:00:00Z"
      },
      "Image": "sha256:3dcb059253b2ebb44de3936620e1cff3dadcd2c1c982d579081ca8128c1eb319",
      "ResolvConfPath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/resolv.conf",
      "HostnamePath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/hostname",
      "HostsPath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/hosts",
      "LogPath": "/var/lib/docker/containers/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85/abdb6ab59573659b11dac9f4973796741be35b642c9b48960709304ce46dbf85-json.log",
      "Name": "/objective_haslett",
      "RestartCount": 0,
      "Driver": "overlayfs",
      "Platform": "linux",
      "MountLabel": "",
      "ProcessLabel": "",
      "AppArmorProfile": "",
      "ExecIDs": [
        "008019d93df4107fcbba78bcc6e1ed7e121844f36c26aca1a56284655a6adb53"
      ],
      "HostConfig": {
        "Binds": null,
        "ContainerIDFile": "",
        "LogConfig": {
          "Type": "json-file",
          "Config": {}
        },
        "NetworkMode": "bridge",
        "PortBindings": {},
        "RestartPolicy": {
          "Name": "no",
          "MaximumRetryCount": 0
        },
        "AutoRemove": false,
        "VolumeDriver": "",
        "VolumesFrom": null,
        "ConsoleSize": [
          0,
          0
        ],
        "CapAdd": null,
        "CapDrop": null,
        "CgroupnsMode": "private",
        "Dns": [],
        "DnsOptions": [],
        "DnsSearch": [],
        "ExtraHosts": null,
        "GroupAdd": null,
        "IpcMode": "private",
        "Cgroup": "",
        "Links": null,
        "OomScoreAdj": 0,
        "PidMode": "",
        "Privileged": false,
        "PublishAllPorts": false,
        "ReadonlyRootfs": false,
        "SecurityOpt": null,
        "UTSMode": "",
        "UsernsMode": "",
        "ShmSize": 67108864,
        "Runtime": "runc",
        "Isolation": "",
        "CpuShares": 0,
        "Memory": 0,
        "NanoCpus": 0,
        "CgroupParent": "",
        "BlkioWeight": 0,
        "BlkioWeightDevice": [],
        "BlkioDeviceReadBps": [],
        "BlkioDeviceWriteBps": [],
        "BlkioDeviceReadIOps": [],
        "BlkioDeviceWriteIOps": [],
        "CpuPeriod": 0,
        "CpuQuota": 0,
        "CpuRealtimePeriod": 0,
        "CpuRealtimeRuntime": 0,
        "CpusetCpus": "",
        "CpusetMems": "",
        "Devices": [],
        "DeviceCgroupRules": null,
        "DeviceRequests": null,
        "MemoryReservation": 0,
        "MemorySwap": 0,
        "MemorySwappiness": null,
        "OomKillDisable": null,
        "PidsLimit": null,
        "Ulimits": [],
        "CpuCount": 0,
        "CpuPercent": 0,
        "IOMaximumIOps": 0,
        "IOMaximumBandwidth": 0,
        "Mounts": [
          {
            "Type": "bind",
            "Source": "/somepath/cli",
            "Target": "/workspaces/cli",
            "Consistency": "cached"
          }
        ],
        "MaskedPaths": [
          "/proc/asound",
          "/proc/acpi",
          "/proc/interrupts",
          "/proc/kcore",
          "/proc/keys",
          "/proc/latency_stats",
          "/proc/timer_list",
          "/proc/timer_stats",
          "/proc/sched_debug",
          "/proc/scsi",
          "/sys/firmware",
          "/sys/devices/virtual/powercap"
        ],
        "ReadonlyPaths": [
          "/proc/bus",
          "/proc/fs",
          "/proc/irq",
          "/proc/sys",
          "/proc/sysrq-trigger"
        ]
      },
      "GraphDriver": {
        "Data": null,
        "Name": "overlayfs"
      },
      "Mounts": [
        {
          "Type": "bind",
          "Source": "/somepath/cli",
          "Destination": "/workspaces/cli",
          "Mode": "",
          "RW": true,
          "Propagation": "rprivate"
        }
      ],
      "Config": {
        "Hostname": "abdb6ab59573",
        "Domainname": "",
        "User": "root",
        "AttachStdin": false,
        "AttachStdout": true,
        "AttachStderr": true,
        "Tty": false,
        "OpenStdin": false,
        "StdinOnce": false,
        "Env": [
          "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
        ],
        "Cmd": [
          "-c",
          "echo Container started\ntrap \"exit 0\" 15\n\nexec \"$@\"\nwhile sleep 1 & wait $!; do :; done",
          "-"
        ],
        "Image": "mcr.microsoft.com/devcontainers/base:ubuntu",
        "Volumes": null,
        "WorkingDir": "",
        "Entrypoint": [
          "/bin/sh"
        ],
        "OnBuild": null,
        "Labels": {
          "dev.containers.features": "common",
          "dev.containers.id": "base-ubuntu",
          "dev.containers.release": "v0.4.24",
          "dev.containers.source": "https://github.com/devcontainers/images",
          "dev.containers.timestamp": "Fri, 30 Jan 2026 16:52:34 GMT",
          "dev.containers.variant": "noble",
          "devcontainer.config_file": "/somepath/cli/.devcontainer/dev_container_2/devcontainer.json",
          "devcontainer.local_folder": "/somepath/cli",
          "devcontainer.metadata": "[{\"id\":\"ghcr.io/devcontainers/features/common-utils:2\"},{\"id\":\"ghcr.io/devcontainers/features/git:1\",\"customizations\":{\"vscode\":{\"settings\":{\"github.copilot.chat.codeGeneration.instructions\":[{\"text\":\"This dev container includes an up-to-date version of Git, built from source as needed, pre-installed and available on the `PATH`.\"}]}}}},{\"remoteUser\":\"vscode\"}]",
          "org.opencontainers.image.ref.name": "ubuntu",
          "org.opencontainers.image.version": "24.04",
          "version": "2.1.6"
        },
        "StopTimeout": 1
      },
      "NetworkSettings": {
        "Bridge": "",
        "SandboxID": "2a94990d542fe532deb75f1cc67f761df2d669e3b41161f914079e88516cc54b",
        "SandboxKey": "/var/run/docker/netns/2a94990d542f",
        "Ports": {},
        "HairpinMode": false,
        "LinkLocalIPv6Address": "",
        "LinkLocalIPv6PrefixLen": 0,
        "SecondaryIPAddresses": null,
        "SecondaryIPv6Addresses": null,
        "EndpointID": "ef5b35a8fbb145565853e1a1d960e737fcc18c20920e96494e4c0cfc55683570",
        "Gateway": "172.17.0.1",
        "GlobalIPv6Address": "",
        "GlobalIPv6PrefixLen": 0,
        "IPAddress": "172.17.0.3",
        "IPPrefixLen": 16,
        "IPv6Gateway": "",
        "MacAddress": "",
        "Networks": {
          "bridge": {
            "IPAMConfig": null,
            "Links": null,
            "Aliases": null,
            "MacAddress": "9a:ec:af:8a:ac:81",
            "DriverOpts": null,
            "GwPriority": 0,
            "NetworkID": "51bb8ccc4d1281db44f16d915963fc728619d4a68e2f90e5ea8f1cb94885063e",
            "EndpointID": "ef5b35a8fbb145565853e1a1d960e737fcc18c20920e96494e4c0cfc55683570",
            "Gateway": "172.17.0.1",
            "IPAddress": "172.17.0.3",
            "IPPrefixLen": 16,
            "IPv6Gateway": "",
            "GlobalIPv6Address": "",
            "GlobalIPv6PrefixLen": 0,
            "DNSNames": null
          }
        }
      },
      "ImageManifestDescriptor": {
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "digest": "sha256:39c3436527190561948236894c55b59fa58aa08d68d8867e703c8d5ab72a3593",
        "size": 2195,
        "platform": {
          "architecture": "arm64",
          "os": "linux"
        }
      }
    }
                "#;
        let config = serde_json_lenient::from_str::<DockerInspect>(given_config).unwrap();

        let target_dir = get_remote_dir_from_config(&config, "/somepath/cli".to_string());

        assert!(target_dir.is_ok());
        assert_eq!(target_dir.unwrap(), "/workspaces/cli/".to_string());
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
                        image: Some(
                            "mcr.microsoft.com/devcontainers/rust:2-1-bookworm".to_string(),
                        ),
                        volumes: vec![MountDefinition {
                            mount_type: Some("bind".to_string()),
                            source: "/path/to".to_string(),
                            target: "/workspaces".to_string(),
                        }],
                        network_mode: Some("service:db".to_string()),
                        ..Default::default()
                    },
                ),
                (
                    "db".to_string(),
                    DockerComposeService {
                        image: Some("postgres:14.1".to_string()),
                        volumes: vec![MountDefinition {
                            mount_type: Some("volume".to_string()),
                            source: "postgres-data".to_string(),
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
}
