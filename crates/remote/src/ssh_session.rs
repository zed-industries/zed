use crate::{
    json_log::LogRecord,
    protocol::{
        MESSAGE_LEN_SIZE, MessageId, message_len_from_buffer, read_message_with_len, write_message,
    },
    proxy::ProxyLaunchError,
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{
    AsyncReadExt as _, Future, FutureExt as _, StreamExt as _,
    channel::{
        mpsc::{self, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::{BoxFuture, Shared},
    select, select_biased,
};
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, BorrowAppContext, Context, Entity,
    EventEmitter, Global, SemanticVersion, Task, WeakEntity,
};
use itertools::Itertools;
use parking_lot::Mutex;

use release_channel::{AppCommitSha, AppVersion, ReleaseChannel};
use rpc::{
    AnyProtoClient, ErrorExt, ProtoClient, ProtoMessageHandlerSet, RpcError,
    proto::{self, Envelope, EnvelopedMessage, PeerId, RequestMessage, build_typed_envelope},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::{
    fs,
    process::{self, Child, Stdio},
};
use std::{
    collections::VecDeque,
    fmt, iter,
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::{
        Arc, Weak,
        atomic::{AtomicU32, AtomicU64, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
use tempfile::TempDir;
use util::{
    ResultExt,
    paths::{PathStyle, RemotePathBuf},
};

#[derive(Clone)]
pub struct SshSocket {
    connection_options: SshConnectionOptions,
    #[cfg(not(target_os = "windows"))]
    socket_path: PathBuf,
    #[cfg(target_os = "windows")]
    envs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema)]
pub struct SshPortForwardOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_host: Option<String>,
    pub local_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    pub remote_port: u16,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SshConnectionOptions {
    pub host: String,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub args: Option<Vec<String>>,
    pub port_forwards: Option<Vec<SshPortForwardOption>>,

    pub nickname: Option<String>,
    pub upload_binary_over_ssh: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshArgs {
    pub arguments: Vec<String>,
    pub envs: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshInfo {
    pub args: SshArgs,
    pub path_style: PathStyle,
    pub shell: String,
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

fn parse_port_number(port_str: &str) -> Result<u16> {
    port_str
        .parse()
        .with_context(|| format!("parsing port number: {port_str}"))
}

fn parse_port_forward_spec(spec: &str) -> Result<SshPortForwardOption> {
    let parts: Vec<&str> = spec.split(':').collect();

    match parts.len() {
        4 => {
            let local_port = parse_port_number(parts[1])?;
            let remote_port = parse_port_number(parts[3])?;

            Ok(SshPortForwardOption {
                local_host: Some(parts[0].to_string()),
                local_port,
                remote_host: Some(parts[2].to_string()),
                remote_port,
            })
        }
        3 => {
            let local_port = parse_port_number(parts[0])?;
            let remote_port = parse_port_number(parts[2])?;

            Ok(SshPortForwardOption {
                local_host: None,
                local_port,
                remote_host: Some(parts[1].to_string()),
                remote_port,
            })
        }
        _ => anyhow::bail!("Invalid port forward format"),
    }
}

impl SshConnectionOptions {
    pub fn parse_command_line(input: &str) -> Result<Self> {
        let input = input.trim_start_matches("ssh ");
        let mut hostname: Option<String> = None;
        let mut username: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut args = Vec::new();
        let mut port_forwards: Vec<SshPortForwardOption> = Vec::new();

        // disallowed: -E, -e, -F, -f, -G, -g, -M, -N, -n, -O, -q, -S, -s, -T, -t, -V, -v, -W
        const ALLOWED_OPTS: &[&str] = &[
            "-4", "-6", "-A", "-a", "-C", "-K", "-k", "-X", "-x", "-Y", "-y",
        ];
        const ALLOWED_ARGS: &[&str] = &[
            "-B", "-b", "-c", "-D", "-F", "-I", "-i", "-J", "-l", "-m", "-o", "-P", "-p", "-R",
            "-w",
        ];

        let mut tokens = shlex::split(input).context("invalid input")?.into_iter();

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
            if arg == "-L" || arg.starts_with("-L") {
                let forward_spec = if arg == "-L" {
                    tokens.next()
                } else {
                    Some(arg.strip_prefix("-L").unwrap().to_string())
                };

                if let Some(spec) = forward_spec {
                    port_forwards.push(parse_port_forward_spec(&spec)?);
                } else {
                    anyhow::bail!("Missing port forward format");
                }
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
            // Destination might be: username1@username2@ip2@ip1
            if let Some((u, rest)) = input.rsplit_once('@') {
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

        let port_forwards = match port_forwards.len() {
            0 => None,
            _ => Some(port_forwards),
        };

        Ok(Self {
            host: hostname,
            username,
            port,
            port_forwards,
            args: Some(args),
            password: None,
            nickname: None,
            upload_binary_over_ssh: false,
        })
    }

    pub fn ssh_url(&self) -> String {
        let mut result = String::from("ssh://");
        if let Some(username) = &self.username {
            // Username might be: username1@username2@ip2
            let username = urlencoding::encode(username);
            result.push_str(&username);
            result.push('@');
        }
        result.push_str(&self.host);
        if let Some(port) = self.port {
            result.push(':');
            result.push_str(&port.to_string());
        }
        result
    }

    pub fn additional_args(&self) -> Vec<String> {
        let mut args = self.args.iter().flatten().cloned().collect::<Vec<String>>();

        if let Some(forwards) = &self.port_forwards {
            args.extend(forwards.iter().map(|pf| {
                let local_host = match &pf.local_host {
                    Some(host) => host,
                    None => "localhost",
                };
                let remote_host = match &pf.remote_host {
                    Some(host) => host,
                    None => "localhost",
                };

                format!(
                    "-L{}:{}:{}:{}",
                    local_host, pf.local_port, remote_host, pf.remote_port
                )
            }));
        }

        args
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

pub trait SshClientDelegate: Send + Sync {
    fn ask_password(&self, prompt: String, tx: oneshot::Sender<String>, cx: &mut AsyncApp);
    fn get_download_params(
        &self,
        platform: SshPlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Option<(String, String)>>>;

    fn download_server_binary_locally(
        &self,
        platform: SshPlatform,
        release_channel: ReleaseChannel,
        version: Option<SemanticVersion>,
        cx: &mut AsyncApp,
    ) -> Task<Result<PathBuf>>;
    fn set_status(&self, status: Option<&str>, cx: &mut AsyncApp);
}

impl SshSocket {
    #[cfg(not(target_os = "windows"))]
    fn new(options: SshConnectionOptions, socket_path: PathBuf) -> Result<Self> {
        Ok(Self {
            connection_options: options,
            socket_path,
        })
    }

    #[cfg(target_os = "windows")]
    fn new(options: SshConnectionOptions, temp_dir: &TempDir, secret: String) -> Result<Self> {
        let askpass_script = temp_dir.path().join("askpass.bat");
        std::fs::write(&askpass_script, "@ECHO OFF\necho %ZED_SSH_ASKPASS%")?;
        let mut envs = HashMap::default();
        envs.insert("SSH_ASKPASS_REQUIRE".into(), "force".into());
        envs.insert("SSH_ASKPASS".into(), askpass_script.display().to_string());
        envs.insert("ZED_SSH_ASKPASS".into(), secret);
        Ok(Self {
            connection_options: options,
            envs,
        })
    }

    // :WARNING: ssh unquotes arguments when executing on the remote :WARNING:
    // e.g. $ ssh host sh -c 'ls -l' is equivalent to $ ssh host sh -c ls -l
    // and passes -l as an argument to sh, not to ls.
    // Furthermore, some setups (e.g. Coder) will change directory when SSH'ing
    // into a machine. You must use `cd` to get back to $HOME.
    // You need to do it like this: $ ssh host "cd; sh -c 'ls -l /tmp'"
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
        let to_run = format!("cd; {to_run}");
        log::debug!("ssh {} {:?}", self.connection_options.ssh_url(), to_run);
        self.ssh_options(&mut command)
            .arg(self.connection_options.ssh_url())
            .arg(to_run);
        command
    }

    async fn run_command(&self, program: &str, args: &[&str]) -> Result<String> {
        let output = self.ssh_command(program, args).output().await?;
        anyhow::ensure!(
            output.status.success(),
            "failed to run command: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[cfg(not(target_os = "windows"))]
    fn ssh_options<'a>(&self, command: &'a mut process::Command) -> &'a mut process::Command {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(self.connection_options.additional_args())
            .args(["-o", "ControlMaster=no", "-o"])
            .arg(format!("ControlPath={}", self.socket_path.display()))
    }

    #[cfg(target_os = "windows")]
    fn ssh_options<'a>(&self, command: &'a mut process::Command) -> &'a mut process::Command {
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(self.connection_options.additional_args())
            .envs(self.envs.clone())
    }

    // On Windows, we need to use `SSH_ASKPASS` to provide the password to ssh.
    // On Linux, we use the `ControlPath` option to create a socket file that ssh can use to
    #[cfg(not(target_os = "windows"))]
    fn ssh_args(&self) -> SshArgs {
        let mut arguments = self.connection_options.additional_args();
        arguments.extend(vec![
            "-o".to_string(),
            "ControlMaster=no".to_string(),
            "-o".to_string(),
            format!("ControlPath={}", self.socket_path.display()),
            self.connection_options.ssh_url(),
        ]);
        SshArgs {
            arguments,
            envs: None,
        }
    }

    #[cfg(target_os = "windows")]
    fn ssh_args(&self) -> SshArgs {
        let mut arguments = self.connection_options.additional_args();
        arguments.push(self.connection_options.ssh_url());
        SshArgs {
            arguments,
            envs: Some(self.envs.clone()),
        }
    }

    async fn platform(&self) -> Result<SshPlatform> {
        let uname = self.run_command("sh", &["-c", "uname -sm"]).await?;
        let Some((os, arch)) = uname.split_once(" ") else {
            anyhow::bail!("unknown uname: {uname:?}")
        };

        let os = match os.trim() {
            "Darwin" => "macos",
            "Linux" => "linux",
            _ => anyhow::bail!(
                "Prebuilt remote servers are not yet available for {os:?}. See https://zed.dev/docs/remote-development"
            ),
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
            anyhow::bail!(
                "Prebuilt remote servers are not yet available for {arch:?}. See https://zed.dev/docs/remote-development"
            )
        };

        Ok(SshPlatform { os, arch })
    }

    async fn shell(&self) -> String {
        match self.run_command("sh", &["-c", "echo $SHELL"]).await {
            Ok(shell) => shell.trim().to_owned(),
            Err(e) => {
                log::error!("Failed to get shell: {e}");
                "sh".to_owned()
            }
        }
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
    path_style: PathStyle,
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

impl SshRemoteClient {
    pub fn new(
        unique_identifier: ConnectionIdentifier,
        connection_options: SshConnectionOptions,
        cancellation: oneshot::Receiver<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut App,
    ) -> Task<Result<Option<Entity<Self>>>> {
        let unique_identifier = unique_identifier.to_string(cx);
        cx.spawn(async move |cx| {
            let success = Box::pin(async move {
                let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
                let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();
                let (connection_activity_tx, connection_activity_rx) = mpsc::channel::<()>(1);

                let client =
                    cx.update(|cx| ChannelClient::new(incoming_rx, outgoing_tx, cx, "client"))?;

                let ssh_connection = cx
                    .update(|cx| {
                        cx.update_default_global(|pool: &mut ConnectionPool, cx| {
                            pool.connect(connection_options.clone(), &delegate, cx)
                        })
                    })?
                    .await
                    .map_err(|e| e.cloned())?;

                let path_style = ssh_connection.path_style();
                let this = cx.new(|_| Self {
                    client: client.clone(),
                    unique_identifier: unique_identifier.clone(),
                    connection_options,
                    path_style,
                    state: Arc::new(Mutex::new(Some(State::Connecting))),
                })?;

                let io_task = ssh_connection.start_proxy(
                    unique_identifier,
                    false,
                    incoming_tx,
                    outgoing_rx,
                    connection_activity_tx,
                    delegate.clone(),
                    cx,
                );

                let multiplex_task = Self::monitor(this.downgrade(), io_task, cx);

                if let Err(error) = client.ping(HEARTBEAT_TIMEOUT).await {
                    log::error!("failed to establish connection: {}", error);
                    return Err(error);
                }

                let heartbeat_task = Self::heartbeat(this.downgrade(), connection_activity_rx, cx);

                this.update(cx, |this, _| {
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

    pub fn proto_client_from_channels(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &App,
        name: &'static str,
    ) -> AnyProtoClient {
        ChannelClient::new(incoming_rx, outgoing_tx, cx, name).into()
    }

    pub fn shutdown_processes<T: RequestMessage>(
        &self,
        shutdown_request: Option<T>,
        executor: BackgroundExecutor,
    ) -> Option<impl Future<Output = ()> + use<T>> {
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
                executor.timer(Duration::from_millis(50)).await;
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

    fn reconnect(&mut self, cx: &mut Context<Self>) -> Result<()> {
        let mut lock = self.state.lock();

        let can_reconnect = lock
            .as_ref()
            .map(|state| state.can_reconnect())
            .unwrap_or(false);
        if !can_reconnect {
            log::info!("aborting reconnect, because not in state that allows reconnecting");
            let error = if let Some(state) = lock.as_ref() {
                format!("invalid state, cannot reconnect while in state {state}")
            } else {
                "no state set".to_string()
            };
            anyhow::bail!(error);
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
        let reconnect_task = cx.spawn(async move |this, cx| {
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
                    cx,
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

            let multiplex_task = Self::monitor(this.clone(), io_task, cx);
            client.reconnect(incoming_rx, outgoing_tx, cx);

            if let Err(error) = client.resync(HEARTBEAT_TIMEOUT).await {
                failed!(error, attempts, ssh_connection, delegate);
            };

            State::Connected {
                ssh_connection,
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
            return Task::ready(Err(anyhow!("SshRemoteClient lost")));
        };

        cx.spawn(async move |cx| {
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
                                    this.set_state(State::ServerNotRunning, cx);
                                })?;
                            }
                        }
                    } else if exit_code > 0 {
                        log::error!("proxy process terminated unexpectedly");
                        this.update(cx, |this, cx| {
                            this.reconnect(cx).ok();
                        })?;
                    }
                }
                Err(error) => {
                    log::warn!("ssh io task died with error: {:?}. reconnecting...", error);
                    this.update(cx, |this, cx| {
                        this.reconnect(cx).ok();
                    })?;
                }
            }

            Ok(())
        })
    }

    fn state_is(&self, check: impl FnOnce(&State) -> bool) -> bool {
        self.state.lock().as_ref().is_some_and(check)
    }

    fn try_set_state(&self, cx: &mut Context<Self>, map: impl FnOnce(&State) -> Option<State>) {
        let mut lock = self.state.lock();
        let new_state = lock.as_ref().and_then(map);

        if let Some(new_state) = new_state {
            lock.replace(new_state);
            cx.notify();
        }
    }

    fn set_state(&self, state: State, cx: &mut Context<Self>) {
        log::info!("setting state to '{}'", &state);

        let is_reconnect_exhausted = state.is_reconnect_exhausted();
        let is_server_not_running = state.is_server_not_running();
        self.state.lock().replace(state);

        if is_reconnect_exhausted || is_server_not_running {
            cx.emit(SshRemoteEvent::Disconnected);
        }
        cx.notify();
    }

    pub fn ssh_info(&self) -> Option<SshInfo> {
        self.state
            .lock()
            .as_ref()
            .and_then(|state| state.ssh_connection())
            .map(|ssh_connection| SshInfo {
                args: ssh_connection.ssh_args(),
                path_style: ssh_connection.path_style(),
                shell: ssh_connection.shell(),
            })
    }

    pub fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
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

    pub fn path_style(&self) -> PathStyle {
        self.path_style
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_disconnect(&self, client_cx: &mut App) -> Task<()> {
        let opts = self.connection_options();
        client_cx.spawn(async move |cx| {
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

            connection.simulate_disconnect(cx);
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_server(
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (SshConnectionOptions, AnyProtoClient) {
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
                        cx.background_spawn({
                            let connection = connection.clone();
                            async move { Ok(connection.clone()) }
                        })
                        .shared(),
                    ),
                );
            })
        });

        (opts, server_client.into())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn fake_client(
        opts: SshConnectionOptions,
        client_cx: &mut gpui::TestAppContext,
    ) -> Entity<Self> {
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
        cx: &mut App,
    ) -> Shared<Task<Result<Arc<dyn RemoteConnection>, Arc<anyhow::Error>>>> {
        let connection = self.connections.get(&opts);
        match connection {
            Some(ConnectionPoolEntry::Connecting(task)) => {
                let delegate = delegate.clone();
                cx.spawn(async move |cx| {
                    delegate.set_status(Some("Waiting for existing connection attempt"), cx);
                })
                .detach();
                return task.clone();
            }
            Some(ConnectionPoolEntry::Connected(ssh)) => {
                if let Some(ssh) = ssh.upgrade()
                    && !ssh.has_been_killed()
                {
                    return Task::ready(Ok(ssh)).shared();
                }
                self.connections.remove(&opts);
            }
            None => {}
        }

        let task = cx
            .spawn({
                let opts = opts.clone();
                let delegate = delegate.clone();
                async move |cx| {
                    let connection = SshRemoteConnection::new(opts.clone(), delegate, cx)
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
        AnyProtoClient::new(client.client)
    }
}

#[async_trait(?Send)]
trait RemoteConnection: Send + Sync {
    fn start_proxy(
        &self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
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
    /// On Windows, we need to use `SSH_ASKPASS` to provide the password to ssh.
    /// On Linux, we use the `ControlPath` option to create a socket file that ssh can use to
    fn ssh_args(&self) -> SshArgs;
    fn connection_options(&self) -> SshConnectionOptions;
    fn path_style(&self) -> PathStyle;
    fn shell(&self) -> String;

    #[cfg(any(test, feature = "test-support"))]
    fn simulate_disconnect(&self, _: &AsyncApp) {}
}

struct SshRemoteConnection {
    socket: SshSocket,
    master_process: Mutex<Option<Child>>,
    remote_binary_path: Option<RemotePathBuf>,
    ssh_platform: SshPlatform,
    ssh_path_style: PathStyle,
    ssh_shell: String,
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

    fn ssh_args(&self) -> SshArgs {
        self.socket.ssh_args()
    }

    fn connection_options(&self) -> SshConnectionOptions {
        self.socket.connection_options.clone()
    }

    fn shell(&self) -> String {
        self.ssh_shell.clone()
    }

    fn upload_directory(
        &self,
        src_path: PathBuf,
        dest_path: RemotePathBuf,
        cx: &App,
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
                dest_path
            ))
            .output();

        cx.background_spawn(async move {
            let output = output.await?;

            anyhow::ensure!(
                output.status.success(),
                "failed to upload directory {} -> {}: {}",
                src_path.display(),
                dest_path.to_string(),
                String::from_utf8_lossy(&output.stderr)
            );

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
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        delegate.set_status(Some("Starting proxy"), cx);

        let Some(remote_binary_path) = self.remote_binary_path.clone() else {
            return Task::ready(Err(anyhow!("Remote binary path not set")));
        };

        let mut start_proxy_command = shell_script!(
            "exec {binary_path} proxy --identifier {identifier}",
            binary_path = &remote_binary_path.to_string(),
            identifier = &unique_identifier,
        );

        for env_var in ["RUST_LOG", "RUST_BACKTRACE", "ZED_GENERATE_MINIDUMPS"] {
            if let Some(value) = std::env::var(env_var).ok() {
                start_proxy_command = format!(
                    "{}={} {} ",
                    env_var,
                    shlex::try_quote(&value).unwrap(),
                    start_proxy_command,
                );
            }
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
                return Task::ready(Err(anyhow!("failed to spawn remote server: {}", error)));
            }
        };

        Self::multiplex(
            ssh_proxy_process,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            cx,
        )
    }

    fn path_style(&self) -> PathStyle {
        self.ssh_path_style
    }
}

impl SshRemoteConnection {
    async fn new(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        use askpass::AskPassResult;

        delegate.set_status(Some("Connecting"), cx);

        let url = connection_options.ssh_url();

        let temp_dir = tempfile::Builder::new()
            .prefix("zed-ssh-session")
            .tempdir()?;
        let askpass_delegate = askpass::AskPassDelegate::new(cx, {
            let delegate = delegate.clone();
            move |prompt, tx, cx| delegate.ask_password(prompt, tx, cx)
        });

        let mut askpass =
            askpass::AskPassSession::new(cx.background_executor(), askpass_delegate).await?;

        // Start the master SSH process, which does not do anything except for establish
        // the connection and keep it open, allowing other ssh commands to reuse it
        // via a control socket.
        #[cfg(not(target_os = "windows"))]
        let socket_path = temp_dir.path().join("ssh.sock");

        let mut master_process = {
            #[cfg(not(target_os = "windows"))]
            let args = [
                "-N",
                "-o",
                "ControlPersist=no",
                "-o",
                "ControlMaster=yes",
                "-o",
            ];
            // On Windows, `ControlMaster` and `ControlPath` are not supported:
            // https://github.com/PowerShell/Win32-OpenSSH/issues/405
            // https://github.com/PowerShell/Win32-OpenSSH/wiki/Project-Scope
            #[cfg(target_os = "windows")]
            let args = ["-N"];
            let mut master_process = util::command::new_smol_command("ssh");
            master_process
                .kill_on_drop(true)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .env("SSH_ASKPASS_REQUIRE", "force")
                .env("SSH_ASKPASS", askpass.script_path())
                .args(connection_options.additional_args())
                .args(args);
            #[cfg(not(target_os = "windows"))]
            master_process.arg(format!("ControlPath={}", socket_path.display()));
            master_process.arg(&url).spawn()?
        };
        // Wait for this ssh process to close its stdout, indicating that authentication
        // has completed.
        let mut stdout = master_process.stdout.take().unwrap();
        let mut output = Vec::new();

        let result = select_biased! {
            result = askpass.run().fuse() => {
                match result {
                    AskPassResult::CancelledByUser => {
                        master_process.kill().ok();
                        anyhow::bail!("SSH connection canceled")
                    }
                    AskPassResult::Timedout => {
                        anyhow::bail!("connecting to host timed out")
                    }
                }
            }
            _ = stdout.read_to_end(&mut output).fuse() => {
                anyhow::Ok(())
            }
        };

        if let Err(e) = result {
            return Err(e.context("Failed to connect to host"));
        }

        if master_process.try_status()?.is_some() {
            output.clear();
            let mut stderr = master_process.stderr.take().unwrap();
            stderr.read_to_end(&mut output).await?;

            let error_message = format!(
                "failed to connect: {}",
                String::from_utf8_lossy(&output).trim()
            );
            anyhow::bail!(error_message);
        }

        #[cfg(not(target_os = "windows"))]
        let socket = SshSocket::new(connection_options, socket_path)?;
        #[cfg(target_os = "windows")]
        let socket = SshSocket::new(connection_options, &temp_dir, askpass.get_password())?;
        drop(askpass);

        let ssh_platform = socket.platform().await?;
        let ssh_path_style = match ssh_platform.os {
            "windows" => PathStyle::Windows,
            _ => PathStyle::Posix,
        };
        let ssh_shell = socket.shell().await;

        let mut this = Self {
            socket,
            master_process: Mutex::new(Some(master_process)),
            _temp_dir: temp_dir,
            remote_binary_path: None,
            ssh_path_style,
            ssh_platform,
            ssh_shell,
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

    fn multiplex(
        mut ssh_proxy_process: Child,
        incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        mut connection_activity_tx: Sender<()>,
        cx: &AsyncApp,
    ) -> Task<Result<i32>> {
        let mut child_stderr = ssh_proxy_process.stderr.take().unwrap();
        let mut child_stdout = ssh_proxy_process.stdout.take().unwrap();
        let mut child_stdin = ssh_proxy_process.stdin.take().unwrap();

        let mut stdin_buffer = Vec::new();
        let mut stdout_buffer = Vec::new();
        let mut stderr_buffer = Vec::new();
        let mut stderr_offset = 0;

        let stdin_task = cx.background_spawn(async move {
            while let Some(outgoing) = outgoing_rx.next().await {
                write_message(&mut child_stdin, &mut stdin_buffer, outgoing).await?;
            }
            anyhow::Ok(())
        });

        let stdout_task = cx.background_spawn({
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

        let stderr_task: Task<anyhow::Result<()>> = cx.background_spawn(async move {
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

        cx.background_spawn(async move {
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
        cx: &mut AsyncApp,
    ) -> Result<RemotePathBuf> {
        let version_str = match release_channel {
            ReleaseChannel::Nightly => {
                let commit = commit.map(|s| s.full()).unwrap_or_default();
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
        let dst_path = RemotePathBuf::new(
            paths::remote_server_dir_relative().join(binary_name),
            self.ssh_path_style,
        );

        let build_remote_server = std::env::var("ZED_BUILD_REMOTE_SERVER").ok();
        #[cfg(debug_assertions)]
        if let Some(build_remote_server) = build_remote_server {
            let src_path = self.build_local(build_remote_server, delegate, cx).await?;
            let tmp_path = RemotePathBuf::new(
                paths::remote_server_dir_relative().join(format!(
                    "download-{}-{}",
                    std::process::id(),
                    src_path.file_name().unwrap().to_string_lossy()
                )),
                self.ssh_path_style,
            );
            self.upload_local_server_binary(&src_path, &tmp_path, delegate, cx)
                .await?;
            self.extract_server_binary(&dst_path, &tmp_path, delegate, cx)
                .await?;
            return Ok(dst_path);
        }

        if self
            .socket
            .run_command(&dst_path.to_string(), &["version"])
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

        let tmp_path_gz = RemotePathBuf::new(
            PathBuf::from(format!("{}-download-{}.gz", dst_path, std::process::id())),
            self.ssh_path_style,
        );
        if !self.socket.connection_options.upload_binary_over_ssh
            && let Some((url, body)) = delegate
                .get_download_params(self.ssh_platform, release_channel, wanted_version, cx)
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

        let src_path = delegate
            .download_server_binary_locally(self.ssh_platform, release_channel, wanted_version, cx)
            .await?;
        self.upload_local_server_binary(&src_path, &tmp_path_gz, delegate, cx)
            .await?;
        self.extract_server_binary(&dst_path, &tmp_path_gz, delegate, cx)
            .await?;
        Ok(dst_path)
    }

    async fn download_binary_on_server(
        &self,
        url: &str,
        body: &str,
        tmp_path_gz: &RemotePathBuf,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.socket
                .run_command(
                    "sh",
                    &[
                        "-c",
                        &shell_script!("mkdir -p {parent}", parent = parent.to_string().as_ref()),
                    ],
                )
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
                    body,
                    url,
                    "-o",
                    &tmp_path_gz.to_string(),
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
                            "--method=GET",
                            "--header=Content-Type: application/json",
                            "--body-data",
                            body,
                            url,
                            "-O",
                            &tmp_path_gz.to_string(),
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
        tmp_path_gz: &RemotePathBuf,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        if let Some(parent) = tmp_path_gz.parent() {
            self.socket
                .run_command(
                    "sh",
                    &[
                        "-c",
                        &shell_script!("mkdir -p {parent}", parent = parent.to_string().as_ref()),
                    ],
                )
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
        self.upload_file(src_path, tmp_path_gz)
            .await
            .context("failed to upload server binary")?;
        log::info!("uploaded remote development server in {:?}", t0.elapsed());
        Ok(())
    }

    async fn extract_server_binary(
        &self,
        dst_path: &RemotePathBuf,
        tmp_path: &RemotePathBuf,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        delegate.set_status(Some("Extracting remote development server"), cx);
        let server_mode = 0o755;

        let orig_tmp_path = tmp_path.to_string();
        let script = if let Some(tmp_path) = orig_tmp_path.strip_suffix(".gz") {
            shell_script!(
                "gunzip -f {orig_tmp_path} && chmod {server_mode} {tmp_path} && mv {tmp_path} {dst_path}",
                server_mode = &format!("{:o}", server_mode),
                dst_path = &dst_path.to_string(),
            )
        } else {
            shell_script!(
                "chmod {server_mode} {orig_tmp_path} && mv {orig_tmp_path} {dst_path}",
                server_mode = &format!("{:o}", server_mode),
                dst_path = &dst_path.to_string()
            )
        };
        self.socket.run_command("sh", &["-c", &script]).await?;
        Ok(())
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &RemotePathBuf) -> Result<()> {
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
                dest_path
            ))
            .output()
            .await?;

        anyhow::ensure!(
            output.status.success(),
            "failed to upload file {} -> {}: {}",
            src_path.display(),
            dest_path.to_string(),
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    #[cfg(debug_assertions)]
    async fn build_local(
        &self,
        build_remote_server: String,
        delegate: &Arc<dyn SshClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Result<PathBuf> {
        use smol::process::{Command, Stdio};
        use std::env::VarError;

        async fn run_cmd(command: &mut Command) -> Result<()> {
            let output = command
                .kill_on_drop(true)
                .stderr(Stdio::inherit())
                .output()
                .await?;
            anyhow::ensure!(
                output.status.success(),
                "Failed to run command: {command:?}"
            );
            Ok(())
        }

        let use_musl = !build_remote_server.contains("nomusl");
        let triple = format!(
            "{}-{}",
            self.ssh_platform.arch,
            match self.ssh_platform.os {
                "linux" =>
                    if use_musl {
                        "unknown-linux-musl"
                    } else {
                        "unknown-linux-gnu"
                    },
                "macos" => "apple-darwin",
                _ => anyhow::bail!("can't cross compile for: {:?}", self.ssh_platform),
            }
        );
        let mut rust_flags = match std::env::var("RUSTFLAGS") {
            Ok(val) => val,
            Err(VarError::NotPresent) => String::new(),
            Err(e) => {
                log::error!("Failed to get env var `RUSTFLAGS` value: {e}");
                String::new()
            }
        };
        if self.ssh_platform.os == "linux" && use_musl {
            rust_flags.push_str(" -C target-feature=+crt-static");
        }
        if build_remote_server.contains("mold") {
            rust_flags.push_str(" -C link-arg=-fuse-ld=mold");
        }

        if self.ssh_platform.arch == std::env::consts::ARCH
            && self.ssh_platform.os == std::env::consts::OS
        {
            delegate.set_status(Some("Building remote server binary from source"), cx);
            log::info!("building remote server binary from source");
            run_cmd(
                Command::new("cargo")
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
                    .env("RUSTFLAGS", &rust_flags),
            )
            .await?;
        } else if build_remote_server.contains("cross") {
            #[cfg(target_os = "windows")]
            use util::paths::SanitizedPath;

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

            // On Windows, the binding needs to be set to the canonical path
            #[cfg(target_os = "windows")]
            let src =
                SanitizedPath::from(smol::fs::canonicalize("./target").await?).to_glob_string();
            #[cfg(not(target_os = "windows"))]
            let src = "./target";
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
                        format!("--mount type=bind,src={src},dst=/app/target"),
                    )
                    .env("RUSTFLAGS", &rust_flags),
            )
            .await?;
        } else {
            let which = cx
                .background_spawn(async move { which::which("zig") })
                .await;

            if which.is_err() {
                #[cfg(not(target_os = "windows"))]
                {
                    anyhow::bail!(
                        "zig not found on $PATH, install zig (see https://ziglang.org/learn/getting-started or use zigup) or pass ZED_BUILD_REMOTE_SERVER=cross to use cross"
                    )
                }
                #[cfg(target_os = "windows")]
                {
                    anyhow::bail!(
                        "zig not found on $PATH, install zig (use `winget install -e --id zig.zig` or see https://ziglang.org/learn/getting-started or use zigup) or pass ZED_BUILD_REMOTE_SERVER=cross to use cross"
                    )
                }
            }

            delegate.set_status(Some("Adding rustup target for cross-compilation"), cx);
            log::info!("adding rustup target");
            run_cmd(Command::new("rustup").args(["target", "add"]).arg(&triple)).await?;

            delegate.set_status(Some("Installing cargo-zigbuild for cross-compilation"), cx);
            log::info!("installing cargo-zigbuild");
            run_cmd(Command::new("cargo").args(["install", "--locked", "cargo-zigbuild"])).await?;

            delegate.set_status(
                Some(&format!(
                    "Building remote binary from source for {triple} with Zig"
                )),
                cx,
            );
            log::info!("building remote binary from source for {triple} with Zig");
            run_cmd(
                Command::new("cargo")
                    .args([
                        "zigbuild",
                        "--package",
                        "remote_server",
                        "--features",
                        "debug-embed",
                        "--target-dir",
                        "target/remote_server",
                        "--target",
                        &triple,
                    ])
                    .env("RUSTFLAGS", &rust_flags),
            )
            .await?;
        };
        let bin_path = Path::new("target")
            .join("remote_server")
            .join(&triple)
            .join("debug")
            .join("remote_server");

        let path = if !build_remote_server.contains("nocompress") {
            delegate.set_status(Some("Compressing binary"), cx);

            #[cfg(not(target_os = "windows"))]
            {
                run_cmd(Command::new("gzip").args(["-f", &bin_path.to_string_lossy()])).await?;
            }
            #[cfg(target_os = "windows")]
            {
                // On Windows, we use 7z to compress the binary
                let seven_zip = which::which("7z.exe").context("7z.exe not found on $PATH, install it (e.g. with `winget install -e --id 7zip.7zip`) or, if you don't want this behaviour, set $env:ZED_BUILD_REMOTE_SERVER=\"nocompress\"")?;
                let gz_path = format!("target/remote_server/{}/debug/remote_server.gz", triple);
                if smol::fs::metadata(&gz_path).await.is_ok() {
                    smol::fs::remove_file(&gz_path).await?;
                }
                run_cmd(Command::new(seven_zip).args([
                    "a",
                    "-tgzip",
                    &gz_path,
                    &bin_path.to_string_lossy(),
                ]))
                .await?;
            }

            let mut archive_path = bin_path;
            archive_path.set_extension("gz");
            std::env::current_dir()?.join(archive_path)
        } else {
            bin_path
        };

        Ok(path)
    }
}

type ResponseChannels = Mutex<HashMap<MessageId, oneshot::Sender<(Envelope, oneshot::Sender<()>)>>>;

struct ChannelClient {
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
    fn new(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &App,
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
        cx: &AsyncApp,
    ) -> Task<Result<()>> {
        cx.spawn(async move |cx| {
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
                    let message_id = envelope.message_id();
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

    fn reconnect(
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
        log::debug!("ssh request start. name:{}", T::NAME);
        let response =
            self.request_dynamic(payload.into_envelope(0, None, None), T::NAME, use_buffer);
        async move {
            let response = response.await?;
            log::debug!("ssh request finish. name:{}", T::NAME);
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
                smol::Timer::after(timeout).await;
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
                smol::Timer::after(timeout).await;
                anyhow::bail!("Timed out pinging remote client")
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
}

#[cfg(any(test, feature = "test-support"))]
mod fake {
    use std::{path::PathBuf, sync::Arc};

    use anyhow::Result;
    use async_trait::async_trait;
    use futures::{
        FutureExt, SinkExt, StreamExt,
        channel::{
            mpsc::{self, Sender},
            oneshot,
        },
        select_biased,
    };
    use gpui::{App, AppContext as _, AsyncApp, SemanticVersion, Task, TestAppContext};
    use release_channel::ReleaseChannel;
    use rpc::proto::Envelope;
    use util::paths::{PathStyle, RemotePathBuf};

    use super::{
        ChannelClient, RemoteConnection, SshArgs, SshClientDelegate, SshConnectionOptions,
        SshPlatform,
    };

    pub(super) struct FakeRemoteConnection {
        pub(super) connection_options: SshConnectionOptions,
        pub(super) server_channel: Arc<ChannelClient>,
        pub(super) server_cx: SendableCx,
    }

    pub(super) struct SendableCx(AsyncApp);
    impl SendableCx {
        // SAFETY: When run in test mode, GPUI is always single threaded.
        pub(super) fn new(cx: &TestAppContext) -> Self {
            Self(cx.to_async())
        }

        // SAFETY: Enforce that we're on the main thread by requiring a valid AsyncApp
        fn get(&self, _: &AsyncApp) -> AsyncApp {
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

        fn ssh_args(&self) -> SshArgs {
            SshArgs {
                arguments: Vec::new(),
                envs: None,
            }
        }

        fn upload_directory(
            &self,
            _src_path: PathBuf,
            _dest_path: RemotePathBuf,
            _cx: &App,
        ) -> Task<Result<()>> {
            unreachable!()
        }

        fn connection_options(&self) -> SshConnectionOptions {
            self.connection_options.clone()
        }

        fn simulate_disconnect(&self, cx: &AsyncApp) {
            let (outgoing_tx, _) = mpsc::unbounded::<Envelope>();
            let (_, incoming_rx) = mpsc::unbounded::<Envelope>();
            self.server_channel
                .reconnect(incoming_rx, outgoing_tx, &self.server_cx.get(cx));
        }

        fn start_proxy(
            &self,
            _unique_identifier: String,
            _reconnect: bool,
            mut client_incoming_tx: mpsc::UnboundedSender<Envelope>,
            mut client_outgoing_rx: mpsc::UnboundedReceiver<Envelope>,
            mut connection_activity_tx: Sender<()>,
            _delegate: Arc<dyn SshClientDelegate>,
            cx: &mut AsyncApp,
        ) -> Task<Result<i32>> {
            let (mut server_incoming_tx, server_incoming_rx) = mpsc::unbounded::<Envelope>();
            let (server_outgoing_tx, mut server_outgoing_rx) = mpsc::unbounded::<Envelope>();

            self.server_channel.reconnect(
                server_incoming_rx,
                server_outgoing_tx,
                &self.server_cx.get(cx),
            );

            cx.background_spawn(async move {
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

        fn path_style(&self) -> PathStyle {
            PathStyle::current()
        }

        fn shell(&self) -> String {
            "sh".to_owned()
        }
    }

    pub(super) struct Delegate;

    impl SshClientDelegate for Delegate {
        fn ask_password(&self, _: String, _: oneshot::Sender<String>, _: &mut AsyncApp) {
            unreachable!()
        }

        fn download_server_binary_locally(
            &self,
            _: SshPlatform,
            _: ReleaseChannel,
            _: Option<SemanticVersion>,
            _: &mut AsyncApp,
        ) -> Task<Result<PathBuf>> {
            unreachable!()
        }

        fn get_download_params(
            &self,
            _platform: SshPlatform,
            _release_channel: ReleaseChannel,
            _version: Option<SemanticVersion>,
            _cx: &mut AsyncApp,
        ) -> Task<Result<Option<(String, String)>>> {
            unreachable!()
        }

        fn set_status(&self, _: Option<&str>, _: &mut AsyncApp) {}
    }
}
