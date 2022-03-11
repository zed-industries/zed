use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{channel::oneshot, io::BufWriter, AsyncRead, AsyncWrite};
use gpui::{executor, Task};
use parking_lot::{Mutex, RwLock};
use postage::{barrier, prelude::Stream};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};
use std::{
    future::Future,
    io::Write,
    path::PathBuf,
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

type NotificationHandler =
    Box<dyn Send + Sync + FnMut(Option<usize>, &str, &mut channel::Sender<Vec<u8>>) -> Result<()>>;
type ResponseHandler = Box<dyn Send + FnOnce(Result<&str, Error>)>;

pub struct LanguageServer {
    next_id: AtomicUsize,
    outbound_tx: channel::Sender<Vec<u8>>,
    name: String,
    capabilities: ServerCapabilities,
    notification_handlers: Arc<RwLock<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<HashMap<usize, ResponseHandler>>>,
    executor: Arc<executor::Background>,
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    root_path: PathBuf,
    options: Option<Value>,
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

#[derive(Serialize)]
struct Response<T> {
    id: usize,
    result: T,
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
    #[serde(default)]
    id: Option<usize>,
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
        args: &[&str],
        root_path: &Path,
        options: Option<Value>,
        background: Arc<executor::Background>,
    ) -> Result<Self> {
        let working_dir = if root_path.is_dir() {
            root_path
        } else {
            root_path.parent().unwrap_or(Path::new("/"))
        };
        let mut server = Command::new(binary_path)
            .current_dir(working_dir)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let mut server = Self::new_internal(stdin, stdout, root_path, options, background);
        if let Some(name) = binary_path.file_name() {
            server.name = name.to_string_lossy().to_string();
        }
        Ok(server)
    }

    fn new_internal<Stdin, Stdout>(
        stdin: Stdin,
        stdout: Stdout,
        root_path: &Path,
        options: Option<Value>,
        executor: Arc<executor::Background>,
    ) -> Self
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut stdin = BufWriter::new(stdin);
        let mut stdout = BufReader::new(stdout);
        let (outbound_tx, outbound_rx) = channel::unbounded::<Vec<u8>>();
        let notification_handlers =
            Arc::new(RwLock::new(HashMap::<_, NotificationHandler>::default()));
        let response_handlers = Arc::new(Mutex::new(HashMap::<_, ResponseHandler>::default()));
        let input_task = executor.spawn(
            {
                let notification_handlers = notification_handlers.clone();
                let response_handlers = response_handlers.clone();
                let mut outbound_tx = outbound_tx.clone();
                async move {
                    let _clear_response_handlers = ClearResponseHandlers(response_handlers.clone());
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

                        if let Ok(AnyNotification { id, method, params }) =
                            serde_json::from_slice(&buffer)
                        {
                            if let Some(handler) = notification_handlers.write().get_mut(method) {
                                if let Err(e) = handler(id, params.get(), &mut outbound_tx) {
                                    log::error!("error handling {} message: {:?}", method, e);
                                }
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
        let output_task = executor.spawn({
            let response_handlers = response_handlers.clone();
            async move {
                let _clear_response_handlers = ClearResponseHandlers(response_handlers);
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
            .log_err()
        });

        Self {
            notification_handlers,
            response_handlers,
            name: Default::default(),
            capabilities: Default::default(),
            next_id: Default::default(),
            outbound_tx,
            executor: executor.clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            root_path: root_path.to_path_buf(),
            options,
        }
    }

    pub async fn initialize(mut self) -> Result<Arc<Self>> {
        let options = self.options.take();
        let mut this = Arc::new(self);
        let root_uri = Url::from_file_path(&this.root_path).unwrap();
        #[allow(deprecated)]
        let params = InitializeParams {
            process_id: Default::default(),
            root_path: Default::default(),
            root_uri: Some(root_uri),
            initialization_options: options,
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

        let response = this.request::<request::Initialize>(params).await?;
        {
            let this = Arc::get_mut(&mut this).unwrap();
            if let Some(info) = response.server_info {
                this.name = info.name;
            }
            this.capabilities = response.capabilities;
        }
        this.notify::<notification::Initialized>(InitializedParams {})?;
        Ok(this)
    }

    pub fn shutdown(&self) -> Option<impl 'static + Send + Future<Output = Option<()>>> {
        if let Some(tasks) = self.io_tasks.lock().take() {
            let response_handlers = self.response_handlers.clone();
            let next_id = AtomicUsize::new(self.next_id.load(SeqCst));
            let outbound_tx = self.outbound_tx.clone();
            let mut output_done = self.output_done_rx.lock().take().unwrap();
            let shutdown_request = Self::request_internal::<request::Shutdown>(
                &next_id,
                &response_handlers,
                &outbound_tx,
                (),
            );
            let exit = Self::notify_internal::<notification::Exit>(&outbound_tx, ());
            outbound_tx.close();
            Some(
                async move {
                    log::debug!("language server shutdown started");
                    shutdown_request.await?;
                    response_handlers.lock().clear();
                    exit?;
                    output_done.recv().await;
                    log::debug!("language server shutdown finished");
                    drop(tasks);
                    Ok(())
                }
                .log_err(),
            )
        } else {
            None
        }
    }

    pub fn on_notification<T, F>(&mut self, f: F) -> Subscription
    where
        T: notification::Notification,
        F: 'static + Send + Sync + FnMut(T::Params),
    {
        self.on_custom_notification(T::METHOD, f)
    }

    pub fn on_request<T, F>(&mut self, f: F) -> Subscription
    where
        T: request::Request,
        F: 'static + Send + Sync + FnMut(T::Params) -> Result<T::Result>,
    {
        self.on_custom_request(T::METHOD, f)
    }

    pub fn on_custom_notification<Params, F>(
        &mut self,
        method: &'static str,
        mut f: F,
    ) -> Subscription
    where
        F: 'static + Send + Sync + FnMut(Params),
        Params: DeserializeOwned,
    {
        let prev_handler = self.notification_handlers.write().insert(
            method,
            Box::new(move |_, params, _| {
                let params = serde_json::from_str(params)?;
                f(params);
                Ok(())
            }),
        );
        assert!(
            prev_handler.is_none(),
            "registered multiple handlers for the same LSP method"
        );
        Subscription {
            method,
            notification_handlers: self.notification_handlers.clone(),
        }
    }

    pub fn on_custom_request<Params, Res, F>(
        &mut self,
        method: &'static str,
        mut f: F,
    ) -> Subscription
    where
        F: 'static + Send + Sync + FnMut(Params) -> Result<Res>,
        Params: DeserializeOwned,
        Res: Serialize,
    {
        let prev_handler = self.notification_handlers.write().insert(
            method,
            Box::new(move |id, params, tx| {
                if let Some(id) = id {
                    let params = serde_json::from_str(params)?;
                    let result = f(params)?;
                    let response = serde_json::to_vec(&Response { id, result })?;
                    tx.try_send(response)?;
                }
                Ok(())
            }),
        );
        assert!(
            prev_handler.is_none(),
            "registered multiple handlers for the same LSP method"
        );
        Subscription {
            method,
            notification_handlers: self.notification_handlers.clone(),
        }
    }

    pub fn name<'a>(self: &'a Arc<Self>) -> &'a str {
        &self.name
    }

    pub fn capabilities<'a>(self: &'a Arc<Self>) -> &'a ServerCapabilities {
        &self.capabilities
    }

    pub fn request<T: request::Request>(
        self: &Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<T::Result>>
    where
        T::Result: 'static + Send,
    {
        Self::request_internal::<T>(
            &self.next_id,
            &self.response_handlers,
            &self.outbound_tx,
            params,
        )
    }

    fn request_internal<T: request::Request>(
        next_id: &AtomicUsize,
        response_handlers: &Mutex<HashMap<usize, ResponseHandler>>,
        outbound_tx: &channel::Sender<Vec<u8>>,
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

        let send = outbound_tx
            .try_send(message)
            .context("failed to write to language server's stdin");

        let (tx, rx) = oneshot::channel();
        response_handlers.lock().insert(
            id,
            Box::new(move |result| {
                let response = match result {
                    Ok(response) => {
                        serde_json::from_str(response).context("failed to deserialize response")
                    }
                    Err(error) => Err(anyhow!("{}", error.message)),
                };
                let _ = tx.send(response);
            }),
        );

        async move {
            send?;
            rx.await?
        }
    }

    pub fn notify<T: notification::Notification>(&self, params: T::Params) -> Result<()> {
        Self::notify_internal::<T>(&self.outbound_tx, params)
    }

    fn notify_internal<T: notification::Notification>(
        outbound_tx: &channel::Sender<Vec<u8>>,
        params: T::Params,
    ) -> Result<()> {
        let message = serde_json::to_vec(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: T::METHOD,
            params,
        })
        .unwrap();
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
    handlers: FakeLanguageServerHandlers,
    outgoing_tx: futures::channel::mpsc::UnboundedSender<Vec<u8>>,
    incoming_rx: futures::channel::mpsc::UnboundedReceiver<Vec<u8>>,
    _input_task: Task<Result<()>>,
    _output_task: Task<Result<()>>,
}

#[cfg(any(test, feature = "test-support"))]
type FakeLanguageServerHandlers = Arc<
    Mutex<
        HashMap<
            &'static str,
            Box<dyn Send + FnMut(usize, &[u8], gpui::AsyncAppContext) -> Vec<u8>>,
        >,
    >,
>;

#[cfg(any(test, feature = "test-support"))]
impl LanguageServer {
    pub fn full_capabilities() -> ServerCapabilities {
        ServerCapabilities {
            document_highlight_provider: Some(OneOf::Left(true)),
            code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            ..Default::default()
        }
    }

    pub fn fake(cx: &mut gpui::MutableAppContext) -> (Self, FakeLanguageServer) {
        Self::fake_with_capabilities(Self::full_capabilities(), cx)
    }

    pub fn fake_with_capabilities(
        capabilities: ServerCapabilities,
        cx: &mut gpui::MutableAppContext,
    ) -> (Self, FakeLanguageServer) {
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

        let executor = cx.background().clone();
        let server =
            Self::new_internal(stdin_writer, stdout_reader, Path::new("/"), None, executor);
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
        let handlers = FakeLanguageServerHandlers::default();

        let input_task = cx.spawn(|cx| {
            let handlers = handlers.clone();
            let outgoing_tx = outgoing_tx.clone();
            async move {
                let mut buffer = Vec::new();
                let mut stdin = smol::io::BufReader::new(stdin);
                while Self::receive(&mut stdin, &mut buffer).await.is_ok() {
                    cx.background().simulate_random_delay().await;

                    if let Ok(request) = serde_json::from_slice::<AnyRequest>(&buffer) {
                        assert_eq!(request.jsonrpc, JSON_RPC_VERSION);

                        let response;
                        if let Some(handler) = handlers.lock().get_mut(request.method) {
                            response =
                                handler(request.id, request.params.get().as_bytes(), cx.clone());
                            log::debug!("handled lsp request. method:{}", request.method);
                        } else {
                            response = serde_json::to_vec(&AnyResponse {
                                id: request.id,
                                error: Some(Error {
                                    message: "no handler".to_string(),
                                }),
                                result: None,
                            })
                            .unwrap();
                            log::debug!("unhandled lsp request. method:{}", request.method);
                        }
                        outgoing_tx.unbounded_send(response)?;
                    } else {
                        incoming_tx.unbounded_send(buffer.clone())?;
                    }
                }
                Ok::<_, anyhow::Error>(())
            }
        });

        let output_task = cx.background().spawn(async move {
            let mut stdout = smol::io::BufWriter::new(stdout);
            while let Some(message) = outgoing_rx.next().await {
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
            Ok(())
        });

        Self {
            outgoing_tx,
            incoming_rx,
            handlers,
            _input_task: input_task,
            _output_task: output_task,
        }
    }

    pub fn notify<T: notification::Notification>(&mut self, params: T::Params) {
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
        F: 'static + Send + FnMut(T::Params, gpui::AsyncAppContext) -> T::Result,
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
        });
    }

    pub async fn end_progress(&mut self, token: impl Into<String>) {
        self.notify::<notification::Progress>(ProgressParams {
            token: NumberOrString::String(token.into()),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(Default::default())),
        });
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
            .ok_or_else(|| anyhow!("invalid content length header"))?
            .trim_end()
            .parse()
            .unwrap();
        buffer.resize(message_len, 0);
        stdin.read_exact(buffer).await?;
        Ok(())
    }
}

struct ClearResponseHandlers(Arc<Mutex<HashMap<usize, ResponseHandler>>>);

impl Drop for ClearResponseHandlers {
    fn drop(&mut self) {
        self.0.lock().clear();
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
    async fn test_fake(cx: &mut TestAppContext) {
        let (mut server, mut fake) = cx.update(LanguageServer::fake);

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

        let server = server.initialize().await.unwrap();
        server
            .notify::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams {
                text_document: TextDocumentItem::new(
                    Url::from_str("file://a/b").unwrap(),
                    "rust".to_string(),
                    0,
                    "".to_string(),
                ),
            })
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
        });
        fake.notify::<notification::PublishDiagnostics>(PublishDiagnosticsParams {
            uri: Url::from_str("file://b/c").unwrap(),
            version: Some(5),
            diagnostics: vec![],
        });
        assert_eq!(message_rx.recv().await.unwrap().message, "ok");
        assert_eq!(
            diagnostics_rx.recv().await.unwrap().uri.as_str(),
            "file://b/c"
        );

        fake.handle_request::<request::Shutdown, _>(|_, _| ());

        drop(server);
        fake.receive_notification::<notification::Exit>().await;
    }
}
