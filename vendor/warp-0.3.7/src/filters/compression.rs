//! Compression Filters
//!
//! Filters that compress the body of a response.

#[cfg(feature = "compression-brotli")]
use async_compression::tokio::bufread::BrotliEncoder;

#[cfg(feature = "compression-gzip")]
use async_compression::tokio::bufread::{DeflateEncoder, GzipEncoder};

use http::header::HeaderValue;
use hyper::{
    header::{CONTENT_ENCODING, CONTENT_LENGTH},
    Body,
};
use tokio_util::io::{ReaderStream, StreamReader};

use crate::filter::{Filter, WrapSealed};
use crate::reject::IsReject;
use crate::reply::{Reply, Response};

use self::internal::{CompressionProps, WithCompression};

enum CompressionAlgo {
    #[cfg(feature = "compression-brotli")]
    BR,
    #[cfg(feature = "compression-gzip")]
    DEFLATE,
    #[cfg(feature = "compression-gzip")]
    GZIP,
}

impl From<CompressionAlgo> for HeaderValue {
    #[inline]
    fn from(algo: CompressionAlgo) -> Self {
        HeaderValue::from_static(match algo {
            #[cfg(feature = "compression-brotli")]
            CompressionAlgo::BR => "br",
            #[cfg(feature = "compression-gzip")]
            CompressionAlgo::DEFLATE => "deflate",
            #[cfg(feature = "compression-gzip")]
            CompressionAlgo::GZIP => "gzip",
        })
    }
}

/// Compression
#[derive(Clone, Copy, Debug)]
pub struct Compression<F> {
    func: F,
}

// TODO: The implementation of `gzip()`, `deflate()`, and `brotli()` could be replaced with
// generics or a macro

/// Create a wrapping filter that compresses the Body of a [`Response`](crate::reply::Response)
/// using gzip, adding `content-encoding: gzip` to the Response's [`HeaderMap`](hyper::HeaderMap)
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::get()
///     .and(warp::path::end())
///     .and(warp::fs::file("./README.md"))
///     .with(warp::compression::gzip());
/// ```
#[cfg(feature = "compression-gzip")]
pub fn gzip() -> Compression<impl Fn(CompressionProps) -> Response + Copy> {
    let func = move |mut props: CompressionProps| {
        let body = Body::wrap_stream(ReaderStream::new(GzipEncoder::new(StreamReader::new(
            props.body,
        ))));
        props
            .head
            .headers
            .append(CONTENT_ENCODING, CompressionAlgo::GZIP.into());
        props.head.headers.remove(CONTENT_LENGTH);
        Response::from_parts(props.head, body)
    };
    Compression { func }
}

/// Create a wrapping filter that compresses the Body of a [`Response`](crate::reply::Response)
/// using deflate, adding `content-encoding: deflate` to the Response's [`HeaderMap`](hyper::HeaderMap)
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::get()
///     .and(warp::path::end())
///     .and(warp::fs::file("./README.md"))
///     .with(warp::compression::deflate());
/// ```
#[cfg(feature = "compression-gzip")]
pub fn deflate() -> Compression<impl Fn(CompressionProps) -> Response + Copy> {
    let func = move |mut props: CompressionProps| {
        let body = Body::wrap_stream(ReaderStream::new(DeflateEncoder::new(StreamReader::new(
            props.body,
        ))));
        props
            .head
            .headers
            .append(CONTENT_ENCODING, CompressionAlgo::DEFLATE.into());
        props.head.headers.remove(CONTENT_LENGTH);
        Response::from_parts(props.head, body)
    };
    Compression { func }
}

/// Create a wrapping filter that compresses the Body of a [`Response`](crate::reply::Response)
/// using brotli, adding `content-encoding: br` to the Response's [`HeaderMap`](hyper::HeaderMap)
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::get()
///     .and(warp::path::end())
///     .and(warp::fs::file("./README.md"))
///     .with(warp::compression::brotli());
/// ```
#[cfg(feature = "compression-brotli")]
pub fn brotli() -> Compression<impl Fn(CompressionProps) -> Response + Copy> {
    let func = move |mut props: CompressionProps| {
        let body = Body::wrap_stream(ReaderStream::new(BrotliEncoder::new(StreamReader::new(
            props.body,
        ))));
        props
            .head
            .headers
            .append(CONTENT_ENCODING, CompressionAlgo::BR.into());
        props.head.headers.remove(CONTENT_LENGTH);
        Response::from_parts(props.head, body)
    };
    Compression { func }
}

impl<FN, F> WrapSealed<F> for Compression<FN>
where
    FN: Fn(CompressionProps) -> Response + Clone + Send,
    F: Filter + Clone + Send,
    F::Extract: Reply,
    F::Error: IsReject,
{
    type Wrapped = WithCompression<FN, F>;

    fn wrap(&self, filter: F) -> Self::Wrapped {
        WithCompression {
            filter,
            compress: self.clone(),
        }
    }
}

mod internal {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use bytes::Bytes;
    use futures_util::{ready, Stream, TryFuture};
    use hyper::Body;
    use pin_project::pin_project;

    use crate::filter::{Filter, FilterBase, Internal};
    use crate::reject::IsReject;
    use crate::reply::{Reply, Response};

    use super::Compression;

    /// A wrapper around any type that implements [`Stream`](futures::Stream) to be
    /// compatible with async_compression's Stream based encoders
    #[pin_project]
    #[derive(Debug)]
    pub struct CompressableBody<S, E>
    where
        E: std::error::Error,
        S: Stream<Item = Result<Bytes, E>>,
    {
        #[pin]
        body: S,
    }

    impl<S, E> Stream for CompressableBody<S, E>
    where
        E: std::error::Error,
        S: Stream<Item = Result<Bytes, E>>,
    {
        type Item = std::io::Result<Bytes>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            use std::io::{Error, ErrorKind};

            let pin = self.project();
            S::poll_next(pin.body, cx).map_err(|_| Error::from(ErrorKind::InvalidData))
        }
    }

    impl From<Body> for CompressableBody<Body, hyper::Error> {
        fn from(body: Body) -> Self {
            CompressableBody { body }
        }
    }

    /// Compression Props
    #[derive(Debug)]
    pub struct CompressionProps {
        pub(super) body: CompressableBody<Body, hyper::Error>,
        pub(super) head: http::response::Parts,
    }

    impl From<http::Response<Body>> for CompressionProps {
        fn from(resp: http::Response<Body>) -> Self {
            let (head, body) = resp.into_parts();
            CompressionProps {
                body: body.into(),
                head,
            }
        }
    }

    #[allow(missing_debug_implementations)]
    pub struct Compressed(pub(super) Response);

    impl Reply for Compressed {
        #[inline]
        fn into_response(self) -> Response {
            self.0
        }
    }

    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct WithCompression<FN, F> {
        pub(super) compress: Compression<FN>,
        pub(super) filter: F,
    }

    impl<FN, F> FilterBase for WithCompression<FN, F>
    where
        FN: Fn(CompressionProps) -> Response + Clone + Send,
        F: Filter + Clone + Send,
        F::Extract: Reply,
        F::Error: IsReject,
    {
        type Extract = (Compressed,);
        type Error = F::Error;
        type Future = WithCompressionFuture<FN, F::Future>;

        fn filter(&self, _: Internal) -> Self::Future {
            WithCompressionFuture {
                compress: self.compress.clone(),
                future: self.filter.filter(Internal),
            }
        }
    }

    #[allow(missing_debug_implementations)]
    #[pin_project]
    pub struct WithCompressionFuture<FN, F> {
        compress: Compression<FN>,
        #[pin]
        future: F,
    }

    impl<FN, F> Future for WithCompressionFuture<FN, F>
    where
        FN: Fn(CompressionProps) -> Response,
        F: TryFuture,
        F::Ok: Reply,
        F::Error: IsReject,
    {
        type Output = Result<(Compressed,), F::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let pin = self.as_mut().project();
            let result = ready!(pin.future.try_poll(cx));
            match result {
                Ok(reply) => {
                    let resp = (self.compress.func)(reply.into_response().into());
                    Poll::Ready(Ok((Compressed(resp),)))
                }
                Err(reject) => Poll::Ready(Err(reject)),
            }
        }
    }
}
