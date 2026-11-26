use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use github_releases::{AssetKind, download_server_binary};
use gpui::{App, AppContext as _, Task};
use project::{FileLockKind, Fs};

const PYTHON: &str = "3.13.18";
const BUILD: &str = "20261124";
const IPYKERNEL: &str = "7.4.0";
const MATPLOTLIB: &str = "3.11.2";

fn release_asset() -> Result<(&'static str, &'static str)> {
    Ok(match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") if cfg!(target_env = "gnu") => (
            "x86_64-unknown-linux-gnu",
            "b56ceb4fc1d8f0c49b7bd4dc91cd176bad7b86a88b9fc509bae6b10b3ee36692",
        ),
        ("linux", "aarch64") if cfg!(target_env = "gnu") => (
            "aarch64-unknown-linux-gnu",
            "300d388cef58ff23f101e1a41512dc51db9adae136b138c61af9ae1b1d9703a7",
        ),
        ("macos", "x86_64") => (
            "x86_64-apple-darwin",
            "bb16e167ec9e38368118ce3e43ba2c7852fea2cdba16e1852555a8997747f78a",
        ),
        ("macos", "aarch64") => (
            "aarch64-apple-darwin",
            "296f0903cccda15a5c9d3d8393cd2204718aeffd9b095a00f812d75c7b1c0967",
        ),
        ("windows", "x86_64") => (
            "x86_64-pc-windows-msvc",
            "a455077dd29dd8637b3ab6d745fc0a210d3504a0dd6bd31195ce1e11a81ec9ae",
        ),
        ("windows", "aarch64") => (
            "aarch64-pc-windows-msvc",
            "cacad3e38a40d320a9c0ef604efba46b3126239eeaa7cd4ad8765af9382d7e21",
        ),
        (os, arch) => anyhow::bail!("Zed cannot install a Python notebook kernel on {os} {arch}"),
    })
}

fn install_directory() -> PathBuf {
    paths::data_dir()
        .join("notebook-kernels")
        .join(format!(
            "{}-{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ))
        .join(format!(
            "python-{PYTHON}-{BUILD}-ipykernel-{IPYKERNEL}-matplotlib-{MATPLOTLIB}"
        ))
}

fn interpreter(directory: &Path) -> PathBuf {
    directory.join(if cfg!(windows) {
        "python/python.exe"
    } else {
        "python/bin/python3"
    })
}

fn ready_python(directory: &Path) -> Option<PathBuf> {
    let python = interpreter(directory);
    (directory.is_absolute() && directory.join("complete").is_file() && python.is_file())
        .then_some(python)
}

pub(crate) fn installed_python() -> Option<PathBuf> {
    ready_python(&install_directory())
}

pub(crate) fn install_python_kernel(cx: &App) -> Task<Result<PathBuf>> {
    install_in(install_directory(), cx)
}

fn install_in(directory: PathBuf, cx: &App) -> Task<Result<PathBuf>> {
    let executor = cx.background_executor().clone();
    let client = cx.http_client();
    let fs = <dyn Fs>::global(cx);
    cx.background_spawn(async move {
        let (target, sha256) = release_asset()?;
        anyhow::ensure!(directory.is_absolute(), "The notebook kernel directory must be absolute");
        let parent = directory
            .parent()
            .context("missing notebook kernel directory")?;
        smol::fs::create_dir_all(parent).await?;
        let _lock = fs
            .lock_file(&parent.join("install.lock"), FileLockKind::Exclusive)
            .await?;
        if let Some(python) = ready_python(&directory) {
            return Ok(python);
        }

        let url = format!(
            "https://github.com/astral-sh/python-build-standalone/releases/download/{BUILD}/cpython-{PYTHON}%2B{BUILD}-{target}-install_only_stripped.tar.gz"
        );
        download_server_binary(
            &executor, client, &url, Some(sha256), &directory, AssetKind::TarGz,
        )
        .await
        .context("Could not download Python. Check your connection, then run the cell again.")?;
        let python = interpreter(&directory);
        let scratch = tempfile::tempdir_in(parent)?;
        let install = [
            "-m",
            "pip",
            "--isolated",
            "--disable-pip-version-check",
            "--no-cache-dir",
            "install",
            "--only-binary=:all:",
            "--no-input",
            "--no-compile",
            "--index-url",
            "https://pypi.org/simple",
            &format!("ipykernel=={IPYKERNEL}"),
            &format!("matplotlib=={MATPLOTLIB}"),
        ];
        run_python(&python, scratch.path(), &install)
            .await
            .context("Could not install the notebook packages. Run the cell again to retry.")?;
        run_python(
            &python,
            scratch.path(),
            &["-c", "import ipykernel_launcher, matplotlib_inline.backend_inline"],
        )
        .await
        .context("checking the notebook kernel")?;
        smol::fs::write(directory.join("complete"), b"").await?;
        Ok(python)
    })
}

async fn run_python(python: &Path, scratch: &Path, args: &[&str]) -> Result<()> {
    let mut command = util::command::new_command(python);
    command
        .env_clear()
        .current_dir(scratch)
        .env("HOME", scratch)
        .env("TMPDIR", scratch)
        .env(
            "PIP_CONFIG_FILE",
            if cfg!(windows) { "nul" } else { "/dev/null" },
        );
    #[cfg(windows)]
    {
        command
            .env(
                "SystemRoot",
                std::env::var_os("SystemRoot").context("missing SystemRoot")?,
            )
            .env("USERPROFILE", scratch)
            .env("APPDATA", scratch)
            .env("LOCALAPPDATA", scratch)
            .env("TEMP", scratch)
            .env("TMP", scratch);
    }

    let output = command
        .arg("-I")
        .args(args)
        .kill_on_drop(true)
        .output()
        .await
        .with_context(|| format!("starting {}", python.display()))?;
    anyhow::ensure!(
        output.status.success(),
        "Python exited with {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr).trim()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_never_creates_or_returns_an_unfinished_install() -> Result<()> {
        let root = tempfile::tempdir()?;
        let directory = root.path().join("kernel");
        assert!(ready_python(&directory).is_none());
        assert!(!directory.exists());
        let python = interpreter(&directory);
        std::fs::create_dir_all(python.parent().context("missing interpreter directory")?)?;
        std::fs::write(&python, b"")?;
        assert!(ready_python(&directory).is_none());
        std::fs::write(directory.join("complete"), b"")?;
        assert_eq!(ready_python(&directory), Some(python.clone()));
        std::fs::remove_file(python)?;
        assert!(ready_python(&directory).is_none());
        Ok(())
    }

    #[gpui::test]
    async fn setup_retries_after_a_failed_download_without_exposing_a_kernel(
        cx: &mut gpui::TestAppContext,
    ) {
        if release_asset().is_err() {
            return;
        }
        cx.executor().allow_parking();
        let root = tempfile::tempdir().expect("temporary kernel directory");
        let directory = root.path().join("kernel");
        let first = cx
            .update(|cx| {
                <dyn Fs>::set_global(
                    project::RealFs::new(None, cx.background_executor().clone()),
                    cx,
                );
                cx.set_http_client(http_client::FakeHttpClient::with_404_response());
                install_in(directory.clone(), cx)
            })
            .await
            .expect_err("download should fail");
        assert!(format!("{first:#}").contains("404"));
        assert!(ready_python(&directory).is_none());

        let retry = cx
            .update(|cx| {
                cx.set_http_client(http_client::FakeHttpClient::with_200_response());
                install_in(directory.clone(), cx)
            })
            .await
            .expect_err("an unverified runtime must never be extracted");
        assert!(format!("{retry:#}").contains("SHA-256 mismatch"));
        assert!(!directory.exists());
    }

    #[gpui::test]
    #[ignore = "downloads Python and wheels from GitHub and PyPI"]
    async fn install_and_reuse_a_working_notebook_kernel(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();
        let root = tempfile::tempdir().expect("temporary kernel directory");
        let directory = root.path().join("kernel");
        let install = cx.update(|cx| {
            <dyn Fs>::set_global(
                project::RealFs::new(None, cx.background_executor().clone()),
                cx,
            );
            cx.set_http_client(std::sync::Arc::new(reqwest_client::ReqwestClient::new()));
            install_in(directory.clone(), cx)
        });
        let python = install.await.expect("install Python and wheels");
        assert_eq!(ready_python(&directory), Some(python.clone()));
        let reused = cx
            .update(|cx| {
                cx.set_http_client(http_client::FakeHttpClient::create(|request| async move {
                    panic!("reusing the kernel must not fetch {}", request.uri());
                }));
                install_in(directory, cx)
            })
            .await
            .expect("reuse the installed kernel without network");
        assert_eq!(python, reused);
        run_python(
            &python,
            root.path(),
            &[
                "-c",
                "from matplotlib.figure import Figure; Figure().savefig('plot.png')",
            ],
        )
        .await
        .expect("the private interpreter should support plots");
        assert!(root.path().join("plot.png").is_file());
    }
}
