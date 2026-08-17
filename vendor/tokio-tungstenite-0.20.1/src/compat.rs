use log::*;
use std::{
    io::{Read, Write},
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::task;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tungstenite::Error as WsError;

pub(crate) enum ContextWaker {
    Read,
    Write,
}

#[derive(Debug)]
pub(crate) struct AllowStd<S> {
    inner: S,
    // We have the problem that external read operations (i.e. the Stream impl)
    // can trigger both read (AsyncRead) and write (AsyncWrite) operations on
    // the underyling stream. At the same time write operations (i.e. the Sink
    // impl) can trigger write operations (AsyncWrite) too.
    // Both the Stream and the Sink can be used on two different tasks, but it
    // is required that AsyncRead and AsyncWrite are only ever used by a single
    // task (or better: with a single waker) at a time.
    //
    // Doing otherwise would cause only the latest waker to be remembered, so
    // in our case either the Stream or the Sink impl would potentially wait
    // forever to be woken up because only the other one would've been woken
    // up.
    //
    // To solve this we implement a waker proxy that has two slots (one for
    // read, one for write) to store wakers. One waker proxy is always passed
    // to the AsyncRead, the other to AsyncWrite so that they will only ever
    // have to store a single waker, but internally we dispatch any wakeups to
    // up to two actual wakers (one from the Sink impl and one from the Stream
    // impl).
    //
    // write_waker_proxy is always used for AsyncWrite, read_waker_proxy for
    // AsyncRead. The read_waker slots of both are used for the Stream impl
    // (and handshaking), the write_waker slots for the Sink impl.
    write_waker_proxy: Arc<WakerProxy>,
    read_waker_proxy: Arc<WakerProxy>,
}

// Internal trait used only in the Handshake module for registering
// the waker for the context used during handshaking. We're using the
// read waker slot for this, but any would do.
//
// Don't ever use this from multiple tasks at the same time!
pub(crate) trait SetWaker {
    fn set_waker(&self, waker: &task::Waker);
}

impl<S> SetWaker for AllowStd<S> {
    fn set_waker(&self, waker: &task::Waker) {
        self.set_waker(ContextWaker::Read, waker);
    }
}

impl<S> AllowStd<S> {
    pub(crate) fn new(inner: S, waker: &task::Waker) -> Self {
        let res = Self {
            inner,
            write_waker_proxy: Default::default(),
            read_waker_proxy: Default::default(),
        };

        // Register the handshake waker as read waker for both proxies,
        // see also the SetWaker trait.
        res.write_waker_proxy.read_waker.register(waker);
        res.read_waker_proxy.read_waker.register(waker);

        res
    }

    // Set the read or write waker for our proxies.
    //
    // Read: this is only supposed to be called by read (or handshake) operations, i.e. the Stream
    // impl on the WebSocketStream.
    // Reading can also cause writes to happen, e.g. in case of Message::Ping handling.
    //
    // Write: this is only supposde to be called by write operations, i.e. the Sink impl on the
    // WebSocketStream.
    pub(crate) fn set_waker(&self, kind: ContextWaker, waker: &task::Waker) {
        match kind {
            ContextWaker::Read => {
                self.write_waker_proxy.read_waker.register(waker);
                self.read_waker_proxy.read_waker.register(waker);
            }
            ContextWaker::Write => {
                self.write_waker_proxy.write_waker.register(waker);
                self.read_waker_proxy.write_waker.register(waker);
            }
        }
    }
}

// Proxy Waker that we pass to the internal AsyncRead/Write of the
// stream underlying the websocket. We have two slots here for the
// actual wakers to allow external read operations to trigger both
// reads and writes, and the same for writes.
#[derive(Debug, Default)]
struct WakerProxy {
    read_waker: task::AtomicWaker,
    write_waker: task::AtomicWaker,
}

impl task::ArcWake for WakerProxy {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.read_waker.wake();
        arc_self.write_waker.wake();
    }
}

impl<S> AllowStd<S>
where
    S: Unpin,
{
    fn with_context<F, R>(&mut self, kind: ContextWaker, f: F) -> Poll<std::io::Result<R>>
    where
        F: FnOnce(&mut Context<'_>, Pin<&mut S>) -> Poll<std::io::Result<R>>,
    {
        trace!("{}:{} AllowStd.with_context", file!(), line!());
        let waker = match kind {
            ContextWaker::Read => task::waker_ref(&self.read_waker_proxy),
            ContextWaker::Write => task::waker_ref(&self.write_waker_proxy),
        };
        let mut context = task::Context::from_waker(&waker);
        f(&mut context, Pin::new(&mut self.inner))
    }

    pub(crate) fn get_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    pub(crate) fn get_ref(&self) -> &S {
        &self.inner
    }
}

impl<S> Read for AllowStd<S>
where
    S: AsyncRead + Unpin,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        trace!("{}:{} Read.read", file!(), line!());
        let mut buf = ReadBuf::new(buf);
        match self.with_context(ContextWaker::Read, |ctx, stream| {
            trace!("{}:{} Read.with_context read -> poll_read", file!(), line!());
            stream.poll_read(ctx, &mut buf)
        }) {
            Poll::Ready(Ok(_)) => Ok(buf.filled().len()),
            Poll::Ready(Err(err)) => Err(err),
            Poll::Pending => Err(std::io::Error::from(std::io::ErrorKind::WouldBlock)),
        }
    }
}

impl<S> Write for AllowStd<S>
where
    S: AsyncWrite + Unpin,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        trace!("{}:{} Write.write", file!(), line!());
        match self.with_context(ContextWaker::Write, |ctx, stream| {
            trace!("{}:{} Write.with_context write -> poll_write", file!(), line!());
            stream.poll_write(ctx, buf)
        }) {
            Poll::Ready(r) => r,
            Poll::Pending => Err(std::io::Error::from(std::io::ErrorKind::WouldBlock)),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        trace!("{}:{} Write.flush", file!(), line!());
        match self.with_context(ContextWaker::Write, |ctx, stream| {
            trace!("{}:{} Write.with_context flush -> poll_flush", file!(), line!());
            stream.poll_flush(ctx)
        }) {
            Poll::Ready(r) => r,
            Poll::Pending => Err(std::io::Error::from(std::io::ErrorKind::WouldBlock)),
        }
    }
}

pub(crate) fn cvt<T>(r: Result<T, WsError>) -> Poll<Result<T, WsError>> {
    match r {
        Ok(v) => Poll::Ready(Ok(v)),
        Err(WsError::Io(ref e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
            trace!("WouldBlock");
            Poll::Pending
        }
        Err(e) => Poll::Ready(Err(e)),
    }
}
