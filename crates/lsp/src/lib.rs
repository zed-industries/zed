use anyhow::{anyhow, Context, Result};
use gpui::{executor, AppContext, Task};
use parking_lot::{Mutex, RwLock};
use postage::{barrier, oneshot, prelude::Stream, sink::Sink};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::RawValue};
use smol::{
    channel,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};
use std::{
    collections::HashMap,
    future::Future,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use std::{path::Path, process::Stdio};
use util::TryFutureExt;

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

#[derive(Serialize)]
struct Request<T> {
    jsonrpc: &'static str,
    id: usize,
    method: &'static str,
    params: T,
}

#[derive(Deserialize)]
struct Response<'a> {
    id: usize,
    #[serde(default)]
    error: Option<Error>,
    #[serde(borrow)]
    result: &'a RawValue,
}

#[derive(Serialize)]
struct OutboundNotification<T> {
    jsonrpc: &'static str,
    method: &'static str,
    params: T,
}

#[derive(Deserialize)]
struct InboundNotification<'a> {
    #[serde(borrow)]
    method: &'a str,
    #[serde(borrow)]
    params: &'a RawValue,
}

#[derive(Debug, Deserialize)]
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
            Self::new(root_path, &rust_analyzer_path, cx.background())
        } else {
            Self::new(root_path, Path::new(&rust_analyzer_name), cx.background())
        }
    }

    pub fn new(
        root_path: &Path,
        server_path: &Path,
        background: &executor::Background,
    ) -> Result<Arc<Self>> {
        let mut server = Command::new(server_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let mut stdin = server.stdin.take().unwrap();
        let mut stdout = BufReader::new(server.stdout.take().unwrap());
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

                        if let Ok(InboundNotification { method, params }) =
                            serde_json::from_slice(&buffer)
                        {
                            if let Some(handler) = notification_handlers.read().get(method) {
                                handler(params.get());
                            }
                        } else if let Ok(Response { id, error, result }) =
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
                    write!(content_len_buffer, "{}", message.len()).unwrap();
                    stdin.write_all(CONTENT_LEN_HEADER.as_bytes()).await?;
                    stdin.write_all(&content_len_buffer).await?;
                    stdin.write_all("\r\n\r\n".as_bytes()).await?;
                    stdin.write_all(&message).await?;
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
        self.request_internal::<lsp_types::request::Initialize>(lsp_types::InitializeParams {
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
        })
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
        let message = serde_json::to_vec(&OutboundNotification {
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

impl Drop for Subscription {
    fn drop(&mut self) {
        self.notification_handlers.write().remove(self.method);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use unindent::Unindent;
    use util::test::temp_tree;

    #[gpui::test]
    async fn test_basic(cx: TestAppContext) {
        let root_dir = temp_tree(json!({
            "Cargo.toml": r#"
                [package]
                name = "temp"
                version = "0.1.0"
                edition = "2018"
            "#.unindent(),
            "src": {
                "lib.rs": r#"
                    fn fun() {
                        let hello = "world";
                    }
                "#.unindent()
            }
        }));

        let server = cx.read(|cx| LanguageServer::rust(root_dir.path(), cx).unwrap());
        server.next_idle_notification().await;

        let hover = server
            .request::<lsp_types::request::HoverRequest>(lsp_types::HoverParams {
                text_document_position_params: lsp_types::TextDocumentPositionParams {
                    text_document: lsp_types::TextDocumentIdentifier::new(
                        lsp_types::Url::from_file_path(root_dir.path().join("src/lib.rs")).unwrap(),
                    ),
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
