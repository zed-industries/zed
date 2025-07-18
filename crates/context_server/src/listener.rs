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
use net::async_net::{UnixListener, UnixStream};
use serde_json::{json, value::RawValue};
use smol::stream::StreamExt;
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

type McpHandler = Box<dyn Fn(RequestId, Option<Box<RawValue>>, &App) -> Task<String>>;

impl McpServer {
    pub fn new(cx: &AsyncApp) -> Task<Result<Self>> {
        let task = cx.background_spawn(async move {
            let temp_dir = tempfile::Builder::new().prefix("zed-mcp").tempdir()?;
            let socket_path = temp_dir.path().join("mcp.sock");
            let listener = UnixListener::bind(&socket_path).context("creating mcp socket")?;

            anyhow::Ok((temp_dir, socket_path, listener))
        });

        cx.spawn(async move |cx| {
            let (temp_dir, socket_path, listener) = task.await?;
            let handlers = Rc::new(RefCell::new(HashMap::default()));
            let server_task = cx.spawn({
                let handlers = handlers.clone();
                async move |cx| {
                    while let Ok((stream, _)) = listener.accept().await {
                        Self::serve_connection(stream, handlers.clone(), cx);
                    }
                    drop(temp_dir)
                }
            });
            Ok(Self {
                socket_path,
                _server_task: server_task,
                handlers: handlers.clone(),
            })
        })
    }

    pub fn handle_request<R: Request>(
        &mut self,
        f: impl Fn(R::Params, &App) -> Task<Result<R::Response>> + 'static,
    ) {
        let f = Box::new(f);
        self.handlers.borrow_mut().insert(
            R::METHOD,
            Box::new(move |req_id, opt_params, cx| {
                let result = match opt_params {
                    Some(params) => serde_json::from_str(params.get()),
                    None => serde_json::from_value(serde_json::Value::Null),
                };

                let params: R::Params = match result {
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

    fn serve_connection(
        stream: UnixStream,
        handlers: Rc<RefCell<HashMap<&'static str, McpHandler>>>,
        cx: &mut AsyncApp,
    ) {
        let (read, write) = smol::io::split(stream);
        let (incoming_tx, mut incoming_rx) = unbounded();
        let (outgoing_tx, outgoing_rx) = unbounded();

        cx.background_spawn(Self::handle_io(outgoing_rx, incoming_tx, write, read))
            .detach();

        cx.spawn(async move |cx| {
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
        })
        .detach();
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
                            outgoing_bytes.write_all(serde_json::to_string(&json!({
                                "jsonrpc": "2.0",
                                "error": json!({
                                    "code": -32603,
                                    "message": format!("Failed to parse: {error}"),
                                }),
                            }))?.as_bytes()).await?;
                            outgoing_bytes.write_all(&[b'\n']).await?;
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

#[derive(Serialize, Deserialize)]
struct RawRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RequestId>,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Box<serde_json::value::RawValue>>,
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
