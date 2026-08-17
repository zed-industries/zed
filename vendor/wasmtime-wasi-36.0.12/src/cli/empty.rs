use crate::cli::{IsTerminal, StdinStream, StdoutStream};
use crate::p2;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, AsyncWrite};
use wasmtime_wasi_io::streams::{InputStream, OutputStream};

// Implementation for tokio::io::Empty
impl IsTerminal for tokio::io::Empty {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdinStream for tokio::io::Empty {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(p2::pipe::ClosedInputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(tokio::io::empty())
    }
}
impl StdoutStream for tokio::io::Empty {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(p2::pipe::SinkOutputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::empty())
    }
}

// Implementation for std::io::Empty
impl IsTerminal for std::io::Empty {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdinStream for std::io::Empty {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(p2::pipe::ClosedInputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(tokio::io::empty())
    }
}
impl StdoutStream for std::io::Empty {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(p2::pipe::SinkOutputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::empty())
    }
}

// Implementation for p2::pipe::ClosedInputStream
impl IsTerminal for p2::pipe::ClosedInputStream {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdinStream for p2::pipe::ClosedInputStream {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(p2::pipe::ClosedInputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(tokio::io::empty())
    }
}

// Implementation for p2::pipe::SinkOutputStream
impl IsTerminal for p2::pipe::SinkOutputStream {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdoutStream for p2::pipe::SinkOutputStream {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(p2::pipe::SinkOutputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::empty())
    }
}

// Implementation for p2::pipe::ClosedOutputStream
impl IsTerminal for p2::pipe::ClosedOutputStream {
    fn is_terminal(&self) -> bool {
        false
    }
}
impl StdoutStream for p2::pipe::ClosedOutputStream {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(p2::pipe::ClosedOutputStream)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        struct AlwaysClosed;

        impl AsyncWrite for AlwaysClosed {
            fn poll_write(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
                _buf: &[u8],
            ) -> Poll<io::Result<usize>> {
                Poll::Ready(Ok(0))
            }
            fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
                Poll::Ready(Ok(()))
            }
            fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
                Poll::Ready(Ok(()))
            }
        }

        Box::new(AlwaysClosed)
    }
}
