use core::pin::Pin;
use core::task::Poll;

#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
/// Adapter from `tokio::io` traits.
pub struct FromTokio<T: ?Sized> {
    inner: T,
}

impl<T> FromTokio<T> {
    /// Create a new adapter.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consume the adapter, returning the inner object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: ?Sized> FromTokio<T> {
    /// Borrow the inner object.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mutably borrow the inner object.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ?Sized> crate::Io for FromTokio<T> {
    type Error = std::io::Error;
}

impl<T: tokio::io::AsyncRead + Unpin + ?Sized> crate::asynch::Read for FromTokio<T> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        poll_fn::poll_fn(|cx| {
            let mut buf = tokio::io::ReadBuf::new(buf);
            match Pin::new(&mut self.inner).poll_read(cx, &mut buf) {
                Poll::Ready(r) => match r {
                    Ok(()) => Poll::Ready(Ok(buf.filled().len())),
                    Err(e) => Poll::Ready(Err(e)),
                },
                Poll::Pending => Poll::Pending,
            }
        })
        .await
    }
}

impl<T: tokio::io::AsyncWrite + Unpin + ?Sized> crate::asynch::Write for FromTokio<T> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        poll_fn::poll_fn(|cx| Pin::new(&mut self.inner).poll_write(cx, buf)).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        poll_fn::poll_fn(|cx| Pin::new(&mut self.inner).poll_flush(cx)).await
    }
}

impl<T: tokio::io::AsyncSeek + Unpin + ?Sized> crate::asynch::Seek for FromTokio<T> {
    async fn seek(&mut self, pos: crate::SeekFrom) -> Result<u64, Self::Error> {
        // Note: `start_seek` can return an error if there is another seek in progress.
        // Therefor it is recommended to call `poll_complete` before any call to `start_seek`.
        poll_fn::poll_fn(|cx| Pin::new(&mut self.inner).poll_complete(cx)).await?;
        Pin::new(&mut self.inner).start_seek(pos.into())?;
        poll_fn::poll_fn(|cx| Pin::new(&mut self.inner).poll_complete(cx)).await
    }
}

// TODO: ToTokio.
// It's a bit tricky because tokio::io is "stateless", while we're "stateful" (we
// return futures that borrow Self and get polled for the duration of the operation.)
// It can probably done by storing the futures in Self, with unsafe Pin hacks because
// we're a self-referential struct

mod poll_fn {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    struct PollFn<F> {
        f: F,
    }

    impl<F> Unpin for PollFn<F> {}

    pub fn poll_fn<T, F>(f: F) -> impl Future<Output = T>
    where
        F: FnMut(&mut Context<'_>) -> Poll<T>,
    {
        PollFn { f }
    }

    impl<T, F> Future for PollFn<F>
    where
        F: FnMut(&mut Context<'_>) -> Poll<T>,
    {
        type Output = T;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
            (&mut self.f)(cx)
        }
    }
}
