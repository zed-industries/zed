#![allow(unused_imports)]

use super::{body::BodyInner, CompressionBody};
use crate::compression::predicate::Predicate;
use crate::compression::CompressionLevel;
use crate::compression_utils::WrapBody;
use crate::content_encoding::Encoding;
use futures_util::ready;
use http::{header, HeaderMap, HeaderValue, Response};
use http_body::Body;
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    /// Response future of [`Compression`].
    ///
    /// [`Compression`]: super::Compression
    #[derive(Debug)]
    pub struct ResponseFuture<F, P> {
        #[pin]
        pub(crate) inner: F,
        pub(crate) encoding: Encoding,
        pub(crate) predicate: P,
        pub(crate) quality: CompressionLevel,
    }
}

impl<F, B, E, P> Future for ResponseFuture<F, P>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: Body,
    P: Predicate,
{
    type Output = Result<Response<CompressionBody<B>>, E>;

    #[allow(unreachable_code, unused_mut, unused_variables)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = ready!(self.as_mut().project().inner.poll(cx)?);

        // never recompress responses that are already compressed
        let should_compress = !res.headers().contains_key(header::CONTENT_ENCODING)
            && self.predicate.should_compress(&res);

        let (mut parts, body) = res.into_parts();

        let body = match (should_compress, self.encoding) {
            // if compression is _not_ support or the client doesn't accept it
            (false, _) | (_, Encoding::Identity) => {
                return Poll::Ready(Ok(Response::from_parts(
                    parts,
                    CompressionBody::new(BodyInner::identity(body)),
                )))
            }

            #[cfg(feature = "compression-gzip")]
            (_, Encoding::Gzip) => {
                CompressionBody::new(BodyInner::gzip(WrapBody::new(body, self.quality)))
            }
            #[cfg(feature = "compression-deflate")]
            (_, Encoding::Deflate) => {
                CompressionBody::new(BodyInner::deflate(WrapBody::new(body, self.quality)))
            }
            #[cfg(feature = "compression-br")]
            (_, Encoding::Brotli) => {
                CompressionBody::new(BodyInner::brotli(WrapBody::new(body, self.quality)))
            }
            #[cfg(feature = "compression-zstd")]
            (_, Encoding::Zstd) => {
                CompressionBody::new(BodyInner::zstd(WrapBody::new(body, self.quality)))
            }
            #[cfg(feature = "fs")]
            (true, _) => {
                // This should never happen because the `AcceptEncoding` struct which is used to determine
                // `self.encoding` will only enable the different compression algorithms if the
                // corresponding crate feature has been enabled. This means
                // Encoding::[Gzip|Brotli|Deflate] should be impossible at this point without the
                // features enabled.
                //
                // The match arm is still required though because the `fs` feature uses the
                // Encoding struct independently and requires no compression logic to be enabled.
                // This means a combination of an individual compression feature and `fs` will fail
                // to compile without this branch even though it will never be reached.
                //
                // To safeguard against refactors that changes this relationship or other bugs the
                // server will return an uncompressed response instead of panicking since that could
                // become a ddos attack vector.
                return Poll::Ready(Ok(Response::from_parts(
                    parts,
                    CompressionBody::new(BodyInner::identity(body)),
                )));
            }
        };

        parts.headers.remove(header::CONTENT_LENGTH);

        parts
            .headers
            .insert(header::CONTENT_ENCODING, self.encoding.into_header_value());

        let res = Response::from_parts(parts, body);
        Poll::Ready(Ok(res))
    }
}
