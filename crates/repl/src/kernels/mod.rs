mod native_kernel;
use std::{fmt::Debug, future::Future, path::PathBuf};

use futures::{channel::mpsc, future::Shared};
use gpui::{App, Entity, Task, Window};
use language::LanguageName;
pub use native_kernel::*;

mod remote_kernels;
use project::{Project, ProjectPath, Toolchains, WorktreeId};
pub use remote_kernels::*;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelSpecification {
    Remote(RemoteKernelSpecification),
    Jupyter(LocalKernelSpecification),
    PythonEnv(PythonEnvKernelSpecification),
}

impl KernelSpecification {
    pub fn name(&self) -> SharedString {
        match self {
            Self::Jupyter(spec) => spec.name.clone().into(),
            Self::PythonEnv(spec) => spec.name.clone().into(),
            Self::Remote(spec) => spec.name.clone().into(),
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
            Self::Remote(_) => "Remote".into(),
        }
    }

    pub fn path(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.path.to_string_lossy().into_owned(),
            Self::PythonEnv(spec) => spec.path.to_string_lossy().into_owned(),
            Self::Remote(spec) => spec.url.to_string(),
        })
    }

    pub fn language(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.kernelspec.language.clone(),
            Self::PythonEnv(spec) => spec.kernelspec.language.clone(),
            Self::Remote(spec) => spec.kernelspec.language.clone(),
        })
    }

    pub fn has_ipykernel(&self) -> bool {
        match self {
            Self::Jupyter(_) | Self::Remote(_) => true,
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
            Self::Remote(_) => Some("Remote".into()),
        }
    }

    pub fn icon(&self, cx: &App) -> Icon {
        let lang_name = match self {
            Self::Jupyter(spec) => spec.kernelspec.language.clone(),
            Self::PythonEnv(spec) => spec.kernelspec.language.clone(),
            Self::Remote(spec) => spec.kernelspec.language.clone(),
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

pub fn python_env_kernel_specifications(
    project: &Entity<Project>,
    worktree_id: WorktreeId,
    cx: &mut App,
) -> impl Future<Output = Result<Vec<KernelSpecification>>> + use<> {
    let python_language = LanguageName::new_static("Python");
    let toolchains = project.read(cx).available_toolchains(
        ProjectPath {
            worktree_id,
            path: RelPath::empty().into(),
        },
        python_language,
        cx,
    );
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
                    let python_path = toolchain.path.to_string();
                    let environment_kind = extract_environment_kind(&toolchain.as_json);

                    let has_ipykernel = util::command::new_command(&python_path)
                        .args(&["-c", "import ipykernel"])
                        .output()
                        .await
                        .map(|output| output.status.success())
                        .unwrap_or(false);

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
                        env: None,
                    };

                    KernelSpecification::PythonEnv(PythonEnvKernelSpecification {
                        name: toolchain.name.to_string(),
                        path: PathBuf::from(&python_path),
                        kernelspec,
                        has_ipykernel,
                        environment_kind,
                    })
                })
            });

        let kernel_specs = futures::future::join_all(kernelspecs).await;

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
