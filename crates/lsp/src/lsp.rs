use anyhow::{anyhow, Context, Result};
use futures::{io::BufWriter, AsyncRead, AsyncWrite};
use gpui::{executor, AsyncAppContext, Task};
use parking_lot::{Mutex, RwLock};
use postage::{barrier, oneshot, prelude::Stream, sink::Sink, watch};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};
use std::{
    collections::HashMap,
    future::Future,
    io::Write,
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use std::{path::Path, process::Stdio};
use util::TryFutureExt;

pub use lsp_types::*;

const JSON_RPC_VERSION: &'static str = "2.0";
const CONTENT_LEN_HEADER: &'static str = "Content-Length: ";

type NotificationHandler = Box<dyn Send + Sync + FnMut(&str)>;
type ResponseHandler = Box<dyn Send + FnOnce(Result<&str, Error>)>;

pub struct LanguageServer {
    next_id: AtomicUsize,
    outbound_tx: RwLock<Option<channel::Sender<Vec<u8>>>>,
    capabilities: watch::Receiver<Option<ServerCapabilities>>,
    notification_handlers: Arc<RwLock<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<HashMap<usize, ResponseHandler>>>,
    executor: Arc<executor::Background>,
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    initialized: barrier::Receiver,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
}

pub struct Subscription {
    method: &'static str,
    notification_handlers: Arc<RwLock<HashMap<&'static str, NotificationHandler>>>,
}

#[derive(Serialize, Deserialize)]
struct Request<'a, T> {
    jsonrpc: &'a str,
    id: usize,
    method: &'a str,
    params: T,
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Deserialize)]
struct AnyRequest<'a> {
    id: usize,
    #[serde(borrow)]
    jsonrpc: &'a str,
    #[serde(borrow)]
    method: &'a str,
    #[serde(borrow)]
    params: &'a RawValue,
}

#[derive(Serialize, Deserialize)]
struct AnyResponse<'a> {
    id: usize,
    #[serde(default)]
    error: Option<Error>,
    #[serde(borrow)]
    result: Option<&'a RawValue>,
}

#[derive(Serialize, Deserialize)]
struct Notification<'a, T> {
    #[serde(borrow)]
    jsonrpc: &'a str,
    #[serde(borrow)]
    method: &'a str,
    params: T,
}

#[derive(Deserialize)]
struct AnyNotification<'a> {
    #[serde(borrow)]
    method: &'a str,
    #[serde(borrow)]
    params: &'a RawValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
}

impl LanguageServer {
    pub fn new(
        binary_path: &Path,
        root_path: &Path,
        background: Arc<executor::Background>,
    ) -> Result<Arc<Self>> {
        let mut server = Command::new(binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        Self::new_internal(stdin, stdout, root_path, background)
    }

    fn new_internal<Stdin, Stdout>(
        stdin: Stdin,
        stdout: Stdout,
        root_path: &Path,
        executor: Arc<executor::Background>,
    ) -> Result<Arc<Self>>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut stdin = BufWriter::new(stdin);
        let mut stdout = BufReader::new(stdout);
        let (outbound_tx, outbound_rx) = channel::unbounded::<Vec<u8>>();
        let notification_handlers = Arc::new(RwLock::new(HashMap::<_, NotificationHandler>::new()));
        let response_handlers = Arc::new(Mutex::new(HashMap::<_, ResponseHandler>::new()));
        let input_task = executor.spawn(
            {
                let notification_handlers = notification_handlers.clone();
                let response_handlers = response_handlers.clone();
                async move {
                    let mut buffer = Vec::new();
                    loop {
                        buffer.clear();
                        stdout.read_until(b'\n', &mut buffer).await?;
                        stdout.read_until(b'\n', &mut buffer).await?;
                        let message_len: usize = std::str::from_utf8(&buffer)?
                            .strip_prefix(CONTENT_LEN_HEADER)
                            .ok_or_else(|| anyhow!("invalid header"))?
                            .trim_end()
                            .parse()?;

                        buffer.resize(message_len, 0);
                        stdout.read_exact(&mut buffer).await?;

                        if let Ok(AnyNotification { method, params }) =
                            serde_json::from_slice(&buffer)
                        {
                            if let Some(handler) = notification_handlers.write().get_mut(method) {
                                handler(params.get());
                            } else {
                                log::info!(
                                    "unhandled notification {}:\n{}",
                                    method,
                                    serde_json::to_string_pretty(
                                        &Value::from_str(params.get()).unwrap()
                                    )
                                    .unwrap()
                                );
                            }
                        } else if let Ok(AnyResponse { id, error, result }) =
                            serde_json::from_slice(&buffer)
                        {
                            if let Some(handler) = response_handlers.lock().remove(&id) {
                                if let Some(error) = error {
                                    handler(Err(error));
                                } else if let Some(result) = result {
                                    handler(Ok(result.get()));
                                } else {
                                    handler(Ok("null"));
                                }
                            }
                        } else {
                            return Err(anyhow!(
                                "failed to deserialize message:\n{}",
                                std::str::from_utf8(&buffer)?
                            ));
                        }
                    }
                }
            }
            .log_err(),
        );
        let (output_done_tx, output_done_rx) = barrier::channel();
        let output_task = executor.spawn(
            async move {
                let mut content_len_buffer = Vec::new();
                while let Ok(message) = outbound_rx.recv().await {
                    content_len_buffer.clear();
                    write!(content_len_buffer, "{}", message.len()).unwrap();
                    stdin.write_all(CONTENT_LEN_HEADER.as_bytes()).await?;
                    stdin.write_all(&content_len_buffer).await?;
                    stdin.write_all("\r\n\r\n".as_bytes()).await?;
                    stdin.write_all(&message).await?;
                    stdin.flush().await?;
                }
                drop(output_done_tx);
                Ok(())
            }
            .log_err(),
        );

        let (initialized_tx, initialized_rx) = barrier::channel();
        let (mut capabilities_tx, capabilities_rx) = watch::channel();
        let this = Arc::new(Self {
            notification_handlers,
            response_handlers,
            capabilities: capabilities_rx,
            next_id: Default::default(),
            outbound_tx: RwLock::new(Some(outbound_tx)),
            executor: executor.clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            initialized: initialized_rx,
            output_done_rx: Mutex::new(Some(output_done_rx)),
        });

        let root_uri = Url::from_file_path(root_path).map_err(|_| anyhow!("invalid root path"))?;
        executor
            .spawn({
                let this = this.clone();
                async move {
                    if let Some(capabilities) = this.init(root_uri).log_err().await {
                        *capabilities_tx.borrow_mut() = Some(capabilities);
                    }

                    drop(initialized_tx);
                }
            })
            .detach();

        Ok(this)
    }

    async fn init(self: Arc<Self>, root_uri: Url) -> Result<ServerCapabilities> {
        #[allow(deprecated)]
        let params = InitializeParams {
            process_id: Default::default(),
            root_path: Default::default(),
            root_uri: Some(root_uri),
            initialization_options: Default::default(),
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    definition: Some(GotoCapability {
                        link_support: Some(true),
                        ..Default::default()
                    }),
                    code_action: Some(CodeActionClientCapabilities {
                        code_action_literal_support: Some(CodeActionLiteralSupport {
                            code_action_kind: CodeActionKindLiteralSupport {
                                value_set: vec![
                                    CodeActionKind::REFACTOR.as_str().into(),
                                    CodeActionKind::QUICKFIX.as_str().into(),
                                ],
                            },
                        }),
                        data_support: Some(true),
                        resolve_support: Some(CodeActionCapabilityResolveSupport {
                            properties: vec!["edit".to_string()],
                        }),
                        ..Default::default()
                    }),
                    completion: Some(CompletionClientCapabilities {
                        completion_item: Some(CompletionItemCapability {
                            snippet_support: Some(true),
                            resolve_support: Some(CompletionItemCapabilityResolveSupport {
                                properties: vec!["additionalTextEdits".to_string()],
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                experimental: Some(json!({
                    "serverStatusNotification": true,
                })),
                window: Some(WindowClientCapabilities {
                    work_done_progress: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            trace: Default::default(),
            workspace_folders: Default::default(),
            client_info: Default::default(),
            locale: Default::default(),
        };

        let this = self.clone();
        let request = Self::request_internal::<request::Initialize>(
            &this.next_id,
            &this.response_handlers,
            this.outbound_tx.read().as_ref(),
            params,
        );
        let response = request.await?;
        Self::notify_internal::<notification::Initialized>(
            this.outbound_tx.read().as_ref(),
            InitializedParams {},
        )?;
        Ok(response.capabilities)
    }

    pub fn shutdown(&self) -> Option<impl 'static + Send + Future<Output = Result<()>>> {
        if let Some(tasks) = self.io_tasks.lock().take() {
            let response_handlers = self.response_handlers.clone();
            let outbound_tx = self.outbound_tx.write().take();
            let next_id = AtomicUsize::new(self.next_id.load(SeqCst));
            let mut output_done = self.output_done_rx.lock().take().unwrap();
            Some(async move {
                Self::request_internal::<request::Shutdown>(
                    &next_id,
                    &response_handlers,
                    outbound_tx.as_ref(),
                    (),
                )
                .await?;
                Self::notify_internal::<notification::Exit>(outbound_tx.as_ref(), ())?;
                drop(outbound_tx);
                output_done.recv().await;
                drop(tasks);
                Ok(())
            })
        } else {
            None
        }
    }

    pub fn on_notification<T, F>(&self, mut f: F) -> Subscription
    where
        T: notification::Notification,
        F: 'static + Send + Sync + FnMut(T::Params),
    {
        let prev_handler = self.notification_handlers.write().insert(
            T::METHOD,
            Box::new(
                move |notification| match serde_json::from_str(notification) {
                    Ok(notification) => f(notification),
                    Err(err) => log::error!("error parsing notification {}: {}", T::METHOD, err),
                },
            ),
        );

        assert!(
            prev_handler.is_none(),
            "registered multiple handlers for the same notification"
        );

        Subscription {
            method: T::METHOD,
            notification_handlers: self.notification_handlers.clone(),
        }
    }

    pub fn capabilities(&self) -> watch::Receiver<Option<ServerCapabilities>> {
        self.capabilities.clone()
    }

    pub fn request<T: request::Request>(
        self: &Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<T::Result>>
    where
        T::Result: 'static + Send,
    {
        let this = self.clone();
        async move {
            this.initialized.clone().recv().await;
            Self::request_internal::<T>(
                &this.next_id,
                &this.response_handlers,
                this.outbound_tx.read().as_ref(),
                params,
            )
            .await
        }
    }

    fn request_internal<T: request::Request>(
        next_id: &AtomicUsize,
        response_handlers: &Mutex<HashMap<usize, ResponseHandler>>,
        outbound_tx: Option<&channel::Sender<Vec<u8>>>,
        params: T::Params,
    ) -> impl 'static + Future<Output = Result<T::Result>>
    where
        T::Result: 'static + Send,
    {
        let id = next_id.fetch_add(1, SeqCst);
        let message = serde_json::to_vec(&Request {
            jsonrpc: JSON_RPC_VERSION,
            id,
            method: T::METHOD,
            params,
        })
        .unwrap();
        let mut response_handlers = response_handlers.lock();
        let (mut tx, mut rx) = oneshot::channel();
        response_handlers.insert(
            id,
            Box::new(move |result| {
                let response = match result {
                    Ok(response) => {
                        serde_json::from_str(response).context("failed to deserialize response")
                    }
                    Err(error) => Err(anyhow!("{}", error.message)),
                };
                let _ = tx.try_send(response);
            }),
        );

        let send = outbound_tx
            .as_ref()
            .ok_or_else(|| {
                anyhow!("tried to send a request to a language server that has been shut down")
            })
            .and_then(|outbound_tx| {
                outbound_tx
                    .try_send(message)
                    .context("failed to write to language server's stdin")?;
                Ok(())
            });
        async move {
            send?;
            rx.recv().await.unwrap()
        }
    }

    pub fn notify<T: notification::Notification>(
        self: &Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            this.initialized.clone().recv().await;
            Self::notify_internal::<T>(this.outbound_tx.read().as_ref(), params)?;
            Ok(())
        }
    }

    fn notify_internal<T: notification::Notification>(
        outbound_tx: Option<&channel::Sender<Vec<u8>>>,
        params: T::Params,
    ) -> Result<()> {
        let message = serde_json::to_vec(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: T::METHOD,
            params,
        })
        .unwrap();
        let outbound_tx = outbound_tx
            .as_ref()
            .ok_or_else(|| anyhow!("tried to notify a language server that has been shut down"))?;
        outbound_tx.try_send(message)?;
        Ok(())
    }
}

impl Drop for LanguageServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown() {
            self.executor.spawn(shutdown).detach();
        }
    }
}

impl Subscription {
    pub fn detach(mut self) {
        self.method = "";
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.notification_handlers.write().remove(self.method);
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeLanguageServer {
    handlers: Arc<
        Mutex<
            HashMap<&'static str, Box<dyn Send + FnMut(usize, &[u8], AsyncAppContext) -> Vec<u8>>>,
        >,
    >,
    outgoing_tx: futures::channel::mpsc::UnboundedSender<Vec<u8>>,
    incoming_rx: futures::channel::mpsc::UnboundedReceiver<Vec<u8>>,
}

#[cfg(any(test, feature = "test-support"))]
impl LanguageServer {
    pub fn fake(cx: &mut gpui::MutableAppContext) -> (Arc<Self>, FakeLanguageServer) {
        Self::fake_with_capabilities(Default::default(), cx)
    }

    pub fn fake_with_capabilities(
        capabilities: ServerCapabilities,
        cx: &mut gpui::MutableAppContext,
    ) -> (Arc<Self>, FakeLanguageServer) {
        let (stdin_writer, stdin_reader) = async_pipe::pipe();
        let (stdout_writer, stdout_reader) = async_pipe::pipe();

        let mut fake = FakeLanguageServer::new(stdin_reader, stdout_writer, cx);
        fake.handle_request::<request::Initialize, _>({
            let capabilities = capabilities.clone();
            move |_, _| InitializeResult {
                capabilities: capabilities.clone(),
                ..Default::default()
            }
        });

        let server = Self::new_internal(
            stdin_writer,
            stdout_reader,
            Path::new("/"),
            cx.background().clone(),
        )
        .unwrap();

        (server, fake)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    fn new(
        stdin: async_pipe::PipeReader,
        stdout: async_pipe::PipeWriter,
        cx: &mut gpui::MutableAppContext,
    ) -> Self {
        use futures::StreamExt as _;

        let (incoming_tx, incoming_rx) = futures::channel::mpsc::unbounded();
        let (outgoing_tx, mut outgoing_rx) = futures::channel::mpsc::unbounded();
        let this = Self {
            outgoing_tx: outgoing_tx.clone(),
            incoming_rx,
            handlers: Default::default(),
        };

        // Receive incoming messages
        let handlers = this.handlers.clone();
        cx.spawn(|cx| async move {
            let mut buffer = Vec::new();
            let mut stdin = smol::io::BufReader::new(stdin);
            while Self::receive(&mut stdin, &mut buffer).await.is_ok() {
                cx.background().simulate_random_delay().await;
                if let Ok(request) = serde_json::from_slice::<AnyRequest>(&buffer) {
                    assert_eq!(request.jsonrpc, JSON_RPC_VERSION);

                    if let Some(handler) = handlers.lock().get_mut(request.method) {
                        let response =
                            handler(request.id, request.params.get().as_bytes(), cx.clone());
                        log::debug!("handled lsp request. method:{}", request.method);
                        outgoing_tx.unbounded_send(response)?;
                    } else {
                        log::debug!("unhandled lsp request. method:{}", request.method);
                        outgoing_tx.unbounded_send(
                            serde_json::to_vec(&AnyResponse {
                                id: request.id,
                                error: Some(Error {
                                    message: "no handler".to_string(),
                                }),
                                result: None,
                            })
                            .unwrap(),
                        )?;
                    }
                } else {
                    incoming_tx.unbounded_send(buffer.clone())?;
                }
            }
            Ok::<_, anyhow::Error>(())
        })
        .detach();

        // Send outgoing messages
        cx.background()
            .spawn(async move {
                let mut stdout = smol::io::BufWriter::new(stdout);
                while let Some(notification) = outgoing_rx.next().await {
                    Self::send(&mut stdout, &notification).await;
                }
            })
            .detach();

        this
    }

    pub async fn notify<T: notification::Notification>(&mut self, params: T::Params) {
        let message = serde_json::to_vec(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: T::METHOD,
            params,
        })
        .unwrap();
        self.outgoing_tx.unbounded_send(message).unwrap();
    }

    pub async fn receive_notification<T: notification::Notification>(&mut self) -> T::Params {
        use futures::StreamExt as _;

        loop {
            let bytes = self.incoming_rx.next().await.unwrap();
            if let Ok(notification) = serde_json::from_slice::<Notification<T::Params>>(&bytes) {
                assert_eq!(notification.method, T::METHOD);
                return notification.params;
            } else {
                log::info!(
                    "skipping message in fake language server {:?}",
                    std::str::from_utf8(&bytes)
                );
            }
        }
    }

    pub fn handle_request<T, F>(
        &mut self,
        mut handler: F,
    ) -> futures::channel::mpsc::UnboundedReceiver<()>
    where
        T: 'static + request::Request,
        F: 'static + Send + FnMut(T::Params, AsyncAppContext) -> T::Result,
    {
        let (responded_tx, responded_rx) = futures::channel::mpsc::unbounded();
        self.handlers.lock().insert(
            T::METHOD,
            Box::new(move |id, params, cx| {
                let result = handler(serde_json::from_slice::<T::Params>(params).unwrap(), cx);
                let result = serde_json::to_string(&result).unwrap();
                let result = serde_json::from_str::<&RawValue>(&result).unwrap();
                let response = AnyResponse {
                    id,
                    error: None,
                    result: Some(result),
                };
                responded_tx.unbounded_send(()).ok();
                serde_json::to_vec(&response).unwrap()
            }),
        );
        responded_rx
    }

    pub fn remove_request_handler<T>(&mut self)
    where
        T: 'static + request::Request,
    {
        self.handlers.lock().remove(T::METHOD);
    }

    pub async fn start_progress(&mut self, token: impl Into<String>) {
        self.notify::<notification::Progress>(ProgressParams {
            token: NumberOrString::String(token.into()),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(Default::default())),
        })
        .await;
    }

    pub async fn end_progress(&mut self, token: impl Into<String>) {
        self.notify::<notification::Progress>(ProgressParams {
            token: NumberOrString::String(token.into()),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(Default::default())),
        })
        .await;
    }

    async fn send(stdout: &mut smol::io::BufWriter<async_pipe::PipeWriter>, message: &[u8]) {
        stdout
            .write_all(CONTENT_LEN_HEADER.as_bytes())
            .await
            .unwrap();
        stdout
            .write_all((format!("{}", message.len())).as_bytes())
            .await
            .unwrap();
        stdout.write_all("\r\n\r\n".as_bytes()).await.unwrap();
        stdout.write_all(&message).await.unwrap();
        stdout.flush().await.unwrap();
    }

    async fn receive(
        stdin: &mut smol::io::BufReader<async_pipe::PipeReader>,
        buffer: &mut Vec<u8>,
    ) -> Result<()> {
        buffer.clear();
        stdin.read_until(b'\n', buffer).await?;
        stdin.read_until(b'\n', buffer).await?;
        let message_len: usize = std::str::from_utf8(buffer)
            .unwrap()
            .strip_prefix(CONTENT_LEN_HEADER)
            .unwrap()
            .trim_end()
            .parse()
            .unwrap();
        buffer.resize(message_len, 0);
        stdin.read_exact(buffer).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test]
    async fn test_fake(mut cx: TestAppContext) {
        let (server, mut fake) = cx.update(LanguageServer::fake);

        let (message_tx, message_rx) = channel::unbounded();
        let (diagnostics_tx, diagnostics_rx) = channel::unbounded();
        server
            .on_notification::<notification::ShowMessage, _>(move |params| {
                message_tx.try_send(params).unwrap()
            })
            .detach();
        server
            .on_notification::<notification::PublishDiagnostics, _>(move |params| {
                diagnostics_tx.try_send(params).unwrap()
            })
            .detach();

        server
            .notify::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams {
                text_document: TextDocumentItem::new(
                    Url::from_str("file://a/b").unwrap(),
                    "rust".to_string(),
                    0,
                    "".to_string(),
                ),
            })
            .await
            .unwrap();
        assert_eq!(
            fake.receive_notification::<notification::DidOpenTextDocument>()
                .await
                .text_document
                .uri
                .as_str(),
            "file://a/b"
        );

        fake.notify::<notification::ShowMessage>(ShowMessageParams {
            typ: MessageType::ERROR,
            message: "ok".to_string(),
        })
        .await;
        fake.notify::<notification::PublishDiagnostics>(PublishDiagnosticsParams {
            uri: Url::from_str("file://b/c").unwrap(),
            version: Some(5),
            diagnostics: vec![],
        })
        .await;
        assert_eq!(message_rx.recv().await.unwrap().message, "ok");
        assert_eq!(
            diagnostics_rx.recv().await.unwrap().uri.as_str(),
            "file://b/c"
        );

        fake.handle_request::<request::Shutdown, _>(|_, _| ());

        drop(server);
        fake.receive_notification::<notification::Exit>().await;
    }

    pub enum ServerStatusNotification {}

    impl notification::Notification for ServerStatusNotification {
        type Params = ServerStatusParams;
        const METHOD: &'static str = "experimental/serverStatus";
    }

    #[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
    pub struct ServerStatusParams {
        pub quiescent: bool,
    }
}
