use anyhow::{Context as _, Result};
use futures::{
    AsyncBufReadExt as _, FutureExt as _, StreamExt as _,
    channel::mpsc::{self},
    io::BufReader,
};
use gpui::{App, Entity, EntityId, Task, Window};
use jupyter_protocol::{
    ExecutionState, JupyterKernelspec, JupyterMessage, KernelInfoReply,
    connection_info::{ConnectionInfo, Transport},
};
use jupyter_zmq_client::dirs;
use project::Fs;
use smol::net::TcpListener;
use std::{
    env,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use util::ResultExt as _;
use uuid::Uuid;

use super::{KernelSession, RunningKernel, start_kernel_tasks};

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
    /// Specs read from a kernelspec directory have a directory path, so their
    /// own argv and env must be launched unchanged.
    pub(crate) fn is_notebook_python(&self) -> bool {
        self.kernelspec
            .argv
            .first()
            .is_some_and(|program| std::path::Path::new(program) == self.path)
    }

    pub(crate) fn for_notebook_python(path: PathBuf, name: String) -> Result<Self> {
        anyhow::ensure!(
            path.is_absolute(),
            "Expected an absolute Python interpreter path"
        );
        let directory = path.parent().context("Missing interpreter directory")?;
        let inherited = env::var_os("PATH").unwrap_or_default();
        let path_variable = env::join_paths(
            std::iter::once(directory.to_owned()).chain(env::split_paths(&inherited)),
        )?;
        Ok(Self {
            kernelspec: JupyterKernelspec {
                argv: vec![
                    path.to_str()
                        .context("Invalid Python interpreter path")?
                        .to_owned(),
                    "-I".into(),
                    "-c".into(),
                    include_str!("notebook_launcher.py").into(),
                    "--InteractiveShellApp.ignore_cwd=True".into(),
                    "-f".into(),
                    "{connection_file}".into(),
                ],
                display_name: name.clone(),
                language: "python".into(),
                interrupt_mode: None,
                metadata: None,
                env: Some(
                    [(
                        "PATH".into(),
                        path_variable
                            .into_string()
                            .map_err(|_| anyhow::anyhow!("Invalid interpreter PATH"))?,
                    )]
                    .into_iter()
                    .collect(),
                ),
            },
            name,
            path,
        })
    }

    #[must_use]
    fn command(&self, connection_path: &PathBuf) -> Result<std::process::Command> {
        let argv = &self.kernelspec.argv;

        anyhow::ensure!(!argv.is_empty(), "Empty argv in kernelspec {}", self.name);
        anyhow::ensure!(argv.len() >= 2, "Invalid argv in kernelspec {}", self.name);
        anyhow::ensure!(
            argv.iter().any(|arg| arg == "{connection_file}"),
            "Missing 'connection_file' in argv in kernelspec {}",
            self.name
        );

        let mut cmd = util::command::new_std_command(&argv[0]);

        for arg in &argv[1..] {
            if arg == "{connection_file}" {
                cmd.arg(connection_path);
            } else {
                cmd.arg(arg);
            }
        }

        if let Some(env) = &self.kernelspec.env {
            log::info!(
                "LocalKernelSpecification: applying env to command: {:?}",
                env.keys()
            );
            cmd.envs(env);
        } else {
            log::info!("LocalKernelSpecification: no env in kernelspec");
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
    pub process: util::process::Child,
    connection_path: PathBuf,
    _process_status_task: Option<Task<()>>,
    pub working_directory: PathBuf,
    pub request_tx: mpsc::Sender<JupyterMessage>,
    pub stdin_tx: mpsc::Sender<JupyterMessage>,
    pub execution_state: ExecutionState,
    pub kernel_info: Option<KernelInfoReply>,
}

impl Debug for NativeRunningKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningKernel")
            .field("process", &*self.process)
            .finish()
    }
}

impl NativeRunningKernel {
    pub fn new<S: KernelSession + 'static>(
        kernel_specification: LocalKernelSpecification,
        entity_id: EntityId,
        working_directory: PathBuf,
        fs: Arc<dyn Fs>,
        // todo: convert to weak view
        session: Entity<S>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Box<dyn RunningKernel>>> {
        window.spawn(cx, async move |cx| {
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            let ports = peek_ports(ip).await?;

            let connection_info = ConnectionInfo {
                transport: Transport::TCP,
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
            let connection_path = runtime_dir.join(format!("kernel-zed-{entity_id}-{}.json", Uuid::new_v4()));
            let content = serde_json::to_string(&connection_info)?;
            fs.atomic_write(connection_path.clone(), content).await?;
            let remove_connection = util::defer(|| {
                if let Err(error) = std::fs::remove_file(&connection_path)
                    && error.kind() != std::io::ErrorKind::NotFound
                {
                    log::warn!("Failed to remove kernel connection file: {error}");
                }
            });

            let mut cmd = kernel_specification.command(&connection_path)?;
            cmd.current_dir(&working_directory);

            session.update(cx, |session, cx| session.check_launch(cx))?;
            let mut process = util::process::Child::spawn(
                cmd,
                std::process::Stdio::piped(),
                std::process::Stdio::piped(),
                std::process::Stdio::piped(),
            ).map_err(|error| anyhow::anyhow!("Failed to start kernel {}: {}", kernel_specification.name, error.root_cause()))?;
            let process_status = process.status();
            let kill_on_cancel = util::defer(|| {
                process.kill().log_err();
            });

            let session_id = Uuid::new_v4().to_string();

            let executor = cx.background_executor().clone();
            let connect = async {
            let mut iopub_socket = jupyter_zmq_client::create_client_iopub_connection(
                &connection_info,
                "",
                &session_id,
            )
            .await?;
            let control_socket =
                jupyter_zmq_client::create_client_control_connection(&connection_info, &session_id)
                    .await?;

            let peer_identity = jupyter_zmq_client::peer_identity_for_session(&session_id)?;
            let mut shell_socket = jupyter_zmq_client::create_client_shell_connection_with_identity(
                &connection_info,
                &session_id,
                peer_identity.clone(),
            )
            .await?;
            let stdin_socket = jupyter_zmq_client::create_client_stdin_connection_with_identity(
                &connection_info,
                &session_id,
                peer_identity,
            )
            .await?;

            if S::IS_NOTEBOOK {
                wait_for_notebook_iopub(&mut iopub_socket, &mut shell_socket, &executor).await?;
            }
            anyhow::Ok((iopub_socket, shell_socket, control_socket, stdin_socket))
            };
            let readiness_timeout = if S::IS_NOTEBOOK {
                executor.timer(std::time::Duration::from_secs(30)).left_future()
            } else {
                futures::future::pending().right_future()
            };
            let (iopub_socket, shell_socket, control_socket, stdin_socket) = futures::select! {
                sockets = connect.fuse() => sockets?,
                status = process_status.fuse() => anyhow::bail!("Kernel exited during startup: {}", status?),
                _ = readiness_timeout.fuse() => anyhow::bail!("The kernel did not become ready within 30 seconds"),
            };
            kill_on_cancel.abort();
            remove_connection.abort();
            let (request_tx, stdin_tx) = start_kernel_tasks(
                session.clone(),
                iopub_socket,
                shell_socket,
                control_socket,
                stdin_socket,
                cx,
            );

            let stderr = process.stderr.take();
            let stdout = process.stdout.take();

            cx.spawn(async move |_cx| {
                use futures::future::Either;

                let stderr_lines = match stderr {
                    Some(s) => Either::Left(
                        BufReader::new(s)
                            .lines()
                            .map(|line| (log::Level::Error, line)),
                    ),
                    None => Either::Right(futures::stream::empty()),
                };
                let stdout_lines = match stdout {
                    Some(s) => Either::Left(
                        BufReader::new(s)
                            .lines()
                            .map(|line| (log::Level::Info, line)),
                    ),
                    None => Either::Right(futures::stream::empty()),
                };
                let mut lines = futures::stream::select(stderr_lines, stdout_lines);
                while let Some((level, Ok(line))) = lines.next().await {
                    log::log!(level, "kernel: {}", line);
                }
            })
            .detach();

            let status = process.status();
            let session = session.downgrade();

            let process_status_task = cx.spawn(async move |cx| {
                let Some(error_message) = kernel_exit_error::<S>(status.await) else {
                    log::info!("kernel process exited successfully");
                    return;
                };

                log::error!("{}", error_message);

                session.update(cx, |session, cx| {
                    session.kernel_errored(error_message, cx);

                    cx.notify();
                }).ok();
            });

            anyhow::Ok(Box::new(Self {
                process,
                request_tx,
                stdin_tx,
                working_directory,
                _process_status_task: Some(process_status_task),
                connection_path,
                execution_state: ExecutionState::Idle,
                kernel_info: None,
            }) as Box<dyn RunningKernel>)
        })
    }
}

async fn wait_for_notebook_iopub(
    iopub: &mut jupyter_zmq_client::ClientIoPubConnection,
    shell: &mut jupyter_zmq_client::ClientShellConnection,
    executor: &gpui::BackgroundExecutor,
) -> Result<()> {
    // A connected SUB socket may still miss the first cell's output until
    // its subscription reaches the publisher.
    loop {
        shell
            .send(jupyter_protocol::KernelInfoRequest {}.into())
            .await?;
        let retry = executor.timer(std::time::Duration::from_millis(500)).fuse();
        futures::pin_mut!(retry);
        loop {
            futures::select! {
                message = iopub.read().fuse() => {
                    if message?.parent_header.is_some_and(|header| header.msg_type == "kernel_info_request") {
                        return Ok(());
                    }
                }
                _ = retry => break,
            }
        }
    }
}

fn kernel_exit_error<S: KernelSession>(
    status: std::io::Result<std::process::ExitStatus>,
) -> Option<String> {
    match status {
        Ok(status) if status.success() && !S::IS_NOTEBOOK => None,
        Ok(status) => Some(format!("kernel process exited with status: {status:?}")),
        Err(error) => Some(format!("kernel process exited with error: {error:?}")),
    }
}

impl RunningKernel for NativeRunningKernel {
    fn request_tx(&self) -> mpsc::Sender<JupyterMessage> {
        self.request_tx.clone()
    }

    fn stdin_tx(&self) -> mpsc::Sender<JupyterMessage> {
        self.stdin_tx.clone()
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
        self.kill();
        Task::ready(Ok(()))
    }

    fn kill(&mut self) {
        self._process_status_task.take();
        self.request_tx.close_channel();
        self.stdin_tx.close_channel();
        self.process.kill().ok();
    }
}

impl Drop for NativeRunningKernel {
    fn drop(&mut self) {
        std::fs::remove_file(&self.connection_path).ok();
        self.kill();
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
        kernel_name.to_string_lossy().into_owned()
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
                if fs.is_dir(path.as_path()).await
                    && let Ok(kernelspec) = read_kernelspec_at(path, fs).await
                {
                    valid_kernelspecs.push(kernelspec);
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
    let command = util::command::new_command("python")
        .arg("-c")
        .arg("import sys; print(sys.prefix)")
        .output()
        .await;

    if let Ok(command) = command
        && command.status.success()
    {
        let python_prefix = String::from_utf8(command.stdout);
        if let Ok(python_prefix) = python_prefix {
            let python_prefix = PathBuf::from(python_prefix.trim());
            let python_data_dir = python_prefix.join("share").join("jupyter");
            data_dirs.push(python_data_dir);
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

pub(crate) async fn notebook_kernel_choices(
    fs: Arc<dyn Fs>,
    project_python: Option<PathBuf>,
    include_kernelspecs: bool,
) -> Result<Vec<LocalKernelSpecification>> {
    let mut candidates = Vec::new();
    if let Some(path) = project_python.filter(|path| path.is_absolute()) {
        candidates.push((path, "Project Python".to_string()));
    }
    if let Some(path) = crate::notebook::kernel_setup::installed_python() {
        candidates.push((path, "Zed Python".to_string()));
    }
    let names: &[&str] = if cfg!(windows) {
        &["python.exe"]
    } else {
        &["python3", "python"]
    };
    let mut path_interpreters = Vec::new();
    if let Some(path) = env::var_os("PATH") {
        for name in names {
            if let Some(path) = env::split_paths(&path)
                .filter(|dir| dir.is_absolute())
                .map(|dir| dir.join(name))
                .find(|python| python.is_file())
            {
                path_interpreters.push((path, "Python".to_string()));
            }
        }
    }
    let mut data_dirs = dirs::data_dirs();
    if include_kernelspecs {
        let prefixes = ["CONDA_PREFIX", "VIRTUAL_ENV"]
            .into_iter()
            .filter_map(env::var_os)
            .map(PathBuf::from);
        let interpreter_prefixes =
            candidates
                .iter()
                .chain(&path_interpreters)
                .filter_map(|(python, _)| {
                    let directory = python.parent()?;
                    if directory.ends_with("bin") || directory.ends_with("Scripts") {
                        directory.parent().map(std::path::Path::to_owned)
                    } else if cfg!(windows) {
                        Some(directory.to_owned())
                    } else {
                        None
                    }
                });
        for prefix in prefixes
            .chain(interpreter_prefixes)
            .filter(|prefix| prefix.is_absolute())
        {
            let dir = prefix.join("share/jupyter");
            if !data_dirs.contains(&dir) {
                data_dirs.push(dir);
            }
        }
    }
    let mut installed = Vec::new();
    for dir in data_dirs.into_iter().filter(|dir| dir.is_absolute()) {
        let kernels = dir.join("kernels");
        if fs.metadata(&kernels).await?.is_some() {
            installed.extend(read_kernels_dir(kernels, fs.as_ref()).await?);
        }
    }
    for spec in &installed {
        if !include_kernelspecs
            && spec.kernelspec.language.eq_ignore_ascii_case("python")
            && let Some(path) = spec.kernelspec.argv.first().map(PathBuf::from)
            && path.is_absolute()
        {
            candidates.push((path, spec.kernelspec.display_name.clone()));
        }
    }
    candidates.extend(path_interpreters);
    let mut seen = collections::HashSet::default();
    let mut specs = Vec::new();
    for (path, name) in candidates {
        if !seen.insert(path.clone()) || !path.is_file() {
            continue;
        }
        specs.push(LocalKernelSpecification::for_notebook_python(path, name)?);
    }
    if include_kernelspecs {
        specs.extend(installed);
    }
    Ok(specs)
}

pub(crate) async fn suitable_notebook_python_kernels(
    candidates: Vec<LocalKernelSpecification>,
    preferred: Option<&std::path::Path>,
) -> (Vec<LocalKernelSpecification>, Option<String>) {
    let mut specs = Vec::new();
    let mut failures = Vec::new();
    for candidate in candidates {
        match has_ipykernel(&candidate.path).await {
            Ok(true) if preferred == Some(candidate.path.as_path()) => {
                return (vec![candidate], None);
            }
            Ok(true) => specs.push(candidate),
            Ok(false) => {}
            Err(error) => {
                log::warn!("{error:#}");
                failures.push(format!("{error:#}"));
            }
        }
    }
    (specs, (!failures.is_empty()).then(|| failures.join("\n")))
}

async fn has_ipykernel(path: &std::path::Path) -> Result<bool> {
    let output = util::command::new_command(path)
        .current_dir(env::temp_dir())
        .args(["-I", "-c", "import importlib.util, sys; sys.exit(0 if importlib.util.find_spec('ipykernel') else 77)"])
        .kill_on_drop(true)
        .output().await.with_context(|| format!("Could not check Python at {}", path.display()))?;
    if output.status.code() == Some(77) {
        return Ok(false);
    }
    anyhow::ensure!(
        output.status.success(),
        "Could not check Python at {}: {} ({})",
        path.display(),
        String::from_utf8_lossy(&output.stderr).trim(),
        output.status
    );
    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;

    #[test]
    fn a_clean_native_exit_is_not_an_ordinary_repl_error() {
        assert_eq!(
            kernel_exit_error::<crate::Session>(Ok(std::process::ExitStatus::default())),
            None
        );
    }

    #[test]
    #[ignore = "requires ZED_NOTEBOOK_TEST_PYTHON pointing to a local interpreter with ipykernel"]
    fn notebook_kernel_imports_siblings_after_startup() -> Result<()> {
        let python =
            env::var_os("ZED_NOTEBOOK_TEST_PYTHON").context("Set ZED_NOTEBOOK_TEST_PYTHON")?;
        let spec =
            LocalKernelSpecification::for_notebook_python(python.into(), "Test Python".into())?;
        let root = tempfile::tempdir()?;
        let kernel_dir = root.path().join("kernels/zed-test");
        std::fs::create_dir_all(&kernel_dir)?;
        std::fs::write(
            kernel_dir.join("kernel.json"),
            serde_json::to_vec(&serde_json::json!({
                "argv": spec.kernelspec.argv,
                "env": spec.kernelspec.env,
                "display_name": spec.kernelspec.display_name,
                "language": spec.kernelspec.language,
            }))?,
        )?;
        std::fs::write(root.path().join("sibling_module.py"), "RESULT = 42\n")?;
        let output = smol::block_on(util::command::new_command(&spec.path)
            .current_dir(root.path())
            .env("JUPYTER_PATH", root.path())
            .env("IPYTHONDIR", root.path().join("ipython"))
            .args(["-I", "-c", r#"
from jupyter_client.manager import run_kernel

with run_kernel(kernel_name="zed-test") as client:
    def execute(code, count, silent=False):
        reply = client.execute_interactive(code, silent=silent, store_history=not silent, timeout=15)
        assert reply["content"]["status"] == "ok", reply["content"]
        assert reply["content"]["execution_count"] == count, reply["content"]

    execute("import sys, importlib.util; assert importlib.util.find_spec('sibling_module') is None", 0, silent=True)
    execute("%pwd\nfrom sibling_module import RESULT\nassert RESULT == 42", 1)
    execute("sys.path.remove('')", 2)
    execute("assert '' not in sys.path and RESULT == 42", 3)
"#])
            .output())?;
        anyhow::ensure!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn notebook_prefers_the_project_interpreter_without_resolving_its_symlink(
        cx: &mut TestAppContext,
    ) {
        use std::os::unix::fs::{PermissionsExt as _, symlink};
        let root = tempfile::tempdir().unwrap();
        let bin = root.path().join(".venv/bin");
        std::fs::create_dir_all(&bin).unwrap();
        let program = root.path().join("interpreter");
        let marker = root.path().join("probed");
        std::fs::write(
            &program,
            format!(
                "#!/bin/sh\ntouch '{}'\n[ \"$1\" = '-I' ] && [ \"$2\" = '-c' ]\n",
                marker.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o700)).unwrap();
        let python = bin.join("python");
        symlink(&program, &python).unwrap();
        cx.executor().allow_parking();
        let mut candidates =
            notebook_kernel_choices(FakeFs::new(cx.executor()), Some(python.clone()), false)
                .await
                .unwrap();
        assert!(
            !marker.exists(),
            "opening the selector must not probe an interpreter"
        );
        candidates.insert(
            0,
            LocalKernelSpecification::for_notebook_python(
                root.path().join("missing"),
                "Missing".into(),
            )
            .unwrap(),
        );
        let (kernels, _) = suitable_notebook_python_kernels(candidates, Some(&python)).await;
        assert!(
            marker.is_file(),
            "Run checks whether the project interpreter has ipykernel"
        );
        let [kernel] = kernels.as_slice() else {
            panic!("expected the project kernel")
        };
        assert_eq!(kernel.path, python);
        assert_eq!(
            kernel.kernelspec.argv.first().unwrap(),
            python.to_str().unwrap()
        );
        let kernel_path = &kernel.kernelspec.env.as_ref().unwrap()["PATH"];
        assert_eq!(env::split_paths(kernel_path).next().unwrap(), bin);
    }

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
