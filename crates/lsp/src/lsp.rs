mod input_handler;

pub use lsp_types::request::*;
pub use lsp_types::*;

use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use futures::{AsyncRead, AsyncWrite, Future, FutureExt, channel::oneshot, io::BufWriter, select};
use gpui::{App, AppContext as _, AsyncApp, BackgroundExecutor, SharedString, Task};
use notification::DidChangeWorkspaceFolders;
use parking_lot::{Mutex, RwLock};
use postage::{barrier, prelude::Stream};
use schemars::{
    JsonSchema,
    r#gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json, value::RawValue};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Child,
};

use std::{
    collections::BTreeSet,
    ffi::{OsStr, OsString},
    fmt,
    io::Write,
    ops::{Deref, DerefMut},
    path::PathBuf,
    pin::Pin,
    sync::{
        Arc, Weak,
        atomic::{AtomicI32, Ordering::SeqCst},
    },
    task::Poll,
    time::{Duration, Instant},
};
use std::{path::Path, process::Stdio};
use util::{ResultExt, TryFutureExt};

const JSON_RPC_VERSION: &str = "2.0";
const CONTENT_LEN_HEADER: &str = "Content-Length: ";

const LSP_REQUEST_TIMEOUT: Duration = Duration::from_secs(60 * 2);
const SERVER_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

type NotificationHandler = Box<dyn Send + FnMut(Option<RequestId>, Value, &mut AsyncApp)>;
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
    pub env: Option<HashMap<String, String>>,
}

/// Configures the search (and installation) of language servers.
#[derive(Debug, Clone, Deserialize)]
pub struct LanguageServerBinaryOptions {
    /// Whether the adapter should look at the users system
    pub allow_path_lookup: bool,
    /// Whether the adapter should download its own version
    pub allow_binary_download: bool,
}

/// A running language server process.
pub struct LanguageServer {
    server_id: LanguageServerId,
    next_id: AtomicI32,
    outbound_tx: channel::Sender<String>,
    name: LanguageServerName,
    process_name: Arc<str>,
    binary: LanguageServerBinary,
    capabilities: RwLock<ServerCapabilities>,
    /// Configuration sent to the server, stored for display in the language server logs
    /// buffer. This is represented as the message sent to the LSP in order to avoid cloning it (can
    /// be large in cases like sending schemas to the json server).
    configuration: Arc<DidChangeConfigurationParams>,
    code_action_kinds: Option<Vec<CodeActionKind>>,
    notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
    executor: BackgroundExecutor,
    #[allow(clippy::type_complexity)]
    io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    server: Arc<Mutex<Option<Child>>>,
    workspace_folders: Arc<Mutex<BTreeSet<Url>>>,
    root_uri: Url,
}

/// Identifies a running language server.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LanguageServerId(pub usize);

impl LanguageServerId {
    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(self) -> u64 {
        self.0 as u64
    }
}

/// A name of a language server.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LanguageServerName(pub SharedString);

impl std::fmt::Display for LanguageServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for LanguageServerName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<OsStr> for LanguageServerName {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref().as_ref()
    }
}

impl JsonSchema for LanguageServerName {
    fn schema_name() -> String {
        "LanguageServerName".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            ..Default::default()
        }
        .into()
    }
}

impl LanguageServerName {
    pub const fn new_static(s: &'static str) -> Self {
        Self(SharedString::new_static(s))
    }

    pub fn from_proto(s: String) -> Self {
        Self(s.into())
    }
}

impl<'a> From<&'a str> for LanguageServerName {
    fn from(str: &'a str) -> LanguageServerName {
        LanguageServerName(str.to_string().into())
    }
}

/// Handle to a language server RPC activity subscription.
pub enum Subscription {
    Notification {
        method: &'static str,
        notification_handlers: Option<Arc<Mutex<HashMap<&'static str, NotificationHandler>>>>,
    },
    Io {
        id: i32,
        io_handlers: Option<Weak<Mutex<HashMap<i32, IoHandler>>>>,
    },
}

/// Language server protocol RPC request message ID.
///
/// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Int(i32),
    Str(String),
}

/// Language server protocol RPC request message.
///
/// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
#[derive(Serialize, Deserialize)]
pub struct Request<'a, T> {
    jsonrpc: &'static str,
    id: RequestId,
    method: &'a str,
    params: T,
}

/// Language server protocol RPC request response message before it is deserialized into a concrete type.
#[derive(Serialize, Deserialize)]
struct AnyResponse<'a> {
    jsonrpc: &'a str,
    id: RequestId,
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
    id: RequestId,
    #[serde(flatten)]
    value: LspResult<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum LspResult<T> {
    #[serde(rename = "result")]
    Ok(Option<T>),
    Error(Option<Error>),
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
struct AnyNotification {
    #[serde(default)]
    id: Option<RequestId>,
    method: String,
    #[serde(default)]
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
}

pub trait LspRequestFuture<O>: Future<Output = O> {
    fn id(&self) -> i32;
}

struct LspRequest<F> {
    id: i32,
    request: F,
}

impl<F> LspRequest<F> {
    pub fn new(id: i32, request: F) -> Self {
        Self { id, request }
    }
}

impl<F: Future> Future for LspRequest<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // SAFETY: This is standard pin projection, we're pinned so our fields must be pinned.
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().request) };
        inner.poll(cx)
    }
}

impl<F: Future> LspRequestFuture<F::Output> for LspRequest<F> {
    fn id(&self) -> i32 {
        self.id
    }
}

/// Combined capabilities of the server and the adapter.
#[derive(Debug)]
pub struct AdapterServerCapabilities {
    // Reported capabilities by the server
    pub server_capabilities: ServerCapabilities,
    // List of code actions supported by the LspAdapter matching the server
    pub code_action_kinds: Option<Vec<CodeActionKind>>,
}

impl LanguageServer {
    /// Starts a language server process.
    pub fn new(
        stderr_capture: Arc<Mutex<Option<String>>>,
        server_id: LanguageServerId,
        server_name: LanguageServerName,
        binary: LanguageServerBinary,
        root_path: &Path,
        code_action_kinds: Option<Vec<CodeActionKind>>,
        workspace_folders: Arc<Mutex<BTreeSet<Url>>>,
        cx: &mut AsyncApp,
    ) -> Result<Self> {
        let working_dir = if root_path.is_dir() {
            root_path
        } else {
            root_path.parent().unwrap_or_else(|| Path::new("/"))
        };

        log::info!(
            "starting language server process. binary path: {:?}, working directory: {:?}, args: {:?}",
            binary.path,
            working_dir,
            &binary.arguments
        );

        let mut server = util::command::new_smol_command(&binary.path)
            .current_dir(working_dir)
            .args(&binary.arguments)
            .envs(binary.env.clone().unwrap_or_default())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .with_context(|| {
                format!(
                    "failed to spawn command. path: {:?}, working directory: {:?}, args: {:?}",
                    binary.path, working_dir, &binary.arguments
                )
            })?;

        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let stderr = server.stderr.take().unwrap();
        let root_uri = Url::from_file_path(&working_dir)
            .map_err(|_| anyhow!("{} is not a valid URI", working_dir.display()))?;
        let server = Self::new_internal(
            server_id,
            server_name,
            stdin,
            stdout,
            Some(stderr),
            stderr_capture,
            Some(server),
            code_action_kinds,
            binary,
            root_uri,
            workspace_folders,
            cx,
            move |notification| {
                log::info!(
                    "Language server with id {} sent unhandled notification {}:\n{}",
                    server_id,
                    notification.method,
                    serde_json::to_string_pretty(&notification.params).unwrap(),
                );
            },
        );

        Ok(server)
    }

    fn new_internal<Stdin, Stdout, Stderr, F>(
        server_id: LanguageServerId,
        server_name: LanguageServerName,
        stdin: Stdin,
        stdout: Stdout,
        stderr: Option<Stderr>,
        stderr_capture: Arc<Mutex<Option<String>>>,
        server: Option<Child>,
        code_action_kinds: Option<Vec<CodeActionKind>>,
        binary: LanguageServerBinary,
        root_uri: Url,
        workspace_folders: Arc<Mutex<BTreeSet<Url>>>,
        cx: &mut AsyncApp,
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
            async move |cx| {
                Self::handle_input(
                    stdout,
                    on_unhandled_notification,
                    notification_handlers,
                    response_handlers,
                    io_handlers,
                    cx,
                )
                .log_err()
                .await
            }
        });
        let stderr_input_task = stderr
            .map(|stderr| {
                let io_handlers = io_handlers.clone();
                let stderr_captures = stderr_capture.clone();
                cx.spawn(async move |_| {
                    Self::handle_stderr(stderr, io_handlers, stderr_captures)
                        .log_err()
                        .await
                })
            })
            .unwrap_or_else(|| Task::ready(None));
        let input_task = cx.spawn(async move |_| {
            let (stdout, stderr) = futures::join!(stdout_input_task, stderr_input_task);
            stdout.or(stderr)
        });
        let output_task = cx.background_spawn({
            Self::handle_output(
                stdin,
                outbound_rx,
                output_done_tx,
                response_handlers.clone(),
                io_handlers.clone(),
            )
            .log_err()
        });

        let configuration = DidChangeConfigurationParams {
            settings: Value::Null,
        }
        .into();

        Self {
            server_id,
            notification_handlers,
            response_handlers,
            io_handlers,
            name: server_name,
            process_name: binary
                .path
                .file_name()
                .map(|name| Arc::from(name.to_string_lossy()))
                .unwrap_or_default(),
            binary,
            capabilities: Default::default(),
            configuration,
            code_action_kinds,
            next_id: Default::default(),
            outbound_tx,
            executor: cx.background_executor().clone(),
            io_tasks: Mutex::new(Some((input_task, output_task))),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            server: Arc::new(Mutex::new(server)),
            workspace_folders,
            root_uri,
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
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
        F: FnMut(AnyNotification) + 'static + Send,
    {
        use smol::stream::StreamExt;
        let stdout = BufReader::new(stdout);
        let _clear_response_handlers = util::defer({
            let response_handlers = response_handlers.clone();
            move || {
                response_handlers.lock().take();
            }
        });
        let mut input_handler = input_handler::LspStdoutHandler::new(
            stdout,
            response_handlers,
            io_handlers,
            cx.background_executor().clone(),
        );

        while let Some(msg) = input_handler.notifications_channel.next().await {
            {
                let mut notification_handlers = notification_handlers.lock();
                if let Some(handler) = notification_handlers.get_mut(msg.method.as_str()) {
                    handler(msg.id, msg.params.unwrap_or(Value::Null), cx);
                } else {
                    drop(notification_handlers);
                    on_unhandled_notification(msg);
                }
            }

            // Don't starve the main thread when receiving lots of notifications at once.
            smol::future::yield_now().await;
        }
        input_handler.loop_handle.await
    }

    async fn handle_stderr<Stderr>(
        stderr: Stderr,
        io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
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

            if let Ok(message) = std::str::from_utf8(&buffer) {
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
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
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

    pub fn default_initialize_params(&self, cx: &App) -> InitializeParams {
        let workspace_folders = self
            .workspace_folders
            .lock()
            .iter()
            .cloned()
            .map(|uri| WorkspaceFolder {
                name: Default::default(),
                uri,
            })
            .collect::<Vec<_>>();
        #[allow(deprecated)]
        InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: Some(self.root_uri.clone()),
            initialization_options: None,
            capabilities: ClientCapabilities {
                general: Some(GeneralClientCapabilities {
                    position_encodings: Some(vec![PositionEncodingKind::UTF16]),
                    ..Default::default()
                }),
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
                    code_lens: Some(CodeLensWorkspaceClientCapabilities {
                        refresh_support: Some(true),
                    }),
                    workspace_edit: Some(WorkspaceEditClientCapabilities {
                        resource_operations: Some(vec![
                            ResourceOperationKind::Create,
                            ResourceOperationKind::Rename,
                            ResourceOperationKind::Delete,
                        ]),
                        document_changes: Some(true),
                        snippet_edit_support: Some(true),
                        ..WorkspaceEditClientCapabilities::default()
                    }),
                    file_operations: Some(WorkspaceFileOperationsClientCapabilities {
                        dynamic_registration: Some(false),
                        did_rename: Some(true),
                        will_rename: Some(true),
                        ..Default::default()
                    }),
                    apply_edit: Some(true),
                    execute_command: Some(ExecuteCommandClientCapabilities {
                        dynamic_registration: Some(false),
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
                            properties: vec![
                                "kind".to_string(),
                                "diagnostics".to_string(),
                                "isPreferred".to_string(),
                                "disabled".to_string(),
                                "edit".to_string(),
                                "command".to_string(),
                            ],
                        }),
                        ..Default::default()
                    }),
                    completion: Some(CompletionClientCapabilities {
                        completion_item: Some(CompletionItemCapability {
                            snippet_support: Some(true),
                            resolve_support: Some(CompletionItemCapabilityResolveSupport {
                                properties: vec![
                                    "additionalTextEdits".to_string(),
                                    "command".to_string(),
                                    "documentation".to_string(),
                                    // NB: Do not have this resolved, otherwise Zed becomes slow to complete things
                                    // "textEdit".to_string(),
                                ],
                            }),
                            insert_replace_support: Some(true),
                            label_details_support: Some(true),
                            insert_text_mode_support: Some(InsertTextModeSupport {
                                value_set: vec![
                                    InsertTextMode::AS_IS,
                                    InsertTextMode::ADJUST_INDENTATION,
                                ],
                            }),
                            ..Default::default()
                        }),
                        insert_text_mode: Some(InsertTextMode::ADJUST_INDENTATION),
                        completion_list: Some(CompletionListCapability {
                            item_defaults: Some(vec![
                                "commitCharacters".to_owned(),
                                "editRange".to_owned(),
                                "insertTextMode".to_owned(),
                                "insertTextFormat".to_owned(),
                                "data".to_owned(),
                            ]),
                        }),
                        context_support: Some(true),
                        ..Default::default()
                    }),
                    rename: Some(RenameClientCapabilities {
                        prepare_support: Some(true),
                        prepare_support_default_behavior: Some(
                            PrepareSupportDefaultBehavior::IDENTIFIER,
                        ),
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
                        dynamic_registration: Some(true),
                    }),
                    range_formatting: Some(DynamicRegistrationClientCapabilities {
                        dynamic_registration: Some(true),
                    }),
                    on_type_formatting: Some(DynamicRegistrationClientCapabilities {
                        dynamic_registration: Some(true),
                    }),
                    signature_help: Some(SignatureHelpClientCapabilities {
                        signature_information: Some(SignatureInformationSettings {
                            documentation_format: Some(vec![
                                MarkupKind::Markdown,
                                MarkupKind::PlainText,
                            ]),
                            parameter_information: Some(ParameterInformationSettings {
                                label_offset_support: Some(true),
                            }),
                            active_parameter_support: Some(true),
                        }),
                        ..SignatureHelpClientCapabilities::default()
                    }),
                    synchronization: Some(TextDocumentSyncClientCapabilities {
                        did_save: Some(true),
                        ..TextDocumentSyncClientCapabilities::default()
                    }),
                    code_lens: Some(CodeLensClientCapabilities {
                        dynamic_registration: Some(false),
                    }),
                    document_symbol: Some(DocumentSymbolClientCapabilities {
                        hierarchical_document_symbol_support: Some(true),
                        ..DocumentSymbolClientCapabilities::default()
                    }),
                    ..TextDocumentClientCapabilities::default()
                }),
                experimental: Some(json!({
                    "serverStatusNotification": true,
                    "localDocs": true,
                })),
                window: Some(WindowClientCapabilities {
                    work_done_progress: Some(true),
                    show_message: Some(ShowMessageRequestClientCapabilities {
                        message_action_item: None,
                    }),
                    ..Default::default()
                }),
            },
            trace: None,
            workspace_folders: Some(workspace_folders),
            client_info: release_channel::ReleaseChannel::try_global(cx).map(|release_channel| {
                ClientInfo {
                    name: release_channel.display_name().to_string(),
                    version: Some(release_channel::AppVersion::global(cx).to_string()),
                }
            }),
            locale: None,

            ..Default::default()
        }
    }

    /// Initializes a language server by sending the `Initialize` request.
    /// Note that `options` is used directly to construct [`InitializeParams`], which is why it is owned.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize)
    pub fn initialize(
        mut self,
        params: InitializeParams,
        configuration: Arc<DidChangeConfigurationParams>,
        cx: &App,
    ) -> Task<Result<Arc<Self>>> {
        cx.spawn(async move |_| {
            let response = self.request::<request::Initialize>(params).await?;
            if let Some(info) = response.server_info {
                self.process_name = info.name.into();
            }
            self.capabilities = RwLock::new(response.capabilities);
            self.configuration = configuration;

            self.notify::<notification::Initialized>(&InitializedParams {})?;
            Ok(Arc::new(self))
        })
    }

    /// Sends a shutdown request to the language server process and prepares the [`LanguageServer`] to be dropped.
    pub fn shutdown(&self) -> Option<impl 'static + Send + Future<Output = Option<()>> + use<>> {
        if let Some(tasks) = self.io_tasks.lock().take() {
            let response_handlers = self.response_handlers.clone();
            let next_id = AtomicI32::new(self.next_id.load(SeqCst));
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
            let exit = Self::notify_internal::<notification::Exit>(&outbound_tx, &());
            outbound_tx.close();

            let server = self.server.clone();
            let name = self.name.clone();
            let mut timer = self.executor.timer(SERVER_SHUTDOWN_TIMEOUT).fuse();
            Some(
                async move {
                    log::debug!("language server shutdown started");

                    select! {
                        request_result = shutdown_request.fuse() => {
                            request_result?;
                        }

                        _ = timer => {
                            log::info!("timeout waiting for language server {name} to shutdown");
                        },
                    }

                    response_handlers.lock().take();
                    exit?;
                    output_done.recv().await;
                    server.lock().take().map(|mut child| child.kill());
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
        F: 'static + Send + FnMut(T::Params, &mut AsyncApp),
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
        F: 'static + FnMut(T::Params, &mut AsyncApp) -> Fut + Send,
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
        F: 'static + FnMut(Params, &mut AsyncApp) + Send,
        Params: DeserializeOwned,
    {
        let prev_handler = self.notification_handlers.lock().insert(
            method,
            Box::new(move |_, params, cx| {
                if let Some(params) = serde_json::from_value(params).log_err() {
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
        F: 'static + FnMut(Params, &mut AsyncApp) -> Fut + Send,
        Fut: 'static + Future<Output = Result<Res>>,
        Params: DeserializeOwned + Send + 'static,
        Res: Serialize,
    {
        let outbound_tx = self.outbound_tx.clone();
        let prev_handler = self.notification_handlers.lock().insert(
            method,
            Box::new(move |id, params, cx| {
                if let Some(id) = id {
                    match serde_json::from_value(params) {
                        Ok(params) => {
                            let response = f(params, cx);
                            cx.foreground_executor()
                                .spawn({
                                    let outbound_tx = outbound_tx.clone();
                                    async move {
                                        let response = match response.await {
                                            Ok(result) => Response {
                                                jsonrpc: JSON_RPC_VERSION,
                                                id,
                                                value: LspResult::Ok(Some(result)),
                                            },
                                            Err(error) => Response {
                                                jsonrpc: JSON_RPC_VERSION,
                                                id,
                                                value: LspResult::Error(Some(Error {
                                                    message: error.to_string(),
                                                })),
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
                            log::error!("error deserializing {} request: {:?}", method, error);
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
    pub fn name(&self) -> LanguageServerName {
        self.name.clone()
    }

    pub fn process_name(&self) -> &str {
        &self.process_name
    }

    /// Get the reported capabilities of the running language server.
    pub fn capabilities(&self) -> ServerCapabilities {
        self.capabilities.read().clone()
    }

    /// Get the reported capabilities of the running language server and
    /// what we know on the client/adapter-side of its capabilities.
    pub fn adapter_server_capabilities(&self) -> AdapterServerCapabilities {
        AdapterServerCapabilities {
            server_capabilities: self.capabilities(),
            code_action_kinds: self.code_action_kinds(),
        }
    }

    pub fn update_capabilities(&self, update: impl FnOnce(&mut ServerCapabilities)) {
        update(self.capabilities.write().deref_mut());
    }

    pub fn configuration(&self) -> &Value {
        &self.configuration.settings
    }

    /// Get the id of the running language server.
    pub fn server_id(&self) -> LanguageServerId {
        self.server_id
    }

    /// Language server's binary information.
    pub fn binary(&self) -> &LanguageServerBinary {
        &self.binary
    }
    /// Sends a RPC request to the language server.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
    pub fn request<T: request::Request>(
        &self,
        params: T::Params,
    ) -> impl LspRequestFuture<Result<T::Result>> + use<T>
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
        next_id: &AtomicI32,
        response_handlers: &Mutex<Option<HashMap<RequestId, ResponseHandler>>>,
        outbound_tx: &channel::Sender<String>,
        executor: &BackgroundExecutor,
        params: T::Params,
    ) -> impl LspRequestFuture<Result<T::Result>> + use<T>
    where
        T::Result: 'static + Send,
    {
        let id = next_id.fetch_add(1, SeqCst);
        let message = serde_json::to_string(&Request {
            jsonrpc: JSON_RPC_VERSION,
            id: RequestId::Int(id),
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
                    RequestId::Int(id),
                    Box::new(move |result| {
                        executor
                            .spawn(async move {
                                let response = match result {
                                    Ok(response) => match serde_json::from_str(&response) {
                                        Ok(deserialized) => Ok(deserialized),
                                        Err(error) => {
                                            log::error!("failed to deserialize response from language server: {}. response from language server: {:?}", error, response);
                                            Err(error).context("failed to deserialize response")
                                        }
                                    }
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
        LspRequest::new(id, async move {
            handle_response?;
            send?;

            let cancel_on_drop = util::defer(move || {
                if let Some(outbound_tx) = outbound_tx.upgrade() {
                    Self::notify_internal::<notification::Cancel>(
                        &outbound_tx,
                        &CancelParams {
                            id: NumberOrString::Number(id),
                        },
                    )
                    .ok();
                }
            });

            let method = T::METHOD;
            select! {
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
        })
    }

    /// Sends a RPC notification to the language server.
    ///
    /// [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#notificationMessage)
    pub fn notify<T: notification::Notification>(&self, params: &T::Params) -> Result<()> {
        Self::notify_internal::<T>(&self.outbound_tx, params)
    }

    fn notify_internal<T: notification::Notification>(
        outbound_tx: &channel::Sender<String>,
        params: &T::Params,
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

    /// Add new workspace folder to the list.
    pub fn add_workspace_folder(&self, uri: Url) {
        if self
            .capabilities()
            .workspace
            .and_then(|ws| {
                ws.workspace_folders.and_then(|folders| {
                    folders
                        .change_notifications
                        .map(|caps| matches!(caps, OneOf::Left(false)))
                })
            })
            .unwrap_or(true)
        {
            return;
        }

        let is_new_folder = self.workspace_folders.lock().insert(uri.clone());
        if is_new_folder {
            let params = DidChangeWorkspaceFoldersParams {
                event: WorkspaceFoldersChangeEvent {
                    added: vec![WorkspaceFolder {
                        uri,
                        name: String::default(),
                    }],
                    removed: vec![],
                },
            };
            self.notify::<DidChangeWorkspaceFolders>(&params).ok();
        }
    }
    /// Add new workspace folder to the list.
    pub fn remove_workspace_folder(&self, uri: Url) {
        if self
            .capabilities()
            .workspace
            .and_then(|ws| {
                ws.workspace_folders.and_then(|folders| {
                    folders
                        .change_notifications
                        .map(|caps| !matches!(caps, OneOf::Left(false)))
                })
            })
            .unwrap_or(true)
        {
            return;
        }
        let was_removed = self.workspace_folders.lock().remove(&uri);
        if was_removed {
            let params = DidChangeWorkspaceFoldersParams {
                event: WorkspaceFoldersChangeEvent {
                    added: vec![],
                    removed: vec![WorkspaceFolder {
                        uri,
                        name: String::default(),
                    }],
                },
            };
            self.notify::<DidChangeWorkspaceFolders>(&params).ok();
        }
    }
    pub fn set_workspace_folders(&self, folders: BTreeSet<Url>) {
        let mut workspace_folders = self.workspace_folders.lock();

        let old_workspace_folders = std::mem::take(&mut *workspace_folders);
        let added: Vec<_> = folders
            .difference(&old_workspace_folders)
            .map(|uri| WorkspaceFolder {
                uri: uri.clone(),
                name: String::default(),
            })
            .collect();

        let removed: Vec<_> = old_workspace_folders
            .difference(&folders)
            .map(|uri| WorkspaceFolder {
                uri: uri.clone(),
                name: String::default(),
            })
            .collect();
        *workspace_folders = folders;
        let should_notify = !added.is_empty() || !removed.is_empty();
        if should_notify {
            drop(workspace_folders);
            let params = DidChangeWorkspaceFoldersParams {
                event: WorkspaceFoldersChangeEvent { added, removed },
            };
            self.notify::<DidChangeWorkspaceFolders>(&params).ok();
        }
    }

    pub fn workspace_folders(&self) -> impl Deref<Target = BTreeSet<Url>> + '_ {
        self.workspace_folders.lock()
    }

    pub fn register_buffer(
        &self,
        uri: Url,
        language_id: String,
        version: i32,
        initial_text: String,
    ) {
        self.notify::<notification::DidOpenTextDocument>(&DidOpenTextDocumentParams {
            text_document: TextDocumentItem::new(uri, language_id, version, initial_text),
        })
        .ok();
    }

    pub fn unregister_buffer(&self, uri: Url) {
        self.notify::<notification::DidCloseTextDocument>(&DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier::new(uri),
        })
        .ok();
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
    pub binary: LanguageServerBinary,
    pub server: Arc<LanguageServer>,
    notifications_rx: channel::Receiver<(String, String)>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    /// Construct a fake language server.
    pub fn new(
        server_id: LanguageServerId,
        binary: LanguageServerBinary,
        name: String,
        capabilities: ServerCapabilities,
        cx: &mut AsyncApp,
    ) -> (LanguageServer, FakeLanguageServer) {
        let (stdin_writer, stdin_reader) = async_pipe::pipe();
        let (stdout_writer, stdout_reader) = async_pipe::pipe();
        let (notifications_tx, notifications_rx) = channel::unbounded();

        let server_name = LanguageServerName(name.clone().into());
        let process_name = Arc::from(name.as_str());
        let root = Self::root_path();
        let workspace_folders: Arc<Mutex<BTreeSet<Url>>> = Default::default();
        let mut server = LanguageServer::new_internal(
            server_id,
            server_name.clone(),
            stdin_writer,
            stdout_reader,
            None::<async_pipe::PipeReader>,
            Arc::new(Mutex::new(None)),
            None,
            None,
            binary.clone(),
            root,
            workspace_folders.clone(),
            cx,
            |_| {},
        );
        server.process_name = process_name;
        let fake = FakeLanguageServer {
            binary: binary.clone(),
            server: Arc::new({
                let mut server = LanguageServer::new_internal(
                    server_id,
                    server_name,
                    stdout_writer,
                    stdin_reader,
                    None::<async_pipe::PipeReader>,
                    Arc::new(Mutex::new(None)),
                    None,
                    None,
                    binary,
                    Self::root_path(),
                    workspace_folders,
                    cx,
                    move |msg| {
                        notifications_tx
                            .try_send((
                                msg.method.to_string(),
                                msg.params.unwrap_or(Value::Null).to_string(),
                            ))
                            .ok();
                    },
                );
                server.process_name = name.as_str().into();
                server
            }),
            notifications_rx,
        };
        fake.set_request_handler::<request::Initialize, _, _>({
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
    #[cfg(target_os = "windows")]
    fn root_path() -> Url {
        Url::from_file_path("C:/").unwrap()
    }

    #[cfg(not(target_os = "windows"))]
    fn root_path() -> Url {
        Url::from_file_path("/").unwrap()
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
            workspace_symbol_provider: Some(OneOf::Left(true)),
            implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
            type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
            ..Default::default()
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    /// See [`LanguageServer::notify`].
    pub fn notify<T: notification::Notification>(&self, params: &T::Params) {
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
        loop {
            let (method, params) = self.notifications_rx.recv().await.ok()?;
            if method == T::METHOD {
                return Some(serde_json::from_str::<T::Params>(&params).unwrap());
            } else {
                log::info!("skipping message in fake language server {:?}", params);
            }
        }
    }

    /// Registers a handler for a specific kind of request. Removes any existing handler for specified request type.
    pub fn set_request_handler<T, F, Fut>(
        &self,
        mut handler: F,
    ) -> futures::channel::mpsc::UnboundedReceiver<()>
    where
        T: 'static + request::Request,
        T::Params: 'static + Send,
        F: 'static + Send + FnMut(T::Params, gpui::AsyncApp) -> Fut,
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
        F: 'static + Send + FnMut(T::Params, gpui::AsyncApp),
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
        self.start_progress_with(token, Default::default()).await
    }

    pub async fn start_progress_with(
        &self,
        token: impl Into<String>,
        progress: WorkDoneProgressBegin,
    ) {
        let token = token.into();
        self.request::<request::WorkDoneProgressCreate>(WorkDoneProgressCreateParams {
            token: NumberOrString::String(token.clone()),
        })
        .await
        .unwrap();
        self.notify::<notification::Progress>(&ProgressParams {
            token: NumberOrString::String(token),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(progress)),
        });
    }

    /// Simulate that the server has completed work and notifies about that with the specified token.
    pub fn end_progress(&self, token: impl Into<String>) {
        self.notify::<notification::Progress>(&ProgressParams {
            token: NumberOrString::String(token.into()),
            value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(Default::default())),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{SemanticVersion, TestAppContext};
    use std::str::FromStr;

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test]
    async fn test_fake(cx: &mut TestAppContext) {
        cx.update(|cx| {
            release_channel::init(SemanticVersion::default(), cx);
        });
        let (server, mut fake) = FakeLanguageServer::new(
            LanguageServerId(0),
            LanguageServerBinary {
                path: "path/to/language-server".into(),
                arguments: vec![],
                env: None,
            },
            "the-lsp".to_string(),
            Default::default(),
            &mut cx.to_async(),
        );

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

        let server = cx
            .update(|cx| {
                let params = server.default_initialize_params(cx);
                let configuration = DidChangeConfigurationParams {
                    settings: Default::default(),
                };
                server.initialize(params, configuration.into(), cx)
            })
            .await
            .unwrap();
        server
            .notify::<notification::DidOpenTextDocument>(&DidOpenTextDocumentParams {
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

        fake.notify::<notification::ShowMessage>(&ShowMessageParams {
            typ: MessageType::ERROR,
            message: "ok".to_string(),
        });
        fake.notify::<notification::PublishDiagnostics>(&PublishDiagnosticsParams {
            uri: Url::from_str("file://b/c").unwrap(),
            version: Some(5),
            diagnostics: vec![],
        });
        assert_eq!(message_rx.recv().await.unwrap().message, "ok");
        assert_eq!(
            diagnostics_rx.recv().await.unwrap().uri.as_str(),
            "file://b/c"
        );

        fake.set_request_handler::<request::Shutdown, _, _>(|_, _| async move { Ok(()) });

        drop(server);
        fake.receive_notification::<notification::Exit>().await;
    }

    #[gpui::test]
    fn test_deserialize_string_digit_id() {
        let json = r#"{"jsonrpc":"2.0","id":"2","method":"workspace/configuration","params":{"items":[{"scopeUri":"file:///Users/mph/Devel/personal/hello-scala/","section":"metals"}]}}"#;
        let notification = serde_json::from_str::<AnyNotification>(json)
            .expect("message with string id should be parsed");
        let expected_id = RequestId::Str("2".to_string());
        assert_eq!(notification.id, Some(expected_id));
    }

    #[gpui::test]
    fn test_deserialize_string_id() {
        let json = r#"{"jsonrpc":"2.0","id":"anythingAtAll","method":"workspace/configuration","params":{"items":[{"scopeUri":"file:///Users/mph/Devel/personal/hello-scala/","section":"metals"}]}}"#;
        let notification = serde_json::from_str::<AnyNotification>(json)
            .expect("message with string id should be parsed");
        let expected_id = RequestId::Str("anythingAtAll".to_string());
        assert_eq!(notification.id, Some(expected_id));
    }

    #[gpui::test]
    fn test_deserialize_int_id() {
        let json = r#"{"jsonrpc":"2.0","id":2,"method":"workspace/configuration","params":{"items":[{"scopeUri":"file:///Users/mph/Devel/personal/hello-scala/","section":"metals"}]}}"#;
        let notification = serde_json::from_str::<AnyNotification>(json)
            .expect("message with string id should be parsed");
        let expected_id = RequestId::Int(2);
        assert_eq!(notification.id, Some(expected_id));
    }

    #[test]
    fn test_serialize_has_no_nulls() {
        // Ensure we're not setting both result and error variants. (ticket #10595)
        let no_tag = Response::<u32> {
            jsonrpc: "",
            id: RequestId::Int(0),
            value: LspResult::Ok(None),
        };
        assert_eq!(
            serde_json::to_string(&no_tag).unwrap(),
            "{\"jsonrpc\":\"\",\"id\":0,\"result\":null}"
        );
        let no_tag = Response::<u32> {
            jsonrpc: "",
            id: RequestId::Int(0),
            value: LspResult::Error(None),
        };
        assert_eq!(
            serde_json::to_string(&no_tag).unwrap(),
            "{\"jsonrpc\":\"\",\"id\":0,\"error\":null}"
        );
    }
}
