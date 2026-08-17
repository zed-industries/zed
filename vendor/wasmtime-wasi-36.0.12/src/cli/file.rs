use crate::cli::{IsTerminal, StdinStream, StdoutStream};
use crate::p2::{InputStream, OutputStream, Pollable, StreamError, StreamResult};
use bytes::Bytes;
use std::io::{Read, Write};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, AsyncWrite};

/// This implementation will yield output streams that block on writes, and
/// output directly to a file. If truly async output is required, [`AsyncStdoutStream`]
/// should be used instead.
#[derive(Clone)]
pub struct OutputFile {
    file: Arc<std::fs::File>,
}

impl OutputFile {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            file: Arc::new(file),
        }
    }
}

impl IsTerminal for OutputFile {
    fn is_terminal(&self) -> bool {
        false
    }
}

impl StdoutStream for OutputFile {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(self.clone())
    }

    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(self.clone())
    }
}

#[async_trait::async_trait]
impl Pollable for OutputFile {
    async fn ready(&mut self) {}
}

impl OutputStream for OutputFile {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        (&*self.file)
            .write_all(&bytes)
            .map_err(|e| StreamError::LastOperationFailed(anyhow::anyhow!(e)))
    }

    fn flush(&mut self) -> StreamResult<()> {
        use std::io::Write;
        self.file
            .flush()
            .map_err(|e| StreamError::LastOperationFailed(anyhow::anyhow!(e)))
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

impl AsyncWrite for OutputFile {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match (&*self.file).write_all(buf) {
            Ok(()) => Poll::Ready(Ok(buf.len())),
            Err(e) => Poll::Ready(Err(e)),
        }
    }
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready((&*self.file).flush())
    }
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// This implementation will yield input streams that block on reads, and
/// reads directly from a file. If truly async input is required,
/// [`AsyncStdinStream`] should be used instead.
#[derive(Clone)]
pub struct InputFile {
    file: Arc<std::fs::File>,
}

impl InputFile {
    pub fn new(file: std::fs::File) -> Self {
        Self {
            file: Arc::new(file),
        }
    }
}

impl StdinStream for InputFile {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(self.clone())
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(self.clone())
    }
}

impl IsTerminal for InputFile {
    fn is_terminal(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
impl Pollable for InputFile {
    async fn ready(&mut self) {}
}

impl InputStream for InputFile {
    fn read(&mut self, size: usize) -> StreamResult<Bytes> {
        let mut buf = bytes::BytesMut::zeroed(size.min(crate::MAX_READ_SIZE_ALLOC));
        let bytes_read = self
            .file
            .read(&mut buf)
            .map_err(|e| StreamError::LastOperationFailed(anyhow::anyhow!(e)))?;
        if bytes_read == 0 {
            return Err(StreamError::Closed);
        }
        buf.truncate(bytes_read);
        StreamResult::Ok(buf.into())
    }
}

impl AsyncRead for InputFile {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match (&*self.file).read(buf.initialize_unfilled()) {
            Ok(n) => {
                buf.advance(n);
                Poll::Ready(Ok(()))
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
