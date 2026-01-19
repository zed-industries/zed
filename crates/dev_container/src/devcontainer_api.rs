use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};

use gpui::AsyncWindowContext;
use node_runtime::NodeRuntime;
use serde::Deserialize;
use settings::{DevContainerConnection, Settings as _};
use smol::{fs, process::Command};
use workspace::Workspace;

use crate::{DevContainerFeature, DevContainerSettings, DevContainerTemplate};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerUp {
    _outcome: String,
    container_id: String,
    _remote_user: String,
    remote_workspace_folder: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerApply {
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DevContainerConfiguration {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DevContainerConfigurationOutput {
    configuration: DevContainerConfiguration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevContainerError {
    DockerNotAvailable,
    DevContainerCliNotAvailable,
    DevContainerTemplateApplyFailed(String),
    DevContainerUpFailed(String),
    DevContainerNotFound,
    DevContainerParseFailed,
    NodeRuntimeNotAvailable,
    NotInValidProject,
}

impl Display for DevContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DevContainerError::DockerNotAvailable =>
                    "Docker CLI not found on $PATH".to_string(),
                DevContainerError::DevContainerCliNotAvailable =>
                    "Docker not found on path".to_string(),
                DevContainerError::DevContainerUpFailed(message) => {
                    format!("DevContainer creation failed with error: {}", message)
                }
                DevContainerError::DevContainerTemplateApplyFailed(message) => {
                    format!("DevContainer template apply failed with error: {}", message)
                }
                DevContainerError::DevContainerNotFound =>
                    "No valid dev container definition found in project".to_string(),
                DevContainerError::DevContainerParseFailed =>
                    "Failed to parse file .devcontainer/devcontainer.json".to_string(),
                DevContainerError::NodeRuntimeNotAvailable =>
                    "Cannot find a valid node runtime".to_string(),
                DevContainerError::NotInValidProject => "Not within a valid project".to_string(),
            }
        )
    }
}

pub(crate) async fn read_devcontainer_configuration_for_project(
    cx: &mut AsyncWindowContext,
    node_runtime: &NodeRuntime,
) -> Result<DevContainerConfigurationOutput, DevContainerError> {
    let (path_to_devcontainer_cli, found_in_path) = ensure_devcontainer_cli(&node_runtime).await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::NotInValidProject);
    };

    devcontainer_read_configuration(
        &path_to_devcontainer_cli,
        found_in_path,
        node_runtime,
        &directory,
        use_podman(cx),
    )
    .await
}

pub(crate) async fn apply_dev_container_template(
    template: &DevContainerTemplate,
    options_selected: &HashMap<String, String>,
    features_selected: &HashSet<DevContainerFeature>,
    cx: &mut AsyncWindowContext,
    node_runtime: &NodeRuntime,
) -> Result<DevContainerApply, DevContainerError> {
    let (path_to_devcontainer_cli, found_in_path) = ensure_devcontainer_cli(&node_runtime).await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::NotInValidProject);
    };

    devcontainer_template_apply(
        template,
        options_selected,
        features_selected,
        &path_to_devcontainer_cli,
        found_in_path,
        node_runtime,
        &directory,
        false, // devcontainer template apply does not use --docker-path option
    )
    .await
}

fn use_podman(cx: &mut AsyncWindowContext) -> bool {
    cx.update(|_, cx| DevContainerSettings::get_global(cx).use_podman)
        .unwrap_or(false)
}

pub async fn start_dev_container(
    cx: &mut AsyncWindowContext,
    node_runtime: NodeRuntime,
) -> Result<(DevContainerConnection, String), DevContainerError> {
    let use_podman = use_podman(cx);
    check_for_docker(use_podman).await?;

    let (path_to_devcontainer_cli, found_in_path) = ensure_devcontainer_cli(&node_runtime).await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::NotInValidProject);
    };

    match devcontainer_up(
        &path_to_devcontainer_cli,
        found_in_path,
        &node_runtime,
        directory.clone(),
        use_podman,
    )
    .await
    {
        Ok(DevContainerUp {
            container_id,
            remote_workspace_folder,
            ..
        }) => {
            let project_name = match devcontainer_read_configuration(
                &path_to_devcontainer_cli,
                found_in_path,
                &node_runtime,
                &directory,
                use_podman,
            )
            .await
            {
                Ok(DevContainerConfigurationOutput {
                    configuration:
                        DevContainerConfiguration {
                            name: Some(project_name),
                        },
                }) => project_name,
                _ => get_backup_project_name(&remote_workspace_folder, &container_id),
            };

            let connection = DevContainerConnection {
                name: project_name,
                container_id: container_id,
                use_podman,
            };

            Ok((connection, remote_workspace_folder))
        }
        Err(err) => {
            let message = format!("Failed with nested error: {}", err);
            Err(DevContainerError::DevContainerUpFailed(message))
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn dev_container_cli() -> String {
    "devcontainer".to_string()
}

#[cfg(target_os = "windows")]
fn dev_container_cli() -> String {
    "devcontainer.cmd".to_string()
}

async fn check_for_docker(use_podman: bool) -> Result<(), DevContainerError> {
    let mut command = if use_podman {
        util::command::new_smol_command("podman")
    } else {
        util::command::new_smol_command("docker")
    };
    command.arg("--version");

    match command.output().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Unable to find docker in $PATH: {:?}", e);
            Err(DevContainerError::DockerNotAvailable)
        }
    }
}

async fn ensure_devcontainer_cli(
    node_runtime: &NodeRuntime,
) -> Result<(PathBuf, bool), DevContainerError> {
    let mut command = util::command::new_smol_command(&dev_container_cli());
    command.arg("--version");

    if let Err(e) = command.output().await {
        log::error!(
            "Unable to find devcontainer CLI in $PATH. Checking for a zed installed version. Error: {:?}",
            e
        );

        let Ok(node_runtime_path) = node_runtime.binary_path().await else {
            return Err(DevContainerError::NodeRuntimeNotAvailable);
        };

        let datadir_cli_path = paths::devcontainer_dir()
            .join("node_modules")
            .join("@devcontainers")
            .join("cli")
            .join(format!("{}.js", &dev_container_cli()));

        log::debug!(
            "devcontainer not found in path, using local location: ${}",
            datadir_cli_path.display()
        );

        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(datadir_cli_path.display().to_string());
        command.arg("--version");

        match command.output().await {
            Err(e) => log::error!(
                "Unable to find devcontainer CLI in Data dir. Will try to install. Error: {:?}",
                e
            ),
            Ok(output) => {
                if output.status.success() {
                    log::info!("Found devcontainer CLI in Data dir");
                    return Ok((datadir_cli_path.clone(), false));
                } else {
                    log::error!(
                        "Could not run devcontainer CLI from data_dir. Will try once more to install. Output: {:?}",
                        output
                    );
                }
            }
        }

        if let Err(e) = fs::create_dir_all(paths::devcontainer_dir()).await {
            log::error!("Unable to create devcontainer directory. Error: {:?}", e);
            return Err(DevContainerError::DevContainerCliNotAvailable);
        }

        if let Err(e) = node_runtime
            .npm_install_packages(
                &paths::devcontainer_dir(),
                &[("@devcontainers/cli", "latest")],
            )
            .await
        {
            log::error!(
                "Unable to install devcontainer CLI to data directory. Error: {:?}",
                e
            );
            return Err(DevContainerError::DevContainerCliNotAvailable);
        };

        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(datadir_cli_path.display().to_string());
        command.arg("--version");
        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer cli after NPM install. Error: {:?}",
                e
            );
            Err(DevContainerError::DevContainerCliNotAvailable)
        } else {
            Ok((datadir_cli_path, false))
        }
    } else {
        log::info!("Found devcontainer cli on $PATH, using it");
        Ok((PathBuf::from(&dev_container_cli()), true))
    }
}

async fn devcontainer_up(
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: &NodeRuntime,
    path: Arc<Path>,
    use_podman: bool,
) -> Result<DevContainerUp, DevContainerError> {
    let Ok(node_runtime_path) = node_runtime.binary_path().await else {
        log::error!("Unable to find node runtime path");
        return Err(DevContainerError::NodeRuntimeNotAvailable);
    };

    let mut command =
        devcontainer_cli_command(path_to_cli, found_in_path, &node_runtime_path, use_podman);
    command.arg("up");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());

    log::info!("Running full devcontainer up command: {:?}", command);

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<DevContainerUp>(&raw).map_err(|e| {
                    log::error!(
                        "Unable to parse response from 'devcontainer up' command, error: {:?}",
                        e
                    );
                    DevContainerError::DevContainerParseFailed
                })
            } else {
                let message = format!(
                    "Non-success status running devcontainer up for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );

                log::error!("{}", &message);
                Err(DevContainerError::DevContainerUpFailed(message))
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer up: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerUpFailed(message))
        }
    }
}
async fn devcontainer_read_configuration(
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: &NodeRuntime,
    path: &Arc<Path>,
    use_podman: bool,
) -> Result<DevContainerConfigurationOutput, DevContainerError> {
    let Ok(node_runtime_path) = node_runtime.binary_path().await else {
        log::error!("Unable to find node runtime path");
        return Err(DevContainerError::NodeRuntimeNotAvailable);
    };

    let mut command =
        devcontainer_cli_command(path_to_cli, found_in_path, &node_runtime_path, use_podman);
    command.arg("read-configuration");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<DevContainerConfigurationOutput>(&raw).map_err(|e| {
                    log::error!(
                        "Unable to parse response from 'devcontainer read-configuration' command, error: {:?}",
                        e
                    );
                    DevContainerError::DevContainerParseFailed
                })
            } else {
                let message = format!(
                    "Non-success status running devcontainer read-configuration for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                log::error!("{}", &message);
                Err(DevContainerError::DevContainerNotFound)
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer read-configuration: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerNotFound)
        }
    }
}

async fn devcontainer_template_apply(
    template: &DevContainerTemplate,
    template_options: &HashMap<String, String>,
    features_selected: &HashSet<DevContainerFeature>,
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime: &NodeRuntime,
    path: &Arc<Path>,
    use_podman: bool,
) -> Result<DevContainerApply, DevContainerError> {
    let Ok(node_runtime_path) = node_runtime.binary_path().await else {
        log::error!("Unable to find node runtime path");
        return Err(DevContainerError::NodeRuntimeNotAvailable);
    };

    let mut command =
        devcontainer_cli_command(path_to_cli, found_in_path, &node_runtime_path, use_podman);

    let Ok(serialized_options) = serde_json::to_string(template_options) else {
        log::error!("Unable to serialize options for {:?}", template_options);
        return Err(DevContainerError::DevContainerParseFailed);
    };

    command.arg("templates");
    command.arg("apply");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());
    command.arg("--template-id");
    command.arg(format!(
        "{}/{}",
        template
            .source_repository
            .as_ref()
            .unwrap_or(&String::from("")),
        template.id
    ));
    command.arg("--template-args");
    command.arg(serialized_options);
    command.arg("--features");
    command.arg(template_features_to_json(features_selected));

    log::debug!("Running full devcontainer apply command: {:?}", command);

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                let raw = String::from_utf8_lossy(&output.stdout);
                serde_json::from_str::<DevContainerApply>(&raw).map_err(|e| {
                    log::error!(
                        "Unable to parse response from 'devcontainer templates apply' command, error: {:?}",
                        e
                    );
                    DevContainerError::DevContainerParseFailed
                })
            } else {
                let message = format!(
                    "Non-success status running devcontainer templates apply for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );

                log::error!("{}", &message);
                Err(DevContainerError::DevContainerTemplateApplyFailed(message))
            }
        }
        Err(e) => {
            let message = format!("Error running devcontainer templates apply: {:?}", e);
            log::error!("{}", &message);
            Err(DevContainerError::DevContainerTemplateApplyFailed(message))
        }
    }
}

fn devcontainer_cli_command(
    path_to_cli: &PathBuf,
    found_in_path: bool,
    node_runtime_path: &PathBuf,
    use_podman: bool,
) -> Command {
    let mut command = if found_in_path {
        util::command::new_smol_command(path_to_cli.display().to_string())
    } else {
        let mut command =
            util::command::new_smol_command(node_runtime_path.as_os_str().display().to_string());
        command.arg(path_to_cli.display().to_string());
        command
    };

    if use_podman {
        command.arg("--docker-path");
        command.arg("podman");
    }
    command
}

fn get_backup_project_name(remote_workspace_folder: &str, container_id: &str) -> String {
    Path::new(remote_workspace_folder)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|string| string.to_string())
        .unwrap_or_else(|| container_id.to_string())
}

fn project_directory(cx: &mut AsyncWindowContext) -> Option<Arc<Path>> {
    let Some(workspace) = cx.window_handle().downcast::<Workspace>() else {
        return None;
    };

    match workspace.update(cx, |workspace, _, cx| {
        workspace.project().read(cx).active_project_directory(cx)
    }) {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Error getting project directory from workspace: {:?}", e);
            None
        }
    }
}

fn template_features_to_json(features_selected: &HashSet<DevContainerFeature>) -> String {
    let things = features_selected
        .iter()
        .map(|feature| {
            let mut map = HashMap::new();
            map.insert(
                "id",
                format!(
                    "{}/{}:{}",
                    feature
                        .source_repository
                        .as_ref()
                        .unwrap_or(&String::from("")),
                    feature.id,
                    feature.major_version()
                ),
            );
            map
        })
        .collect::<Vec<HashMap<&str, String>>>();
    serde_json::to_string(&things).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::devcontainer_api::DevContainerUp;

    #[test]
    fn should_parse_from_devcontainer_json() {
        let json = r#"{"outcome":"success","containerId":"826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a","remoteUser":"vscode","remoteWorkspaceFolder":"/workspaces/zed"}"#;
        let up: DevContainerUp = serde_json::from_str(json).unwrap();
        assert_eq!(up._outcome, "success");
        assert_eq!(
            up.container_id,
            "826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a"
        );
        assert_eq!(up._remote_user, "vscode");
        assert_eq!(up.remote_workspace_folder, "/workspaces/zed");
    }
}
