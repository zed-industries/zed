//! A signal notifier that uses an asynchronous pipe.

use crate::Signal;

use async_io::Async;
use futures_core::ready;
use futures_io::AsyncRead;

use std::io::{self, prelude::*};
use std::mem;
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, RawFd};
use std::os::unix::net::UnixStream;
use std::pin::Pin;
use std::task::{Context, Poll};

const BUFFER_LEN: usize = mem::size_of::<std::os::raw::c_int>();

/// The notifier that uses an asynchronous pipe.
#[derive(Debug)]
pub(super) struct Notifier {
    /// The read end of the signal pipe.
    read: Async<UnixStream>,

    /// The write end of the signal pipe.
    write: UnixStream,
}

impl Notifier {
    /// Create a new signal notifier.
    pub(super) fn new() -> io::Result<Self> {
        let (read, write) = UnixStream::pair()?;
        let read = Async::new(read)?;
        write.set_nonblocking(true)?;

        Ok(Self { read, write })
    }

    /// Add a signal to the notifier.
    ///
    /// Returns a closure to be passed to signal-hook.
    pub(super) fn add_signal(
        &mut self,
        signal: Signal,
    ) -> io::Result<impl Fn() + Send + Sync + 'static> {
        let number = signal.number();
        let write = self.write.try_clone()?;

        Ok(move || {
            // SAFETY: to_ne_bytes() and write() are both signal safe.
            let bytes = number.to_ne_bytes();
            let _ = (&write).write(&bytes);
        })
    }

    /// Remove a signal from the notifier.
    pub(super) fn remove_signal(&mut self, _signal: Signal) -> io::Result<()> {
        Ok(())
    }

    /// Get the next signal.
    pub(super) fn poll_next(&self, cx: &mut Context<'_>) -> Poll<io::Result<Signal>> {
        let mut buffer = [0; BUFFER_LEN];
        let mut buffer_len = 0;

        // Read into the buffer.
        loop {
            if buffer_len >= BUFFER_LEN {
                break;
            }

            // Try to fill up the entire buffer.
            let buf_range = buffer_len..BUFFER_LEN;
            let res = ready!(Pin::new(&mut &self.read).poll_read(cx, &mut buffer[buf_range]));

            match res {
                Ok(0) => return Poll::Ready(Err(io::Error::from(io::ErrorKind::UnexpectedEof))),
                Ok(n) => buffer_len += n,
                Err(e) => return Poll::Ready(Err(e)),
            }
        }

        // Convert the buffer into a signal number.
        let number = std::os::raw::c_int::from_ne_bytes(buffer);

        // Convert the signal number into a signal.
        let signal = match Signal::from_number(number) {
            Some(signal) => signal,
            None => return Poll::Ready(Err(io::Error::from(io::ErrorKind::InvalidData))),
        };

        // Return the signal.
        Poll::Ready(Ok(signal))
    }
}

impl AsRawFd for Notifier {
    fn as_raw_fd(&self) -> RawFd {
        self.read.as_raw_fd()
    }
}

impl AsFd for Notifier {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.read.as_fd()
    }
}
