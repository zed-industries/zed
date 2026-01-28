use super::{KernelSession, RunningKernel, SshRemoteKernelSpecification};
use anyhow::{Context as _, Result};
use client::proto;
use futures::stream::SelectAll;
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext, Entity, Task, Window};
use project::Project;
use runtimelib::{ExecutionState, JupyterMessage, JupyterMessageContent, KernelInfoReply};
use std::path::PathBuf;
use util::ResultExt;

#[derive(Debug)]
pub struct SshRunningKernel {
    request_tx: mpsc::Sender<JupyterMessage>,
    execution_state: ExecutionState,
    kernel_info: Option<KernelInfoReply>,
    working_directory: PathBuf,
    _ssh_tunnel_process: smol::process::Child,
    _local_connection_file: PathBuf,
    kernel_id: String,
    project: Entity<Project>,
    project_id: u64,
}

impl SshRunningKernel {
    pub fn new<S: KernelSession + 'static>(
        kernel_spec: SshRemoteKernelSpecification,
        working_directory: PathBuf,
        project: Entity<Project>,
        session: Entity<S>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn RunningKernel>>> {
        let client = project.read(cx).client();
        let remote_client = project.read(cx).remote_client();
        let project_id_opt = project.read(cx).remote_id();

        window.spawn(cx, async move |cx| {
            let project_id =
                project_id_opt.ok_or_else(|| anyhow::anyhow!("not connected to remote project"))?;

            let request = proto::SpawnKernel {
                kernel_name: kernel_spec.name.clone(),
                working_directory: working_directory.to_string_lossy().to_string(),
                project_id,
            };
            let response = client.request::<proto::SpawnKernel>(request).await?;

            let kernel_id = response.kernel_id.clone();
            let connection_info: serde_json::Value =
                serde_json::from_str(&response.connection_file)?;

            // Setup SSH Tunneling - allocate local ports
            let mut local_ports = Vec::new();
            for _ in 0..5 {
                let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
                let port = listener.local_addr()?.port();
                drop(listener);
                local_ports.push(port);
            }

            let remote_shell_port = connection_info["shell_port"]
                .as_u64()
                .context("missing shell_port")? as u16;
            let remote_iopub_port = connection_info["iopub_port"]
                .as_u64()
                .context("missing iopub_port")? as u16;
            let remote_stdin_port = connection_info["stdin_port"]
                .as_u64()
                .context("missing stdin_port")? as u16;
            let remote_control_port = connection_info["control_port"]
                .as_u64()
                .context("missing control_port")? as u16;
            let remote_hb_port = connection_info["hb_port"]
                .as_u64()
                .context("missing hb_port")? as u16;

            let forwards = vec![
                (local_ports[0], "127.0.0.1".to_string(), remote_shell_port),
                (local_ports[1], "127.0.0.1".to_string(), remote_iopub_port),
                (local_ports[2], "127.0.0.1".to_string(), remote_stdin_port),
                (local_ports[3], "127.0.0.1".to_string(), remote_control_port),
                (local_ports[4], "127.0.0.1".to_string(), remote_hb_port),
            ];

            let remote_client = remote_client.ok_or_else(|| anyhow::anyhow!("no remote client"))?;
            let command_template = cx.update(|_window, cx| {
                remote_client.read(cx).build_forward_ports_command(forwards)
            })??;

            let mut command = util::command::new_smol_command(&command_template.program);
            command.args(&command_template.args);
            command.envs(&command_template.env);

            let ssh_tunnel_process = command.spawn().context("failed to spawn ssh tunnel")?;

            // We might or might not need this, perhaps we can just wait for a second or test it this way
            let shell_port = local_ports[0];
            let max_attempts = 20;
            let mut connected = false;
            for attempt in 0..max_attempts {
                match smol::net::TcpStream::connect(format!("127.0.0.1:{}", shell_port)).await {
                    Ok(_) => {
                        connected = true;
                        break;
                    }
                    Err(_) => {
                        if attempt < max_attempts - 1 {
                            smol::Timer::after(std::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            }
            if !connected {
                anyhow::bail!(
                    "SSH tunnel failed to establish after {} attempts",
                    max_attempts
                );
            }

            let mut local_connection_info = connection_info.clone();
            local_connection_info["shell_port"] = serde_json::json!(local_ports[0]);
            local_connection_info["iopub_port"] = serde_json::json!(local_ports[1]);
            local_connection_info["stdin_port"] = serde_json::json!(local_ports[2]);
            local_connection_info["control_port"] = serde_json::json!(local_ports[3]);
            local_connection_info["hb_port"] = serde_json::json!(local_ports[4]);
            local_connection_info["ip"] = serde_json::json!("127.0.0.1");

            let local_connection_file =
                std::env::temp_dir().join(format!("zed_ssh_kernel_{}.json", kernel_id));
            std::fs::write(
                &local_connection_file,
                serde_json::to_string_pretty(&local_connection_info)?,
            )?;

            // Parse connection info and create ZMQ connections
            let connection_info_struct: runtimelib::ConnectionInfo =
                serde_json::from_value(local_connection_info)?;
            let session_id = uuid::Uuid::new_v4().to_string();

            let mut iopub_socket = runtimelib::create_client_iopub_connection(
                &connection_info_struct,
                "",
                &session_id,
            )
            .await
            .context("failed to create iopub connection")?;
            let mut shell_socket =
                runtimelib::create_client_shell_connection(&connection_info_struct, &session_id)
                    .await
                    .context("failed to create shell connection")?;
            let mut control_socket =
                runtimelib::create_client_control_connection(&connection_info_struct, &session_id)
                    .await
                    .context("failed to create control connection")?;

            let (request_tx, mut request_rx) = mpsc::channel::<JupyterMessage>(100);
            let (mut control_reply_tx, control_reply_rx) = mpsc::channel(100);
            let (mut shell_reply_tx, shell_reply_rx) = mpsc::channel(100);
            let (mut control_request_tx, mut control_request_rx) = mpsc::channel(100);
            let (mut shell_request_tx, mut shell_request_rx) = mpsc::channel(100);

            let mut messages_rx = SelectAll::new();
            messages_rx.push(control_reply_rx);
            messages_rx.push(shell_reply_rx);

            cx.spawn({
                let session = session.clone();
                async move |cx| {
                    while let Some(message) = messages_rx.next().await {
                        session
                            .update_in(cx, |session, window, cx| {
                                session.route(&message, window, cx);
                            })
                            .log_err();
                    }
                }
            })
            .detach();

            cx.spawn({
                let session = session.clone();
                async move |cx| {
                    loop {
                        match iopub_socket.read().await {
                            Ok(message) => {
                                session
                                    .update_in(cx, |session, window, cx| {
                                        session.route(&message, window, cx);
                                    })
                                    .log_err();
                            }
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            })
            .detach();

            cx.background_spawn(async move {
                while let Some(message) = request_rx.next().await {
                    match message.content {
                        JupyterMessageContent::DebugRequest(_)
                        | JupyterMessageContent::InterruptRequest(_)
                        | JupyterMessageContent::ShutdownRequest(_) => {
                            control_request_tx.send(message).await?;
                        }
                        _ => {
                            shell_request_tx.send(message).await?;
                        }
                    }
                }
                anyhow::Ok(())
            })
            .detach();

            cx.background_spawn(async move {
                while let Some(message) = shell_request_rx.next().await {
                    shell_socket.send(message).await.log_err();
                    let reply = shell_socket.read().await?;
                    shell_reply_tx.send(reply).await?;
                }
                anyhow::Ok(())
            })
            .detach();

            cx.background_spawn(async move {
                while let Some(message) = control_request_rx.next().await {
                    control_socket.send(message).await.log_err();
                    let reply = control_socket.read().await?;
                    control_reply_tx.send(reply).await?;
                }
                anyhow::Ok(())
            })
            .detach();

            Ok(Box::new(SshRunningKernel {
                request_tx,
                execution_state: ExecutionState::Idle,
                kernel_info: None,
                working_directory,
                _ssh_tunnel_process: ssh_tunnel_process,
                _local_connection_file: local_connection_file,
                kernel_id,
                project,
                project_id,
            }) as Box<dyn RunningKernel>)
        })
    }
}

impl RunningKernel for SshRunningKernel {
    fn request_tx(&self) -> mpsc::Sender<JupyterMessage> {
        self.request_tx.clone()
    }

    fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    fn execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }

    fn set_execution_state(&mut self, state: ExecutionState) {
        self.execution_state = state;
    }

    fn kernel_info(&self) -> Option<&KernelInfoReply> {
        self.kernel_info.as_ref()
    }

    fn set_kernel_info(&mut self, info: KernelInfoReply) {
        self.kernel_info = Some(info);
    }

    fn force_shutdown(&mut self, _window: &mut Window, cx: &mut App) -> Task<Result<()>> {
        let kernel_id = self.kernel_id.clone();
        let project_id = self.project_id;
        let client = self.project.read(cx).client();

        cx.background_executor().spawn(async move {
            let request = proto::KillKernel {
                kernel_id,
                project_id,
            };
            client.request::<proto::KillKernel>(request).await?;
            Ok(())
        })
    }
}
