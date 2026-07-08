use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use std::sync::Arc;

use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context as _, Result, bail};
use futures::{AsyncReadExt as _, AsyncWriteExt as _, FutureExt as _};
use gpui::{App, AppContext as _, Task};
use http_client::{AsyncBody, HttpClientWithUrl};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::lock::OnceCell;
use ui::SharedString;
use util::command::{Stdio, new_command};
use util::markdown::MarkdownEscaped;

use crate::{AgentTool, ToolCallEventStream, ToolInput};

const PACKAGE_NAME: &str = "browser-use";

/// Controls the user's web browser by running a short Python script with the
/// Browser Use helper functions, returning whatever the script prints.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BrowserToolInput {
    /// The Python script to run in the browser session.
    script: String,
}

pub struct BrowserTool {
    http_client: Arc<HttpClientWithUrl>,
    cli_path: OnceCell<Result<Arc<Path>, String>>,
}

impl BrowserTool {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            http_client,
            cli_path: OnceCell::new(),
        }
    }

    async fn cli_path(&self) -> Result<Arc<Path>, String> {
        self.cli_path
            .get_or_init(|| async {
                if let Some(cli_path) = Self::user_installed_cli().await {
                    log::debug!(
                        "Using user-installed {PACKAGE_NAME} CLI from: {}",
                        cli_path.display()
                    );
                    return Ok(cli_path);
                }
                Self::ensure_managed_cli(&self.http_client)
                    .await
                    .map_err(|error| format!("{error:#}"))
            })
            .await
            .clone()
    }

    async fn user_installed_cli() -> Option<Arc<Path>> {
        let output = new_command(PACKAGE_NAME)
            .arg("--version")
            .output()
            .await
            .ok()?;
        output
            .status
            .success()
            .then(|| Arc::from(Path::new(PACKAGE_NAME)))
    }

    async fn ensure_managed_cli(http_client: &Arc<HttpClientWithUrl>) -> Result<Arc<Path>> {
        const CLI_PATH_IN_VENV: &str = if cfg!(target_os = "windows") {
            "Scripts/browser-use.exe"
        } else {
            "bin/browser-use"
        };

        let install_dir = paths::data_dir().join("browser_use");
        std::fs::create_dir_all(&install_dir)?;
        let cli_path: Arc<Path> = install_dir.join("zed_venv").join(CLI_PATH_IN_VENV).into();
        let version_path = install_dir.join("installed_version");

        let installed_version = std::fs::read_to_string(&version_path).ok();
        let cli_exists = cli_path.exists();

        match Self::latest_version(http_client).await {
            Ok(latest_version) => {
                let is_up_to_date =
                    cli_exists && installed_version.as_deref() == Some(latest_version.as_str());
                if !is_up_to_date {
                    Self::pip_install(&install_dir, &latest_version).await?;
                    std::fs::write(&version_path, &latest_version)?;
                }
            }
            Err(error) => {
                if cli_exists {
                    log::warn!(
                        "Failed to check for the latest {PACKAGE_NAME} version, \
                         using the installed one: {error:#}"
                    );
                } else {
                    return Err(
                        error.context(format!("checking the latest {PACKAGE_NAME} version"))
                    );
                }
            }
        }

        Ok(cli_path)
    }

    async fn latest_version(http_client: &Arc<HttpClientWithUrl>) -> Result<String> {
        let response = http_client
            .get(
                &format!("https://pypi.org/pypi/{PACKAGE_NAME}/json"),
                AsyncBody::empty(),
                false,
            )
            .await?;
        anyhow::ensure!(
            response.status().is_success(),
            "PyPI responded with {}",
            response.status()
        );
        let mut body = String::new();
        response.into_body().read_to_string(&mut body).await?;
        let as_json = serde_json::Value::from_str(&body)?;
        as_json
            .get("info")
            .and_then(|info| info.get("version"))
            .and_then(|version| version.as_str())
            .map(ToOwned::to_owned)
            .context("parsing latest release information")
    }

    async fn pip_install(install_dir: &Path, version: &str) -> Result<()> {
        let venv_python = Self::venv_python(install_dir).await?;
        let output = new_command(&venv_python)
            .args(["-m", "pip", "install", "--upgrade"])
            .arg(format!("{PACKAGE_NAME}=={version}"))
            .output()
            .await
            .context("spawning pip")?;
        if !output.status.success() {
            bail!(
                "installing {PACKAGE_NAME} {version} failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    async fn venv_python(install_dir: &Path) -> Result<PathBuf> {
        const PYTHON_PATH_IN_VENV: &str = if cfg!(target_os = "windows") {
            "Scripts/python.exe"
        } else {
            "bin/python3"
        };

        let venv_python = install_dir.join("zed_venv").join(PYTHON_PATH_IN_VENV);
        if venv_python.exists() {
            return Ok(venv_python);
        }

        let base_python = Self::system_python().await.with_context(|| {
            let mut message = "Could not find a Python installation".to_owned();
            if cfg!(windows) {
                message.push_str(
                    ". Install Python from the Microsoft Store, or manually from \
                     https://www.python.org/downloads/windows.",
                );
            }
            message
        })?;

        let output = new_command(&base_python)
            .args(["-m", "venv", "zed_venv"])
            .current_dir(install_dir)
            .output()
            .await
            .context("spawning python to create a virtual environment")?;
        if !output.status.success() {
            bail!(
                "Failed to create a virtual environment with {base_python} in {}:\n{}{}",
                install_dir.display(),
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout),
            );
        }

        Ok(venv_python)
    }

    async fn system_python() -> Option<String> {
        const BINARY_NAMES: [&str; 3] = ["python3", "python", "py"];
        for binary_name in BINARY_NAMES {
            // Detect situations where `python3` exists but is not a real Python
            // interpreter. Notably, on fresh Windows installs, `python3` is a shim
            // that opens the Microsoft Store app when run with no arguments, and
            // just fails otherwise.
            let Ok(output) = new_command(binary_name)
                .args(["-c", "print(1 + 2)"])
                .output()
                .await
            else {
                continue;
            };
            if output.status.success() && output.stdout.trim_ascii() == b"3" {
                return Some(binary_name.to_owned());
            }
        }
        None
    }

    async fn run_script(cli_path: &Path, script: &str) -> Result<String> {
        let mut child = new_command(cli_path)
            .env("ANONYMIZED_TELEMETRY", "false")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("spawning {}", cli_path.display()))?;

        let mut stdin = child.stdin.take().context("opening the CLI's stdin")?;
        stdin.write_all(script.as_bytes()).await?;
        stdin.close().await?;
        drop(stdin);

        let output = child.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !output.status.success() {
            bail!(
                "{PACKAGE_NAME} exited with {}:\n{stderr}{stdout}",
                output.status
            );
        }
        if stdout.trim().is_empty() {
            Ok("The script produced no output. Print the values you need returned.".into())
        } else {
            Ok(stdout)
        }
    }
}

impl AgentTool for BrowserTool {
    type Input = BrowserToolInput;
    type Output = String;

    const NAME: &'static str = "browser";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Fetch
    }

    fn allow_in_restricted_mode() -> bool {
        false
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) => {
                let first_line = input.script.lines().next().unwrap_or_default();
                format!("Browser: {}", MarkdownEscaped(first_line)).into()
            }
            Err(_) => "Use browser".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|e| e.to_string())?;

            let authorize = cx.update(|cx| {
                let context =
                    crate::ToolPermissionContext::new(Self::NAME, vec![input.script.clone()]);
                event_stream.authorize("Control the browser".to_string(), context, cx)
            });
            futures::select! {
                result = authorize.fuse() => result.map_err(|e| e.to_string())?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Browser use cancelled by user".to_string());
                }
            };

            let script_task = cx.background_spawn({
                let this = self.clone();
                async move {
                    let cli_path = this.cli_path().await.map_err(|error| {
                        format!("Failed to provision the {PACKAGE_NAME} CLI: {error}")
                    })?;
                    Self::run_script(&cli_path, &input.script)
                        .await
                        .map_err(|error| format!("{error:#}"))
                }
            });
            futures::select! {
                result = script_task.fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    Err("Browser use cancelled by user".to_string())
                }
            }
        })
    }
}
