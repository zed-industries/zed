use crate::{AgentServer, AgentServerCommand, AgentServerVersion};
use acp_thread::{AcpClientDelegate, AcpThread, LoadError};
use agentic_coding_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, AsyncApp, Entity, Task, prelude::*};
use project::Project;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, paths};

pub trait StdioAgentServer: Send + Clone {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> &'static str;
    fn empty_state_headline(&self) -> &'static str;
    fn empty_state_message(&self) -> &'static str;
    fn supports_always_allow(&self) -> bool;

    fn command(
        &self,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<AgentServerCommand>>;

    fn version(
        &self,
        command: &AgentServerCommand,
    ) -> impl Future<Output = Result<AgentServerVersion>> + Send;
}

impl<T: StdioAgentServer + 'static> AgentServer for T {
    fn name(&self) -> &'static str {
        self.name()
    }

    fn empty_state_headline(&self) -> &'static str {
        self.empty_state_headline()
    }

    fn empty_state_message(&self) -> &'static str {
        self.empty_state_message()
    }

    fn logo(&self) -> ui::IconName {
        self.logo()
    }

    fn supports_always_allow(&self) -> bool {
        self.supports_always_allow()
    }

    fn new_thread(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>> {
        let root_dir = root_dir.to_path_buf();
        let project = project.clone();
        let this = self.clone();
        let title = self.name().into();

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

            cx.new(|cx| {
                let foreground_executor = cx.foreground_executor().clone();

                let (connection, io_fut) = acp::AgentConnection::connect_to_agent(
                    AcpClientDelegate::new(cx.entity().downgrade(), cx.to_async()),
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

                AcpThread::new(connection, title, Some(child_status), project.clone(), cx)
            })
        })
    }
}

pub async fn find_bin_in_path(
    bin_name: &'static str,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Option<PathBuf> {
    let (env_task, root_dir) = project
        .update(cx, |project, cx| {
            let worktree = project.visible_worktrees(cx).next();
            match worktree {
                Some(worktree) => {
                    let env_task = project.environment().update(cx, |env, cx| {
                        env.get_worktree_environment(worktree.clone(), cx)
                    });

                    let path = worktree.read(cx).abs_path();
                    (env_task, path)
                }
                None => {
                    let path: Arc<Path> = paths::home_dir().as_path().into();
                    let env_task = project.environment().update(cx, |env, cx| {
                        env.get_directory_environment(path.clone(), cx)
                    });
                    (env_task, path)
                }
            }
        })
        .log_err()?;

    cx.background_executor()
        .spawn(async move {
            let which_result = if cfg!(windows) {
                which::which(bin_name)
            } else {
                let env = env_task.await.unwrap_or_default();
                let shell_path = env.get("PATH").cloned();
                which::which_in(bin_name, shell_path.as_ref(), root_dir.as_ref())
            };

            if let Err(which::Error::CannotFindBinaryPath) = which_result {
                return None;
            }

            which_result.log_err()
        })
        .await
}
