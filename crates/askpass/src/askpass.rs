mod encrypted_password;

pub use encrypted_password::{EncryptedPassword, IKnowWhatIAmDoingAndIHaveReadTheDocs};

use net::async_net::UnixListener;
use smol::lock::Mutex;
use util::fs::make_file_executable;

use std::ffi::OsStr;
use std::ops::ControlFlow;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{Context as _, Result};
use futures::channel::{mpsc, oneshot};
use futures::{
    AsyncBufReadExt as _, AsyncWriteExt as _, FutureExt as _, SinkExt, StreamExt, io::BufReader,
    select_biased,
};
use gpui::{AsyncApp, BackgroundExecutor, Task};
use smol::fs;
use util::{ResultExt as _, debug_panic, maybe, paths::PathExt, shell::ShellKind};

/// Path to the program used for askpass
///
/// On Unix and remote servers, this defaults to the current executable
/// On Windows, this is set to the CLI variant of zed
static ASKPASS_PROGRAM: OnceLock<std::path::PathBuf> = OnceLock::new();

#[derive(PartialEq, Eq)]
pub enum AskPassResult {
    CancelledByUser,
    Timedout,
}

pub struct AskPassDelegate {
    tx: mpsc::UnboundedSender<(String, oneshot::Sender<EncryptedPassword>)>,
    executor: BackgroundExecutor,
    _task: Task<()>,
}

impl AskPassDelegate {
    pub fn new(
        cx: &mut AsyncApp,
        password_prompt: impl Fn(String, oneshot::Sender<EncryptedPassword>, &mut AsyncApp)
        + Send
        + Sync
        + 'static,
    ) -> Self {
        let (tx, mut rx) = mpsc::unbounded::<(String, oneshot::Sender<_>)>();
        let task = cx.spawn(async move |cx: &mut AsyncApp| {
            while let Some((prompt, channel)) = rx.next().await {
                password_prompt(prompt, channel, cx);
            }
        });
        Self {
            tx,
            _task: task,
            executor: cx.background_executor().clone(),
        }
    }

    pub fn ask_password(&mut self, prompt: String) -> Task<Option<EncryptedPassword>> {
        let mut this_tx = self.tx.clone();
        self.executor.spawn(async move {
            let (tx, rx) = oneshot::channel();
            this_tx.send((prompt, tx)).await.ok()?;
            rx.await.ok()
        })
    }
}

pub struct AskPassSession {
    #[cfg(target_os = "windows")]
    secret: std::sync::Arc<OnceLock<EncryptedPassword>>,
    askpass_task: PasswordProxy,
    askpass_opened_rx: Option<oneshot::Receiver<()>>,
    askpass_kill_master_rx: Option<oneshot::Receiver<()>>,
}

#[cfg(not(target_os = "windows"))]
const ASKPASS_SCRIPT_NAME: &str = "askpass.sh";
#[cfg(target_os = "windows")]
const ASKPASS_SCRIPT_NAME: &str = "askpass.ps1";

impl AskPassSession {
    /// This will create a new AskPassSession.
    /// You must retain this session until the master process exits.
    #[must_use]
    pub async fn new(executor: &BackgroundExecutor, mut delegate: AskPassDelegate) -> Result<Self> {
        #[cfg(target_os = "windows")]
        let secret = std::sync::Arc::new(OnceLock::new());
        let (askpass_opened_tx, askpass_opened_rx) = oneshot::channel::<()>();

        let askpass_opened_tx = Arc::new(Mutex::new(Some(askpass_opened_tx)));

        let (askpass_kill_master_tx, askpass_kill_master_rx) = oneshot::channel::<()>();
        let kill_tx = Arc::new(Mutex::new(Some(askpass_kill_master_tx)));

        #[cfg(target_os = "windows")]
        let askpass_secret = secret.clone();
        let get_password = {
            let executor = executor.clone();

            move |prompt| {
                let prompt = delegate.ask_password(prompt);
                let kill_tx = kill_tx.clone();
                let askpass_opened_tx = askpass_opened_tx.clone();
                #[cfg(target_os = "windows")]
                let askpass_secret = askpass_secret.clone();
                executor.spawn(async move {
                    if let Some(askpass_opened_tx) = askpass_opened_tx.lock().await.take() {
                        askpass_opened_tx.send(()).ok();
                    }
                    if let Some(password) = prompt.await {
                        #[cfg(target_os = "windows")]
                        {
                            _ = askpass_secret.set(password.clone());
                        }
                        ControlFlow::Continue(Ok(password))
                    } else {
                        if let Some(kill_tx) = kill_tx.lock().await.take() {
                            kill_tx.send(()).log_err();
                        }
                        ControlFlow::Break(())
                    }
                })
            }
        };
        let askpass_task = PasswordProxy::new(get_password, executor.clone()).await?;

        Ok(Self {
            #[cfg(target_os = "windows")]
            secret,

            askpass_task,
            askpass_kill_master_rx: Some(askpass_kill_master_rx),
            askpass_opened_rx: Some(askpass_opened_rx),
        })
    }

    // This will run the askpass task forever, resolving as many authentication requests as needed.
    // The caller is responsible for examining the result of their own commands and cancelling this
    // future when this is no longer needed. Note that this can only be called once, but due to the
    // drop order this takes an &mut, so you can `drop()` it after you're done with the master process.
    pub async fn run(&mut self) -> AskPassResult {
        // This is the default timeout setting used by VSCode.
        let connection_timeout = Duration::from_secs(17);
        let askpass_opened_rx = self.askpass_opened_rx.take().expect("Only call run once");
        let askpass_kill_master_rx = self
            .askpass_kill_master_rx
            .take()
            .expect("Only call run once");

        select_biased! {
            _ = askpass_opened_rx.fuse() => {
                // Note: this await can only resolve after we are dropped.
                askpass_kill_master_rx.await.ok();
                AskPassResult::CancelledByUser
            }

            _ = futures::FutureExt::fuse(smol::Timer::after(connection_timeout)) => {
                AskPassResult::Timedout
            }
        }
    }

    /// This will return the password that was last set by the askpass script.
    #[cfg(target_os = "windows")]
    pub fn get_password(&self) -> Option<EncryptedPassword> {
        self.secret.get().cloned()
    }

    pub fn script_path(&self) -> impl AsRef<OsStr> {
        self.askpass_task.script_path()
    }
}

pub struct PasswordProxy {
    _task: Task<()>,
    #[cfg(not(target_os = "windows"))]
    askpass_script_path: std::path::PathBuf,
    #[cfg(target_os = "windows")]
    askpass_helper: String,
}

impl PasswordProxy {
    pub async fn new(
        mut get_password: impl FnMut(String) -> Task<ControlFlow<(), Result<EncryptedPassword>>>
        + 'static
        + Send
        + Sync,
        executor: BackgroundExecutor,
    ) -> Result<Self> {
        let temp_dir = tempfile::Builder::new().prefix("zed-askpass").tempdir()?;
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let askpass_script_path = temp_dir.path().join(ASKPASS_SCRIPT_NAME);
        let current_exec =
            std::env::current_exe().context("Failed to determine current zed executable path.")?;

        // TODO: inferred from the use of powershell.exe in askpass_helper_script
        let shell_kind = if cfg!(windows) {
            ShellKind::PowerShell
        } else {
            ShellKind::Posix
        };
        let askpass_program = ASKPASS_PROGRAM
            .get_or_init(|| current_exec)
            .try_shell_safe(shell_kind)
            .context("Failed to shell-escape Askpass program path.")?
            .to_string();
        // Create an askpass script that communicates back to this process.
        let askpass_script = generate_askpass_script(&askpass_program, &askpass_socket);
        let _task = executor.spawn(async move {
            maybe!(async move {
                let listener =
                    UnixListener::bind(&askpass_socket).context("creating askpass socket")?;

                while let Ok((mut stream, _)) = listener.accept().await {
                    let mut buffer = Vec::new();
                    let mut reader = BufReader::new(&mut stream);
                    if reader.read_until(b'\0', &mut buffer).await.is_err() {
                        buffer.clear();
                    }
                    let prompt = String::from_utf8_lossy(&buffer).into_owned();
                    let password = get_password(prompt).await;
                    match password {
                        ControlFlow::Continue(password) => {
                            if let Ok(password) = password
                                && let Ok(decrypted) =
                                    password.decrypt(IKnowWhatIAmDoingAndIHaveReadTheDocs)
                            {
                                stream.write_all(decrypted.as_bytes()).await.log_err();
                            }
                        }
                        ControlFlow::Break(()) => {
                            // note: we expect the caller to drop this task when it's done.
                            // We need to keep the stream open until the caller is done to avoid
                            // spurious errors from ssh.
                            std::future::pending::<()>().await;
                            drop(stream);
                        }
                    }
                }
                drop(temp_dir);
                Result::<_, anyhow::Error>::Ok(())
            })
            .await
            .log_err();
        });

        fs::write(&askpass_script_path, askpass_script)
            .await
            .with_context(|| format!("creating askpass script at {askpass_script_path:?}"))?;
        make_file_executable(&askpass_script_path).await?;
        #[cfg(target_os = "windows")]
        let askpass_helper = format!(
            "powershell.exe -ExecutionPolicy Bypass -File {}",
            askpass_script_path.display()
        );

        Ok(Self {
            _task,
            #[cfg(not(target_os = "windows"))]
            askpass_script_path,
            #[cfg(target_os = "windows")]
            askpass_helper,
        })
    }

    pub fn script_path(&self) -> impl AsRef<OsStr> {
        #[cfg(not(target_os = "windows"))]
        {
            &self.askpass_script_path
        }
        #[cfg(target_os = "windows")]
        {
            &self.askpass_helper
        }
    }
}
/// The main function for when Zed is running in netcat mode for use in askpass.
/// Called from both the remote server binary and the zed binary in their respective main functions.
pub fn main(socket: &str) {
    use net::UnixStream;
    use std::io::{self, Read, Write};
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

    #[cfg(target_os = "windows")]
    while buffer.last().is_some_and(|&b| b == b'\n' || b == b'\r') {
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

pub fn set_askpass_program(path: std::path::PathBuf) {
    if ASKPASS_PROGRAM.set(path).is_err() {
        debug_panic!("askpass program has already been set");
    }
}

#[inline]
#[cfg(not(target_os = "windows"))]
fn generate_askpass_script(askpass_program: &str, askpass_socket: &std::path::Path) -> String {
    format!(
        "{shebang}\n{print_args} | {askpass_program} --askpass={askpass_socket} 2> /dev/null \n",
        askpass_socket = askpass_socket.display(),
        print_args = "printf '%s\\0' \"$@\"",
        shebang = "#!/bin/sh",
    )
}

#[inline]
#[cfg(target_os = "windows")]
fn generate_askpass_script(askpass_program: &str, askpass_socket: &std::path::Path) -> String {
    format!(
        r#"
        $ErrorActionPreference = 'Stop';
        ($args -join [char]0) | & {askpass_program} --askpass={askpass_socket} 2> $null
        "#,
        askpass_socket = askpass_socket.display(),
    )
}
