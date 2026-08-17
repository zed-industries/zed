use crate::state::{State, BUFFER_SIZE};
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// The write half of the pipe
///
/// Implements [`tokio::io::AsyncWrite`][tokio-async-write] when feature `tokio` is enabled (the
/// default). Implements [`futures::io::AsyncWrite`][futures-async-write] when feature `futures` is
/// enabled.
///
/// [futures-async-write]: https://docs.rs/futures/0.3.16/futures/io/trait.AsyncWrite.html
/// [tokio-async-write]: https://docs.rs/tokio/1.9.0/tokio/io/trait.AsyncWrite.html
pub struct PipeWriter {
    pub(crate) state: Arc<Mutex<State>>,
}

impl PipeWriter {
    /// Closes the pipe, any further read will return EOF and any further write will raise an error.
    pub fn close(&self) -> io::Result<()> {
        match self.state.lock() {
            Ok(mut state) => {
                state.closed = true;
                self.wake_reader_half(&*state);
                Ok(())
            }
            Err(err) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "{}: PipeWriter: Failed to lock the channel state: {}",
                    env!("CARGO_PKG_NAME"),
                    err
                ),
            )),
        }
    }

    /// It returns true if the next data chunk is written and consumed by the reader; Otherwise it returns false.
    pub fn is_flushed(&self) -> io::Result<bool> {
        let state = match self.state.lock() {
            Ok(s) => s,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: PipeWriter: Failed to lock the channel state: {}",
                        env!("CARGO_PKG_NAME"),
                        err
                    ),
                ));
            }
        };

        Ok(state.buffer.is_empty())
    }

    fn wake_reader_half(&self, state: &State) {
        if let Some(ref waker) = state.reader_waker {
            waker.clone().wake();
        }
    }

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        if Arc::strong_count(&self.state) == 1 {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                format!(
                    "{}: PipeWriter: The channel is closed",
                    env!("CARGO_PKG_NAME")
                ),
            )));
        }

        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(err) => {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: PipeWriter: Failed to lock the channel state: {}",
                        env!("CARGO_PKG_NAME"),
                        err
                    ),
                )))
            }
        };

        self.wake_reader_half(&*state);

        let remaining = BUFFER_SIZE - state.buffer.len();
        if remaining == 0 {
            state.writer_waker = Some(cx.waker().clone());
            Poll::Pending
        } else {
            let bytes_to_write = remaining.min(buf.len());
            state.buffer.extend_from_slice(&buf[..bytes_to_write]);
            Poll::Ready(Ok(bytes_to_write))
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        if Arc::strong_count(&self.state) == 1 {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                format!(
                    "{}: PipeWriter: The channel is closed",
                    env!("CARGO_PKG_NAME")
                ),
            )));
        }

        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(err) => {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: PipeWriter: Failed to lock the channel state: {}",
                        env!("CARGO_PKG_NAME"),
                        err
                    ),
                )))
            }
        };

        if state.buffer.is_empty() {
            Poll::Ready(Ok(()))
        } else {
            state.writer_waker = Some(cx.waker().clone());
            self.wake_reader_half(&*state);
            Poll::Pending
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        match self.close() {
            Ok(_) => Poll::Ready(Ok(())),
            Err(err) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "{}: PipeWriter: Failed to shutdown the channel: {}",
                    env!("CARGO_PKG_NAME"),
                    err
                ),
            ))),
        }
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        self.close().ok();
    }
}

#[cfg(feature = "tokio")]
impl tokio::io::AsyncWrite for PipeWriter {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        self.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        self.poll_shutdown(cx)
    }
}

#[cfg(feature = "futures")]
impl futures::io::AsyncWrite for PipeWriter {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        self.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        self.poll_shutdown(cx)
    }
}
