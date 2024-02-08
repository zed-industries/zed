use log::warn;
pub use lsp_types::request::*;
pub use lsp_types::*;

use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{channel::oneshot, io::BufWriter, AsyncRead, AsyncWrite, FutureExt};
use gpui::{AppContext, AsyncAppContext, BackgroundExecutor, Task};
use parking_lot::Mutex;
use postage::{barrier, prelude::Stream};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, value::RawValue, Value};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{self, Child},
};
use std::{
    ffi::OsString,
    fmt,
    future::Future,
    io::Write,
    path::PathBuf,
    str::{self, FromStr as _},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc, Weak,
    },
    time::{Duration, Instant},
};
use std::{path::Path, process::Stdio};
use util::{ResultExt, TryFutureExt};

const JSON_RPC_VERSION: &str = "2.0";
const CONTENT_LEN_HEADER: &str = "Content-Length: ";
const LSP_REQUEST_TIMEOUT: Duration = Duration::from_secs(60 * 2);

type NotificationHandler = Box<dyn Send + FnMut(Option<usize>, &str, AsyncAppContext)>;
type ResponseHandler = Box<dyn Send + FnOnce(Result<String, Error>)>;
type IoHandler = Box<dyn Send + FnMut(IoKind, &str)>;

/// Kind of language server stdio given to an IO handler.
#[derive(Debug, Clone, Copy)]
pub enum IoKind {
    StdOut,
    StdIn,
    StdErr,
}

/// Represents a launchable language server. This can either be a standalone binary or the path
/// to a runtime with arguments to instruct it to launch the actual language server file.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageServerBinary {
    pub path: PathBuf,
    pub arguments: Vec<OsString>,
}

/// A running language server process.
pub struct LanguageServer {
    server_id: LanguageServerId,
    next_id: AtomicUsize,
    outbound_tx: channel::Sender<String>,
    name: String,
    capabilities: ServerCapabilities,
    code_action_kinds: Option<Vec<CodeActionKind>>,
    notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<Option<HashMap<usize, ResponseHandler>>>>,
    io_handlers: Arc<Mutex<HashMap<usize, IoHandler>>>,
    executor: BackgroundExecutor,
    #[allow(clippy::type_complexity)]
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    root_path: PathBuf,
    _server: Option<Mutex<Child>>,
}

/// Identifies a running language server.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LanguageServerId(pub usize);

/// Handle to a language server RPC activity subscription.
pub enum Subscription {
    Notification {
        method: &'static str,
        notification_handlers: Option<Arc<Mutex<HashMap<&'static str, NotificationHandler>>>>,
    },
    Io {
        id: usize,
        io_handlers: Option<Weak<Mutex<HashMap<usize, IoHandler>>>>,
    },
}

/// Language server protocol RPC request message.
///
/// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
#[derive(Serialize, Deserialize)]
pub struct Request<'a, T> {
    jsonrpc: &'static str,
    id: usize,
    method: &'a str,
    params: T,
}

/// Language server protocol RPC request response message before it is deserialized into a concrete type.
#[derive(Serialize, Deserialize)]
struct AnyResponse<'a> {
    jsonrpc: &'a str,
    id: usize,
    #[serde(default)]
    error: Option<Error>,
    #[serde(borrow)]
    result: Option<&'a RawValue>,
}

/// Language server protocol RPC request response message.
///
/// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#responseMessage)
#[derive(Serialize)]
struct Response<T> {
    jsonrpc: &'static str,
    id: usize,
    result: Option<T>,
    error: Option<Error>,
}

/// Language server protocol RPC notification message.
///
/// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#notificationMessage)
#[derive(Serialize, Deserialize)]
struct Notification<'a, T> {
    jsonrpc: &'static str,
    #[serde(borrow)]
    method: &'a str,
    params: T,
}

/// Language server RPC notification message before it is deserialized into a concrete type.
#[derive(Debug, Clone, Deserialize)]
struct AnyNotification<'a> {
    #[serde(default)]
    id: Option<usize>,
    #[serde(borrow)]
    method: &'a str,
    #[serde(borrow, default)]
    params: Option<&'a RawValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
}

impl LanguageServer {
    /// Starts a language server process.
    pub fn new(
        stderr_capture: Arc<Mutex<Option<String>>>,
        server_id: LanguageServerId,
        binary: LanguageServerBinary,
        root_path: &Path,
        code_action_kinds: Option<Vec<CodeActionKind>>,
        cx: AsyncAppContext,
    ) -> Result<Self> {
        let working_dir = if root_path.is_dir() {
            root_path
        } else {
            root_path.parent().unwrap_or_else(|| Path::new("/"))
        };

        let mut server = process::Command::new(&binary.path)
            .current_dir(working_dir)
            .args(binary.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let stderr = server.stderr.take().unwrap();
        let mut server = Self::new_internal(
            server_id.clone(),
            stdin,
            stdout,
            Some(stderr),
            stderr_capture,
            Some(server),
            root_path,
            code_action_kinds,
            cx,
            move |notification| {
                log::info!(
                    "{} unhandled notification {}:\n{}",
                    server_id,
                    notification.method,
                    serde_json::to_string_pretty(
                        &notification
                            .params
                            .and_then(|params| Value::from_str(params.get()).ok())
                            .unwrap_or(Value::Null)
                    )
                    .unwrap(),
                );
            },
        );

        if let Some(name) = binary.path.file_name() {
            server.name = name.to_string_lossy().to_string();
        }

        Ok(server)
    }

    fn new_internal<Stdin, Stdout, Stderr, F>(
        server_id: LanguageServerId,
        stdin: Stdin,
        stdout: Stdout,
        stderr: Option<Stderr>,
        stderr_capture: Arc<Mutex<Option<String>>>,
        server: Option<Child>,
        root_path: &Path,
        code_action_kinds: Option<Vec<CodeActionKind>>,
        cx: AsyncAppContext,
        on_unhandled_notification: F,
    ) -> Self
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
        Stdout: AsyncRead + Unpin + Send + 'static,
        Stderr: AsyncRead + Unpin + Send + 'static,
        F: FnMut(AnyNotification) + 'static + Send + Sync + Clone,
    {
        let (outbound_tx, outbound_rx) = channel::unbounded::<String>();
        let (output_done_tx, output_done_rx) = barrier::channel();
        let notification_handlers =
            Arc::new(Mutex::new(HashMap::<_, NotificationHandler>::default()));
        let response_handlers =
            Arc::new(Mutex::new(Some(HashMap::<_, ResponseHandler>::default())));
        let io_handlers = Arc::new(Mutex::new(HashMap::default()));

        let stdout_input_task = cx.spawn({
            let on_unhandled_notification = on_unhandled_notification.clone();
            let notification_handlers = notification_handlers.clone();
            let response_handlers = response_handlers.clone();
            let io_handlers = io_handlers.clone();
            move |cx| {
                Self::handle_input(
                    stdout,
                    on_unhandled_notification,
                    notification_handlers,
                    response_handlers,
                    io_handlers,
                    cx,
                )
                .log_err()
            }
        });
        let stderr_input_task = stderr
            .map(|stderr| {
                let io_handlers = io_handlers.clone();
                let stderr_captures = stderr_capture.clone();
                cx.spawn(|_| Self::handle_stderr(stderr, io_handlers, stderr_captures).log_err())
            })
            .unwrap_or_else(|| Task::Ready(Some(None)));
        let input_task = cx.spawn(|_| async move {
            let (stdout, stderr) = futures::join!(stdout_input_task, stderr_input_task);
            stdout.or(stderr)
        });
        let output_task = cx.background_executor().spawn({
            Self::handle_output(
                stdin,
                outbound_rx,
                output_done_tx,
                response_handlers.clone(),
                io_handlers.clone(),
            )
            .log_err()
        });

        Self {
            server_id,
            notification_handlers,
            response_handlers,
            io_handlers,
            name: Default::default(),
            capabilities: Default::default(),
            code_action_kinds,
            next_id: Default::default(),
            outbound_tx,
            executor: cx.background_executor().clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            root_path: root_path.to_path_buf(),
            _server: server.map(|server| Mutex::new(server)),
        }
    }

    /// List of code action kinds this language server reports being able to emit.
    pub fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        self.code_action_kinds.clone()
    }

    async fn handle_input<Stdout, F>(
        stdout: Stdout,
        mut on_unhandled_notification: F,
        notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
        response_handlers: Arc<Mutex<Option<HashMap<usize, ResponseHandler>>>>,
        io_handlers: Arc<Mutex<HashMap<usize, IoHandler>>>,
        cx: AsyncAppContext,
    ) -> anyhow::Result<()>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
        F: FnMut(AnyNotification) + 'static + Send,
    {
        let mut stdout = BufReader::new(stdout);
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        let mut buffer = Vec::new();
        loop {
            buffer.clear();

            if stdout.read_until(b'\n', &mut buffer).await? == 0 {
                break;
            };

            if stdout.read_until(b'\n', &mut buffer).await? == 0 {
                break;
            };

            let header = std::str::from_utf8(&buffer)?;
            let mut segments = header.lines();

            let message_len: usize = segments
                .next()
                .with_context(|| {
                    format!("unable to find the first line of the LSP message header `{header}`")
                })?
                .strip_prefix(CONTENT_LEN_HEADER)
                .with_context(|| format!("invalid LSP message header `{header}`"))?
                .parse()
                .with_context(|| {
                    format!("failed to parse Content-Length of LSP message header: `{header}`")
                })?;

            if let Some(second_segment) = segments.next() {
                match second_segment {
                    "" => (), // Header end
                    header_field => {
                        if header_field.starts_with("Content-Type:") {
                            stdout.read_until(b'\n', &mut buffer).await?;
                        } else {
                            anyhow::bail!(
                                "inside `{header}`, expected a Content-Type header field or a header ending CRLF, got `{second_segment:?}`"
                            )
                        }
                    }
                }
            } else {
                anyhow::bail!(
                    "unable to find the second line of the LSP message header `{header}`"
                );
            }

            buffer.resize(message_len, 0);
            stdout.read_exact(&mut buffer).await?;

            if let Ok(message) = str::from_utf8(&buffer) {
                log::trace!("incoming message: {message}");
                for handler in io_handlers.lock().values_mut() {
                    handler(IoKind::StdOut, message);
                }
            }

            if let Ok(msg) = serde_json::from_slice::<AnyNotification>(&buffer) {
                if let Some(handler) = notification_handlers.lock().get_mut(msg.method) {
                    handler(
                        msg.id,
                        msg.params.map(|params| params.get()).unwrap_or("null"),
                        cx.clone(),
                    );
                } else {
                    on_unhandled_notification(msg);
                }
            } else if let Ok(AnyResponse {
                id, error, result, ..
            }) = serde_json::from_slice(&buffer)
            {
                if let Some(handler) = response_handlers
                    .lock()
                    .as_mut()
                    .and_then(|handlers| handlers.remove(&id))
                {
                    if let Some(error) = error {
                        handler(Err(error));
                    } else if let Some(result) = result {
                        handler(Ok(result.get().into()));
                    } else {
                        handler(Ok("null".into()));
                    }
                }
            } else {
                warn!(
                    "failed to deserialize LSP message:\n{}",
                    std::str::from_utf8(&buffer)?
                );
            }

            // Don't starve the main thread when receiving lots of messages at once.
            smol::future::yield_now().await;
        }

        Ok(())
    }

    async fn handle_stderr<Stderr>(
        stderr: Stderr,
        io_handlers: Arc<Mutex<HashMap<usize, IoHandler>>>,
        stderr_capture: Arc<Mutex<Option<String>>>,
    ) -> anyhow::Result<()>
    where
        Stderr: AsyncRead + Unpin + Send + 'static,
    {
        let mut stderr = BufReader::new(stderr);
        let mut buffer = Vec::new();

        loop {
            buffer.clear();

            let bytes_read = stderr.read_until(b'\n', &mut buffer).await?;
            if bytes_read == 0 {
                return Ok(());
            }

            if let Ok(message) = str::from_utf8(&buffer) {
                log::trace!("incoming stderr message:{message}");
                for handler in io_handlers.lock().values_mut() {
                    handler(IoKind::StdErr, message);
                }

                if let Some(stderr) = stderr_capture.lock().as_mut() {
                    stderr.push_str(message);
                }
            }

            // Don't starve the main thread when receiving lots of messages at once.
            smol::future::yield_now().await;
        }
    }

    async fn handle_output<Stdin>(
        stdin: Stdin,
        outbound_rx: channel::Receiver<String>,
        output_done_tx: barrier::Sender,
        response_handlers: Arc<Mutex<Option<HashMap<usize, ResponseHandler>>>>,
        io_handlers: Arc<Mutex<HashMap<usize, IoHandler>>>,
    ) -> anyhow::Result<()>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
    {
        let mut stdin = BufWriter::new(stdin);
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        let mut content_len_buffer = Vec::new();
        while let Ok(message) = outbound_rx.recv().await {
            log::trace!("outgoing message:{}", message);
            for handler in io_handlers.lock().values_mut() {
                handler(IoKind::StdIn, &message);
            }

            content_len_buffer.clear();
            write!(content_len_buffer, "{}", message.len()).unwrap();
            stdin.write_all(CONTENT_LEN_HEADER.as_bytes()).await?;
            stdin.write_all(&content_len_buffer).await?;
            stdin.write_all("\r\n\r\n".as_bytes()).await?;
            stdin.write_all(message.as_bytes()).await?;
            stdin.flush().await?;
        }
        drop(output_done_tx);
        Ok(())
    }

    /// Initializes a language server by sending the `Initialize` request.
    /// Note that `options` is used directly to construct [`InitializeParams`], which is why it is owned.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize)
    pub fn initialize(
        mut self,
        options: Option<Value>,
        cx: &AppContext,
    ) -> Task<Result<Arc<Self>>> {
        let root_uri = Url::from_file_path(&self.root_path).unwrap();
        #[allow(deprecated)]
        let params = InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: Some(root_uri.clone()),
            initialization_options: options,
            capabilities: ClientCapabilities {
                workspace: Some(WorkspaceClientCapabilities {
                    configuration: Some(true),
                    did_change_watched_files: Some(DidChangeWatchedFilesClientCapabilities {
                        dynamic_registration: Some(true),
                        relative_pattern_support: Some(true),
                    }),
                    did_change_configuration: Some(DynamicRegistrationClientCapabilities {
                        dynamic_registration: Some(true),
                    }),
                    workspace_folders: Some(true),
                    symbol: Some(WorkspaceSymbolClientCapabilities {
                        resolve_support: None,
                        ..WorkspaceSymbolClientCapabilities::default()
                    }),
                    inlay_hint: Some(InlayHintWorkspaceClientCapabilities {
                        refresh_support: Some(true),
                    }),
                    diagnostic: Some(DiagnosticWorkspaceClientCapabilities {
                        refresh_support: None,
                    }),
                    workspace_edit: Some(WorkspaceEditClientCapabilities {
                        resource_operations: Some(vec![
                            ResourceOperationKind::Create,
                            ResourceOperationKind::Rename,
                            ResourceOperationKind::Delete,
                        ]),
                        document_changes: Some(true),
                        ..WorkspaceEditClientCapabilities::default()
                    }),
                    ..Default::default()
                }),
                text_document: Some(TextDocumentClientCapabilities {
                    definition: Some(GotoCapability {
                        link_support: Some(true),
                        dynamic_registration: None,
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
                                properties: vec![
                                    "documentation".to_string(),
                                    "additionalTextEdits".to_string(),
                                ],
                            }),
                            ..Default::default()
                        }),
                        completion_list: Some(CompletionListCapability {
                            item_defaults: Some(vec![
                                "commitCharacters".to_owned(),
                                "editRange".to_owned(),
                                "insertTextMode".to_owned(),
                                "data".to_owned(),
                            ]),
                        }),
                        ..Default::default()
                    }),
                    rename: Some(RenameClientCapabilities {
                        prepare_support: Some(true),
                        ..Default::default()
                    }),
                    hover: Some(HoverClientCapabilities {
                        content_format: Some(vec![MarkupKind::Markdown]),
                        dynamic_registration: None,
                    }),
                    inlay_hint: Some(InlayHintClientCapabilities {
                        resolve_support: Some(InlayHintResolveClientCapabilities {
                            properties: vec![
                                "textEdits".to_string(),
                                "tooltip".to_string(),
                                "label.tooltip".to_string(),
                                "label.location".to_string(),
                                "label.command".to_string(),
                            ],
                        }),
                        dynamic_registration: Some(false),
                    }),
                    publish_diagnostics: Some(PublishDiagnosticsClientCapabilities {
                        related_information: Some(true),
                        ..Default::default()
                    }),
                    formatting: Some(DynamicRegistrationClientCapabilities {
                        dynamic_registration: None,
                    }),
                    on_type_formatting: Some(DynamicRegistrationClientCapabilities {
                        dynamic_registration: None,
                    }),
                    diagnostic: Some(DiagnosticClientCapabilities {
                        related_document_support: Some(true),
                        dynamic_registration: None,
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
                general: None,
            },
            trace: None,
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: root_uri,
                name: Default::default(),
            }]),
            client_info: Some(ClientInfo {
                name: release_channel::ReleaseChannel::global(cx)
                    .display_name()
                    .to_string(),
                version: Some(release_channel::AppVersion::global(cx).to_string()),
            }),
            locale: None,
        };

        cx.spawn(|_| async move {
            let response = self.request::<request::Initialize>(params).await?;
            if let Some(info) = response.server_info {
                self.name = info.name;
            }
            self.capabilities = response.capabilities;

            self.notify::<notification::Initialized>(InitializedParams {})?;
            Ok(Arc::new(self))
        })
    }

    /// Sends a shutdown request to the language server process and prepares the [`LanguageServer`] to be dropped.
    pub fn shutdown(&self) -> Option<impl 'static + Send + Future<Output = Option<()>>> {
        if let Some(tasks) = self.io_tasks.lock().take() {
            let response_handlers = self.response_handlers.clone();
            let next_id = AtomicUsize::new(self.next_id.load(SeqCst));
            let outbound_tx = self.outbound_tx.clone();
            let executor = self.executor.clone();
            let mut output_done = self.output_done_rx.lock().take().unwrap();
            let shutdown_request = Self::request_internal::<request::Shutdown>(
                &next_id,
                &response_handlers,
                &outbound_tx,
                &executor,
                (),
            );
            let exit = Self::notify_internal::<notification::Exit>(&outbound_tx, ());
            outbound_tx.close();
            Some(
                async move {
                    log::debug!("language server shutdown started");
                    shutdown_request.await?;
                    response_handlers.lock().take();
                    exit?;
                    output_done.recv().await;
                    log::debug!("language server shutdown finished");
                    drop(tasks);
                    anyhow::Ok(())
                }
                .log_err(),
            )
        } else {
            None
        }
    }

    /// Register a handler to handle incoming LSP notifications.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#notificationMessage)
    #[must_use]
    pub fn on_notification<T, F>(&self, f: F) -> Subscription
    where
        T: notification::Notification,
        F: 'static + Send + FnMut(T::Params, AsyncAppContext),
    {
        self.on_custom_notification(T::METHOD, f)
    }

    /// Register a handler to handle incoming LSP requests.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
    #[must_use]
    pub fn on_request<T, F, Fut>(&self, f: F) -> Subscription
    where
        T: request::Request,
        T::Params: 'static + Send,
        F: 'static + FnMut(T::Params, AsyncAppContext) -> Fut + Send,
        Fut: 'static + Future<Output = Result<T::Result>>,
    {
        self.on_custom_request(T::METHOD, f)
    }

    /// Registers a handler to inspect all language server process stdio.
    #[must_use]
    pub fn on_io<F>(&self, f: F) -> Subscription
    where
        F: 'static + Send + FnMut(IoKind, &str),
    {
        let id = self.next_id.fetch_add(1, SeqCst);
        self.io_handlers.lock().insert(id, Box::new(f));
        Subscription::Io {
            id,
            io_handlers: Some(Arc::downgrade(&self.io_handlers)),
        }
    }

    /// Removes a request handler registers via [`Self::on_request`].
    pub fn remove_request_handler<T: request::Request>(&self) {
        self.notification_handlers.lock().remove(T::METHOD);
    }

    /// Removes a notification handler registers via [`Self::on_notification`].
    pub fn remove_notification_handler<T: notification::Notification>(&self) {
        self.notification_handlers.lock().remove(T::METHOD);
    }

    /// Checks if a notification handler has been registered via [`Self::on_notification`].
    pub fn has_notification_handler<T: notification::Notification>(&self) -> bool {
        self.notification_handlers.lock().contains_key(T::METHOD)
    }

    #[must_use]
    fn on_custom_notification<Params, F>(&self, method: &'static str, mut f: F) -> Subscription
    where
        F: 'static + FnMut(Params, AsyncAppContext) + Send,
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
        Subscription::Notification {
            method,
            notification_handlers: Some(self.notification_handlers.clone()),
        }
    }

    #[must_use]
    fn on_custom_request<Params, Res, Fut, F>(&self, method: &'static str, mut f: F) -> Subscription
    where
        F: 'static + FnMut(Params, AsyncAppContext) -> Fut + Send,
        Fut: 'static + Future<Output = Result<Res>>,
        Params: DeserializeOwned + Send + 'static,
        Res: Serialize,
    {
        let outbound_tx = self.outbound_tx.clone();
        let prev_handler = self.notification_handlers.lock().insert(
            method,
            Box::new(move |id, params, cx| {
                if let Some(id) = id {
                    match serde_json::from_str(params) {
                        Ok(params) => {
                            let response = f(params, cx.clone());
                            cx.foreground_executor()
                                .spawn({
                                    let outbound_tx = outbound_tx.clone();
                                    async move {
                                        let response = match response.await {
                                            Ok(result) => Response {
                                                jsonrpc: JSON_RPC_VERSION,
                                                id,
                                                result: Some(result),
                                                error: None,
                                            },
                                            Err(error) => Response {
                                                jsonrpc: JSON_RPC_VERSION,
                                                id,
                                                result: None,
                                                error: Some(Error {
                                                    message: error.to_string(),
                                                }),
                                            },
                                        };
                                        if let Some(response) =
                                            serde_json::to_string(&response).log_err()
                                        {
                                            outbound_tx.try_send(response).ok();
                                        }
                                    }
                                })
                                .detach();
                        }

                        Err(error) => {
                            log::error!(
                                "error deserializing {} request: {:?}, message: {:?}",
                                method,
                                error,
                                params
                            );
                            let response = AnyResponse {
                                jsonrpc: JSON_RPC_VERSION,
                                id,
                                result: None,
                                error: Some(Error {
                                    message: error.to_string(),
                                }),
                            };
                            if let Some(response) = serde_json::to_string(&response).log_err() {
                                outbound_tx.try_send(response).ok();
                            }
                        }
                    }
                }
            }),
        );
        assert!(
            prev_handler.is_none(),
            "registered multiple handlers for the same LSP method"
        );
        Subscription::Notification {
            method,
            notification_handlers: Some(self.notification_handlers.clone()),
        }
    }

    /// Get the name of the running language server.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the reported capabilities of the running language server.
    pub fn capabilities(&self) -> &ServerCapabilities {
        &self.capabilities
    }

    /// Get the id of the running language server.
    pub fn server_id(&self) -> LanguageServerId {
        self.server_id
    }

    /// Get the root path of the project the language server is running against.
    pub fn root_path(&self) -> &PathBuf {
        &self.root_path
    }

    /// Sends a RPC request to the language server.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
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
            &self.executor,
            params,
        )
    }

    fn request_internal<T: request::Request>(
        next_id: &AtomicUsize,
        response_handlers: &Mutex<Option<HashMap<usize, ResponseHandler>>>,
        outbound_tx: &channel::Sender<String>,
        executor: &BackgroundExecutor,
        params: T::Params,
    ) -> impl 'static + Future<Output = anyhow::Result<T::Result>>
    where
        T::Result: 'static + Send,
    {
        let id = next_id.fetch_add(1, SeqCst);
        let message = serde_json::to_string(&Request {
            jsonrpc: JSON_RPC_VERSION,
            id,
            method: T::METHOD,
            params,
        })
        .unwrap();

        let (tx, rx) = oneshot::channel();
        let handle_response = response_handlers
            .lock()
            .as_mut()
            .ok_or_else(|| anyhow!("server shut down"))
            .map(|handlers| {
                let executor = executor.clone();
                handlers.insert(
                    id,
                    Box::new(move |result| {
                        executor
                            .spawn(async move {
                                let response = match result {
                                    Ok(response) => serde_json::from_str(&response)
                                        .context("failed to deserialize response"),
                                    Err(error) => Err(anyhow!("{}", error.message)),
                                };
                                _ = tx.send(response);
                            })
                            .detach();
                    }),
                );
            });

        let send = outbound_tx
            .try_send(message)
            .context("failed to write to language server's stdin");

        let outbound_tx = outbound_tx.downgrade();
        let mut timeout = executor.timer(LSP_REQUEST_TIMEOUT).fuse();
        let started = Instant::now();
        async move {
            handle_response?;
            send?;

            let cancel_on_drop = util::defer(move || {
                if let Some(outbound_tx) = outbound_tx.upgrade() {
                    Self::notify_internal::<notification::Cancel>(
                        &outbound_tx,
                        CancelParams {
                            id: NumberOrString::Number(id as i32),
                        },
                    )
                    .log_err();
                }
            });

            let method = T::METHOD;
            futures::select! {
                response = rx.fuse() => {
                    let elapsed = started.elapsed();
                    log::trace!("Took {elapsed:?} to receive response to {method:?} id {id}");
                    cancel_on_drop.abort();
                    response?
                }

                _ = timeout => {
                    log::error!("Cancelled LSP request task for {method:?} id {id} which took over {LSP_REQUEST_TIMEOUT:?}");
                    anyhow::bail!("LSP request timeout");
                }
            }
        }
    }

    /// Sends a RPC notification to the language server.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#notificationMessage)
    pub fn notify<T: notification::Notification>(&self, params: T::Params) -> Result<()> {
        Self::notify_internal::<T>(&self.outbound_tx, params)
    }

    fn notify_internal<T: notification::Notification>(
        outbound_tx: &channel::Sender<String>,
        params: T::Params,
    ) -> Result<()> {
        let message = serde_json::to_string(&Notification {
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
    /// Detaching a subscription handle prevents it from unsubscribing on drop.
    pub fn detach(&mut self) {
        match self {
            Subscription::Notification {
                notification_handlers,
                ..
            } => *notification_handlers = None,
            Subscription::Io { io_handlers, .. } => *io_handlers = None,
        }
    }
}

impl fmt::Display for LanguageServerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for LanguageServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LanguageServer")
            .field("id", &self.server_id.0)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        match self {
            Subscription::Notification {
                method,
                notification_handlers,
            } => {
                if let Some(handlers) = notification_handlers {
                    handlers.lock().remove(method);
                }
            }
            Subscription::Io { id, io_handlers } => {
                if let Some(io_handlers) = io_handlers.as_ref().and_then(|h| h.upgrade()) {
                    io_handlers.lock().remove(id);
                }
            }
        }
    }
}

/// Mock language server for use in tests.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone)]
pub struct FakeLanguageServer {
    pub server: Arc<LanguageServer>,
    notifications_rx: channel::Receiver<(String, String)>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    /// Construct a fake language server.
    pub fn new(
        name: String,
        capabilities: ServerCapabilities,
        cx: AsyncAppContext,
    ) -> (LanguageServer, FakeLanguageServer) {
        let (stdin_writer, stdin_reader) = async_pipe::pipe();
        let (stdout_writer, stdout_reader) = async_pipe::pipe();
        let (notifications_tx, notifications_rx) = channel::unbounded();

        let server = LanguageServer::new_internal(
            LanguageServerId(0),
            stdin_writer,
            stdout_reader,
            None::<async_pipe::PipeReader>,
            Arc::new(Mutex::new(None)),
            None,
            Path::new("/"),
            None,
            cx.clone(),
            |_| {},
        );
        let fake = FakeLanguageServer {
            server: Arc::new(LanguageServer::new_internal(
                LanguageServerId(0),
                stdout_writer,
                stdin_reader,
                None::<async_pipe::PipeReader>,
                Arc::new(Mutex::new(None)),
                None,
                Path::new("/"),
                None,
                cx,
                move |msg| {
                    notifications_tx
                        .try_send((
                            msg.method.to_string(),
                            msg.params
                                .map(|raw_value| raw_value.get())
                                .unwrap_or("null")
                                .to_string(),
                        ))
                        .ok();
                },
            )),
            notifications_rx,
        };
        fake.handle_request::<request::Initialize, _, _>({
            let capabilities = capabilities;
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
                    })
                }
            }
        });

        (server, fake)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl LanguageServer {
    pub fn full_capabilities() -> ServerCapabilities {
        ServerCapabilities {
            document_highlight_provider: Some(OneOf::Left(true)),
            code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            definition_provider: Some(OneOf::Left(true)),
            type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
            ..Default::default()
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    /// See [`LanguageServer::notify`].
    pub fn notify<T: notification::Notification>(&self, params: T::Params) {
        self.server.notify::<T>(params).ok();
    }

    /// See [`LanguageServer::request`].
    pub async fn request<T>(&self, params: T::Params) -> Result<T::Result>
    where
        T: request::Request,
        T::Result: 'static + Send,
    {
        self.server.executor.start_waiting();
        self.server.request::<T>(params).await
    }

    /// Attempts [`Self::try_receive_notification`], unwrapping if it has not received the specified type yet.
    pub async fn receive_notification<T: notification::Notification>(&mut self) -> T::Params {
        self.server.executor.start_waiting();
        self.try_receive_notification::<T>().await.unwrap()
    }

    /// Consumes the notification channel until it finds a notification for the specified type.
    pub async fn try_receive_notification<T: notification::Notification>(
        &mut self,
    ) -> Option<T::Params> {
        use futures::StreamExt as _;

        loop {
            let (method, params) = self.notifications_rx.next().await?;
            if method == T::METHOD {
                return Some(serde_json::from_str::<T::Params>(&params).unwrap());
            } else {
                log::info!("skipping message in fake language server {:?}", params);
            }
        }
    }

    /// Registers a handler for a specific kind of request. Removes any existing handler for specified request type.
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
                let executor = cx.background_executor().clone();
                async move {
                    executor.simulate_random_delay().await;
                    let result = result.await;
                    responded_tx.unbounded_send(()).ok();
                    result
                }
            })
            .detach();
        responded_rx
    }

    /// Registers a handler for a specific kind of notification. Removes any existing handler for specified notification type.
    pub fn handle_notification<T, F>(
        &self,
        mut handler: F,
    ) -> futures::channel::mpsc::UnboundedReceiver<()>
    where
        T: 'static + notification::Notification,
        T::Params: 'static + Send,
        F: 'static + Send + FnMut(T::Params, gpui::AsyncAppContext),
    {
        let (handled_tx, handled_rx) = futures::channel::mpsc::unbounded();
        self.server.remove_notification_handler::<T>();
        self.server
            .on_notification::<T, _>(move |params, cx| {
                handler(params, cx.clone());
                handled_tx.unbounded_send(()).ok();
            })
            .detach();
        handled_rx
    }

    /// Removes any existing handler for specified notification type.
    pub fn remove_request_handler<T>(&mut self)
    where
        T: 'static + request::Request,
    {
        self.server.remove_request_handler::<T>();
    }

    /// Simulate that the server has started work and notifies about its progress with the specified token.
    pub async fn start_progress(&self, token: impl Into<String>) {
        let token = token.into();
        self.request::<request::WorkDoneProgressCreate>(WorkDoneProgressCreateParams {
            token: NumberOrString::String(token.clone()),
        })
        .await
        .unwrap();
        self.notify::<notification::Progress>(ProgressParams {
            token: NumberOrString::String(token),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(Default::default())),
        });
    }

    /// Simulate that the server has completed work and notifies about that with the specified token.
    pub fn end_progress(&self, token: impl Into<String>) {
        self.notify::<notification::Progress>(ProgressParams {
            token: NumberOrString::String(token.into()),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(Default::default())),
        });
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
        cx.update(|cx| {
            release_channel::init("0.0.0", cx);
        });
        let (server, mut fake) =
            FakeLanguageServer::new("the-lsp".to_string(), Default::default(), cx.to_async());

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

        let server = cx.update(|cx| server.initialize(None, cx)).await.unwrap();
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
