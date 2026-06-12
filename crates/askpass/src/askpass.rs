mod encrypted_password;

pub use encrypted_password::{EncryptedPassword, IKnowWhatIAmDoingAndIHaveReadTheDocs};

use net::async_net::UnixListener;
use smol::lock::Mutex;
#[cfg(not(target_os = "windows"))]
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
#[cfg(not(target_os = "windows"))]
use smol::fs;
use util::{ResultExt as _, debug_panic, maybe};

#[cfg(not(target_os = "windows"))]
use util::{paths::PathExt, shell::ShellKind};

/// Path to the program used for askpass
///
/// On Unix and remote servers, this defaults to the current executable.
/// On Windows, this must be set to the CLI variant of zed via set_askpass_program(),
/// because SSH_ASKPASS must point to a directly executable binary. The CLI binary
/// handles the ZED_ASKPASS_SOCKET env var to communicate with Zed over a Unix socket
/// without needing a wrapper script.
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
    secret: std::sync::Arc<std::sync::Mutex<Option<EncryptedPassword>>>,
    askpass_task: PasswordProxy,
    askpass_opened_rx: Option<oneshot::Receiver<()>>,
    askpass_kill_master_rx: Option<oneshot::Receiver<()>>,
    executor: BackgroundExecutor,
}

#[cfg(not(target_os = "windows"))]
const ASKPASS_SCRIPT_NAME: &str = "askpass.sh";

impl AskPassSession {
    /// This will create a new AskPassSession.
    /// You must retain this session until the master process exits.
    #[must_use]
    pub async fn new(executor: BackgroundExecutor, mut delegate: AskPassDelegate) -> Result<Self> {
        #[cfg(target_os = "windows")]
        let secret = std::sync::Arc::new(std::sync::Mutex::new(None));

        let (askpass_opened_tx, askpass_opened_rx) = oneshot::channel::<()>();

        let askpass_opened_tx = Arc::new(Mutex::new(Some(askpass_opened_tx)));

        let (askpass_kill_master_tx, askpass_kill_master_rx) = oneshot::channel::<()>();
        let kill_tx = Arc::new(Mutex::new(Some(askpass_kill_master_tx)));

        let get_password = {
            let executor = executor.clone();

            #[cfg(target_os = "windows")]
            let askpass_secret = secret.clone();
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
                            askpass_secret.lock().unwrap().replace(password.clone());
                        }
                        ControlFlow::Continue(Ok(password))
                    } else {
                        if let Some(kill_tx) = kill_tx.lock().await.take() {
                            kill_tx.send(()).ok();
                        }
                        ControlFlow::Break(())
                    }
                })
            }
        };
        let askpass_task = PasswordProxy::new(Box::new(get_password), executor.clone()).await?;

        Ok(Self {
            #[cfg(target_os = "windows")]
            secret,

            askpass_task,
            askpass_kill_master_rx: Some(askpass_kill_master_rx),
            askpass_opened_rx: Some(askpass_opened_rx),
            executor,
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
        let executor = self.executor.clone();

        select_biased! {
            _ = askpass_opened_rx.fuse() => {
                // Note: this await can only resolve after we are dropped.
                askpass_kill_master_rx.await.ok();
                AskPassResult::CancelledByUser
            }

            _ = futures::FutureExt::fuse(executor.timer(connection_timeout)) => {
                AskPassResult::Timedout
            }
        }
    }

    /// This will return the password that was last set by the askpass script.
    #[cfg(target_os = "windows")]
    pub fn get_password(&self) -> Option<EncryptedPassword> {
        self.secret.lock().ok()?.clone()
    }

    /// Returns the value to set as SSH_ASKPASS.
    /// On Unix this is the path to the generated shell script.
    /// On Windows this is the path to cli.exe directly — no script needed.
    pub fn script_path(&self) -> impl AsRef<OsStr> {
        self.askpass_task.script_path()
    }

    /// Returns the socket path to set as ZED_ASKPASS_SOCKET.
    ///
    /// On Windows, SSH_ASKPASS points directly to cli.exe. SSH passes only
    /// the prompt string as argv[1] with no mechanism for extra arguments,
    /// so the socket path is communicated via this environment variable instead.
    /// cli.exe must check ZED_ASKPASS_SOCKET before clap parses args.
    #[cfg(target_os = "windows")]
    pub fn socket_path(&self) -> impl AsRef<OsStr> {
        self.askpass_task.socket_path()
    }
}

pub struct PasswordProxy {
    _task: Task<()>,
    /// On Unix: path to the generated .sh askpass script (set as SSH_ASKPASS).
    /// On Windows: path to cli.exe (set as SSH_ASKPASS directly — no script needed).
    askpass_script_path: std::path::PathBuf,
    /// On Windows only: path to the Unix socket, passed as ZED_ASKPASS_SOCKET
    /// so cli.exe can find it without --askpass argument parsing.
    #[cfg(target_os = "windows")]
    askpass_socket_path: std::path::PathBuf,
}

impl PasswordProxy {
    pub async fn new(
        mut get_password: Box<
            dyn FnMut(String) -> Task<ControlFlow<(), Result<EncryptedPassword>>>
                + 'static
                + Send
                + Sync,
        >,
        executor: BackgroundExecutor,
    ) -> Result<Self> {
        let temp_dir = tempfile::Builder::new().prefix("zed-askpass").tempdir()?;
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let current_exec =
            std::env::current_exe().context("Failed to determine current zed executable path.")?;

        let askpass_program = ASKPASS_PROGRAM.get_or_init(|| current_exec);

        // Unix: SSH_ASKPASS = path to generated .sh script in temp dir.
        // Windows: SSH_ASKPASS = path to cli.exe directly. No script is written.
        #[cfg(not(target_os = "windows"))]
        let askpass_script_path = temp_dir.path().join(ASKPASS_SCRIPT_NAME);
        #[cfg(target_os = "windows")]
        let askpass_script_path = askpass_program.to_path_buf();

        let askpass_socket_path = askpass_socket.clone();

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

        // Unix only: write the shell script and mark it executable.
        // On Windows cli.exe is invoked directly, so no script is needed.
        #[cfg(not(target_os = "windows"))]
        {
            let askpass_script = generate_askpass_script(askpass_program, &askpass_socket_path)?;
            fs::write(&askpass_script_path, askpass_script)
                .await
                .with_context(|| format!("creating askpass script at {askpass_script_path:?}"))?;
            make_file_executable(&askpass_script_path)
                .await
                .with_context(|| {
                    format!("marking askpass script executable at {askpass_script_path:?}")
                })?;
        }

        Ok(Self {
            _task,
            askpass_script_path,
            #[cfg(target_os = "windows")]
            askpass_socket_path,
        })
    }

    pub fn script_path(&self) -> impl AsRef<OsStr> {
        &self.askpass_script_path
    }

    #[cfg(target_os = "windows")]
    pub fn socket_path(&self) -> impl AsRef<OsStr> {
        &self.askpass_socket_path
    }
}

/// Runs Zed in netcat mode for use in askpass.
pub fn main(socket: &str) {
    use std::io::{self, Read};
    use std::process::exit;

    let mut buffer = Vec::new();
    if let Err(err) = io::stdin().read_to_end(&mut buffer) {
        eprintln!("Error reading from stdin: {}", err);
        exit(1);
    }

    connect_and_write_prompt(socket, buffer)
}

/// Runs Zed in askpass mode using prompts passed as arguments.
pub fn main_from_args(socket: &str, args: impl IntoIterator<Item = String>) {
    let prompt = args.into_iter().collect::<Vec<_>>().join("\0");
    connect_and_write_prompt(socket, prompt.into_bytes())
}

fn connect_and_write_prompt(socket: &str, mut buffer: Vec<u8>) {
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

/// Generates the Unix shell askpass script.
/// Not used on Windows — cli.exe is invoked directly as SSH_ASKPASS.
#[cfg(not(target_os = "windows"))]
fn generate_askpass_script(
    askpass_program: &std::path::Path,
    askpass_socket: &std::path::Path,
) -> Result<String> {
    let shell_kind = ShellKind::Posix;
    let askpass_program = shell_kind.prepend_command_prefix(
        askpass_program
            .to_str()
            .context("Askpass program is on a non-utf8 path")?,
    );
    let askpass_program = shell_kind
        .try_quote_prefix_aware(&askpass_program)
        .context("Failed to shell-escape Askpass program path")?;
    let askpass_socket = askpass_socket
        .try_shell_safe(shell_kind)
        .context("Failed to shell-escape Askpass socket path")?;
    let print_args = "printf '%s\\0' \"$@\"";
    let shebang = "#!/bin/sh";
    Ok(format!(
        "{shebang}\n{print_args} | {askpass_program} --askpass={askpass_socket} 2> /dev/null \n",
    ))
}
