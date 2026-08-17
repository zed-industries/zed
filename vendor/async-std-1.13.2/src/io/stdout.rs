use std::pin::Pin;
use std::sync::Mutex;
use std::future::Future;

use crate::io::{self, Write};
use crate::task::{spawn_blocking, Context, JoinHandle, Poll};

/// Constructs a new handle to the standard output of the current process.
///
/// This function is an async version of [`std::io::stdout`].
///
/// [`std::io::stdout`]: https://doc.rust-lang.org/std/io/fn.stdout.html
///
/// ### Note: Windows Portability Consideration
///
/// When operating in a console, the Windows implementation of this stream does not support
/// non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return
/// an error.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
/// use async_std::prelude::*;
///
/// let mut stdout = io::stdout();
/// stdout.write_all(b"Hello, world!").await?;
/// #
/// # Ok(()) }) }
/// ```
pub fn stdout() -> Stdout {
    Stdout(Mutex::new(State::Idle(Some(Inner {
        stdout: std::io::stdout(),
        buf: Vec::new(),
        last_op: None,
    }))))
}

/// A handle to the standard output of the current process.
///
/// This writer is created by the [`stdout`] function. See its documentation
/// for more.
///
/// ### Note: Windows Portability Consideration
///
/// When operating in a console, the Windows implementation of this stream does not support
/// non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return
/// an error.
///
/// [`stdout`]: fn.stdout.html
#[derive(Debug)]
pub struct Stdout(Mutex<State>);

/// The state of the asynchronous stdout.
///
/// The stdout can be either idle or busy performing an asynchronous operation.
#[derive(Debug)]
enum State {
    /// The stdout is idle.
    Idle(Option<Inner>),

    /// The stdout is blocked on an asynchronous operation.
    ///
    /// Awaiting this operation will result in the new state of the stdout.
    Busy(JoinHandle<State>),
}

/// Inner representation of the asynchronous stdout.
#[derive(Debug)]
struct Inner {
    /// The blocking stdout handle.
    stdout: std::io::Stdout,

    /// The write buffer.
    buf: Vec<u8>,

    /// The result of the last asynchronous operation on the stdout.
    last_op: Option<Operation>,
}

/// Possible results of an asynchronous operation on the stdout.
#[derive(Debug)]
enum Operation {
    Write(io::Result<usize>),
    Flush(io::Result<()>),
}

impl Write for Stdout {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut state_guard = self.0.lock().unwrap();
        let state = &mut *state_guard;

        loop {
            match state {
                State::Idle(opt) => {
                    let inner = opt.as_mut().unwrap();

                    // Check if the operation has completed.
                    if let Some(Operation::Write(res)) = inner.last_op.take() {
                        let n = res?;

                        // If more data was written than is available in the buffer, let's retry
                        // the write operation.
                        if n <= buf.len() {
                            return Poll::Ready(Ok(n));
                        }
                    } else {
                        let mut inner = opt.take().unwrap();

                        // Set the length of the inner buffer to the length of the provided buffer.
                        if inner.buf.len() < buf.len() {
                            inner.buf.reserve(buf.len() - inner.buf.len());
                        }
                        unsafe {
                            inner.buf.set_len(buf.len());
                        }

                        // Copy the data to write into the inner buffer.
                        inner.buf[..buf.len()].copy_from_slice(buf);

                        // Start the operation asynchronously.
                        *state = State::Busy(spawn_blocking(move || {
                            let res = std::io::Write::write(&mut inner.stdout, &inner.buf);
                            inner.last_op = Some(Operation::Write(res));
                            State::Idle(Some(inner))
                        }));
                    }
                }
                // Poll the asynchronous operation the stdout is currently blocked on.
                State::Busy(task) => *state = futures_core::ready!(Pin::new(task).poll(cx)),
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut state_guard = self.0.lock().unwrap();
        let state = &mut *state_guard;

        loop {
            match state {
                State::Idle(opt) => {
                    let inner = opt.as_mut().unwrap();

                    // Check if the operation has completed.
                    if let Some(Operation::Flush(res)) = inner.last_op.take() {
                        return Poll::Ready(res);
                    } else {
                        let mut inner = opt.take().unwrap();

                        // Start the operation asynchronously.
                        *state = State::Busy(spawn_blocking(move || {
                            let res = std::io::Write::flush(&mut inner.stdout);
                            inner.last_op = Some(Operation::Flush(res));
                            State::Idle(Some(inner))
                        }));
                    }
                }
                // Poll the asynchronous operation the stdout is currently blocked on.
                State::Busy(task) => *state = futures_core::ready!(Pin::new(task).poll(cx)),
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.poll_flush(cx)
    }
}

cfg_unix! {
    use crate::os::unix::io::{AsRawFd, RawFd};

    impl AsRawFd for Stdout {
        fn as_raw_fd(&self) -> RawFd {
            std::io::stdout().as_raw_fd()
        }
    }

    cfg_io_safety! {
        use crate::os::unix::io::{AsFd, BorrowedFd};

        impl AsFd for Stdout {
            fn as_fd(&self) -> BorrowedFd<'_> {
                unsafe {
                    BorrowedFd::borrow_raw(std::io::stdout().as_raw_fd())
                }
            }
        }
    }
}

cfg_windows! {
    use crate::os::windows::io::{AsRawHandle, RawHandle};

    impl AsRawHandle for Stdout {
        fn as_raw_handle(&self) -> RawHandle {
            std::io::stdout().as_raw_handle()
        }
    }

    cfg_io_safety! {
        use crate::os::windows::io::{AsHandle, BorrowedHandle};

        impl AsHandle for Stdout {
            fn as_handle(&self) -> BorrowedHandle<'_> {
                unsafe {
                    BorrowedHandle::borrow_raw(std::io::stdout().as_raw_handle())
                }
            }
        }
    }
}
