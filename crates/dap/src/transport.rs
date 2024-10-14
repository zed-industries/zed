use anyhow::{anyhow, Context, Result};
use dap_types::{
    messages::{Message, Response},
    ErrorResponse,
};
use futures::{AsyncBufRead, AsyncWrite};
use gpui::AsyncAppContext;
use smol::{
    channel::{unbounded, Receiver, Sender},
    io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt},
    lock::Mutex,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub struct Transport {
    pub server_tx: Sender<Message>,
    pub server_rx: Receiver<Message>,
    pub current_requests: Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>,
    pub pending_requests: Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>,
}

impl Transport {
    pub fn start(
        server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        server_stderr: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        cx: &mut AsyncAppContext,
    ) -> Arc<Self> {
        let (client_tx, server_rx) = unbounded::<Message>();
        let (server_tx, client_rx) = unbounded::<Message>();

        let current_requests = Arc::new(Mutex::new(HashMap::default()));
        let pending_requests = Arc::new(Mutex::new(HashMap::default()));

        cx.background_executor()
            .spawn(Self::receive(
                pending_requests.clone(),
                server_stdout,
                client_tx,
            ))
            .detach();

        if let Some(stderr) = server_stderr {
            cx.background_executor().spawn(Self::err(stderr)).detach();
        }

        cx.background_executor()
            .spawn(Self::send(
                current_requests.clone(),
                pending_requests.clone(),
                server_stdin,
                client_rx,
            ))
            .detach();

        Arc::new(Self {
            server_rx,
            server_tx,
            current_requests,
            pending_requests,
        })
    }

    async fn recv_server_message(
        reader: &mut Box<dyn AsyncBufRead + Unpin + Send>,
        buffer: &mut String,
    ) -> Result<Message> {
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
        Ok(serde_json::from_str::<Message>(msg)?)
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
        current_requests: &Mutex<HashMap<u64, Sender<Result<Response>>>>,
        pending_requests: &Mutex<HashMap<u64, Sender<Result<Response>>>>,
        server_stdin: &mut Box<dyn AsyncWrite + Unpin + Send>,
        mut payload: Message,
    ) -> Result<()> {
        if let Message::Request(request) = &mut payload {
            if let Some(sender) = current_requests.lock().await.remove(&request.seq) {
                pending_requests.lock().await.insert(request.seq, sender);
            }
        }
        Self::send_string_to_server(server_stdin, serde_json::to_string(&payload)?).await
    }

    async fn send_string_to_server(
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
            if let Some(body) = response.body {
                if let Ok(error) = serde_json::from_value::<ErrorResponse>(body) {
                    if let Some(message) = error.error {
                        return Err(anyhow!(message.format));
                    };
                };
            }

            Err(anyhow!("Received error response from adapter"))
        }
    }

    async fn process_server_message(
        pending_requests: &Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>,
        client_tx: &Sender<Message>,
        message: Message,
    ) -> Result<()> {
        match message {
            Message::Response(res) => {
                if let Some(tx) = pending_requests.lock().await.remove(&res.request_seq) {
                    tx.send(Self::process_response(res)).await?;
                } else {
                    client_tx.send(Message::Response(res)).await?;
                };
            }
            Message::Request(_) => {
                client_tx.send(message).await?;
            }
            Message::Event(_) => {
                client_tx.send(message).await?;
            }
        }
        Ok(())
    }

    async fn receive(
        pending_requests: Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>,
        mut server_stdout: Box<dyn AsyncBufRead + Unpin + Send>,
        client_tx: Sender<Message>,
    ) -> Result<()> {
        let mut recv_buffer = String::new();

        while let Ok(msg) = Self::recv_server_message(&mut server_stdout, &mut recv_buffer).await {
            Self::process_server_message(&pending_requests, &client_tx, msg)
                .await
                .context("Process server message failed in transport::receive")?;
        }

        Ok(())
    }

    async fn send(
        current_requests: Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>,
        pending_requests: Arc<Mutex<HashMap<u64, Sender<Result<Response>>>>>,
        mut server_stdin: Box<dyn AsyncWrite + Unpin + Send>,
        client_rx: Receiver<Message>,
    ) -> Result<()> {
        while let Ok(payload) = client_rx.recv().await {
            Self::send_payload_to_server(
                &current_requests,
                &pending_requests,
                &mut server_stdin,
                payload,
            )
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
