use anyhow::{anyhow, Context, Result};
use futures::{io::BufWriter, AsyncRead, AsyncWrite};
use gpui::{executor, AppContext, Task};
use parking_lot::{Mutex, RwLock};
use postage::{barrier, oneshot, prelude::Stream, sink::Sink};
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

type NotificationHandler = Box<dyn Send + Sync + Fn(&str)>;
type ResponseHandler = Box<dyn Send + FnOnce(Result<&str, Error>)>;

pub struct LanguageServer {
    next_id: AtomicUsize,
    outbound_tx: channel::Sender<Vec<u8>>,
    notification_handlers: Arc<RwLock<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<HashMap<usize, ResponseHandler>>>,
    _input_task: Task<Option<()>>,
    _output_task: Task<Option<()>>,
    initialized: barrier::Receiver,
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

#[derive(Serialize, Deserialize)]
struct AnyResponse<'a> {
    id: usize,
    #[serde(default)]
    error: Option<Error>,
    #[serde(borrow)]
    result: &'a RawValue,
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
    pub fn rust(root_path: &Path, cx: &AppContext) -> Result<Arc<Self>> {
        const ZED_BUNDLE: Option<&'static str> = option_env!("ZED_BUNDLE");
        const ZED_TARGET: &'static str = env!("ZED_TARGET");

        let rust_analyzer_name = format!("rust-analyzer-{}", ZED_TARGET);
        if ZED_BUNDLE.map_or(Ok(false), |b| b.parse())? {
            let rust_analyzer_path = cx
                .platform()
                .path_for_resource(Some(&rust_analyzer_name), None)?;
            Self::new(root_path, &rust_analyzer_path, &[], cx.background())
        } else {
            Self::new(
                root_path,
                Path::new(&rust_analyzer_name),
                &[],
                cx.background(),
            )
        }
    }

    pub fn new(
        root_path: &Path,
        server_path: &Path,
        server_args: &[&str],
        background: &executor::Background,
    ) -> Result<Arc<Self>> {
        let mut server = Command::new(server_path)
            .args(server_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        Self::new_internal(root_path, stdin, stdout, background)
    }

    fn new_internal<Stdin, Stdout>(
        root_path: &Path,
        stdin: Stdin,
        stdout: Stdout,
        background: &executor::Background,
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
        let _input_task = background.spawn(
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

                        println!("{}", std::str::from_utf8(&buffer).unwrap());
                        if let Ok(AnyNotification { method, params }) =
                            serde_json::from_slice(&buffer)
                        {
                            if let Some(handler) = notification_handlers.read().get(method) {
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
                                } else {
                                    handler(Ok(result.get()));
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
        let _output_task = background.spawn(
            async move {
                let mut content_len_buffer = Vec::new();
                loop {
                    content_len_buffer.clear();

                    let message = outbound_rx.recv().await?;
                    println!("{}", std::str::from_utf8(&message).unwrap());
                    write!(content_len_buffer, "{}", message.len()).unwrap();
                    stdin.write_all(CONTENT_LEN_HEADER.as_bytes()).await?;
                    stdin.write_all(&content_len_buffer).await?;
                    stdin.write_all("\r\n\r\n".as_bytes()).await?;
                    stdin.write_all(&message).await?;
                    stdin.flush().await?;
                }
            }
            .log_err(),
        );

        let (initialized_tx, initialized_rx) = barrier::channel();
        let this = Arc::new(Self {
            notification_handlers,
            response_handlers,
            next_id: Default::default(),
            outbound_tx,
            _input_task,
            _output_task,
            initialized: initialized_rx,
        });

        let root_uri =
            lsp_types::Url::from_file_path(root_path).map_err(|_| anyhow!("invalid root path"))?;
        background
            .spawn({
                let this = this.clone();
                async move {
                    this.init(root_uri).log_err().await;
                    drop(initialized_tx);
                }
            })
            .detach();

        Ok(this)
    }

    async fn init(self: Arc<Self>, root_uri: lsp_types::Url) -> Result<()> {
        #[allow(deprecated)]
        let params = lsp_types::InitializeParams {
            process_id: Default::default(),
            root_path: Default::default(),
            root_uri: Some(root_uri),
            initialization_options: Default::default(),
            capabilities: lsp_types::ClientCapabilities {
                experimental: Some(json!({
                    "serverStatusNotification": true,
                })),
                ..Default::default()
            },
            trace: Default::default(),
            workspace_folders: Default::default(),
            client_info: Default::default(),
            locale: Default::default(),
        };

        self.request_internal::<lsp_types::request::Initialize>(params)
            .await?;
        self.notify_internal::<lsp_types::notification::Initialized>(
            lsp_types::InitializedParams {},
        )
        .await?;
        Ok(())
    }

    pub fn on_notification<T, F>(&self, f: F) -> Subscription
    where
        T: lsp_types::notification::Notification,
        F: 'static + Send + Sync + Fn(T::Params),
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

    pub fn request<T: lsp_types::request::Request>(
        self: Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<T::Result>>
    where
        T::Result: 'static + Send,
    {
        let this = self.clone();
        async move {
            this.initialized.clone().recv().await;
            this.request_internal::<T>(params).await
        }
    }

    fn request_internal<T: lsp_types::request::Request>(
        self: &Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<T::Result>>
    where
        T::Result: 'static + Send,
    {
        let id = self.next_id.fetch_add(1, SeqCst);
        let message = serde_json::to_vec(&Request {
            jsonrpc: JSON_RPC_VERSION,
            id,
            method: T::METHOD,
            params,
        })
        .unwrap();
        let mut response_handlers = self.response_handlers.lock();
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

        let this = self.clone();
        async move {
            this.outbound_tx.send(message).await?;
            rx.recv().await.unwrap()
        }
    }

    pub fn notify<T: lsp_types::notification::Notification>(
        self: &Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        async move {
            this.initialized.clone().recv().await;
            this.notify_internal::<T>(params).await
        }
    }

    fn notify_internal<T: lsp_types::notification::Notification>(
        self: &Arc<Self>,
        params: T::Params,
    ) -> impl Future<Output = Result<()>> {
        let message = serde_json::to_vec(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: T::METHOD,
            params,
        })
        .unwrap();

        let this = self.clone();
        async move {
            this.outbound_tx.send(message).await?;
            Ok(())
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
    buffer: Vec<u8>,
    stdin: smol::io::BufReader<async_pipe::PipeReader>,
    stdout: smol::io::BufWriter<async_pipe::PipeWriter>,
}

#[cfg(any(test, feature = "test-support"))]
pub struct RequestId<T> {
    id: usize,
    _type: std::marker::PhantomData<T>,
}

#[cfg(any(test, feature = "test-support"))]
impl LanguageServer {
    pub async fn fake(executor: &executor::Background) -> (Arc<Self>, FakeLanguageServer) {
        let stdin = async_pipe::pipe();
        let stdout = async_pipe::pipe();
        let mut fake = FakeLanguageServer {
            stdin: smol::io::BufReader::new(stdin.1),
            stdout: smol::io::BufWriter::new(stdout.0),
            buffer: Vec::new(),
        };

        let server = Self::new_internal(Path::new("/"), stdin.0, stdout.1, executor).unwrap();

        let (init_id, _) = fake.receive_request::<request::Initialize>().await;
        fake.respond(init_id, InitializeResult::default()).await;
        fake.receive_notification::<notification::Initialized>()
            .await;

        (server, fake)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeLanguageServer {
    pub async fn notify<T: notification::Notification>(&mut self, params: T::Params) {
        let message = serde_json::to_vec(&Notification {
            jsonrpc: JSON_RPC_VERSION,
            method: T::METHOD,
            params,
        })
        .unwrap();
        self.send(message).await;
    }

    pub async fn respond<'a, T: request::Request>(
        &mut self,
        request_id: RequestId<T>,
        result: T::Result,
    ) {
        let result = serde_json::to_string(&result).unwrap();
        let message = serde_json::to_vec(&AnyResponse {
            id: request_id.id,
            error: None,
            result: &RawValue::from_string(result).unwrap(),
        })
        .unwrap();
        self.send(message).await;
    }

    pub async fn receive_request<T: request::Request>(&mut self) -> (RequestId<T>, T::Params) {
        self.receive().await;
        let request = serde_json::from_slice::<Request<T::Params>>(&self.buffer).unwrap();
        assert_eq!(request.method, T::METHOD);
        assert_eq!(request.jsonrpc, JSON_RPC_VERSION);
        (
            RequestId {
                id: request.id,
                _type: std::marker::PhantomData,
            },
            request.params,
        )
    }

    pub async fn receive_notification<T: notification::Notification>(&mut self) -> T::Params {
        self.receive().await;
        let notification = serde_json::from_slice::<Notification<T::Params>>(&self.buffer).unwrap();
        assert_eq!(notification.method, T::METHOD);
        notification.params
    }

    async fn send(&mut self, message: Vec<u8>) {
        self.stdout
            .write_all(CONTENT_LEN_HEADER.as_bytes())
            .await
            .unwrap();
        self.stdout
            .write_all((format!("{}", message.len())).as_bytes())
            .await
            .unwrap();
        self.stdout.write_all("\r\n\r\n".as_bytes()).await.unwrap();
        self.stdout.write_all(&message).await.unwrap();
        self.stdout.flush().await.unwrap();
    }

    async fn receive(&mut self) {
        self.buffer.clear();
        self.stdin
            .read_until(b'\n', &mut self.buffer)
            .await
            .unwrap();
        self.stdin
            .read_until(b'\n', &mut self.buffer)
            .await
            .unwrap();
        let message_len: usize = std::str::from_utf8(&self.buffer)
            .unwrap()
            .strip_prefix(CONTENT_LEN_HEADER)
            .unwrap()
            .trim_end()
            .parse()
            .unwrap();
        self.buffer.resize(message_len, 0);
        self.stdin.read_exact(&mut self.buffer).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use simplelog::SimpleLogger;
    use unindent::Unindent;
    use util::test::temp_tree;

    #[gpui::test]
    async fn test_basic(cx: TestAppContext) {
        let lib_source = r#"
            fn fun() {
                let hello = "world";
            }
        "#
        .unindent();
        let root_dir = temp_tree(json!({
            "Cargo.toml": r#"
                [package]
                name = "temp"
                version = "0.1.0"
                edition = "2018"
            "#.unindent(),
            "src": {
                "lib.rs": &lib_source
            }
        }));
        let lib_file_uri =
            lsp_types::Url::from_file_path(root_dir.path().join("src/lib.rs")).unwrap();

        let server = cx.read(|cx| LanguageServer::rust(root_dir.path(), cx).unwrap());
        server.next_idle_notification().await;

        server
            .notify::<lsp_types::notification::DidOpenTextDocument>(
                lsp_types::DidOpenTextDocumentParams {
                    text_document: lsp_types::TextDocumentItem::new(
                        lib_file_uri.clone(),
                        "rust".to_string(),
                        0,
                        lib_source,
                    ),
                },
            )
            .await
            .unwrap();

        let hover = server
            .request::<lsp_types::request::HoverRequest>(lsp_types::HoverParams {
                text_document_position_params: lsp_types::TextDocumentPositionParams {
                    text_document: lsp_types::TextDocumentIdentifier::new(lib_file_uri),
                    position: lsp_types::Position::new(1, 21),
                },
                work_done_progress_params: Default::default(),
            })
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            hover.contents,
            lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: "&str".to_string()
            })
        );
    }

    #[gpui::test]
    async fn test_fake(cx: TestAppContext) {
        SimpleLogger::init(log::LevelFilter::Info, Default::default()).unwrap();

        let (server, mut fake) = LanguageServer::fake(&cx.background()).await;

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
    }

    impl LanguageServer {
        async fn next_idle_notification(self: &Arc<Self>) {
            let (tx, rx) = channel::unbounded();
            let _subscription =
                self.on_notification::<ServerStatusNotification, _>(move |params| {
                    if params.quiescent {
                        tx.try_send(()).unwrap();
                    }
                });
            let _ = rx.recv().await;
        }
    }

    pub enum ServerStatusNotification {}

    impl lsp_types::notification::Notification for ServerStatusNotification {
        type Params = ServerStatusParams;
        const METHOD: &'static str = "experimental/serverStatus";
    }

    #[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
    pub struct ServerStatusParams {
        pub quiescent: bool,
    }
}
