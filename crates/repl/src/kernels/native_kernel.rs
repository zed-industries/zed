use anyhow::{Context as _, Result};
use futures::{
    channel::mpsc::{self},
    stream::{SelectAll, StreamExt},
    SinkExt as _,
};
use gpui::{AppContext, EntityId, Task};
use jupyter_protocol::{JupyterMessage, JupyterMessageContent, KernelInfoReply};
use project::Fs;
use runtimelib::{dirs, ConnectionInfo, ExecutionState, JupyterKernelspec};
use smol::{net::TcpListener, process::Command};
use std::{
    env,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use uuid::Uuid;

use super::{JupyterMessageChannel, RunningKernel};

#[derive(Debug, Clone)]
pub struct LocalKernelSpecification {
    pub name: String,
    pub path: PathBuf,
    pub kernelspec: JupyterKernelspec,
}

impl PartialEq for LocalKernelSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl Eq for LocalKernelSpecification {}

impl LocalKernelSpecification {
    #[must_use]
    fn command(&self, connection_path: &PathBuf) -> Result<Command> {
        let argv = &self.kernelspec.argv;

        anyhow::ensure!(!argv.is_empty(), "Empty argv in kernelspec {}", self.name);
        anyhow::ensure!(argv.len() >= 2, "Invalid argv in kernelspec {}", self.name);
        anyhow::ensure!(
            argv.iter().any(|arg| arg == "{connection_file}"),
            "Missing 'connection_file' in argv in kernelspec {}",
            self.name
        );

        let mut cmd = Command::new(&argv[0]);

        for arg in &argv[1..] {
            if arg == "{connection_file}" {
                cmd.arg(connection_path);
            } else {
                cmd.arg(arg);
            }
        }

        if let Some(env) = &self.kernelspec.env {
            cmd.envs(env);
        }

        #[cfg(windows)]
        {
            use smol::process::windows::CommandExt;
            cmd.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);
        }

        Ok(cmd)
    }
}

// Find a set of open ports. This creates a listener with port set to 0. The listener will be closed at the end when it goes out of scope.
// There's a race condition between closing the ports and usage by a kernel, but it's inherent to the Jupyter protocol.
async fn peek_ports(ip: IpAddr) -> Result<[u16; 5]> {
    let mut addr_zeroport: SocketAddr = SocketAddr::new(ip, 0);
    addr_zeroport.set_port(0);
    let mut ports: [u16; 5] = [0; 5];
    for i in 0..5 {
        let listener = TcpListener::bind(addr_zeroport).await?;
        let addr = listener.local_addr()?;
        ports[i] = addr.port();
    }
    Ok(ports)
}

pub struct NativeRunningKernel {
    pub process: smol::process::Child,
    _shell_task: Task<Result<()>>,
    _iopub_task: Task<Result<()>>,
    _control_task: Task<Result<()>>,
    _routing_task: Task<Result<()>>,
    connection_path: PathBuf,
    pub working_directory: PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
}

impl Debug for NativeRunningKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningKernel")
            .field("process", &self.process)
            .finish()
    }
}

impl NativeRunningKernel {
    pub fn new(
        kernel_specification: LocalKernelSpecification,
        entity_id: EntityId,
        working_directory: PathBuf,
        fs: Arc<dyn Fs>,
        cx: &mut AppContext,
    ) -> Task<Result<(Self, JupyterMessageChannel)>> {
        cx.spawn(|cx| async move {
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            let ports = peek_ports(ip).await?;

            let connection_info = ConnectionInfo {
                transport: "tcp".to_string(),
                ip: ip.to_string(),
                stdin_port: ports[0],
                control_port: ports[1],
                hb_port: ports[2],
                shell_port: ports[3],
                iopub_port: ports[4],
                signature_scheme: "hmac-sha256".to_string(),
                key: uuid::Uuid::new_v4().to_string(),
                kernel_name: Some(format!("zed-{}", kernel_specification.name)),
            };

            let runtime_dir = dirs::runtime_dir();
            fs.create_dir(&runtime_dir)
                .await
                .with_context(|| format!("Failed to create jupyter runtime dir {runtime_dir:?}"))?;
            let connection_path = runtime_dir.join(format!("kernel-zed-{entity_id}.json"));
            let content = serde_json::to_string(&connection_info)?;
            fs.atomic_write(connection_path.clone(), content).await?;

            let mut cmd = kernel_specification.command(&connection_path)?;

            let process = cmd
                .current_dir(&working_directory)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .stdin(std::process::Stdio::piped())
                .kill_on_drop(true)
                .spawn()
                .context("failed to start the kernel process")?;

            let session_id = Uuid::new_v4().to_string();

            let mut iopub_socket =
                runtimelib::create_client_iopub_connection(&connection_info, "", &session_id)
                    .await?;
            let mut shell_socket =
                runtimelib::create_client_shell_connection(&connection_info, &session_id).await?;
            let mut control_socket =
                runtimelib::create_client_control_connection(&connection_info, &session_id).await?;

            let (mut iopub, iosub) = futures::channel::mpsc::channel(100);

            let (request_tx, mut request_rx) =
                futures::channel::mpsc::channel::<JupyterMessage>(100);

            let (mut control_reply_tx, control_reply_rx) = futures::channel::mpsc::channel(100);
            let (mut shell_reply_tx, shell_reply_rx) = futures::channel::mpsc::channel(100);

            let mut messages_rx = SelectAll::new();
            messages_rx.push(iosub);
            messages_rx.push(control_reply_rx);
            messages_rx.push(shell_reply_rx);

            let iopub_task = cx.background_executor().spawn({
                async move {
                    while let Ok(message) = iopub_socket.read().await {
                        iopub.send(message).await?;
                    }
                    anyhow::Ok(())
                }
            });

            let (mut control_request_tx, mut control_request_rx) =
                futures::channel::mpsc::channel(100);
            let (mut shell_request_tx, mut shell_request_rx) = futures::channel::mpsc::channel(100);

            let routing_task = cx.background_executor().spawn({
                async move {
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
                }
            });

            let shell_task = cx.background_executor().spawn({
                async move {
                    while let Some(message) = shell_request_rx.next().await {
                        shell_socket.send(message).await.ok();
                        let reply = shell_socket.read().await?;
                        shell_reply_tx.send(reply).await?;
                    }
                    anyhow::Ok(())
                }
            });

            let control_task = cx.background_executor().spawn({
                async move {
                    while let Some(message) = control_request_rx.next().await {
                        control_socket.send(message).await.ok();
                        let reply = control_socket.read().await?;
                        control_reply_tx.send(reply).await?;
                    }
                    anyhow::Ok(())
                }
            });

            anyhow::Ok((
                Self {
                    process,
                    request_tx,
                    working_directory,
                    _shell_task: shell_task,
                    _iopub_task: iopub_task,
                    _control_task: control_task,
                    _routing_task: routing_task,
                    connection_path,
                    execution_state: ExecutionState::Idle,
                    kernel_info: None,
                },
                messages_rx,
            ))
        })
    }
}

impl RunningKernel for NativeRunningKernel {
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

    fn force_shutdown(&mut self) -> anyhow::Result<()> {
        match self.process.kill() {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow::anyhow!(
                "Failed to kill the kernel process: {}",
                error
            )),
        }
    }
}

impl Drop for NativeRunningKernel {
    fn drop(&mut self) {
        std::fs::remove_file(&self.connection_path).ok();
        self.request_tx.close_channel();
        self.process.kill().ok();
    }
}

async fn read_kernelspec_at(
    // Path should be a directory to a jupyter kernelspec, as in
    // /usr/local/share/jupyter/kernels/python3
    kernel_dir: PathBuf,
    fs: &dyn Fs,
) -> Result<LocalKernelSpecification> {
    let path = kernel_dir;
    let kernel_name = if let Some(kernel_name) = path.file_name() {
        kernel_name.to_string_lossy().to_string()
    } else {
        anyhow::bail!("Invalid kernelspec directory: {path:?}");
    };

    if !fs.is_dir(path.as_path()).await {
        anyhow::bail!("Not a directory: {path:?}");
    }

    let expected_kernel_json = path.join("kernel.json");
    let spec = fs.load(expected_kernel_json.as_path()).await?;
    let spec = serde_json::from_str::<JupyterKernelspec>(&spec)?;

    Ok(LocalKernelSpecification {
        name: kernel_name,
        path,
        kernelspec: spec,
    })
}

/// Read a directory of kernelspec directories
async fn read_kernels_dir(path: PathBuf, fs: &dyn Fs) -> Result<Vec<LocalKernelSpecification>> {
    let mut kernelspec_dirs = fs.read_dir(&path).await?;

    let mut valid_kernelspecs = Vec::new();
    while let Some(path) = kernelspec_dirs.next().await {
        match path {
            Ok(path) => {
                if fs.is_dir(path.as_path()).await {
                    if let Ok(kernelspec) = read_kernelspec_at(path, fs).await {
                        valid_kernelspecs.push(kernelspec);
                    }
                }
            }
            Err(err) => log::warn!("Error reading kernelspec directory: {err:?}"),
        }
    }

    Ok(valid_kernelspecs)
}

pub async fn local_kernel_specifications(fs: Arc<dyn Fs>) -> Result<Vec<LocalKernelSpecification>> {
    let mut data_dirs = dirs::data_dirs();

    // Pick up any kernels from conda or conda environment
    if let Ok(conda_prefix) = env::var("CONDA_PREFIX") {
        let conda_prefix = PathBuf::from(conda_prefix);
        let conda_data_dir = conda_prefix.join("share").join("jupyter");
        data_dirs.push(conda_data_dir);
    }

    // Search for kernels inside the base python environment
    let mut command = Command::new("python");
    command.arg("-c");
    command.arg("import sys; print(sys.prefix)");

    #[cfg(windows)]
    {
        use smol::process::windows::CommandExt;
        command.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);
    }

    let command = command.output().await;

    if let Ok(command) = command {
        if command.status.success() {
            let python_prefix = String::from_utf8(command.stdout);
            if let Ok(python_prefix) = python_prefix {
                let python_prefix = PathBuf::from(python_prefix.trim());
                let python_data_dir = python_prefix.join("share").join("jupyter");
                data_dirs.push(python_data_dir);
            }
        }
    }

    let kernel_dirs = data_dirs
        .iter()
        .map(|dir| dir.join("kernels"))
        .map(|path| read_kernels_dir(path, fs.as_ref()))
        .collect::<Vec<_>>();

    let kernel_dirs = futures::future::join_all(kernel_dirs).await;
    let kernel_dirs = kernel_dirs
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect::<Vec<_>>();

    Ok(kernel_dirs)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;

    #[gpui::test]
    async fn test_get_kernelspecs(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/jupyter",
            json!({
                ".zed": {
                    "settings.json": r#"{ "tab_size": 8 }"#,
                    "tasks.json": r#"[{
                        "label": "cargo check",
                        "command": "cargo",
                        "args": ["check", "--all"]
                    },]"#,
                },
                "kernels": {
                    "python": {
                        "kernel.json": r#"{
                            "display_name": "Python 3",
                            "language": "python",
                            "argv": ["python3", "-m", "ipykernel_launcher", "-f", "{connection_file}"],
                            "env": {}
                        }"#
                    },
                    "deno": {
                        "kernel.json": r#"{
                            "display_name": "Deno",
                            "language": "typescript",
                            "argv": ["deno", "run", "--unstable", "--allow-net", "--allow-read", "https://deno.land/std/http/file_server.ts", "{connection_file}"],
                            "env": {}
                        }"#
                    }
                },
            }),
        )
        .await;

        let mut kernels = read_kernels_dir(PathBuf::from("/jupyter/kernels"), fs.as_ref())
            .await
            .unwrap();

        kernels.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(
            kernels.iter().map(|c| c.name.clone()).collect::<Vec<_>>(),
            vec!["deno", "python"]
        );
    }
}
