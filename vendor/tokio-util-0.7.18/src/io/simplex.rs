//! Unidirectional byte-oriented channel.

use crate::util::poll_proceed;

use bytes::Buf;
use bytes::BytesMut;
use futures_core::ready;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::IoSlice;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

type IoResult<T> = Result<T, IoError>;

const CLOSED_ERROR_MSG: &str = "simplex has been closed";

#[derive(Debug)]
struct Inner {
    /// `poll_write` will return [`Poll::Pending`] if the backpressure boundary is reached
    backpressure_boundary: usize,

    /// either [`Sender`] or [`Receiver`] is closed
    is_closed: bool,

    /// Waker used to wake the [`Receiver`]
    receiver_waker: Option<Waker>,

    /// Waker used to wake the [`Sender`]
    sender_waker: Option<Waker>,

    /// Buffer used to read and write data
    buf: BytesMut,
}

impl Inner {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            backpressure_boundary: capacity,
            is_closed: false,
            receiver_waker: None,
            sender_waker: None,
            buf: BytesMut::with_capacity(capacity),
        }
    }

    fn register_receiver_waker(&mut self, waker: &Waker) -> Option<Waker> {
        match self.receiver_waker.as_mut() {
            Some(old) if old.will_wake(waker) => None,
            _ => self.receiver_waker.replace(waker.clone()),
        }
    }

    fn register_sender_waker(&mut self, waker: &Waker) -> Option<Waker> {
        match self.sender_waker.as_mut() {
            Some(old) if old.will_wake(waker) => None,
            _ => self.sender_waker.replace(waker.clone()),
        }
    }

    fn take_receiver_waker(&mut self) -> Option<Waker> {
        self.receiver_waker.take()
    }

    fn take_sender_waker(&mut self) -> Option<Waker> {
        self.sender_waker.take()
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }

    fn close_receiver(&mut self) -> Option<Waker> {
        self.is_closed = true;
        self.take_sender_waker()
    }

    fn close_sender(&mut self) -> Option<Waker> {
        self.is_closed = true;
        self.take_receiver_waker()
    }
}

/// Receiver of the simplex channel.
///
/// # Cancellation safety
///
/// The `Receiver` is cancel safe. If it is used as the event in a
/// [`tokio::select!`](macro@tokio::select) statement and some other branch
/// completes first, it is guaranteed that no bytes were received on this
/// channel.
///
/// You can still read the remaining data from the buffer
/// even if the write half has been dropped.
/// See [`Sender::poll_shutdown`] and [`Sender::drop`] for more details.
#[derive(Debug)]
pub struct Receiver {
    inner: Arc<Mutex<Inner>>,
}

impl Drop for Receiver {
    /// This also wakes up the [`Sender`].
    fn drop(&mut self) {
        let maybe_waker = {
            let mut inner = self.inner.lock().unwrap();
            inner.close_receiver()
        };

        if let Some(waker) = maybe_waker {
            waker.wake();
        }
    }
}

impl AsyncRead for Receiver {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<IoResult<()>> {
        let coop = ready!(poll_proceed(cx));

        let mut inner = self.inner.lock().unwrap();

        let to_read = buf.remaining().min(inner.buf.remaining());
        if to_read == 0 {
            if inner.is_closed() || buf.remaining() == 0 {
                return Poll::Ready(Ok(()));
            }

            let old_waker = inner.register_receiver_waker(cx.waker());
            let maybe_waker = inner.take_sender_waker();

            // unlock before waking up and dropping old waker
            drop(inner);
            drop(old_waker);
            if let Some(waker) = maybe_waker {
                waker.wake();
            }
            return Poll::Pending;
        }

        // this is to avoid starving other tasks
        coop.made_progress();

        buf.put_slice(&inner.buf[..to_read]);
        inner.buf.advance(to_read);

        let waker = inner.take_sender_waker();
        drop(inner); // unlock before waking up
        if let Some(waker) = waker {
            waker.wake();
        }

        Poll::Ready(Ok(()))
    }
}

/// Sender of the simplex channel.
///
/// # Cancellation safety
///
/// The `Sender` is cancel safe. If it is used as the event in a
/// [`tokio::select!`](macro@tokio::select) statement and some other branch
/// completes first, it is guaranteed that no bytes were sent on this
/// channel.
///
/// # Shutdown
///
/// See [`Sender::poll_shutdown`].
#[derive(Debug)]
pub struct Sender {
    inner: Arc<Mutex<Inner>>,
}

impl Drop for Sender {
    /// This also wakes up the [`Receiver`].
    fn drop(&mut self) {
        let maybe_waker = {
            let mut inner = self.inner.lock().unwrap();
            inner.close_sender()
        };

        if let Some(waker) = maybe_waker {
            waker.wake();
        }
    }
}

impl AsyncWrite for Sender {
    /// # Errors
    ///
    /// This method will return [`IoErrorKind::BrokenPipe`]
    /// if the channel has been closed.
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<IoResult<usize>> {
        let coop = ready!(poll_proceed(cx));

        let mut inner = self.inner.lock().unwrap();

        if inner.is_closed() {
            return Poll::Ready(Err(IoError::new(IoErrorKind::BrokenPipe, CLOSED_ERROR_MSG)));
        }

        let free = inner
            .backpressure_boundary
            .checked_sub(inner.buf.len())
            .expect("backpressure boundary overflow");
        let to_write = buf.len().min(free);
        if to_write == 0 {
            if buf.is_empty() {
                return Poll::Ready(Ok(0));
            }

            let old_waker = inner.register_sender_waker(cx.waker());
            let waker = inner.take_receiver_waker();

            // unlock before waking up and dropping old waker
            drop(inner);
            drop(old_waker);
            if let Some(waker) = waker {
                waker.wake();
            }

            return Poll::Pending;
        }

        // this is to avoid starving other tasks
        coop.made_progress();

        inner.buf.extend_from_slice(&buf[..to_write]);

        let waker = inner.take_receiver_waker();
        drop(inner); // unlock before waking up
        if let Some(waker) = waker {
            waker.wake();
        }

        Poll::Ready(Ok(to_write))
    }

    /// # Errors
    ///
    /// This method will return [`IoErrorKind::BrokenPipe`]
    /// if the channel has been closed.
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<IoResult<()>> {
        let inner = self.inner.lock().unwrap();
        if inner.is_closed() {
            Poll::Ready(Err(IoError::new(IoErrorKind::BrokenPipe, CLOSED_ERROR_MSG)))
        } else {
            Poll::Ready(Ok(()))
        }
    }

    /// After returns [`Poll::Ready`], all the following call to
    /// [`Sender::poll_write`] and [`Sender::poll_flush`]
    /// will return error.
    ///
    /// The [`Receiver`] can still be used to read remaining data
    /// until all bytes have been consumed.
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<IoResult<()>> {
        let maybe_waker = {
            let mut inner = self.inner.lock().unwrap();
            inner.close_sender()
        };

        if let Some(waker) = maybe_waker {
            waker.wake();
        }

        Poll::Ready(Ok(()))
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, IoError>> {
        let coop = ready!(poll_proceed(cx));

        let mut inner = self.inner.lock().unwrap();
        if inner.is_closed() {
            return Poll::Ready(Err(IoError::new(IoErrorKind::BrokenPipe, CLOSED_ERROR_MSG)));
        }

        let free = inner
            .backpressure_boundary
            .checked_sub(inner.buf.len())
            .expect("backpressure boundary overflow");
        if free == 0 {
            let old_waker = inner.register_sender_waker(cx.waker());
            let maybe_waker = inner.take_receiver_waker();

            // unlock before waking up and dropping old waker
            drop(inner);
            drop(old_waker);
            if let Some(waker) = maybe_waker {
                waker.wake();
            }

            return Poll::Pending;
        }

        // this is to avoid starving other tasks
        coop.made_progress();

        let mut rem = free;
        for buf in bufs {
            if rem == 0 {
                break;
            }

            let to_write = buf.len().min(rem);
            if to_write == 0 {
                assert_ne!(rem, 0);
                assert_eq!(buf.len(), 0);
                continue;
            }

            inner.buf.extend_from_slice(&buf[..to_write]);
            rem -= to_write;
        }

        let waker = inner.take_receiver_waker();
        drop(inner); // unlock before waking up
        if let Some(waker) = waker {
            waker.wake();
        }

        Poll::Ready(Ok(free - rem))
    }
}

/// Create a simplex channel.
///
/// The `capacity` parameter specifies the maximum number of bytes that can be
/// stored in the channel without making the [`Sender::poll_write`]
/// return [`Poll::Pending`].
///
/// # Panics
///
/// This function will panic if `capacity` is zero.
pub fn new(capacity: usize) -> (Sender, Receiver) {
    assert_ne!(capacity, 0, "capacity must be greater than zero");

    let inner = Arc::new(Mutex::new(Inner::with_capacity(capacity)));
    let tx = Sender {
        inner: Arc::clone(&inner),
    };
    let rx = Receiver { inner };
    (tx, rx)
}
