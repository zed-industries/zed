use crate::protocol::{
    message_len_from_buffer, read_message_with_len, write_message, MessageId, MESSAGE_LEN_SIZE,
};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, LocalBoxFuture},
    select_biased, AsyncReadExt as _, AsyncWriteExt as _, Future, FutureExt as _, StreamExt as _,
};
use gpui::{AppContext, AsyncAppContext, Model, SemanticVersion, WeakModel};
use parking_lot::Mutex;
use rpc::{
    proto::{
        self, build_typed_envelope, AnyTypedEnvelope, Envelope, EnvelopedMessage, PeerId,
        ProtoClient, RequestMessage,
    },
    TypedEnvelope,
};
use smol::{
    fs,
    process::{self, Stdio},
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

pub struct SshSession {
    next_message_id: AtomicU32,
    response_channels: ResponseChannels,
    outgoing_tx: mpsc::UnboundedSender<Envelope>,
    spawn_process_tx: mpsc::UnboundedSender<SpawnRequest>,
    message_handlers: Mutex<
        HashMap<
            TypeId,
            Arc<
                dyn Send
                    + Sync
                    + Fn(
                        Box<dyn AnyTypedEnvelope>,
                        Arc<SshSession>,
                        AsyncAppContext,
                    ) -> Option<LocalBoxFuture<'static, Result<()>>>,
            >,
        >,
    >,
}

struct SshClientState {
    socket_path: PathBuf,
    port: u16,
    url: String,
    _master_process: process::Child,
    _temp_dir: TempDir,
}

struct SpawnRequest {
    command: String,
    process_tx: oneshot::Sender<process::Child>,
}

#[derive(Copy, Clone, Debug)]
pub struct SshPlatform {
    pub os: &'static str,
    pub arch: &'static str,
}

pub trait SshClientDelegate {
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
}

type ResponseChannels = Mutex<HashMap<MessageId, oneshot::Sender<(Envelope, oneshot::Sender<()>)>>>;

impl SshSession {
    pub async fn client(
        user: String,
        host: String,
        port: u16,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<Self>> {
        let client_state = SshClientState::new(user, host, port, delegate.clone(), cx).await?;

        let platform = query_platform(&client_state).await?;
        let (local_binary_path, version) = delegate.get_server_binary(platform, cx).await??;
        let remote_binary_path = delegate.remote_server_binary_path(cx)?;
        ensure_server_binary(
            &client_state,
            &delegate,
            &local_binary_path,
            &remote_binary_path,
            version,
            cx,
        )
        .await?;

        let (spawn_process_tx, mut spawn_process_rx) = mpsc::unbounded::<SpawnRequest>();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded::<Envelope>();
        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();

        let mut remote_server_child = client_state
            .ssh_command(&remote_binary_path)
            .arg("run")
            .spawn()
            .context("failed to spawn remote server")?;
        let mut child_stderr = remote_server_child.stderr.take().unwrap();
        let mut child_stdout = remote_server_child.stdout.take().unwrap();
        let mut child_stdin = remote_server_child.stdin.take().unwrap();

        let executor = cx.background_executor().clone();
        executor.clone().spawn(async move {
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

                    request = spawn_process_rx.next().fuse() => {
                        let Some(request) = request else {
                            return Ok(());
                        };

                        log::info!("spawn process: {:?}", request.command);
                        let child = client_state
                            .ssh_command(&request.command)
                            .spawn()
                            .context("failed to create channel")?;
                        request.process_tx.send(child).ok();
                    }

                    result = child_stdout.read(&mut stdout_buffer).fuse() => {
                        match result {
                            Ok(len) => {
                                if len == 0 {
                                    child_stdin.close().await?;
                                    let status = remote_server_child.status().await?;
                                    if !status.success() {
                                        log::info!("channel exited with status: {status:?}");
                                    }
                                    return Ok(());
                                }

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
                                    let content = String::from_utf8_lossy(&stderr_buffer[start_ix..line_ix]);
                                    start_ix = line_ix + 1;
                                    eprintln!("(remote) {}", content);
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
        }).detach();

        cx.update(|cx| Self::new(incoming_rx, outgoing_tx, spawn_process_tx, cx))
    }

    pub fn server(
        incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        cx: &AppContext,
    ) -> Arc<SshSession> {
        let (tx, _rx) = mpsc::unbounded();
        Self::new(incoming_rx, outgoing_tx, tx, cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(
        client_cx: &mut gpui::TestAppContext,
        server_cx: &mut gpui::TestAppContext,
    ) -> (Arc<Self>, Arc<Self>) {
        let (server_to_client_tx, server_to_client_rx) = mpsc::unbounded();
        let (client_to_server_tx, client_to_server_rx) = mpsc::unbounded();
        let (tx, _rx) = mpsc::unbounded();
        (
            client_cx
                .update(|cx| Self::new(server_to_client_rx, client_to_server_tx, tx.clone(), cx)),
            server_cx
                .update(|cx| Self::new(client_to_server_rx, server_to_client_tx, tx.clone(), cx)),
        )
    }

    fn new(
        mut incoming_rx: mpsc::UnboundedReceiver<Envelope>,
        outgoing_tx: mpsc::UnboundedSender<Envelope>,
        spawn_process_tx: mpsc::UnboundedSender<SpawnRequest>,
        cx: &AppContext,
    ) -> Arc<SshSession> {
        let this = Arc::new(Self {
            next_message_id: AtomicU32::new(0),
            response_channels: ResponseChannels::default(),
            outgoing_tx,
            spawn_process_tx,
            message_handlers: Default::default(),
        });

        cx.spawn(|cx| {
            let this = this.clone();
            async move {
                let peer_id = PeerId { owner_id: 0, id: 0 };
                while let Some(incoming) = incoming_rx.next().await {
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
                        log::debug!(
                            "ssh message received. name:{}",
                            envelope.payload_type_name()
                        );
                        let type_id = envelope.payload_type_id();
                        let handler = this.message_handlers.lock().get(&type_id).cloned();
                        if let Some(handler) = handler {
                            if let Some(future) = handler(envelope, this.clone(), cx.clone()) {
                                future.await.ok();
                            } else {
                                this.message_handlers.lock().remove(&type_id);
                            }
                        }
                    }
                }
                anyhow::Ok(())
            }
        })
        .detach();

        this
    }

    pub fn request<T: RequestMessage>(
        &self,
        payload: T,
    ) -> impl 'static + Future<Output = Result<T::Response>> {
        log::debug!("ssh request start. name:{}", T::NAME);
        let response = self.request_dynamic(payload.into_envelope(0, None, None), "");
        async move {
            let response = response.await?;
            log::debug!("ssh request finish. name:{}", T::NAME);
            T::Response::from_envelope(response)
                .ok_or_else(|| anyhow!("received a response of the wrong type"))
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, payload: T) -> Result<()> {
        self.send_dynamic(payload.into_envelope(0, None, None))
    }

    pub fn request_dynamic(
        &self,
        mut envelope: proto::Envelope,
        _request_type: &'static str,
    ) -> impl 'static + Future<Output = Result<proto::Envelope>> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        let (tx, rx) = oneshot::channel();
        self.response_channels
            .lock()
            .insert(MessageId(envelope.id), tx);
        self.outgoing_tx.unbounded_send(envelope).ok();
        async move { Ok(rx.await.context("connection lost")?.0) }
    }

    pub fn send_dynamic(&self, mut envelope: proto::Envelope) -> Result<()> {
        envelope.id = self.next_message_id.fetch_add(1, SeqCst);
        self.outgoing_tx.unbounded_send(envelope)?;
        Ok(())
    }

    pub async fn spawn_process(&self, command: String) -> process::Child {
        let (process_tx, process_rx) = oneshot::channel();
        self.spawn_process_tx
            .unbounded_send(SpawnRequest {
                command,
                process_tx,
            })
            .ok();
        process_rx.await.unwrap()
    }

    pub fn add_message_handler<M, E, H, F>(&self, entity: WeakModel<E>, handler: H)
    where
        M: EnvelopedMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(Model<E>, TypedEnvelope<M>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();
        self.message_handlers.lock().insert(
            message_type_id,
            Arc::new(move |envelope, _, cx| {
                let entity = entity.upgrade()?;
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                Some(handler(entity, *envelope, cx).boxed_local())
            }),
        );
    }

    pub fn add_request_handler<M, E, H, F>(&self, entity: WeakModel<E>, handler: H)
    where
        M: EnvelopedMessage + RequestMessage,
        E: 'static,
        H: 'static + Sync + Send + Fn(Model<E>, TypedEnvelope<M>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        let message_type_id = TypeId::of::<M>();
        self.message_handlers.lock().insert(
            message_type_id,
            Arc::new(move |envelope, this, cx| {
                let entity = entity.upgrade()?;
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                let request_id = envelope.message_id();
                Some(
                    handler(entity, *envelope, cx)
                        .then(move |result| async move {
                            this.outgoing_tx.unbounded_send(result?.into_envelope(
                                this.next_message_id.fetch_add(1, SeqCst),
                                Some(request_id),
                                None,
                            ))?;
                            Ok(())
                        })
                        .boxed_local(),
                )
            }),
        );
    }
}

impl ProtoClient for SshSession {
    fn request(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>> {
        self.request_dynamic(envelope, request_type).boxed()
    }

    fn send(&self, envelope: proto::Envelope) -> Result<()> {
        self.send_dynamic(envelope)
    }
}

impl SshClientState {
    #[cfg(not(unix))]
    async fn new(
        user: String,
        host: String,
        port: u16,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &AsyncAppContext,
    ) -> Result<Self> {
        Err(anyhow!("ssh is not supported on this platform"))
    }

    #[cfg(unix)]
    async fn new(
        user: String,
        host: String,
        port: u16,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &AsyncAppContext,
    ) -> Result<Self> {
        use futures::{io::BufReader, AsyncBufReadExt as _};
        use smol::{fs::unix::PermissionsExt as _, net::unix::UnixListener};
        use util::ResultExt as _;

        let url = format!("{user}@{host}");
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-ssh-session")
            .tempdir()?;

        // Create a domain socket listener to handle requests from the askpass program.
        let askpass_socket = temp_dir.path().join("askpass.sock");
        let listener =
            UnixListener::bind(&askpass_socket).context("failed to create askpass socket")?;

        let askpass_task = cx.spawn(|mut cx| async move {
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
            .args(["-p", &port.to_string()])
            .arg(&url)
            .spawn()?;

        // Wait for this ssh process to close its stdout, indicating that authentication
        // has completed.
        let stdout = master_process.stdout.as_mut().unwrap();
        let mut output = Vec::new();
        stdout.read_to_end(&mut output).await?;
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
            url,
            port,
            socket_path,
            _master_process: master_process,
            _temp_dir: temp_dir,
        })
    }

    async fn upload_file(&self, src_path: &Path, dest_path: &Path) -> Result<()> {
        let mut command = process::Command::new("scp");
        let output = self
            .ssh_options(&mut command)
            .arg("-P")
            .arg(&self.port.to_string())
            .arg(&src_path)
            .arg(&format!("{}:{}", self.url, dest_path.display()))
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

    fn ssh_command<S: AsRef<OsStr>>(&self, program: S) -> process::Command {
        let mut command = process::Command::new("ssh");
        self.ssh_options(&mut command)
            .arg("-p")
            .arg(&self.port.to_string())
            .arg(&self.url)
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

async fn query_platform(session: &SshClientState) -> Result<SshPlatform> {
    let os = run_cmd(session.ssh_command("uname").arg("-s")).await?;
    let arch = run_cmd(session.ssh_command("uname").arg("-m")).await?;

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

async fn ensure_server_binary(
    session: &SshClientState,
    delegate: &Arc<dyn SshClientDelegate>,
    src_path: &Path,
    dst_path: &Path,
    version: SemanticVersion,
    cx: &mut AsyncAppContext,
) -> Result<()> {
    let mut dst_path_gz = dst_path.to_path_buf();
    dst_path_gz.set_extension("gz");

    if let Some(parent) = dst_path.parent() {
        run_cmd(session.ssh_command("mkdir").arg("-p").arg(parent)).await?;
    }

    let mut server_binary_exists = false;
    if let Ok(installed_version) = run_cmd(session.ssh_command(&dst_path).arg("version")).await {
        if installed_version.trim() == version.to_string() {
            server_binary_exists = true;
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
    session
        .upload_file(src_path, &dst_path_gz)
        .await
        .context("failed to upload server binary")?;
    log::info!("uploaded remote development server in {:?}", t0.elapsed());

    delegate.set_status(Some("extracting remote development server"), cx);
    run_cmd(
        session
            .ssh_command("gunzip")
            .arg("--force")
            .arg(&dst_path_gz),
    )
    .await?;

    delegate.set_status(Some("unzipping remote development server"), cx);
    run_cmd(
        session
            .ssh_command("chmod")
            .arg(format!("{:o}", server_mode))
            .arg(&dst_path),
    )
    .await?;

    Ok(())
}
