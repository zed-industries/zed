use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use jupyter_protocol::JupyterKernelspec;

use crate::{Result, RuntimeError};

#[cfg(feature = "tokio-runtime")]
use tokio::{fs, io::AsyncReadExt, process::Command};

#[cfg(feature = "async-dispatcher-runtime")]
use smol::process::Command;

/// A pointer to a kernelspec directory, with name and specification
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KernelspecDir {
    pub kernel_name: String,
    pub path: PathBuf,
    pub kernelspec: JupyterKernelspec,
}

impl KernelspecDir {
    pub fn command(
        self,
        connection_path: &Path,
        stderr: Option<Stdio>,
        stdout: Option<Stdio>,
    ) -> Result<Command> {
        let kernel_name = &self.kernel_name;

        let argv = self.kernelspec.argv;
        if argv.is_empty() {
            return Err(RuntimeError::EmptyArgv {
                kernel_name: kernel_name.to_owned(),
            });
        }

        let mut cmd_builder = Command::new(&argv[0]);

        let stdout = stdout.unwrap_or(Stdio::null());
        let stderr = stderr.unwrap_or(Stdio::null());
        cmd_builder
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr);

        for arg in &argv[1..] {
            cmd_builder.arg(if arg == "{connection_file}" {
                connection_path.as_os_str()
            } else {
                OsStr::new(arg)
            });
        }
        if let Some(env) = self.kernelspec.env {
            cmd_builder.envs(env);
        }

        Ok(cmd_builder)
    }
}

// We look for files of the sort:
//    `<datadir>/kernels/<kernel_name>/kernel.json`
// But we must check through all the possible <datadir> to figure that out.
//
// For now, just use a combination of the standard system and user data directories.
#[cfg(feature = "tokio-runtime")]
pub async fn list_kernelspecs() -> Vec<KernelspecDir> {
    let mut kernelspecs = Vec::new();
    let data_dirs = crate::dirs::data_dirs();
    for data_dir in data_dirs {
        let mut specs = read_kernelspec_jsons(&data_dir).await;
        kernelspecs.append(&mut specs);
    }
    kernelspecs
}

/// Find a kernelspec by name.
///
/// This function first uses `list_kernelspec_names_at` to soft-check if the kernel
/// exists before doing an actual read of the kernelspec JSON file.
#[cfg(feature = "tokio-runtime")]
pub async fn find_kernelspec(name: &str) -> Result<KernelspecDir> {
    let dirs = crate::dirs::data_dirs();
    let mut available_kernels = Vec::new();

    for data_dir in &dirs {
        let kernel_names = list_kernelspec_names_at(data_dir).await;
        available_kernels.extend(kernel_names.clone());

        if kernel_names.contains(&name.to_string()) {
            let kernel_path = data_dir.join("kernels").join(name);
            let kernelspec = read_kernelspec_json(&kernel_path.join("kernel.json")).await?;
            return Ok(KernelspecDir {
                kernel_name: name.to_string(),
                path: kernel_path,
                kernelspec,
            });
        }
    }

    Err(crate::RuntimeError::KernelNotFound {
        name: name.to_string(),
        available: available_kernels,
    })
}

// Design choice here is to not report any errors, keep going if possible,
// and skip any paths that don't have a kernels subdirectory.
#[cfg(feature = "tokio-runtime")]
pub async fn list_kernelspec_names_at(data_dir: &Path) -> Vec<String> {
    let mut kernelspecs = Vec::new();
    let kernels_dir = data_dir.join("kernels");
    if let Ok(mut entries) = fs::read_dir(kernels_dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.path().is_dir() {
                if let Some(kernel_name) = entry.file_name().to_str() {
                    kernelspecs.push(kernel_name.to_string());
                }
            }
        }
    }
    kernelspecs
}

// For a given data directory, return all the parsed kernelspecs and corresponding directories
#[cfg(feature = "tokio-runtime")]
pub async fn read_kernelspec_jsons(data_dir: &Path) -> Vec<KernelspecDir> {
    let mut kernelspecs = Vec::new();
    let kernel_names = list_kernelspec_names_at(data_dir).await;
    for kernel_name in kernel_names {
        let kernel_path = data_dir.join("kernels").join(&kernel_name);
        if let Ok(jupyter_runtime) = read_kernelspec_json(&kernel_path.join("kernel.json")).await {
            kernelspecs.push(KernelspecDir {
                kernel_name,
                path: kernel_path,
                kernelspec: jupyter_runtime,
            });
        }
    }
    kernelspecs
}

#[cfg(feature = "tokio-runtime")]
async fn read_kernelspec_json(json_file_path: &Path) -> Result<JupyterKernelspec> {
    let mut file = fs::File::open(json_file_path).await?;
    let mut contents = vec![];

    file.read_to_end(&mut contents).await?;
    let jupyter_runtime: JupyterKernelspec = serde_json::from_slice(&contents)?;
    Ok(jupyter_runtime)
}

#[cfg(all(test, feature = "tokio-runtime"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_jupyter_runtime_config() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/kernels/ir/kernel.json");
        let jupyter_runtime = read_kernelspec_json(&d).await.unwrap();
        assert_eq!(jupyter_runtime.display_name, "R");
        assert_eq!(jupyter_runtime.language, "R");
        assert!(jupyter_runtime
            .env
            .as_ref()
            .unwrap()
            .contains_key("R_LIBS_USER"));
        assert_eq!(jupyter_runtime.env.as_ref().unwrap().len(), 1);
        assert!(jupyter_runtime.metadata.is_none());
        assert_eq!(jupyter_runtime.argv.len(), 6);
        assert_eq!(jupyter_runtime.interrupt_mode, Some("signal".to_string()));
    }

    #[tokio::test]
    async fn test_read_missing_config() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/kernels/NONEXISTENT/kernel.json");
        let jupyter_runtime = read_kernelspec_json(&d).await;
        assert!(jupyter_runtime.is_err());
    }

    #[tokio::test]
    async fn test_list_kernelspec_jsons() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        let kernelspecs = list_kernelspec_names_at(&d).await;
        assert_eq!(kernelspecs.len(), 3);
        assert!(kernelspecs.contains(&"ir".to_string()));
        assert!(kernelspecs.contains(&"python3".to_string()));
        assert!(kernelspecs.contains(&"rust".to_string()));
    }

    #[tokio::test]
    async fn test_read_kernelspec_jsons() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        let kernels = read_kernelspec_jsons(&d).await;
        assert_eq!(kernels.len(), 3);
        let mut r_count = 0;
        let mut python_count = 0;
        let mut rust_count = 0;
        for kerneldir in kernels {
            let kernelspec = &kerneldir.kernelspec;
            match kernelspec.display_name.as_str() {
                "R" => {
                    assert_eq!(kernelspec.language, "R");
                    assert_eq!(kernelspec.argv.len(), 6);
                    assert_eq!(kernelspec.interrupt_mode, Some("signal".to_string()));
                    r_count += 1;
                }
                "Python 3" => {
                    assert_eq!(kernelspec.language, "python");
                    assert_eq!(kernelspec.argv.len(), 5);
                    assert_eq!(kernelspec.interrupt_mode, None);
                    python_count += 1;
                }
                "Rust" => {
                    assert_eq!(kernelspec.language, "rust");
                    assert_eq!(kernelspec.argv.len(), 3);
                    assert_eq!(kernelspec.interrupt_mode, Some("message".to_string()));
                    rust_count += 1;
                }
                _ => panic!("Unexpected kernelspec found: {}", &kernelspec.display_name),
            }
        }
        assert_eq!(r_count, 1);
        assert_eq!(python_count, 1);
        assert_eq!(rust_count, 1);
    }

    #[tokio::test]
    async fn list_nonexistent_kernelspec_datadir() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/NOTHINGHERE");
        let kernels = list_kernelspec_names_at(&d).await;
        assert_eq!(kernels.len(), 0);
    }
}
