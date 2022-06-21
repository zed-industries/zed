pub use lsp_types::*;

use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{channel::oneshot, io::BufWriter, AsyncRead, AsyncWrite};
use gpui::{executor, AsyncAppContext, Task};
use parking_lot::Mutex;
use postage::{barrier, prelude::Stream};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process,
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
use util::{ResultExt, TryFutureExt};

const JSON_RPC_VERSION: &'static str = "2.0";
const CONTENT_LEN_HEADER: &'static str = "Content-Length: ";

type NotificationHandler = Box<dyn Send + FnMut(Option<usize>, &str, AsyncAppContext)>;
type ResponseHandler = Box<dyn Send + FnOnce(Result<&str, Error>)>;

pub struct LanguageServer {
    server_id: usize,
    next_id: AtomicUsize,
    outbound_tx: channel::Sender<Vec<u8>>,
    name: String,
    capabilities: ServerCapabilities,
    notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<HashMap<usize, ResponseHandler>>>,
    executor: Arc<executor::Background>,
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    root_path: PathBuf,
}

pub struct Subscription {
    method: &'static str,
    notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
}

#[derive(Serialize, Deserialize)]
struct Request<'a, T> {
    jsonrpc: &'a str,
    id: usize,
    method: &'a str,
    params: T,
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
    result: Option<T>,
    error: Option<Error>,
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
        server_id: usize,
        binary_path: &Path,
        args: &[&str],
        root_path: &Path,
        cx: AsyncAppContext,
    ) -> Result<Self> {
        let working_dir = if root_path.is_dir() {
            root_path
        } else {
            root_path.parent().unwrap_or(Path::new("/"))
        };
        let mut server = process::Command::new(binary_path)
            .current_dir(working_dir)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let mut server =
            Self::new_internal(server_id, stdin, stdout, root_path, cx, |notification| {
                log::info!(
                    "unhandled notification {}:\n{}",
                    notification.method,
                    serde_json::to_string_pretty(
                        &Value::from_str(notification.params.get()).unwrap()
                    )
                    .unwrap()
                );
            });
        if let Some(name) = binary_path.file_name() {
            server.name = name.to_string_lossy().to_string();
        }
        Ok(server)
    }

    fn new_internal<Stdin, Stdout, F>(
        server_id: usize,
        stdin: Stdin,
        stdout: Stdout,
        root_path: &Path,
        cx: AsyncAppContext,
        mut on_unhandled_notification: F,
    ) -> Self
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
        Stdout: AsyncRead + Unpin + Send + 'static,
        F: FnMut(AnyNotification) + 'static + Send,
    {
        let mut stdin = BufWriter::new(stdin);
        let mut stdout = BufReader::new(stdout);
        let (outbound_tx, outbound_rx) = channel::unbounded::<Vec<u8>>();
        let notification_handlers =
            Arc::new(Mutex::new(HashMap::<_, NotificationHandler>::default()));
        let response_handlers = Arc::new(Mutex::new(HashMap::<_, ResponseHandler>::default()));
        let input_task = cx.spawn(|cx| {
            let notification_handlers = notification_handlers.clone();
            let response_handlers = response_handlers.clone();
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
                    log::trace!("incoming message:{}", String::from_utf8_lossy(&buffer));

                    if let Ok(msg) = serde_json::from_slice::<AnyNotification>(&buffer) {
                        if let Some(handler) = notification_handlers.lock().get_mut(msg.method) {
                            handler(msg.id, msg.params.get(), cx.clone());
                        } else {
                            on_unhandled_notification(msg);
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

                    // Don't starve the main thread when receiving lots of messages at once.
                    smol::future::yield_now().await;
                }
            }
            .log_err()
        });
        let (output_done_tx, output_done_rx) = barrier::channel();
        let output_task = cx.background().spawn({
            let response_handlers = response_handlers.clone();
            async move {
                let _clear_response_handlers = ClearResponseHandlers(response_handlers);
                let mut content_len_buffer = Vec::new();
                while let Ok(message) = outbound_rx.recv().await {
                    log::trace!("outgoing message:{}", String::from_utf8_lossy(&message));
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
            server_id,
            notification_handlers,
            response_handlers,
            name: Default::default(),
            capabilities: Default::default(),
            next_id: Default::default(),
            outbound_tx,
            executor: cx.background().clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            root_path: root_path.to_path_buf(),
        }
    }

    pub async fn initialize(mut self, options: Option<Value>) -> Result<Arc<Self>> {
        let root_uri = Url::from_file_path(&self.root_path).unwrap();
        #[allow(deprecated)]
        let params = InitializeParams {
            process_id: Default::default(),
            root_path: Default::default(),
            root_uri: Some(root_uri),
            initialization_options: options,
            capabilities: ClientCapabilities {
                workspace: Some(WorkspaceClientCapabilities {
                    configuration: Some(true),
                    did_change_configuration: Some(DynamicRegistrationClientCapabilities {
                        dynamic_registration: Some(true),
                    }),
                    ..Default::default()
                }),
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
                                    CodeActionKind::SOURCE.as_str().into(),
                                ],
                            },
                        }),
                        data_support: Some(true),
                        resolve_support: Some(CodeActionCapabilityResolveSupport {
                            properties: vec!["edit".to_string(), "command".to_string()],
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
                    rename: Some(RenameClientCapabilities {
                        prepare_support: Some(true),
                        ..Default::default()
                    }),
                    hover: Some(HoverClientCapabilities {
                        content_format: Some(vec![MarkupKind::Markdown]),
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

        let response = self.request::<request::Initialize>(params).await?;
        if let Some(info) = response.server_info {
            self.name = info.name;
        }
        self.capabilities = response.capabilities;

        self.notify::<notification::Initialized>(InitializedParams {})?;
        Ok(Arc::new(self))
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

    #[must_use]
    pub fn on_notification<T, F>(&self, f: F) -> Subscription
    where
        T: notification::Notification,
        F: 'static + Send + FnMut(T::Params, AsyncAppContext),
    {
        self.on_custom_notification(T::METHOD, f)
    }

    #[must_use]
    pub fn on_request<T, F, Fut>(&self, f: F) -> Subscription
    where
        T: request::Request,
        T::Params: 'static + Send,
        F: 'static + Send + FnMut(T::Params, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = Result<T::Result>>,
    {
        self.on_custom_request(T::METHOD, f)
    }

    pub fn remove_request_handler<T: request::Request>(&self) {
        self.notification_handlers.lock().remove(T::METHOD);
    }

    #[must_use]
    pub fn on_custom_notification<Params, F>(&self, method: &'static str, mut f: F) -> Subscription
    where
        F: 'static + Send + FnMut(Params, AsyncAppContext),
        Params: DeserializeOwned,
    {
        let prev_handler = self.notification_handlers.lock().insert(
            method,
            Box::new(move |_, params, cx| {
                if let Some(params) = serde_json::from_str(params).log_err() {
                    f(params, cx);
                }
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

    #[must_use]
    pub fn on_custom_request<Params, Res, Fut, F>(
        &self,
        method: &'static str,
        mut f: F,
    ) -> Subscription
    where
        F: 'static + Send + FnMut(Params, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = Result<Res>>,
        Params: DeserializeOwned + Send + 'static,
        Res: Serialize,
    {
        let outbound_tx = self.outbound_tx.clone();
        let prev_handler = self.notification_handlers.lock().insert(
            method,
            Box::new(move |id, params, cx| {
                if let Some(id) = id {
                    if let Some(params) = serde_json::from_str(params).log_err() {
                        let response = f(params, cx.clone());
                        cx.foreground()
                            .spawn({
                                let outbound_tx = outbound_tx.clone();
                                async move {
                                    let response = match response.await {
                                        Ok(result) => Response {
                                            id,
                                            result: Some(result),
                                            error: None,
                                        },
                                        Err(error) => Response {
                                            id,
                                            result: None,
                                            error: Some(Error {
                                                message: error.to_string(),
                                            }),
                                        },
                                    };
                                    if let Some(response) = serde_json::to_vec(&response).log_err()
                                    {
                                        outbound_tx.try_send(response).ok();
                                    }
                                }
                            })
                            .detach();
                    }
                }
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

    pub fn server_id(&self) -> usize {
        self.server_id
    }

    pub fn request<T: request::Request>(
        &self,
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
        self.notification_handlers.lock().remove(self.method);
    }
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone)]
pub struct FakeLanguageServer {
    pub server: Arc<LanguageServer>,
    notifications_rx: channel::Receiver<(String, String)>,
}

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

    pub fn fake(
        name: String,
        capabilities: ServerCapabilities,
        cx: AsyncAppContext,
    ) -> (Self, FakeLanguageServer) {
        let (stdin_writer, stdin_reader) = async_pipe::pipe();
        let (stdout_writer, stdout_reader) = async_pipe::pipe();
        let (notifications_tx, notifications_rx) = channel::unbounded();

        let server = Self::new_internal(
            0,
            stdin_writer,
            stdout_reader,
            Path::new("/"),
            cx.clone(),
            |_| {},
        );
        let fake = FakeLanguageServer {
            server: Arc::new(Self::new_internal(
                0,
                stdout_writer,
                stdin_reader,
                Path::new("/"),
                cx.clone(),
                move |msg| {
                    notifications_tx
                        .try_send((msg.method.to_string(), msg.params.get().to_string()))
                        .ok();
                },
            )),
            notifications_rx,
        };
        fake.handle_request::<request::Initialize, _, _>({
            let capabilities = capabilities.clone();
            move |_, _| {
                let capabilities = capabilities.clone();
                let name = name.clone();
                async move {
                    Ok(InitializeResult {
                        capabilities,
                        server_info: Some(ServerInfo {
                            name,
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                }
            }
        });

        (server, fake)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    pub fn notify<T: notification::Notification>(&self, params: T::Params) {
        self.server.notify::<T>(params).ok();
    }

    pub async fn receive_notification<T: notification::Notification>(&mut self) -> T::Params {
        self.try_receive_notification::<T>().await.unwrap()
    }

    pub async fn try_receive_notification<T: notification::Notification>(
        &mut self,
    ) -> Option<T::Params> {
        use futures::StreamExt as _;

        loop {
            let (method, params) = self.notifications_rx.next().await?;
            if &method == T::METHOD {
                return Some(serde_json::from_str::<T::Params>(&params).unwrap());
            } else {
                log::info!("skipping message in fake language server {:?}", params);
            }
        }
    }

    pub fn handle_request<T, F, Fut>(
        &self,
        mut handler: F,
    ) -> futures::channel::mpsc::UnboundedReceiver<()>
    where
        T: 'static + request::Request,
        T::Params: 'static + Send,
        F: 'static + Send + FnMut(T::Params, gpui::AsyncAppContext) -> Fut,
        Fut: 'static + Send + Future<Output = Result<T::Result>>,
    {
        let (responded_tx, responded_rx) = futures::channel::mpsc::unbounded();
        self.server.remove_request_handler::<T>();
        self.server
            .on_request::<T, _, _>(move |params, cx| {
                let result = handler(params, cx.clone());
                let responded_tx = responded_tx.clone();
                async move {
                    cx.background().simulate_random_delay().await;
                    let result = result.await;
                    responded_tx.unbounded_send(()).ok();
                    result
                }
            })
            .detach();
        responded_rx
    }

    pub fn remove_request_handler<T>(&mut self)
    where
        T: 'static + request::Request,
    {
        self.server.remove_request_handler::<T>();
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
        let (server, mut fake) =
            LanguageServer::fake("the-lsp".to_string(), Default::default(), cx.to_async());

        let (message_tx, message_rx) = channel::unbounded();
        let (diagnostics_tx, diagnostics_rx) = channel::unbounded();
        server
            .on_notification::<notification::ShowMessage, _>(move |params, _| {
                message_tx.try_send(params).unwrap()
            })
            .detach();
        server
            .on_notification::<notification::PublishDiagnostics, _>(move |params, _| {
                diagnostics_tx.try_send(params).unwrap()
            })
            .detach();

        let server = server.initialize(None).await.unwrap();
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

        fake.handle_request::<request::Shutdown, _, _>(|_, _| async move { Ok(()) });

        drop(server);
        fake.receive_notification::<notification::Exit>().await;
    }
}
