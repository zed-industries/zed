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
use gpui::{AppContext, AsyncAppContext, Model, SemanticVersion, Task};
use parking_lot::Mutex;
use rpc::{
    proto::{self, build_typed_envelope, Envelope, EnvelopedMessage, PeerId, RequestMessage},
    AnyProtoClient, EntityMessageSubscriber, ProtoClient, ProtoMessageHandlerSet, RpcError,
};
use smol::{
    fs,
    process::{self, Child, Stdio},
};
use std::{
    any::TypeId,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, Ordering::SeqCst},
        Arc,
    },
    time::Instant,
};
use tempfile::TempDir;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct SshProjectId(pub u64);

#[derive(Clone)]
pub struct SshSocket {
    connection_options: SshConnectionOptions,
    socket_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[cfg(unix)]
async fn read_with_timeout(
    stdout: &mut process::ChildStdout,
    timeout: std::time::Duration,
    output: &mut Vec<u8>,
) -> Result<(), std::io::Error> {
    smol::future::or(
        async {
            stdout.read_to_end(output).await?;
            Ok::<_, std::io::Error>(())
        },
        async {
            smol::Timer::after(timeout).await;

            Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Read operation timed out",
            ))
        },
    )
    .await
}

struct ChannelForwarder {
    quit_tx: UnboundedSender<()>,
    forwarding_task: Task<(UnboundedSender<Envelope>, UnboundedReceiver<Envelope>)>,
}

impl ChannelForwarder {
    fn new(
        mut incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        cx: &mut AsyncAppContext,
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

struct SshRemoteClientState {
    ssh_connection: SshRemoteConnection,
    delegate: Arc<dyn SshClientDelegate>,
    forwarder: ChannelForwarder,
    _multiplex_task: Task<Result<()>>,
}

pub struct SshRemoteClient {
    client: Arc<ChannelClient>,
    inner_state: Arc<Mutex<Option<SshRemoteClientState>>>,
}

impl SshRemoteClient {
    pub async fn new(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<Self>> {
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<Envelope>();
        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();

        let client = cx.update(|cx| ChannelClient::new(incoming_rx, outgoing_tx, cx))?;
        let this = Arc::new(Self {
            client,
            inner_state: Arc::new(Mutex::new(None)),
        });

        let inner_state = {
            let (proxy, proxy_incoming_tx, proxy_outgoing_rx) =
                ChannelForwarder::new(incoming_tx, outgoing_rx, cx);

            let (ssh_connection, ssh_process) =
                Self::establish_connection(connection_options.clone(), delegate.clone(), cx)
                    .await?;

            let multiplex_task = Self::multiplex(
                this.clone(),
                ssh_process,
                proxy_incoming_tx,
                proxy_outgoing_rx,
                cx,
            );

            SshRemoteClientState {
                ssh_connection,
                delegate,
                forwarder: proxy,
                _multiplex_task: multiplex_task,
            }
        };

        this.inner_state.lock().replace(inner_state);

        Ok(this)
    }

    fn reconnect(this: Arc<Self>, cx: &mut AsyncAppContext) -> Result<()> {
        let Some(state) = this.inner_state.lock().take() else {
            return Err(anyhow!("reconnect is already in progress"));
        };

        let SshRemoteClientState {
            mut ssh_connection,
            delegate,
            forwarder: proxy,
            _multiplex_task,
        } = state;
        drop(_multiplex_task);

        cx.spawn(|mut cx| async move {
            let (incoming_tx, outgoing_rx) = proxy.into_channels().await;

            ssh_connection.master_process.kill()?;
            ssh_connection
                .master_process
                .status()
                .await
                .context("Failed to kill ssh process")?;

            let connection_options = ssh_connection.socket.connection_options.clone();

            let (ssh_connection, ssh_process) =
                Self::establish_connection(connection_options, delegate.clone(), &mut cx).await?;

            let (proxy, proxy_incoming_tx, proxy_outgoing_rx) =
                ChannelForwarder::new(incoming_tx, outgoing_rx, &mut cx);

            let inner_state = SshRemoteClientState {
                ssh_connection,
                delegate,
                forwarder: proxy,
                _multiplex_task: Self::multiplex(
                    this.clone(),
                    ssh_process,
                    proxy_incoming_tx,
                    proxy_outgoing_rx,
                    &mut cx,
                ),
            };
            this.inner_state.lock().replace(inner_state);

            anyhow::Ok(())
        })
        .detach();

        anyhow::Ok(())
    }

    fn multiplex(
        this: Arc<Self>,
        mut ssh_process: Child,
        incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        cx: &mut AsyncAppContext,
    ) -> Task<Result<()>> {
        let mut child_stderr = ssh_process.stderr.take().unwrap();
        let mut child_stdout = ssh_process.stdout.take().unwrap();
        let mut child_stdin = ssh_process.stdin.take().unwrap();

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
                                let status = ssh_process.status().await?;
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
                Self::reconnect(this, &mut cx).ok();
            }

            Ok(())
        })
    }

    async fn establish_connection(
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

        let ssh_process = socket
            .ssh_command(format!(
                "RUST_LOG={} RUST_BACKTRACE={} {:?} run",
                std::env::var("RUST_LOG").unwrap_or_default(),
                std::env::var("RUST_BACKTRACE").unwrap_or_default(),
                remote_binary_path,
            ))
            .spawn()
            .context("failed to spawn remote server")?;

        Ok((ssh_connection, ssh_process))
    }

    pub fn subscribe_to_entity<E: 'static>(&self, remote_id: u64, entity: &Model<E>) {
        self.client.subscribe_to_entity(remote_id, entity);
    }

    pub fn ssh_args(&self) -> Option<Vec<String>> {
        let state = self.inner_state.lock();
        state
            .as_ref()
            .map(|state| state.ssh_connection.socket.ssh_args())
    }

    pub fn to_proto_client(&self) -> AnyProtoClient {
        self.client.clone().into()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (Arc<Self>, Arc<ChannelClient>) {
        let (server_to_client_tx, server_to_client_rx) = mpsc::unbounded();
        let (client_to_server_tx, client_to_server_rx) = mpsc::unbounded();

        (
            client_cx.update(|cx| {
                let client = ChannelClient::new(server_to_client_rx, client_to_server_tx, cx);
                Arc::new(Self {
                    client,
                    inner_state: Arc::new(Mutex::new(None)),
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
        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;

        let askpass_task = cx.spawn({
            let delegate = delegate.clone();
            |mut cx| async move {
                while let Ok((mut stream, _)) = listener.accept().await {
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
        let connection_timeout = std::time::Duration::from_secs(10);
        let result = read_with_timeout(stdout, connection_timeout, &mut output).await;
        if let Err(e) = result {
            let error_message = if e.kind() == std::io::ErrorKind::TimedOut {
                format!(
                    "Failed to connect to host. Timed out after {:?}.",
                    connection_timeout
                )
            } else {
                format!("Failed to connect to host: {}.", e)
            };

            delegate.set_error(error_message, cx);
            return Err(e.into());
        }

        drop(askpass_task);

        if master_process.try_status()?.is_some() {
            output.clear();
            let mut stderr = master_process.stderr.take().unwrap();
            stderr.read_to_end(&mut output).await?;
            Err(anyhow!(
                "failed to connect: {}",
                String::from_utf8_lossy(&output)
            ))?;
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
