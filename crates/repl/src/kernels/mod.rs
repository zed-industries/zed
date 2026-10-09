mod native_kernel;
use std::{
    fmt::Debug,
    future::Future,
    path::{Path, PathBuf},
};

use futures::{channel::mpsc, future::Shared};
use gpui::{App, Entity, Task, Window};
use language::LanguageName;
use log;
pub use native_kernel::*;

mod remote_kernels;
use project::{Project, ProjectPath, Toolchains, WorktreeId};
use remote::RemoteConnectionOptions;
pub use remote_kernels::*;

mod ssh_kernel;
pub use ssh_kernel::*;

mod wsl_kernel;
pub use wsl_kernel::*;

use std::collections::HashMap;

use anyhow::{Context as _, Result};
use futures::{FutureExt, StreamExt};
use gpui::{AppContext, AsyncWindowContext, Context};
use jupyter_protocol::{JupyterKernelspec, JupyterMessageContent};
use runtimelib::{
    ClientControlConnection, ClientIoPubConnection, ClientShellConnection, ClientStdinConnection,
    ExecutionState, JupyterMessage, KernelInfoReply,
};
use ui::{Icon, IconName, SharedString};
use util::rel_path::RelPath;

pub(crate) const VENV_DIR_NAMES: &[&str] = &[".venv", "venv", ".env", "env"];

// Build a POSIX shell script that attempts to find and exec the best Python binary to run with the given arguments.
pub(crate) fn build_python_exec_shell_script(
    python_args: &str,
    cd_command: &str,
    env_command: &str,
) -> String {
    let venv_dirs = VENV_DIR_NAMES.join(" ");
    format!(
        "set -e; \
         {cd_command}\
         {env_command}\
         for venv_dir in {venv_dirs}; do \
           if [ -f \"$venv_dir/pyvenv.cfg\" ] || [ -f \"$venv_dir/bin/activate\" ]; then \
             if [ -x \"$venv_dir/bin/python\" ]; then \
               exec \"$venv_dir/bin/python\" {python_args}; \
             elif [ -x \"$venv_dir/bin/python3\" ]; then \
               exec \"$venv_dir/bin/python3\" {python_args}; \
             fi; \
           fi; \
         done; \
         if command -v python3 >/dev/null 2>&1; then \
           exec python3 {python_args}; \
         elif command -v python >/dev/null 2>&1; then \
           exec python {python_args}; \
         else \
           echo 'Error: Python not found in virtual environment or PATH' >&2; \
           exit 127; \
         fi"
    )
}

/// Build a POSIX shell script that outputs the best Python binary.
#[cfg(target_os = "windows")]
pub(crate) fn build_python_discovery_shell_script() -> String {
    let venv_dirs = VENV_DIR_NAMES.join(" ");
    format!(
        "for venv_dir in {venv_dirs}; do \
           if [ -f \"$venv_dir/pyvenv.cfg\" ] || [ -f \"$venv_dir/bin/activate\" ]; then \
             if [ -x \"$venv_dir/bin/python\" ]; then \
               echo \"$venv_dir/bin/python\"; exit 0; \
             elif [ -x \"$venv_dir/bin/python3\" ]; then \
               echo \"$venv_dir/bin/python3\"; exit 0; \
             fi; \
           fi; \
         done; \
         if command -v python3 >/dev/null 2>&1; then \
           echo python3; exit 0; \
         elif command -v python >/dev/null 2>&1; then \
           echo python; exit 0; \
         fi; \
         exit 1"
    )
}

pub fn start_kernel_tasks<S: KernelSession + 'static>(
    session: Entity<S>,
    iopub_socket: ClientIoPubConnection,
    shell_socket: ClientShellConnection,
    control_socket: ClientControlConnection,
    stdin_socket: ClientStdinConnection,
    cx: &mut AsyncWindowContext,
) -> (
    futures::channel::mpsc::Sender<JupyterMessage>,
    futures::channel::mpsc::Sender<JupyterMessage>,
) {
    let (mut shell_send, shell_recv) = shell_socket.split();
    let (mut control_send, control_recv) = control_socket.split();
    let (mut stdin_send, stdin_recv) = stdin_socket.split();

    let (request_tx, mut request_rx) = futures::channel::mpsc::channel::<JupyterMessage>(100);
    let (stdin_tx, mut stdin_rx) = futures::channel::mpsc::channel::<JupyterMessage>(100);

    let recv_task = cx.spawn({
        let session = session.clone();
        let mut iopub = iopub_socket;
        let mut shell = shell_recv;
        let mut control = control_recv;
        let mut stdin = stdin_recv;

        async move |cx| -> anyhow::Result<()> {
            loop {
                let (channel, result) = futures::select! {
                    msg = iopub.read().fuse() => ("iopub", msg),
                    msg = shell.read().fuse() => ("shell", msg),
                    msg = control.read().fuse() => ("control", msg),
                    msg = stdin.read().fuse() => ("stdin", msg),
                };
                match result {
                    Ok(message) => {
                        session
                            .update_in(cx, |session, window, cx| {
                                session.route(&message, window, cx);
                            })
                            .ok();
                    }
                    Err(
                        ref err @ (runtimelib::RuntimeError::ParseError { .. }
                        | runtimelib::RuntimeError::SerdeError(_)),
                    ) => {
                        let error_detail = format!("Kernel issue on {channel} channel\n\n{err}");
                        log::warn!("kernel: {error_detail}");
                        session
                            .update_in(cx, |session, _window, cx| {
                                session.kernel_errored(error_detail, cx);
                                cx.notify();
                            })
                            .ok();
                    }
                    Err(err) => {
                        log::warn!("kernel: error reading from {channel}: {err:?}");
                        anyhow::bail!("{channel} recv: {err}");
                    }
                }
            }
        }
    });

    let routing_task = cx.background_spawn(async move {
        while let Some(message) = request_rx.next().await {
            match message.content {
                JupyterMessageContent::DebugRequest(_)
                | JupyterMessageContent::InterruptRequest(_)
                | JupyterMessageContent::ShutdownRequest(_) => {
                    control_send.send(message).await?;
                }
                _ => {
                    shell_send.send(message).await?;
                }
            }
        }
        anyhow::Ok(())
    });

    let stdin_routing_task = cx.background_spawn(async move {
        while let Some(message) = stdin_rx.next().await {
            stdin_send.send(message).await?;
        }
        anyhow::Ok(())
    });

    cx.spawn({
        async move |cx| {
            async fn with_name(
                name: &'static str,
                task: Task<Result<()>>,
            ) -> (&'static str, Result<()>) {
                (name, task.await)
            }

            let mut tasks = futures::stream::FuturesUnordered::new();
            tasks.push(with_name("recv task", recv_task));
            tasks.push(with_name("routing task", routing_task));
            tasks.push(with_name("stdin routing task", stdin_routing_task));

            while let Some((name, result)) = tasks.next().await {
                if let Err(err) = result {
                    session.update(cx, |session, cx| {
                        session.kernel_errored(format!("handling failed for {name}: {err}"), cx);
                        cx.notify();
                    });
                }
            }
        }
    })
    .detach();

    (request_tx, stdin_tx)
}

pub trait KernelSession: Sized {
    fn route(&mut self, message: &JupyterMessage, window: &mut Window, cx: &mut Context<Self>);
    fn kernel_errored(&mut self, error_message: String, cx: &mut Context<Self>);
}

#[derive(Debug, Clone)]
pub struct PythonEnvKernelSpecification {
    pub name: String,
    pub path: PathBuf,
    pub kernelspec: JupyterKernelspec,
    pub has_ipykernel: bool,
    /// Display label for the environment type: "venv", "Conda", "Pyenv", etc.
    pub environment_kind: Option<String>,
}

impl PartialEq for PythonEnvKernelSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl Eq for PythonEnvKernelSpecification {}

impl PythonEnvKernelSpecification {
    pub fn as_local_spec(&self) -> LocalKernelSpecification {
        LocalKernelSpecification {
            name: self.name.clone(),
            path: self.path.clone(),
            kernelspec: self.kernelspec.clone(),
        }
    }

    pub fn is_uv(&self) -> bool {
        matches!(
            self.environment_kind.as_deref(),
            Some("uv" | "uv (Workspace)")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelSpecification {
    JupyterServer(RemoteKernelSpecification),
    Jupyter(LocalKernelSpecification),
    PythonEnv(PythonEnvKernelSpecification),
    SshRemote(SshRemoteKernelSpecification),
    WslRemote(WslKernelSpecification),
}

#[derive(Debug, Clone)]
pub struct SshRemoteKernelSpecification {
    pub name: String,
    pub path: SharedString,
    pub kernelspec: JupyterKernelspec,
}

#[derive(Debug, Clone)]
pub struct WslKernelSpecification {
    pub name: String,
    pub kernelspec: JupyterKernelspec,
    pub distro: String,
}

impl PartialEq for SshRemoteKernelSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.kernelspec.argv == other.kernelspec.argv
            && self.path == other.path
            && self.kernelspec.display_name == other.kernelspec.display_name
            && self.kernelspec.language == other.kernelspec.language
            && self.kernelspec.interrupt_mode == other.kernelspec.interrupt_mode
            && self.kernelspec.env == other.kernelspec.env
            && self.kernelspec.metadata == other.kernelspec.metadata
    }
}

impl Eq for SshRemoteKernelSpecification {}

impl PartialEq for WslKernelSpecification {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.kernelspec.argv == other.kernelspec.argv
            && self.kernelspec.display_name == other.kernelspec.display_name
            && self.kernelspec.language == other.kernelspec.language
            && self.kernelspec.interrupt_mode == other.kernelspec.interrupt_mode
            && self.kernelspec.env == other.kernelspec.env
            && self.kernelspec.metadata == other.kernelspec.metadata
            && self.distro == other.distro
    }
}

impl Eq for WslKernelSpecification {}

impl KernelSpecification {
    pub fn name(&self) -> SharedString {
        match self {
            Self::Jupyter(spec) => spec.name.clone().into(),
            Self::PythonEnv(spec) => spec.name.clone().into(),
            Self::JupyterServer(spec) => spec.name.clone().into(),
            Self::SshRemote(spec) => spec.name.clone().into(),
            Self::WslRemote(spec) => spec.kernelspec.display_name.clone().into(),
        }
    }

    pub fn type_name(&self) -> SharedString {
        match self {
            Self::Jupyter(_) => "Jupyter".into(),
            Self::PythonEnv(spec) => SharedString::from(
                spec.environment_kind
                    .clone()
                    .unwrap_or_else(|| "Python Environment".to_string()),
            ),
            Self::JupyterServer(_) => "Jupyter Server".into(),
            Self::SshRemote(_) => "SSH Remote".into(),
            Self::WslRemote(_) => "WSL Remote".into(),
        }
    }

    pub fn path(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.path.to_string_lossy().into_owned(),
            Self::PythonEnv(spec) => spec.path.to_string_lossy().into_owned(),
            Self::JupyterServer(spec) => spec.url.to_string(),
            Self::SshRemote(spec) => spec.path.to_string(),
            Self::WslRemote(spec) => spec.distro.clone(),
        })
    }

    pub fn language(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.kernelspec.language.clone(),
            Self::PythonEnv(spec) => spec.kernelspec.language.clone(),
            Self::JupyterServer(spec) => spec.kernelspec.language.clone(),
            Self::SshRemote(spec) => spec.kernelspec.language.clone(),
            Self::WslRemote(spec) => spec.kernelspec.language.clone(),
        })
    }

    pub fn has_ipykernel(&self) -> bool {
        match self {
            Self::Jupyter(_) | Self::JupyterServer(_) | Self::SshRemote(_) | Self::WslRemote(_) => {
                true
            }
            Self::PythonEnv(spec) => spec.has_ipykernel,
        }
    }

    pub fn environment_kind_label(&self) -> Option<SharedString> {
        match self {
            Self::PythonEnv(spec) => spec
                .environment_kind
                .as_ref()
                .map(|kind| SharedString::from(kind.clone())),
            Self::Jupyter(_) => Some("Jupyter".into()),
            Self::JupyterServer(_) => Some("Jupyter Server".into()),
            Self::SshRemote(_) => Some("SSH Remote".into()),
            Self::WslRemote(_) => Some("WSL Remote".into()),
        }
    }

    pub fn icon(&self, cx: &App) -> Icon {
        let lang_name = match self {
            Self::Jupyter(spec) => spec.kernelspec.language.clone(),
            Self::PythonEnv(spec) => spec.kernelspec.language.clone(),
            Self::JupyterServer(spec) => spec.kernelspec.language.clone(),
            Self::SshRemote(spec) => spec.kernelspec.language.clone(),
            Self::WslRemote(spec) => spec.kernelspec.language.clone(),
        };

        file_icons::FileIcons::get(cx)
            .get_icon_for_type(&lang_name.to_lowercase(), cx)
            .map(Icon::from_path)
            .unwrap_or(Icon::new(IconName::ReplNeutral))
    }
}

fn extract_environment_kind(toolchain_json: &serde_json::Value) -> Option<String> {
    let kind_str = toolchain_json.get("kind")?.as_str()?;
    let label = match kind_str {
        "Conda" => "Conda",
        "Pixi" => "pixi",
        "Homebrew" => "Homebrew",
        "Pyenv" => "global (Pyenv)",
        "GlobalPaths" => "global",
        "PyenvVirtualEnv" => "Pyenv",
        "Pipenv" => "Pipenv",
        "Poetry" => "Poetry",
        "MacPythonOrg" => "global (Python.org)",
        "MacCommandLineTools" => "global (Command Line Tools for Xcode)",
        "LinuxGlobal" => "global",
        "MacXCode" => "global (Xcode)",
        "Venv" => "venv",
        "VirtualEnv" => "virtualenv",
        "VirtualEnvWrapper" => "virtualenvwrapper",
        "WindowsStore" => "global (Windows Store)",
        "WindowsRegistry" => "global (Windows Registry)",
        "Uv" => "uv",
        "UvWorkspace" => "uv (Workspace)",
        _ => kind_str,
    };
    Some(label.to_string())
}

/// Environment variables that make the environment owning `python_path` the active one,
/// the same way activating a virtual environment would.
pub fn python_environment_variables(python_path: &Path) -> HashMap<String, String> {
    let mut env = HashMap::new();
    if let Some(python_bin_dir) = python_path.parent() {
        if let Some(path_var) = std::env::var_os("PATH") {
            let mut paths = std::env::split_paths(&path_var).collect::<Vec<_>>();
            paths.insert(0, python_bin_dir.to_path_buf());
            if let Ok(new_path) = std::env::join_paths(paths) {
                env.insert("PATH".to_string(), new_path.to_string_lossy().to_string());
            }
        }

        if let Some(venv_root) = python_bin_dir.parent() {
            env.insert(
                "VIRTUAL_ENV".to_string(),
                venv_root.to_string_lossy().to_string(),
            );
        }
    }
    env
}

pub fn python_env_kernel_specification(
    name: String,
    python_path: PathBuf,
    has_ipykernel: bool,
    environment_kind: Option<String>,
) -> PythonEnvKernelSpecification {
    let kernelspec = JupyterKernelspec {
        argv: vec![
            python_path.to_string_lossy().to_string(),
            "-m".to_string(),
            "ipykernel_launcher".to_string(),
            "-f".to_string(),
            "{connection_file}".to_string(),
        ],
        display_name: name.clone(),
        language: "python".to_string(),
        interrupt_mode: None,
        metadata: None,
        env: Some(python_environment_variables(&python_path)),
    };
    PythonEnvKernelSpecification {
        name,
        path: python_path,
        kernelspec,
        has_ipykernel,
        environment_kind,
    }
}

pub fn venv_python_path(environment_root: &Path) -> PathBuf {
    if cfg!(target_os = "windows") {
        environment_root.join("Scripts").join("python.exe")
    } else {
        environment_root.join("bin").join("python")
    }
}

/// Looks for a virtual environment directly inside `directory` (e.g. `.venv`), for when
/// toolchain discovery doesn't cover it, such as a notebook opened on its own.
pub fn find_venv_python(directory: &Path) -> Option<PathBuf> {
    VENV_DIR_NAMES
        .iter()
        .map(|name| venv_python_path(&directory.join(name)))
        .find(|python_path| python_path.is_file())
}

pub async fn python_has_ipykernel(python_path: &Path) -> bool {
    util::command::new_command(python_path)
        .args(&["-c", "import ipykernel"])
        .output()
        .await
        .is_ok_and(|output| output.status.success())
}

async fn python_has_pip(python_path: &Path) -> bool {
    util::command::new_command(python_path)
        .args(&["-m", "pip", "--version"])
        .output()
        .await
        .is_ok_and(|output| output.status.success())
}

/// Installs `ipykernel` into the environment owning `python_path`. Environments created by
/// uv don't ship pip, so uv is used for those and whenever pip is unavailable.
pub async fn install_ipykernel(python_path: &Path, prefer_uv: bool) -> Result<()> {
    let output = if prefer_uv || !python_has_pip(python_path).await {
        util::command::new_command("uv")
            .args(&["pip", "install", "ipykernel", "--python"])
            .arg(python_path)
            .output()
            .await
            .context("failed to run uv pip install ipykernel")?
    } else {
        util::command::new_command(python_path)
            .args(&["-m", "pip", "install", "ipykernel"])
            .output()
            .await
            .context("failed to run pip install ipykernel")?
    };

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{}", stderr.lines().last().unwrap_or("unknown error"))
    }
}

pub const GENERIC_PYTHON_KERNEL_NAME: &str = "Python 3 (Zed)";

pub fn generic_python_environment_dir() -> PathBuf {
    paths::data_dir().join("repl").join("python")
}

/// Creates, if needed, the environment Zed falls back to when the project has no Python
/// environment of its own, and returns its interpreter. It is a regular virtual environment
/// so users can install packages into it from a terminal.
pub async fn ensure_generic_python_environment() -> Result<PathBuf> {
    let environment_root = generic_python_environment_dir();
    let python_path = venv_python_path(&environment_root);

    if !python_path.is_file() {
        if let Some(parent) = environment_root.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }

        let created_with_uv = util::command::new_command("uv")
            .arg("venv")
            .arg(&environment_root)
            .output()
            .await
            .is_ok_and(|output| output.status.success());

        if !created_with_uv {
            let system_python = if cfg!(target_os = "windows") {
                "python"
            } else {
                "python3"
            };
            let output = util::command::new_command(system_python)
                .args(&["-m", "venv"])
                .arg(&environment_root)
                .output()
                .await
                .with_context(|| {
                    format!("failed to run {system_python}; install Python or uv to run notebooks")
                })?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!(
                    "failed to create a Python environment in {}: {}",
                    environment_root.display(),
                    stderr.lines().last().unwrap_or("unknown error")
                );
            }
        }
    }

    if !python_has_ipykernel(&python_path).await {
        install_ipykernel(&python_path, false)
            .await
            .context("failed to install ipykernel in Zed's Python environment")?;
    }

    Ok(python_path)
}

pub fn python_env_kernel_specifications(
    project: &Entity<Project>,
    worktree_id: WorktreeId,
    cx: &mut App,
) -> impl Future<Output = Result<Vec<KernelSpecification>>> + use<> {
    let python_language = LanguageName::new_static("Python");
    let is_remote = project.read(cx).is_remote();
    let wsl_distro = project
        .read(cx)
        .remote_connection_options(cx)
        .and_then(|opts| {
            if let RemoteConnectionOptions::Wsl(wsl) = opts {
                Some(wsl.distro_name)
            } else {
                None
            }
        });

    let toolchains = project.read(cx).available_toolchains(
        ProjectPath {
            worktree_id,
            path: RelPath::empty_arc(),
        },
        python_language,
        cx,
    );
    #[allow(unused)]
    let worktree_root_path: Option<std::sync::Arc<std::path::Path>> = project
        .read(cx)
        .worktree_for_id(worktree_id, cx)
        .map(|w| w.read(cx).abs_path());

    let background_executor = cx.background_executor().clone();

    async move {
        let (toolchains, user_toolchains) = if let Some(Toolchains {
            toolchains,
            root_path: _,
            user_toolchains,
        }) = toolchains.await
        {
            (toolchains, user_toolchains)
        } else {
            return Ok(Vec::new());
        };

        let kernelspecs = user_toolchains
            .into_values()
            .flatten()
            .chain(toolchains.toolchains)
            .map(|toolchain| {
                let wsl_distro = wsl_distro.clone();
                background_executor.spawn(async move {
                    // For remote projects, we assume python is available assuming toolchain is reported.
                    // We can skip the `ipykernel` check or run it remotely.
                    // For MVP, lets trust the toolchain existence or do the check if it's cheap.
                    // `new_smol_command` runs locally. We need to run remotely if `is_remote`.

                    if is_remote {
                        let default_kernelspec = JupyterKernelspec {
                            argv: vec![
                                toolchain.path.to_string(),
                                "-m".to_string(),
                                "ipykernel_launcher".to_string(),
                                "-f".to_string(),
                                "{connection_file}".to_string(),
                            ],
                            display_name: toolchain.name.to_string(),
                            language: "python".to_string(),
                            interrupt_mode: None,
                            metadata: None,
                            env: None,
                        };

                        if let Some(distro) = wsl_distro {
                            log::debug!(
                                "python_env_kernel_specifications: returning WslRemote for toolchain {}",
                                toolchain.name
                            );
                            return Some(KernelSpecification::WslRemote(WslKernelSpecification {
                                name: toolchain.name.to_string(),
                                kernelspec: default_kernelspec,
                                distro,
                            }));
                        }

                        log::debug!(
                            "python_env_kernel_specifications: returning SshRemote for toolchain {}",
                            toolchain.name
                        );
                        return Some(KernelSpecification::SshRemote(
                            SshRemoteKernelSpecification {
                                name: format!("Remote {}", toolchain.name),
                                path: toolchain.path.clone(),
                                kernelspec: default_kernelspec,
                            },
                        ));
                    }

                    let python_path = toolchain.path.to_string();
                    let environment_kind = extract_environment_kind(&toolchain.as_json);

                    let has_ipykernel = util::command::new_command(&python_path)
                        .args(&["-c", "import ipykernel"])
                        .output()
                        .await
                        .map(|output| output.status.success())
                        .unwrap_or(false);

                    let env = python_environment_variables(Path::new(&python_path));

                    log::info!("Preparing Python kernel for toolchain: {}", toolchain.name);
                    log::info!("Python path: {}", python_path);
                    if let Some(path) = env.get("PATH") {
                         log::info!("Kernel PATH: {}", path);
                    } else {
                         log::info!("Kernel PATH not set in env");
                    }
                    if let Some(venv) = env.get("VIRTUAL_ENV") {
                         log::info!("Kernel VIRTUAL_ENV: {}", venv);
                    }

                    let kernelspec = JupyterKernelspec {
                        argv: vec![
                            python_path.clone(),
                            "-m".to_string(),
                            "ipykernel_launcher".to_string(),
                            "-f".to_string(),
                            "{connection_file}".to_string(),
                        ],
                        display_name: toolchain.name.to_string(),
                        language: "python".to_string(),
                        interrupt_mode: None,
                        metadata: None,
                        env: Some(env),
                    };

                    Some(KernelSpecification::PythonEnv(PythonEnvKernelSpecification {
                        name: toolchain.name.to_string(),
                        path: PathBuf::from(&python_path),
                        kernelspec,
                        has_ipykernel,
                        environment_kind,
                    }))
                })
            });

        #[allow(unused_mut)]
        let mut kernel_specs: Vec<KernelSpecification> = futures::stream::iter(kernelspecs)
            .buffer_unordered(4)
            .filter_map(|x| async move { x })
            .collect::<Vec<_>>()
            .await;

        #[cfg(target_os = "windows")]
        if kernel_specs.is_empty() && !is_remote {
            if let Some(root_path) = worktree_root_path {
                let root_path_str: std::borrow::Cow<str> = root_path.to_string_lossy();
                let (distro, internal_path) =
                    if let Some(path_without_prefix) = root_path_str.strip_prefix(r"\\wsl$\") {
                        if let Some((distro, path)) = path_without_prefix.split_once('\\') {
                            let replaced_path: String = path.replace('\\', "/");
                            (Some(distro), Some(format!("/{}", replaced_path)))
                        } else {
                            (Some(path_without_prefix), Some("/".to_string()))
                        }
                    } else if let Some(path_without_prefix) =
                        root_path_str.strip_prefix(r"\\wsl.localhost\")
                    {
                        if let Some((distro, path)) = path_without_prefix.split_once('\\') {
                            let replaced_path: String = path.replace('\\', "/");
                            (Some(distro), Some(format!("/{}", replaced_path)))
                        } else {
                            (Some(path_without_prefix), Some("/".to_string()))
                        }
                    } else {
                        (None, None)
                    };

                if let (Some(distro), Some(internal_path)) = (distro, internal_path) {
                    let discovery_script = build_python_discovery_shell_script();
                    let script = format!(
                        "cd {} && {}",
                        shlex::try_quote(&internal_path)
                            .unwrap_or(std::borrow::Cow::Borrowed(&internal_path)),
                        discovery_script
                    );
                    let output = util::command::new_command("wsl")
                        .arg("-d")
                        .arg(distro)
                        .arg("bash")
                        .arg("-l")
                        .arg("-c")
                        .arg(&script)
                        .output()
                        .await;

                    if let Ok(output) = output {
                        if output.status.success() {
                            let python_cmd =
                                String::from_utf8_lossy(&output.stdout).trim().to_string();
                            let (python_path, display_suffix) = if python_cmd.contains('/') {
                                let venv_name = python_cmd.split('/').next().unwrap_or("venv");
                                (
                                    format!("{}/{}", internal_path, python_cmd),
                                    format!("({})", venv_name),
                                )
                            } else {
                                (python_cmd, "(System)".to_string())
                            };

                            let display_name = format!("WSL: {} {}", distro, display_suffix);
                            let default_kernelspec = JupyterKernelspec {
                                argv: vec![
                                    python_path,
                                    "-m".to_string(),
                                    "ipykernel_launcher".to_string(),
                                    "-f".to_string(),
                                    "{connection_file}".to_string(),
                                ],
                                display_name: display_name.clone(),
                                language: "python".to_string(),
                                interrupt_mode: None,
                                metadata: None,
                                env: None,
                            };

                            kernel_specs.push(KernelSpecification::WslRemote(
                                WslKernelSpecification {
                                    name: display_name,
                                    kernelspec: default_kernelspec,
                                    distro: distro.to_string(),
                                },
                            ));
                        }
                    }
                }
            }
        }

        anyhow::Ok(kernel_specs)
    }
}

pub trait RunningKernel: Send + Debug {
    fn request_tx(&self) -> mpsc::Sender<JupyterMessage>;
    fn stdin_tx(&self) -> mpsc::Sender<JupyterMessage>;
    fn working_directory(&self) -> &PathBuf;
    fn execution_state(&self) -> &ExecutionState;
    fn set_execution_state(&mut self, state: ExecutionState);
    fn kernel_info(&self) -> Option<&KernelInfoReply>;
    fn set_kernel_info(&mut self, info: KernelInfoReply);
    fn force_shutdown(&mut self, window: &mut Window, cx: &mut App) -> Task<anyhow::Result<()>>;
    fn kill(&mut self);
}

#[derive(Debug, Clone)]
pub enum KernelStatus {
    Idle,
    Busy,
    Starting,
    Error,
    ShuttingDown,
    Shutdown,
    Restarting,
}

impl KernelStatus {
    pub fn is_connected(&self) -> bool {
        matches!(self, KernelStatus::Idle | KernelStatus::Busy)
    }
}

impl ToString for KernelStatus {
    fn to_string(&self) -> String {
        match self {
            KernelStatus::Idle => "Idle".to_string(),
            KernelStatus::Busy => "Busy".to_string(),
            KernelStatus::Starting => "Starting".to_string(),
            KernelStatus::Error => "Error".to_string(),
            KernelStatus::ShuttingDown => "Shutting Down".to_string(),
            KernelStatus::Shutdown => "Shutdown".to_string(),
            KernelStatus::Restarting => "Restarting".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Kernel {
    RunningKernel(Box<dyn RunningKernel>),
    StartingKernel(Shared<Task<()>>),
    ErroredLaunch(String),
    ShuttingDown,
    Shutdown,
    Restarting,
}

impl From<&Kernel> for KernelStatus {
    fn from(kernel: &Kernel) -> Self {
        match kernel {
            Kernel::RunningKernel(kernel) => match kernel.execution_state() {
                ExecutionState::Idle => KernelStatus::Idle,
                ExecutionState::Busy => KernelStatus::Busy,
                ExecutionState::Unknown => KernelStatus::Error,
                ExecutionState::Starting => KernelStatus::Starting,
                ExecutionState::Restarting => KernelStatus::Restarting,
                ExecutionState::Terminating => KernelStatus::ShuttingDown,
                ExecutionState::AutoRestarting => KernelStatus::Restarting,
                ExecutionState::Dead => KernelStatus::Error,
                ExecutionState::Other(_) => KernelStatus::Error,
            },
            Kernel::StartingKernel(_) => KernelStatus::Starting,
            Kernel::ErroredLaunch(_) => KernelStatus::Error,
            Kernel::ShuttingDown => KernelStatus::ShuttingDown,
            Kernel::Shutdown => KernelStatus::Shutdown,
            Kernel::Restarting => KernelStatus::Restarting,
        }
    }
}

impl Kernel {
    pub fn status(&self) -> KernelStatus {
        self.into()
    }

    pub fn set_execution_state(&mut self, status: &ExecutionState) {
        if let Kernel::RunningKernel(running_kernel) = self {
            running_kernel.set_execution_state(status.clone());
        }
    }

    pub fn set_kernel_info(&mut self, kernel_info: &KernelInfoReply) {
        if let Kernel::RunningKernel(running_kernel) = self {
            running_kernel.set_kernel_info(kernel_info.clone());
        }
    }

    pub fn is_shutting_down(&self) -> bool {
        match self {
            Kernel::Restarting | Kernel::ShuttingDown => true,
            Kernel::RunningKernel(_)
            | Kernel::StartingKernel(_)
            | Kernel::ErroredLaunch(_)
            | Kernel::Shutdown => false,
        }
    }
}
