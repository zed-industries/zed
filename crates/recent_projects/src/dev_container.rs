// TODO, parse devcontainer.json from here
// Things to do:
// - Runs `devcontainer up`
// - Parses the output, and provides the project for opening

use std::{path::Path, sync::Arc};

use gpui::{AppContext, AsyncWindowContext};
use language::Patch;
use project::Project;
use serde::Deserialize;
use settings::DevContainerConnection;
use workspace::Workspace;

use crate::remote_connections::Connection;

// {"outcome":"success","containerId":"826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a","remoteUser":"vscode","remoteWorkspaceFolder":"/workspaces/zed"}
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
// {"outcome":"success","containerId":"826abcac45afd412abff083ab30793daff2f3c8ce2c831df728baf39933cb37a","remoteUser":"vscode","remoteWorkspaceFolder":"/workspaces/zed"}
async fn devcontainer_up(path: Arc<Path>) -> Result<DevContainerUp, DevContainerError> {
    let mut command = util::command::new_smol_command("devcontainer");
    command.arg("up");
    command.arg("--workspace-folder");
    command.arg(path.display().to_string());

    match command.output().await {
        Ok(output) => {
            let raw = String::from_utf8_lossy(&output.stdout);
            log::info!("Response is {}", raw);
            let parsed: DevContainerUp = serde_json::from_str(&raw).unwrap();
            Ok(parsed)
        }
        Err(e) => {
            log::error!("Unable to find devcontainer CLI in $PATH: {:?}", e);
            Err(DevContainerError::DevContainerCliNotAvailable)
        }
    }
}

fn project_directory(cx: &mut AsyncWindowContext) -> Arc<Path> {
    let workspace = cx.window_handle().downcast::<Workspace>().unwrap();

    let foo = workspace.update(cx, |workspace, _, cx| {
        workspace.project().read(cx).active_project_directory(cx)
    });

    foo.unwrap().expect("is some") // Has to be handled since there is a chance there's no project open
}

pub(crate) async fn start_dev_container(
    cx: &mut AsyncWindowContext,
) -> Result<(Connection, String), DevContainerError> {
    check_for_docker().await?;

    check_for_devcontainer_cli().await?;

    let directory = project_directory(cx);

    if let Ok(DevContainerUp {
        outcome,
        container_id,
        remote_user,
        remote_workspace_folder,
    }) = devcontainer_up(directory).await
    {
        let connection = Connection::DevContainer(DevContainerConnection {
            name: "test".into(), // TODO is this needed
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
