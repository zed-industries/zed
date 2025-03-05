use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context as _;
use futures::channel::oneshot;
use futures::{io::BufReader, AsyncBufReadExt as _};
use futures::{select_biased, AsyncWriteExt as _, FutureExt as _};
use gpui::{AsyncApp, Task};
use smol::fs;
use smol::{fs::unix::PermissionsExt as _, net::unix::UnixListener};
use tempfile::TempDir;
use util::ResultExt as _;

pub struct AskPassSession {
    script_path: PathBuf,
    _askpass_task: Task<()>,
    askpass_opened_rx: Option<oneshot::Receiver<()>>,
    askpass_kill_master_rx: Option<oneshot::Receiver<()>>,
}

#[derive(PartialEq, Eq)]
pub enum AskPassResult {
    CancelledByUser,
    Timedout,
}

impl AskPassSession {
    /// This will create a new AskPassSession.
    /// You must retain this session until the master process exits.
    #[must_use]
    pub async fn new(
        temp_dir: &TempDir,
        cx: &mut AsyncApp,
        password_prompt: impl Fn(String, &mut AsyncApp) -> oneshot::Receiver<anyhow::Result<String>>
            + Send
            + Sync
            + 'static,
    ) -> anyhow::Result<Self> {
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let (askpass_opened_tx, askpass_opened_rx) = oneshot::channel::<()>();
        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;

        let (askpass_kill_master_tx, askpass_kill_master_rx) = oneshot::channel::<()>();
        let mut kill_tx = Some(askpass_kill_master_tx);

        let askpass_task = cx.spawn(|mut cx| async move {
            let mut askpass_opened_tx = Some(askpass_opened_tx);

            while let Ok((mut stream, _)) = listener.accept().await {
                if let Some(askpass_opened_tx) = askpass_opened_tx.take() {
                    askpass_opened_tx.send(()).ok();
                }
                let mut buffer = Vec::new();
                let mut reader = BufReader::new(&mut stream);
                if reader.read_until(b'\0', &mut buffer).await.is_err() {
                    buffer.clear();
                }
                let prompt = String::from_utf8_lossy(&buffer);
                if let Some(password) = password_prompt(prompt.to_string(), &mut cx)
                    .await
                    .context("failed to get askpass password")
                    .and_then(|p| p)
                    .log_err()
                {
                    stream.write_all(password.as_bytes()).await.log_err();
                } else {
                    if let Some(kill_tx) = kill_tx.take() {
                        kill_tx.send(()).log_err();
                    }
                    // note: we expect the caller to drop this task when it's done.
                    // We need to keep the stream open until the caller is done to avoid
                    // spurious errors from ssh.
                    std::future::pending::<()>().await;
                    drop(stream);
                }
            }
        });

        anyhow::ensure!(
            which::which("nc").is_ok(),
            "Cannot find `nc` command (netcat), which is required to connect over SSH."
        );

        // Create an askpass script that communicates back to this process.
        let askpass_script = format!(
            "{shebang}\n{print_args} | {nc} -U {askpass_socket} 2> /dev/null \n",
            // on macOS `brew install netcat` provides the GNU netcat implementation
            // which does not support -U.
            nc = if cfg!(target_os = "macos") {
                "/usr/bin/nc"
            } else {
                "nc"
            },
            askpass_socket = askpass_socket.display(),
            print_args = "printf '%s\\0' \"$@\"",
            shebang = "#!/bin/sh",
        );
        let askpass_script_path = temp_dir.path().join("askpass.sh");
        fs::write(&askpass_script_path, askpass_script).await?;
        fs::set_permissions(&askpass_script_path, std::fs::Permissions::from_mode(0o755)).await?;

        Ok(Self {
            script_path: askpass_script_path,
            _askpass_task: askpass_task,
            askpass_kill_master_rx: Some(askpass_kill_master_rx),
            askpass_opened_rx: Some(askpass_opened_rx),
        })
    }

    pub fn script_path(&self) -> &Path {
        &self.script_path
    }

    // This will run the askpass task forever, resolving as many authentication requests as needed.
    // The caller is responsible for examining the result of their own commands and cancelling this
    // future when this is no longer needed. Note that this can only be called once, but due to the
    // drop order this takes an &mut, so you can `drop()` it after you're done with the master process.
    pub async fn run(&mut self) -> AskPassResult {
        let connection_timeout = Duration::from_secs(10);
        let askpass_opened_rx = self.askpass_opened_rx.take().expect("Only call run once");
        let askpass_kill_master_rx = self
            .askpass_kill_master_rx
            .take()
            .expect("Only call run once");

        select_biased! {
            _ = askpass_opened_rx.fuse() => {
                // Note: this await can only resolve after we are dropped.
                askpass_kill_master_rx.await.ok();
                return AskPassResult::CancelledByUser
            }

            _ = futures::FutureExt::fuse(smol::Timer::after(connection_timeout)) => {
                return AskPassResult::Timedout
            }
        }
    }
}
