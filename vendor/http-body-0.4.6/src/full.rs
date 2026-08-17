use crate::{Body, SizeHint};
use bytes::{Buf, Bytes};
use http::HeaderMap;
use pin_project_lite::pin_project;
use std::borrow::Cow;
use std::convert::{Infallible, TryFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// A body that consists of a single chunk.
    #[derive(Clone, Copy, Debug)]
    pub struct Full<D> {
        data: Option<D>,
    }
}

impl<D> Full<D>
where
    D: Buf,
{
    /// Create a new `Full`.
    pub fn new(data: D) -> Self {
        let data = if data.has_remaining() {
            Some(data)
        } else {
            None
        };
        Full { data }
    }
}

impl<D> Body for Full<D>
where
    D: Buf,
{
    type Data = D;
    type Error = Infallible;

    fn poll_data(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<D, Self::Error>>> {
        Poll::Ready(self.data.take().map(Ok))
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }

    fn is_end_stream(&self) -> bool {
        self.data.is_none()
    }

    fn size_hint(&self) -> SizeHint {
        self.data
            .as_ref()
            .map(|data| SizeHint::with_exact(u64::try_from(data.remaining()).unwrap()))
            .unwrap_or_else(|| SizeHint::with_exact(0))
    }
}

impl<D> Default for Full<D>
where
    D: Buf,
{
    /// Create an empty `Full`.
    fn default() -> Self {
        Full { data: None }
    }
}

impl<D> From<Bytes> for Full<D>
where
    D: Buf + From<Bytes>,
{
    fn from(bytes: Bytes) -> Self {
        Full::new(D::from(bytes))
    }
}

impl<D> From<Vec<u8>> for Full<D>
where
    D: Buf + From<Vec<u8>>,
{
    fn from(vec: Vec<u8>) -> Self {
        Full::new(D::from(vec))
    }
}

impl<D> From<&'static [u8]> for Full<D>
where
    D: Buf + From<&'static [u8]>,
{
    fn from(slice: &'static [u8]) -> Self {
        Full::new(D::from(slice))
    }
}

impl<D, B> From<Cow<'static, B>> for Full<D>
where
    D: Buf + From<&'static B> + From<B::Owned>,
    B: ToOwned + ?Sized,
{
    fn from(cow: Cow<'static, B>) -> Self {
        match cow {
            Cow::Borrowed(b) => Full::new(D::from(b)),
            Cow::Owned(o) => Full::new(D::from(o)),
        }
    }
}

impl<D> From<String> for Full<D>
where
    D: Buf + From<String>,
{
    fn from(s: String) -> Self {
        Full::new(D::from(s))
    }
}

impl<D> From<&'static str> for Full<D>
where
    D: Buf + From<&'static str>,
{
    fn from(slice: &'static str) -> Self {
        Full::new(D::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn full_returns_some() {
        let mut full = Full::new(&b"hello"[..]);
        assert_eq!(full.size_hint().exact(), Some(b"hello".len() as u64));
        assert_eq!(full.data().await, Some(Ok(&b"hello"[..])));
        assert!(full.data().await.is_none());
    }

    #[tokio::test]
    async fn empty_full_returns_none() {
        assert!(Full::<&[u8]>::default().data().await.is_none());
        assert!(Full::new(&b""[..]).data().await.is_none());
    }
}
