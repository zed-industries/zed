use crate::protocol::{
    message_len_from_buffer, read_message_with_len, write_message, MessageId, MESSAGE_LEN_SIZE,
};
use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, LocalBoxFuture},
    AsyncBufReadExt as _, AsyncWriteExt as _, Future,
};
use futures::{select_biased, AsyncReadExt as _, FutureExt as _, StreamExt as _};
use gpui::{AppContext, AsyncAppContext, Model, WeakModel};
use parking_lot::Mutex;
use rpc::{
    proto::{
        self, build_typed_envelope, AnyTypedEnvelope, Envelope, EnvelopedMessage, PeerId,
        ProtoClient, RequestMessage,
    },
    TypedEnvelope,
};
use smol::{
    fs::{
        self,
        unix::{MetadataExt, PermissionsExt as _},
    },
    io::BufReader,
    process,
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

const SERVER_BINARY_LOCAL_PATH: &str = "target/debug/remote_server";
const SERVER_BINARY_REMOTE_PATH: &str = "./.zed_remote_server";

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

#[derive(Debug)]
struct SshFileStat {
    size: u64,
    mode: u32,
}

struct SpawnRequest {
    command: String,
    process_tx: oneshot::Sender<process::Child>,
}

type ResponseChannels = Mutex<HashMap<MessageId, oneshot::Sender<(Envelope, oneshot::Sender<()>)>>>;

impl SshSession {
    pub async fn client(
        user: String,
        host: String,
        port: u16,
        password_callback: Box<dyn Send + FnOnce(&mut AsyncAppContext) -> String>,
        cx: &AsyncAppContext,
    ) -> Result<Arc<Self>> {
        let client_state = SshClientState::new(user, host, port, password_callback, cx).await?;

        let (spawn_process_tx, mut spawn_process_rx) = mpsc::unbounded::<SpawnRequest>();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded::<Envelope>();
        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();

        ensure_server_binary(&client_state).await?;

        let mut remote_server_child = client_state
            .ssh_command(SERVER_BINARY_REMOTE_PATH)
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
    async fn new(
        user: String,
        host: String,
        port: u16,
        password_callback: Box<dyn Send + FnOnce(&mut AsyncAppContext) -> String>,
        cx: &AsyncAppContext,
    ) -> Result<Self> {
        let url = format!("{user}@{host}");
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-ssh-session")
            .tempdir()?;

        let listener = smol::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to find open port");
        let askpass_port = listener.local_addr().unwrap().port();
        let password_task = cx.spawn(|mut cx| async move {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut buffer = [0; 1024];
                if stream.read(&mut buffer).await.is_ok() {
                    let password = password_callback(&mut cx);
                    let _ = stream.write_all(password.as_bytes()).await;
                    return;
                }
            }
        });

        // TODO remove
        password_task.detach();

        let askpass_script =
            format!("#!/bin/sh\nnc 127.0.0.1 {askpass_port} < /dev/null 2> /dev/null");
        let askpass_script_path = temp_dir.path().join("askpass.sh");
        fs::write(&askpass_script_path, askpass_script).await?;
        fs::set_permissions(&askpass_script_path, std::fs::Permissions::from_mode(0o700)).await?;

        let socket_path = temp_dir.path().join("control.sock");
        let mut master_process = process::Command::new("ssh")
            .env("SSH_ASKPASS_REQUIRE", "force")
            .env("SSH_ASKPASS", &askpass_script_path)
            .args(["-o", "ControlMaster=yes", "-o"])
            .arg(format!("ControlPath={}", socket_path.display()))
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .args(["-p", &port.to_string()])
            .arg(&url)
            .spawn()?;

        let stdin = master_process.stdin.as_mut().unwrap();
        let mut stdout = BufReader::new(master_process.stdout.as_mut().unwrap());
        stdin.write_all(b"echo hello zed\n").await?;
        stdin.flush().await?;
        let mut line = String::new();
        stdout.read_line(&mut line).await?;

        Ok(Self {
            _master_process: master_process,
            port: askpass_port,
            _temp_dir: temp_dir,
            socket_path,
            url,
        })
    }

    async fn file_stat(&self, path: &Path) -> Result<Option<SshFileStat>> {
        let output = self
            .ssh_command("stat")
            .args(["-f", "%z %p"])
            .arg(path)
            .output()
            .await?;

        if output.status.success() {
            let output = String::from_utf8(output.stdout)?;
            let mut stats = output.split_whitespace();
            let size = stats
                .next()
                .ok_or_else(|| anyhow!("Failed to parse size"))?
                .parse()?;
            let mode = stats
                .next()
                .ok_or_else(|| anyhow!("Failed to parse mode"))?
                .parse::<u32>()?;
            Ok(Some(SshFileStat { size, mode }))
        } else {
            Ok(None)
        }
    }

    async fn chmod(&self, path: &Path, mode: u32) -> Result<()> {
        let output = self
            .ssh_command("chmod")
            .arg(format!("{:o}", mode))
            .arg(path)
            .output()
            .await?;
        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "failed to chmod file file {} {:o}: {}",
                path.display(),
                mode,
                String::from_utf8_lossy(&output.stderr)
            ))
        }
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

    fn ssh_command<S: AsRef<OsStr>>(&self, binary: S) -> process::Command {
        let mut command = process::Command::new("ssh");
        self.ssh_options(&mut command)
            .arg("-p")
            .arg(&self.port.to_string())
            .arg(&self.url)
            .arg(binary);
        command
    }

    fn ssh_options<'a>(&self, command: &'a mut process::Command) -> &'a mut process::Command {
        command
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .args(["-o", "ControlMaster=no", "-o"])
            .arg(format!("ControlPath={}", self.socket_path.display()))
    }
}

async fn ensure_server_binary(session: &SshClientState) -> Result<()> {
    let src_path = Path::new(SERVER_BINARY_LOCAL_PATH);
    let dst_path = Path::new(SERVER_BINARY_REMOTE_PATH);

    let src_stat = fs::metadata(src_path).await?;
    let size = src_stat.size();
    let perm = 0o755_u32;
    let dst_stat = session.file_stat(&dst_path).await?;
    let server_binary_exists = dst_stat.map_or(false, |stats| {
        stats.size == src_stat.size() && stats.mode == perm
    });
    if server_binary_exists {
        log::info!("remote development server already present",);
        return Ok(());
    }

    let t0 = Instant::now();
    log::info!("uploading remote development server ({size} bytes)",);
    session
        .upload_file(src_path, dst_path)
        .await
        .context("failed to upload server binary")?;

    session.chmod(dst_path, 0o755).await?;
    log::info!("uploaded remote development server in {:?}", t0.elapsed());

    Ok(())
}
