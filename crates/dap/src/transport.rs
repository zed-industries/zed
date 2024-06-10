use crate::events::Event;
use anyhow::{anyhow, Context, Result};
use futures::{
    channel::mpsc::{unbounded, Sender, UnboundedReceiver, UnboundedSender},
    AsyncBufRead, AsyncWrite, SinkExt as _, StreamExt,
};
use gpui::AsyncWindowContext;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smol::io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _};
use std::{collections::HashMap, sync::Arc};
use util::ResultExt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    #[serde(skip)]
    pub back_ch: Option<Sender<Result<Response>>>,
    pub seq: u64,
    pub command: String,
    pub arguments: Option<Value>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Response {
    pub request_seq: u64,
    pub success: bool,
    pub command: String,
    pub message: Option<String>,
    pub body: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Payload {
    Event(Box<Event>),
    Response(Response),
    Request(Request),
}

#[derive(Debug)]
pub struct Transport {
    pending_requests: Mutex<HashMap<u64, Sender<Result<Response>>>>,
}

impl Transport {
    pub fn start(
        server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        server_stderr: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        cx: &mut AsyncWindowContext,
    ) -> (UnboundedReceiver<Payload>, UnboundedSender<Payload>) {
        let (client_tx, server_rx) = unbounded::<Payload>();
        let (server_tx, client_rx) = unbounded::<Payload>();

        let transport = Self {
            pending_requests: Mutex::new(HashMap::default()),
        };

        let transport = Arc::new(transport);

        cx.update(|cx| {
            cx.spawn(|_| Self::recv(transport.clone(), server_stdout, client_tx))
                .detach_and_log_err(cx);
            cx.spawn(|_| Self::send(transport, server_stdin, client_rx))
                .detach_and_log_err(cx);

            if let Some(stderr) = server_stderr {
                cx.spawn(|_| Self::err(stderr)).detach();
            }
        });

        (server_rx, server_tx)
    }

    async fn recv_server_message(
        reader: &mut Box<dyn AsyncBufRead + Unpin + Send>,
        buffer: &mut String,
    ) -> Result<Payload> {
        let mut content_length = None;
        loop {
            buffer.truncate(0);
            if reader.read_line(buffer).await? == 0 {
                return Err(anyhow!("stream closed"));
            };

            if buffer == "\r\n" {
                break;
            }

            let parts = buffer.trim().split_once(": ");

            match parts {
                Some(("Content-Length", value)) => {
                    content_length = Some(value.parse().context("invalid content length")?);
                }
                _ => {}
            }
        }

        let content_length = content_length.context("missing content length")?;

        //TODO: reuse vector
        let mut content = vec![0; content_length];
        reader.read_exact(&mut content).await?;
        let msg = std::str::from_utf8(&content).context("invalid utf8 from server")?;

        dbg!("<- DAP {}", msg);

        Ok(serde_json::from_str::<Payload>(msg)?)
    }

    async fn recv_server_error(
        err: &mut (impl AsyncBufRead + Unpin + Send),
        buffer: &mut String,
    ) -> Result<()> {
        buffer.truncate(0);
        if err.read_line(buffer).await? == 0 {
            return Err(anyhow!("stream closed"));
        };

        Ok(())
    }

    async fn send_payload_to_server(
        &self,
        server_stdin: &mut Box<dyn AsyncWrite + Unpin + Send>,
        mut payload: Payload,
    ) -> Result<()> {
        if let Payload::Request(request) = &mut payload {
            if let Some(back) = request.back_ch.take() {
                self.pending_requests.lock().insert(request.seq, back);
            }
        }
        self.send_string_to_server(server_stdin, serde_json::to_string(&payload)?)
            .await
    }

    async fn send_string_to_server(
        &self,
        server_stdin: &mut Box<dyn AsyncWrite + Unpin + Send>,
        request: String,
    ) -> Result<()> {
        dbg!("Request {}", &request);

        server_stdin
            .write_all(format!("Content-Length: {}\r\n\r\n", request.len()).as_bytes())
            .await?;

        server_stdin.write_all(request.as_bytes()).await?;

        server_stdin.flush().await?;

        Ok(())
    }

    fn process_response(response: Response) -> Result<Response> {
        if response.success {
            Ok(response)
        } else {
            Err(anyhow!("some error"))
        }
    }

    async fn process_server_message(
        &self,
        mut client_tx: &UnboundedSender<Payload>,
        msg: Payload,
    ) -> Result<()> {
        match msg {
            Payload::Response(res) => {
                match self.pending_requests.lock().remove(&res.request_seq) {
                    Some(mut tx) => match tx.send(Self::process_response(res)).await {
                        Ok(_) => (),
                        Err(_) => (),
                    },
                    None => {
                        dbg!("Response to nonexistent request #{}", res.request_seq);
                        client_tx.send(Payload::Response(res)).await.log_err();
                    }
                }

                Ok(())
            }
            Payload::Request(_) => {
                client_tx.send(msg).await.log_err();
                Ok(())
            }
            Payload::Event(_) => {
                client_tx.send(msg).await.log_err();
                Ok(())
            }
        }
    }

    async fn recv(
        transport: Arc<Self>,
        mut server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        client_tx: UnboundedSender<Payload>,
    ) -> Result<()> {
        let mut recv_buffer = String::new();
        loop {
            transport
                .process_server_message(
                    &client_tx,
                    Self::recv_server_message(&mut server_stdout, &mut recv_buffer).await?,
                )
                .await?;
        }
    }

    async fn send(
        transport: Arc<Self>,
        mut server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        mut client_rx: UnboundedReceiver<Payload>,
    ) -> Result<()> {
        while let Some(payload) = client_rx.next().await {
            transport
                .send_payload_to_server(&mut server_stdin, payload)
                .await?;
        }

        Ok(())
    }

    async fn err(mut server_stderr: Box<dyn AsyncBufRead + Unpin + Send>) {
        let mut recv_buffer = String::new();
        loop {
            match Self::recv_server_error(&mut server_stderr, &mut recv_buffer).await {
                Ok(_) => {}
                Err(err) => {
                    dbg!("err: <- {:?}", err);
                    break;
                }
            }
        }
    }
}
