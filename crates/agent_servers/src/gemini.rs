use anyhow::anyhow;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use util::ResultExt as _;

use crate::{AgentServer, AgentServerCommand, AgentServerVersion};
use acp_thread::{AgentConnection, LoadError, OldAcpAgentConnection, OldAcpClientDelegate};
use agentic_coding_protocol as acp_old;
use anyhow::{Context as _, Result};
use gpui::{AppContext as _, AsyncApp, Entity, Task, WeakEntity};
use project::Project;
use settings::SettingsStore;
use ui::App;

use crate::AllAgentServersSettings;

#[derive(Clone)]
pub struct Gemini;

const ACP_ARG: &str = "--experimental-acp";

impl AgentServer for Gemini {
    fn name(&self) -> &'static str {
        "Gemini"
    }

    fn empty_state_headline(&self) -> &'static str {
        "Welcome to Gemini"
    }

    fn empty_state_message(&self) -> &'static str {
        "Ask questions, edit files, run commands.\nBe specific for the best results."
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiGemini
    }

    fn connect(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let root_dir = root_dir.to_path_buf();
        let project = project.clone();
        let this = self.clone();
        let name = self.name();

        cx.spawn(async move |cx| {
            let command = this.command(&project, cx).await?;

            let mut child = util::command::new_smol_command(&command.path)
                .args(command.args.iter())
                .current_dir(root_dir)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::inherit())
                .kill_on_drop(true)
                .spawn()?;

            let stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            let foreground_executor = cx.foreground_executor().clone();

            let thread_rc = Rc::new(RefCell::new(WeakEntity::new_invalid()));

            let (connection, io_fut) = acp_old::AgentConnection::connect_to_agent(
                OldAcpClientDelegate::new(thread_rc.clone(), cx.clone()),
                stdin,
                stdout,
                move |fut| foreground_executor.spawn(fut).detach(),
            );

            let io_task = cx.background_spawn(async move {
                io_fut.await.log_err();
            });

            let child_status = cx.background_spawn(async move {
                let result = match child.status().await {
                    Err(e) => Err(anyhow!(e)),
                    Ok(result) if result.success() => Ok(()),
                    Ok(result) => {
                        if let Some(AgentServerVersion::Unsupported {
                            error_message,
                            upgrade_message,
                            upgrade_command,
                        }) = this.version(&command).await.log_err()
                        {
                            Err(anyhow!(LoadError::Unsupported {
                                error_message,
                                upgrade_message,
                                upgrade_command
                            }))
                        } else {
                            Err(anyhow!(LoadError::Exited(result.code().unwrap_or(-127))))
                        }
                    }
                };
                drop(io_task);
                result
            });

            let connection: Rc<dyn AgentConnection> = Rc::new(OldAcpAgentConnection {
                name,
                connection,
                child_status,
            });

            Ok(connection)
        })
    }
}

impl Gemini {
    async fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> Result<AgentServerCommand> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).gemini.clone()
        })?;

        if let Some(command) =
            AgentServerCommand::resolve("gemini", &[ACP_ARG], settings, &project, cx).await
        {
            return Ok(command);
        };

        let (fs, node_runtime) = project.update(cx, |project, _| {
            (project.fs().clone(), project.node_runtime().cloned())
        })?;
        let node_runtime = node_runtime.context("gemini not found on path")?;

        let directory = ::paths::agent_servers_dir().join("gemini");
        fs.create_dir(&directory).await?;
        node_runtime
            .npm_install_packages(&directory, &[("@google/gemini-cli", "latest")])
            .await?;
        let path = directory.join("node_modules/.bin/gemini");

        Ok(AgentServerCommand {
            path,
            args: vec![ACP_ARG.into()],
            env: None,
        })
    }

    async fn version(&self, command: &AgentServerCommand) -> Result<AgentServerVersion> {
        let version_fut = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .arg("--version")
            .kill_on_drop(true)
            .output();

        let help_fut = util::command::new_smol_command(&command.path)
            .args(command.args.iter())
            .arg("--help")
            .kill_on_drop(true)
            .output();

        let (version_output, help_output) = futures::future::join(version_fut, help_fut).await;

        let current_version = String::from_utf8(version_output?.stdout)?;
        let supported = String::from_utf8(help_output?.stdout)?.contains(ACP_ARG);

        if supported {
            Ok(AgentServerVersion::Supported)
        } else {
            Ok(AgentServerVersion::Unsupported {
                error_message: format!(
                    "Your installed version of Gemini {} doesn't support the Agentic Coding Protocol (ACP).",
                    current_version
                ).into(),
                upgrade_message: "Upgrade Gemini to Latest".into(),
                upgrade_command: "npm install -g @google/gemini-cli@latest".into(),
            })
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::AgentServerCommand;
    use std::path::Path;

    crate::common_e2e_tests!(Gemini);

    pub fn local_command() -> AgentServerCommand {
        let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../gemini-cli/packages/cli")
            .to_string_lossy()
            .to_string();

        AgentServerCommand {
            path: "node".into(),
            args: vec![cli_path, ACP_ARG.into()],
            env: None,
        }
    }
}
