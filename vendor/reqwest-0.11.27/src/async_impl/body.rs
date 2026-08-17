use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures_core::Stream;
use http_body::Body as HttpBody;
use pin_project_lite::pin_project;
use sync_wrapper::SyncWrapper;
#[cfg(feature = "stream")]
use tokio::fs::File;
use tokio::time::Sleep;
#[cfg(feature = "stream")]
use tokio_util::io::ReaderStream;

/// An asynchronous request body.
pub struct Body {
    inner: Inner,
}

// The `Stream` trait isn't stable, so the impl isn't public.
pub(crate) struct ImplStream(Body);

enum Inner {
    Reusable(Bytes),
    Streaming {
        body: Pin<
            Box<
                dyn HttpBody<Data = Bytes, Error = Box<dyn std::error::Error + Send + Sync>>
                    + Send
                    + Sync,
            >,
        >,
        timeout: Option<Pin<Box<Sleep>>>,
    },
}

pin_project! {
    struct WrapStream<S> {
        #[pin]
        inner: SyncWrapper<S>,
    }
}

struct WrapHyper(hyper::Body);

impl Body {
    /// Returns a reference to the internal data of the `Body`.
    ///
    /// `None` is returned, if the underlying data is a stream.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match &self.inner {
            Inner::Reusable(bytes) => Some(bytes.as_ref()),
            Inner::Streaming { .. } => None,
        }
    }

    /// Wrap a futures `Stream` in a box inside `Body`.
    ///
    /// # Example
    ///
    /// ```
    /// # use reqwest::Body;
    /// # use futures_util;
    /// # fn main() {
    /// let chunks: Vec<Result<_, ::std::io::Error>> = vec![
    ///     Ok("hello"),
    ///     Ok(" "),
    ///     Ok("world"),
    /// ];
    ///
    /// let stream = futures_util::stream::iter(chunks);
    ///
    /// let body = Body::wrap_stream(stream);
    /// # }
    /// ```
    ///
    /// # Optional
    ///
    /// This requires the `stream` feature to be enabled.
    #[cfg(feature = "stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    pub fn wrap_stream<S>(stream: S) -> Body
    where
        S: futures_core::stream::TryStream + Send + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        Bytes: From<S::Ok>,
    {
        Body::stream(stream)
    }

    pub(crate) fn stream<S>(stream: S) -> Body
    where
        S: futures_core::stream::TryStream + Send + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        Bytes: From<S::Ok>,
    {
        use futures_util::TryStreamExt;

        let body = Box::pin(WrapStream {
            inner: SyncWrapper::new(stream.map_ok(Bytes::from).map_err(Into::into)),
        });
        Body {
            inner: Inner::Streaming {
                body,
                timeout: None,
            },
        }
    }

    pub(crate) fn response(body: hyper::Body, timeout: Option<Pin<Box<Sleep>>>) -> Body {
        Body {
            inner: Inner::Streaming {
                body: Box::pin(WrapHyper(body)),
                timeout,
            },
        }
    }

    #[cfg(feature = "blocking")]
    pub(crate) fn wrap(body: hyper::Body) -> Body {
        Body {
            inner: Inner::Streaming {
                body: Box::pin(WrapHyper(body)),
                timeout: None,
            },
        }
    }

    pub(crate) fn empty() -> Body {
        Body::reusable(Bytes::new())
    }

    pub(crate) fn reusable(chunk: Bytes) -> Body {
        Body {
            inner: Inner::Reusable(chunk),
        }
    }

    pub(crate) fn try_reuse(self) -> (Option<Bytes>, Self) {
        let reuse = match self.inner {
            Inner::Reusable(ref chunk) => Some(chunk.clone()),
            Inner::Streaming { .. } => None,
        };

        (reuse, self)
    }

    pub(crate) fn try_clone(&self) -> Option<Body> {
        match self.inner {
            Inner::Reusable(ref chunk) => Some(Body::reusable(chunk.clone())),
            Inner::Streaming { .. } => None,
        }
    }

    pub(crate) fn into_stream(self) -> ImplStream {
        ImplStream(self)
    }

    #[cfg(feature = "multipart")]
    pub(crate) fn content_length(&self) -> Option<u64> {
        match self.inner {
            Inner::Reusable(ref bytes) => Some(bytes.len() as u64),
            Inner::Streaming { ref body, .. } => body.size_hint().exact(),
        }
    }
}

impl From<hyper::Body> for Body {
    #[inline]
    fn from(body: hyper::Body) -> Body {
        Self {
            inner: Inner::Streaming {
                body: Box::pin(WrapHyper(body)),
                timeout: None,
            },
        }
    }
}

impl From<Bytes> for Body {
    #[inline]
    fn from(bytes: Bytes) -> Body {
        Body::reusable(bytes)
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(vec: Vec<u8>) -> Body {
        Body::reusable(vec.into())
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(s: &'static [u8]) -> Body {
        Body::reusable(Bytes::from_static(s))
    }
}

impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Body {
        Body::reusable(s.into())
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(s: &'static str) -> Body {
        s.as_bytes().into()
    }
}

#[cfg(feature = "stream")]
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
impl From<File> for Body {
    #[inline]
    fn from(file: File) -> Body {
        Body::wrap_stream(ReaderStream::new(file))
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Body").finish()
    }
}

// ===== impl ImplStream =====

impl HttpBody for ImplStream {
    type Data = Bytes;
    type Error = crate::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let opt_try_chunk = match self.0.inner {
            Inner::Streaming {
                ref mut body,
                ref mut timeout,
            } => {
                if let Some(ref mut timeout) = timeout {
                    if let Poll::Ready(()) = timeout.as_mut().poll(cx) {
                        return Poll::Ready(Some(Err(crate::error::body(crate::error::TimedOut))));
                    }
                }
                futures_core::ready!(Pin::new(body).poll_data(cx))
                    .map(|opt_chunk| opt_chunk.map(Into::into).map_err(crate::error::body))
            }
            Inner::Reusable(ref mut bytes) => {
                if bytes.is_empty() {
                    None
                } else {
                    Some(Ok(std::mem::replace(bytes, Bytes::new())))
                }
            }
        };

        Poll::Ready(opt_try_chunk)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }

    fn is_end_stream(&self) -> bool {
        match self.0.inner {
            Inner::Streaming { ref body, .. } => body.is_end_stream(),
            Inner::Reusable(ref bytes) => bytes.is_empty(),
        }
    }

    fn size_hint(&self) -> http_body::SizeHint {
        match self.0.inner {
            Inner::Streaming { ref body, .. } => body.size_hint(),
            Inner::Reusable(ref bytes) => {
                let mut hint = http_body::SizeHint::default();
                hint.set_exact(bytes.len() as u64);
                hint
            }
        }
    }
}

impl Stream for ImplStream {
    type Item = Result<Bytes, crate::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.poll_data(cx)
    }
}

// ===== impl WrapStream =====

impl<S, D, E> HttpBody for WrapStream<S>
where
    S: Stream<Item = Result<D, E>>,
    D: Into<Bytes>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Data = Bytes;
    type Error = E;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let item = futures_core::ready!(self.project().inner.get_pin_mut().poll_next(cx)?);

        Poll::Ready(item.map(|val| Ok(val.into())))
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}

// ===== impl WrapHyper =====

impl HttpBody for WrapHyper {
    type Data = Bytes;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        // safe pin projection
        Pin::new(&mut self.0)
            .poll_data(cx)
            .map(|opt| opt.map(|res| res.map_err(Into::into)))
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }

    fn is_end_stream(&self) -> bool {
        self.0.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        HttpBody::size_hint(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Body;

    #[test]
    fn test_as_bytes() {
        let test_data = b"Test body";
        let body = Body::from(&test_data[..]);
        assert_eq!(body.as_bytes(), Some(&test_data[..]));
    }
}
