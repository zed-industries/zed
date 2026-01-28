mod native_kernel;
use std::{fmt::Debug, future::Future, path::PathBuf};

use futures::{
    channel::mpsc::{self, Receiver},
    future::Shared,
    stream,
};
use gpui::{App, Entity, Task, Window};
use language::LanguageName;
use log;
pub use native_kernel::*;

mod remote_kernels;
use project::{Project, ProjectPath, Toolchains, WorktreeId};
pub use remote_kernels::*;

mod ssh_kernel;
pub use ssh_kernel::*;

mod wsl_kernel;
pub use wsl_kernel::*;

use anyhow::Result;
use gpui::Context;
use jupyter_protocol::JupyterKernelspec;
use runtimelib::{ExecutionState, JupyterMessage, KernelInfoReply};
use ui::{Icon, IconName, SharedString};
use util::rel_path::RelPath;

pub trait KernelSession: Sized {
    fn route(&mut self, message: &JupyterMessage, window: &mut Window, cx: &mut Context<Self>);
    fn kernel_errored(&mut self, error_message: String, cx: &mut Context<Self>);
}

pub type JupyterMessageChannel = stream::SelectAll<Receiver<JupyterMessage>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelSpecification {
    Remote(RemoteKernelSpecification),
    Jupyter(LocalKernelSpecification),
    PythonEnv(LocalKernelSpecification),
    SshRemote(SshRemoteKernelSpecification),
    WslRemote(WslKernelSpecification),
}

#[derive(Debug, Clone)]
pub struct SshRemoteKernelSpecification {
    pub name: String,
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
            Self::Remote(spec) => spec.name.clone().into(),
            Self::SshRemote(spec) => spec.name.clone().into(),
            Self::WslRemote(spec) => spec.name.clone().into(),
        }
    }

    pub fn type_name(&self) -> SharedString {
        match self {
            Self::Jupyter(_) => "Jupyter".into(),
            Self::PythonEnv(_) => "Python Environment".into(),
            Self::Remote(_) => "Remote".into(),
            Self::SshRemote(_) => "SSH Remote".into(),
            Self::WslRemote(_) => "WSL Remote".into(),
        }
    }

    pub fn path(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.path.to_string_lossy().into_owned(),
            Self::PythonEnv(spec) => spec.path.to_string_lossy().into_owned(),
            Self::Remote(spec) => spec.url.to_string(),
            Self::SshRemote(_) => "Remote".to_string(),
            Self::WslRemote(_) => "WSL".to_string(),
        })
    }

    pub fn language(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.kernelspec.language.clone(),
            Self::PythonEnv(spec) => spec.kernelspec.language.clone(),
            Self::Remote(spec) => spec.kernelspec.language.clone(),
            Self::SshRemote(spec) => spec.kernelspec.language.clone(),
            Self::WslRemote(spec) => spec.kernelspec.language.clone(),
        })
    }

    pub fn icon(&self, cx: &App) -> Icon {
        let lang_name = match self {
            Self::Jupyter(spec) => spec.kernelspec.language.clone(),
            Self::PythonEnv(spec) => spec.kernelspec.language.clone(),
            Self::Remote(spec) => spec.kernelspec.language.clone(),
            Self::SshRemote(spec) => spec.kernelspec.language.clone(),
            Self::WslRemote(spec) => spec.kernelspec.language.clone(),
        };

        file_icons::FileIcons::get(cx)
            .get_icon_for_type(&lang_name.to_lowercase(), cx)
            .map(Icon::from_path)
            .unwrap_or(Icon::new(IconName::ReplNeutral))
    }
}

pub fn python_env_kernel_specifications(
    project: &Entity<Project>,
    worktree_id: WorktreeId,
    cx: &mut App,
) -> impl Future<Output = Result<Vec<KernelSpecification>>> + use<> {
    let python_language = LanguageName::new_static("Python");
    let is_remote = project.read(cx).is_remote();
    log::info!("python_env_kernel_specifications: is_remote: {}", is_remote);

    let toolchains = project.read(cx).available_toolchains(
        ProjectPath {
            worktree_id,
            path: RelPath::empty().into(),
        },
        python_language,
        cx,
    );
    #[allow(unused)]
    let worktree_root_path: Option<std::sync::Arc<std::path::Path>> = project
        .read(cx)
        .worktree_for_id(worktree_id, cx)
        .map(|w| w.read(cx).abs_path().clone());

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
                background_executor.spawn(async move {
                    // For remote projects, we assume python is available assuming toolchain is reported.
                    // We can skip the `ipykernel` check or run it remotely.
                    // For MVP, lets trust the toolchain existence or do the check if it's cheap.
                    // `new_smol_command` runs locally. We need to run remotely if `is_remote`.

                    if is_remote {
                        log::info!(
                            "python_env_kernel_specifications: returning SshRemote for toolchain {}",
                            toolchain.name
                        );
                        let default_kernelspec = JupyterKernelspec {
                            argv: vec![
                                "python3".to_string(), // using generic python3 for now on remote
                            ],
                            display_name: format!("Remote {}", toolchain.name),
                            language: "python".to_string(),
                            interrupt_mode: None,
                            metadata: None,
                            env: None,
                        };

                        return Some(KernelSpecification::SshRemote(
                            SshRemoteKernelSpecification {
                                name: format!("Remote {}", toolchain.name),
                                kernelspec: default_kernelspec,
                            },
                        ));
                    }

                    let python_path = toolchain.path.to_string();

                    // Check if ipykernel is installed
                    let ipykernel_check = util::command::new_smol_command(&python_path)
                        .args(&["-c", "import ipykernel"])
                        .output()
                        .await;

                    if ipykernel_check.is_ok() && ipykernel_check.unwrap().status.success() {
                        // Create a default kernelspec for this environment
                        let default_kernelspec = JupyterKernelspec {
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
                            env: None,
                        };

                        Some(KernelSpecification::PythonEnv(LocalKernelSpecification {
                            name: toolchain.name.to_string(),
                            path: PathBuf::from(&python_path),
                            kernelspec: default_kernelspec,
                        }))
                    } else {
                        log::info!(
                            "python_env_kernel_specifications: ipykernel check failed for toolchain {}",
                            toolchain.name
                        );
                        None
                    }
                })
            });

        #[allow(unused_mut)]
        let mut kernel_specs: Vec<KernelSpecification> = futures::future::join_all(kernelspecs)
            .await
            .into_iter()
            .flatten()
            .collect();

        #[cfg(target_os = "windows")]
        if kernel_specs.is_empty() && !is_remote {
            if let Some(root_path) = worktree_root_path {
                let root_path_str: std::borrow::Cow<str> = root_path.to_string_lossy();
                let (distro, internal_path) = if root_path_str.starts_with(r"\\wsl$\") {
                    let path_without_prefix = &root_path_str[r"\\wsl$\".len()..];
                    if let Some((distro, path)) = path_without_prefix.split_once('\\') {
                        let replaced_path: String = path.replace('\\', "/");
                        (Some(distro), Some(format!("/{}", replaced_path)))
                    } else {
                        (Some(path_without_prefix), Some("/".to_string()))
                    }
                } else if root_path_str.starts_with(r"\\wsl.localhost\") {
                    let path_without_prefix = &root_path_str[r"\\wsl.localhost\".len()..];
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
                    let python_path = format!("{}/.venv/bin/python", internal_path);
                    let check = util::command::new_smol_command("wsl")
                        .args(&["-d", distro, "test", "-f", &python_path])
                        .output()
                        .await;

                    if check.is_ok() && check.unwrap().status.success() {
                        let default_kernelspec = JupyterKernelspec {
                            argv: vec![
                                python_path.clone(),
                                "-m".to_string(),
                                "ipykernel_launcher".to_string(),
                                "-f".to_string(),
                                "{connection_file}".to_string(),
                            ],
                            display_name: format!("WSL: {} (.venv)", distro),
                            language: "python".to_string(),
                            interrupt_mode: None,
                            metadata: None,
                            env: None,
                        };

                        kernel_specs.push(KernelSpecification::WslRemote(WslKernelSpecification {
                            name: format!("WSL: {} (.venv)", distro),
                            kernelspec: default_kernelspec,
                            distro: distro.to_string(),
                        }));
                    } else {
                        let check_system = util::command::new_smol_command("wsl")
                            .args(&["-d", distro, "command", "-v", "python3"])
                            .output()
                            .await;

                        if check_system.is_ok() && check_system.unwrap().status.success() {
                            let default_kernelspec = JupyterKernelspec {
                                argv: vec![
                                    "python3".to_string(),
                                    "-m".to_string(),
                                    "ipykernel_launcher".to_string(),
                                    "-f".to_string(),
                                    "{connection_file}".to_string(),
                                ],
                                display_name: format!("WSL: {} (System)", distro),
                                language: "python".to_string(),
                                interrupt_mode: None,
                                metadata: None,
                                env: None,
                            };

                            kernel_specs.push(KernelSpecification::WslRemote(
                                WslKernelSpecification {
                                    name: format!("WSL: {} (System)", distro),
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
    fn working_directory(&self) -> &PathBuf;
    fn execution_state(&self) -> &ExecutionState;
    fn set_execution_state(&mut self, state: ExecutionState);
    fn kernel_info(&self) -> Option<&KernelInfoReply>;
    fn set_kernel_info(&mut self, info: KernelInfoReply);
    fn force_shutdown(&mut self, window: &mut Window, cx: &mut App) -> Task<anyhow::Result<()>>;
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
