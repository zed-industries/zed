use crate::state::State;
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// The read half of the pipe
///
/// Implements [`tokio::io::AsyncRead`][tokio-async-read] when feature `tokio` is enabled (the
/// default). Implements [`futures::io::AsyncRead`][futures-async-read] when feature `futures` is
/// enabled.
///
/// [futures-async-read]: https://docs.rs/futures/0.3.16/futures/io/trait.AsyncRead.html
/// [tokio-async-read]: https://docs.rs/tokio/1.9.0/tokio/io/trait.AsyncRead.html
pub struct PipeReader {
    pub(crate) state: Arc<Mutex<State>>,
}

impl PipeReader {
    /// Closes the pipe, any further read will return EOF and any further write will raise an error.
    pub fn close(&self) -> io::Result<()> {
        match self.state.lock() {
            Ok(mut state) => {
                state.closed = true;
                self.wake_writer_half(&*state);
                Ok(())
            }
            Err(err) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "{}: PipeReader: Failed to lock the channel state: {}",
                    env!("CARGO_PKG_NAME"),
                    err
                ),
            )),
        }
    }

    /// It returns true if the next data chunk is written by the writer and consumed by the reader; Otherwise it returns false.
    pub fn is_flushed(&self) -> io::Result<bool> {
        let state = match self.state.lock() {
            Ok(s) => s,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: PipeReader: Failed to lock the channel state: {}",
                        env!("CARGO_PKG_NAME"),
                        err
                    ),
                ));
            }
        };

        Ok(state.buffer.is_empty())
    }

    fn wake_writer_half(&self, state: &State) {
        if let Some(ref waker) = state.writer_waker {
            waker.clone().wake();
        }
    }

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(err) => {
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: PipeReader: Failed to lock the channel state: {}",
                        env!("CARGO_PKG_NAME"),
                        err
                    ),
                )))
            }
        };

        if state.buffer.is_empty() {
            if state.closed || Arc::strong_count(&self.state) == 1 {
                Poll::Ready(Ok(0))
            } else {
                self.wake_writer_half(&*state);
                state.reader_waker = Some(cx.waker().clone());
                Poll::Pending
            }
        } else {
            self.wake_writer_half(&*state);
            let size_to_read = state.buffer.len().min(buf.len());
            let (to_read, rest) = state.buffer.split_at(size_to_read);
            buf[..size_to_read].copy_from_slice(to_read);
            state.buffer = rest.to_vec();

            Poll::Ready(Ok(size_to_read))
        }
    }
}

#[cfg(feature = "tokio")]
impl tokio::io::AsyncRead for PipeReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut tokio::io::ReadBuf,
    ) -> Poll<io::Result<()>> {
        let dst = buf.initialize_unfilled();
        self.poll_read(cx, dst).map_ok(|read| {
            buf.advance(read);
        })
    }
}

#[cfg(feature = "futures")]
impl futures::io::AsyncRead for PipeReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.poll_read(cx, buf)
    }
}

impl Drop for PipeReader {
    fn drop(&mut self) {
        self.close().ok();
    }
}
