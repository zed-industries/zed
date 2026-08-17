use core::pin::Pin;

use futures::future::poll_fn;

#[cfg_attr(docsrs, doc(cfg(feature = "futures")))]
/// Adapter from `futures::io` traits.
pub struct FromFutures<T: ?Sized> {
    inner: T,
}

impl<T> FromFutures<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromFutures<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> crate::Io for FromFutures<T> {
    type Error = std::io::Error;
}

impl<T: futures::io::AsyncRead + Unpin + ?Sized> crate::asynch::Read for FromFutures<T> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_read(cx, buf)).await
    }
}

impl<T: futures::io::AsyncWrite + Unpin + ?Sized> crate::asynch::Write for FromFutures<T> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_write(cx, buf)).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        poll_fn(|cx| Pin::new(&mut self.inner).poll_flush(cx)).await
    }
}

impl<T: futures::io::AsyncSeek + Unpin + ?Sized> crate::asynch::Seek for FromFutures<T> {
    async fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error> {
        poll_fn(move |cx| Pin::new(&mut self.inner).poll_seek(cx, pos.into())).await
    }
}

// TODO: ToFutures.
// It's a bit tricky because futures::io is "stateless", while we're "stateful" (we
// return futures that borrow Self and get polled for the duration of the operation.)
// It can probably done by storing the futures in Self, with unsafe Pin hacks because
// we're a self-referential struct
