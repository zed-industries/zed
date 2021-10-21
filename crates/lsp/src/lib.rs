use anyhow::{anyhow, Context, Result};
use gpui::{executor, Task};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
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

pub struct LanguageServer {
    next_id: AtomicUsize,
    outbound_tx: channel::Sender<Vec<u8>>,
    response_handlers: Arc<Mutex<HashMap<usize, ResponseHandler>>>,
    _input_task: Task<Option<()>>,
    _output_task: Task<Option<()>>,
}

type ResponseHandler = Box<dyn Send + FnOnce(Result<&str, Error>)>;

#[derive(Serialize)]
struct Request<T> {
    jsonrpc: &'static str,
    id: usize,
    method: &'static str,
    params: T,
}

#[derive(Deserialize)]
struct Error {
    message: String,
}

#[derive(Deserialize)]
struct Notification<'a> {
    method: String,
    #[serde(borrow)]
    params: &'a RawValue,
}

#[derive(Deserialize)]
struct Response<'a> {
    id: usize,
    #[serde(default)]
    error: Option<Error>,
    #[serde(default, borrow)]
    result: Option<&'a RawValue>,
}

impl LanguageServer {
    pub fn new(path: &Path, background: &executor::Background) -> Result<Arc<Self>> {
        let mut server = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let mut stdin = server.stdin.take().unwrap();
        let mut stdout = BufReader::new(server.stdout.take().unwrap());
        let (outbound_tx, outbound_rx) = channel::unbounded::<Vec<u8>>();
        let response_handlers = Arc::new(Mutex::new(HashMap::<usize, ResponseHandler>::new()));
        let _input_task = background.spawn(
            {
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
                        if let Ok(Notification { .. }) = serde_json::from_slice(&buffer) {
                        } else if let Ok(Response { id, error, result }) =
                            serde_json::from_slice(&buffer)
                        {
                            if let Some(handler) = response_handlers.lock().remove(&id) {
                                if let Some(result) = result {
                                    handler(Ok(result.get()));
                                } else if let Some(error) = error {
                                    handler(Err(error));
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

        let this = Arc::new(Self {
            response_handlers,
            next_id: Default::default(),
            outbound_tx,
            _input_task,
            _output_task,
        });
        let init = this.clone().init();
        background
            .spawn(async move {
                init.log_err().await;
            })
            .detach();

        Ok(this)
    }

    async fn init(self: Arc<Self>) -> Result<()> {
        let init_response = self
            .request::<lsp_types::request::Initialize>(lsp_types::InitializeParams {
                process_id: Default::default(),
                root_path: Default::default(),
                root_uri: Default::default(),
                initialization_options: Default::default(),
                capabilities: Default::default(),
                trace: Default::default(),
                workspace_folders: Default::default(),
                client_info: Default::default(),
                locale: Default::default(),
            })
            .await?;
        Ok(())
    }

    pub fn request<T: lsp_types::request::Request>(
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
        let (tx, rx) = smol::channel::bounded(1);
        response_handlers.insert(
            id,
            Box::new(move |result| {
                let response = match result {
                    Ok(response) => {
                        serde_json::from_str(response).context("failed to deserialize response")
                    }
                    Err(error) => Err(anyhow!("{}", error.message)),
                };
                let _ = smol::block_on(tx.send(response));
            }),
        );

        let outbound_tx = self.outbound_tx.clone();
        async move {
            outbound_tx.send(message).await?;
            rx.recv().await?
        }
    }
}
