use super::{Body, SizeHint};
use bytes::Buf;
use http::HeaderMap;
use std::{
    convert::Infallible,
    fmt,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

/// A body that is always empty.
pub struct Empty<D> {
    _marker: PhantomData<fn() -> D>,
}

impl<D> Empty<D> {
    /// Create a new `Empty`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D: Buf> Body for Empty<D> {
    type Data = D;
    type Error = Infallible;

    #[inline]
    fn poll_data(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        Poll::Ready(None)
    }

    #[inline]
    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }

    fn is_end_stream(&self) -> bool {
        true
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(0)
    }
}

impl<D> fmt::Debug for Empty<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Empty").finish()
    }
}

impl<D> Default for Empty<D> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<D> Clone for Empty<D> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<D> Copy for Empty<D> {}
