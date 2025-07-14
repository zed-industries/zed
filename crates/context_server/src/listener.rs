use ::serde::{Deserialize, Serialize};
use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::{
    AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, FutureExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
    io::BufReader,
    select_biased,
};
use gpui::{App, AppContext, AsyncApp, Task};
use serde_json::value::RawValue;
use smol::{
    net::unix::{UnixListener, UnixStream},
    stream::StreamExt,
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
};
use util::ResultExt;

use crate::{
    client::{CspResult, RequestId, Response},
    types::Request,
};

pub struct McpServer {
    socket_path: PathBuf,
    handlers: Rc<RefCell<HashMap<&'static str, McpHandler>>>,
    _server_task: Task<()>,
}

type McpHandler = Box<dyn Fn(RequestId, Box<RawValue>, &App) -> Task<String>>;

#[cfg(not(target_os = "windows"))]
const ZED_MCP_SCRIPT_NAME: &str = "mcp.sh";
#[cfg(target_os = "windows")]
const ZED_MCP_SCRIPT_NAME: &str = "mcp.ps1";

impl McpServer {
    pub fn new(cx: &App) -> Result<Self> {
        let temp_dir = tempfile::Builder::new().prefix("zed-mcp").tempdir()?;
        let mcp_socket = temp_dir.path().join("mcp.sock");
        let mcp_script_path = temp_dir.path().join(ZED_MCP_SCRIPT_NAME);

        let listener = UnixListener::bind(&mcp_socket).context("creating mcp socket")?;
        let handlers = Rc::new(RefCell::new(HashMap::default()));

        let server_task = cx.spawn({
            let handlers = handlers.clone();
            async move |cx| {
                while let Ok((stream, _)) = listener.accept().await {
                    McpConnection::serve(stream, handlers.clone(), cx);
                }
                drop(temp_dir)
            }
        });

        Ok(Self {
            socket_path: mcp_socket,
            _server_task: server_task,
            handlers: handlers.clone(),
        })
    }

    pub fn handle_request<R: Request>(
        &mut self,
        f: Box<dyn Fn(R::Params, &App) -> Task<Result<R::Response>>>,
    ) {
        self.handlers.borrow_mut().insert(
            R::METHOD,
            Box::new(move |req_id, v, cx| {
                let params: R::Params = match serde_json::from_str(v.get()) {
                    Ok(params) => params,
                    Err(e) => {
                        return Task::ready(
                            serde_json::to_string(&Response::<R::Response> {
                                jsonrpc: "2.0",
                                id: req_id,
                                value: CspResult::Error(Some(crate::client::Error {
                                    message: format!("{e}"),
                                    code: -32700,
                                })),
                            })
                            .unwrap(),
                        );
                    }
                };
                let task = f(params, cx);
                cx.background_spawn(async move {
                    match task.await {
                        Ok(result) => serde_json::to_string(&Response {
                            jsonrpc: "2.0",
                            id: req_id,
                            value: CspResult::Ok(Some(result)),
                        })
                        .unwrap(),
                        Err(e) => serde_json::to_string(&Response {
                            jsonrpc: "2.0",
                            id: req_id,
                            value: CspResult::Error::<R::Response>(Some(crate::client::Error {
                                message: format!("{e}"),
                                code: -32603,
                            })),
                        })
                        .unwrap(),
                    }
                })
            }),
        );
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

#[derive(Serialize, Deserialize)]
struct RawRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RequestId>,
    method: String,
    params: Box<serde_json::value::RawValue>,
}

#[derive(Serialize, Deserialize)]
struct RawResponse {
    jsonrpc: &'static str,
    id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<crate::client::Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Box<serde_json::value::RawValue>>,
}

struct McpConnection {
    _io_task: Task<Result<()>>,
    _handle_task: Task<()>,
}

impl McpConnection {
    pub fn serve(
        stream: UnixStream,
        handlers: Rc<RefCell<HashMap<&'static str, McpHandler>>>,
        cx: &mut AsyncApp,
    ) -> Self {
        let (read, write) = smol::io::split(stream);
        let (incoming_tx, mut incoming_rx) = unbounded();
        let (outgoing_tx, outgoing_rx) = unbounded();

        let io_task = cx.background_spawn(Self::handle_io(outgoing_rx, incoming_tx, write, read));

        let handle_task = cx.spawn(async move |cx| {
            while let Some(request) = incoming_rx.next().await {
                let Some(request_id) = request.id.clone() else {
                    continue;
                };
                if let Some(handler) = handlers.borrow().get(&request.method.as_ref()) {
                    let outgoing_tx = outgoing_tx.clone();

                    if let Some(task) = cx
                        .update(|cx| handler(request_id, request.params, cx))
                        .log_err()
                    {
                        cx.spawn(async move |_| {
                            let response = task.await;
                            outgoing_tx.unbounded_send(response).ok();
                        })
                        .detach();
                    }
                } else {
                    outgoing_tx
                        .unbounded_send(
                            serde_json::to_string(&Response::<()> {
                                jsonrpc: "2.0",
                                id: request.id.unwrap(),
                                value: CspResult::Error(Some(crate::client::Error {
                                    message: format!("unhandled method {}", request.method),
                                    code: -32601,
                                })),
                            })
                            .unwrap(),
                        )
                        .ok();
                }
            }
        });

        Self {
            _io_task: io_task,
            _handle_task: handle_task,
        }
    }

    async fn handle_io(
        mut outgoing_rx: UnboundedReceiver<String>,
        incoming_tx: UnboundedSender<RawRequest>,
        mut outgoing_bytes: impl Unpin + AsyncWrite,
        incoming_bytes: impl Unpin + AsyncRead,
    ) -> Result<()> {
        let mut output_reader = BufReader::new(incoming_bytes);
        let mut incoming_line = String::new();
        loop {
            select_biased! {
                message = outgoing_rx.next().fuse() => {
                    if let Some(message) = message {
                        log::trace!("send: {}", &message);
                        outgoing_bytes.write_all(message.as_bytes()).await?;
                        outgoing_bytes.write_all(&[b'\n']).await?;
                    } else {
                        break;
                    }
                }
                bytes_read = output_reader.read_line(&mut incoming_line).fuse() => {
                    if bytes_read? == 0 {
                        break
                    }
                    log::trace!("recv: {}", &incoming_line);
                    match serde_json::from_str(&incoming_line) {
                        Ok(message) => {
                            incoming_tx.unbounded_send(message).log_err();
                        }
                        Err(error) => {
                            log::error!("failed to parse incoming message: {error}. Raw: {incoming_line}");
                        }
                    }
                    incoming_line.clear();
                }
            }
        }
        Ok(())
    }
}
