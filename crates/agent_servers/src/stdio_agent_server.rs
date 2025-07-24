use crate::{AgentServer, AgentServerCommand, AgentServerVersion};
use acp_thread::{AgentConnection, LoadError, OldAcpAgentConnection, OldAcpClientDelegate};
use agentic_coding_protocol as acp_old;
use anyhow::{Result, anyhow};
use gpui::{App, AsyncApp, Entity, Task, WeakEntity, prelude::*};
use project::Project;
use std::{cell::RefCell, path::Path, rc::Rc};
use util::ResultExt;

pub trait StdioAgentServer: Send + Clone {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> &'static str;
    fn empty_state_headline(&self) -> &'static str;
    fn empty_state_message(&self) -> &'static str;

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

    fn connect(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let root_dir = root_dir.to_path_buf();
        let project = project.clone();
        let this = self.clone();

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
                connection,
                child_status,
            });

            Ok(connection)
        })
    }
}
