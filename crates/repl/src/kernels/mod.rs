mod native_kernel;
use std::{future::Future, path::PathBuf};

use gpui::{AppContext, Model};
use language::LanguageName;
pub use native_kernel::*;

mod remote_kernels;
use project::{Project, WorktreeId};
pub use remote_kernels::*;

use anyhow::Result;
use runtimelib::JupyterKernelspec;
use smol::process::Command;
use ui::SharedString;

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
}

pub fn python_env_kernel_specifications(
    project: &Model<Project>,
    worktree_id: WorktreeId,
    cx: &mut AppContext,
) -> impl Future<Output = Result<Vec<KernelSpecification>>> {
    let python_language = LanguageName::new("Python");
    let toolchains = project
        .read(cx)
        .available_toolchains(worktree_id, python_language, cx);
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

                // Check if ipykernel is installed
                let ipykernel_check = Command::new(&python_path)
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
