use std::path::{Path, PathBuf};
use std::sync::Arc;

use gpui::AsyncWindowContext;
use node_runtime::NodeRuntime;
use serde::Deserialize;
use settings::DevContainerConnection;
use smol::fs;
use workspace::Workspace;

use crate::remote_connections::Connection;

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
struct DevContainerConfiguration {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DevContainerConfigurationOutput {
    configuration: DevContainerConfiguration,
}

#[cfg(not(target_os = "windows"))]
fn dev_container_cli() -> String {
    "devcontainer".to_string()
}

#[cfg(target_os = "windows")]
fn dev_container_cli() -> String {
    "devcontainer.cmd".to_string()
}

async fn check_for_docker() -> Result<(), DevContainerError> {
    let mut command = util::command::new_smol_command("docker");
    command.arg("--version");

    match command.output().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Unable to find docker in $PATH: {:?}", e);
            Err(DevContainerError::DockerNotAvailable)
        }
    }
}

async fn ensure_devcontainer_cli(node_runtime: NodeRuntime) -> Result<PathBuf, DevContainerError> {
    let mut command = util::command::new_smol_command(&dev_container_cli());
    command.arg("--version");

    if let Err(e) = command.output().await {
        log::error!(
            "Unable to find devcontainer CLI in $PATH. Checking for a zed installed version. Error: {:?}",
            e
        );

        let datadir_cli_path = paths::devcontainer_dir()
            .join("node_modules")
            .join(".bin")
            .join(&dev_container_cli());

        let mut command =
            util::command::new_smol_command(&datadir_cli_path.as_os_str().display().to_string());
        command.arg("--version");

        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer CLI in Data dir. Will try to install. Error: {:?}",
                e
            );
        } else {
            log::info!("Found devcontainer CLI in Data dir");
            return Ok(datadir_cli_path.clone());
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

        let mut command = util::command::new_smol_command(&datadir_cli_path.display().to_string());
        command.arg("--version");
        if let Err(e) = command.output().await {
            log::error!(
                "Unable to find devcontainer cli after NPM install. Error: {:?}",
                e
            );
            Err(DevContainerError::DevContainerCliNotAvailable)
        } else {
            Ok(datadir_cli_path)
        }
    } else {
        log::info!("Found devcontainer cli on $PATH, using it");
        Ok(PathBuf::from(&dev_container_cli()))
    }
}

async fn devcontainer_up(
    path_to_cli: &PathBuf,
    path: Arc<Path>,
) -> Result<DevContainerUp, DevContainerError> {
    let mut command = util::command::new_smol_command(path_to_cli.display().to_string());
    command.arg("up");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());

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
                log::error!(
                    "Non-success status running devcontainer up for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                Err(DevContainerError::DevContainerUpFailed)
            }
        }
        Err(e) => {
            log::error!("Error running devcontainer up: {:?}", e);
            Err(DevContainerError::DevContainerUpFailed)
        }
    }
}

async fn devcontainer_read_configuration(
    path_to_cli: &PathBuf,
    path: Arc<Path>,
) -> Result<DevContainerConfigurationOutput, DevContainerError> {
    let mut command = util::command::new_smol_command(path_to_cli.display().to_string());
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
                log::error!(
                    "Non-success status running devcontainer read-configuration for workspace: out: {:?}, err: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
                Err(DevContainerError::DevContainerUpFailed)
            }
        }
        Err(e) => {
            log::error!("Error running devcontainer read-configuration: {:?}", e);
            Err(DevContainerError::DevContainerUpFailed)
        }
    }
}

// Name the project with two fallbacks
async fn get_project_name(
    path_to_cli: &PathBuf,
    path: Arc<Path>,
    remote_workspace_folder: String,
    container_id: String,
) -> Result<String, DevContainerError> {
    if let Ok(dev_container_configuration) =
        devcontainer_read_configuration(path_to_cli, path).await
        && let Some(name) = dev_container_configuration.configuration.name
    {
        // Ideally, name the project after the name defined in devcontainer.json
        Ok(name)
    } else {
        // Otherwise, name the project after the remote workspace folder name
        Ok(Path::new(&remote_workspace_folder)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|string| string.into())
            // Finally, name the project after the container ID as a last resort
            .unwrap_or_else(|| container_id.clone()))
    }
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

pub(crate) async fn start_dev_container(
    cx: &mut AsyncWindowContext,
    node_runtime: NodeRuntime,
) -> Result<(Connection, String), DevContainerError> {
    check_for_docker().await?;

    let path_to_devcontainer_cli = ensure_devcontainer_cli(node_runtime).await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::DevContainerNotFound);
    };

    if let Ok(DevContainerUp {
        container_id,
        remote_workspace_folder,
        ..
    }) = devcontainer_up(&path_to_devcontainer_cli, directory.clone()).await
    {
        let project_name = get_project_name(
            &path_to_devcontainer_cli,
            directory,
            remote_workspace_folder.clone(),
            container_id.clone(),
        )
        .await?;

        let connection = Connection::DevContainer(DevContainerConnection {
            name: project_name.into(),
            container_id: container_id.into(),
        });

        Ok((connection, remote_workspace_folder))
    } else {
        Err(DevContainerError::DevContainerUpFailed)
    }
}

#[derive(Debug)]
pub(crate) enum DevContainerError {
    DockerNotAvailable,
    DevContainerCliNotAvailable,
    DevContainerUpFailed,
    DevContainerNotFound,
    DevContainerParseFailed,
}

#[cfg(test)]
mod test {

    use crate::dev_container::DevContainerUp;

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
