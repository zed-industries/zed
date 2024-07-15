use crate::protocol::{
    message_len_from_buffer, read_message_with_len, write_message, MessageId, MESSAGE_LEN_SIZE,
};
use anyhow::{anyhow, Context as _, Result};
use async_pipe::{PipeReader, PipeWriter};
use async_ssh2_lite::{AsyncSession, AsyncSessionStream};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, LocalBoxFuture},
    AsyncWriteExt as _, Future,
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
use smol::{fs::unix::MetadataExt, io, Async};
use std::{
    any::TypeId,
    net::{SocketAddr, TcpStream},
    path::Path,
    sync::{
        atomic::{AtomicU32, Ordering::SeqCst},
        Arc,
    },
    time::Instant,
};

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

type ResponseChannels = Mutex<HashMap<MessageId, oneshot::Sender<(Envelope, oneshot::Sender<()>)>>>;

impl SshSession {
    pub async fn client(
        address: SocketAddr,
        user: &str,
        password: &str,
        cx: &AsyncAppContext,
    ) -> Result<Arc<Self>> {
        let (spawn_process_tx, mut spawn_process_rx) = mpsc::unbounded::<SpawnRequest>();
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded::<Envelope>();
        let (incoming_tx, incoming_rx) = mpsc::unbounded::<Envelope>();

        let stream = Async::<TcpStream>::connect(address)
            .await
            .context("failed to connect to remote address")?;

        let mut session =
            AsyncSession::new(stream, None).context("failed to create ssh session")?;
        session.handshake().await.context("ssh handshake failed")?;
        session.userauth_password(user, password).await.unwrap();

        ensure_server_binary(&session).await?;

        let mut channel = session
            .channel_session()
            .await
            .context("failed to create channel")?;
        channel.exec(SERVER_BINARY_REMOTE_PATH).await?;
        let mut stderr = channel.stderr();

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

                        write_message(&mut channel, &mut stdin_buffer, outgoing).await?;
                    }

                    request = spawn_process_rx.next().fuse() => {
                        let Some(request) = request else {
                            return Ok(());
                        };

                        log::info!("spawn process: {:?}", request.command);
                        let mut channel = session
                            .channel_session()
                            .await
                            .context("failed to create channel")?;
                        channel.exec(&request.command).await?;
                        let mut stderr = channel.stderr();

                        let (stdin_tx, mut stdin_rx) = async_pipe::pipe();
                        let (mut stdout_tx, stdout_rx) = async_pipe::pipe();
                        let (mut stderr_tx, stderr_rx) = async_pipe::pipe();
                        let (exit_tx, exit_rx) = oneshot::channel();
                        request.process_tx.send(SshChildProcess {
                            stdin: stdin_tx,
                            stdout: stdout_rx,
                            stderr: stderr_rx,
                            exit: exit_rx,
                        }).ok();

                        executor.spawn(async move {
                            let mut stdin_buffer = [0; 1024];
                            let mut stdout_buffer = [0; 1024];
                            let mut stderr_buffer = [0; 1024];
                            loop {
                                select_biased! {
                                    read = channel.read(&mut stdout_buffer).fuse() => {
                                        match read {
                                            Ok(len) => {
                                                if len == 0 {
                                                    stdout_tx.close().ok();
                                                    break;
                                                }
                                                stdout_tx.write_all(&stdout_buffer[..len]).await?;
                                            }
                                            Err(error) => {
                                                log::error!("error reading stdout: {error:?}");
                                                break
                                            }
                                        }
                                    }
                                    read = stderr.read(&mut stderr_buffer).fuse() => {
                                        match read {
                                            Ok(len) => {
                                                if len == 0 {
                                                    stderr_tx.close().ok();
                                                    break;
                                                }
                                                stderr_tx.write_all(&stderr_buffer[..len]).await?;
                                            }
                                            Err(error) => {
                                                log::error!("error reading stdout: {error:?}");
                                                break
                                            }
                                        }
                                    }
                                    read = stdin_rx.read(&mut stdin_buffer).fuse() => {
                                        match read {
                                            Ok(len) => {
                                                if len == 0 {
                                                    channel.send_eof().await?;
                                                    break;
                                                }
                                                channel.write_all(&stdin_buffer[..len]).await?;
                                            }
                                            Err(error) => {
                                                log::error!("error reading stdout: {error:?}");
                                                break
                                            }
                                        }
                                    }
                                }
                            }

                            io::copy(&mut channel, &mut stdout_tx).await?;
                            io::copy(&mut stderr, &mut stderr_tx).await?;
                            if let Ok(status) = channel.exit_status() {
                                exit_tx.send(status).ok();
                            }
                            anyhow::Ok(())
                        }).detach();
                    }

                    result = channel.read(&mut stdout_buffer).fuse() => {
                        match result {
                            Ok(len) => {
                                if len == 0 {
                                    let status = channel.exit_status()?;
                                    if status != 0 {
                                        let signal = channel.exit_signal().await?;
                                        log::info!("channel exited with status: {status:?}, signal: {:?}", signal.error_message);
                                    }
                                    return Ok(());
                                }

                                if len < stdout_buffer.len() {
                                    channel.read_exact(&mut stdout_buffer[len..]).await?;
                                }

                                let message_len = message_len_from_buffer(&stdout_buffer);
                                match read_message_with_len(&mut channel, &mut stdout_buffer, message_len).await {
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

                    result = stderr.read(&mut stderr_buffer[stderr_offset..]).fuse() => {
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
                        if let Some(sender) = this.response_channels.lock().remove(&request_id) {
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
                        if let Some(handler) = this.message_handlers.lock().get(&type_id) {
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

    pub async fn request<T: RequestMessage>(&self, payload: T) -> Result<T::Response> {
        log::debug!("ssh request start. name:{}", T::NAME);
        let response = self
            .request_dynamic(payload.into_envelope(0, None, None), "")
            .await?;
        log::debug!("ssh request finish. name:{}", T::NAME);
        T::Response::from_envelope(response)
            .ok_or_else(|| anyhow!("received a response of the wrong type"))
    }

    pub fn send<T: EnvelopedMessage>(&self, payload: T) -> Result<()> {
        self.send_dynamic(payload.into_envelope(0, None, None))
    }

    pub fn request_dynamic(
        &self,
        mut envelope: proto::Envelope,
        _request_type: &'static str,
    ) -> impl Future<Output = Result<proto::Envelope>> {
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

    pub async fn spawn_process(&self, command: String) -> SshChildProcess {
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

struct SpawnRequest {
    command: String,
    process_tx: oneshot::Sender<SshChildProcess>,
}

pub struct SshChildProcess {
    pub stdin: PipeWriter,
    pub stdout: PipeReader,
    pub stderr: PipeReader,
    pub exit: oneshot::Receiver<i32>,
}

async fn ensure_server_binary<S: AsyncSessionStream + Send + Sync + 'static>(
    session: &AsyncSession<S>,
) -> Result<()> {
    let src_path = Path::new(SERVER_BINARY_LOCAL_PATH);
    let dst_path = Path::new(SERVER_BINARY_REMOTE_PATH);
    let ftp = session
        .sftp()
        .await
        .context("failed to initialize sftp channel")?;

    let src_stat = smol::fs::metadata(src_path).await?;
    let size = src_stat.size();
    let perm = Some(0o755);
    let dst_stat = ftp.stat(dst_path).await.ok();
    let server_binary_exists = dst_stat.map_or(false, |stats| {
        stats.is_file() && stats.size == Some(src_stat.size()) && stats.perm == perm
    });
    if server_binary_exists {
        log::info!("remote development server already present",);
        return Ok(());
    }

    let t0 = Instant::now();
    log::info!("uploading remote development server ({size} bytes)",);
    let mut src_file = smol::fs::File::open(src_path)
        .await
        .with_context(|| format!("failed to open server binary {src_path:?}"))?;
    let mut dst_file = ftp
        .create(dst_path)
        .await
        .context("failed to create server binary")?;
    let result = io::copy(&mut src_file, &mut dst_file).await;
    let mut stat = ftp.stat(dst_path).await?;
    stat.perm = perm;
    ftp.setstat(dst_path, stat).await?;
    if result.is_err() {
        ftp.unlink(dst_path)
            .await
            .context("failed to remove server binary")?;
    }
    result?;
    log::info!("uploaded remote development server in {:?}", t0.elapsed());

    Ok(())
}
