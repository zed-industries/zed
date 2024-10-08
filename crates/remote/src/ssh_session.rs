use crate::{
    json_log::LogRecord,
    protocol::{
        message_len_from_buffer, read_message_with_len, write_message, MessageId, MESSAGE_LEN_SIZE,
    },
};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use futures::{
    channel::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::BoxFuture,
    select_biased, AsyncReadExt as _, AsyncWriteExt as _, Future, FutureExt as _, SinkExt,
    StreamExt as _,
};
use gpui::{
    AppContext, AsyncAppContext, Context, Model, ModelContext, SemanticVersion, Task, WeakModel,
};
use parking_lot::Mutex;
use rpc::{
    proto::{self, build_typed_envelope, Envelope, EnvelopedMessage, PeerId, RequestMessage},
    AnyProtoClient, EntityMessageSubscriber, ProtoClient, ProtoMessageHandlerSet, RpcError,
};
use smol::{
    fs,
    process::{self, Child, Stdio},
    Timer,
};
use std::{
    any::TypeId,
    ffi::OsStr,
    fmt,
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, Ordering::SeqCst},
        Arc,
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SshConnectionOptions {
    pub host: String,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
}

impl SshConnectionOptions {
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
    pub fn dev_server_identifier(&self) -> String {
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

pub trait SshClientDelegate: Send + Sync {
    fn ask_password(
        &self,
        prompt: String,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<String>>;
    fn remote_server_binary_path(&self, cx: &mut AsyncAppContext) -> Result<PathBuf>;
    fn get_server_binary(
        &self,
        platform: SshPlatform,
        cx: &mut AsyncAppContext,
    ) -> oneshot::Receiver<Result<(PathBuf, SemanticVersion)>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncAppContext);
    fn set_error(&self, error_message: String, cx: &mut AsyncAppContext);
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

struct ChannelForwarder {
    quit_tx: UnboundedSender<()>,
    forwarding_task: Task<(UnboundedSender<Envelope>, UnboundedReceiver<Envelope>)>,
}

impl ChannelForwarder {
    fn new(
        mut incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        cx: &AsyncAppContext,
    ) -> (Self, UnboundedSender<Envelope>, UnboundedReceiver<Envelope>) {
        let (quit_tx, mut quit_rx) = mpsc::unbounded::<()>();

        let (proxy_incoming_tx, mut proxy_incoming_rx) = mpsc::unbounded::<Envelope>();
        let (mut proxy_outgoing_tx, proxy_outgoing_rx) = mpsc::unbounded::<Envelope>();

        let forwarding_task = cx.background_executor().spawn(async move {
            loop {
                select_biased! {
                    _ = quit_rx.next().fuse() => {
                        break;
                    },
                    incoming_envelope = proxy_incoming_rx.next().fuse() => {
                        if let Some(envelope) = incoming_envelope {
                            if incoming_tx.send(envelope).await.is_err() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    outgoing_envelope = outgoing_rx.next().fuse() => {
                        if let Some(envelope) = outgoing_envelope {
                            if proxy_outgoing_tx.send(envelope).await.is_err() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            (incoming_tx, outgoing_rx)
        });

        (
            Self {
                forwarding_task,
                quit_tx,
            },
            proxy_incoming_tx,
            proxy_outgoing_rx,
        )
    }

    async fn into_channels(mut self) -> (UnboundedSender<Envelope>, UnboundedReceiver<Envelope>) {
        let _ = self.quit_tx.send(()).await;
        self.forwarding_task.await
    }
}

const MAX_MISSED_HEARTBEATS: usize = 5;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(5);

const MAX_RECONNECT_ATTEMPTS: usize = 3;

enum State {
    Connecting,
    Connected {
        ssh_connection: SshRemoteConnection,
        delegate: Arc<dyn SshClientDelegate>,
        forwarder: ChannelForwarder,

        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    HeartbeatMissed {
        missed_heartbeats: usize,

        ssh_connection: SshRemoteConnection,
        delegate: Arc<dyn SshClientDelegate>,
        forwarder: ChannelForwarder,

        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    Reconnecting,
    ReconnectFailed {
        ssh_connection: SshRemoteConnection,
        delegate: Arc<dyn SshClientDelegate>,
        forwarder: ChannelForwarder,

        error: anyhow::Error,
        attempts: usize,
    },
    ReconnectExhausted,
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
        }
    }
}

impl State {
    fn ssh_connection(&self) -> Option<&SshRemoteConnection> {
        match self {
            Self::Connected { ssh_connection, .. } => Some(ssh_connection),
            Self::HeartbeatMissed { ssh_connection, .. } => Some(ssh_connection),
            Self::ReconnectFailed { ssh_connection, .. } => Some(ssh_connection),
            _ => None,
        }
    }

    fn can_reconnect(&self) -> bool {
        matches!(
            self,
            Self::Connected { .. } | Self::HeartbeatMissed { .. } | Self::ReconnectFailed { .. }
        )
    }

    fn heartbeat_recovered(self) -> Self {
        match self {
            Self::HeartbeatMissed {
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task,
                ..
            } => Self::Connected {
                ssh_connection,
                delegate,
                forwarder,
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
                forwarder,
                multiplex_task,
                heartbeat_task,
            } => Self::HeartbeatMissed {
                missed_heartbeats: 1,
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task,
            },
            Self::HeartbeatMissed {
                missed_heartbeats,
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task,
            } => Self::HeartbeatMissed {
                missed_heartbeats: missed_heartbeats + 1,
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task,
            },
            _ => self,
        }
    }
}

/// The state of the ssh connection.
#[derive(Clone, Copy, Debug)]
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
        }
    }
}

pub struct SshRemoteClient {
    client: Arc<ChannelClient>,
    unique_identifier: String,
    connection_options: SshConnectionOptions,
    state: Arc<Mutex<Option<State>>>,
}

impl Drop for SshRemoteClient {
    fn drop(&mut self) {
        self.shutdown_processes();
    }
}

impl SshRemoteClient {
    pub fn new(
        unique_identifier: String,
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &AppContext,
    ) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
            let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();

            let client = cx.update(|cx| ChannelClient::new(incoming_rx, outgoing_tx, cx))?;
            let this = cx.new_model(|cx| {
                cx.on_app_quit(|this: &mut Self, _| {
                    this.shutdown_processes();
                    futures::future::ready(())
                })
                .detach();

                Self {
                    client: client.clone(),
                    unique_identifier: unique_identifier.clone(),
                    connection_options: connection_options.clone(),
                    state: Arc::new(Mutex::new(Some(State::Connecting))),
                }
            })?;

            let (proxy, proxy_incoming_tx, proxy_outgoing_rx) =
                ChannelForwarder::new(incoming_tx, outgoing_rx, &mut cx);

            let (ssh_connection, ssh_proxy_process) = Self::establish_connection(
                unique_identifier,
                connection_options,
                delegate.clone(),
                &mut cx,
            )
            .await?;

            let multiplex_task = Self::multiplex(
                this.downgrade(),
                ssh_proxy_process,
                proxy_incoming_tx,
                proxy_outgoing_rx,
                &mut cx,
            );

            if let Err(error) = client.ping(HEARTBEAT_TIMEOUT).await {
                log::error!("failed to establish connection: {}", error);
                delegate.set_error(error.to_string(), &mut cx);
                return Err(error);
            }

            let heartbeat_task = Self::heartbeat(this.downgrade(), &mut cx);

            this.update(&mut cx, |this, _| {
                *this.state.lock() = Some(State::Connected {
                    ssh_connection,
                    delegate,
                    forwarder: proxy,
                    multiplex_task,
                    heartbeat_task,
                });
            })?;

            Ok(this)
        })
    }

    fn shutdown_processes(&self) {
        let Some(state) = self.state.lock().take() else {
            return;
        };
        log::info!("shutting down ssh processes");

        let State::Connected {
            multiplex_task,
            heartbeat_task,
            ..
        } = state
        else {
            return;
        };
        // Drop `multiplex_task` because it owns our ssh_proxy_process, which is a
        // child of master_process.
        drop(multiplex_task);
        // Now drop the rest of state, which kills master process.
        drop(heartbeat_task);
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
            return Err(anyhow!(error));
        }

        let state = lock.take().unwrap();
        let (attempts, mut ssh_connection, delegate, forwarder) = match state {
            State::Connected {
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task,
            }
            | State::HeartbeatMissed {
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task,
                ..
            } => {
                drop(multiplex_task);
                drop(heartbeat_task);
                (0, ssh_connection, delegate, forwarder)
            }
            State::ReconnectFailed {
                attempts,
                ssh_connection,
                delegate,
                forwarder,
                ..
            } => (attempts, ssh_connection, delegate, forwarder),
            State::Connecting | State::Reconnecting | State::ReconnectExhausted => unreachable!(),
        };

        let attempts = attempts + 1;
        if attempts > MAX_RECONNECT_ATTEMPTS {
            log::error!(
                "Failed to reconnect to after {} attempts, giving up",
                MAX_RECONNECT_ATTEMPTS
            );
            *lock = Some(State::ReconnectExhausted);
            return Ok(());
        }
        *lock = Some(State::Reconnecting);
        drop(lock);

        log::info!("Trying to reconnect to ssh server... Attempt {}", attempts);

        let identifier = self.unique_identifier.clone();
        let client = self.client.clone();
        let reconnect_task = cx.spawn(|this, mut cx| async move {
            macro_rules! failed {
                ($error:expr, $attempts:expr, $ssh_connection:expr, $delegate:expr, $forwarder:expr) => {
                    return State::ReconnectFailed {
                        error: anyhow!($error),
                        attempts: $attempts,
                        ssh_connection: $ssh_connection,
                        delegate: $delegate,
                        forwarder: $forwarder,
                    };
                };
            }

            if let Err(error) = ssh_connection.master_process.kill() {
                failed!(error, attempts, ssh_connection, delegate, forwarder);
            };

            if let Err(error) = ssh_connection
                .master_process
                .status()
                .await
                .context("Failed to kill ssh process")
            {
                failed!(error, attempts, ssh_connection, delegate, forwarder);
            }

            let connection_options = ssh_connection.socket.connection_options.clone();

            let (incoming_tx, outgoing_rx) = forwarder.into_channels().await;
            let (forwarder, proxy_incoming_tx, proxy_outgoing_rx) =
                ChannelForwarder::new(incoming_tx, outgoing_rx, &mut cx);

            let (ssh_connection, ssh_process) = match Self::establish_connection(
                identifier,
                connection_options,
                delegate.clone(),
                &mut cx,
            )
            .await
            {
                Ok((ssh_connection, ssh_process)) => (ssh_connection, ssh_process),
                Err(error) => {
                    failed!(error, attempts, ssh_connection, delegate, forwarder);
                }
            };

            let multiplex_task = Self::multiplex(
                this.clone(),
                ssh_process,
                proxy_incoming_tx,
                proxy_outgoing_rx,
                &mut cx,
            );

            if let Err(error) = client.ping(HEARTBEAT_TIMEOUT).await {
                failed!(error, attempts, ssh_connection, delegate, forwarder);
            };

            State::Connected {
                ssh_connection,
                delegate,
                forwarder,
                multiplex_task,
                heartbeat_task: Self::heartbeat(this.clone(), &mut cx),
            }
        });

        cx.spawn(|this, mut cx| async move {
            let new_state = reconnect_task.await;
            this.update(&mut cx, |this, cx| {
                match &new_state {
                    State::Connecting
                    | State::Reconnecting { .. }
                    | State::HeartbeatMissed { .. } => {}
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

                let reconnect_failed = matches!(new_state, State::ReconnectFailed { .. });
                *this.state.lock() = Some(new_state);
                cx.notify();
                if reconnect_failed {
                    this.reconnect(cx)
                } else {
                    Ok(())
                }
            })
        })
        .detach_and_log_err(cx);

        Ok(())
    }

    fn heartbeat(this: WeakModel<Self>, cx: &mut AsyncAppContext) -> Task<Result<()>> {
        let Ok(client) = this.update(cx, |this, _| this.client.clone()) else {
            return Task::ready(Err(anyhow!("SshRemoteClient lost")));
        };
        cx.spawn(|mut cx| {
            let this = this.clone();
            async move {
                let mut missed_heartbeats = 0;

                let mut timer = Timer::interval(HEARTBEAT_INTERVAL);
                loop {
                    timer.next().await;

                    log::info!("Sending heartbeat to server...");

                    let result = client.ping(HEARTBEAT_TIMEOUT).await;
                    if result.is_err() {
                        missed_heartbeats += 1;
                        log::warn!(
                            "No heartbeat from server after {:?}. Missed heartbeat {} out of {}.",
                            HEARTBEAT_TIMEOUT,
                            missed_heartbeats,
                            MAX_MISSED_HEARTBEATS
                        );
                    } else {
                        missed_heartbeats = 0;
                    }

                    let result = this.update(&mut cx, |this, mut cx| {
                        this.handle_heartbeat_result(missed_heartbeats, &mut cx)
                    })?;
                    if result.is_break() {
                        return Ok(());
                    }
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
        self.state.lock().replace(if missed_heartbeats > 0 {
            state.heartbeat_missed()
        } else {
            state.heartbeat_recovered()
        });
        cx.notify();

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

    fn multiplex(
        this: WeakModel<Self>,
        mut ssh_proxy_process: Child,
        incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        cx: &AsyncAppContext,
    ) -> Task<Result<()>> {
        let mut child_stderr = ssh_proxy_process.stderr.take().unwrap();
        let mut child_stdout = ssh_proxy_process.stdout.take().unwrap();
        let mut child_stdin = ssh_proxy_process.stdin.take().unwrap();

        let io_task = cx.background_executor().spawn(async move {
            let mut stdin_buffer = Vec::new();
            let mut stdout_buffer = Vec::new();
            let mut stderr_buffer = Vec::new();
            let mut stderr_offset = 0;

            loop {
                stdout_buffer.resize(MESSAGE_LEN_SIZE, 0);
                stderr_buffer.resize(stderr_offset + 1024, 0);

                select_biased! {
                    outgoing = outgoing_rx.next().fuse() => {
                        let Some(outgoing) = outgoing else {
                            return anyhow::Ok(());
                        };

                        write_message(&mut child_stdin, &mut stdin_buffer, outgoing).await?;
                    }

                    result = child_stdout.read(&mut stdout_buffer).fuse() => {
                        match result {
                            Ok(0) => {
                                child_stdin.close().await?;
                                outgoing_rx.close();
                                let status = ssh_proxy_process.status().await?;
                                if !status.success() {
                                    log::error!("ssh process exited with status: {status:?}");
                                    return Err(anyhow!("ssh process exited with non-zero status code: {:?}", status.code()));
                                }
                                return Ok(());
                            }
                            Ok(len) => {
                                if len < stdout_buffer.len() {
                                    child_stdout.read_exact(&mut stdout_buffer[len..]).await?;
                                }

                                let message_len = message_len_from_buffer(&stdout_buffer);
                                match read_message_with_len(&mut child_stdout, &mut stdout_buffer, message_len).await {
                                    Ok(envelope) => {
                                        incoming_tx.unbounded_send(envelope).ok();
                                    }
                                    Err(error) => {
                                        log::error!("error decoding message {error:?}");
                                    }
                                }
                            }
                            Err(error) => {
                                Err(anyhow!("error reading stdout: {error:?}"))?;
                            }
                        }
                    }

                    result = child_stderr.read(&mut stderr_buffer[stderr_offset..]).fuse() => {
                        match result {
                            Ok(len) => {
                                stderr_offset += len;
                                let mut start_ix = 0;
                                while let Some(ix) = stderr_buffer[start_ix..stderr_offset].iter().position(|b| b == &b'\n') {
                                    let line_ix = start_ix + ix;
                                    let content = &stderr_buffer[start_ix..line_ix];
                                    start_ix = line_ix + 1;
                                    if let Ok(mut record) = serde_json::from_slice::<LogRecord>(content) {
                                        record.message = format!("(remote) {}", record.message);
                                        record.log(log::logger())
                                    } else {
                                        eprintln!("(remote) {}", String::from_utf8_lossy(content));
                                    }
                                }
                                stderr_buffer.drain(0..start_ix);
                                stderr_offset -= start_ix;
                            }
                            Err(error) => {
                                Err(anyhow!("error reading stderr: {error:?}"))?;
                            }
                        }
                    }
                }
            }
        });

        cx.spawn(|mut cx| async move {
            let result = io_task.await;

            if let Err(error) = result {
                log::warn!("ssh io task died with error: {:?}. reconnecting...", error);
                this.update(&mut cx, |this, cx| {
                    this.reconnect(cx).ok();
                })?;
            }

            Ok(())
        })
    }

    async fn establish_connection(
        unique_identifier: String,
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<(SshRemoteConnection, Child)> {
        let ssh_connection =
            SshRemoteConnection::new(connection_options, delegate.clone(), cx).await?;

        let platform = ssh_connection.query_platform().await?;
        let (local_binary_path, version) = delegate.get_server_binary(platform, cx).await??;
        let remote_binary_path = delegate.remote_server_binary_path(cx)?;
        ssh_connection
            .ensure_server_binary(
                &delegate,
                &local_binary_path,
                &remote_binary_path,
                version,
                cx,
            )
            .await?;

        let socket = ssh_connection.socket.clone();
        run_cmd(socket.ssh_command(&remote_binary_path).arg("version")).await?;

        delegate.set_status(Some("Starting proxy"), cx);

        let ssh_proxy_process = socket
            .ssh_command(format!(
                "RUST_LOG={} RUST_BACKTRACE={} {:?} proxy --identifier {}",
                std::env::var("RUST_LOG").unwrap_or_default(),
                std::env::var("RUST_BACKTRACE").unwrap_or_default(),
                remote_binary_path,
                unique_identifier,
            ))
            // IMPORTANT: we kill this process when we drop the task that uses it.
            .kill_on_drop(true)
            .spawn()
            .context("failed to spawn remote server")?;

        Ok((ssh_connection, ssh_proxy_process))
    }

    pub fn subscribe_to_entity<E: 'static>(&self, remote_id: u64, entity: &Model<E>) {
        self.client.subscribe_to_entity(remote_id, entity);
    }

    pub fn ssh_args(&self) -> Option<Vec<String>> {
        self.state
            .lock()
            .as_ref()
            .and_then(|state| state.ssh_connection())
            .map(|ssh_connection| ssh_connection.socket.ssh_args())
    }

    pub fn to_proto_client(&self) -> AnyProtoClient {
        self.client.clone().into()
    }

    pub fn connection_string(&self) -> String {
        self.connection_options.connection_string()
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.state
            .lock()
            .as_ref()
            .map(ConnectionState::from)
            .unwrap_or(ConnectionState::Disconnected)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (Model<Self>, Arc<ChannelClient>) {
        use gpui::Context;

        let (server_to_client_tx, server_to_client_rx) = mpsc::unbounded();
        let (client_to_server_tx, client_to_server_rx) = mpsc::unbounded();

        (
            client_cx.update(|cx| {
                let client = ChannelClient::new(server_to_client_rx, client_to_server_tx, cx);
                cx.new_model(|_| Self {
                    client,
                    unique_identifier: "fake".to_string(),
                    connection_options: SshConnectionOptions::default(),
                    state: Arc::new(Mutex::new(None)),
                })
            }),
            server_cx.update(|cx| ChannelClient::new(client_to_server_rx, server_to_client_tx, cx)),
        )
    }
}

impl From<SshRemoteClient> for AnyProtoClient {
    fn from(client: SshRemoteClient) -> Self {
        AnyProtoClient::new(client.client.clone())
    }
}

struct SshRemoteConnection {
    socket: SshSocket,
    master_process: process::Child,
    _temp_dir: TempDir,
}

impl Drop for SshRemoteConnection {
    fn drop(&mut self) {
        if let Err(error) = self.master_process.kill() {
            log::error!("failed to kill SSH master process: {}", error);
        }
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
        use futures::{io::BufReader, AsyncBufReadExt as _};
        use smol::{fs::unix::PermissionsExt as _, net::unix::UnixListener};
        use util::ResultExt as _;

        delegate.set_status(Some("connecting"), cx);

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
            .args(["-N", "-o", "ControlMaster=yes", "-o"])
            .arg(format!("ControlPath={}", socket_path.display()))
            .arg(&url)
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
            let error_message = format!("Failed to connect to host: {}.", e);
            delegate.set_error(error_message, cx);
            return Err(e);
        }

        drop(askpass_task);

        if master_process.try_status()?.is_some() {
            output.clear();
            let mut stderr = master_process.stderr.take().unwrap();
            stderr.read_to_end(&mut output).await?;

            let error_message = format!("failed to connect: {}", String::from_utf8_lossy(&output));
            delegate.set_error(error_message.clone(), cx);
            Err(anyhow!(error_message))?;
        }

        Ok(Self {
            socket: SshSocket {
                connection_options,
                socket_path,
            },
            master_process,
            _temp_dir: temp_dir,
        })
    }

    async fn ensure_server_binary(
        &self,
        delegate: &Arc<dyn SshClientDelegate>,
        src_path: &Path,
        dst_path: &Path,
        version: SemanticVersion,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let mut dst_path_gz = dst_path.to_path_buf();
        dst_path_gz.set_extension("gz");

        if let Some(parent) = dst_path.parent() {
            run_cmd(self.socket.ssh_command("mkdir").arg("-p").arg(parent)).await?;
        }

        let mut server_binary_exists = false;
        if cfg!(not(debug_assertions)) {
            if let Ok(installed_version) =
                run_cmd(self.socket.ssh_command(dst_path).arg("version")).await
            {
                if installed_version.trim() == version.to_string() {
                    server_binary_exists = true;
                }
            }
        }

        if server_binary_exists {
            log::info!("remote development server already present",);
            return Ok(());
        }

        let src_stat = fs::metadata(src_path).await?;
        let size = src_stat.len();
        let server_mode = 0o755;

        let t0 = Instant::now();
        delegate.set_status(Some("uploading remote development server"), cx);
        log::info!("uploading remote development server ({}kb)", size / 1024);
        self.upload_file(src_path, &dst_path_gz)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());

        delegate.set_status(Some("extracting remote development server"), cx);
        run_cmd(
            self.socket
                .ssh_command("gunzip")
                .arg("--force")
                .arg(&dst_path_gz),
        )
        .await?;

        delegate.set_status(Some("unzipping remote development server"), cx);
        run_cmd(
            self.socket
                .ssh_command("chmod")
                .arg(format!("{:o}", server_mode))
                .arg(dst_path),
        )
        .await?;

        Ok(())
    }

    async fn query_platform(&self) -> Result<SshPlatform> {
        let os = run_cmd(self.socket.ssh_command("uname").arg("-s")).await?;
        let arch = run_cmd(self.socket.ssh_command("uname").arg("-m")).await?;

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

        Ok(SshPlatform { os, arch })
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
    outgoing_tx: mpsc::UnboundedSender<Envelope>,
    response_channels: ResponseChannels,             // Lock
    message_handlers: Mutex<ProtoMessageHandlerSet>, // Lock
}

impl ChannelClient {
    pub fn new(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &AppContext,
    ) -> Arc<Self> {
        let this = Arc::new(Self {
            outgoing_tx,
            next_message_id: AtomicU32::new(0),
            response_channels: ResponseChannels::default(),
            message_handlers: Default::default(),
        });

        Self::start_handling_messages(this.clone(), incoming_rx, cx);

        this
    }

    fn start_handling_messages(
        this: Arc<Self>,
        mut incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        cx: &AppContext,
    ) {
        cx.spawn(|cx| {
            let this = Arc::downgrade(&this);
            async move {
                let peer_id = PeerId { owner_id: 0, id: 0 };
                while let Some(incoming) = incoming_rx.next().await {
                    let Some(this) = this.upgrade() else {
                        return anyhow::Ok(());
                    };

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
                            log::debug!("ssh message received. name:{type_name}");
                            match future.await {
                                Ok(_) => {
                                    log::debug!("ssh message handled. name:{type_name}");
                                }
                                Err(error) => {
                                    log::error!(
                                        "error handling message. type:{type_name}, error:{error}",
                                    );
                                }
                            }
                        } else {
                            log::error!("unhandled ssh message name:{type_name}");
                        }
                    }
                }
                anyhow::Ok(())
            }
        })
        .detach();
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
        log::debug!("ssh request start. name:{}", T::NAME);
        let response = self.request_dynamic(payload.into_envelope(0, None, None), T::NAME);
        async move {
            let response = response.await?;
            log::debug!("ssh request finish. name:{}", T::NAME);
            T::Response::from_envelope(response)
                .ok_or_else(|| anyhow!("received a response of the wrong type"))
        }
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

    pub fn request_dynamic(
        &self,
        mut envelope: proto::Envelope,
        type_name: &'static str,
    ) -> impl 'static + Future<Output = Result<proto::Envelope>> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        let (tx, rx) = oneshot::channel();
        let mut response_channels_lock = self.response_channels.lock();
        response_channels_lock.insert(MessageId(envelope.id), tx);
        drop(response_channels_lock);
        let result = self.outgoing_tx.unbounded_send(envelope);
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
        self.outgoing_tx.unbounded_send(envelope)?;
        Ok(())
    }
}

impl ProtoClient for ChannelClient {
    fn request(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>> {
        self.request_dynamic(envelope, request_type).boxed()
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
