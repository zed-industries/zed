use crate::{
    json_log::LogRecord,
    protocol::{
        message_len_from_buffer, read_message_with_len, write_message, MessageId, MESSAGE_LEN_SIZE,
    },
    proxy::ProxyLaunchError,
};
use anyhow::{anyhow, Context as _, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    channel::{
        mpsc::{self, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::{BoxFuture, Shared},
    select, select_biased, AsyncReadExt as _, Future, FutureExt as _, StreamExt as _,
};
use gpui::{
    AppContext, AsyncAppContext, BorrowAppContext, Context, EventEmitter, Global, Model,
    ModelContext, SemanticVersion, Task, WeakModel,
};
use itertools::Itertools;
use parking_lot::Mutex;
use paths;
use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::{
    proto::{self, build_typed_envelope, Envelope, EnvelopedMessage, PeerId, RequestMessage},
    AnyProtoClient, EntityMessageSubscriber, ErrorExt, ProtoClient, ProtoMessageHandlerSet,
    RpcError,
};
use smol::{
    fs,
    process::{self, Child, Stdio},
};
use std::{
    any::TypeId,
    collections::VecDeque,
    fmt, iter,
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering::SeqCst},
        Arc, Weak,
    },
    time::{Duration, Instant},
};
use tempfile::TempDir;
use util::ResultExt;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct SshProjectId(pub u64);

#[derive(Clone)]
pub struct SshSocket {
    connection_options: SshConnectionOptions,
    socket_path: PathBuf,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SshConnectionOptions {
    pub host: String,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub args: Option<Vec<String>>,

    pub nickname: Option<String>,
    pub upload_binary_over_ssh: bool,
}

#[macro_export]
macro_rules! shell_script {
    ($fmt:expr, $($name:ident = $arg:expr),+ $(,)?) => {{
        format!(
            $fmt,
            $(
                $name = shlex::try_quote($arg).unwrap()
            ),+
        )
    }};
}

impl SshConnectionOptions {
    pub fn parse_command_line(input: &str) -> Result<Self> {
        let input = input.trim_start_matches("ssh ");
        let mut hostname: Option<String> = None;
        let mut username: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut args = Vec::new();

        // disallowed: -E, -e, -F, -f, -G, -g, -M, -N, -n, -O, -q, -S, -s, -T, -t, -V, -v, -W
        const ALLOWED_OPTS: &[&str] = &[
            "-4", "-6", "-A", "-a", "-C", "-K", "-k", "-X", "-x", "-Y", "-y",
        ];
        const ALLOWED_ARGS: &[&str] = &[
            "-B", "-b", "-c", "-D", "-I", "-i", "-J", "-L", "-l", "-m", "-o", "-P", "-p", "-R",
            "-w",
        ];

        let mut tokens = shlex::split(input)
            .ok_or_else(|| anyhow!("invalid input"))?
            .into_iter();

        'outer: while let Some(arg) = tokens.next() {
            if ALLOWED_OPTS.contains(&(&arg as &str)) {
                args.push(arg.to_string());
                continue;
            }
            if arg == "-p" {
                port = tokens.next().and_then(|arg| arg.parse().ok());
                continue;
            } else if let Some(p) = arg.strip_prefix("-p") {
                port = p.parse().ok();
                continue;
            }
            if arg == "-l" {
                username = tokens.next();
                continue;
            } else if let Some(l) = arg.strip_prefix("-l") {
                username = Some(l.to_string());
                continue;
            }
            for a in ALLOWED_ARGS {
                if arg == *a {
                    args.push(arg);
                    if let Some(next) = tokens.next() {
                        args.push(next);
                    }
                    continue 'outer;
                } else if arg.starts_with(a) {
                    args.push(arg);
                    continue 'outer;
                }
            }
            if arg.starts_with("-") || hostname.is_some() {
                anyhow::bail!("unsupported argument: {:?}", arg);
            }
            let mut input = &arg as &str;
            if let Some((u, rest)) = input.split_once('@') {
                input = rest;
                username = Some(u.to_string());
            }
            if let Some((rest, p)) = input.split_once(':') {
                input = rest;
                port = p.parse().ok()
            }
            hostname = Some(input.to_string())
        }

        let Some(hostname) = hostname else {
            anyhow::bail!("missing hostname");
        };

        Ok(Self {
            host: hostname.to_string(),
            username: username.clone(),
            port,
            args: Some(args),
            password: None,
            nickname: None,
            upload_binary_over_ssh: false,
        })
    }

    pub fn ssh_url(&self) -> String {
        let mut result = String::from("ssh://");
        if let Some(username) = &self.username {
            result.push_str(username);
            result.push('@');
        }
        result.push_str(&self.host);
        if let Some(port) = self.port {
            result.push(':');
            result.push_str(&port.to_string());
        }
        result
    }

    pub fn additional_args(&self) -> Option<&Vec<String>> {
        self.args.as_ref()
    }

    fn scp_url(&self) -> String {
        if let Some(username) = &self.username {
            format!("{}@{}", username, self.host)
        } else {
            self.host.clone()
        }
    }

    pub fn connection_string(&self) -> String {
        let host = if let Some(username) = &self.username {
            format!("{}@{}", username, self.host)
        } else {
            self.host.clone()
        };
        if let Some(port) = &self.port {
            format!("{}:{}", host, port)
        } else {
            host
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SshPlatform {
    pub os: &'static str,
    pub arch: &'static str,
}

impl SshPlatform {
    pub fn triple(&self) -> Option<String> {
        Some(format!(
            "{}-{}",
            self.arch,
            match self.os {
                "linux" => "unknown-linux-gnu",
                "macos" => "apple-darwin",
                _ => return None,
            }
        ))
    }
}

pub trait SshClientDelegate: Send + Sync {
    fn ask_password(
        &self,
        prompt: String,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<String>>;
    fn get_download_params(
        &self,
        platform: SshPlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<Option<(String, String)>>>;

    fn download_server_binary_locally(
        &self,
        platform: SshPlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<PathBuf>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncAppContext);
}

impl SshSocket {
    // :WARNING: ssh unquotes arguments when executing on the remote :WARNING:
    // e.g. $ ssh host sh -c 'ls -l' is equivalent to $ ssh host sh -c ls -l
    // and passes -l as an argument to sh, not to ls.
    // You need to do it like this: $ ssh host "sh -c 'ls -l /tmp'"
    fn ssh_command(&self, program: &str, args: &[&str]) -> process::Command {
        let mut command = util::command::new_smol_command("ssh");
        let to_run = iter::once(&program)
            .chain(args.iter())
            .map(|token| {
                // We're trying to work with: sh, bash, zsh, fish, tcsh, ...?
                debug_assert!(
                    !token.contains('\n'),
                    "multiline arguments do not work in all shells"
                );
                shlex::try_quote(token).unwrap()
            })
            .join(" ");
        log::debug!("ssh {} {:?}", self.connection_options.ssh_url(), to_run);
        self.ssh_options(&mut command)
            .arg(self.connection_options.ssh_url())
            .arg(to_run);
        command
    }

    async fn run_command(&self, program: &str, args: &[&str]) -> Result<String> {
        let output = self.ssh_command(program, args).output().await?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!(
                "failed to run command: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn ssh_options<'a>(&self, command: &'a mut process::Command) -> &'a mut process::Command {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(["-o", "ControlMaster=no", "-o"])
            .arg(format!("ControlPath={}", self.socket_path.display()))
    }

    fn ssh_args(&self) -> Vec<String> {
        vec![
            "-o".to_string(),
            "ControlMaster=no".to_string(),
            "-o".to_string(),
            format!("ControlPath={}", self.socket_path.display()),
            self.connection_options.ssh_url(),
        ]
    }
}

const MAX_MISSED_HEARTBEATS: usize = 5;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(5);

const MAX_RECONNECT_ATTEMPTS: usize = 3;

enum State {
    Connecting,
    Connected {
        ssh_connection: Arc<dyn RemoteConnection>,
        delegate: Arc<dyn SshClientDelegate>,

        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    HeartbeatMissed {
        missed_heartbeats: usize,

        ssh_connection: Arc<dyn RemoteConnection>,
        delegate: Arc<dyn SshClientDelegate>,

        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    Reconnecting,
    ReconnectFailed {
        ssh_connection: Arc<dyn RemoteConnection>,
        delegate: Arc<dyn SshClientDelegate>,

        error: anyhow::Error,
        attempts: usize,
    },
    ReconnectExhausted,
    ServerNotRunning,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connecting => write!(f, "connecting"),
            Self::Connected { .. } => write!(f, "connected"),
            Self::Reconnecting => write!(f, "reconnecting"),
            Self::ReconnectFailed { .. } => write!(f, "reconnect failed"),
            Self::ReconnectExhausted => write!(f, "reconnect exhausted"),
            Self::HeartbeatMissed { .. } => write!(f, "heartbeat missed"),
            Self::ServerNotRunning { .. } => write!(f, "server not running"),
        }
    }
}

impl State {
    fn ssh_connection(&self) -> Option<&dyn RemoteConnection> {
        match self {
            Self::Connected { ssh_connection, .. } => Some(ssh_connection.as_ref()),
            Self::HeartbeatMissed { ssh_connection, .. } => Some(ssh_connection.as_ref()),
            Self::ReconnectFailed { ssh_connection, .. } => Some(ssh_connection.as_ref()),
            _ => None,
        }
    }

    fn can_reconnect(&self) -> bool {
        match self {
            Self::Connected { .. }
            | Self::HeartbeatMissed { .. }
            | Self::ReconnectFailed { .. } => true,
            State::Connecting
            | State::Reconnecting
            | State::ReconnectExhausted
            | State::ServerNotRunning => false,
        }
    }

    fn is_reconnect_failed(&self) -> bool {
        matches!(self, Self::ReconnectFailed { .. })
    }

    fn is_reconnect_exhausted(&self) -> bool {
        matches!(self, Self::ReconnectExhausted { .. })
    }

    fn is_server_not_running(&self) -> bool {
        matches!(self, Self::ServerNotRunning)
    }

    fn is_reconnecting(&self) -> bool {
        matches!(self, Self::Reconnecting { .. })
    }

    fn heartbeat_recovered(self) -> Self {
        match self {
            Self::HeartbeatMissed {
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
                ..
            } => Self::Connected {
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            },
            _ => self,
        }
    }

    fn heartbeat_missed(self) -> Self {
        match self {
            Self::Connected {
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            } => Self::HeartbeatMissed {
                missed_heartbeats: 1,
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            },
            Self::HeartbeatMissed {
                missed_heartbeats,
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            } => Self::HeartbeatMissed {
                missed_heartbeats: missed_heartbeats + 1,
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            },
            _ => self,
        }
    }
}

/// The state of the ssh connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    HeartbeatMissed,
    Reconnecting,
    Disconnected,
}

impl From<&State> for ConnectionState {
    fn from(value: &State) -> Self {
        match value {
            State::Connecting => Self::Connecting,
            State::Connected { .. } => Self::Connected,
            State::Reconnecting | State::ReconnectFailed { .. } => Self::Reconnecting,
            State::HeartbeatMissed { .. } => Self::HeartbeatMissed,
            State::ReconnectExhausted => Self::Disconnected,
            State::ServerNotRunning => Self::Disconnected,
        }
    }
}

pub struct SshRemoteClient {
    client: Arc<ChannelClient>,
    unique_identifier: String,
    connection_options: SshConnectionOptions,
    state: Arc<Mutex<Option<State>>>,
}

#[derive(Debug)]
pub enum SshRemoteEvent {
    Disconnected,
}

impl EventEmitter<SshRemoteEvent> for SshRemoteClient {}

// Identifies the socket on the remote server so that reconnects
// can re-join the same project.
pub enum ConnectionIdentifier {
    Setup(u64),
    Workspace(i64),
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl ConnectionIdentifier {
    pub fn setup() -> Self {
        Self::Setup(NEXT_ID.fetch_add(1, SeqCst))
    }
    // This string gets used in a socket name, and so must be relatively short.
    // The total length of:
    //   /home/{username}/.local/share/zed/server_state/{name}/stdout.sock
    // Must be less than about 100 characters
    //   https://unix.stackexchange.com/questions/367008/why-is-socket-path-length-limited-to-a-hundred-chars
    // So our strings should be at most 20 characters or so.
    fn to_string(&self, cx: &AppContext) -> String {
        let identifier_prefix = match ReleaseChannel::global(cx) {
            ReleaseChannel::Stable => "".to_string(),
            release_channel => format!("{}-", release_channel.dev_name()),
        };
        match self {
            Self::Setup(setup_id) => format!("{identifier_prefix}setup-{setup_id}"),
            Self::Workspace(workspace_id) => {
                format!("{identifier_prefix}workspace-{workspace_id}",)
            }
        }
    }
}

impl SshRemoteClient {
    pub fn new(
        unique_identifier: ConnectionIdentifier,
        connection_options: SshConnectionOptions,
        cancellation: oneshot::Receiver<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AppContext,
    ) -> Task<Result<Option<Model<Self>>>> {
        let unique_identifier = unique_identifier.to_string(cx);
        cx.spawn(|mut cx| async move {
            let success = Box::pin(async move {
                let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
                let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
                let (connection_activity_tx, connection_activity_rx) = mpsc::channel::<()>(1);

                let client =
                    cx.update(|cx| ChannelClient::new(incoming_rx, outgoing_tx, cx, "client"))?;
                let this = cx.new_model(|_| Self {
                    client: client.clone(),
                    unique_identifier: unique_identifier.clone(),
                    connection_options: connection_options.clone(),
                    state: Arc::new(Mutex::new(Some(State::Connecting))),
                })?;

                let ssh_connection = cx
                    .update(|cx| {
                        cx.update_default_global(|pool: &mut ConnectionPool, cx| {
                            pool.connect(connection_options, &delegate, cx)
                        })
                    })?
                    .await
                    .map_err(|e| e.cloned())?;

                let io_task = ssh_connection.start_proxy(
                    unique_identifier,
                    false,
                    incoming_tx,
                    outgoing_rx,
                    connection_activity_tx,
                    delegate.clone(),
                    &mut cx,
                );

                let multiplex_task = Self::monitor(this.downgrade(), io_task, &cx);

                if let Err(error) = client.ping(HEARTBEAT_TIMEOUT).await {
                    log::error!("failed to establish connection: {}", error);
                    return Err(error);
                }

                let heartbeat_task =
                    Self::heartbeat(this.downgrade(), connection_activity_rx, &mut cx);

                this.update(&mut cx, |this, _| {
                    *this.state.lock() = Some(State::Connected {
                        ssh_connection,
                        delegate,
                        multiplex_task,
                        heartbeat_task,
                    });
                })?;

                Ok(Some(this))
            });

            select! {
                _ = cancellation.fuse() => {
                    Ok(None)
                }
                result = success.fuse() =>  result
            }
        })
    }

    pub fn shutdown_processes<T: RequestMessage>(
        &self,
        shutdown_request: Option<T>,
    ) -> Option<impl Future<Output = ()>> {
        let state = self.state.lock().take()?;
        log::info!("shutting down ssh processes");

        let State::Connected {
            multiplex_task,
            heartbeat_task,
            ssh_connection,
            delegate,
        } = state
        else {
            return None;
        };

        let client = self.client.clone();

        Some(async move {
            if let Some(shutdown_request) = shutdown_request {
                client.send(shutdown_request).log_err();
                // We wait 50ms instead of waiting for a response, because
                // waiting for a response would require us to wait on the main thread
                // which we want to avoid in an `on_app_quit` callback.
                smol::Timer::after(Duration::from_millis(50)).await;
            }

            // Drop `multiplex_task` because it owns our ssh_proxy_process, which is a
            // child of master_process.
            drop(multiplex_task);
            // Now drop the rest of state, which kills master process.
            drop(heartbeat_task);
            drop(ssh_connection);
            drop(delegate);
        })
    }

    fn reconnect(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        let mut lock = self.state.lock();

        let can_reconnect = lock
            .as_ref()
            .map(|state| state.can_reconnect())
            .unwrap_or(false);
        if !can_reconnect {
            let error = if let Some(state) = lock.as_ref() {
                format!("invalid state, cannot reconnect while in state {state}")
            } else {
                "no state set".to_string()
            };
            log::info!("aborting reconnect, because not in state that allows reconnecting");
            return Err(anyhow!(error));
        }

        let state = lock.take().unwrap();
        let (attempts, ssh_connection, delegate) = match state {
            State::Connected {
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            }
            | State::HeartbeatMissed {
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
                ..
            } => {
                drop(multiplex_task);
                drop(heartbeat_task);
                (0, ssh_connection, delegate)
            }
            State::ReconnectFailed {
                attempts,
                ssh_connection,
                delegate,
                ..
            } => (attempts, ssh_connection, delegate),
            State::Connecting
            | State::Reconnecting
            | State::ReconnectExhausted
            | State::ServerNotRunning => unreachable!(),
        };

        let attempts = attempts + 1;
        if attempts > MAX_RECONNECT_ATTEMPTS {
            log::error!(
                "Failed to reconnect to after {} attempts, giving up",
                MAX_RECONNECT_ATTEMPTS
            );
            drop(lock);
            self.set_state(State::ReconnectExhausted, cx);
            return Ok(());
        }
        drop(lock);

        self.set_state(State::Reconnecting, cx);

        log::info!("Trying to reconnect to ssh server... Attempt {}", attempts);

        let unique_identifier = self.unique_identifier.clone();
        let client = self.client.clone();
        let reconnect_task = cx.spawn(|this, mut cx| async move {
            macro_rules! failed {
                ($error:expr, $attempts:expr, $ssh_connection:expr, $delegate:expr) => {
                    return State::ReconnectFailed {
                        error: anyhow!($error),
                        attempts: $attempts,
                        ssh_connection: $ssh_connection,
                        delegate: $delegate,
                    };
                };
            }

            if let Err(error) = ssh_connection
                .kill()
                .await
                .context("Failed to kill ssh process")
            {
                failed!(error, attempts, ssh_connection, delegate);
            };

            let connection_options = ssh_connection.connection_options();

            let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
            let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
            let (connection_activity_tx, connection_activity_rx) = mpsc::channel::<()>(1);

            let (ssh_connection, io_task) = match async {
                let ssh_connection = cx
                    .update_global(|pool: &mut ConnectionPool, cx| {
                        pool.connect(connection_options, &delegate, cx)
                    })?
                    .await
                    .map_err(|error| error.cloned())?;

                let io_task = ssh_connection.start_proxy(
                    unique_identifier,
                    true,
                    incoming_tx,
                    outgoing_rx,
                    connection_activity_tx,
                    delegate.clone(),
                    &mut cx,
                );
                anyhow::Ok((ssh_connection, io_task))
            }
            .await
            {
                Ok((ssh_connection, io_task)) => (ssh_connection, io_task),
                Err(error) => {
                    failed!(error, attempts, ssh_connection, delegate);
                }
            };

            let multiplex_task = Self::monitor(this.clone(), io_task, &cx);
            client.reconnect(incoming_rx, outgoing_tx, &cx);

            if let Err(error) = client.resync(HEARTBEAT_TIMEOUT).await {
                failed!(error, attempts, ssh_connection, delegate);
            };

            State::Connected {
                ssh_connection,
                delegate,
                multiplex_task,
                heartbeat_task: Self::heartbeat(this.clone(), connection_activity_rx, &mut cx),
            }
        });

        cx.spawn(|this, mut cx| async move {
            let new_state = reconnect_task.await;
            this.update(&mut cx, |this, cx| {
                this.try_set_state(cx, |old_state| {
                    if old_state.is_reconnecting() {
                        match &new_state {
                            State::Connecting
                            | State::Reconnecting { .. }
                            | State::HeartbeatMissed { .. }
                            | State::ServerNotRunning => {}
                            State::Connected { .. } => {
                                log::info!("Successfully reconnected");
                            }
                            State::ReconnectFailed {
                                error, attempts, ..
                            } => {
                                log::error!(
                                    "Reconnect attempt {} failed: {:?}. Starting new attempt...",
                                    attempts,
                                    error
                                );
                            }
                            State::ReconnectExhausted => {
                                log::error!("Reconnect attempt failed and all attempts exhausted");
                            }
                        }
                        Some(new_state)
                    } else {
                        None
                    }
                });

                if this.state_is(State::is_reconnect_failed) {
                    this.reconnect(cx)
                } else if this.state_is(State::is_reconnect_exhausted) {
                    Ok(())
                } else {
                    log::debug!("State has transition from Reconnecting into new state while attempting reconnect.");
                    Ok(())
                }
            })
        })
        .detach_and_log_err(cx);

        Ok(())
    }

    fn heartbeat(
        this: WeakModel<Self>,
        mut connection_activity_rx: mpsc::Receiver<()>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<()>> {
        let Ok(client) = this.update(cx, |this, _| this.client.clone()) else {
            return Task::ready(Err(anyhow!("SshRemoteClient lost")));
        };

        cx.spawn(|mut cx| {
            let this = this.clone();
            async move {
                let mut missed_heartbeats = 0;

                let keepalive_timer = cx.background_executor().timer(HEARTBEAT_INTERVAL).fuse();
                futures::pin_mut!(keepalive_timer);

                loop {
                    select_biased! {
                        result = connection_activity_rx.next().fuse() => {
                            if result.is_none() {
                                log::warn!("ssh heartbeat: connection activity channel has been dropped. stopping.");
                                return Ok(());
                            }

                            if missed_heartbeats != 0 {
                                missed_heartbeats = 0;
                                this.update(&mut cx, |this, mut cx| {
                                    this.handle_heartbeat_result(missed_heartbeats, &mut cx)
                                })?;
                            }
                        }
                        _ = keepalive_timer => {
                            log::debug!("Sending heartbeat to server...");

                            let result = select_biased! {
                                _ = connection_activity_rx.next().fuse() => {
                                    Ok(())
                                }
                                ping_result = client.ping(HEARTBEAT_TIMEOUT).fuse() => {
                                    ping_result
                                }
                            };

                            if result.is_err() {
                                missed_heartbeats += 1;
                                log::warn!(
                                    "No heartbeat from server after {:?}. Missed heartbeat {} out of {}.",
                                    HEARTBEAT_TIMEOUT,
                                    missed_heartbeats,
                                    MAX_MISSED_HEARTBEATS
                                );
                            } else if missed_heartbeats != 0 {
                                missed_heartbeats = 0;
                            } else {
                                continue;
                            }

                            let result = this.update(&mut cx, |this, mut cx| {
                                this.handle_heartbeat_result(missed_heartbeats, &mut cx)
                            })?;
                            if result.is_break() {
                                return Ok(());
                            }
                        }
                    }

                    keepalive_timer.set(cx.background_executor().timer(HEARTBEAT_INTERVAL).fuse());
                }
            }
        })
    }

    fn handle_heartbeat_result(
        &mut self,
        missed_heartbeats: usize,
        cx: &mut ModelContext<Self>,
    ) -> ControlFlow<()> {
        let state = self.state.lock().take().unwrap();
        let next_state = if missed_heartbeats > 0 {
            state.heartbeat_missed()
        } else {
            state.heartbeat_recovered()
        };

        self.set_state(next_state, cx);

        if missed_heartbeats >= MAX_MISSED_HEARTBEATS {
            log::error!(
                "Missed last {} heartbeats. Reconnecting...",
                missed_heartbeats
            );

            self.reconnect(cx)
                .context("failed to start reconnect process after missing heartbeats")
                .log_err();
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn monitor(
        this: WeakModel<Self>,
        io_task: Task<Result<i32>>,
        cx: &AsyncAppContext,
    ) -> Task<Result<()>> {
        cx.spawn(|mut cx| async move {
            let result = io_task.await;

            match result {
                Ok(exit_code) => {
                    if let Some(error) = ProxyLaunchError::from_exit_code(exit_code) {
                        match error {
                            ProxyLaunchError::ServerNotRunning => {
                                log::error!("failed to reconnect because server is not running");
                                this.update(&mut cx, |this, cx| {
                                    this.set_state(State::ServerNotRunning, cx);
                                })?;
                            }
                        }
                    } else if exit_code > 0 {
                        log::error!("proxy process terminated unexpectedly");
                        this.update(&mut cx, |this, cx| {
                            this.reconnect(cx).ok();
                        })?;
                    }
                }
                Err(error) => {
                    log::warn!("ssh io task died with error: {:?}. reconnecting...", error);
                    this.update(&mut cx, |this, cx| {
                        this.reconnect(cx).ok();
                    })?;
                }
            }

            Ok(())
        })
    }

    fn state_is(&self, check: impl FnOnce(&State) -> bool) -> bool {
        self.state.lock().as_ref().map_or(false, check)
    }

    fn try_set_state(
        &self,
        cx: &mut ModelContext<Self>,
        map: impl FnOnce(&State) -> Option<State>,
    ) {
        let mut lock = self.state.lock();
        let new_state = lock.as_ref().and_then(map);

        if let Some(new_state) = new_state {
            lock.replace(new_state);
            cx.notify();
        }
    }

    fn set_state(&self, state: State, cx: &mut ModelContext<Self>) {
        log::info!("setting state to '{}'", &state);

        let is_reconnect_exhausted = state.is_reconnect_exhausted();
        let is_server_not_running = state.is_server_not_running();
        self.state.lock().replace(state);

        if is_reconnect_exhausted || is_server_not_running {
            cx.emit(SshRemoteEvent::Disconnected);
        }
        cx.notify();
    }

    pub fn subscribe_to_entity<E: 'static>(&self, remote_id: u64, entity: &Model<E>) {
        self.client.subscribe_to_entity(remote_id, entity);
    }

    pub fn ssh_args(&self) -> Option<Vec<String>> {
        self.state
            .lock()
            .as_ref()
            .and_then(|state| state.ssh_connection())
            .map(|ssh_connection| ssh_connection.ssh_args())
    }

    pub fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: PathBuf,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let state = self.state.lock();
        let Some(connection) = state.as_ref().and_then(|state| state.ssh_connection()) else {
            return Task::ready(Err(anyhow!("no ssh connection")));
        };
        connection.upload_directory(src_path, dest_path, cx)
    }

    pub fn proto_client(&self) -> AnyProtoClient {
        self.client.clone().into()
    }

    pub fn connection_string(&self) -> String {
        self.connection_options.connection_string()
    }

    pub fn connection_options(&self) -> SshConnectionOptions {
        self.connection_options.clone()
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.state
            .lock()
            .as_ref()
            .map(ConnectionState::from)
            .unwrap_or(ConnectionState::Disconnected)
    }

    pub fn is_disconnected(&self) -> bool {
        self.connection_state() == ConnectionState::Disconnected
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_disconnect(&self, client_cx: &mut AppContext) -> Task<()> {
        let opts = self.connection_options();
        client_cx.spawn(|cx| async move {
            let connection = cx
                .update_global(|c: &mut ConnectionPool, _| {
                    if let Some(ConnectionPoolEntry::Connecting(c)) = c.connections.get(&opts) {
                        c.clone()
                    } else {
                        panic!("missing test connection")
                    }
                })
                .unwrap()
                .await
                .unwrap();

            connection.simulate_disconnect(&cx);
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_server(
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (SshConnectionOptions, Arc<ChannelClient>) {
        let port = client_cx
            .update(|cx| cx.default_global::<ConnectionPool>().connections.len() as u16 + 1);
        let opts = SshConnectionOptions {
            host: "<fake>".to_string(),
            port: Some(port),
            ..Default::default()
        };
        let (outgoing_tx, _) = mpsc::unbounded::<Envelope>();
        let (_, incoming_rx) = mpsc::unbounded::<Envelope>();
        let server_client =
            server_cx.update(|cx| ChannelClient::new(incoming_rx, outgoing_tx, cx, "fake-server"));
        let connection: Arc<dyn RemoteConnection> = Arc::new(fake::FakeRemoteConnection {
            connection_options: opts.clone(),
            server_cx: fake::SendableCx::new(server_cx),
            server_channel: server_client.clone(),
        });

        client_cx.update(|cx| {
            cx.update_default_global(|c: &mut ConnectionPool, cx| {
                c.connections.insert(
                    opts.clone(),
                    ConnectionPoolEntry::Connecting(
                        cx.background_executor()
                            .spawn({
                                let connection = connection.clone();
                                async move { Ok(connection.clone()) }
                            })
                            .shared(),
                    ),
                );
            })
        });

        (opts, server_client)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn fake_client(
        opts: SshConnectionOptions,
        client_cx: &mut gpui::TestAppContext,
    ) -> Model<Self> {
        let (_tx, rx) = oneshot::channel();
        client_cx
            .update(|cx| {
                Self::new(
                    ConnectionIdentifier::setup(),
                    opts,
                    rx,
                    Arc::new(fake::Delegate),
                    cx,
                )
            })
            .await
            .unwrap()
            .unwrap()
    }
}

enum ConnectionPoolEntry {
    Connecting(Shared<Task<Result<Arc<dyn RemoteConnection>, Arc<anyhow::Error>>>>),
    Connected(Weak<dyn RemoteConnection>),
}

#[derive(Default)]
struct ConnectionPool {
    connections: HashMap<SshConnectionOptions, ConnectionPoolEntry>,
}

impl Global for ConnectionPool {}

impl ConnectionPool {
    pub fn connect(
        &mut self,
        opts: SshConnectionOptions,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AppContext,
    ) -> Shared<Task<Result<Arc<dyn RemoteConnection>, Arc<anyhow::Error>>>> {
        let connection = self.connections.get(&opts);
        match connection {
            Some(ConnectionPoolEntry::Connecting(task)) => {
                let delegate = delegate.clone();
                cx.spawn(|mut cx| async move {
                    delegate.set_status(Some("Waiting for existing connection attempt"), &mut cx);
                })
                .detach();
                return task.clone();
            }
            Some(ConnectionPoolEntry::Connected(ssh)) => {
                if let Some(ssh) = ssh.upgrade() {
                    if !ssh.has_been_killed() {
                        return Task::ready(Ok(ssh)).shared();
                    }
                }
                self.connections.remove(&opts);
            }
            None => {}
        }

        let task = cx
            .spawn({
                let opts = opts.clone();
                let delegate = delegate.clone();
                |mut cx| async move {
                    let connection = SshRemoteConnection::new(opts.clone(), delegate, &mut cx)
                        .await
                        .map(|connection| Arc::new(connection) as Arc<dyn RemoteConnection>);

                    cx.update_global(|pool: &mut Self, _| {
                        debug_assert!(matches!(
                            pool.connections.get(&opts),
                            Some(ConnectionPoolEntry::Connecting(_))
                        ));
                        match connection {
                            Ok(connection) => {
                                pool.connections.insert(
                                    opts.clone(),
                                    ConnectionPoolEntry::Connected(Arc::downgrade(&connection)),
                                );
                                Ok(connection)
                            }
                            Err(error) => {
                                pool.connections.remove(&opts);
                                Err(Arc::new(error))
                            }
                        }
                    })?
                }
            })
            .shared();

        self.connections
            .insert(opts.clone(), ConnectionPoolEntry::Connecting(task.clone()));
        task
    }
}

impl From<SshRemoteClient> for AnyProtoClient {
    fn from(client: SshRemoteClient) -> Self {
        AnyProtoClient::new(client.client.clone())
    }
}

#[async_trait(?Send)]
trait RemoteConnection: Send + Sync {
    #[allow(clippy::too_many_arguments)]
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<i32>>;
    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: PathBuf,
        cx: &AppContext,
    ) -> Task<Result<()>>;
    async fn kill(&self) -> Result<()>;
    fn has_been_killed(&self) -> bool;
    fn ssh_args(&self) -> Vec<String>;
    fn connection_options(&self) -> SshConnectionOptions;

    #[cfg(any(test, feature = "test-support"))]
    fn simulate_disconnect(&self, _: &AsyncAppContext) {}
}

struct SshRemoteConnection {
    socket: SshSocket,
    master_process: Mutex<Option<Child>>,
    remote_binary_path: Option<PathBuf>,
    _temp_dir: TempDir,
}

#[async_trait(?Send)]
impl RemoteConnection for SshRemoteConnection {
    async fn kill(&self) -> Result<()> {
        let Some(mut process) = self.master_process.lock().take() else {
            return Ok(());
        };
        process.kill().ok();
        process.status().await?;
        Ok(())
    }

    fn has_been_killed(&self) -> bool {
        self.master_process.lock().is_none()
    }

    fn ssh_args(&self) -> Vec<String> {
        self.socket.ssh_args()
    }

    fn connection_options(&self) -> SshConnectionOptions {
        self.socket.connection_options.clone()
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: PathBuf,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        let mut command = util::command::new_smol_command("scp");
        let output = self
            .socket
            .ssh_options(&mut command)
            .args(
                self.socket
                    .connection_options
                    .port
                    .map(|port| vec!["-P".to_string(), port.to_string()])
                    .unwrap_or_default(),
            )
            .arg("-C")
            .arg("-r")
            .arg(&src_path)
            .arg(format!(
                "{}:{}",
                self.socket.connection_options.scp_url(),
                dest_path.display()
            ))
            .output();

        cx.background_executor().spawn(async move {
            let output = output.await?;

            if !output.status.success() {
                return Err(anyhow!(
                    "failed to upload directory {} -> {}: {}",
                    src_path.display(),
                    dest_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            Ok(())
        })
    }

    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<i32>> {
        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_path) = self.remote_binary_path.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut start_proxy_command = shell_script!(
            "exec {binary_path} proxy --identifier {identifier}",
            binary_path = &remote_binary_path.to_string_lossy(),
            identifier = &unique_identifier,
        );

        if let Some(rust_log) = std::env::var("RUST_LOG").ok() {
            start_proxy_command = format!(
                "RUST_LOG={} {}",
                shlex::try_quote(&rust_log).unwrap(),
                start_proxy_command
            )
        }
        if let Some(rust_backtrace) = std::env::var("RUST_BACKTRACE").ok() {
            start_proxy_command = format!(
                "RUST_BACKTRACE={} {}",
                shlex::try_quote(&rust_backtrace).unwrap(),
                start_proxy_command
            )
        }
        if reconnect {
            start_proxy_command.push_str(" --reconnect");
        }

        let ssh_proxy_process = match self
            .socket
            .ssh_command("sh", &["-c", &start_proxy_command])
            // IMPORTANT: we kill this process when we drop the task that uses it.
            .kill_on_drop(true)
            .spawn()
        {
            Ok(process) => process,
            Err(error) => {
                return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)))
            }
        };

        Self::multiplex(
            ssh_proxy_process,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            &cx,
        )
    }
}

impl SshRemoteConnection {
    #[cfg(not(unix))]
    async fn new(
        _connection_options: SshConnectionOptions,
        _delegate: Arc<dyn SshClientDelegate>,
        _cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        Err(anyhow!("ssh is not supported on this platform"))
    }

    #[cfg(unix)]
    async fn new(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        use futures::AsyncWriteExt as _;
        use futures::{io::BufReader, AsyncBufReadExt as _};
        use smol::net::unix::UnixStream;
        use smol::{fs::unix::PermissionsExt as _, net::unix::UnixListener};
        use util::ResultExt as _;

        delegate.set_status(Some("Connecting"), cx);

        let url = connection_options.ssh_url();
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-ssh-session")
            .tempdir()?;

        // Create a domain socket listener to handle requests from the askpass program.
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let (askpass_opened_tx, askpass_opened_rx) = oneshot::channel::<()>();
        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;

        let (askpass_kill_master_tx, askpass_kill_master_rx) = oneshot::channel::<UnixStream>();
        let mut kill_tx = Some(askpass_kill_master_tx);

        let askpass_task = cx.spawn({
            let delegate = delegate.clone();
            |mut cx| async move {
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
                    let password_prompt = String::from_utf8_lossy(&buffer);
                    if let Some(password) = delegate
                        .ask_password(password_prompt.to_string(), &mut cx)
                        .await
                        .context("failed to get ssh password")
                        .and_then(|p| p)
                        .log_err()
                    {
                        stream.write_all(password.as_bytes()).await.log_err();
                    } else {
                        if let Some(kill_tx) = kill_tx.take() {
                            kill_tx.send(stream).log_err();
                            break;
                        }
                    }
                }
            }
        });

        anyhow::ensure!(
            which::which("nc").is_ok(),
            "Cannot find nc, which is required to connect over ssh."
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

        // Start the master SSH process, which does not do anything except for establish
        // the connection and keep it open, allowing other ssh commands to reuse it
        // via a control socket.
        let socket_path = temp_dir.path().join("ssh.sock");

        let mut master_process = process::Command::new("ssh")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("SSH_ASKPASS_REQUIRE", "force")
            .env("SSH_ASKPASS", &askpass_script_path)
            .args(connection_options.additional_args().unwrap_or(&Vec::new()))
            .args([
                "-N",
                "-o",
                "ControlPersist=no",
                "-o",
                "ControlMaster=yes",
                "-o",
            ])
            .arg(format!("ControlPath={}", socket_path.display()))
            .arg(&url)
            .kill_on_drop(true)
            .spawn()?;

        // Wait for this ssh process to close its stdout, indicating that authentication
        // has completed.
        let mut stdout = master_process.stdout.take().unwrap();
        let mut output = Vec::new();
        let connection_timeout = Duration::from_secs(10);

        let result = select_biased! {
            _ = askpass_opened_rx.fuse() => {
                select_biased! {
                    stream = askpass_kill_master_rx.fuse() => {
                        master_process.kill().ok();
                        drop(stream);
                        Err(anyhow!("SSH connection canceled"))
                    }
                    // If the askpass script has opened, that means the user is typing
                    // their password, in which case we don't want to timeout anymore,
                    // since we know a connection has been established.
                    result = stdout.read_to_end(&mut output).fuse() => {
                        result?;
                        Ok(())
                    }
                }
            }
            _ = stdout.read_to_end(&mut output).fuse() => {
                Ok(())
            }
            _ = futures::FutureExt::fuse(smol::Timer::after(connection_timeout)) => {
                Err(anyhow!("Exceeded {:?} timeout trying to connect to host", connection_timeout))
            }
        };

        if let Err(e) = result {
            return Err(e.context("Failed to connect to host"));
        }

        drop(askpass_task);

        if master_process.try_status()?.is_some() {
            output.clear();
            let mut stderr = master_process.stderr.take().unwrap();
            stderr.read_to_end(&mut output).await?;

            let error_message = format!(
                "failed to connect: {}",
                String::from_utf8_lossy(&output).trim()
            );
            Err(anyhow!(error_message))?;
        }

        let socket = SshSocket {
            connection_options,
            socket_path,
        };

        let mut this = Self {
            socket,
            master_process: Mutex::new(Some(master_process)),
            _temp_dir: temp_dir,
            remote_binary_path: None,
        };

        let (release_channel, version, commit) = cx.update(|cx| {
            (
                ReleaseChannel::global(cx),
                AppVersion::global(cx),
                AppCommitSha::try_global(cx),
            )
        })?;
        this.remote_binary_path = Some(
            this.ensure_server_binary(&delegate, release_channel, version, commit, cx)
                .await?,
        );

        Ok(this)
    }

    async fn platform(&self) -> Result<SshPlatform> {
        let uname = self.socket.run_command("uname", &["-sm"]).await?;
        let Some((os, arch)) = uname.split_once(" ") else {
            Err(anyhow!("unknown uname: {uname:?}"))?
        };

        let os = match os.trim() {
            "Darwin" => "macos",
            "Linux" => "linux",
            _ => Err(anyhow!(
                "Prebuilt remote servers are not yet available for {os:?}. See https://zed.dev/docs/remote-development"
            ))?,
        };
        // exclude armv5,6,7 as they are 32-bit.
        let arch = if arch.starts_with("armv8")
            || arch.starts_with("armv9")
            || arch.starts_with("arm64")
            || arch.starts_with("aarch64")
        {
            "aarch64"
        } else if arch.starts_with("x86") {
            "x86_64"
        } else {
            Err(anyhow!(
                "Prebuilt remote servers are not yet available for {arch:?}. See https://zed.dev/docs/remote-development"
            ))?
        };

        Ok(SshPlatform { os, arch })
    }

    fn multiplex(
        mut ssh_proxy_process: Child,
        incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        mut connection_activity_tx: Sender<()>,
        cx: &AsyncAppContext,
    ) -> Task<Result<i32>> {
        let mut child_stderr = ssh_proxy_process.stderr.take().unwrap();
        let mut child_stdout = ssh_proxy_process.stdout.take().unwrap();
        let mut child_stdin = ssh_proxy_process.stdin.take().unwrap();

        let mut stdin_buffer = Vec::new();
        let mut stdout_buffer = Vec::new();
        let mut stderr_buffer = Vec::new();
        let mut stderr_offset = 0;

        let stdin_task = cx.background_executor().spawn(async move {
            while let Some(outgoing) = outgoing_rx.next().await {
                write_message(&mut child_stdin, &mut stdin_buffer, outgoing).await?;
            }
            anyhow::Ok(())
        });

        let stdout_task = cx.background_executor().spawn({
            let mut connection_activity_tx = connection_activity_tx.clone();
            async move {
                loop {
                    stdout_buffer.resize(MESSAGE_LEN_SIZE, 0);
                    let len = child_stdout.read(&mut stdout_buffer).await?;

                    if len == 0 {
                        return anyhow::Ok(());
                    }

                    if len < MESSAGE_LEN_SIZE {
                        child_stdout.read_exact(&mut stdout_buffer[len..]).await?;
                    }

                    let message_len = message_len_from_buffer(&stdout_buffer);
                    let envelope =
                        read_message_with_len(&mut child_stdout, &mut stdout_buffer, message_len)
                            .await?;
                    connection_activity_tx.try_send(()).ok();
                    incoming_tx.unbounded_send(envelope).ok();
                }
            }
        });

        let stderr_task: Task<anyhow::Result<()>> = cx.background_executor().spawn(async move {
            loop {
                stderr_buffer.resize(stderr_offset + 1024, 0);

                let len = child_stderr
                    .read(&mut stderr_buffer[stderr_offset..])
                    .await?;
                if len == 0 {
                    return anyhow::Ok(());
                }

                stderr_offset += len;
                let mut start_ix = 0;
                while let Some(ix) = stderr_buffer[start_ix..stderr_offset]
                    .iter()
                    .position(|b| b == &b'\n')
                {
                    let line_ix = start_ix + ix;
                    let content = &stderr_buffer[start_ix..line_ix];
                    start_ix = line_ix + 1;
                    if let Ok(record) = serde_json::from_slice::<LogRecord>(content) {
                        record.log(log::logger())
                    } else {
                        eprintln!("(remote) {}", String::from_utf8_lossy(content));
                    }
                }
                stderr_buffer.drain(0..start_ix);
                stderr_offset -= start_ix;

                connection_activity_tx.try_send(()).ok();
            }
        });

        cx.spawn(|_| async move {
            let result = futures::select! {
                result = stdin_task.fuse() => {
                    result.context("stdin")
                }
                result = stdout_task.fuse() => {
                    result.context("stdout")
                }
                result = stderr_task.fuse() => {
                    result.context("stderr")
                }
            };

            let status = ssh_proxy_process.status().await?.code().unwrap_or(1);
            match result {
                Ok(_) => Ok(status),
                Err(error) => Err(error),
            }
        })
    }

    #[allow(unused)]
    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn SshClientDelegate>,
        release_channel: ReleaseChannel,
        version: SemanticVersion,
        commit: Option<AppCommitSha>,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf> {
        let version_str = match release_channel {
            ReleaseChannel::Nightly => {
                let commit = commit.map(|s| s.0.to_string()).unwrap_or_default();

                format!("{}-{}", version, commit)
            }
            ReleaseChannel::Dev => "build".to_string(),
            _ => version.to_string(),
        };
        let binary_name = format!(
            "zed-remote-server-{}-{}",
            release_channel.dev_name(),
            version_str
        );
        let dst_path = paths::remote_server_dir_relative().join(binary_name);
        let tmp_path_gz = PathBuf::from(format!(
            "{}-download-{}.gz",
            dst_path.to_string_lossy(),
            std::process::id()
        ));

        #[cfg(debug_assertions)]
        if std::env::var("ZED_BUILD_REMOTE_SERVER").is_ok() {
            let src_path = self
                .build_local(self.platform().await?, delegate, cx)
                .await?;
            self.upload_local_server_binary(&src_path, &tmp_path_gz, delegate, cx)
                .await?;
            self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .socket
            .run_command(&dst_path.to_string_lossy(), &["version"])
            .await
            .is_ok()
        {
            return Ok(dst_path);
        }

        let wanted_version = cx.update(|cx| match release_channel {
            ReleaseChannel::Nightly => Ok(None),
            ReleaseChannel::Dev => {
                anyhow::bail!(
                    "ZED_BUILD_REMOTE_SERVER is not set and no remote server exists at ({:?})",
                    dst_path
                )
            }
            _ => Ok(Some(AppVersion::global(cx))),
        })??;

        let platform = self.platform().await?;

        if !self.socket.connection_options.upload_binary_over_ssh {
            if let Some((url, body)) = delegate
                .get_download_params(platform, release_channel, wanted_version, cx)
                .await?
            {
                match self
                    .download_binary_on_server(&url, &body, &tmp_path_gz, delegate, cx)
                    .await
                {
                    Ok(_) => {
                        self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
                            .await?;
                        return Ok(dst_path);
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to download binary on server, attempting to upload server: {}",
                            e
                        )
                    }
                }
            }
        }

        let src_path = delegate
            .download_server_binary_locally(platform, release_channel, wanted_version, cx)
            .await?;
        self.upload_local_server_binary(&src_path, &tmp_path_gz, delegate, cx)
            .await?;
        self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
            .await?;
        return Ok(dst_path);
    }

    async fn download_binary_on_server(
        &self,
        url: &str,
        body: &str,
        tmp_path_gz: &Path,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.socket
                .run_command("mkdir", &["-p", &parent.to_string_lossy()])
                .await?;
        }

        delegate.set_status(Some("Downloading remote development server on host"), cx);

        match self
            .socket
            .run_command(
                "curl",
                &[
                    "-f",
                    "-L",
                    "-X",
                    "GET",
                    "-H",
                    "Content-Type: application/json",
                    "-d",
                    &body,
                    &url,
                    "-o",
                    &tmp_path_gz.to_string_lossy(),
                ],
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                if self.socket.run_command("which", &["curl"]).await.is_ok() {
                    return Err(e);
                }

                match self
                    .socket
                    .run_command(
                        "wget",
                        &[
                            "--max-redirect=5",
                            "--method=GET",
                            "--header=Content-Type: application/json",
                            "--body-data",
                            &body,
                            &url,
                            "-O",
                            &tmp_path_gz.to_string_lossy(),
                        ],
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if self.socket.run_command("which", &["wget"]).await.is_ok() {
                            return Err(e);
                        } else {
                            anyhow::bail!("Neither curl nor wget is available");
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn upload_local_server_binary(
        &self,
        src_path: &Path,
        tmp_path_gz: &Path,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.socket
                .run_command("mkdir", &["-p", &parent.to_string_lossy()])
                .await?;
        }

        let src_stat = fs::metadata(&src_path).await?;
        let size = src_stat.len();

        let t0 = Instant::now();
        delegate.set_status(Some("Uploading remote development server"), cx);
        log::info!(
            "uploading remote development server to {:?} ({}kb)",
            tmp_path_gz,
            size / 1024
        );
        self.upload_file(&src_path, &tmp_path_gz)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn extract_server_binary(
        &self,
        dst_path: &Path,
        tmp_path_gz: &Path,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote development server"), cx);
        let server_mode = 0o755;

        let script = shell_script!(
            "gunzip -f {tmp_path_gz} && chmod {server_mode} {tmp_path} && mv {tmp_path} {dst_path}",
            tmp_path_gz = &tmp_path_gz.to_string_lossy(),
            tmp_path = &tmp_path_gz.to_string_lossy().strip_suffix(".gz").unwrap(),
            server_mode = &format!("{:o}", server_mode),
            dst_path = &dst_path.to_string_lossy()
        );
        self.socket.run_command("sh", &["-c", &script]).await?;
        Ok(())
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &Path) -> Result<()> {
        log::debug!("uploading file {:?} to {:?}", src_path, dest_path);
        let mut command = util::command::new_smol_command("scp");
        let output = self
            .socket
            .ssh_options(&mut command)
            .args(
                self.socket
                    .connection_options
                    .port
                    .map(|port| vec!["-P".to_string(), port.to_string()])
                    .unwrap_or_default(),
            )
            .arg(src_path)
            .arg(format!(
                "{}:{}",
                self.socket.connection_options.scp_url(),
                dest_path.display()
            ))
            .output()
            .await?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "failed to upload file {} -> {}: {}",
                src_path.display(),
                dest_path.display(),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    #[cfg(debug_assertions)]
    async fn build_local(
        &self,
        platform: SshPlatform,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf> {
        use smol::process::{Command, Stdio};

        async fn run_cmd(command: &mut Command) -> Result<()> {
            let output = command
                .kill_on_drop(true)
                .stderr(Stdio::inherit())
                .output()
                .await?;
            if !output.status.success() {
                Err(anyhow!("Failed to run command: {:?}", command))?;
            }
            Ok(())
        }

        if platform.arch == std::env::consts::ARCH && platform.os == std::env::consts::OS {
            delegate.set_status(Some("Building remote server binary from source"), cx);
            log::info!("building remote server binary from source");
            run_cmd(Command::new("cargo").args([
                "build",
                "--package",
                "remote_server",
                "--features",
                "debug-embed",
                "--target-dir",
                "target/remote_server",
            ]))
            .await?;

            delegate.set_status(Some("Compressing binary"), cx);

            run_cmd(Command::new("gzip").args([
                "-9",
                "-f",
                "target/remote_server/debug/remote_server",
            ]))
            .await?;

            let path = std::env::current_dir()?.join("target/remote_server/debug/remote_server.gz");
            return Ok(path);
        }
        let Some(triple) = platform.triple() else {
            anyhow::bail!("can't cross compile for: {:?}", platform);
        };
        smol::fs::create_dir_all("target/remote_server").await?;

        delegate.set_status(Some("Installing cross.rs for cross-compilation"), cx);
        log::info!("installing cross");
        run_cmd(Command::new("cargo").args([
            "install",
            "cross",
            "--git",
            "https://github.com/cross-rs/cross",
        ]))
        .await?;

        delegate.set_status(
            Some(&format!(
                "Building remote server binary from source for {} with Docker",
                &triple
            )),
            cx,
        );
        log::info!("building remote server binary from source for {}", &triple);
        run_cmd(
            Command::new("cross")
                .args([
                    "build",
                    "--package",
                    "remote_server",
                    "--features",
                    "debug-embed",
                    "--target-dir",
                    "target/remote_server",
                    "--target",
                    &triple,
                ])
                .env(
                    "CROSS_CONTAINER_OPTS",
                    "--mount type=bind,src=./target,dst=/app/target",
                ),
        )
        .await?;

        delegate.set_status(Some("Compressing binary"), cx);

        run_cmd(Command::new("gzip").args([
            "-9",
            "-f",
            &format!("target/remote_server/{}/debug/remote_server", triple),
        ]))
        .await?;

        let path = std::env::current_dir()?.join(format!(
            "target/remote_server/{}/debug/remote_server.gz",
            triple
        ));

        return Ok(path);
    }
}

type ResponseChannels = Mutex<HashMap<MessageId, oneshot::Sender<(Envelope, oneshot::Sender<()>)>>>;

pub struct ChannelClient {
    next_message_id: AtomicU32,
    outgoing_tx: Mutex<mpsc::UnboundedSender<Envelope>>,
    buffer: Mutex<VecDeque<Envelope>>,
    response_channels: ResponseChannels,
    message_handlers: Mutex<ProtoMessageHandlerSet>,
    max_received: AtomicU32,
    name: &'static str,
    task: Mutex<Task<Result<()>>>,
}

impl ChannelClient {
    pub fn new(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &AppContext,
        name: &'static str,
    ) -> Arc<Self> {
        Arc::new_cyclic(|this| Self {
            outgoing_tx: Mutex::new(outgoing_tx),
            next_message_id: AtomicU32::new(0),
            max_received: AtomicU32::new(0),
            response_channels: ResponseChannels::default(),
            message_handlers: Default::default(),
            buffer: Mutex::new(VecDeque::new()),
            name,
            task: Mutex::new(Self::start_handling_messages(
                this.clone(),
                incoming_rx,
                &cx.to_async(),
            )),
        })
    }

    fn start_handling_messages(
        this: Weak<Self>,
        mut incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        cx: &AsyncAppContext,
    ) -> Task<Result<()>> {
        cx.spawn(|cx| async move {
            let peer_id = PeerId { owner_id: 0, id: 0 };
            while let Some(incoming) = incoming_rx.next().await {
                let Some(this) = this.upgrade() else {
                    return anyhow::Ok(());
                };
                if let Some(ack_id) = incoming.ack_id {
                    let mut buffer = this.buffer.lock();
                    while buffer.front().is_some_and(|msg| msg.id <= ack_id) {
                        buffer.pop_front();
                    }
                }
                if let Some(proto::envelope::Payload::FlushBufferedMessages(_)) = &incoming.payload
                {
                    log::debug!(
                        "{}:ssh message received. name:FlushBufferedMessages",
                        this.name
                    );
                    {
                        let buffer = this.buffer.lock();
                        for envelope in buffer.iter() {
                            this.outgoing_tx
                                .lock()
                                .unbounded_send(envelope.clone())
                                .ok();
                        }
                    }
                    let mut envelope = proto::Ack {}.into_envelope(0, Some(incoming.id), None);
                    envelope.id = this.next_message_id.fetch_add(1, SeqCst);
                    this.outgoing_tx.lock().unbounded_send(envelope).ok();
                    continue;
                }

                this.max_received.store(incoming.id, SeqCst);

                if let Some(request_id) = incoming.responding_to {
                    let request_id = MessageId(request_id);
                    let sender = this.response_channels.lock().remove(&request_id);
                    if let Some(sender) = sender {
                        let (tx, rx) = oneshot::channel();
                        if incoming.payload.is_some() {
                            sender.send((incoming, tx)).ok();
                        }
                        rx.await.ok();
                    }
                } else if let Some(envelope) =
                    build_typed_envelope(peer_id, Instant::now(), incoming)
                {
                    let type_name = envelope.payload_type_name();
                    if let Some(future) = ProtoMessageHandlerSet::handle_message(
                        &this.message_handlers,
                        envelope,
                        this.clone().into(),
                        cx.clone(),
                    ) {
                        log::debug!("{}:ssh message received. name:{type_name}", this.name);
                        cx.foreground_executor()
                            .spawn(async move {
                                match future.await {
                                    Ok(_) => {
                                        log::debug!(
                                            "{}:ssh message handled. name:{type_name}",
                                            this.name
                                        );
                                    }
                                    Err(error) => {
                                        log::error!(
                                            "{}:error handling message. type:{}, error:{}",
                                            this.name,
                                            type_name,
                                            format!("{error:#}").lines().fold(
                                                String::new(),
                                                |mut message, line| {
                                                    if !message.is_empty() {
                                                        message.push(' ');
                                                    }
                                                    message.push_str(line);
                                                    message
                                                }
                                            )
                                        );
                                    }
                                }
                            })
                            .detach()
                    } else {
                        log::error!("{}:unhandled ssh message name:{type_name}", this.name);
                    }
                }
            }
            anyhow::Ok(())
        })
    }

    pub fn reconnect(
        self: &Arc<Self>,
        incoming_rx: UnboundedReceiver<Envelope>,
        outgoing_tx: UnboundedSender<Envelope>,
        cx: &AsyncAppContext,
    ) {
        *self.outgoing_tx.lock() = outgoing_tx;
        *self.task.lock() = Self::start_handling_messages(Arc::downgrade(self), incoming_rx, cx);
    }

    pub fn subscribe_to_entity<E: 'static>(&self, remote_id: u64, entity: &Model<E>) {
        let id = (TypeId::of::<E>(), remote_id);

        let mut message_handlers = self.message_handlers.lock();
        if message_handlers
            .entities_by_type_and_remote_id
            .contains_key(&id)
        {
            panic!("already subscribed to entity");
        }

        message_handlers.entities_by_type_and_remote_id.insert(
            id,
            EntityMessageSubscriber::Entity {
                handle: entity.downgrade().into(),
            },
        );
    }

    pub fn request<T: RequestMessage>(
        &self,
        payload: T,
    ) -> impl 'static + Future<Output = Result<T::Response>> {
        self.request_internal(payload, true)
    }

    fn request_internal<T: RequestMessage>(
        &self,
        payload: T,
        use_buffer: bool,
    ) -> impl 'static + Future<Output = Result<T::Response>> {
        log::debug!("ssh request start. name:{}", T::NAME);
        let response =
            self.request_dynamic(payload.into_envelope(0, None, None), T::NAME, use_buffer);
        async move {
            let response = response.await?;
            log::debug!("ssh request finish. name:{}", T::NAME);
            T::Response::from_envelope(response)
                .ok_or_else(|| anyhow!("received a response of the wrong type"))
        }
    }

    pub async fn resync(&self, timeout: Duration) -> Result<()> {
        smol::future::or(
            async {
                self.request_internal(proto::FlushBufferedMessages {}, false)
                    .await?;

                for envelope in self.buffer.lock().iter() {
                    self.outgoing_tx
                        .lock()
                        .unbounded_send(envelope.clone())
                        .ok();
                }
                Ok(())
            },
            async {
                smol::Timer::after(timeout).await;
                Err(anyhow!("Timeout detected"))
            },
        )
        .await
    }

    pub async fn ping(&self, timeout: Duration) -> Result<()> {
        smol::future::or(
            async {
                self.request(proto::Ping {}).await?;
                Ok(())
            },
            async {
                smol::Timer::after(timeout).await;
                Err(anyhow!("Timeout detected"))
            },
        )
        .await
    }

    pub fn send<T: EnvelopedMessage>(&self, payload: T) -> Result<()> {
        log::debug!("ssh send name:{}", T::NAME);
        self.send_dynamic(payload.into_envelope(0, None, None))
    }

    fn request_dynamic(
        &self,
        mut envelope: proto::Envelope,
        type_name: &'static str,
        use_buffer: bool,
    ) -> impl 'static + Future<Output = Result<proto::Envelope>> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        let (tx, rx) = oneshot::channel();
        let mut response_channels_lock = self.response_channels.lock();
        response_channels_lock.insert(MessageId(envelope.id), tx);
        drop(response_channels_lock);

        let result = if use_buffer {
            self.send_buffered(envelope)
        } else {
            self.send_unbuffered(envelope)
        };
        async move {
            if let Err(error) = &result {
                log::error!("failed to send message: {}", error);
                return Err(anyhow!("failed to send message: {}", error));
            }

            let response = rx.await.context("connection lost")?.0;
            if let Some(proto::envelope::Payload::Error(error)) = &response.payload {
                return Err(RpcError::from_proto(error, type_name));
            }
            Ok(response)
        }
    }

    pub fn send_dynamic(&self, mut envelope: proto::Envelope) -> Result<()> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        self.send_buffered(envelope)
    }

    fn send_buffered(&self, mut envelope: proto::Envelope) -> Result<()> {
        envelope.ack_id = Some(self.max_received.load(SeqCst));
        self.buffer.lock().push_back(envelope.clone());
        // ignore errors on send (happen while we're reconnecting)
        // assume that the global "disconnected" overlay is sufficient.
        self.outgoing_tx.lock().unbounded_send(envelope).ok();
        Ok(())
    }

    fn send_unbuffered(&self, mut envelope: proto::Envelope) -> Result<()> {
        envelope.ack_id = Some(self.max_received.load(SeqCst));
        self.outgoing_tx.lock().unbounded_send(envelope).ok();
        Ok(())
    }
}

impl ProtoClient for ChannelClient {
    fn request(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>> {
        self.request_dynamic(envelope, request_type, true).boxed()
    }

    fn send(&self, envelope: proto::Envelope, _message_type: &'static str) -> Result<()> {
        self.send_dynamic(envelope)
    }

    fn send_response(&self, envelope: Envelope, _message_type: &'static str) -> anyhow::Result<()> {
        self.send_dynamic(envelope)
    }

    fn message_handler_set(&self) -> &Mutex<ProtoMessageHandlerSet> {
        &self.message_handlers
    }

    fn is_via_collab(&self) -> bool {
        false
    }
}

#[cfg(any(test, feature = "test-support"))]
mod fake {
    use std::{path::PathBuf, sync::Arc};

    use anyhow::Result;
    use async_trait::async_trait;
    use futures::{
        channel::{
            mpsc::{self, Sender},
            oneshot,
        },
        select_biased, FutureExt, SinkExt, StreamExt,
    };
    use gpui::{AppContext, AsyncAppContext, SemanticVersion, Task, TestAppContext};
    use release_channel::ReleaseChannel;
    use rpc::proto::Envelope;

    use super::{
        ChannelClient, RemoteConnection, SshClientDelegate, SshConnectionOptions, SshPlatform,
    };

    pub(super) struct FakeRemoteConnection {
        pub(super) connection_options: SshConnectionOptions,
        pub(super) server_channel: Arc<ChannelClient>,
        pub(super) server_cx: SendableCx,
    }

    pub(super) struct SendableCx(AsyncAppContext);
    impl SendableCx {
        // SAFETY: When run in test mode, GPUI is always single threaded.
        pub(super) fn new(cx: &TestAppContext) -> Self {
            Self(cx.to_async())
        }

        // SAFETY: Enforce that we're on the main thread by requiring a valid AsyncAppContext
        fn get(&self, _: &AsyncAppContext) -> AsyncAppContext {
            self.0.clone()
        }
    }

    // SAFETY: There is no way to access a SendableCx from a different thread, see [`SendableCx::new`] and [`SendableCx::get`]
    unsafe impl Send for SendableCx {}
    unsafe impl Sync for SendableCx {}

    #[async_trait(?Send)]
    impl RemoteConnection for FakeRemoteConnection {
        async fn kill(&self) -> Result<()> {
            Ok(())
        }

        fn has_been_killed(&self) -> bool {
            false
        }

        fn ssh_args(&self) -> Vec<String> {
            Vec::new()
        }
        fn upload_directory(
            &self,
            _src_path: PathBuf,
            _dest_path: PathBuf,
            _cx: &AppContext,
        ) -> Task<Result<()>> {
            unreachable!()
        }

        fn connection_options(&self) -> SshConnectionOptions {
            self.connection_options.clone()
        }

        fn simulate_disconnect(&self, cx: &AsyncAppContext) {
            let (outgoing_tx, _) = mpsc::unbounded::<Envelope>();
            let (_, incoming_rx) = mpsc::unbounded::<Envelope>();
            self.server_channel
                .reconnect(incoming_rx, outgoing_tx, &self.server_cx.get(&cx));
        }

        fn start_proxy(
            &self,

            _unique_identifier: String,
            _reconnect: bool,
            mut client_incoming_tx: mpsc::UnboundedSender<Envelope>,
            mut client_outgoing_rx: mpsc::UnboundedReceiver<Envelope>,
            mut connection_activity_tx: Sender<()>,
            _delegate: Arc<dyn SshClientDelegate>,
            cx: &mut AsyncAppContext,
        ) -> Task<Result<i32>> {
            let (mut server_incoming_tx, server_incoming_rx) = mpsc::unbounded::<Envelope>();
            let (server_outgoing_tx, mut server_outgoing_rx) = mpsc::unbounded::<Envelope>();

            self.server_channel.reconnect(
                server_incoming_rx,
                server_outgoing_tx,
                &self.server_cx.get(cx),
            );

            cx.background_executor().spawn(async move {
                loop {
                    select_biased! {
                        server_to_client = server_outgoing_rx.next().fuse() => {
                            let Some(server_to_client) = server_to_client else {
                                return Ok(1)
                            };
                            connection_activity_tx.try_send(()).ok();
                            client_incoming_tx.send(server_to_client).await.ok();
                        }
                        client_to_server = client_outgoing_rx.next().fuse() => {
                            let Some(client_to_server) = client_to_server else {
                                return Ok(1)
                            };
                            server_incoming_tx.send(client_to_server).await.ok();
                        }
                    }
                }
            })
        }
    }

    pub(super) struct Delegate;

    impl SshClientDelegate for Delegate {
        fn ask_password(
            &self,
            _: String,
            _: &mut AsyncAppContext,
        ) -> oneshot::Receiver<Result<String>> {
            unreachable!()
        }

        fn download_server_binary_locally(
            &self,
            _: SshPlatform,
            _: ReleaseChannel,
            _: Option<SemanticVersion>,
            _: &mut AsyncAppContext,
        ) -> Task<Result<PathBuf>> {
            unreachable!()
        }

        fn get_download_params(
            &self,
            _platform: SshPlatform,
            _release_channel: ReleaseChannel,
            _version: Option<SemanticVersion>,
            _cx: &mut AsyncAppContext,
        ) -> Task<Result<Option<(String, String)>>> {
            unreachable!()
        }

        fn set_status(&self, _: Option<&str>, _: &mut AsyncAppContext) {}
    }
}
