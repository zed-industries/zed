mod native_kernel;
use std::{fmt::Debug, future::Future, path::PathBuf, sync::Arc};

use futures::{
    channel::mpsc::{self, Receiver},
    future::Shared,
    stream,
};
use gpui::{App, Entity, Task, Window};
use language::LanguageName;
pub use native_kernel::*;

mod remote_kernels;
use project::{Project, ProjectPath, WorktreeId};
pub use remote_kernels::*;

use anyhow::Result;
use jupyter_protocol::JupyterKernelspec;
use runtimelib::{ExecutionState, JupyterMessage, KernelInfoReply};
use ui::{Icon, IconName, SharedString};

pub type JupyterMessageChannel = stream::SelectAll<Receiver<JupyterMessage>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelSpecification {
    Remote(RemoteKernelSpecification),
    Jupyter(LocalKernelSpecification),
    PythonEnv(LocalKernelSpecification),
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
            Self::PythonEnv(_) => "Python Environment".into(),
            Self::Remote(_) => "Remote".into(),
        }
    }

    pub fn path(&self) -> SharedString {
        SharedString::from(match self {
            Self::Jupyter(spec) => spec.path.to_string_lossy().to_string(),
            Self::PythonEnv(spec) => spec.path.to_string_lossy().to_string(),
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

pub fn python_env_kernel_specifications(
    project: &Entity<Project>,
    worktree_id: WorktreeId,
    cx: &mut App,
) -> impl Future<Output = Result<Vec<KernelSpecification>>> + use<> {
    let python_language = LanguageName::new("Python");
    let toolchains = project.read(cx).available_toolchains(
        ProjectPath {
            worktree_id,
            path: Arc::from("".as_ref()),
        },
        python_language,
        cx,
    );
    let background_executor = cx.background_executor().clone();

    async move {
        let toolchains = if let Some(toolchains) = toolchains.await {
            toolchains
        } else {
            return Ok(Vec::new());
        };

        let kernelspecs = toolchains.toolchains.into_iter().map(|toolchain| {
            background_executor.spawn(async move {
                let python_path = toolchain.path.to_string();
                let env = environment::in_home_dir().await;

                // Check if ipykernel is installed
                let ipykernel_check = util::command::new_smol_command(&python_path, &env)
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
                    None
                }
            })
        });

        let kernel_specs = futures::future::join_all(kernelspecs)
            .await
            .into_iter()
            .flatten()
            .collect();

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
        match self {
            KernelStatus::Idle | KernelStatus::Busy => true,
            _ => false,
        }
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
