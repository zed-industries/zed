use crate::cli::{IsTerminal, StdinStream, StdoutStream};
use crate::p2;
use bytes::Bytes;
use std::mem;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, ready};
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::sync::{Mutex, OwnedMutexGuard};
use wasmtime_wasi_io::streams::{InputStream, OutputStream};

trait SharedHandleReady: Send + Sync + 'static {
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<()>;
}

impl SharedHandleReady for p2::pipe::AsyncWriteStream {
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        <Self>::poll_ready(self, cx)
    }
}

impl SharedHandleReady for p2::pipe::AsyncReadStream {
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        <Self>::poll_ready(self, cx)
    }
}

/// An impl of [`StdinStream`] built on top of [`AsyncRead`].
//
// Note the usage of `tokio::sync::Mutex` here as opposed to a
// `std::sync::Mutex`. This is intentionally done to implement the `Pollable`
// variant of this trait. Note that in doing so we're left with the quandry of
// how to implement methods of `InputStream` since those methods are not
// `async`. They're currently implemented with `try_lock`, which then raises the
// question of what to do on contention. Currently traps are returned.
//
// Why should it be ok to return a trap? In general concurrency/contention
// shouldn't return a trap since it should be able to happen normally. The
// current assumption, though, is that WASI stdin/stdout streams are special
// enough that the contention case should never come up in practice. Currently
// in WASI there is no actually concurrency, there's just the items in a single
// `Store` and that store owns all of its I/O in a single Tokio task. There's no
// means to actually spawn multiple Tokio tasks that use the same store. This
// means at the very least that there's zero parallelism. Due to the lack of
// multiple tasks that also means that there's no concurrency either.
//
// This `AsyncStdinStream` wrapper is only intended to be used by the WASI
// bindings themselves. It's possible for the host to take this and work with it
// on its own task, but that's niche enough it's not designed for.
//
// Overall that means that the guest is either calling `Pollable` or
// `InputStream` methods. This means that there should never be contention
// between the two at this time. This may all change in the future with WASI
// 0.3, but perhaps we'll have a better story for stdio at that time (see the
// doc block on the `OutputStream` impl below)
pub struct AsyncStdinStream(Arc<Mutex<p2::pipe::AsyncReadStream>>);

impl AsyncStdinStream {
    pub fn new(s: impl AsyncRead + Send + Sync + 'static) -> Self {
        Self(Arc::new(Mutex::new(p2::pipe::AsyncReadStream::new(s))))
    }
}

impl StdinStream for AsyncStdinStream {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(Self(self.0.clone()))
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(StdioHandle::Ready(self.0.clone()))
    }
}

impl IsTerminal for AsyncStdinStream {
    fn is_terminal(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
impl InputStream for AsyncStdinStream {
    fn read(&mut self, size: usize) -> Result<bytes::Bytes, p2::StreamError> {
        match self.0.try_lock() {
            Ok(mut stream) => stream.read(size),
            Err(_) => Err(p2::StreamError::trap("concurrent reads are not supported")),
        }
    }
    fn skip(&mut self, size: usize) -> Result<usize, p2::StreamError> {
        match self.0.try_lock() {
            Ok(mut stream) => stream.skip(size),
            Err(_) => Err(p2::StreamError::trap("concurrent skips are not supported")),
        }
    }
    async fn cancel(&mut self) {
        // Cancel the inner stream if we're the last reference to it:
        if let Some(mutex) = Arc::get_mut(&mut self.0) {
            match mutex.try_lock() {
                Ok(mut stream) => stream.cancel().await,
                Err(_) => {}
            }
        }
    }
}

#[async_trait::async_trait]
impl p2::Pollable for AsyncStdinStream {
    async fn ready(&mut self) {
        self.0.lock().await.ready().await
    }
}

impl AsyncRead for StdioHandle<p2::pipe::AsyncReadStream> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match ready!(self.as_mut().poll(cx, |g| g.read(buf.remaining()))) {
            Some(Ok(bytes)) => {
                buf.put_slice(&bytes);
                Poll::Ready(Ok(()))
            }
            Some(Err(e)) => Poll::Ready(Err(e)),
            // If the guard can't be acquired that means that this stream is
            // closed, so return that we're ready without filling in data.
            None => Poll::Ready(Ok(())),
        }
    }
}

/// A wrapper of [`crate::p2::pipe::AsyncWriteStream`] that implements
/// [`StdoutStream`]. Note that the [`OutputStream`] impl for this is not
/// correct when used for interleaved async IO.
//
// Note that the use of `tokio::sync::Mutex` here is intentional, in addition to
// the `try_lock()` calls below in the implementation of `OutputStream`. For
// more information see the documentation on `AsyncStdinStream`.
pub struct AsyncStdoutStream(Arc<Mutex<p2::pipe::AsyncWriteStream>>);

impl AsyncStdoutStream {
    pub fn new(budget: usize, s: impl AsyncWrite + Send + Sync + 'static) -> Self {
        Self(Arc::new(Mutex::new(p2::pipe::AsyncWriteStream::new(
            budget, s,
        ))))
    }
}

impl StdoutStream for AsyncStdoutStream {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(Self(self.0.clone()))
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(StdioHandle::Ready(self.0.clone()))
    }
}

impl IsTerminal for AsyncStdoutStream {
    fn is_terminal(&self) -> bool {
        false
    }
}

// This implementation is known to be bogus. All check-writes and writes are
// directed at the same underlying stream. The check-write/write protocol does
// require the size returned by a check-write to be accepted by write, even if
// other side-effects happen between those calls, and this implementation
// permits another view (created by StdoutStream::stream()) of the same
// underlying stream to accept a write which will invalidate a prior
// check-write of another view.
// Ultimately, the Std{in,out}Stream::stream() methods exist because many
// different places in a linked component (which may itself contain many
// modules) may need to access stdio without any coordination to keep those
// accesses all using pointing to the same resource. So, we allow many
// resources to be created. We have the reasonable expectation that programs
// won't attempt to interleave async IO from these disparate uses of stdio.
// If that expectation doesn't turn out to be true, and you find yourself at
// this comment to correct it: sorry about that.
#[async_trait::async_trait]
impl OutputStream for AsyncStdoutStream {
    fn check_write(&mut self) -> Result<usize, p2::StreamError> {
        match self.0.try_lock() {
            Ok(mut stream) => stream.check_write(),
            Err(_) => Err(p2::StreamError::trap("concurrent writes are not supported")),
        }
    }
    fn write(&mut self, bytes: Bytes) -> Result<(), p2::StreamError> {
        match self.0.try_lock() {
            Ok(mut stream) => stream.write(bytes),
            Err(_) => Err(p2::StreamError::trap("concurrent writes not supported yet")),
        }
    }
    fn flush(&mut self) -> Result<(), p2::StreamError> {
        match self.0.try_lock() {
            Ok(mut stream) => stream.flush(),
            Err(_) => Err(p2::StreamError::trap(
                "concurrent flushes not supported yet",
            )),
        }
    }
    async fn cancel(&mut self) {
        // Cancel the inner stream if we're the last reference to it:
        if let Some(mutex) = Arc::get_mut(&mut self.0) {
            match mutex.try_lock() {
                Ok(mut stream) => stream.cancel().await,
                Err(_) => {}
            }
        }
    }
}

#[async_trait::async_trait]
impl p2::Pollable for AsyncStdoutStream {
    async fn ready(&mut self) {
        self.0.lock().await.ready().await
    }
}

impl AsyncWrite for StdioHandle<p2::pipe::AsyncWriteStream> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match ready!(self.poll(cx, |i| i.write(Bytes::copy_from_slice(buf)))) {
            Some(Ok(())) => Poll::Ready(Ok(buf.len())),
            Some(Err(e)) => Poll::Ready(Err(e)),
            None => Poll::Ready(Ok(0)),
        }
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match ready!(self.poll(cx, |i| i.flush())) {
            Some(result) => Poll::Ready(result),
            None => Poll::Ready(Ok(())),
        }
    }
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// State necessary for effectively transforming `Arc<Mutex<dyn
/// {Input,Output}Stream>>` into `Async{Read,Write}`.
///
/// This is a beast and inefficient. It should get the job done in theory but
/// one must truly ask oneself at some point "but at what cost".
///
/// More seriously, it's unclear if this is the best way to transform a single
/// `AsyncRead` into a "multiple `AsyncRead`". This certainly is an attempt and
/// the hope is that everything here is private enough that we can refactor as
/// necessary in the future without causing much churn.
enum StdioHandle<S> {
    Ready(Arc<Mutex<S>>),
    Locking(Box<dyn Future<Output = OwnedMutexGuard<S>> + Send + Sync>),
    Locked(OwnedMutexGuard<S>),
    Closed,
}

impl<S> StdioHandle<S>
where
    S: SharedHandleReady,
{
    fn poll<T>(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        op: impl FnOnce(&mut S) -> p2::StreamResult<T>,
    ) -> Poll<Option<io::Result<T>>> {
        // If we don't currently have the lock on this handle, initiate the
        // lock acquisition.
        if let StdioHandle::Ready(lock) = &*self {
            self.set(StdioHandle::Locking(Box::new(lock.clone().lock_owned())));
        }

        // If we're in the process of locking this handle, wait for that to
        // finish.
        if let Some(lock) = self.as_mut().as_locking() {
            let guard = ready!(lock.poll(cx));
            self.set(StdioHandle::Locked(guard));
        }

        let mut guard = match self.as_mut().take_guard() {
            Some(guard) => guard,
            // If the guard can't be acquired that means that this stream is
            // closed, so return that we're ready without filling in data.
            None => return Poll::Ready(None),
        };

        // Wait for our locked stream to be ready, resetting to the "locked"
        // state if it's not quite ready yet.
        match guard.poll_ready(cx) {
            Poll::Ready(()) => {}

            // If the read isn't ready yet then restore our "locked" state
            // since we haven't finished, then return pending.
            Poll::Pending => {
                self.set(StdioHandle::Locked(guard));
                return Poll::Pending;
            }
        }

        // Perform the I/O and delegate on the result.
        match op(&mut guard) {
            // The I/O succeeded so relinquish the lock on this stream by
            // transitioning back to the "Ready" state.
            Ok(result) => {
                self.set(StdioHandle::Ready(OwnedMutexGuard::mutex(&guard).clone()));
                Poll::Ready(Some(Ok(result)))
            }

            // The stream is closed, and `take_guard` above already set the
            // closed state, so return nothing indicating the closure.
            Err(p2::StreamError::Closed) => Poll::Ready(None),

            // The stream failed so propagate the error. Errors should only
            // come from the underlying I/O object and thus should cast
            // successfully. Additionally `take_guard` replaced our state
            // with "closed" above which is the desired state at this point.
            Err(p2::StreamError::LastOperationFailed(e)) => {
                Poll::Ready(Some(Err(e.downcast().unwrap())))
            }

            // Shouldn't be possible to produce a trap here.
            Err(p2::StreamError::Trap(_)) => unreachable!(),
        }
    }

    fn as_locking(
        self: Pin<&mut Self>,
    ) -> Option<Pin<&mut dyn Future<Output = OwnedMutexGuard<S>>>> {
        // SAFETY: this is a pin-projection from `self` into the `Locking`
        // field.
        unsafe {
            match self.get_unchecked_mut() {
                StdioHandle::Locking(future) => Some(Pin::new_unchecked(&mut **future)),
                _ => None,
            }
        }
    }

    fn take_guard(self: Pin<&mut Self>) -> Option<OwnedMutexGuard<S>> {
        if !matches!(*self, StdioHandle::Locked(_)) {
            return None;
        }
        // SAFETY: the `Locked` arm is safe to move as it's an invariant of this
        // type that it's not pinned.
        unsafe {
            match mem::replace(self.get_unchecked_mut(), StdioHandle::Closed) {
                StdioHandle::Locked(guard) => Some(guard),
                _ => unreachable!(),
            }
        }
    }
}
