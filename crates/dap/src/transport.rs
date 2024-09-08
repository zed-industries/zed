use anyhow::{anyhow, Context, Result};
use dap_types::{
    BreakpointEvent, Capabilities, CapabilitiesEvent, ContinuedEvent, ExitedEvent,
    InvalidatedEvent, LoadedSourceEvent, MemoryEvent, ModuleEvent, OutputEvent, ProcessEvent,
    ProgressEndEvent, ProgressStartEvent, ProgressUpdateEvent, StoppedEvent, TerminatedEvent,
    ThreadEvent,
};
use futures::{AsyncBufRead, AsyncWrite};
use gpui::AsyncAppContext;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use smol::{
    channel::{unbounded, Receiver, Sender},
    io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt},
    lock::Mutex,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Payload {
    Event(Box<Events>),
    Response(Response),
    Request(Request),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "event", content = "body")]
#[serde(rename_all = "camelCase")]
pub enum Events {
    Initialized(Option<Capabilities>),
    Stopped(StoppedEvent),
    Continued(ContinuedEvent),
    Exited(ExitedEvent),
    Terminated(Option<TerminatedEvent>),
    Thread(ThreadEvent),
    Output(OutputEvent),
    Breakpoint(BreakpointEvent),
    Module(ModuleEvent),
    LoadedSource(LoadedSourceEvent),
    Process(ProcessEvent),
    Capabilities(CapabilitiesEvent),
    ProgressStart(ProgressStartEvent),
    ProgressUpdate(ProgressUpdateEvent),
    ProgressEnd(ProgressEndEvent),
    Invalidated(InvalidatedEvent),
    Memory(MemoryEvent),
    #[serde(untagged)]
    Other(HashMap<String, Value>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    #[serde(skip)]
    pub back_ch: Option<Sender<Result<Response>>>,
    pub seq: u64,
    pub command: String,
    pub arguments: Option<Value>,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq && self.command == other.command && self.arguments == other.arguments
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Response {
    pub request_seq: u64,
    pub success: bool,
    pub command: String,
    pub message: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub body: Option<Value>,
}

fn deserialize_empty_object<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value == Value::Object(serde_json::Map::new()) {
        Ok(None)
    } else {
        Ok(Some(value))
    }
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
        cx: &mut AsyncAppContext,
    ) -> (Receiver<Payload>, Sender<Payload>) {
        let (client_tx, server_rx) = unbounded::<Payload>();
        let (server_tx, client_rx) = unbounded::<Payload>();

        let transport = Arc::new(Self {
            pending_requests: Mutex::new(HashMap::default()),
        });

        let _ = cx.update(|cx| {
            let transport = transport.clone();

            cx.background_executor()
                .spawn(Self::receive(transport.clone(), server_stdout, client_tx))
                .detach_and_log_err(cx);

            cx.background_executor()
                .spawn(Self::send(transport.clone(), server_stdin, client_rx))
                .detach_and_log_err(cx);

            if let Some(stderr) = server_stderr {
                cx.background_executor()
                    .spawn(Self::err(stderr))
                    .detach_and_log_err(cx);
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

            if reader
                .read_line(buffer)
                .await
                .with_context(|| "reading a message from server")?
                == 0
            {
                return Err(anyhow!("debugger reader stream closed"));
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

        let mut content = vec![0; content_length];
        reader
            .read_exact(&mut content)
            .await
            .with_context(|| "reading after a loop")?;

        let msg = std::str::from_utf8(&content).context("invalid utf8 from server")?;
        Ok(serde_json::from_str::<Payload>(msg)?)
    }

    async fn recv_server_error(
        err: &mut (impl AsyncBufRead + Unpin + Send),
        buffer: &mut String,
    ) -> Result<()> {
        buffer.truncate(0);
        if err.read_line(buffer).await? == 0 {
            return Err(anyhow!("debugger error stream closed"));
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
                self.pending_requests.lock().await.insert(request.seq, back);
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
        server_stdin
            .write_all(format!("Content-Length: {}\r\n\r\n{}", request.len(), request).as_bytes())
            .await?;

        server_stdin.flush().await?;
        Ok(())
    }

    fn process_response(response: Response) -> Result<Response> {
        if response.success {
            Ok(response)
        } else {
            Err(anyhow!("Received failed response"))
        }
    }

    async fn process_server_message(
        &self,
        client_tx: &Sender<Payload>,
        payload: Payload,
    ) -> Result<()> {
        match payload {
            Payload::Response(res) => {
                if let Some(tx) = self.pending_requests.lock().await.remove(&res.request_seq) {
                    if !tx.is_closed() {
                        tx.send(Self::process_response(res)).await?;
                    } else {
                        log::warn!(
                            "Response stream associated with request seq: {} is closed",
                            &res.request_seq
                        ); // TODO: Fix this case so it never happens
                    }
                } else {
                    client_tx.send(Payload::Response(res)).await?;
                };
            }

            Payload::Request(_) => {
                client_tx.send(payload).await?;
            }
            Payload::Event(_) => {
                client_tx.send(payload).await?;
            }
        }
        Ok(())
    }

    async fn receive(
        transport: Arc<Self>,
        mut server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        client_tx: Sender<Payload>,
    ) -> Result<()> {
        let mut recv_buffer = String::new();
        loop {
            transport
                .process_server_message(
                    &client_tx,
                    Self::recv_server_message(&mut server_stdout, &mut recv_buffer).await?,
                )
                .await
                .context("Process server message failed in transport::receive")?;
        }
    }

    async fn send(
        transport: Arc<Self>,
        mut server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        client_rx: Receiver<Payload>,
    ) -> Result<()> {
        while let Ok(payload) = client_rx.recv().await {
            transport
                .send_payload_to_server(&mut server_stdin, payload)
                .await?;
        }

        Ok(())
    }

    async fn err(mut server_stderr: Box<dyn AsyncBufRead + Unpin + Send>) -> Result<()> {
        let mut recv_buffer = String::new();
        loop {
            Self::recv_server_error(&mut server_stderr, &mut recv_buffer).await?;
        }
    }
}
