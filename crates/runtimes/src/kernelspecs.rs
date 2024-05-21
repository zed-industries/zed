// Detect all kernelspecs available on the system,
// watch for changes to the kernelspecs directory,

// Since runtimelib uses tokio, we'll only use `runtimelib::dirs` for paths and reimplement
// the rest using `project::Fs`.

use futures::StreamExt;
use project::Fs;
use std::process::Stdio;
use std::{path::PathBuf, sync::Arc};

use smol::process::Command;

use runtimelib::dirs;
use runtimelib::JupyterKernelspec;

#[derive(Debug)]
pub struct Runtime {
    pub name: String,
    pub path: PathBuf,
    pub spec: JupyterKernelspec,
}

impl Runtime {
    fn command(&self, connection_path: &PathBuf) -> anyhow::Result<Command> {
        let argv = &self.spec.argv;

        if argv.is_empty() {
            return Err(anyhow::anyhow!("Empty argv in kernelspec {}", self.name));
        }

        if argv.len() < 2 {
            return Err(anyhow::anyhow!("Invalid argv in kernelspec {}", self.name));
        }

        if !argv.contains(&"{connection_file}".to_string()) {
            return Err(anyhow::anyhow!(
                "Missing 'connection_file' in argv in kernelspec {}",
                self.name
            ));
        }

        let mut cmd = Command::new(&argv[0]);

        for arg in &argv[1..] {
            if arg == "{connection_file}" {
                cmd.arg(connection_path);
            } else {
                cmd.arg(arg);
            }
        }

        if let Some(env) = &self.spec.env {
            cmd.envs(env);
        }

        Ok(cmd)
    }
}

pub async fn read_kernelspec_at(
    // Path should be a directory to a jupyter kernelspec, as in
    // /usr/local/share/jupyter/kernels/python3
    kernel_dir: PathBuf,
    fs: Arc<dyn Fs>,
) -> anyhow::Result<Runtime> {
    let path = kernel_dir;
    let kernel_name = if let Some(kernel_name) = path.file_name() {
        kernel_name.to_string_lossy().to_string()
    } else {
        return Err(anyhow::anyhow!("Invalid kernelspec directory: {:?}", path));
    };

    if !fs.is_dir(path.as_path()).await {
        return Err(anyhow::anyhow!("Not a directory: {:?}", path));
    }

    let expected_kernel_json = path.join("kernel.json");
    let spec = fs.load(expected_kernel_json.as_path()).await?;
    let spec = serde_json::from_str::<JupyterKernelspec>(&spec)?;

    Ok(Runtime {
        name: kernel_name,
        path,
        spec,
    })
}

/// Read a directory of kernelspec directories
pub async fn read_kernels_dir(path: PathBuf, fs: Arc<dyn Fs>) -> anyhow::Result<Vec<Runtime>> {
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

pub async fn get_runtimes(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<Runtime>> {
    let data_dirs = dirs::data_dirs();
    let kernel_dirs = data_dirs
        .iter()
        .map(|dir| dir.join("kernels"))
        .map(|path| read_kernels_dir(path, fs.clone()))
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

        let mut kernels = read_kernels_dir(PathBuf::from("/jupyter/kernels"), fs)
            .await
            .unwrap();

        kernels.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(
            kernels.iter().map(|c| c.name.clone()).collect::<Vec<_>>(),
            vec!["deno", "python"]
        );
    }
}
