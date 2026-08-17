//! Utilities for connecting to ACP agents and proxies.
//!
//! This module provides [`AcpAgent`], a convenient component for launching an
//! ACP agent subprocess from an [`AcpAgentConfig`], command string, or JSON
//! configuration.

use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use async_process::Child;
use serde::{Deserialize, Serialize};
use std::pin::pin;

use crate::{Client, Conductor, Role};

type DebugCallback = Arc<dyn Fn(&str, LineDirection) + Send + Sync + 'static>;

const STDERR_CAPTURE_LIMIT: usize = 64 * 1024;
const STDERR_READ_BUFFER_SIZE: usize = 8 * 1024;
const STDERR_LINE_TRUNCATION_MARKER: &str = "… [stderr line truncated]";
const SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_secs(1);

/// Direction of a line being sent or received.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineDirection {
    /// Line being sent to the agent (stdin)
    Stdin,
    /// Line being received from the agent (stdout)
    Stdout,
    /// Line being received from the agent (stderr)
    Stderr,
}

/// Configuration for launching an ACP agent subprocess.
///
/// This is local SDK configuration, not an ACP wire-protocol type. It contains
/// only the values used to launch the child process.
///
/// ```
/// use agent_client_protocol::{AcpAgent, AcpAgentConfig};
///
/// let agent = AcpAgent::new(
///     AcpAgentConfig::new("python")
///         .arg("agent.py")
///         .env("RUST_LOG", "debug"),
/// );
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AcpAgentConfig {
    command: PathBuf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    env: BTreeMap<String, String>,
}

impl AcpAgentConfig {
    /// Create a configuration for the given executable or command name.
    #[must_use]
    pub fn new(command: impl Into<PathBuf>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
            env: BTreeMap::new(),
        }
    }

    /// Append one command-line argument.
    #[must_use]
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Append command-line arguments.
    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Set one environment variable for the child process.
    #[must_use]
    pub fn env(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(name.into(), value.into());
        self
    }

    /// Set environment variables for the child process.
    #[must_use]
    pub fn envs<I, K, V>(mut self, env: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.env.extend(
            env.into_iter()
                .map(|(name, value)| (name.into(), value.into())),
        );
        self
    }

    /// The executable path or command name.
    #[must_use]
    pub fn command(&self) -> &Path {
        &self.command
    }

    /// Command-line arguments passed to the executable.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.args
    }

    /// Environment variables set for the child process.
    #[must_use]
    pub fn environment(&self) -> &BTreeMap<String, String> {
        &self.env
    }
}

/// A component representing an external ACP agent running in a separate process.
///
/// `AcpAgent` implements the [`ConnectTo`](`crate::ConnectTo`) trait for spawning and communicating with
/// external agents or proxies via stdio. It handles process spawning, stream setup, and
/// byte stream serialization automatically. This is the primary way to connect to agents
/// that run as separate executables.
///
/// The launch configuration is independent from ACP wire-schema types and can
/// be parsed from command-line strings or JSON configurations.
/// On Unix, dropping an active connection terminates the spawned process group, including agents
/// started through wrapper commands such as `npx` and `uvx`.
///
/// # Use Cases
///
/// - **External agents**: Connect to agents written in any language (Python, Node.js, Rust, etc.)
/// - **Proxy chains**: Spawn intermediate proxies that transform or intercept messages
/// - **Conductor components**: Use with the conductor to build proxy chains
/// - **Subprocess isolation**: Run potentially untrusted code in a separate process
///
/// # Examples
///
/// Parse from a command string:
/// ```
/// # use agent_client_protocol::AcpAgent;
/// # use std::str::FromStr;
/// let agent = AcpAgent::from_str("python my_agent.py --verbose").unwrap();
/// ```
///
/// Parse from JSON:
/// ```
/// # use agent_client_protocol::AcpAgent;
/// # use std::str::FromStr;
/// let agent = AcpAgent::from_str(r#"{"command": "python", "args": ["my_agent.py"], "env": {"RUST_LOG": "info"}}"#).unwrap();
/// ```
pub struct AcpAgent {
    config: AcpAgentConfig,
    debug_callback: Option<DebugCallback>,
}

impl std::fmt::Debug for AcpAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AcpAgent")
            .field("config", &self.config)
            .field(
                "debug_callback",
                &self.debug_callback.as_ref().map(|_| "..."),
            )
            .finish()
    }
}

impl AcpAgent {
    /// Create an ACP agent from its process-launch configuration.
    #[must_use]
    pub fn new(config: AcpAgentConfig) -> Self {
        Self {
            config,
            debug_callback: None,
        }
    }

    /// Create an ACP agent for the Claude Agent adapter.
    /// Just runs `npx -y @agentclientprotocol/claude-agent-acp@latest`.
    #[must_use]
    pub fn claude_agent() -> Self {
        Self::from_str("npx -y @agentclientprotocol/claude-agent-acp@latest")
            .expect("valid bash command")
    }

    /// Create an ACP agent for the Codex adapter.
    /// Just runs `npx -y @agentclientprotocol/codex-acp@latest`.
    #[must_use]
    pub fn codex() -> Self {
        Self::from_str("npx -y @agentclientprotocol/codex-acp@latest").expect("valid bash command")
    }

    /// Get the process-launch configuration.
    #[must_use]
    pub fn config(&self) -> &AcpAgentConfig {
        &self.config
    }

    /// Convert into the process-launch configuration.
    #[must_use]
    pub fn into_config(self) -> AcpAgentConfig {
        self.config
    }

    /// Add a debug callback that will be invoked for each line sent/received.
    ///
    /// The callback receives the line content and the direction (stdin/stdout/stderr).
    /// This is useful for logging, debugging, or monitoring agent communication.
    /// Exceptionally long stderr lines are truncated to keep memory usage bounded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use agent_client_protocol::{AcpAgent, LineDirection};
    /// # use std::str::FromStr;
    /// let agent = AcpAgent::from_str("python my_agent.py")
    ///     .unwrap()
    ///     .with_debug(|line, direction| {
    ///         eprintln!("{:?}: {}", direction, line);
    ///     });
    /// ```
    #[must_use]
    pub fn with_debug<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, LineDirection) + Send + Sync + 'static,
    {
        self.debug_callback = Some(Arc::new(callback));
        self
    }

    /// Spawn the configured process and return its stdio streams and raw child handle.
    ///
    /// This is a low-level escape hatch. The caller owns the returned child process and is
    /// responsible for terminating it. Connections created through [`crate::ConnectTo`] instead
    /// install a guard that tears down the spawned process group on Unix.
    pub fn spawn_process(
        &self,
    ) -> Result<
        (
            async_process::ChildStdin,
            async_process::ChildStdout,
            async_process::ChildStderr,
            Child,
        ),
        crate::Error,
    > {
        let mut std_cmd = std::process::Command::new(&self.config.command);
        std_cmd.args(&self.config.args);
        std_cmd.envs(&self.config.env);
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt as _;

            // Make the child the leader of its own process group so
            // `ChildGuard` can terminate the entire process tree.
            // Agents are commonly distributed behind wrapper launchers
            // (`npx …`, `uvx …`): killing only the immediate child
            // orphans the real agent, which re-parents to pid 1 and
            // does not reliably exit on stdin EOF.
            std_cmd.process_group(0);
        }
        let mut cmd = async_process::Command::from(std_cmd);
        #[cfg(windows)]
        {
            use async_process::windows::CommandExt as _;

            cmd.creation_flags(windows_sys::Win32::System::Threading::CREATE_NO_WINDOW);
        }
        cmd.stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn().map_err(crate::Error::into_internal_error)?;

        let child_stdin = child
            .stdin
            .take()
            .ok_or_else(|| crate::util::internal_error("Failed to open stdin"))?;
        let child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| crate::util::internal_error("Failed to open stdout"))?;
        let child_stderr = child
            .stderr
            .take()
            .ok_or_else(|| crate::util::internal_error("Failed to open stderr"))?;

        Ok((child_stdin, child_stdout, child_stderr, child))
    }
}

/// A wrapper around Child that kills the process — and, on unix, its whole
/// process group (see `spawn_process`) — when dropped.
struct ChildGuard(Child);

impl ChildGuard {
    async fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.0.status().await
    }

    fn terminate(&mut self) {
        // SIGKILL the child's process group first: the child was spawned as
        // its own group leader, so this reaches grandchildren spawned by
        // wrapper launchers (`npx → node`, `uvx → python`). This also covers
        // the case where the direct child already exited but its wrapper left
        // the real agent running. An error (e.g. `ESRCH`) just means the
        // group is already gone.
        #[cfg(unix)]
        if let Some(pid) = rustix::process::Pid::from_raw(self.0.id().cast_signed()) {
            let _result = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
        }
        // Fallback for platforms without group semantics (and a no-op double
        // tap on unix).
        drop(self.0.kill());
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        self.terminate();
    }
}

#[derive(Default)]
struct StderrTail {
    bytes: VecDeque<u8>,
    truncated: bool,
}

impl StderrTail {
    fn push(&mut self, bytes: &[u8]) {
        if bytes.len() >= STDERR_CAPTURE_LIMIT {
            self.truncated |= !self.bytes.is_empty() || bytes.len() > STDERR_CAPTURE_LIMIT;
            self.bytes.clear();
            self.bytes
                .extend(bytes[bytes.len() - STDERR_CAPTURE_LIMIT..].iter().copied());
            return;
        }

        let overflow = self
            .bytes
            .len()
            .saturating_add(bytes.len())
            .saturating_sub(STDERR_CAPTURE_LIMIT);
        if overflow > 0 {
            self.truncated = true;
            drop(self.bytes.drain(..overflow));
        }
        self.bytes.extend(bytes.iter().copied());
    }

    fn into_string(mut self) -> String {
        let truncated = self.truncated;
        let stderr = String::from_utf8_lossy(self.bytes.make_contiguous());
        if truncated {
            format!("[stderr truncated; showing last {STDERR_CAPTURE_LIMIT} bytes]\n{stderr}")
        } else {
            stderr.into_owned()
        }
    }
}

#[derive(Default)]
struct StderrDebugLines {
    current: Vec<u8>,
    truncated: bool,
    pending_carriage_return: bool,
}

impl StderrDebugLines {
    fn push(&mut self, bytes: &[u8], callback: &DebugCallback) {
        for &byte in bytes {
            if self.pending_carriage_return {
                if byte == b'\n' {
                    self.pending_carriage_return = false;
                    self.emit(callback);
                    continue;
                }

                self.push_byte(b'\r');
                self.pending_carriage_return = false;
            }

            match byte {
                b'\r' => self.pending_carriage_return = true,
                b'\n' => self.emit(callback),
                byte => self.push_byte(byte),
            }
        }
    }

    fn finish(&mut self, callback: &DebugCallback) {
        if self.pending_carriage_return {
            self.push_byte(b'\r');
            self.pending_carriage_return = false;
        }
        if !self.current.is_empty() || self.truncated {
            self.emit(callback);
        }
    }

    fn push_byte(&mut self, byte: u8) {
        if self.current.len() < STDERR_CAPTURE_LIMIT {
            self.current.push(byte);
        } else {
            self.truncated = true;
        }
    }

    fn emit(&mut self, callback: &DebugCallback) {
        let line = String::from_utf8_lossy(&self.current);

        if self.truncated {
            let mut line = line.into_owned();
            line.push_str(STDERR_LINE_TRUNCATION_MARKER);
            callback(&line, LineDirection::Stderr);
        } else {
            callback(line.as_ref(), LineDirection::Stderr);
        }

        self.current.clear();
        self.truncated = false;
    }
}

struct StderrDrainResult {
    captured: String,
    read_error: Option<std::io::Error>,
}

async fn drain_stderr(
    mut stderr: impl futures::AsyncRead + Unpin,
    debug_callback: Option<DebugCallback>,
) -> StderrDrainResult {
    use futures::AsyncReadExt as _;

    let mut tail = StderrTail::default();
    let mut debug_lines = debug_callback.as_ref().map(|_| StderrDebugLines::default());
    let mut buffer = [0; STDERR_READ_BUFFER_SIZE];

    let read_error = loop {
        match stderr.read(&mut buffer).await {
            Ok(0) => break None,
            Ok(read) => {
                let bytes = &buffer[..read];
                tail.push(bytes);
                if let (Some(lines), Some(callback)) =
                    (debug_lines.as_mut(), debug_callback.as_ref())
                {
                    lines.push(bytes, callback);
                }
            }
            Err(error) => break Some(error),
        }
    };

    if let (Some(lines), Some(callback)) = (debug_lines.as_mut(), debug_callback.as_ref()) {
        lines.finish(callback);
    }

    StderrDrainResult {
        captured: tail.into_string(),
        read_error,
    }
}

struct ExitedChild {
    guard: ChildGuard,
    status: std::process::ExitStatus,
    stderr_rx: futures::channel::oneshot::Receiver<String>,
}

/// Waits for the direct child process while retaining its process-group guard
/// and stderr receiver for exit reporting.
async fn wait_for_child(
    mut guard: ChildGuard,
    stderr_rx: futures::channel::oneshot::Receiver<String>,
) -> Result<ExitedChild, crate::Error> {
    let status = guard
        .wait()
        .await
        .map_err(|e| crate::util::internal_error(format!("Failed to wait for process: {e}")))?;

    Ok(ExitedChild {
        guard,
        status,
        stderr_rx,
    })
}

/// Reports an observed child exit, including a bounded stderr tail for a
/// nonzero status.
async fn finish_child_exit(child: ExitedChild) -> Result<(), crate::Error> {
    let ExitedChild {
        mut guard,
        status,
        stderr_rx,
    } = child;

    // A launcher may exit while a descendant remains alive holding inherited
    // stdio. Terminate the rest of the group before waiting for stderr EOF.
    guard.terminate();

    if status.success() {
        Ok(())
    } else {
        let stderr =
            match futures::future::select(stderr_rx, async_io::Timer::after(SHUTDOWN_GRACE_PERIOD))
                .await
            {
                futures::future::Either::Left((stderr, _)) => stderr.unwrap_or_default(),
                futures::future::Either::Right((_, stderr_rx)) => {
                    tracing::debug!(
                        grace = ?SHUTDOWN_GRACE_PERIOD,
                        "Agent stderr remained open after process exit; reporting status without it"
                    );
                    drop(stderr_rx);
                    String::new()
                }
            };

        let message = if stderr.is_empty() {
            format!("Process exited with {status}")
        } else {
            format!("Process exited with {status}: {stderr}")
        };

        Err(crate::util::internal_error(message))
    }
}

async fn await_protocol_shutdown_after_successful_child_exit<F>(
    protocol_future: F,
    grace: Duration,
) -> Result<(), crate::Error>
where
    F: std::future::Future<Output = Result<(), crate::Error>> + Unpin,
{
    match futures::future::select(protocol_future, async_io::Timer::after(grace)).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right((_, protocol_future)) => {
            tracing::debug!(
                ?grace,
                "Protocol transport remained open after successful agent process exit; stopping it"
            );
            drop(protocol_future);
            Ok(())
        }
    }
}

async fn write_line_with_shutdown_timeout<W>(
    writer: &mut W,
    line: String,
    stdout_eof_rx: &mut Option<futures::channel::oneshot::Receiver<()>>,
    stdout_eof_seen: &mut bool,
    grace: Duration,
) -> std::io::Result<()>
where
    W: futures::AsyncWrite + Unpin + ?Sized,
{
    let write = Box::pin(crate::jsonrpc::write_line(writer, line));

    if *stdout_eof_seen {
        return await_write_during_shutdown(write, grace).await;
    }

    let Some(stdout_eof) = stdout_eof_rx.as_mut() else {
        return write.await;
    };

    match futures::future::select(write, stdout_eof).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right((stdout_eof, write)) => {
            *stdout_eof_rx = None;
            if stdout_eof.is_err() {
                // Dropping the incoming stream cancels the signal. Only an
                // explicit send represents a clean EOF.
                return write.await;
            }

            *stdout_eof_seen = true;
            await_write_during_shutdown(write, grace).await
        }
    }
}

async fn await_write_during_shutdown<F>(write: F, grace: Duration) -> std::io::Result<()>
where
    F: std::future::Future<Output = std::io::Result<()>> + Unpin,
{
    match futures::future::select(write, async_io::Timer::after(grace)).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right((_, write)) => {
            tracing::debug!(
                ?grace,
                "Pending protocol output did not drain after agent stdout closed"
            );
            drop(write);
            Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                format!(
                    "Agent closed its protocol output but pending protocol output did not drain within {grace:?}"
                ),
            ))
        }
    }
}

/// Roles that an ACP agent executable can potentially serve.
pub trait AcpAgentCounterpartRole: Role {}

impl AcpAgentCounterpartRole for Client {}

impl AcpAgentCounterpartRole for Conductor {}

impl<Counterpart: AcpAgentCounterpartRole> crate::ConnectTo<Counterpart> for AcpAgent {
    async fn connect_to(
        self,
        client: impl crate::ConnectTo<Counterpart::Counterpart>,
    ) -> Result<(), crate::Error> {
        use futures::io::BufReader;
        use futures::{AsyncBufReadExt, StreamExt};

        let (child_stdin, child_stdout, child_stderr, child) = self.spawn_process()?;

        // Create a channel to collect stderr for error reporting
        let (stderr_tx, stderr_rx) = futures::channel::oneshot::channel::<String>();

        // Read stderr concurrently, optionally calling the debug callback.
        // We use futures::future::select below to race this against the protocol,
        // so this runs as part of the same task — no tokio::spawn needed.
        let debug_callback = self.debug_callback.clone();
        let stderr_future = async move {
            let StderrDrainResult {
                captured,
                read_error,
            } = drain_stderr(child_stderr, debug_callback).await;
            drop(stderr_tx.send(captured));

            if let Some(error) = read_error {
                tracing::warn!(
                    ?error,
                    "Failed to read process stderr; stderr will no longer be captured"
                );
            }
        };

        // Create the guard eagerly so cancelling this connection before the
        // monitor is first polled still terminates the whole process group.
        let child_wait = wait_for_child(ChildGuard(child), stderr_rx);

        // Convert stdio to line streams with optional debug inspection.
        let incoming_lines: std::pin::Pin<
            Box<dyn futures::Stream<Item = std::io::Result<String>> + Send>,
        > = if let Some(callback) = self.debug_callback.clone() {
            Box::pin(BufReader::new(child_stdout).lines().inspect(move |result| {
                if let Ok(line) = result {
                    callback(line, LineDirection::Stdout);
                }
            }))
        } else {
            Box::pin(BufReader::new(child_stdout).lines())
        };

        // The JSON-RPC transport keeps polling stdout while it drains stdin.
        // Signal physical EOF so a child that half-closes stdout and stops
        // reading cannot hold a final write open forever. Dropping this stream
        // merely cancels the signal and is not treated as EOF.
        let (stdout_eof_tx, stdout_eof_rx) = futures::channel::oneshot::channel();
        let mut stdout_eof_tx = Some(stdout_eof_tx);
        let mut incoming_lines = incoming_lines;
        let incoming_lines = Box::pin(futures::stream::poll_fn(move |cx| {
            let next = incoming_lines.as_mut().poll_next(cx);
            if matches!(next, std::task::Poll::Ready(None))
                && let Some(stdout_eof_tx) = stdout_eof_tx.take()
            {
                let _ = stdout_eof_tx.send(());
            }
            next
        }));

        // Create a sink that writes lines (with newlines) to stdin with optional debug logging
        let outgoing_sink: std::pin::Pin<
            Box<dyn futures::Sink<String, Error = std::io::Error> + Send>,
        > = Box::pin(futures::sink::unfold(
            (
                child_stdin,
                self.debug_callback.clone(),
                Some(stdout_eof_rx),
                false,
            ),
            async move |(mut writer, callback, mut stdout_eof_rx, mut stdout_eof_seen),
                        line: String| {
                if let Some(callback) = callback.as_ref() {
                    callback(&line, LineDirection::Stdin);
                }
                write_line_with_shutdown_timeout(
                    &mut writer,
                    line,
                    &mut stdout_eof_rx,
                    &mut stdout_eof_seen,
                    SHUTDOWN_GRACE_PERIOD,
                )
                .await?;
                Ok::<_, std::io::Error>((writer, callback, stdout_eof_rx, stdout_eof_seen))
            },
        ));

        // Race the protocol against child process exit.
        // Also run stderr collection concurrently.
        let protocol_future = crate::ConnectTo::<Counterpart>::connect_to(
            crate::Lines::new(outgoing_sink, incoming_lines),
            client,
        );

        let stderr_future = pin!(stderr_future);
        let protocol_future = Box::pin(protocol_future);
        let child_wait = Box::pin(child_wait);

        // Run stderr reader alongside the main race. Errors still stop the
        // connection immediately. After protocol shutdown succeeds, give the
        // child a bounded grace period so delayed failures remain observable
        // without letting a non-exiting launcher hang shutdown forever.
        let main_race = async {
            match futures::future::select(protocol_future, child_wait).await {
                futures::future::Either::Left((result, child_wait)) => {
                    result?;
                    match futures::future::select(
                        child_wait,
                        async_io::Timer::after(SHUTDOWN_GRACE_PERIOD),
                    )
                    .await
                    {
                        futures::future::Either::Left((child, _)) => {
                            finish_child_exit(child?).await
                        }
                        futures::future::Either::Right((_, child_wait)) => {
                            tracing::debug!(
                                grace = ?SHUTDOWN_GRACE_PERIOD,
                                "Agent process did not exit after protocol shutdown; terminating it"
                            );
                            drop(child_wait);
                            Ok(())
                        }
                    }
                }
                futures::future::Either::Right((child, protocol_future)) => {
                    finish_child_exit(child?).await?;
                    await_protocol_shutdown_after_successful_child_exit(
                        protocol_future,
                        SHUTDOWN_GRACE_PERIOD,
                    )
                    .await
                }
            }
        };

        // Run stderr collection concurrently with the main logic.
        // When main_race completes, we don't need stderr anymore.
        let main_race = pin!(main_race);
        match futures::future::select(main_race, stderr_future).await {
            futures::future::Either::Left((result, _)) => result,
            futures::future::Either::Right(((), main_race)) => main_race.await,
        }
    }
}

impl AcpAgent {
    /// Create an `AcpAgent` from an iterator of command-line arguments.
    ///
    /// Leading arguments of the form `NAME=value` are parsed as environment variables.
    /// The first non-env argument is the command, and the rest are arguments.
    ///
    /// # Example
    ///
    /// ```
    /// # use agent_client_protocol::AcpAgent;
    /// let agent = AcpAgent::from_args([
    ///     "RUST_LOG=debug",
    ///     "cargo",
    ///     "run",
    ///     "-p",
    ///     "my-crate",
    /// ]).unwrap();
    /// ```
    pub fn from_args<I, T>(args: I) -> Result<Self, crate::Error>
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();

        if args.is_empty() {
            return Err(crate::util::internal_error("Arguments cannot be empty"));
        }

        let mut env = BTreeMap::new();
        let mut command_idx = 0;

        for (i, arg) in args.iter().enumerate() {
            if let Some((name, value)) = parse_env_var(arg) {
                env.insert(name, value);
                command_idx = i + 1;
            } else {
                break;
            }
        }

        if command_idx >= args.len() {
            return Err(crate::util::internal_error(
                "No command found (only environment variables provided)",
            ));
        }

        let command = PathBuf::from(&args[command_idx]);
        let cmd_args = args[command_idx + 1..].to_vec();

        Ok(Self::new(
            AcpAgentConfig::new(command).args(cmd_args).envs(env),
        ))
    }
}

/// Parse a string as an environment variable assignment (NAME=value).
fn parse_env_var(s: &str) -> Option<(String, String)> {
    let eq_pos = s.find('=')?;
    if eq_pos == 0 {
        return None;
    }

    let name = &s[..eq_pos];
    let value = &s[eq_pos + 1..];

    let mut chars = name.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() && first != '_' {
        return None;
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }

    Some((name.to_string(), value.to_string()))
}

impl FromStr for AcpAgent {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed.starts_with('{') {
            let config = serde_json::from_str(trimmed)
                .map_err(|e| crate::util::internal_error(format!("Failed to parse JSON: {e}")))?;
            return Ok(Self::new(config));
        }

        let parts = shell_words::split(trimmed)
            .map_err(|e| crate::util::internal_error(format!("Failed to parse command: {e}")))?;

        Self::from_args(parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn recording_debug_callback() -> (DebugCallback, Arc<Mutex<Vec<String>>>) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let recorded = lines.clone();
        let callback = Arc::new(move |line: &str, direction| {
            assert_eq!(direction, LineDirection::Stderr);
            recorded.lock().unwrap().push(line.to_owned());
        });
        (callback, lines)
    }

    #[test]
    fn stderr_tail_keeps_last_bytes() {
        let initial = vec![b'a'; STDERR_CAPTURE_LIMIT];

        let mut exact = StderrTail::default();
        exact.push(&initial);
        assert_eq!(exact.into_string(), String::from_utf8(initial).unwrap());

        let mut truncated = StderrTail::default();
        truncated.push(&vec![b'a'; STDERR_CAPTURE_LIMIT]);
        truncated.push(b"the end");
        let captured = truncated.into_string();
        let (notice, tail) = captured.split_once('\n').unwrap();
        assert_eq!(
            notice,
            format!("[stderr truncated; showing last {STDERR_CAPTURE_LIMIT} bytes]")
        );
        assert_eq!(tail.len(), STDERR_CAPTURE_LIMIT);
        assert!(tail.ends_with("the end"));
    }

    #[test]
    fn stderr_debug_callback_preserves_lines() {
        let (callback, recorded) = recording_debug_callback();
        let mut lines = StderrDebugLines::default();

        lines.push(b"one\r", &callback);
        lines.push(b"\n\ntw", &callback);
        lines.push(b"o\nbad\xff\nlast\r", &callback);
        lines.finish(&callback);

        assert_eq!(
            *recorded.lock().unwrap(),
            ["one", "", "two", "bad\u{fffd}", "last\r"]
        );
    }

    #[test]
    fn stderr_debug_callback_truncates_oversized_lines() {
        let (callback, recorded) = recording_debug_callback();
        let mut lines = StderrDebugLines::default();
        let exact = vec![b'y'; STDERR_CAPTURE_LIMIT];
        let oversized = vec![b'x'; STDERR_CAPTURE_LIMIT + 1];

        lines.push(&exact, &callback);
        lines.push(b"\r\n", &callback);
        lines.push(&oversized, &callback);
        assert_eq!(lines.current.len(), STDERR_CAPTURE_LIMIT);
        assert!(lines.truncated);
        lines.push(b"\nnext\n", &callback);

        let recorded = recorded.lock().unwrap();
        assert_eq!(recorded.len(), 3);
        assert_eq!(recorded[0].len(), STDERR_CAPTURE_LIMIT);
        assert!(!recorded[0].ends_with(STDERR_LINE_TRUNCATION_MARKER));
        assert_eq!(
            recorded[1].len(),
            STDERR_CAPTURE_LIMIT + STDERR_LINE_TRUNCATION_MARKER.len()
        );
        assert!(recorded[1].ends_with(STDERR_LINE_TRUNCATION_MARKER));
        assert_eq!(recorded[2], "next");
    }

    struct ErrorAfterData {
        polls: Arc<AtomicUsize>,
    }

    impl futures::AsyncRead for ErrorAfterData {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buffer: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            match self.polls.fetch_add(1, Ordering::SeqCst) {
                0 => {
                    buffer[..7].copy_from_slice(b"partial");
                    std::task::Poll::Ready(Ok(7))
                }
                1 => std::task::Poll::Ready(Err(std::io::Error::other("read failed"))),
                _ => panic!("stderr reader was polled again after an error"),
            }
        }
    }

    #[tokio::test]
    async fn stderr_drain_stops_after_read_error() {
        let polls = Arc::new(AtomicUsize::new(0));
        let (callback, recorded) = recording_debug_callback();

        let result = drain_stderr(
            ErrorAfterData {
                polls: polls.clone(),
            },
            Some(callback),
        )
        .await;

        assert_eq!(result.captured, "partial");
        assert_eq!(result.read_error.unwrap().to_string(), "read failed");
        assert_eq!(polls.load(Ordering::SeqCst), 2);
        assert_eq!(*recorded.lock().unwrap(), ["partial"]);
    }

    #[tokio::test]
    async fn successful_child_exit_bounds_protocol_shutdown_cleanly() {
        let grace = std::time::Duration::from_millis(10);
        tokio::time::timeout(
            std::time::Duration::from_secs(1),
            await_protocol_shutdown_after_successful_child_exit(
                futures::future::pending::<Result<(), crate::Error>>(),
                grace,
            ),
        )
        .await
        .expect("protocol shutdown wait should be bounded")
        .expect("a successful child exit should stop the pending protocol cleanly");
    }

    #[tokio::test]
    async fn successful_child_exit_preserves_ready_protocol_error() {
        let error = await_protocol_shutdown_after_successful_child_exit(
            futures::future::ready(Err(crate::util::internal_error(
                "protocol failed during shutdown",
            ))),
            std::time::Duration::from_secs(1),
        )
        .await
        .expect_err("a ready protocol error should remain authoritative");
        let detail = error
            .data
            .as_ref()
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();

        assert!(
            detail.contains("protocol failed during shutdown"),
            "unexpected protocol error: {error:?}"
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn large_unterminated_stderr_is_fully_drained() {
        let agent = AcpAgent::from_args([
            "/bin/sh",
            "-c",
            r#"i=0; while [ "$i" -lt 4096 ]; do printf '%01024d' 0; i=$((i + 1)); done >&2; printf ACP_END >&2; exit 17"#,
        ])
        .unwrap();
        let (child_stdin, child_stdout, child_stderr, child) = agent.spawn_process().unwrap();
        drop(child_stdin);
        drop(child_stdout);
        let mut guard = ChildGuard(child);

        let (drained, status) = tokio::time::timeout(std::time::Duration::from_secs(10), async {
            futures::join!(drain_stderr(child_stderr, None), guard.wait())
        })
        .await
        .expect("stderr drain should not block after its retained tail is full");

        assert_eq!(status.unwrap().code(), Some(17));
        assert!(drained.read_error.is_none());
        let (notice, tail) = drained.captured.split_once('\n').unwrap();
        assert_eq!(
            notice,
            format!("[stderr truncated; showing last {STDERR_CAPTURE_LIMIT} bytes]")
        );
        assert_eq!(tail.len(), STDERR_CAPTURE_LIMIT);
        assert!(tail.ends_with("ACP_END"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn protocol_eof_still_reports_nonzero_child_exit() {
        let agent = AcpAgent::from_args([
            "/bin/sh",
            "-c",
            "exec 1>&-; cat >/dev/null; printf ACP_TEST_FAILURE_AFTER_STDOUT_EOF >&2; exit 17",
        ])
        .unwrap();

        let error = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            Client.builder().connect_to(agent),
        )
        .await
        .expect("connection should finish after the child exits")
        .expect_err("nonzero child exit after protocol EOF should be reported");
        let detail = error
            .data
            .as_ref()
            .map(serde_json::Value::to_string)
            .unwrap_or_default();

        assert!(
            detail.contains("exit status: 17"),
            "child exit status should be preserved: {error:?}"
        );
        assert!(
            detail.contains("ACP_TEST_FAILURE_AFTER_STDOUT_EOF"),
            "child stderr should be preserved: {error:?}"
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn successful_child_exit_does_not_cancel_active_foreground() {
        let agent = AcpAgent::from_args(["/bin/sh", "-c", "exit 0"]).unwrap();
        let (started_tx, started_rx) = futures::channel::oneshot::channel();
        let (closed_tx, closed_rx) = futures::channel::oneshot::channel();
        let (close_release_tx, close_release_rx) = futures::channel::oneshot::channel();
        let (release_tx, release_rx) = futures::channel::oneshot::channel();
        let connection = tokio::spawn(
            Client
                .builder()
                .on_close(async move |_cx| {
                    closed_tx.send(()).map_err(|()| {
                        crate::Error::internal_error().data("close observer dropped")
                    })?;
                    close_release_rx.await.map_err(|_| {
                        crate::Error::internal_error().data("close callback release dropped")
                    })
                })
                .connect_with(agent, async move |_cx| {
                    started_tx.send(()).map_err(|()| {
                        crate::Error::internal_error().data("foreground observer dropped")
                    })?;
                    release_rx.await.map_err(|_| {
                        crate::Error::internal_error().data("foreground release dropped")
                    })
                }),
        );

        tokio::time::timeout(std::time::Duration::from_secs(5), started_rx)
            .await
            .expect("foreground should start")
            .expect("foreground should report that it started");

        tokio::time::timeout(std::time::Duration::from_secs(5), closed_rx)
            .await
            .expect("successful child exit should close the protocol transport")
            .expect("successful child exit should invoke close callbacks");

        tokio::time::sleep(SHUTDOWN_GRACE_PERIOD + std::time::Duration::from_millis(250)).await;
        assert!(
            !connection.is_finished(),
            "successful child exit canceled active cleanup"
        );

        close_release_tx
            .send(())
            .expect("clean child exit should preserve close callbacks");
        release_tx
            .send(())
            .expect("clean child exit should preserve the foreground");
        tokio::time::timeout(std::time::Duration::from_secs(5), connection)
            .await
            .expect("released foreground should finish")
            .expect("connection task should not panic")
            .expect("successful child exit should remain a clean EOF");
    }

    #[cfg(unix)]
    struct KillOnDrop(Option<rustix::process::Pid>);

    #[cfg(unix)]
    impl KillOnDrop {
        fn disarm(&mut self) {
            self.0 = None;
        }
    }

    #[cfg(unix)]
    impl Drop for KillOnDrop {
        fn drop(&mut self) {
            if let Some(pid) = self.0 {
                let _result = rustix::process::kill_process(pid, rustix::process::Signal::KILL);
            }
        }
    }

    #[cfg(unix)]
    fn wrapper_agent(script: &str) -> (AcpAgent, tokio::sync::mpsc::UnboundedReceiver<String>) {
        let (pid_tx, pid_rx) = tokio::sync::mpsc::unbounded_channel();
        let agent = AcpAgent::from_args(["/bin/sh", "-c", script])
            .unwrap()
            .with_debug(move |line, direction| {
                if direction == LineDirection::Stderr {
                    drop(pid_tx.send(line.to_owned()));
                }
            });
        (agent, pid_rx)
    }

    #[cfg(unix)]
    fn process_is_running(pid: rustix::process::Pid) -> bool {
        if rustix::process::test_kill_process(pid).is_err() {
            return false;
        }

        // A killed orphan can remain as a zombie under a container PID 1 that
        // does not reap promptly. Treat zombies as exited for this test.
        match std::process::Command::new("ps")
            .args(["-o", "stat=", "-p", &pid.to_string()])
            .output()
        {
            Ok(output) if output.status.success() => {
                let state = String::from_utf8_lossy(&output.stdout);
                !state.trim().is_empty() && !state.trim_start().starts_with('Z')
            }
            Ok(_) => false,
            Err(_) => true,
        }
    }

    #[cfg(unix)]
    async fn reported_descendant_pid(
        connection: &mut futures::future::BoxFuture<'static, Result<(), crate::Error>>,
        pid_rx: &mut tokio::sync::mpsc::UnboundedReceiver<String>,
    ) -> rustix::process::Pid {
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                tokio::select! {
                    biased;
                    line = pid_rx.recv() => {
                        let line = line.expect("wrapper stderr should remain open");
                        if let Some(pid) = line.strip_prefix("ACP_TEST_CHILD_PID=") {
                            let pid = pid.parse::<i32>().expect("valid descendant PID");
                            break rustix::process::Pid::from_raw(pid)
                                .expect("nonzero descendant PID");
                        }
                    }
                    result = &mut *connection => {
                        panic!("agent connection exited before reporting descendant PID: {result:?}");
                    }
                }
            }
        })
        .await
        .expect("wrapper should report descendant PID")
    }

    #[cfg(unix)]
    async fn assert_process_exits(pid: rustix::process::Pid) {
        let exited = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while process_is_running(pid) {
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            }
        })
        .await
        .is_ok();
        assert!(exited, "descendant process {pid} remained alive");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn protocol_eof_terminates_a_child_that_does_not_exit() {
        let (agent, mut pid_rx) =
            wrapper_agent("echo ACP_TEST_CHILD_PID=$$ >&2; exec 1>&-; while :; do sleep 30; done");
        let mut connection: futures::future::BoxFuture<'static, Result<(), crate::Error>> =
            Box::pin(Client.builder().connect_to(agent));
        let child_pid = reported_descendant_pid(&mut connection, &mut pid_rx).await;
        let mut cleanup = KillOnDrop(Some(child_pid));

        assert!(process_is_running(child_pid));
        tokio::time::timeout(std::time::Duration::from_secs(5), &mut connection)
            .await
            .expect("protocol shutdown should bound its child-exit wait")
            .expect("clean protocol shutdown should terminate a non-exiting child");
        assert_process_exits(child_pid).await;
        cleanup.disarm();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn protocol_eof_bounds_a_blocked_outgoing_drain() {
        let (agent, mut pid_rx) = wrapper_agent(
            "echo ACP_TEST_CHILD_PID=$$ >&2; exec 1>&-; sleep 30 & child=$!; wait \"$child\"",
        );
        let (channel, mut connection) = crate::ConnectTo::<Client>::into_channel_and_future(agent);
        let crate::Channel {
            rx: _incoming,
            tx: outgoing,
        } = channel;

        let response = crate::RawJsonRpcMessage::response(
            crate::schema::v1::RequestId::Number(1),
            Ok(serde_json::json!({ "payload": "x".repeat(4 * 1024 * 1024) })),
        );
        outgoing
            .unbounded_send(crate::TransportFrame::Single(response))
            .expect("response should be accepted before the connection starts");
        outgoing.close_channel();

        let child_pid = reported_descendant_pid(&mut connection, &mut pid_rx).await;
        let mut cleanup = KillOnDrop(Some(child_pid));

        let error = tokio::time::timeout(std::time::Duration::from_secs(5), &mut connection)
            .await
            .expect("stdout EOF should bound a blocked outgoing drain")
            .expect_err("an undelivered accepted response must not report success");
        let detail = error
            .data
            .as_ref()
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        assert!(
            detail.contains("pending protocol output did not drain"),
            "the error should identify the blocked outgoing drain: {error:?}"
        );

        assert_process_exits(child_pid).await;
        cleanup.disarm();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_connection_drop_kills_wrapper_descendant() {
        let (agent, mut pid_rx) = wrapper_agent(
            "sleep 30 & child=$!; echo ACP_TEST_CHILD_PID=$child >&2; wait \"$child\"",
        );
        let (_channel, mut connection) = crate::ConnectTo::<Client>::into_channel_and_future(agent);
        let descendant_pid = reported_descendant_pid(&mut connection, &mut pid_rx).await;
        let mut cleanup = KillOnDrop(Some(descendant_pid));

        assert!(process_is_running(descendant_pid));
        drop(connection);
        assert_process_exits(descendant_pid).await;
        cleanup.disarm();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_launcher_exit_kills_descendant_before_stderr_wait() {
        let (agent, mut pid_rx) = wrapper_agent(
            "sh -c 'trap \"\" HUP; exec sleep 30' >/dev/null & child=$!; echo ACP_TEST_CHILD_PID=$child >&2; exit 17",
        );
        let (_channel, mut connection) = crate::ConnectTo::<Client>::into_channel_and_future(agent);
        let descendant_pid = reported_descendant_pid(&mut connection, &mut pid_rx).await;
        let mut cleanup = KillOnDrop(Some(descendant_pid));

        let result = tokio::time::timeout(std::time::Duration::from_secs(5), &mut connection)
            .await
            .expect("connection should observe the launcher exit");
        let error = result.expect_err("nonzero launcher exit should be an error");
        let detail = error
            .data
            .as_ref()
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        assert!(
            detail.contains("ACP_TEST_CHILD_PID="),
            "launcher stderr should be preserved: {error:?}"
        );
        assert_process_exits(descendant_pid).await;
        cleanup.disarm();
    }

    #[test]
    fn test_parse_simple_command() {
        let agent = AcpAgent::from_str("python agent.py").unwrap();
        let config = agent.config();
        assert_eq!(config.command(), Path::new("python"));
        assert_eq!(config.arguments(), ["agent.py"]);
        assert!(config.environment().is_empty());
    }

    #[test]
    fn test_parse_environment_from_args() {
        let agent =
            AcpAgent::from_args(["RUST_LOG=debug", "NO_COLOR=1", "python", "agent.py"]).unwrap();
        let config = agent.config();

        assert_eq!(config.command(), Path::new("python"));
        assert_eq!(config.arguments(), ["agent.py"]);
        assert_eq!(
            config.environment(),
            &BTreeMap::from([
                ("NO_COLOR".to_owned(), "1".to_owned()),
                ("RUST_LOG".to_owned(), "debug".to_owned()),
            ])
        );
    }

    #[test]
    fn test_new_accepts_agent_configuration() {
        let config = AcpAgentConfig::new("/usr/bin/agent")
            .arg("--verbose")
            .env("RUST_LOG", "debug");
        let agent = AcpAgent::new(config.clone());

        assert_eq!(agent.into_config(), config);
    }

    #[test]
    fn test_parse_command_with_args() {
        let agent = AcpAgent::from_str("node server.js --port 8080 --verbose").unwrap();
        let config = agent.config();
        assert_eq!(config.command(), Path::new("node"));
        assert_eq!(
            config.arguments(),
            ["server.js", "--port", "8080", "--verbose"]
        );
        assert!(config.environment().is_empty());
    }

    #[test]
    fn test_parse_command_with_quotes() {
        let agent = AcpAgent::from_str(r#"python "my agent.py" --name "Test Agent""#).unwrap();
        let config = agent.into_config();
        assert_eq!(config.command(), Path::new("python"));
        assert_eq!(config.arguments(), ["my agent.py", "--name", "Test Agent"]);
        assert!(config.environment().is_empty());
    }

    #[test]
    fn test_parse_json_config() {
        let json = r#"{
            "command": "/usr/bin/python",
            "args": ["agent.py", "--verbose"],
            "env": {"RUST_LOG": "debug"}
        }"#;
        let agent = AcpAgent::from_str(json).unwrap();
        let config = agent.config();
        assert_eq!(config.command(), Path::new("/usr/bin/python"));
        assert_eq!(config.arguments(), ["agent.py", "--verbose"]);
        assert_eq!(
            config.environment().get("RUST_LOG").map(String::as_str),
            Some("debug")
        );
    }

    #[test]
    fn test_config_json_round_trip() {
        let config = AcpAgentConfig::new("agent")
            .args(["--mode", "fast"])
            .envs([("RUST_LOG", "debug"), ("NO_COLOR", "1")]);

        let json = serde_json::to_value(&config).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "command": "agent",
                "args": ["--mode", "fast"],
                "env": {
                    "NO_COLOR": "1",
                    "RUST_LOG": "debug"
                }
            })
        );
        assert_eq!(
            serde_json::from_value::<AcpAgentConfig>(json).unwrap(),
            config
        );
    }

    #[test]
    fn test_config_json_defaults_and_omits_empty_collections() {
        let config = serde_json::from_value::<AcpAgentConfig>(serde_json::json!({
            "command": "agent"
        }))
        .unwrap();

        assert!(config.arguments().is_empty());
        assert!(config.environment().is_empty());
        assert_eq!(
            serde_json::to_value(config).unwrap(),
            serde_json::json!({ "command": "agent" })
        );
    }

    #[test]
    fn test_reject_mcp_server_json() {
        let json = r#"{
            "type": "stdio",
            "name": "my-agent",
            "command": "/usr/bin/python",
            "args": ["agent.py"],
            "env": []
        }"#;
        let error = AcpAgent::from_str(json).unwrap_err();
        assert!(
            error
                .data
                .as_ref()
                .and_then(serde_json::Value::as_str)
                .is_some_and(|message| message.contains("unknown field")),
            "unexpected error: {error:?}"
        );
    }
}
