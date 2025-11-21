use std::{path::Path, sync::Arc};

use gpui::AsyncWindowContext;
use serde::Deserialize;
use settings::DevContainerConnection;
use workspace::Workspace;

use crate::remote_connections::Connection;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerUp {
    outcome: String,
    container_id: String,
    remote_user: String,
    remote_workspace_folder: String,
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

// TODO we probably want to package this with Zed
async fn check_for_devcontainer_cli() -> Result<(), DevContainerError> {
    let mut command = util::command::new_smol_command("devcontainer");
    command.arg("--version");

    match command.output().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Unable to find devcontainer CLI in $PATH: {:?}", e);
            Err(DevContainerError::DevContainerCliNotAvailable)
        }
    }
}

async fn devcontainer_up(path: Arc<Path>) -> Result<DevContainerUp, DevContainerError> {
    let mut command = util::command::new_smol_command("devcontainer");
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
) -> Result<(Connection, String), DevContainerError> {
    check_for_docker().await?;

    check_for_devcontainer_cli().await?;

    let Some(directory) = project_directory(cx) else {
        return Err(DevContainerError::DevContainerNotFound);
    };

    if let Ok(DevContainerUp {
        container_id,
        remote_workspace_folder,
        ..
    }) = devcontainer_up(directory).await
    {
        let connection = Connection::DevContainer(DevContainerConnection {
            name: container_id.clone().into(),
            image: "mcr.microsoft.com/devcontainers/rust:latest".into(),
            container_id: container_id.into(),
            working_directory: remote_workspace_folder.clone().into(),
        });

        Ok((connection, remote_workspace_folder.into()))
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
        assert_eq!(up.outcome, "success");
        assert_eq!(
            up.container_id,
            "826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a"
        );
        assert_eq!(up.remote_user, "vscode");
        assert_eq!(up.remote_workspace_folder, "/workspaces/zed");
    }
}
