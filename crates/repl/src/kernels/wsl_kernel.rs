use super::{KernelSession, KernelSpecification, RunningKernel, WslKernelSpecification};
use anyhow::{Context as _, Result};
use futures::{
    AsyncBufReadExt as _, SinkExt as _,
    channel::mpsc::{self},
    io::BufReader,
    stream::{FuturesUnordered, SelectAll, StreamExt},
};
use gpui::{App, AppContext as _, BackgroundExecutor, Entity, EntityId, Task, Window};
use jupyter_protocol::{
    ExecutionState, JupyterMessage, JupyterMessageContent, KernelInfoReply,
    connection_info::{ConnectionInfo, Transport},
};
use log;
use project::Fs;
use runtimelib::dirs;
use smol::net::TcpListener;
use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use uuid::Uuid;

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

pub struct WslRunningKernel {
    pub process: smol::process::Child,
    connection_path: PathBuf,
    _process_status_task: Option<Task<()>>,
    pub working_directory: PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
}

impl Debug for WslRunningKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WslRunningKernel")
            .field("process", &self.process)
            .finish()
    }
}

impl WslRunningKernel {
    pub fn new<S: KernelSession + 'static>(
        kernel_specification: WslKernelSpecification,
        entity_id: EntityId,
        working_directory: PathBuf,
        fs: Arc<dyn Fs>,
        session: Entity<S>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn RunningKernel>>> {
        window.spawn(cx, async move |cx| {
            // For WSL2, we need to get the WSL VM's IP address to connect to it
            // because WSL2 runs in a lightweight VM with its own network namespace.
            // The kernel will bind to 127.0.0.1 inside WSL, and we connect to localhost.
            // WSL2 localhost forwarding handles the rest.
            let bind_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

            // Use 127.0.0.1 and rely on WSL 2 localhost forwarding.
            // This avoids issues where the VM IP is unreachable or binding fails on Windows.
            let connect_ip = "127.0.0.1".to_string();

            let ports = peek_ports(bind_ip).await?;

            let connection_info = ConnectionInfo {
                transport: Transport::TCP,
                ip: bind_ip.to_string(),
                stdin_port: ports[0],
                control_port: ports[1],
                hb_port: ports[2],
                shell_port: ports[3],
                iopub_port: ports[4],
                signature_scheme: "hmac-sha256".to_string(),
                key: uuid::Uuid::new_v4().to_string(),
                kernel_name: Some(format!("zed-wsl-{}", kernel_specification.name)),
            };

            let runtime_dir = dirs::runtime_dir();
            fs.create_dir(&runtime_dir)
                .await
                .with_context(|| format!("Failed to create jupyter runtime dir {runtime_dir:?}"))?;
            let connection_path = runtime_dir.join(format!("kernel-zed-wsl-{entity_id}.json"));
            let content = serde_json::to_string(&connection_info)?;
            fs.atomic_write(connection_path.clone(), content).await?;

            // Convert connection_path to WSL path
            // yeah we can't assume this is available on WSL.
            // running `wsl -d <distro> wslpath -u <windows_path>`
            let mut wslpath_cmd = util::command::new_smol_command("wsl");

            // On Windows, passing paths with backslashes to wsl.exe can sometimes cause
            // escaping issues or be misinterpreted. Converting to forward slashes is safer
            // and often accepted by wslpath.
            let connection_path_str = connection_path.to_string_lossy().replace('\\', "/");

            wslpath_cmd
                .arg("-d")
                .arg(&kernel_specification.distro)
                .arg("wslpath")
                .arg("-u")
                .arg(&connection_path_str);

            let output = wslpath_cmd.output().await?;
            if !output.status.success() {
                anyhow::bail!("Failed to convert path to WSL path: {:?}", output);
            }
            let wsl_connection_path = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Construct the kernel command
            // The kernel spec argv might have absolute paths valid INSIDE WSL.
            // We need to run inside WSL.
            // `wsl -d <distro> --exec <argv0> <argv1> ...`
            // But we need to replace {connection_file} with wsl_connection_path.

            let argv = kernel_specification.kernelspec.argv;
            anyhow::ensure!(
                !argv.is_empty(),
                "Empty argv in kernelspec {}",
                kernel_specification.name
            );

            let working_directory_str = working_directory.to_string_lossy().replace('\\', "/");

            let wsl_working_directory = if working_directory_str.starts_with('/') {
                // If path starts with /, assume it is already a WSL path (e.g. /home/user)
                Some(working_directory_str)
            } else {
                let mut wslpath_wd_cmd = util::command::new_smol_command("wsl");
                wslpath_wd_cmd
                    .arg("-d")
                    .arg(&kernel_specification.distro)
                    .arg("wslpath")
                    .arg("-u")
                    .arg(&working_directory_str);

                let wd_output = wslpath_wd_cmd.output().await;
                if let Ok(output) = wd_output {
                    if output.status.success() {
                        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            // If we couldn't convert the working directory or it's a temp directory,
            // and the kernel spec uses a relative path (like .venv/bin/python),
            // we need to handle this better. For now, let's use the converted path
            // if available, otherwise we'll rely on WSL's default home directory.

            let mut cmd = util::command::new_smol_command("wsl");
            cmd.arg("-d").arg(&kernel_specification.distro);

            // Set CWD for the host process to a safe location to avoid "Directory name is invalid"
            // if the project root is a path not supported by Windows CWD (e.g. UNC path for some tools).
            cmd.current_dir(std::env::temp_dir());

            if let Some(wd) = wsl_working_directory.as_ref() {
                cmd.arg("--cd").arg(wd);
            }

            // Build the command to run inside WSL
            // We use bash -lc to run in a login shell for proper environment setup
            let mut kernel_args: Vec<String> = Vec::new();

            if let Some(env) = &kernel_specification.kernelspec.env {
                if !env.is_empty() {
                    kernel_args.push("env".to_string());
                    for (k, v) in env {
                        kernel_args.push(format!("{}={}", k, v));
                    }
                }
            }

            for arg in argv {
                if arg == "{connection_file}" {
                    kernel_args.push(wsl_connection_path.clone());
                } else {
                    kernel_args.push(arg.clone());
                }
            }

            // because first command is python/python3 we need make sure it's present in the env
            let first_cmd = kernel_args.first().map(|arg| {
                arg.split_whitespace().next().unwrap_or(arg)
            });

            let needs_python_resolution = first_cmd.map_or(false, |cmd| {
                cmd == "python" || cmd == "python3" || !cmd.starts_with('/')
            });

            let shell_command = if needs_python_resolution {
                // 1. Check for .venv/bin/python or .venv/bin/python3 in working directory
                // 2. Fall back to system python3 or python
                let rest_args: Vec<String> = kernel_args.iter().skip(1).cloned().collect();
                let rest_string = rest_args
                    .iter()
                    .map(|arg| {
                        if arg.contains(' ') || arg.contains('\'') || arg.contains('"') {
                            format!("'{}'", arg.replace('\'', "'\\''"))
                        } else {
                            arg.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                let cd_command = if let Some(wd) = wsl_working_directory.as_ref() {
                    format!("cd '{}' && ", wd.replace('\'', "'\\''"))
                } else {
                    String::new()
                };
                // TODO: find a better way to debug missing python issues in WSL

                format!(
                    "set -e; \
                     {} \
                     echo \"Working directory: $(pwd)\" >&2; \
                     if [ -x .venv/bin/python ]; then \
                       echo \"Found .venv/bin/python\" >&2; \
                       exec .venv/bin/python {}; \
                     elif [ -x .venv/bin/python3 ]; then \
                       echo \"Found .venv/bin/python3\" >&2; \
                       exec .venv/bin/python3 {}; \
                     elif command -v python3 >/dev/null 2>&1; then \
                       echo \"Found system python3\" >&2; \
                       exec python3 {}; \
                     elif command -v python >/dev/null 2>&1; then \
                       echo \"Found system python\" >&2; \
                       exec python {}; \
                     else \
                       echo 'Error: Python not found in .venv or PATH' >&2; \
                       echo 'Contents of current directory:' >&2; \
                       ls -la >&2; \
                       echo 'PATH:' \"$PATH\" >&2; \
                       exit 127; \
                     fi",
                    cd_command, rest_string, rest_string, rest_string, rest_string
                )
            } else {
                kernel_args
                    .iter()
                    .map(|arg| {
                        if arg.contains(' ') || arg.contains('\'') || arg.contains('"') {
                            format!("'{}'", arg.replace('\'', "'\\''"))
                        } else {
                            arg.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };

            cmd.arg("bash")
                .arg("-l")
                .arg("-c")
                .arg(&shell_command);

            let mut process = cmd
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .stdin(std::process::Stdio::piped())
                .kill_on_drop(true)
                .spawn()
                .context("failed to start the kernel process")?;

            let session_id = Uuid::new_v4().to_string();

            let mut client_connection_info = connection_info.clone();
            client_connection_info.ip = connect_ip.clone();

            // Give the kernel a moment to start and bind to ports.
            // WSL kernel startup can be slow, I am not sure if this is because of my testing environment
            // or inherent to WSL. We can improve this later with better readiness checks.
            cx.background_executor()
                .timer(std::time::Duration::from_secs(2))
                .await;

            match process.try_status() {
                Ok(Some(status)) => {
                    let mut stderr_content = String::new();
                    if let Some(mut stderr) = process.stderr.take() {
                        use futures::AsyncReadExt;
                        let mut buf = Vec::new();
                        if stderr.read_to_end(&mut buf).await.is_ok() {
                            stderr_content = String::from_utf8_lossy(&buf).to_string();
                        }
                    }

                    let mut stdout_content = String::new();
                    if let Some(mut stdout) = process.stdout.take() {
                        use futures::AsyncReadExt;
                        let mut buf = Vec::new();
                        if stdout.read_to_end(&mut buf).await.is_ok() {
                            stdout_content = String::from_utf8_lossy(&buf).to_string();
                        }
                    }

                    anyhow::bail!(
                        "WSL kernel process exited prematurely with status: {:?}\nstderr: {}\nstdout: {}",
                        status,
                        stderr_content,
                        stdout_content
                    );
                }
                Ok(None) => {}
                Err(_) => {}
            }

            let mut iopub_socket = runtimelib::create_client_iopub_connection(
                &client_connection_info,
                "",
                &session_id,
            )
            .await?;

            let mut shell_socket =
                runtimelib::create_client_shell_connection(&client_connection_info, &session_id)
                    .await?;

            let mut control_socket =
                runtimelib::create_client_control_connection(&client_connection_info, &session_id)
                    .await?;

            let (request_tx, mut request_rx) =
                futures::channel::mpsc::channel::<JupyterMessage>(100);

            let (mut control_reply_tx, control_reply_rx) = futures::channel::mpsc::channel(100);
            let (mut shell_reply_tx, shell_reply_rx) = futures::channel::mpsc::channel(100);

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
                            .ok();
                    }
                }
            })
            .detach();

            // iopub task
            let iopub_task = cx.spawn({
                let session = session.clone();

                async move |cx| -> anyhow::Result<()> {
                    loop {
                        let message = iopub_socket.read().await?;
                        session
                            .update_in(cx, |session, window, cx| {
                                session.route(&message, window, cx);
                            })
                            .ok();
                    }
                }
            });

            let (mut control_request_tx, mut control_request_rx) =
                futures::channel::mpsc::channel(100);
            let (mut shell_request_tx, mut shell_request_rx) = futures::channel::mpsc::channel(100);

            let routing_task = cx.background_spawn({
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

            let shell_task = cx.background_spawn({
                async move {
                    while let Some(message) = shell_request_rx.next().await {
                        shell_socket.send(message).await.ok();
                        let reply = shell_socket.read().await?;
                        shell_reply_tx.send(reply).await?;
                    }
                    anyhow::Ok(())
                }
            });

            let control_task = cx.background_spawn({
                async move {
                    while let Some(message) = control_request_rx.next().await {
                        control_socket.send(message).await.ok();
                        let reply = control_socket.read().await?;
                        control_reply_tx.send(reply).await?;
                    }
                    anyhow::Ok(())
                }
            });

            let stderr = process.stderr.take();

            cx.spawn(async move |_cx| {
                if stderr.is_none() {
                    return;
                }
                let reader = BufReader::new(stderr.unwrap());
                let mut lines = reader.lines();
                while let Some(Ok(line)) = lines.next().await {
                    log::error!("{}", line);
                }
            })
            .detach();

            let stdout = process.stdout.take();

            cx.spawn(async move |_cx| {
                if stdout.is_none() {
                    return;
                }
                let reader = BufReader::new(stdout.unwrap());
                let mut lines = reader.lines();
                while let Some(Ok(_line)) = lines.next().await {}
            })
            .detach();

            cx.spawn({
                let session = session.clone();
                async move |cx| {
                    async fn with_name(
                        name: &'static str,
                        task: Task<Result<()>>,
                    ) -> (&'static str, Result<()>) {
                        (name, task.await)
                    }

                    let mut tasks = FuturesUnordered::new();
                    tasks.push(with_name("iopub task", iopub_task));
                    tasks.push(with_name("shell task", shell_task));
                    tasks.push(with_name("control task", control_task));
                    tasks.push(with_name("routing task", routing_task));

                    while let Some((name, result)) = tasks.next().await {
                        if let Err(err) = result {
                            session.update(cx, |session, cx| {
                                session.kernel_errored(
                                    format!("handling failed for {name}: {err}"),
                                    cx,
                                );
                                cx.notify();
                            });
                        }
                    }
                }
            })
            .detach();

            let status = process.status();

            let process_status_task = cx.spawn(async move |cx| {
                let error_message = match status.await {
                    Ok(status) => {
                        if status.success() {
                            return;
                        }

                        format!("WSL kernel: kernel process exited with status: {:?}", status)
                    }
                    Err(err) => {
                        format!("WSL kernel: kernel process exited with error: {:?}", err)
                    }
                };

                session.update(cx, |session, cx| {
                    session.kernel_errored(error_message, cx);

                    cx.notify();
                });
            });

            anyhow::Ok(Box::new(Self {
                process,
                request_tx,
                working_directory,
                _process_status_task: Some(process_status_task),
                connection_path,
                execution_state: ExecutionState::Idle,
                kernel_info: None,
            }) as Box<dyn RunningKernel>)
        })
    }
}

impl RunningKernel for WslRunningKernel {
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

    fn force_shutdown(&mut self, _window: &mut Window, _cx: &mut App) -> Task<anyhow::Result<()>> {
        self._process_status_task.take();
        self.request_tx.close_channel();
        Task::ready(self.process.kill().context("killing the kernel process"))
    }
}

impl Drop for WslRunningKernel {
    fn drop(&mut self) {
        std::fs::remove_file(&self.connection_path).ok();
        self.request_tx.close_channel();
        self.process.kill().ok();
    }
}

#[derive(serde::Deserialize)]
struct LocalKernelSpecsResponse {
    kernelspecs: std::collections::HashMap<String, LocalKernelSpec>,
}

#[derive(serde::Deserialize)]
struct LocalKernelSpec {
    spec: LocalKernelSpecContent,
}

#[derive(serde::Deserialize)]
struct LocalKernelSpecContent {
    argv: Vec<String>,
    display_name: String,
    language: String,
    interrupt_mode: Option<String>,
    env: Option<std::collections::HashMap<String, String>>,
    metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

pub async fn wsl_kernel_specifications(
    background_executor: BackgroundExecutor,
) -> Result<Vec<KernelSpecification>> {
    let output = util::command::new_smol_command("wsl")
        .arg("-l")
        .arg("-q")
        .output()
        .await;

    if output.is_err() {
        return Ok(Vec::new());
    }

    let output = output.unwrap();
    if !output.status.success() {
        return Ok(Vec::new());
    }

    // wsl output is often UTF-16LE, but -l -q might be simpler or just ASCII compatible if not using weird charsets.
    // However, on Windows, wsl often outputs UTF-16LE.
    // We can try to detect or use from_utf16 if valid, or just use String::from_utf8_lossy and see.
    // Actually, `smol::process` on Windows might receive bytes that are UTF-16LE if wsl writes that.
    // But typically terminal output for wsl is UTF-16.
    // Let's try to parse as UTF-16LE if it looks like it (BOM or just 00 bytes).

    let stdout = output.stdout;
    let distros_str = if stdout.len() >= 2 && stdout[1] == 0 {
        // likely UTF-16LE
        let u16s: Vec<u16> = stdout
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16_lossy(&u16s)
    } else {
        String::from_utf8_lossy(&stdout).to_string()
    };

    let distros: Vec<String> = distros_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let tasks = distros.into_iter().map(|distro| {
        background_executor.spawn(async move {
            let output = util::command::new_smol_command("wsl")
                .arg("-d")
                .arg(&distro)
                .arg("bash")
                .arg("-l")
                .arg("-c")
                .arg("jupyter kernelspec list --json")
                .output()
                .await;

            if let Ok(output) = output {
                if output.status.success() {
                    let json_str = String::from_utf8_lossy(&output.stdout);
                    // Use local permissive struct instead of strict KernelSpecsResponse from jupyter-protocol
                    if let Ok(specs_response) =
                        serde_json::from_str::<LocalKernelSpecsResponse>(&json_str)
                    {
                        return specs_response
                            .kernelspecs
                            .into_iter()
                            .map(|(name, spec)| {
                                KernelSpecification::WslRemote(WslKernelSpecification {
                                    name,
                                    kernelspec: jupyter_protocol::JupyterKernelspec {
                                        argv: spec.spec.argv,
                                        display_name: spec.spec.display_name,
                                        language: spec.spec.language,
                                        interrupt_mode: spec.spec.interrupt_mode,
                                        env: spec.spec.env,
                                        metadata: spec.spec.metadata,
                                    },
                                    distro: distro.clone(),
                                })
                            })
                            .collect::<Vec<_>>();
                    }
                }
            }

            Vec::new()
        })
    });

    let specs: Vec<_> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .flatten()
        .collect();

    Ok(specs)
}
