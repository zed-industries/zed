use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

use crate::future;
use crate::io::{self, Read};
use crate::task::{spawn_blocking, Context, JoinHandle, Poll};
use crate::utils::Context as _;

/// Constructs a new handle to the standard input of the current process.
///
/// This function is an async version of [`std::io::stdin`].
///
/// [`std::io::stdin`]: https://doc.rust-lang.org/std/io/fn.stdin.html
///
/// ### Note: Windows Portability Consideration
///
/// When operating in a console, the Windows implementation of this stream does not support
/// non-UTF-8 byte sequences. Attempting to read bytes that are not valid UTF-8 will return
/// an error.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
///
/// let stdin = io::stdin();
/// let mut line = String::new();
/// stdin.read_line(&mut line).await?;
/// #
/// # Ok(()) }) }
/// ```
pub fn stdin() -> Stdin {
    Stdin(Mutex::new(State::Idle(Some(Inner {
        stdin: std::io::stdin(),
        line: String::new(),
        buf: Vec::new(),
        last_op: None,
    }))))
}

/// A handle to the standard input of the current process.
///
/// This reader is created by the [`stdin`] function. See its documentation for
/// more.
///
/// ### Note: Windows Portability Consideration
///
/// When operating in a console, the Windows implementation of this stream does not support
/// non-UTF-8 byte sequences. Attempting to read bytes that are not valid UTF-8 will return
/// an error.
///
/// [`stdin`]: fn.stdin.html
#[derive(Debug)]
pub struct Stdin(Mutex<State>);

/// The state of the asynchronous stdin.
///
/// The stdin can be either idle or busy performing an asynchronous operation.
#[derive(Debug)]
enum State {
    /// The stdin is idle.
    Idle(Option<Inner>),

    /// The stdin is blocked on an asynchronous operation.
    ///
    /// Awaiting this operation will result in the new state of the stdin.
    Busy(JoinHandle<State>),
}

/// Inner representation of the asynchronous stdin.
#[derive(Debug)]
struct Inner {
    /// The blocking stdin handle.
    stdin: std::io::Stdin,

    /// The line buffer.
    line: String,

    /// The read buffer.
    buf: Vec<u8>,

    /// The result of the last asynchronous operation on the stdin.
    last_op: Option<Operation>,
}

/// Possible results of an asynchronous operation on the stdin.
#[derive(Debug)]
enum Operation {
    ReadLine(io::Result<usize>),
    Read(io::Result<usize>),
}

impl Stdin {
    /// Reads a line of input into the specified buffer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::io;
    ///
    /// let stdin = io::stdin();
    /// let mut line = String::new();
    /// stdin.read_line(&mut line).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn read_line(&self, buf: &mut String) -> io::Result<usize> {
        future::poll_fn(|cx| {
            let state = &mut *self.0.lock().unwrap();

            loop {
                match state {
                    State::Idle(opt) => {
                        let inner = opt.as_mut().unwrap();

                        // Check if the operation has completed.
                        if let Some(Operation::ReadLine(res)) = inner.last_op.take() {
                            let n = res?;

                            // Copy the read data into the buffer and return.
                            buf.push_str(&inner.line);
                            return Poll::Ready(Ok(n));
                        } else {
                            let mut inner = opt.take().unwrap();

                            // Start the operation asynchronously.
                            *state = State::Busy(spawn_blocking(move || {
                                inner.line.clear();
                                let res = inner.stdin.read_line(&mut inner.line);
                                inner.last_op = Some(Operation::ReadLine(res));
                                State::Idle(Some(inner))
                            }));
                        }
                    }
                    // Poll the asynchronous operation the stdin is currently blocked on.
                    State::Busy(task) => *state = futures_core::ready!(Pin::new(task).poll(cx)),
                }
            }
        })
        .await
        .context(|| String::from("could not read line on stdin"))
    }
}

impl Read for Stdin {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut state_guard = self.0.lock().unwrap();
        let state = &mut *state_guard;

        loop {
            match state {
                State::Idle(opt) => {
                    let inner = opt.as_mut().unwrap();

                    // Check if the operation has completed.
                    if let Some(Operation::Read(res)) = inner.last_op.take() {
                        let n = res?;

                        // If more data was read than fits into the buffer, let's retry the read
                        // operation.
                        if n <= buf.len() {
                            // Copy the read data into the buffer and return.
                            buf[..n].copy_from_slice(&inner.buf[..n]);
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

                        // Start the operation asynchronously.
                        *state = State::Busy(spawn_blocking(move || {
                            let res = std::io::Read::read(&mut inner.stdin, &mut inner.buf);
                            inner.last_op = Some(Operation::Read(res));
                            State::Idle(Some(inner))
                        }));
                    }
                }
                // Poll the asynchronous operation the stdin is currently blocked on.
                State::Busy(task) => *state = futures_core::ready!(Pin::new(task).poll(cx)),
            }
        }
    }
}

cfg_unix! {
    use crate::os::unix::io::{AsRawFd, RawFd};

    impl AsRawFd for Stdin {
        fn as_raw_fd(&self) -> RawFd {
            std::io::stdin().as_raw_fd()
        }
    }

    cfg_io_safety! {
        use crate::os::unix::io::{AsFd, BorrowedFd};

        impl AsFd for Stdin {
            fn as_fd(&self) -> BorrowedFd<'_> {
                unsafe {
                    BorrowedFd::borrow_raw(std::io::stdin().as_raw_fd())
                }
            }
        }
    }
}

cfg_windows! {
    use crate::os::windows::io::{AsRawHandle, RawHandle};

    impl AsRawHandle for Stdin {
        fn as_raw_handle(&self) -> RawHandle {
            std::io::stdin().as_raw_handle()
        }
    }

    cfg_io_safety! {
        use crate::os::windows::io::{AsHandle, BorrowedHandle};

        impl AsHandle for Stdin {
            fn as_handle(&self) -> BorrowedHandle<'_> {
                unsafe {
                    BorrowedHandle::borrow_raw(std::io::stdin().as_raw_handle())
                }
            }
        }
    }
}
