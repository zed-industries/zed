use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use serde::Deserialize;
use util::command::Command;

use crate::{
    DevContainerHost,
    command_json::deserialize_json_output,
    devcontainer_api::DevContainerError,
    docker::{
        DockerClient, DockerComposeConfig, DockerConfigLabels, DockerInspect, DockerInspectConfig,
        DockerInspectMount, DockerPs, DockerState, exec_args, no_env, start_container_args,
    },
};

/// Drives Apple's `container` CLI (macOS only) directly, rather than through
/// a Docker-API bridge. Its argument shapes differ from Docker's in places
/// (no `--filter`, no `-f`/Go-template `inspect`, a different JSON schema
/// entirely) but the underlying operations are the same, so this translates
/// at the edge into the existing `DockerInspect`/`DockerPs` currency rather
/// than propagating a second inspect type through the manifest code.
pub(crate) struct AppleContainer {
    host: DevContainerHost,
}

impl AppleContainer {
    pub(crate) fn new(host: DevContainerHost) -> Self {
        Self { host }
    }

    async fn run(
        &self,
        args: Vec<String>,
        env: HashMap<String, String>,
    ) -> Result<std::process::Output, DevContainerError> {
        let mut command = self.host.command("container", &args, &env, None)?;
        log::debug!("Running `container {}`", args.join(" "));
        command.output().await.map_err(|e| {
            log::error!("Error running `container {}`: {e}", args.join(" "));
            DevContainerError::CommandFailed("container".to_string())
        })
    }

    async fn pull_image(&self, image: &str) -> Result<(), DevContainerError> {
        let output = self
            .run(
                vec![
                    "image".to_string(),
                    "pull".to_string(),
                    image.to_string(),
                ],
                no_env(),
            )
            .await
            .map_err(|_| DevContainerError::ResourceFetchFailed)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success result from container image pull: {stderr}");
            return Err(DevContainerError::ResourceFetchFailed);
        }
        Ok(())
    }

    /// Tries `container inspect` (running/stopped containers), then falls
    /// back to `container image inspect` (image tags, e.g. the feature
    /// content image or a base image). Does not pull on miss; callers that
    /// want that retry once after [`Self::pull_image`].
    async fn try_inspect_once(&self, id: &str) -> Option<DockerInspect> {
        if let Ok(output) = self.run(inspect_args(id), no_env()).await
            && output.status.success()
            && let Ok(Some(entries)) =
                deserialize_json_output::<Vec<AppleContainerEntry>>(output)
            && let Some(entry) = entries.into_iter().next()
        {
            return entry_to_docker_inspect(entry, id).ok();
        }

        let output = self.run(image_inspect_args(id), no_env()).await.ok()?;
        if !output.status.success() {
            return None;
        }
        let entries: Vec<AppleImageEntry> =
            deserialize_json_output(output).ok().flatten()?;
        let entry = entries.into_iter().next()?;
        image_entry_to_docker_inspect(entry, id).ok()
    }
}

fn inspect_args(id: &str) -> Vec<String> {
    vec!["inspect".to_string(), id.to_string()]
}

fn image_inspect_args(id: &str) -> Vec<String> {
    vec!["image".to_string(), "inspect".to_string(), id.to_string()]
}

fn list_containers_args() -> Vec<String> {
    vec![
        "ls".to_string(),
        "-a".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ]
}

#[async_trait]
impl DockerClient for AppleContainer {
    async fn inspect(&self, id: &String) -> Result<DockerInspect, DevContainerError> {
        if let Some(inspect) = self.try_inspect_once(id).await {
            return Ok(inspect);
        }

        self.pull_image(id).await.ok();

        self.try_inspect_once(id).await.ok_or_else(|| {
            log::error!(
                "`container inspect`/`container image inspect` produced no output for {id}"
            );
            DevContainerError::CommandFailed("container".to_string())
        })
    }

    async fn get_docker_compose_config(
        &self,
        _config_files: &Vec<PathBuf>,
    ) -> Result<Option<DockerComposeConfig>, DevContainerError> {
        // Unreachable in practice: `resolve_engine` skips this engine
        // whenever the project's devcontainer.json names a compose file.
        Err(DevContainerError::UnsupportedHost("container".to_string()))
    }

    async fn docker_compose_build(
        &self,
        _config_files: &Vec<PathBuf>,
        _project_name: &str,
        _services: Option<&Vec<String>>,
    ) -> Result<(), DevContainerError> {
        Err(DevContainerError::UnsupportedHost("container".to_string()))
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
            log::error!("Non-success status from container start: {stderr}");
            return Err(DevContainerError::CommandFailed("container".to_string()));
        }
        Ok(())
    }

    async fn stop_container(&self, id: &str) -> Result<(), DevContainerError> {
        let output = self
            .run(vec!["stop".to_string(), id.to_string()], no_env())
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from container stop: {stderr}");
            return Err(DevContainerError::CommandFailed("container".to_string()));
        }
        Ok(())
    }

    async fn remove_container(&self, id: &str) -> Result<(), DevContainerError> {
        // `-f` also stops the container first if it's still running.
        let output = self
            .run(
                vec!["rm".to_string(), "-f".to_string(), id.to_string()],
                no_env(),
            )
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from container rm: {stderr}");
            return Err(DevContainerError::CommandFailed("container".to_string()));
        }
        Ok(())
    }

    async fn find_process_by_filters(
        &self,
        filters: Vec<String>,
    ) -> Result<Option<DockerPs>, DevContainerError> {
        let output = self.run(list_containers_args(), no_env()).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Non-success status from container ls: {stderr}");
            return Err(DevContainerError::CommandFailed("container".to_string()));
        }
        let entries: Vec<AppleContainerEntry> = deserialize_json_output(output)
            .map_err(|e| {
                log::error!("Error parsing container ls output: {e}");
                DevContainerError::CommandFailed("container".to_string())
            })?
            .unwrap_or_default();

        // `container ls` has no `--filter`, so the `label=k=v` matching that
        // `docker ps --filter` would have done happens here instead.
        let wanted: Vec<(&str, &str)> = filters
            .iter()
            .filter_map(|f| f.strip_prefix("label=")?.split_once('='))
            .collect();

        let mut matches = Vec::new();
        for entry in entries {
            let serde_json_lenient::Value::Object(labels) = &entry.configuration.labels else {
                continue;
            };
            let is_match = wanted
                .iter()
                .all(|(key, value)| labels.get(*key).and_then(|v| v.as_str()) == Some(*value));
            if is_match {
                matches.push(entry.id);
            }
        }

        match matches.len() {
            0 => Ok(None),
            1 => Ok(matches.pop().map(|id| DockerPs { id })),
            _ => Err(DevContainerError::MultipleMatchingContainers(matches)),
        }
    }

    fn new_command(&self) -> Command {
        Command::new("container")
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
        self.host.command("container", &args, &env, None)
    }

    fn is_podman(&self) -> bool {
        false
    }

    fn supports_sig_proxy(&self) -> bool {
        false
    }

    fn supports_mount_consistency(&self) -> bool {
        false
    }

    fn sets_buildkit_env(&self) -> bool {
        false
    }

    fn docker_cli(&self) -> String {
        "container".to_string()
    }

    fn supports_compose_buildkit(&self) -> bool {
        false
    }
}

/// One element of `container inspect`'s or `container ls --format json`'s
/// JSON array.
#[derive(Debug, Deserialize)]
struct AppleContainerEntry {
    id: String,
    configuration: AppleConfiguration,
    #[serde(default)]
    status: Option<AppleStatus>,
}

#[derive(Debug, Deserialize)]
struct AppleConfiguration {
    #[serde(default)]
    labels: serde_json_lenient::Value,
    #[serde(default)]
    mounts: Vec<AppleMount>,
    #[serde(rename = "initProcess")]
    init_process: AppleInitProcess,
}

#[derive(Debug, Deserialize)]
struct AppleMount {
    source: String,
    destination: String,
}

#[derive(Debug, Deserialize)]
struct AppleInitProcess {
    #[serde(default)]
    environment: Vec<String>,
    user: AppleUser,
}

/// `container`'s `initProcess.user` is reported as a numeric id pair when the
/// devcontainer resolved to one, or as `{"raw":{"userString":...}}` when it's
/// still just a name (e.g. `remoteUser: "root"`) that hasn't been resolved
/// against `/etc/passwd`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum AppleUser {
    Id { id: AppleUserId },
    Raw { raw: AppleRawUser },
}

#[derive(Debug, Deserialize)]
struct AppleUserId {
    uid: u32,
    gid: u32,
}

#[derive(Debug, Deserialize)]
struct AppleRawUser {
    #[serde(rename = "userString")]
    user_string: String,
}

#[derive(Debug, Deserialize)]
struct AppleStatus {
    state: String,
    #[serde(default, rename = "startedDate")]
    started_date: Option<String>,
}

fn entry_to_docker_inspect(
    entry: AppleContainerEntry,
    id: &str,
) -> Result<DockerInspect, DevContainerError> {
    let labels = match entry.configuration.labels {
        serde_json_lenient::Value::Null => DockerConfigLabels::default(),
        value => serde_json_lenient::from_value(value).map_err(|e| {
            log::error!("Error deserializing Apple Container labels: {e}");
            DevContainerError::CommandFailed("container".to_string())
        })?,
    };

    let image_user = match &entry.configuration.init_process.user {
        AppleUser::Id { id } => format!("{}:{}", id.uid, id.gid),
        AppleUser::Raw { raw } => raw.user_string.clone(),
    };

    let config = DockerInspectConfig {
        labels,
        image_user: Some(image_user),
        env: entry.configuration.init_process.environment,
    };

    let mounts = if entry.configuration.mounts.is_empty() {
        None
    } else {
        Some(
            entry
                .configuration
                .mounts
                .into_iter()
                .map(|mount| DockerInspectMount {
                    source: mount.source,
                    destination: mount.destination,
                })
                .collect(),
        )
    };

    let state = entry.status.map(|status| DockerState {
        running: status.state == "running",
        started_at: status.started_date,
    });

    Ok(DockerInspect {
        id: id.to_string(),
        config,
        mounts,
        state,
    })
}

/// One element of `container image inspect`'s JSON array.
#[derive(Debug, Deserialize)]
struct AppleImageEntry {
    variants: Vec<AppleImageVariant>,
}

#[derive(Debug, Deserialize)]
struct AppleImageVariant {
    config: AppleImageConfigWrapper,
}

/// The OCI image config nested under a variant is already Docker-shaped
/// (`Env`, `Labels`, `User`), so it decodes straight into the same struct
/// `docker inspect`'s `.Config` does.
#[derive(Debug, Deserialize)]
struct AppleImageConfigWrapper {
    config: DockerInspectConfig,
}

fn image_entry_to_docker_inspect(
    entry: AppleImageEntry,
    id: &str,
) -> Result<DockerInspect, DevContainerError> {
    // ponytail: takes the first variant rather than matching the host's
    // platform; fine while every image `container build` produces here is
    // single-arch (arm64/macOS host). Match on platform if cross-arch images
    // ever reach this path.
    let variant = entry.variants.into_iter().next().ok_or_else(|| {
        log::error!("`container image inspect` returned no variants for {id}");
        DevContainerError::ContainerNotValid(id.to_string())
    })?;

    Ok(DockerInspect {
        id: id.to_string(),
        config: variant.config.config,
        mounts: None,
        state: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Captured from `container ls -a --format json` against `container`
    /// 1.5.0 for a running container with two devcontainer identity labels.
    const RUNNING_CONTAINER_JSON: &str = r#"
    {
        "id": "probe1",
        "configuration": {
            "labels": {
                "devcontainer.local_folder": "/Users/x/proj",
                "devcontainer.config_file": "/Users/x/proj/.devcontainer/devcontainer.json"
            },
            "mounts": [],
            "initProcess": {
                "arguments": ["300"],
                "environment": [
                    "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
                    "FOO=bar"
                ],
                "user": { "id": { "uid": 0, "gid": 0 } }
            }
        },
        "status": {
            "networks": [{ "ipv4Address": "192.168.64.5/24" }],
            "startedDate": "2026-09-29T22:12:54Z",
            "state": "running"
        }
    }
    "#;

    const STOPPED_CONTAINER_JSON: &str = r#"
    {
        "id": "probe2",
        "configuration": {
            "labels": {},
            "mounts": [
                { "source": "/host/src", "destination": "/workspace" }
            ],
            "initProcess": {
                "environment": ["PATH=/usr/bin"],
                "user": { "id": { "uid": 501, "gid": 20 } }
            }
        },
        "status": {
            "networks": [],
            "state": "stopped"
        }
    }
    "#;

    /// Captured from `container ls -a --format json` for a container whose
    /// `remoteUser` is a name (e.g. `"root"`) rather than a resolved numeric
    /// id: `initProcess.user` is then `{"raw":{"userString":...}}` instead of
    /// `{"id":{"uid":...,"gid":...}}`.
    const RUNNING_CONTAINER_WITH_RAW_USER_JSON: &str = r#"
    {
        "id": "probe3",
        "configuration": {
            "labels": {},
            "mounts": [],
            "initProcess": {
                "environment": ["PATH=/usr/local/sbin:/usr/local/bin"],
                "user": { "raw": { "userString": "root" } }
            }
        },
        "status": {
            "networks": [],
            "state": "running"
        }
    }
    "#;

    /// Captured from `container image inspect alpine:3.19`, with a
    /// `devcontainer.metadata` label added to exercise that path (the real
    /// alpine image has no labels at all).
    const IMAGE_WITH_METADATA_JSON: &str = r#"
    [{
        "id": "sha256:abc123",
        "variants": [{
            "config": {
                "config": {
                    "Env": ["PATH=/usr/local/sbin:/usr/local/bin"],
                    "Labels": {
                        "devcontainer.metadata": "{\"remoteUser\":\"vscode\"}"
                    },
                    "WorkingDir": "/"
                },
                "platform": { "architecture": "arm64", "os": "linux" }
            }
        }]
    }]
    "#;

    #[test]
    fn running_container_translates_to_docker_inspect() {
        let entry: AppleContainerEntry =
            serde_json_lenient::from_str(RUNNING_CONTAINER_JSON).unwrap();
        let inspect = entry_to_docker_inspect(entry, "probe1").unwrap();

        assert!(inspect.is_running());
        assert_eq!(inspect.config.image_user, Some("0:0".to_string()));
        assert_eq!(
            inspect.config.labels.local_folder,
            Some("/Users/x/proj".to_string())
        );
        assert_eq!(
            inspect.config.labels.config_file,
            Some("/Users/x/proj/.devcontainer/devcontainer.json".to_string())
        );
        let env = inspect.config.env_as_map().unwrap();
        assert_eq!(env.get("FOO").unwrap(), "bar");
        assert!(inspect.mounts.is_none());
    }

    #[test]
    fn stopped_container_translates_mounts_and_state() {
        let entry: AppleContainerEntry =
            serde_json_lenient::from_str(STOPPED_CONTAINER_JSON).unwrap();
        let inspect = entry_to_docker_inspect(entry, "probe2").unwrap();

        assert!(!inspect.is_running());
        assert_eq!(inspect.config.image_user, Some("501:20".to_string()));
        let mounts = inspect.mounts.expect("mounts should be present");
        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].source, "/host/src");
        assert_eq!(mounts[0].destination, "/workspace");
    }

    #[test]
    fn raw_user_name_translates_to_docker_inspect() {
        let entry: AppleContainerEntry =
            serde_json_lenient::from_str(RUNNING_CONTAINER_WITH_RAW_USER_JSON).unwrap();
        let inspect = entry_to_docker_inspect(entry, "probe3").unwrap();

        assert_eq!(inspect.config.image_user, Some("root".to_string()));
    }

    #[test]
    fn image_metadata_label_survives_translation() {
        let mut entries: Vec<AppleImageEntry> =
            serde_json_lenient::from_str(IMAGE_WITH_METADATA_JSON).unwrap();
        let entry = entries.pop().unwrap();
        let inspect = image_entry_to_docker_inspect(entry, "alpine:3.19").unwrap();

        let metadata = inspect
            .config
            .labels
            .metadata
            .expect("metadata label should be parsed");
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0]["remoteUser"], "vscode");
        assert!(inspect.state.is_none());
    }

    #[test]
    fn find_process_by_filters_matches_on_labels() {
        let entries: Vec<AppleContainerEntry> = serde_json_lenient::from_str(&format!(
            "[{RUNNING_CONTAINER_JSON}, {STOPPED_CONTAINER_JSON}]"
        ))
        .unwrap();

        let wanted = [(
            "devcontainer.local_folder",
            "/Users/x/proj",
        )];
        let mut matches = Vec::new();
        for entry in entries {
            let serde_json_lenient::Value::Object(labels) = &entry.configuration.labels else {
                continue;
            };
            let is_match = wanted
                .iter()
                .all(|(key, value)| labels.get(*key).and_then(|v| v.as_str()) == Some(*value));
            if is_match {
                matches.push(entry.id);
            }
        }

        assert_eq!(matches, vec!["probe1".to_string()]);
    }
}
