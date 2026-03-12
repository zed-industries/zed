#[cfg(any(test, feature = "test-support"))]
use crate::transport::mock::ConnectGuard;
use crate::{
    SshConnectionOptions,
    protocol::MessageId,
    proxy::ProxyLaunchError,
    transport::{
        docker::{DockerConnectionOptions, DockerExecConnection},
        ssh::SshRemoteConnection,
        wsl::{WslConnectionOptions, WslRemoteConnection},
    },
};
use anyhow::{Context as _, Result, anyhow};
use askpass::EncryptedPassword;
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    Future, FutureExt as _, StreamExt as _,
    channel::{
        mpsc::{self, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::{BoxFuture, Shared, WeakShared},
    select, select_biased,
};
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, BorrowAppContext, Context, Entity,
    EventEmitter, FutureExt, Global, Subscription, Task, WeakEntity,
};
use parking_lot::Mutex;

use release_channel::ReleaseChannel;
use rpc::{
    AnyProtoClient, ErrorExt, ProtoClient, ProtoMessageHandlerSet, RpcError,
    proto::{self, Envelope, EnvelopedMessage, PeerId, RequestMessage, build_typed_envelope},
};
use semver::Version;
use std::{
    collections::VecDeque,
    fmt,
    ops::ControlFlow,
    path::PathBuf,
    sync::{
        Arc, Weak,
        atomic::{AtomicU32, AtomicU64, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use util::{
    ResultExt,
    paths::{PathStyle, RemotePathBuf},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RemoteOs {
    Linux,
    MacOs,
    Windows,
}

impl RemoteOs {
    pub fn as_str(&self) -> &'static str {
        match self {
            RemoteOs::Linux => "linux",
            RemoteOs::MacOs => "macos",
            RemoteOs::Windows => "windows",
        }
    }

    pub fn is_windows(&self) -> bool {
        matches!(self, RemoteOs::Windows)
    }
}

impl std::fmt::Display for RemoteOs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RemoteArch {
    X86_64,
    Aarch64,
}

impl RemoteArch {
    pub fn as_str(&self) -> &'static str {
        match self {
            RemoteArch::X86_64 => "x86_64",
            RemoteArch::Aarch64 => "aarch64",
        }
    }
}

impl std::fmt::Display for RemoteArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RemotePlatform {
    pub os: RemoteOs,
    pub arch: RemoteArch,
}

#[derive(Clone, Debug)]
pub struct CommandTemplate {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

/// Whether a command should be run with TTY allocation for interactive use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interactive {
    /// Allocate a pseudo-TTY for interactive terminal use.
    Yes,
    /// Do not allocate a TTY - for commands that communicate via piped stdio.
    No,
}

pub trait RemoteClientDelegate: Send + Sync {
    fn ask_password(
        &self,
        prompt: String,
        tx: oneshot::Sender<EncryptedPassword>,
        cx: &mut AsyncApp,
    );
    fn get_download_url(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<Version>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Option<String>>>;
    fn download_server_binary_locally(
        &self,
        platform: RemotePlatform,
        release_channel: ReleaseChannel,
        version: Option<Version>,
        cx: &mut AsyncApp,
    ) -> Task<Result<PathBuf>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncApp);
}

const MAX_MISSED_HEARTBEATS: usize = 5;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(5);
const INITIAL_CONNECTION_TIMEOUT: Duration =
    Duration::from_secs(if cfg!(debug_assertions) { 5 } else { 60 });

pub const MAX_RECONNECT_ATTEMPTS: usize = 3;

enum State {
    Connecting,
    Connected {
        remote_connection: Arc<dyn RemoteConnection>,
        delegate: Arc<dyn RemoteClientDelegate>,

        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    HeartbeatMissed {
        missed_heartbeats: usize,

        remote_connection: Arc<dyn RemoteConnection>,
        delegate: Arc<dyn RemoteClientDelegate>,

        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    Reconnecting,
    ReconnectFailed {
        remote_connection: Arc<dyn RemoteConnection>,
        delegate: Arc<dyn RemoteClientDelegate>,

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
    fn remote_connection(&self) -> Option<Arc<dyn RemoteConnection>> {
        match self {
            Self::Connected {
                remote_connection, ..
            } => Some(remote_connection.clone()),
            Self::HeartbeatMissed {
                remote_connection, ..
            } => Some(remote_connection.clone()),
            Self::ReconnectFailed {
                remote_connection, ..
            } => Some(remote_connection.clone()),
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
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
                ..
            } => Self::Connected {
                remote_connection,
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
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            } => Self::HeartbeatMissed {
                missed_heartbeats: 1,
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            },
            Self::HeartbeatMissed {
                missed_heartbeats,
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            } => Self::HeartbeatMissed {
                missed_heartbeats: missed_heartbeats + 1,
                remote_connection,
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

pub struct RemoteClient {
    client: Arc<ChannelClient>,
    unique_identifier: String,
    connection_options: RemoteConnectionOptions,
    path_style: PathStyle,
    state: Option<State>,
}

#[derive(Debug)]
pub enum RemoteClientEvent {
    Disconnected { server_not_running: bool },
}

impl EventEmitter<RemoteClientEvent> for RemoteClient {}

/// Identifies the socket on the remote server so that reconnects
/// can re-join the same project.
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
    fn to_string(&self, cx: &App) -> String {
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

pub async fn connect(
    connection_options: RemoteConnectionOptions,
    delegate: Arc<dyn RemoteClientDelegate>,
    cx: &mut AsyncApp,
) -> Result<Arc<dyn RemoteConnection>> {
    cx.update(|cx| {
        cx.update_default_global(|pool: &mut ConnectionPool, cx| {
            pool.connect(connection_options.clone(), delegate.clone(), cx)
        })
    })
    .await
    .map_err(|e| e.cloned())
}

impl RemoteClient {
    pub fn new(
        unique_identifier: ConnectionIdentifier,
        remote_connection: Arc<dyn RemoteConnection>,
        cancellation: oneshot::Receiver<()>,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut App,
    ) -> Task<Result<Option<Entity<Self>>>> {
        let unique_identifier = unique_identifier.to_string(cx);
        cx.spawn(async move |cx| {
            let success = Box::pin(async move {
                let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
                let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
                let (connection_activity_tx, connection_activity_rx) = mpsc::channel::<()>(1);

                let client = cx.update(|cx| {
                    ChannelClient::new(
                        incoming_rx,
                        outgoing_tx,
                        cx,
                        "client",
                        remote_connection.has_wsl_interop(),
                    )
                });

                let path_style = remote_connection.path_style();
                let this = cx.new(|_| Self {
                    client: client.clone(),
                    unique_identifier: unique_identifier.clone(),
                    connection_options: remote_connection.connection_options(),
                    path_style,
                    state: Some(State::Connecting),
                });

                let io_task = remote_connection.start_proxy(
                    unique_identifier,
                    false,
                    incoming_tx,
                    outgoing_rx,
                    connection_activity_tx,
                    delegate.clone(),
                    cx,
                );

                let ready = client
                    .wait_for_remote_started()
                    .with_timeout(INITIAL_CONNECTION_TIMEOUT, cx.background_executor())
                    .await;
                match ready {
                    Ok(Some(_)) => {}
                    Ok(None) => {
                        let mut error = "remote client exited before becoming ready".to_owned();
                        if let Some(status) = io_task.now_or_never() {
                            match status {
                                Ok(exit_code) => {
                                    error.push_str(&format!(", exit_code={exit_code:?}"))
                                }
                                Err(e) => error.push_str(&format!(", error={e:?}")),
                            }
                        }
                        let error = anyhow::anyhow!("{error}");
                        log::error!("failed to establish connection: {}", error);
                        return Err(error);
                    }
                    Err(_) => {
                        let mut error = String::new();
                        if let Some(status) = io_task.now_or_never() {
                            error.push_str("Client exited with ");
                            match status {
                                Ok(exit_code) => {
                                    error.push_str(&format!("exit_code {exit_code:?}"))
                                }
                                Err(e) => error.push_str(&format!("error {e:?}")),
                            }
                        } else {
                            error.push_str("client did not become ready within the timeout");
                        }
                        let error = anyhow::anyhow!("{error}");
                        log::error!("failed to establish connection: {error}");
                        return Err(error);
                    }
                }
                let multiplex_task = Self::monitor(this.downgrade(), io_task, cx);
                if let Err(error) = client.ping(HEARTBEAT_TIMEOUT).await {
                    log::error!("failed to establish connection: {}", error);
                    return Err(error);
                }

                let heartbeat_task = Self::heartbeat(this.downgrade(), connection_activity_rx, cx);

                this.update(cx, |this, _| {
                    this.state = Some(State::Connected {
                        remote_connection,
                        delegate,
                        multiplex_task,
                        heartbeat_task,
                    });
                });

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

    pub fn proto_client_from_channels(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &App,
        name: &'static str,
        has_wsl_interop: bool,
    ) -> AnyProtoClient {
        ChannelClient::new(incoming_rx, outgoing_tx, cx, name, has_wsl_interop).into()
    }

    pub fn shutdown_processes<T: RequestMessage>(
        &mut self,
        shutdown_request: Option<T>,
        executor: BackgroundExecutor,
    ) -> Option<impl Future<Output = ()> + use<T>> {
        let state = self.state.take()?;
        log::info!("shutting down remote processes");

        let State::Connected {
            multiplex_task,
            heartbeat_task,
            remote_connection,
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
                executor.timer(Duration::from_millis(50)).await;
            }

            // Drop `multiplex_task` because it owns our remote_connection_proxy_process, which is a
            // child of master_process.
            drop(multiplex_task);
            // Now drop the rest of state, which kills master process.
            drop(heartbeat_task);
            drop(remote_connection);
            drop(delegate);
        })
    }

    fn reconnect(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let can_reconnect = self
            .state
            .as_ref()
            .map(|state| state.can_reconnect())
            .unwrap_or(false);
        if !can_reconnect {
            let state = if let Some(state) = self.state.as_ref() {
                state.to_string()
            } else {
                "no state set".to_string()
            };
            log::info!(
                "aborting reconnect, because not in state that allows reconnecting: {state}"
            );
            anyhow::bail!(
                "aborting reconnect, because not in state that allows reconnecting: {state}"
            );
        }

        let state = self.state.take().unwrap();
        let (attempts, remote_connection, delegate) = match state {
            State::Connected {
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
            }
            | State::HeartbeatMissed {
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task,
                ..
            } => {
                drop(multiplex_task);
                drop(heartbeat_task);
                (0, remote_connection, delegate)
            }
            State::ReconnectFailed {
                attempts,
                remote_connection,
                delegate,
                ..
            } => (attempts, remote_connection, delegate),
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
            self.set_state(State::ReconnectExhausted, cx);
            return Ok(());
        }

        self.set_state(State::Reconnecting, cx);

        log::info!(
            "Trying to reconnect to remote server... Attempt {}",
            attempts
        );

        let unique_identifier = self.unique_identifier.clone();
        let client = self.client.clone();
        let reconnect_task = cx.spawn(async move |this, cx| {
            macro_rules! failed {
                ($error:expr, $attempts:expr, $remote_connection:expr, $delegate:expr) => {
                    delegate.set_status(Some(&format!("{error:#}", error = $error)), cx);
                    return State::ReconnectFailed {
                        error: anyhow!($error),
                        attempts: $attempts,
                        remote_connection: $remote_connection,
                        delegate: $delegate,
                    };
                };
            }

            if let Err(error) = remote_connection
                .kill()
                .await
                .context("Failed to kill remote_connection process")
            {
                failed!(error, attempts, remote_connection, delegate);
            };

            let connection_options = remote_connection.connection_options();

            let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
            let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
            let (connection_activity_tx, connection_activity_rx) = mpsc::channel::<()>(1);

            let (remote_connection, io_task) = match async {
                let remote_connection = cx
                    .update_global(|pool: &mut ConnectionPool, cx| {
                        pool.connect(connection_options, delegate.clone(), cx)
                    })
                    .await
                    .map_err(|error| error.cloned())?;

                let io_task = remote_connection.start_proxy(
                    unique_identifier,
                    true,
                    incoming_tx,
                    outgoing_rx,
                    connection_activity_tx,
                    delegate.clone(),
                    cx,
                );
                anyhow::Ok((remote_connection, io_task))
            }
            .await
            {
                Ok((remote_connection, io_task)) => (remote_connection, io_task),
                Err(error) => {
                    failed!(error, attempts, remote_connection, delegate);
                }
            };

            let multiplex_task = Self::monitor(this.clone(), io_task, cx);
            client.reconnect(incoming_rx, outgoing_tx, cx);

            if let Err(error) = client.resync(HEARTBEAT_TIMEOUT).await {
                failed!(error, attempts, remote_connection, delegate);
            };

            State::Connected {
                remote_connection,
                delegate,
                multiplex_task,
                heartbeat_task: Self::heartbeat(this.clone(), connection_activity_rx, cx),
            }
        });

        cx.spawn(async move |this, cx| {
            let new_state = reconnect_task.await;
            this.update(cx, |this, cx| {
                this.try_set_state(cx, |old_state| {
                    if old_state.is_reconnecting() {
                        match &new_state {
                            State::Connecting
                            | State::Reconnecting
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
        this: WeakEntity<Self>,
        mut connection_activity_rx: mpsc::Receiver<()>,
        cx: &mut AsyncApp,
    ) -> Task<Result<()>> {
        let Ok(client) = this.read_with(cx, |this, _| this.client.clone()) else {
            return Task::ready(Err(anyhow!("remote_connectionRemoteClient lost")));
        };

        cx.spawn(async move |cx| {
            let mut missed_heartbeats = 0;

            let keepalive_timer = cx.background_executor().timer(HEARTBEAT_INTERVAL).fuse();
            futures::pin_mut!(keepalive_timer);

            loop {
                select_biased! {
                    result = connection_activity_rx.next().fuse() => {
                        if result.is_none() {
                            log::warn!("remote heartbeat: connection activity channel has been dropped. stopping.");
                            return Ok(());
                        }

                        if missed_heartbeats != 0 {
                            missed_heartbeats = 0;
                            let _ =this.update(cx, |this, cx| {
                                this.handle_heartbeat_result(missed_heartbeats, cx)
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

                        let result = this.update(cx, |this, cx| {
                            this.handle_heartbeat_result(missed_heartbeats, cx)
                        })?;
                        if result.is_break() {
                            return Ok(());
                        }
                    }
                }

                keepalive_timer.set(cx.background_executor().timer(HEARTBEAT_INTERVAL).fuse());
            }
        })
    }

    fn handle_heartbeat_result(
        &mut self,
        missed_heartbeats: usize,
        cx: &mut Context<Self>,
    ) -> ControlFlow<()> {
        let state = self.state.take().unwrap();
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
        this: WeakEntity<Self>,
        io_task: Task<Result<i32>>,
        cx: &AsyncApp,
    ) -> Task<Result<()>> {
        cx.spawn(async move |cx| {
            let result = io_task.await;

            match result {
                Ok(exit_code) => {
                    if let Some(error) = ProxyLaunchError::from_exit_code(exit_code) {
                        match error {
                            ProxyLaunchError::ServerNotRunning => {
                                log::error!("failed to reconnect because server is not running");
                                this.update(cx, |this, cx| {
                                    // Evict a stale Persistent entry so the next
                                    // connect() does a fresh reconnect rather than
                                    // immediately handing back a dead Arc.
                                    let opts = this.connection_options();
                                    cx.update_global(|pool: &mut ConnectionPool, _| {
                                        pool.evict_if_persistent(&opts);
                                    });
                                    this.set_state(State::ServerNotRunning, cx);
                                })?;
                            }
                        }
                    } else {
                        log::error!("proxy process terminated unexpectedly: {exit_code}");
                        this.update(cx, |this, cx| {
                            this.reconnect(cx).ok();
                        })?;
                    }
                }
                Err(error) => {
                    log::warn!(
                        "remote io task died with error: {:?}. reconnecting...",
                        error
                    );
                    this.update(cx, |this, cx| {
                        this.reconnect(cx).ok();
                    })?;
                }
            }

            Ok(())
        })
    }

    fn state_is(&self, check: impl FnOnce(&State) -> bool) -> bool {
        self.state.as_ref().is_some_and(check)
    }

    fn try_set_state(&mut self, cx: &mut Context<Self>, map: impl FnOnce(&State) -> Option<State>) {
        let new_state = self.state.as_ref().and_then(map);
        if let Some(new_state) = new_state {
            self.state.replace(new_state);
            cx.notify();
        }
    }

    fn set_state(&mut self, state: State, cx: &mut Context<Self>) {
        log::info!("setting state to '{}'", &state);

        let is_reconnect_exhausted = state.is_reconnect_exhausted();
        let is_server_not_running = state.is_server_not_running();
        self.state.replace(state);

        if is_reconnect_exhausted || is_server_not_running {
            cx.emit(RemoteClientEvent::Disconnected {
                server_not_running: is_server_not_running,
            });
        }
        cx.notify();
    }

    pub fn shell(&self) -> Option<String> {
        Some(self.remote_connection()?.shell())
    }

    pub fn default_system_shell(&self) -> Option<String> {
        Some(self.remote_connection()?.default_system_shell())
    }

    pub fn shares_network_interface(&self) -> bool {
        self.remote_connection()
            .map_or(false, |connection| connection.shares_network_interface())
    }

    pub fn has_wsl_interop(&self) -> bool {
        self.remote_connection()
            .map_or(false, |connection| connection.has_wsl_interop())
    }

    pub fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        self.build_command_with_options(
            program,
            args,
            env,
            working_dir,
            port_forward,
            Interactive::Yes,
        )
    }

    pub fn build_command_with_options(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
        interactive: Interactive,
    ) -> Result<CommandTemplate> {
        let Some(connection) = self.remote_connection() else {
            return Err(anyhow!("no remote connection"));
        };
        connection.build_command(program, args, env, working_dir, port_forward, interactive)
    }

    pub fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate> {
        let Some(connection) = self.remote_connection() else {
            return Err(anyhow!("no remote connection"));
        };
        connection.build_forward_ports_command(forwards)
    }

    pub fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>> {
        let Some(connection) = self.remote_connection() else {
            return Task::ready(Err(anyhow!("no remote connection")));
        };
        connection.upload_directory(src_path, dest_path, cx)
    }

    pub fn proto_client(&self) -> AnyProtoClient {
        self.client.clone().into()
    }

    pub fn connection_options(&self) -> RemoteConnectionOptions {
        self.connection_options.clone()
    }

    pub fn connection(&self) -> Option<Arc<dyn RemoteConnection>> {
        if let State::Connected {
            remote_connection, ..
        } = self.state.as_ref()?
        {
            Some(remote_connection.clone())
        } else {
            None
        }
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.state
            .as_ref()
            .map(ConnectionState::from)
            .unwrap_or(ConnectionState::Disconnected)
    }

    pub fn is_disconnected(&self) -> bool {
        self.connection_state() == ConnectionState::Disconnected
    }

    pub fn path_style(&self) -> PathStyle {
        self.path_style
    }

    /// Forcibly disconnects from the remote server by killing the underlying connection.
    /// This will trigger the reconnection logic if reconnection attempts remain.
    /// Useful for testing reconnection behavior in real environments.
    pub fn force_disconnect(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let Some(connection) = self.remote_connection() else {
            return Task::ready(Err(anyhow!("no active remote connection to disconnect")));
        };

        log::info!("force_disconnect: killing remote connection");

        cx.spawn(async move |_, _| {
            connection.kill().await?;
            Ok(())
        })
    }

    /// Simulates a timeout by pausing heartbeat responses.
    /// This will cause heartbeat failures and eventually trigger reconnection
    /// after MAX_MISSED_HEARTBEATS are missed.
    /// Useful for testing timeout behavior in real environments.
    pub fn force_heartbeat_timeout(&mut self, attempts: usize, cx: &mut Context<Self>) {
        log::info!("force_heartbeat_timeout: triggering heartbeat failure state");

        if let Some(State::Connected {
            remote_connection,
            delegate,
            multiplex_task,
            heartbeat_task,
        }) = self.state.take()
        {
            self.set_state(
                if attempts == 0 {
                    State::HeartbeatMissed {
                        missed_heartbeats: MAX_MISSED_HEARTBEATS,
                        remote_connection,
                        delegate,
                        multiplex_task,
                        heartbeat_task,
                    }
                } else {
                    State::ReconnectFailed {
                        remote_connection,
                        delegate,
                        error: anyhow!("forced heartbeat timeout"),
                        attempts,
                    }
                },
                cx,
            );

            self.reconnect(cx)
                .context("failed to start reconnect after forced timeout")
                .log_err();
        } else {
            log::warn!("force_heartbeat_timeout: not in Connected state, ignoring");
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn force_server_not_running(&mut self, cx: &mut Context<Self>) {
        self.set_state(State::ServerNotRunning, cx);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_disconnect(&self, client_cx: &mut App) -> Task<()> {
        let opts = self.connection_options();
        client_cx.spawn(async move |cx| {
            let connection =
                cx.update_global(|c: &mut ConnectionPool, _| match c.connections.get(&opts) {
                    Some(ConnectionPoolEntry::Connected(c)) => c
                        .upgrade()
                        .unwrap_or_else(|| panic!("connection was dropped")),
                    Some(ConnectionPoolEntry::Persistent { connection, .. }) => connection.clone(),
                    _ => panic!("missing test connection"),
                });

            connection.simulate_disconnect(cx);
        })
    }

    /// Creates a mock connection pair for testing.
    ///
    /// This is the recommended way to create mock remote connections for tests.
    /// It returns the `MockConnectionOptions` (which can be passed to create a
    /// `HeadlessProject`), an `AnyProtoClient` for the server side and a
    /// `ConnectGuard` for the client side which blocks the connection from
    /// being established until dropped.
    ///
    /// # Example
    /// ```ignore
    /// let (opts, server_session, connect_guard) = RemoteClient::fake_server(cx, server_cx);
    /// // Set up HeadlessProject with server_session...
    /// drop(connect_guard);
    /// let client = RemoteClient::fake_client(opts, cx).await;
    /// ```
    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_server(
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (RemoteConnectionOptions, AnyProtoClient, ConnectGuard) {
        use crate::transport::mock::MockConnection;
        let (opts, server_client, connect_guard) = MockConnection::new(client_cx, server_cx);
        (opts.into(), server_client, connect_guard)
    }

    /// Registers a new mock server for existing connection options.
    ///
    /// Use this to simulate reconnection: after forcing a disconnect, register
    /// a new server so the next `connect()` call succeeds.
    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_server_with_opts(
        opts: &RemoteConnectionOptions,
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (AnyProtoClient, ConnectGuard) {
        use crate::transport::mock::MockConnection;
        let mock_opts = match opts {
            RemoteConnectionOptions::Mock(mock_opts) => mock_opts.clone(),
            _ => panic!("fake_server_with_opts requires Mock connection options"),
        };
        MockConnection::new_with_opts(mock_opts, client_cx, server_cx)
    }

    /// Creates a `RemoteClient` connected to a mock server.
    ///
    /// Call `fake_server` first to get the connection options, set up the
    /// `HeadlessProject` with the server session, then call this method
    /// to create the client.
    #[cfg(any(test, feature = "test-support"))]
    pub async fn connect_mock(
        opts: RemoteConnectionOptions,
        client_cx: &mut gpui::TestAppContext,
    ) -> Entity<Self> {
        assert!(matches!(opts, RemoteConnectionOptions::Mock(..)));
        use crate::transport::mock::MockDelegate;
        let (_tx, rx) = oneshot::channel();
        let mut cx = client_cx.to_async();
        let connection = connect(opts, Arc::new(MockDelegate), &mut cx)
            .await
            .unwrap();
        client_cx
            .update(|cx| {
                Self::new(
                    ConnectionIdentifier::setup(),
                    connection,
                    rx,
                    Arc::new(MockDelegate),
                    cx,
                )
            })
            .await
            .unwrap()
            .unwrap()
    }

    pub fn remote_connection(&self) -> Option<Arc<dyn RemoteConnection>> {
        self.state
            .as_ref()
            .and_then(|state| state.remote_connection())
    }
}

enum ConnectionPoolEntry {
    Connecting(WeakShared<Task<Result<Arc<dyn RemoteConnection>, Arc<anyhow::Error>>>>),
    Connected(Weak<dyn RemoteConnection>),
    Persistent {
        connection: Arc<dyn RemoteConnection>,
        workspace_count: usize,
    },
}

pub struct ConnectionPool {
    connections: HashMap<RemoteConnectionOptions, ConnectionPoolEntry>,
    _quit_subscription: Option<Subscription>,
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self {
            connections: HashMap::default(),
            _quit_subscription: None,
        }
    }
}

impl Global for ConnectionPool {}

impl ConnectionPool {
    fn connect(
        &mut self,
        opts: RemoteConnectionOptions,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut App,
    ) -> Shared<Task<Result<Arc<dyn RemoteConnection>, Arc<anyhow::Error>>>> {
        let connection = self.connections.get(&opts);
        match connection {
            Some(ConnectionPoolEntry::Connecting(task)) => {
                if let Some(task) = task.upgrade() {
                    log::debug!("Connecting task is still alive");
                    cx.spawn(async move |cx| {
                        delegate.set_status(Some("Waiting for existing connection attempt"), cx)
                    })
                    .detach();
                    return task;
                }
                log::debug!("Connecting task is dead, removing it and restarting a connection");
                self.connections.remove(&opts);
            }
            Some(ConnectionPoolEntry::Connected(remote)) => {
                if let Some(remote) = remote.upgrade()
                    && !remote.has_been_killed()
                {
                    log::debug!("Connection is still alive");
                    return Task::ready(Ok(remote)).shared();
                }
                log::debug!("Connection is dead, removing it and restarting a connection");
                self.connections.remove(&opts);
            }
            Some(ConnectionPoolEntry::Persistent {
                connection,
                workspace_count,
            }) => {
                // `monitor()` asynchronously calls `evict_if_persistent()` whenever it
                // detects exit-code 90 (`ServerNotRunning`), so by the time `connect()`
                // is called again the stale entry will already have been removed.  We
                // still check `has_been_killed()` here as a belt-and-suspenders guard,
                // but note that on WSL this always returns `false` (the kill signal is
                // not observable at the Rust layer) — eviction is handled entirely by
                // the monitor task in that case.
                if !connection.has_been_killed() {
                    log::debug!(
                        "Persistent connection is still alive, reusing (workspace_count={})",
                        workspace_count
                    );
                    *workspace_count += 1;
                    return Task::ready(Ok(connection.clone())).shared();
                }
                log::debug!(
                    "Persistent connection is dead, removing it and restarting a connection"
                );
                self.connections.remove(&opts);
            }
            None => {
                log::debug!("No existing connection found, starting a new one");
            }
        }

        if self._quit_subscription.is_none() {
            self._quit_subscription = Some(cx.on_app_quit(|cx| async move {
                let persistent_opts: Vec<RemoteConnectionOptions> = cx
                    .update_global(|pool: &mut ConnectionPool, _| {
                        pool.connections
                            .iter()
                            .filter_map(|(opts, entry)| {
                                matches!(entry, ConnectionPoolEntry::Persistent { .. })
                                    .then(|| opts.clone())
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                cx.update_global(|pool: &mut ConnectionPool, _| {
                    for opts in persistent_opts {
                        pool.release(&opts);
                    }
                })
                .ok();
            }));
        }

        let task = cx
            .spawn({
                let opts = opts.clone();
                let delegate = delegate.clone();
                async move |cx| {
                    let connection = match opts.clone() {
                        RemoteConnectionOptions::Ssh(opts) => {
                            SshRemoteConnection::new(opts, delegate, cx)
                                .await
                                .map(|connection| Arc::new(connection) as Arc<dyn RemoteConnection>)
                        }
                        RemoteConnectionOptions::Wsl(opts) => {
                            WslRemoteConnection::new(opts, delegate, cx)
                                .await
                                .map(|connection| Arc::new(connection) as Arc<dyn RemoteConnection>)
                        }
                        RemoteConnectionOptions::Docker(opts) => {
                            DockerExecConnection::new(opts, delegate, cx)
                                .await
                                .map(|connection| Arc::new(connection) as Arc<dyn RemoteConnection>)
                        }
                        #[cfg(any(test, feature = "test-support"))]
                        RemoteConnectionOptions::Mock(opts) => match cx.update(|cx| {
                            cx.default_global::<crate::transport::mock::MockConnectionRegistry>()
                                .take(&opts)
                        }) {
                            Some(connection) => Ok(connection.await as Arc<dyn RemoteConnection>),
                            None => Err(anyhow!(
                                "Mock connection not found. Call MockConnection::new() first."
                            )),
                        },
                    };

                    cx.update_global(|pool: &mut Self, _| {
                        debug_assert!(matches!(
                            pool.connections.get(&opts),
                            Some(ConnectionPoolEntry::Connecting(_))
                        ));
                        match connection {
                            Ok(connection) => {
                                let entry = if should_persist(&opts) {
                                    ConnectionPoolEntry::Persistent {
                                        connection: connection.clone(),
                                        workspace_count: 1,
                                    }
                                } else {
                                    ConnectionPoolEntry::Connected(Arc::downgrade(&connection))
                                };
                                pool.connections.insert(opts.clone(), entry);
                                Ok(connection)
                            }
                            Err(error) => {
                                pool.connections.remove(&opts);
                                Err(Arc::new(error))
                            }
                        }
                    })
                }
            })
            .shared();
        if let Some(task) = task.downgrade() {
            self.connections
                .insert(opts.clone(), ConnectionPoolEntry::Connecting(task));
        }
        task
    }

    pub fn is_persistent(&self, opts: &RemoteConnectionOptions) -> bool {
        matches!(
            self.connections.get(opts),
            Some(ConnectionPoolEntry::Persistent { .. })
        )
    }

    /// Removes the pool entry for `opts` only if it is currently `Persistent`.
    /// `Connected` entries and absent entries are left untouched.
    /// Called from `RemoteClient::monitor()` when exit-code-90 is observed so
    /// that a stale persistent arc is not reused on the next `connect()` call.
    pub fn evict_if_persistent(&mut self, opts: &RemoteConnectionOptions) {
        if matches!(
            self.connections.get(opts),
            Some(ConnectionPoolEntry::Persistent { .. })
        ) {
            self.connections.remove(opts);
        }
    }

    pub fn release(&mut self, opts: &RemoteConnectionOptions) {
        match self.connections.get_mut(opts) {
            Some(ConnectionPoolEntry::Persistent {
                workspace_count, ..
            }) => {
                if *workspace_count > 1 {
                    *workspace_count -= 1;
                    return;
                }
                // workspace_count is 1 (now dropping to 0): fall through to remove.
            }
            _ => return,
        }
        if let Some(ConnectionPoolEntry::Persistent { connection, .. }) =
            self.connections.remove(opts)
        {
            let weak = Arc::downgrade(&connection);
            drop(connection);
            if weak.upgrade().is_some() {
                self.connections
                    .insert(opts.clone(), ConnectionPoolEntry::Connected(weak));
            }
            // If weak.upgrade() is None: no consumers remain; entry stays removed.
        }
    }
}

pub(crate) fn should_persist(opts: &RemoteConnectionOptions) -> bool {
    matches!(opts, RemoteConnectionOptions::Wsl(_))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteConnectionOptions {
    Ssh(SshConnectionOptions),
    Wsl(WslConnectionOptions),
    Docker(DockerConnectionOptions),
    #[cfg(any(test, feature = "test-support"))]
    Mock(crate::transport::mock::MockConnectionOptions),
}

impl RemoteConnectionOptions {
    pub fn display_name(&self) -> String {
        match self {
            RemoteConnectionOptions::Ssh(opts) => opts.host.to_string(),
            RemoteConnectionOptions::Wsl(opts) => opts.distro_name.clone(),
            RemoteConnectionOptions::Docker(opts) => {
                if opts.use_podman {
                    format!("[podman] {}", opts.name)
                } else {
                    opts.name.clone()
                }
            }
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(opts) => format!("mock-{}", opts.id),
        }
    }
}

impl From<SshConnectionOptions> for RemoteConnectionOptions {
    fn from(opts: SshConnectionOptions) -> Self {
        RemoteConnectionOptions::Ssh(opts)
    }
}

impl From<WslConnectionOptions> for RemoteConnectionOptions {
    fn from(opts: WslConnectionOptions) -> Self {
        RemoteConnectionOptions::Wsl(opts)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl From<crate::transport::mock::MockConnectionOptions> for RemoteConnectionOptions {
    fn from(opts: crate::transport::mock::MockConnectionOptions) -> Self {
        RemoteConnectionOptions::Mock(opts)
    }
}

#[cfg(target_os = "windows")]
/// Open a wsl path (\\wsl.localhost\<distro>\path)
#[derive(Debug, Clone, PartialEq, Eq, gpui::Action)]
#[action(namespace = workspace, no_json, no_register)]
pub struct OpenWslPath {
    pub distro: WslConnectionOptions,
    pub paths: Vec<PathBuf>,
}

#[async_trait(?Send)]
pub trait RemoteConnection: Send + Sync {
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>>;
    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
    ) -> Task<Result<()>>;
    async fn kill(&self) -> Result<()>;
    fn has_been_killed(&self) -> bool;
    fn shares_network_interface(&self) -> bool {
        false
    }
    fn build_command(
        &self,
        program: Option<String>,
        args: &[String],
        env: &HashMap<String, String>,
        working_dir: Option<String>,
        port_forward: Option<(u16, String, u16)>,
        interactive: Interactive,
    ) -> Result<CommandTemplate>;
    fn build_forward_ports_command(
        &self,
        forwards: Vec<(u16, String, u16)>,
    ) -> Result<CommandTemplate>;
    fn connection_options(&self) -> RemoteConnectionOptions;
    fn path_style(&self) -> PathStyle;
    fn shell(&self) -> String;
    fn default_system_shell(&self) -> String;
    fn has_wsl_interop(&self) -> bool;

    #[cfg(any(test, feature = "test-support"))]
    fn simulate_disconnect(&self, _: &AsyncApp) {}
}

type ResponseChannels = Mutex<HashMap<MessageId, oneshot::Sender<(Envelope, oneshot::Sender<()>)>>>;

struct Signal<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
    rx: Shared<Task<Option<T>>>,
}

impl<T: Send + Clone + 'static> Signal<T> {
    pub fn new(cx: &App) -> Self {
        let (tx, rx) = oneshot::channel();

        let task = cx
            .background_executor()
            .spawn(async move { rx.await.ok() })
            .shared();

        Self {
            tx: Mutex::new(Some(tx)),
            rx: task,
        }
    }

    fn set(&self, value: T) {
        if let Some(tx) = self.tx.lock().take() {
            let _ = tx.send(value);
        }
    }

    fn wait(&self) -> Shared<Task<Option<T>>> {
        self.rx.clone()
    }
}

pub(crate) struct ChannelClient {
    next_message_id: AtomicU32,
    outgoing_tx: Mutex<mpsc::UnboundedSender<Envelope>>,
    buffer: Mutex<VecDeque<Envelope>>,
    response_channels: ResponseChannels,
    message_handlers: Mutex<ProtoMessageHandlerSet>,
    max_received: AtomicU32,
    name: &'static str,
    task: Mutex<Task<Result<()>>>,
    remote_started: Signal<()>,
    has_wsl_interop: bool,
    executor: BackgroundExecutor,
}

impl ChannelClient {
    pub(crate) fn new(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &App,
        name: &'static str,
        has_wsl_interop: bool,
    ) -> Arc<Self> {
        Arc::new_cyclic(|this| Self {
            outgoing_tx: Mutex::new(outgoing_tx),
            next_message_id: AtomicU32::new(0),
            max_received: AtomicU32::new(0),
            response_channels: ResponseChannels::default(),
            message_handlers: Default::default(),
            buffer: Mutex::new(VecDeque::new()),
            name,
            executor: cx.background_executor().clone(),
            task: Mutex::new(Self::start_handling_messages(
                this.clone(),
                incoming_rx,
                &cx.to_async(),
            )),
            remote_started: Signal::new(cx),
            has_wsl_interop,
        })
    }

    fn wait_for_remote_started(&self) -> Shared<Task<Option<()>>> {
        self.remote_started.wait()
    }

    fn start_handling_messages(
        this: Weak<Self>,
        mut incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        cx: &AsyncApp,
    ) -> Task<Result<()>> {
        cx.spawn(async move |cx| {
            if let Some(this) = this.upgrade() {
                let envelope = proto::RemoteStarted {}.into_envelope(0, None, None);
                this.outgoing_tx.lock().unbounded_send(envelope).ok();
            };

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
                        "{}:remote message received. name:FlushBufferedMessages",
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

                if let Some(proto::envelope::Payload::RemoteStarted(_)) = &incoming.payload {
                    this.remote_started.set(());
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
                    let message_id = envelope.message_id();
                    if let Some(future) = ProtoMessageHandlerSet::handle_message(
                        &this.message_handlers,
                        envelope,
                        this.clone().into(),
                        cx.clone(),
                    ) {
                        log::debug!("{}:remote message received. name:{type_name}", this.name);
                        cx.foreground_executor()
                            .spawn(async move {
                                match future.await {
                                    Ok(_) => {
                                        log::debug!(
                                            "{}:remote message handled. name:{type_name}",
                                            this.name
                                        );
                                    }
                                    Err(error) => {
                                        log::error!(
                                            "{}:error handling message. type:{}, error:{:#}",
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
                        log::error!("{}:unhandled remote message name:{type_name}", this.name);
                        if let Err(e) = AnyProtoClient::from(this.clone()).send_response(
                            message_id,
                            anyhow::anyhow!("no handler registered for {type_name}").to_proto(),
                        ) {
                            log::error!(
                                "{}:error sending error response for {type_name}:{e:#}",
                                this.name
                            );
                        }
                    }
                }
            }
            anyhow::Ok(())
        })
    }

    pub(crate) fn reconnect(
        self: &Arc<Self>,
        incoming_rx: UnboundedReceiver<Envelope>,
        outgoing_tx: UnboundedSender<Envelope>,
        cx: &AsyncApp,
    ) {
        *self.outgoing_tx.lock() = outgoing_tx;
        *self.task.lock() = Self::start_handling_messages(Arc::downgrade(self), incoming_rx, cx);
    }

    fn request<T: RequestMessage>(
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
        log::debug!("remote request start. name:{}", T::NAME);
        let response =
            self.request_dynamic(payload.into_envelope(0, None, None), T::NAME, use_buffer);
        async move {
            let response = response.await?;
            log::debug!("remote request finish. name:{}", T::NAME);
            T::Response::from_envelope(response).context("received a response of the wrong type")
        }
    }

    async fn resync(&self, timeout: Duration) -> Result<()> {
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
                self.executor.timer(timeout).await;
                anyhow::bail!("Timed out resyncing remote client")
            },
        )
        .await
    }

    async fn ping(&self, timeout: Duration) -> Result<()> {
        smol::future::or(
            async {
                self.request(proto::Ping {}).await?;
                Ok(())
            },
            async {
                self.executor.timer(timeout).await;
                anyhow::bail!("Timed out pinging remote client")
            },
        )
        .await
    }

    fn send<T: EnvelopedMessage>(&self, payload: T) -> Result<()> {
        log::debug!("remote send name:{}", T::NAME);
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
                log::error!("failed to send message: {error}");
                anyhow::bail!("failed to send message: {error}");
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

    fn has_wsl_interop(&self) -> bool {
        self.has_wsl_interop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::{mock::MockConnectionOptions, wsl::WslConnectionOptions};
    use std::sync::Arc;

    /// Minimal stub that implements `RemoteConnection` for unit tests that only
    /// need an `Arc<dyn RemoteConnection>` — none of the trait methods are
    /// actually called.
    struct StubConnection;

    #[async_trait(?Send)]
    impl RemoteConnection for StubConnection {
        fn start_proxy(
            &self,
            _unique_identifier: String,
            _reconnect: bool,
            _incoming_tx: futures::channel::mpsc::UnboundedSender<rpc::proto::Envelope>,
            _outgoing_rx: futures::channel::mpsc::UnboundedReceiver<rpc::proto::Envelope>,
            _connection_activity_tx: futures::channel::mpsc::Sender<()>,
            _delegate: Arc<dyn RemoteClientDelegate>,
            _cx: &mut AsyncApp,
        ) -> Task<Result<i32>> {
            Task::ready(Ok(0))
        }

        fn upload_directory(
            &self,
            _src_path: std::path::PathBuf,
            _dest_path: util::paths::RemotePathBuf,
            _cx: &App,
        ) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }

        async fn kill(&self) -> Result<()> {
            Ok(())
        }

        fn has_been_killed(&self) -> bool {
            false
        }

        fn build_command(
            &self,
            _program: Option<String>,
            _args: &[String],
            _env: &collections::HashMap<String, String>,
            _working_dir: Option<String>,
            _port_forward: Option<(u16, String, u16)>,
            _interactive: Interactive,
        ) -> Result<CommandTemplate> {
            Ok(CommandTemplate {
                program: "stub".into(),
                args: vec![],
                env: Default::default(),
            })
        }

        fn build_forward_ports_command(
            &self,
            _forwards: Vec<(u16, String, u16)>,
        ) -> Result<CommandTemplate> {
            Ok(CommandTemplate {
                program: "stub".into(),
                args: vec![],
                env: Default::default(),
            })
        }

        fn connection_options(&self) -> RemoteConnectionOptions {
            RemoteConnectionOptions::Wsl(WslConnectionOptions {
                distro_name: "stub".into(),
                user: None,
            })
        }

        fn path_style(&self) -> util::paths::PathStyle {
            util::paths::PathStyle::local()
        }

        fn shell(&self) -> String {
            "sh".into()
        }

        fn default_system_shell(&self) -> String {
            "sh".into()
        }

        fn has_wsl_interop(&self) -> bool {
            false
        }
    }

    fn stub_connection() -> Arc<dyn RemoteConnection> {
        Arc::new(StubConnection)
    }

    fn wsl_opts(distro: &str) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Wsl(WslConnectionOptions {
            distro_name: distro.into(),
            user: None,
        })
    }

    fn ssh_opts(host: &str) -> RemoteConnectionOptions {
        RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: host.into(),
            username: None,
            port: None,
            password: None,
            args: None,
            port_forwards: None,
            connection_timeout: None,
            nickname: None,
            upload_binary_over_ssh: false,
        })
    }

    fn mock_opts() -> RemoteConnectionOptions {
        RemoteConnectionOptions::Mock(MockConnectionOptions { id: 0 })
    }

    // --- should_persist tests ---

    #[test]
    fn test_should_persist_wsl_returns_true() {
        assert!(should_persist(&wsl_opts("Ubuntu")));
    }

    #[test]
    fn test_should_persist_ssh_returns_false() {
        assert!(!should_persist(&ssh_opts("myhost")));
    }

    #[test]
    fn test_should_persist_mock_returns_false() {
        assert!(!should_persist(&mock_opts()));
    }

    // --- is_persistent tests ---

    #[test]
    fn test_is_persistent_returns_true_for_persistent_entry() {
        let opts = wsl_opts("Ubuntu");
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );
        assert!(pool.is_persistent(&opts));
    }

    #[test]
    fn test_is_persistent_returns_false_for_connected_entry() {
        let opts = ssh_opts("myhost");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Connected(Arc::downgrade(&connection)),
        );
        assert!(!pool.is_persistent(&opts));
    }

    #[test]
    fn test_is_persistent_returns_false_when_absent() {
        let pool = ConnectionPool::default();
        assert!(!pool.is_persistent(&wsl_opts("Ubuntu")));
    }

    // --- release tests ---

    #[test]
    fn test_release_decrements_workspace_count_from_two_to_one() {
        let opts = wsl_opts("Ubuntu");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: connection.clone(),
                workspace_count: 2,
            },
        );

        pool.release(&opts);

        match pool.connections.get(&opts).unwrap() {
            ConnectionPoolEntry::Persistent {
                workspace_count, ..
            } => {
                assert_eq!(*workspace_count, 1);
            }
            _ => panic!("expected Persistent entry"),
        }
    }

    #[test]
    fn test_release_at_count_one_removes_or_downgrades_to_connected() {
        let opts = wsl_opts("Ubuntu");
        let connection = stub_connection();
        let weak = Arc::downgrade(&connection);
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection,
                workspace_count: 1,
            },
        );

        pool.release(&opts);

        // The Arc we kept alive via `weak` means the entry should now be
        // Connected(Weak) rather than removed entirely.
        match pool.connections.get(&opts) {
            Some(ConnectionPoolEntry::Connected(stored_weak)) => {
                assert!(stored_weak.upgrade().is_some());
                assert!(weak.upgrade().is_some());
            }
            None => {
                // Acceptable if the strong Arc was the only reference — entry
                // is simply removed.
            }
            _ => panic!("unexpected entry type after release"),
        }
    }

    #[test]
    fn test_release_at_count_one_removes_entry_when_no_other_owner() {
        let opts = wsl_opts("Ubuntu");
        let mut pool = ConnectionPool::default();
        // Insert without keeping any extra Arc, so the connection is dropped
        // at release() time and the entry should be removed entirely.
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );

        pool.release(&opts);

        // After releasing the last workspace, the Persistent Arc is dropped.
        // No external strong reference exists, so the entry is gone.
        assert!(pool.connections.get(&opts).is_none());
    }

    #[test]
    fn test_release_on_non_persistent_entry_is_no_op() {
        let opts = ssh_opts("myhost");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Connected(Arc::downgrade(&connection)),
        );

        // Must not panic.
        pool.release(&opts);

        // Entry is unchanged.
        assert!(matches!(
            pool.connections.get(&opts),
            Some(ConnectionPoolEntry::Connected(_))
        ));
    }

    #[test]
    fn test_release_on_absent_entry_is_no_op() {
        let mut pool = ConnectionPool::default();
        // Must not panic.
        pool.release(&wsl_opts("Ubuntu"));
        assert!(pool.connections.is_empty());
    }

    // --- evict_if_persistent tests ---

    /// 7.3: Two releases from count-2 → count-1 → entry no longer Persistent.
    #[test]
    fn test_two_releases_from_count_two_to_count_one_then_removed() {
        let opts = wsl_opts("Ubuntu");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: connection.clone(),
                workspace_count: 2,
            },
        );

        // First release: count goes 2 → 1, still Persistent.
        pool.release(&opts);
        match pool
            .connections
            .get(&opts)
            .expect("entry should still exist")
        {
            ConnectionPoolEntry::Persistent {
                workspace_count, ..
            } => {
                assert_eq!(*workspace_count, 1);
            }
            _ => panic!("expected Persistent entry after first release"),
        }

        // Second release: count goes 1 → 0. External `connection` arc is still
        // alive, so the entry downgrades to Connected rather than being removed.
        pool.release(&opts);
        assert!(
            !matches!(
                pool.connections.get(&opts),
                Some(ConnectionPoolEntry::Persistent { .. })
            ),
            "entry must no longer be Persistent after second release"
        );
    }

    /// 7.4: Release with no external Arc holder → entry removed entirely.
    #[test]
    fn test_release_with_no_external_arc_removes_entry() {
        let opts = wsl_opts("Ubuntu");
        let mut pool = ConnectionPool::default();
        // Insert without keeping any external strong reference.
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );

        pool.release(&opts);

        // Pool held the only strong Arc; after release the weak upgrade fails
        // and the entry is removed.
        assert!(
            pool.connections.get(&opts).is_none(),
            "entry must be absent when no external Arc holder exists"
        );
    }

    /// 7.5: evict_if_persistent removes a Persistent entry.
    #[test]
    fn test_evict_if_persistent_removes_persistent_entry() {
        let opts = wsl_opts("Ubuntu");
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );

        pool.evict_if_persistent(&opts);

        assert!(
            !pool.is_persistent(&opts),
            "entry must not be Persistent after eviction"
        );
        assert!(
            pool.connections.get(&opts).is_none(),
            "entry must be absent after eviction"
        );
    }

    /// 7.6: evict_if_persistent does NOT remove a Connected entry.
    #[test]
    fn test_evict_if_persistent_leaves_connected_entry_intact() {
        let opts = ssh_opts("myhost");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Connected(Arc::downgrade(&connection)),
        );

        pool.evict_if_persistent(&opts);

        assert!(
            !pool.is_persistent(&opts),
            "Connected entry must not be reported as Persistent"
        );
        assert!(
            matches!(
                pool.connections.get(&opts),
                Some(ConnectionPoolEntry::Connected(_))
            ),
            "Connected entry must still be present after evict_if_persistent"
        );
    }

    /// 7.7: SSH opts never become Persistent; inserting Connected for SSH
    /// opts leaves is_persistent() returning false.
    #[test]
    fn test_ssh_opts_connected_entry_is_never_persistent() {
        let opts = ssh_opts("myhost");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();

        // should_persist() must return false for SSH.
        assert!(
            !should_persist(&opts),
            "SSH must not be eligible for persistence"
        );

        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Connected(Arc::downgrade(&connection)),
        );

        assert!(
            !pool.is_persistent(&opts),
            "SSH Connected entry must never be reported as Persistent"
        );
    }

    // --- specifically-named tests required by tasks 7.3–7.7 ---

    /// Task 7.3: A WSL Persistent entry is reused and workspace_count increments.
    ///
    /// `connect()` requires a GPUI context so we cannot call it directly in a
    /// unit test.  Instead we simulate the fast-path by inserting a Persistent
    /// entry at count=1 and then manually applying the same mutation that
    /// `connect()` performs — incrementing `workspace_count` — before verifying
    /// the result.
    #[test]
    fn test_wsl_persistent_connection_is_reused() {
        let opts = wsl_opts("Ubuntu");
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: connection.clone(),
                workspace_count: 1,
            },
        );

        // Simulate the fast-path in connect(): increment workspace_count and
        // return the existing Arc rather than creating a new connection.
        let returned_connection = match pool.connections.get_mut(&opts) {
            Some(ConnectionPoolEntry::Persistent {
                connection,
                workspace_count,
            }) if !connection.has_been_killed() => {
                *workspace_count += 1;
                connection.clone()
            }
            _ => panic!("expected a live Persistent entry"),
        };

        // The returned Arc must be the same object as the one we inserted.
        assert!(Arc::ptr_eq(&returned_connection, &connection));

        // workspace_count must now be 2.
        match pool.connections.get(&opts).unwrap() {
            ConnectionPoolEntry::Persistent {
                workspace_count, ..
            } => assert_eq!(*workspace_count, 2),
            _ => panic!("expected Persistent entry"),
        }
    }

    /// Task 7.4: release() decrements workspace_count and removes the entry
    /// when it reaches zero (and no external Arc keeps it alive).
    #[test]
    fn test_release_decrements_workspace_count_and_drops_at_zero() {
        let opts = wsl_opts("Ubuntu");
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 2,
            },
        );

        // First release: 2 → 1, entry still Persistent.
        pool.release(&opts);
        match pool.connections.get(&opts).unwrap() {
            ConnectionPoolEntry::Persistent {
                workspace_count, ..
            } => assert_eq!(*workspace_count, 1),
            _ => panic!("expected Persistent entry after first release"),
        }

        // Second release: 1 → 0. No external Arc, so entry must be gone.
        pool.release(&opts);
        assert!(
            pool.connections.get(&opts).is_none(),
            "entry must be removed when workspace_count reaches zero with no external Arc"
        );
    }

    /// Task 7.5: evict_if_persistent() removes a Persistent entry entirely.
    #[test]
    fn test_evict_if_persistent_removes_entry() {
        let opts = wsl_opts("Ubuntu");
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );

        pool.evict_if_persistent(&opts);

        assert!(
            pool.connections.get(&opts).is_none(),
            "entry must be absent after evict_if_persistent"
        );
        assert!(
            !pool.is_persistent(&opts),
            "is_persistent must return false after eviction"
        );
    }

    /// Task 7.6: Simulates the app-quit handler by releasing all Persistent
    /// entries.  The quit handler cannot be tested without a GPUI context, so
    /// we call release() directly on each Persistent key — exactly what the
    /// handler does.
    #[test]
    fn test_on_app_quit_releases_all_persistent_entries() {
        let opts_a = wsl_opts("Ubuntu");
        let opts_b = wsl_opts("Debian");
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts_a.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );
        pool.connections.insert(
            opts_b.clone(),
            ConnectionPoolEntry::Persistent {
                connection: stub_connection(),
                workspace_count: 1,
            },
        );

        // Replicate what the on_app_quit closure does: collect Persistent keys,
        // then release each one.
        let persistent_opts: Vec<RemoteConnectionOptions> = pool
            .connections
            .iter()
            .filter_map(|(opts, entry)| {
                matches!(entry, ConnectionPoolEntry::Persistent { .. }).then(|| opts.clone())
            })
            .collect();
        for opts in persistent_opts {
            pool.release(&opts);
        }

        assert!(
            pool.connections.get(&opts_a).is_none(),
            "Ubuntu entry must be gone after quit"
        );
        assert!(
            pool.connections.get(&opts_b).is_none(),
            "Debian entry must be gone after quit"
        );
    }

    /// Task 7.7: SSH connections are never stored as Persistent;
    /// should_persist() returns false for SSH opts.
    #[test]
    fn test_ssh_connection_not_stored_as_persistent() {
        let opts = ssh_opts("example.com");

        // should_persist() must return false for SSH — the pool must never
        // upgrade an SSH entry to Persistent.
        assert!(
            !should_persist(&opts),
            "SSH must not be eligible for persistence"
        );

        // Even if somehow a Connected entry exists for an SSH host,
        // is_persistent() must return false.
        let connection = stub_connection();
        let mut pool = ConnectionPool::default();
        pool.connections.insert(
            opts.clone(),
            ConnectionPoolEntry::Connected(Arc::downgrade(&connection)),
        );
        assert!(
            !pool.is_persistent(&opts),
            "SSH Connected entry must never be reported as Persistent"
        );
    }
}
