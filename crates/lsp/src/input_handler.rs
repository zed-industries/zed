use std::str;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::{
    AsyncBufReadExt, AsyncRead, AsyncReadExt as _, SinkExt as _,
    channel::mpsc::{Receiver, Sender, channel},
    io::BufReader,
};
use gpui::{BackgroundExecutor, Task};
use log::warn;
use parking_lot::Mutex;

use crate::{
    AnyResponse, CONTENT_LEN_HEADER, IoHandler, IoKind, NotificationOrRequest, RequestId,
    ResponseHandler,
};

const HEADER_DELIMITER: &[u8; 4] = b"\r\n\r\n";

/// Bounds the number of incoming LSP messages buffered between the background
/// reader and the foreground dispatcher. When the queue is full, the reader
/// stops reading the server's stdout, letting the OS pipe apply backpressure
/// to the server instead of buffering messages in memory without limit while
/// the foreground thread is unresponsive.
pub(crate) const INCOMING_MESSAGE_QUEUE_CAPACITY: usize = 128;

/// Handler for stdout of language server.
pub struct LspStdoutHandler {
    pub(super) loop_handle: Task<Result<()>>,
    pub(super) incoming_messages: Receiver<NotificationOrRequest>,
}

async fn read_headers<Stdout>(reader: &mut BufReader<Stdout>, buffer: &mut Vec<u8>) -> Result<()>
where
    Stdout: AsyncRead + Unpin + Send + 'static,
{
    loop {
        if buffer.len() >= HEADER_DELIMITER.len()
            && buffer[(buffer.len() - HEADER_DELIMITER.len())..] == HEADER_DELIMITER[..]
        {
            return Ok(());
        }

        if reader.read_until(b'\n', buffer).await? == 0 {
            anyhow::bail!("cannot read LSP message headers");
        }
    }
}

impl LspStdoutHandler {
    pub fn new<Input>(
        stdout: Input,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
        cx: BackgroundExecutor,
    ) -> Self
    where
        Input: AsyncRead + Unpin + Send + 'static,
    {
        let (tx, notifications_channel) = channel(INCOMING_MESSAGE_QUEUE_CAPACITY);
        let loop_handle = cx.spawn(Self::handler(stdout, tx, response_handlers, io_handlers));
        Self {
            loop_handle,
            incoming_messages: notifications_channel,
        }
    }

    async fn handler<Input>(
        stdout: Input,
        mut notifications_sender: Sender<NotificationOrRequest>,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
    ) -> anyhow::Result<()>
    where
        Input: AsyncRead + Unpin + Send + 'static,
    {
        let mut stdout = BufReader::new(stdout);

        let mut buffer = Vec::new();

        loop {
            buffer.clear();

            read_headers(&mut stdout, &mut buffer).await?;

            let headers = std::str::from_utf8(&buffer)?;

            let message_len = headers
                .split('\n')
                .find(|line| line.starts_with(CONTENT_LEN_HEADER))
                .and_then(|line| line.strip_prefix(CONTENT_LEN_HEADER))
                .with_context(|| format!("invalid LSP message header {headers:?}"))?
                .trim_end()
                .parse()?;

            buffer.resize(message_len, 0);
            stdout.read_exact(&mut buffer).await?;

            if let Ok(message) = str::from_utf8(&buffer) {
                log::trace!("incoming message: {message}");
                for handler in io_handlers.lock().values_mut() {
                    handler(IoKind::StdOut, message);
                }
            }

            if let Ok(msg) = serde_json::from_slice::<NotificationOrRequest>(&buffer) {
                notifications_sender.send(msg).await?;
            } else if let Ok(AnyResponse {
                id, error, result, ..
            }) = serde_json::from_slice(&buffer)
            {
                let handler = {
                    response_handlers
                        .lock()
                        .as_mut()
                        .and_then(|handlers| handlers.remove(&id))
                };
                if let Some(handler) = handler {
                    if let Some(error) = error {
                        handler(Err(error)).await;
                    } else if let Some(result) = result {
                        handler(Ok(result.get().into())).await;
                    } else {
                        handler(Ok("null".into())).await;
                    }
                }
            } else {
                warn!(
                    "failed to deserialize LSP message:\n{}",
                    std::str::from_utf8(&buffer)?
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{AsyncWriteExt as _, StreamExt as _};
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_backpressure_when_messages_are_not_consumed(cx: &mut TestAppContext) {
        let total_messages = INCOMING_MESSAGE_QUEUE_CAPACITY * 4;
        let (mut writer, reader) = async_pipe::pipe();
        let mut handler = LspStdoutHandler::new(
            reader,
            Arc::new(Mutex::new(Some(HashMap::default()))),
            Arc::new(Mutex::new(HashMap::default())),
            cx.background_executor.clone(),
        );

        cx.background_executor
            .spawn(async move {
                let payload = r#"{"jsonrpc":"2.0","method":"test/notification","params":{}}"#;
                let message = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);
                for _ in 0..total_messages {
                    writer.write_all(message.as_bytes()).await.unwrap();
                }
            })
            .detach();

        cx.run_until_parked();
        let mut received = 0;
        while handler.incoming_messages.try_recv().is_ok() {
            received += 1;
        }
        assert!(
            received < total_messages,
            "the reader buffered all {total_messages} messages while the consumer was wedged"
        );
        assert!(
            received <= INCOMING_MESSAGE_QUEUE_CAPACITY + 2,
            "expected at most {} buffered messages, got {received}",
            INCOMING_MESSAGE_QUEUE_CAPACITY + 2
        );

        while received < total_messages {
            assert!(
                handler.incoming_messages.next().await.is_some(),
                "the message stream ended after {received} of {total_messages} messages"
            );
            received += 1;
        }
    }

    #[gpui::test]
    async fn test_read_headers() {
        let mut buf = Vec::new();
        let mut reader = BufReader::new(b"Content-Length: 123\r\n\r\n" as &[u8]);
        read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(buf, b"Content-Length: 123\r\n\r\n");

        let mut buf = Vec::new();
        let mut reader = BufReader::new(b"Content-Type: application/vscode-jsonrpc\r\nContent-Length: 1235\r\n\r\n{\"somecontent\":123}" as &[u8]);
        read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(
            buf,
            b"Content-Type: application/vscode-jsonrpc\r\nContent-Length: 1235\r\n\r\n"
        );

        let mut buf = Vec::new();
        let mut reader = BufReader::new(b"Content-Length: 1235\r\nContent-Type: application/vscode-jsonrpc\r\n\r\n{\"somecontent\":true}" as &[u8]);
        read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(
            buf,
            b"Content-Length: 1235\r\nContent-Type: application/vscode-jsonrpc\r\n\r\n"
        );
    }
}
