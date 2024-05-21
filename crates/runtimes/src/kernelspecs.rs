// Detect all kernelspecs available on the system,
// watch for changes to the kernelspecs directory,

// Since runtimelib uses tokio, we'll only use `runtimelib::dirs` for paths and reimplement
// the rest using `project::Fs`.

use futures::StreamExt;
use project::Fs;
use std::{path::PathBuf, sync::Arc};

use runtimelib::dirs;
use runtimelib::JupyterKernelspec;

pub async fn read_kernelspec_at(
    // Path should be a directory to a jupyter kernelspec, as in
    // /usr/local/share/jupyter/kernels/python3
    path: PathBuf,
    fs: Arc<dyn Fs>,
) -> anyhow::Result<JupyterKernelspec> {
    let expected_kernel_json = path.join("kernel.json");
    let kernelspec = fs.load(expected_kernel_json.as_path()).await?;
    let kernelspec = serde_json::from_str::<JupyterKernelspec>(&kernelspec)?;

    Ok(kernelspec)
}

/// Read a directory of kernelspec directories
pub async fn read_kernels_dir(
    path: PathBuf,
    fs: Arc<dyn Fs>,
) -> anyhow::Result<Vec<JupyterKernelspec>> {
    let mut kernelspec_dirs = fs.read_dir(&path).await?;

    let mut valid_kernelspecs = Vec::new();
    while let Some(path) = kernelspec_dirs.next().await {
        match path {
            Ok(path) => {
                if fs.is_dir(path.as_path()).await {
                    let fs = fs.clone();
                    if let Ok(kernelspec) = read_kernelspec_at(path, fs).await {
                        valid_kernelspecs.push(kernelspec);
                    }
                }
            }
            Err(err) => {
                log::warn!("Error reading kernelspec directory: {:?}", err);
            }
        }
    }

    Ok(valid_kernelspecs)
}

pub async fn get_kernelspecs(fs: Arc<dyn Fs>) -> anyhow::Result<()> {
    let data_dirs = dirs::data_dirs();
    let kernel_dirs = data_dirs
        .iter()
        .map(|dir| dir.join("kernels"))
        .map(|path| read_kernels_dir(path, fs.clone()))
        .collect::<Vec<_>>();

    let kernel_dirs = futures::future::join_all(kernel_dirs).await;

    for kernel_dir in kernel_dirs {
        match kernel_dir {
            Ok(kernel_dir) => {}
            Err(err) => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    use gpui::prelude::*;
    use gpui::{TestAppContext};
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

        let mut kernels = read_kernels_dir(PathBuf::from("/jupyter/kernels"), fs)
            .await
            .unwrap();

        kernels.sort_by(|a, b| a.language.cmp(&b.language));

        assert_eq!(
            kernels
                .iter()
                .map(|c| c.language.clone())
                .collect::<Vec<_>>(),
            vec!["python", "typescript"]
        );
    }
}
