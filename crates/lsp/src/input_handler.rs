use std::str;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::{
    AsyncBufReadExt, AsyncRead, AsyncReadExt as _,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
};
use gpui::{BackgroundExecutor, Task};
use log::warn;
use parking_lot::Mutex;
use smol::io::BufReader;

use crate::{
    AnyResponse, CONTENT_LEN_HEADER, IoHandler, IoKind, NotificationOrRequest, RequestId,
    ResponseHandler,
};

const HEADER_DELIMITER: &[u8; 4] = b"\r\n\r\n";
/// Handler for stdout of language server.
pub struct LspStdoutHandler {
    pub(super) loop_handle: Task<Result<()>>,
    pub(super) incoming_messages: UnboundedReceiver<NotificationOrRequest>,
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
        let (tx, notifications_channel) = unbounded();
        let loop_handle = cx.spawn(Self::handler(stdout, tx, response_handlers, io_handlers));
        Self {
            loop_handle,
            incoming_messages: notifications_channel,
        }
    }

    async fn handler<Input>(
        stdout: Input,
        notifications_sender: UnboundedSender<NotificationOrRequest>,
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
                notifications_sender.unbounded_send(msg)?;
            } else if let Ok(AnyResponse {
                id, error, result, ..
            }) = serde_json::from_slice(&buffer)
            {
                let mut response_handlers = response_handlers.lock();
                if let Some(handler) = response_handlers
                    .as_mut()
                    .and_then(|handlers| handlers.remove(&id))
                {
                    drop(response_handlers);
                    if let Some(error) = error {
                        handler(Err(error));
                    } else if let Some(result) = result {
                        handler(Ok(result.get().into()));
                    } else {
                        handler(Ok("null".into()));
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

    #[gpui::test]
    async fn test_read_headers() {
        let mut buf = Vec::new();
        let mut reader = smol::io::BufReader::new(b"Content-Length: 123\r\n\r\n" as &[u8]);
        read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(buf, b"Content-Length: 123\r\n\r\n");

        let mut buf = Vec::new();
        let mut reader = smol::io::BufReader::new(b"Content-Type: application/vscode-jsonrpc\r\nContent-Length: 1235\r\n\r\n{\"somecontent\":123}" as &[u8]);
        read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(
            buf,
            b"Content-Type: application/vscode-jsonrpc\r\nContent-Length: 1235\r\n\r\n"
        );

        let mut buf = Vec::new();
        let mut reader = smol::io::BufReader::new(b"Content-Length: 1235\r\nContent-Type: application/vscode-jsonrpc\r\n\r\n{\"somecontent\":true}" as &[u8]);
        read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(
            buf,
            b"Content-Length: 1235\r\nContent-Type: application/vscode-jsonrpc\r\n\r\n"
        );
    }
}
