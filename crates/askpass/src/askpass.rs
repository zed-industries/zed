use std::time::Duration;

use anyhow::Context as _;
use futures::channel::{mpsc, oneshot};
use futures::{
    AsyncBufReadExt as _, AsyncWriteExt as _, FutureExt as _, SinkExt, StreamExt, io::BufReader,
    select_biased,
};
use gpui::{AsyncApp, BackgroundExecutor, Task};
use smol::fs;
use util::ResultExt as _;
#[cfg(unix)]
use util::get_shell_safe_zed_path;

#[derive(PartialEq, Eq)]
pub enum AskPassResult {
    CancelledByUser,
    Timedout,
}

pub struct AskPassDelegate {
    tx: mpsc::UnboundedSender<(String, oneshot::Sender<String>)>,
    _task: Task<()>,
}

impl AskPassDelegate {
    pub fn new(
        cx: &mut AsyncApp,
        password_prompt: impl Fn(String, oneshot::Sender<String>, &mut AsyncApp) + Send + Sync + 'static,
    ) -> Self {
        let (tx, mut rx) = mpsc::unbounded::<(String, oneshot::Sender<String>)>();
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            while let Some((prompt, channel)) = rx.next().await {
                password_prompt(prompt, channel, cx);
            }
        });
        Self { tx, _task: task }
    }

    pub async fn ask_password(&mut self, prompt: String) -> anyhow::Result<String> {
        let (tx, rx) = oneshot::channel();
        self.tx.send((prompt, tx)).await?;
        Ok(rx.await?)
    }
}

#[cfg(unix)]
pub struct AskPassSession {
    script_path: std::path::PathBuf,
    _askpass_task: Task<()>,
    askpass_opened_rx: Option<oneshot::Receiver<()>>,
    askpass_kill_master_rx: Option<oneshot::Receiver<()>>,
}

#[cfg(unix)]
impl AskPassSession {
    /// This will create a new AskPassSession.
    /// You must retain this session until the master process exits.
    #[must_use]
    pub async fn new(
        executor: &BackgroundExecutor,
        mut delegate: AskPassDelegate,
    ) -> anyhow::Result<Self> {
        use smol::{fs::unix::PermissionsExt as _, net::unix::UnixListener};

        let temp_dir = tempfile::Builder::new().prefix("zed-askpass").tempdir()?;
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let askpass_script_path = temp_dir.path().join("askpass.sh");
        let (askpass_opened_tx, askpass_opened_rx) = oneshot::channel::<()>();
        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;
        let zed_path = get_shell_safe_zed_path()?;

        let (askpass_kill_master_tx, askpass_kill_master_rx) = oneshot::channel::<()>();
        let mut kill_tx = Some(askpass_kill_master_tx);

        let askpass_task = executor.spawn(async move {
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
                if let Some(password) = delegate
                    .ask_password(prompt.to_string())
                    .await
                    .context("failed to get askpass password")
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
            drop(temp_dir)
        });

        // Create an askpass script that communicates back to this process.
        let askpass_script = format!(
            "{shebang}\n{print_args} | {zed_exe} --askpass={askpass_socket} 2> /dev/null \n",
            zed_exe = zed_path,
            askpass_socket = askpass_socket.display(),
            print_args = "printf '%s\\0' \"$@\"",
            shebang = "#!/bin/sh",
        );
        fs::write(&askpass_script_path, askpass_script).await?;
        fs::set_permissions(&askpass_script_path, std::fs::Permissions::from_mode(0o755)).await?;

        Ok(Self {
            script_path: askpass_script_path,
            _askpass_task: askpass_task,
            askpass_kill_master_rx: Some(askpass_kill_master_rx),
            askpass_opened_rx: Some(askpass_opened_rx),
        })
    }

    pub fn script_path(&self) -> &std::path::Path {
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

/// The main function for when Zed is running in netcat mode for use in askpass.
/// Called from both the remote server binary and the zed binary in their respective main functions.
#[cfg(unix)]
pub fn main(socket: &str) {
    use std::io::{self, Read, Write};
    use std::os::unix::net::UnixStream;
    use std::process::exit;

    let mut stream = match UnixStream::connect(socket) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Error connecting to socket {}: {}", socket, err);
            exit(1);
        }
    };

    let mut buffer = Vec::new();
    if let Err(err) = io::stdin().read_to_end(&mut buffer) {
        eprintln!("Error reading from stdin: {}", err);
        exit(1);
    }

    if buffer.last() != Some(&b'\0') {
        buffer.push(b'\0');
    }

    if let Err(err) = stream.write_all(&buffer) {
        eprintln!("Error writing to socket: {}", err);
        exit(1);
    }

    let mut response = Vec::new();
    if let Err(err) = stream.read_to_end(&mut response) {
        eprintln!("Error reading from socket: {}", err);
        exit(1);
    }

    if let Err(err) = io::stdout().write_all(&response) {
        eprintln!("Error writing to stdout: {}", err);
        exit(1);
    }
}

#[cfg(not(unix))]
pub fn main(socket: &str) {
    use std::io::{self, Read, Write};
    use std::process::exit;

    use windows_net::UnixStream;

    let mut stream = match UnixStream::connect(socket) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Error connecting to socket {}: {}", socket, err);
            exit(1);
        }
    };

    let mut buffer = Vec::new();
    if let Err(err) = io::stdin().read_to_end(&mut buffer) {
        eprintln!("Error reading from stdin: {}", err);
        exit(1);
    }

    while buffer.last().map_or(false, |&b| b == b'\n' || b == b'\r') {
        buffer.pop();
    }
    if buffer.last() != Some(&b'\0') {
        buffer.push(b'\0');
    }

    if let Err(err) = stream.write_all(&buffer) {
        eprintln!("Error writing to socket: {}", err);
        exit(1);
    }

    let mut response = Vec::new();
    if let Err(err) = stream.read_to_end(&mut response) {
        eprintln!("Error reading from socket: {}", err);
        exit(1);
    }

    if let Err(err) = io::stdout().write_all(&response) {
        eprintln!("Error writing to stdout: {}", err);
        exit(1);
    }
}

#[cfg(not(unix))]
pub struct AskPassSession {
    askpass_helper: String,
    _askpass_task: Task<()>,
    secrete: std::sync::Arc<std::sync::Mutex<String>>,
    askpass_opened_rx: Option<oneshot::Receiver<()>>,
    askpass_kill_master_rx: Option<oneshot::Receiver<()>>,
}

#[cfg(not(unix))]
impl AskPassSession {
    #[must_use]
    pub async fn new(
        executor: &BackgroundExecutor,
        mut delegate: AskPassDelegate,
    ) -> anyhow::Result<Self> {
        use windows_net::async_net::UnixListener;

        let secrete = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let temp_dir = tempfile::Builder::new().prefix("zed-askpass").tempdir()?;
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let askpass_script_path = temp_dir.path().join("askpass.ps1");
        let (askpass_opened_tx, askpass_opened_rx) = oneshot::channel::<()>();
        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;
        let zed_path = std::env::current_exe()
            .context("Failed to figure out current executable path for use in askpass")?;

        let (askpass_kill_master_tx, askpass_kill_master_rx) = oneshot::channel::<()>();
        let mut kill_tx = Some(askpass_kill_master_tx);

        let askpass_task = executor.spawn({
            let secrete = secrete.clone();
            async move {
                let mut askpass_opened_tx = Some(askpass_opened_tx);

                while let Ok(mut stream) = listener.accept().await {
                    if let Some(askpass_opened_tx) = askpass_opened_tx.take() {
                        askpass_opened_tx.send(()).ok();
                    }
                    let mut buffer = Vec::new();
                    let mut reader = BufReader::new(&mut stream);
                    if reader.read_until(b'\0', &mut buffer).await.is_err() {
                        buffer.clear();
                    }
                    let prompt = String::from_utf8_lossy(&buffer);
                    if let Some(password) = delegate
                        .ask_password(prompt.to_string())
                        .await
                        .context("failed to get askpass password")
                        .log_err()
                    {
                        stream.write_all(password.as_bytes()).await.log_err();
                        *secrete.lock().unwrap() = password;
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
                drop(temp_dir)
            }
        });

        // Create an askpass script that communicates back to this process.
        let askpass_script = format!(
            r#"
            $ErrorActionPreference = 'Stop';
            ($args -join [char]0) | & "{zed_exe}" --askpass={askpass_socket} 2> $null
            "#,
            zed_exe = zed_path.display(),
            askpass_socket = askpass_socket.display(),
        );
        fs::write(&askpass_script_path, askpass_script).await?;
        let askpass_helper = format!(
            "powershell.exe -ExecutionPolicy Bypass -File {}",
            askpass_script_path.display()
        );

        Ok(Self {
            askpass_helper,
            _askpass_task: askpass_task,
            secrete,
            askpass_kill_master_rx: Some(askpass_kill_master_rx),
            askpass_opened_rx: Some(askpass_opened_rx),
        })
    }

    pub fn script_path(&self) -> &str {
        &self.askpass_helper
    }

    // This will run the askpass task forever, resolving as many authentication requests as needed.
    // The caller is responsible for examining the result of their own commands and cancelling this
    // future when this is no longer needed. Note that this can only be called once, but due to the
    // drop order this takes an &mut, so you can `drop()` it after you're done with the master process.
    pub async fn run(&mut self) -> AskPassResult {
        let connection_timeout = Duration::from_secs(60);
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

    /// TODO:
    pub fn get_password(&self) -> String {
        let secrete = self.secrete.lock().unwrap();
        secrete.clone()
    }
}
