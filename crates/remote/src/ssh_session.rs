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
use parking_lot::Mutex;
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
    ffi::OsStr,
    fmt,
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, Ordering::SeqCst},
        Arc, Weak,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
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

    // Uniquely identifies dev server projects on a remote host. Needs to be
    // stable for the same dev server project.
    pub fn remote_server_identifier(&self) -> String {
        let mut identifier = format!("dev-server-{:?}", self.host);
        if let Some(username) = self.username.as_ref() {
            identifier.push('-');
            identifier.push_str(&username);
        }
        identifier
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

pub enum ServerBinary {
    LocalBinary(PathBuf),
    ReleaseUrl { url: String, body: String },
}

pub trait SshClientDelegate: Send + Sync {
    fn ask_password(
        &self,
        prompt: String,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<String>>;
    fn remote_server_binary_path(
        &self,
        platform: SshPlatform,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf>;
    fn get_server_binary(
        &self,
        platform: SshPlatform,
        upload_binary_over_ssh: bool,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<(ServerBinary, SemanticVersion)>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncAppContext);
}

impl SshSocket {
    fn ssh_command<S: AsRef<OsStr>>(&self, program: S) -> process::Command {
        let mut command = process::Command::new("ssh");
        self.ssh_options(&mut command)
            .arg(self.connection_options.ssh_url())
            .arg(program);
        command
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

async fn run_cmd(command: &mut process::Command) -> Result<String> {
    let output = command.output().await?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!(
            "failed to run command: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
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

impl SshRemoteClient {
    pub fn new(
        unique_identifier: String,
        connection_options: SshConnectionOptions,
        cancellation: oneshot::Receiver<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AppContext,
    ) -> Task<Result<Option<Model<Self>>>> {
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
                let remote_binary_path = ssh_connection
                    .get_remote_binary_path(&delegate, false, &mut cx)
                    .await?;

                let io_task = ssh_connection.start_proxy(
                    remote_binary_path,
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

                let remote_binary_path = ssh_connection
                    .get_remote_binary_path(&delegate, true, &mut cx)
                    .await?;

                let io_task = ssh_connection.start_proxy(
                    remote_binary_path,
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
            server_cx: fake::SendableCx::new(server_cx.to_async()),
            server_channel: server_client.clone(),
        });

        client_cx.update(|cx| {
            cx.update_default_global(|c: &mut ConnectionPool, cx| {
                c.connections.insert(
                    opts.clone(),
                    ConnectionPoolEntry::Connecting(
                        cx.foreground_executor()
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
            .update(|cx| Self::new("fake".to_string(), opts, rx, Arc::new(fake::Delegate), cx))
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
        remote_binary_path: PathBuf,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<i32>>;
    async fn get_remote_binary_path(
        &self,
        delegate: &Arc<dyn SshClientDelegate>,
        reconnect: bool,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf>;
    async fn kill(&self) -> Result<()>;
    fn has_been_killed(&self) -> bool;
    fn ssh_args(&self) -> Vec<String>;
    fn connection_options(&self) -> SshConnectionOptions;

    #[cfg(any(test, feature = "test-support"))]
    fn simulate_disconnect(&self, _: &AsyncAppContext) {}
}

struct SshRemoteConnection {
    socket: SshSocket,
    master_process: Mutex<Option<process::Child>>,
    platform: SshPlatform,
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

    async fn get_remote_binary_path(
        &self,
        delegate: &Arc<dyn SshClientDelegate>,
        reconnect: bool,
        cx: &mut AsyncAppContext,
    ) -> Result<PathBuf> {
        let platform = self.platform;
        let remote_binary_path = delegate.remote_server_binary_path(platform, cx)?;
        if !reconnect {
            self.ensure_server_binary(&delegate, &remote_binary_path, platform, cx)
                .await?;
        }

        let socket = self.socket.clone();
        run_cmd(socket.ssh_command(&remote_binary_path).arg("version")).await?;
        Ok(remote_binary_path)
    }

    fn start_proxy(
        &self,
        remote_binary_path: PathBuf,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<i32>> {
        delegate.set_status(Some("Starting proxy"), cx);

        let mut start_proxy_command = format!(
            "RUST_LOG={} RUST_BACKTRACE={} {:?} proxy --identifier {}",
            std::env::var("RUST_LOG").unwrap_or_default(),
            std::env::var("RUST_BACKTRACE").unwrap_or_default(),
            remote_binary_path,
            unique_identifier,
        );
        if reconnect {
            start_proxy_command.push_str(" --reconnect");
        }

        let ssh_proxy_process = match self
            .socket
            .ssh_command(start_proxy_command)
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
                    }
                }
            }
        });

        // Create an askpass script that communicates back to this process.
        let askpass_script = format!(
            "{shebang}\n{print_args} | nc -U {askpass_socket} 2> /dev/null \n",
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
        let stdout = master_process.stdout.as_mut().unwrap();
        let mut output = Vec::new();
        let connection_timeout = Duration::from_secs(10);

        let result = select_biased! {
            _ = askpass_opened_rx.fuse() => {
                // If the askpass script has opened, that means the user is typing
                // their password, in which case we don't want to timeout anymore,
                // since we know a connection has been established.
                stdout.read_to_end(&mut output).await?;
                Ok(())
            }
            result = stdout.read_to_end(&mut output).fuse() => {
                result?;
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

        let os = run_cmd(socket.ssh_command("uname").arg("-s")).await?;
        let arch = run_cmd(socket.ssh_command("uname").arg("-m")).await?;

        let os = match os.trim() {
            "Darwin" => "macos",
            "Linux" => "linux",
            _ => Err(anyhow!("unknown uname os {os:?}"))?,
        };
        let arch = if arch.starts_with("arm") || arch.starts_with("aarch64") {
            "aarch64"
        } else if arch.starts_with("x86") || arch.starts_with("i686") {
            "x86_64"
        } else {
            Err(anyhow!("unknown uname architecture {arch:?}"))?
        };

        let platform = SshPlatform { os, arch };

        Ok(Self {
            socket,
            master_process: Mutex::new(Some(master_process)),
            platform,
            _temp_dir: temp_dir,
        })
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

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn SshClientDelegate>,
        dst_path: &Path,
        platform: SshPlatform,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let lock_file = dst_path.with_extension("lock");
        let lock_content = {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("failed to get timestamp")?
                .as_secs();
            let source_port = self.get_ssh_source_port().await?;
            format!("{} {}", source_port, timestamp)
        };

        let lock_stale_age = Duration::from_secs(10 * 60);
        let max_wait_time = Duration::from_secs(10 * 60);
        let check_interval = Duration::from_secs(5);
        let start_time = Instant::now();

        loop {
            let lock_acquired = self.create_lock_file(&lock_file, &lock_content).await?;
            if lock_acquired {
                delegate.set_status(Some("Acquired lock file on host"), cx);
                let result = self
                    .update_server_binary_if_needed(delegate, dst_path, platform, cx)
                    .await;

                self.remove_lock_file(&lock_file).await.ok();

                return result;
            } else {
                if let Ok(is_stale) = self.is_lock_stale(&lock_file, &lock_stale_age).await {
                    if is_stale {
                        delegate.set_status(
                            Some("Detected lock file on host being stale. Removing"),
                            cx,
                        );
                        self.remove_lock_file(&lock_file).await?;
                        continue;
                    } else {
                        if start_time.elapsed() > max_wait_time {
                            return Err(anyhow!("Timeout waiting for lock to be released"));
                        }
                        log::info!(
                            "Found lockfile: {:?}. Will check again in {:?}",
                            lock_file,
                            check_interval
                        );
                        delegate.set_status(
                            Some("Waiting for another Zed instance to finish uploading binary"),
                            cx,
                        );
                        smol::Timer::after(check_interval).await;
                        continue;
                    }
                } else {
                    // Unable to check lock, assume it's valid and wait
                    if start_time.elapsed() > max_wait_time {
                        return Err(anyhow!("Timeout waiting for lock to be released"));
                    }
                    smol::Timer::after(check_interval).await;
                    continue;
                }
            }
        }
    }

    async fn get_ssh_source_port(&self) -> Result<String> {
        let output = run_cmd(
            self.socket
                .ssh_command("sh")
                .arg("-c")
                .arg(r#""echo $SSH_CLIENT | cut -d' ' -f2""#),
        )
        .await
        .context("failed to get source port from SSH_CLIENT on host")?;

        Ok(output.trim().to_string())
    }

    async fn create_lock_file(&self, lock_file: &Path, content: &str) -> Result<bool> {
        let parent_dir = lock_file
            .parent()
            .ok_or_else(|| anyhow!("Lock file path has no parent directory"))?;

        let script = format!(
            r#"'mkdir -p "{parent_dir}" && [ ! -f "{lock_file}" ] && echo "{content}" > "{lock_file}" && echo "created" || echo "exists"'"#,
            parent_dir = parent_dir.display(),
            lock_file = lock_file.display(),
            content = content,
        );

        let output = run_cmd(self.socket.ssh_command("sh").arg("-c").arg(&script))
            .await
            .with_context(|| format!("failed to create a lock file at {:?}", lock_file))?;

        Ok(output.trim() == "created")
    }

    fn generate_stale_check_script(lock_file: &Path, max_age: u64) -> String {
        format!(
            r#"
            if [ ! -f "{lock_file}" ]; then
                echo "lock file does not exist"
                exit 0
            fi

            read -r port timestamp < "{lock_file}"

            # Check if port is still active
            if command -v ss >/dev/null 2>&1; then
                if ! ss -n | grep -q ":$port[[:space:]]"; then
                    echo "ss reports port $port is not open"
                    exit 0
                fi
            elif command -v netstat >/dev/null 2>&1; then
                if ! netstat -n | grep -q ":$port[[:space:]]"; then
                    echo "netstat reports port $port is not open"
                    exit 0
                fi
            fi

            # Check timestamp
            if [ $(( $(date +%s) - timestamp )) -gt {max_age} ]; then
                echo "timestamp in lockfile is too old"
            else
                echo "recent"
            fi"#,
            lock_file = lock_file.display(),
            max_age = max_age
        )
    }

    async fn is_lock_stale(&self, lock_file: &Path, max_age: &Duration) -> Result<bool> {
        let script = format!(
            "'{}'",
            Self::generate_stale_check_script(lock_file, max_age.as_secs())
        );

        let output = run_cmd(self.socket.ssh_command("sh").arg("-c").arg(&script))
            .await
            .with_context(|| {
                format!("failed to check whether lock file {:?} is stale", lock_file)
            })?;

        let trimmed = output.trim();
        let is_stale = trimmed != "recent";
        log::info!("checked lockfile for staleness. stale: {is_stale}, output: {trimmed:?}");
        Ok(is_stale)
    }

    async fn remove_lock_file(&self, lock_file: &Path) -> Result<()> {
        run_cmd(self.socket.ssh_command("rm").arg("-f").arg(lock_file))
            .await
            .context("failed to remove lock file")?;
        Ok(())
    }

    async fn update_server_binary_if_needed(
        &self,
        delegate: &Arc<dyn SshClientDelegate>,
        dst_path: &Path,
        platform: SshPlatform,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        if std::env::var("ZED_USE_CACHED_REMOTE_SERVER").is_ok() {
            if let Ok(installed_version) =
                run_cmd(self.socket.ssh_command(dst_path).arg("version")).await
            {
                log::info!("using cached server binary version {}", installed_version);
                return Ok(());
            }
        }

        if self.is_binary_in_use(dst_path).await? {
            log::info!("server binary is opened by another process. not updating");
            delegate.set_status(
                Some("Skipping update of remote development server, since it's still in use"),
                cx,
            );
            return Ok(());
        }

        let upload_binary_over_ssh = self.socket.connection_options.upload_binary_over_ssh;
        let (binary, version) = delegate
            .get_server_binary(platform, upload_binary_over_ssh, cx)
            .await??;

        let mut remote_version = None;
        if cfg!(not(debug_assertions)) {
            if let Ok(installed_version) =
                run_cmd(self.socket.ssh_command(dst_path).arg("version")).await
            {
                if let Ok(version) = installed_version.trim().parse::<SemanticVersion>() {
                    remote_version = Some(version);
                } else {
                    log::warn!("failed to parse version of remote server: {installed_version:?}",);
                }
            }

            if let Some(remote_version) = remote_version {
                if remote_version == version {
                    log::info!("remote development server present and matching client version");
                    return Ok(());
                } else if remote_version > version {
                    let error = anyhow!("The version of the remote server ({}) is newer than the Zed version ({}). Please update Zed.", remote_version, version);
                    return Err(error);
                } else {
                    log::info!(
                        "remote development server has older version: {}. updating...",
                        remote_version
                    );
                }
            }
        }

        match binary {
            ServerBinary::LocalBinary(src_path) => {
                self.upload_local_server_binary(&src_path, dst_path, delegate, cx)
                    .await
            }
            ServerBinary::ReleaseUrl { url, body } => {
                self.download_binary_on_server(&url, &body, dst_path, delegate, cx)
                    .await
            }
        }
    }

    async fn is_binary_in_use(&self, binary_path: &Path) -> Result<bool> {
        let script = format!(
            r#"'
            if command -v lsof >/dev/null 2>&1; then
                if lsof "{}" >/dev/null 2>&1; then
                    echo "in_use"
                    exit 0
                fi
            elif command -v fuser >/dev/null 2>&1; then
                if fuser "{}" >/dev/null 2>&1; then
                    echo "in_use"
                    exit 0
                fi
            fi
            echo "not_in_use"
            '"#,
            binary_path.display(),
            binary_path.display(),
        );

        let output = run_cmd(self.socket.ssh_command("sh").arg("-c").arg(script))
            .await
            .context("failed to check if binary is in use")?;

        Ok(output.trim() == "in_use")
    }

    async fn download_binary_on_server(
        &self,
        url: &str,
        body: &str,
        dst_path: &Path,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let mut dst_path_gz = dst_path.to_path_buf();
        dst_path_gz.set_extension("gz");

        if let Some(parent) = dst_path.parent() {
            run_cmd(self.socket.ssh_command("mkdir").arg("-p").arg(parent)).await?;
        }

        delegate.set_status(Some("Downloading remote development server on host"), cx);

        let script = format!(
            r#"
            if command -v wget >/dev/null 2>&1; then
                wget --max-redirect=5 --method=GET --header="Content-Type: application/json" --body-data='{}' '{}' -O '{}' && echo "wget"
            elif command -v curl >/dev/null 2>&1; then
                curl -L -X GET -H "Content-Type: application/json" -d '{}' '{}' -o '{}' && echo "curl"
            else
                echo "Neither curl nor wget is available" >&2
                exit 1
            fi
            "#,
            body.replace("'", r#"\'"#),
            url,
            dst_path_gz.display(),
            body.replace("'", r#"\'"#),
            url,
            dst_path_gz.display(),
        );

        let output = run_cmd(self.socket.ssh_command("bash").arg("-c").arg(script))
            .await
            .context("Failed to download server binary")?;

        if !output.contains("curl") && !output.contains("wget") {
            return Err(anyhow!("Failed to download server binary: {}", output));
        }

        self.extract_server_binary(dst_path, &dst_path_gz, delegate, cx)
            .await
    }

    async fn upload_local_server_binary(
        &self,
        src_path: &Path,
        dst_path: &Path,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let mut dst_path_gz = dst_path.to_path_buf();
        dst_path_gz.set_extension("gz");

        if let Some(parent) = dst_path.parent() {
            run_cmd(self.socket.ssh_command("mkdir").arg("-p").arg(parent)).await?;
        }

        let src_stat = fs::metadata(&src_path).await?;
        let size = src_stat.len();

        let t0 = Instant::now();
        delegate.set_status(Some("Uploading remote development server"), cx);
        log::info!("uploading remote development server ({}kb)", size / 1024);
        self.upload_file(&src_path, &dst_path_gz)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());

        self.extract_server_binary(dst_path, &dst_path_gz, delegate, cx)
            .await
    }

    async fn extract_server_binary(
        &self,
        dst_path: &Path,
        dst_path_gz: &Path,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote development server"), cx);
        run_cmd(
            self.socket
                .ssh_command("gunzip")
                .arg("--force")
                .arg(&dst_path_gz),
        )
        .await?;

        let server_mode = 0o755;
        delegate.set_status(Some("Marking remote development server executable"), cx);
        run_cmd(
            self.socket
                .ssh_command("chmod")
                .arg(format!("{:o}", server_mode))
                .arg(dst_path),
        )
        .await?;

        Ok(())
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &Path) -> Result<()> {
        let mut command = process::Command::new("scp");
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
        cx.spawn(|cx| {
            async move {
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
                    if let Some(proto::envelope::Payload::FlushBufferedMessages(_)) =
                        &incoming.payload
                    {
                        log::debug!("{}:ssh message received. name:FlushBufferedMessages", this.name);
                        {
                            let buffer = this.buffer.lock();
                            for envelope in buffer.iter() {
                                this.outgoing_tx.lock().unbounded_send(envelope.clone()).ok();
                            }
                        }
                        let mut envelope = proto::Ack{}.into_envelope(0, Some(incoming.id), None);
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
                            cx.foreground_executor().spawn(async move {
                                match future.await {
                                    Ok(_) => {
                                        log::debug!("{}:ssh message handled. name:{type_name}", this.name);
                                    }
                                    Err(error) => {
                                        log::error!(
                                            "{}:error handling message. type:{type_name}, error:{error}", this.name,
                                        );
                                    }
                                }
                            }).detach()
                        } else {
                            log::error!("{}:unhandled ssh message name:{type_name}", this.name);
                        }
                    }
                }
                anyhow::Ok(())
            }
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
    use gpui::{AsyncAppContext, SemanticVersion, Task};
    use rpc::proto::Envelope;

    use super::{
        ChannelClient, RemoteConnection, ServerBinary, SshClientDelegate, SshConnectionOptions,
        SshPlatform,
    };

    pub(super) struct FakeRemoteConnection {
        pub(super) connection_options: SshConnectionOptions,
        pub(super) server_channel: Arc<ChannelClient>,
        pub(super) server_cx: SendableCx,
    }

    pub(super) struct SendableCx(AsyncAppContext);
    // safety: you can only get the other cx on the main thread.
    impl SendableCx {
        pub(super) fn new(cx: AsyncAppContext) -> Self {
            Self(cx)
        }
        fn get(&self, _: &AsyncAppContext) -> AsyncAppContext {
            self.0.clone()
        }
    }
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

        fn connection_options(&self) -> SshConnectionOptions {
            self.connection_options.clone()
        }

        fn simulate_disconnect(&self, cx: &AsyncAppContext) {
            let (outgoing_tx, _) = mpsc::unbounded::<Envelope>();
            let (_, incoming_rx) = mpsc::unbounded::<Envelope>();
            self.server_channel
                .reconnect(incoming_rx, outgoing_tx, &self.server_cx.get(&cx));
        }

        async fn get_remote_binary_path(
            &self,
            _delegate: &Arc<dyn SshClientDelegate>,
            _reconnect: bool,
            _cx: &mut AsyncAppContext,
        ) -> Result<PathBuf> {
            Ok(PathBuf::new())
        }

        fn start_proxy(
            &self,
            _remote_binary_path: PathBuf,
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
        fn remote_server_binary_path(
            &self,
            _: SshPlatform,
            _: &mut AsyncAppContext,
        ) -> Result<PathBuf> {
            unreachable!()
        }
        fn get_server_binary(
            &self,
            _: SshPlatform,
            _: bool,
            _: &mut AsyncAppContext,
        ) -> oneshot::Receiver<Result<(ServerBinary, SemanticVersion)>> {
            unreachable!()
        }

        fn set_status(&self, _: Option<&str>, _: &mut AsyncAppContext) {}
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn run_stale_check_script(
        lock_file: &Path,
        max_age: Duration,
        simulate_port_open: Option<&str>,
    ) -> Result<String> {
        let wrapper = format!(
            r#"
            # Mock ss/netstat commands
            ss() {{
                # Only handle the -n argument
                if [ "$1" = "-n" ]; then
                    # If we're simulating an open port, output a line containing that port
                    if [ "{simulated_port}" != "" ]; then
                        echo "ESTAB 0 0 1.2.3.4:{simulated_port} 5.6.7.8:12345"
                    fi
                fi
            }}
            netstat() {{
                ss "$@"
            }}
            export -f ss netstat

            # Real script starts here
            {script}"#,
            simulated_port = simulate_port_open.unwrap_or(""),
            script = SshRemoteConnection::generate_stale_check_script(lock_file, max_age.as_secs())
        );

        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(&wrapper)
            .output()?;

        if !output.stderr.is_empty() {
            eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    #[test]
    fn test_lock_staleness() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let lock_file = temp_dir.path().join("test.lock");

        // Test 1: No lock file
        let output = run_stale_check_script(&lock_file, Duration::from_secs(600), None)?;
        assert_eq!(output, "lock file does not exist");

        // Test 2: Lock file with port that's not open
        fs::write(&lock_file, "54321 1234567890")?;
        let output = run_stale_check_script(&lock_file, Duration::from_secs(600), Some("98765"))?;
        assert_eq!(output, "ss reports port 54321 is not open");

        // Test 3: Lock file with port that is open but old timestamp
        let old_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - 700; // 700 seconds ago
        fs::write(&lock_file, format!("54321 {}", old_timestamp))?;
        let output = run_stale_check_script(&lock_file, Duration::from_secs(600), Some("54321"))?;
        assert_eq!(output, "timestamp in lockfile is too old");

        // Test 4: Lock file with port that is open and recent timestamp
        let recent_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - 60; // 1 minute ago
        fs::write(&lock_file, format!("54321 {}", recent_timestamp))?;
        let output = run_stale_check_script(&lock_file, Duration::from_secs(600), Some("54321"))?;
        assert_eq!(output, "recent");

        Ok(())
    }
}
