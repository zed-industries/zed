use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(feature = "gzip")]
use async_compression::tokio::bufread::GzipDecoder;

#[cfg(feature = "brotli")]
use async_compression::tokio::bufread::BrotliDecoder;

#[cfg(feature = "deflate")]
use async_compression::tokio::bufread::ZlibDecoder;

use bytes::Bytes;
use futures_core::Stream;
use futures_util::stream::Peekable;
use http::HeaderMap;
use hyper::body::HttpBody;

#[cfg(any(feature = "gzip", feature = "brotli", feature = "deflate"))]
use tokio_util::codec::{BytesCodec, FramedRead};
#[cfg(any(feature = "gzip", feature = "brotli", feature = "deflate"))]
use tokio_util::io::StreamReader;

use super::super::Body;
use crate::error;

#[derive(Clone, Copy, Debug)]
pub(super) struct Accepts {
    #[cfg(feature = "gzip")]
    pub(super) gzip: bool,
    #[cfg(feature = "brotli")]
    pub(super) brotli: bool,
    #[cfg(feature = "deflate")]
    pub(super) deflate: bool,
}

/// A response decompressor over a non-blocking stream of chunks.
///
/// The inner decoder may be constructed asynchronously.
pub(crate) struct Decoder {
    inner: Inner,
}

type PeekableIoStream = Peekable<IoStream>;

#[cfg(any(feature = "gzip", feature = "brotli", feature = "deflate"))]
type PeekableIoStreamReader = StreamReader<PeekableIoStream, Bytes>;

enum Inner {
    /// A `PlainText` decoder just returns the response content as is.
    PlainText(super::body::ImplStream),

    /// A `Gzip` decoder will uncompress the gzipped response content before returning it.
    #[cfg(feature = "gzip")]
    Gzip(Pin<Box<FramedRead<GzipDecoder<PeekableIoStreamReader>, BytesCodec>>>),

    /// A `Brotli` decoder will uncompress the brotlied response content before returning it.
    #[cfg(feature = "brotli")]
    Brotli(Pin<Box<FramedRead<BrotliDecoder<PeekableIoStreamReader>, BytesCodec>>>),

    /// A `Deflate` decoder will uncompress the deflated response content before returning it.
    #[cfg(feature = "deflate")]
    Deflate(Pin<Box<FramedRead<ZlibDecoder<PeekableIoStreamReader>, BytesCodec>>>),

    /// A decoder that doesn't have a value yet.
    #[cfg(any(feature = "brotli", feature = "gzip", feature = "deflate"))]
    Pending(Pin<Box<Pending>>),
}

/// A future attempt to poll the response body for EOF so we know whether to use gzip or not.
struct Pending(PeekableIoStream, DecoderType);

struct IoStream(super::body::ImplStream);

enum DecoderType {
    #[cfg(feature = "gzip")]
    Gzip,
    #[cfg(feature = "brotli")]
    Brotli,
    #[cfg(feature = "deflate")]
    Deflate,
}

impl fmt::Debug for Decoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

impl Decoder {
    #[cfg(feature = "blocking")]
    pub(crate) fn empty() -> Decoder {
        Decoder {
            inner: Inner::PlainText(Body::empty().into_stream()),
        }
    }

    /// A plain text decoder.
    ///
    /// This decoder will emit the underlying chunks as-is.
    fn plain_text(body: Body) -> Decoder {
        Decoder {
            inner: Inner::PlainText(body.into_stream()),
        }
    }

    /// A gzip decoder.
    ///
    /// This decoder will buffer and decompress chunks that are gzipped.
    #[cfg(feature = "gzip")]
    fn gzip(body: Body) -> Decoder {
        use futures_util::StreamExt;

        Decoder {
            inner: Inner::Pending(Box::pin(Pending(
                IoStream(body.into_stream()).peekable(),
                DecoderType::Gzip,
            ))),
        }
    }

    /// A brotli decoder.
    ///
    /// This decoder will buffer and decompress chunks that are brotlied.
    #[cfg(feature = "brotli")]
    fn brotli(body: Body) -> Decoder {
        use futures_util::StreamExt;

        Decoder {
            inner: Inner::Pending(Box::pin(Pending(
                IoStream(body.into_stream()).peekable(),
                DecoderType::Brotli,
            ))),
        }
    }

    /// A deflate decoder.
    ///
    /// This decoder will buffer and decompress chunks that are deflated.
    #[cfg(feature = "deflate")]
    fn deflate(body: Body) -> Decoder {
        use futures_util::StreamExt;

        Decoder {
            inner: Inner::Pending(Box::pin(Pending(
                IoStream(body.into_stream()).peekable(),
                DecoderType::Deflate,
            ))),
        }
    }

    #[cfg(any(feature = "brotli", feature = "gzip", feature = "deflate"))]
    fn detect_encoding(headers: &mut HeaderMap, encoding_str: &str) -> bool {
        use http::header::{CONTENT_ENCODING, CONTENT_LENGTH, TRANSFER_ENCODING};
        use log::warn;

        let mut is_content_encoded = {
            headers
                .get_all(CONTENT_ENCODING)
                .iter()
                .any(|enc| enc == encoding_str)
                || headers
                    .get_all(TRANSFER_ENCODING)
                    .iter()
                    .any(|enc| enc == encoding_str)
        };
        if is_content_encoded {
            if let Some(content_length) = headers.get(CONTENT_LENGTH) {
                if content_length == "0" {
                    warn!("{encoding_str} response with content-length of 0");
                    is_content_encoded = false;
                }
            }
        }
        if is_content_encoded {
            headers.remove(CONTENT_ENCODING);
            headers.remove(CONTENT_LENGTH);
        }
        is_content_encoded
    }

    /// Constructs a Decoder from a hyper request.
    ///
    /// A decoder is just a wrapper around the hyper request that knows
    /// how to decode the content body of the request.
    ///
    /// Uses the correct variant by inspecting the Content-Encoding header.
    pub(super) fn detect(_headers: &mut HeaderMap, body: Body, _accepts: Accepts) -> Decoder {
        #[cfg(feature = "gzip")]
        {
            if _accepts.gzip && Decoder::detect_encoding(_headers, "gzip") {
                return Decoder::gzip(body);
            }
        }

        #[cfg(feature = "brotli")]
        {
            if _accepts.brotli && Decoder::detect_encoding(_headers, "br") {
                return Decoder::brotli(body);
            }
        }

        #[cfg(feature = "deflate")]
        {
            if _accepts.deflate && Decoder::detect_encoding(_headers, "deflate") {
                return Decoder::deflate(body);
            }
        }

        Decoder::plain_text(body)
    }
}

impl Stream for Decoder {
    type Item = Result<Bytes, error::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // Do a read or poll for a pending decoder value.
        match self.inner {
            #[cfg(any(feature = "brotli", feature = "gzip", feature = "deflate"))]
            Inner::Pending(ref mut future) => match Pin::new(future).poll(cx) {
                Poll::Ready(Ok(inner)) => {
                    self.inner = inner;
                    self.poll_next(cx)
                }
                Poll::Ready(Err(e)) => Poll::Ready(Some(Err(crate::error::decode_io(e)))),
                Poll::Pending => Poll::Pending,
            },
            Inner::PlainText(ref mut body) => Pin::new(body).poll_next(cx),
            #[cfg(feature = "gzip")]
            Inner::Gzip(ref mut decoder) => {
                match futures_core::ready!(Pin::new(decoder).poll_next(cx)) {
                    Some(Ok(bytes)) => Poll::Ready(Some(Ok(bytes.freeze()))),
                    Some(Err(err)) => Poll::Ready(Some(Err(crate::error::decode_io(err)))),
                    None => Poll::Ready(None),
                }
            }
            #[cfg(feature = "brotli")]
            Inner::Brotli(ref mut decoder) => {
                match futures_core::ready!(Pin::new(decoder).poll_next(cx)) {
                    Some(Ok(bytes)) => Poll::Ready(Some(Ok(bytes.freeze()))),
                    Some(Err(err)) => Poll::Ready(Some(Err(crate::error::decode_io(err)))),
                    None => Poll::Ready(None),
                }
            }
            #[cfg(feature = "deflate")]
            Inner::Deflate(ref mut decoder) => {
                match futures_core::ready!(Pin::new(decoder).poll_next(cx)) {
                    Some(Ok(bytes)) => Poll::Ready(Some(Ok(bytes.freeze()))),
                    Some(Err(err)) => Poll::Ready(Some(Err(crate::error::decode_io(err)))),
                    None => Poll::Ready(None),
                }
            }
        }
    }
}

impl HttpBody for Decoder {
    type Data = Bytes;
    type Error = crate::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        self.poll_next(cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }

    fn size_hint(&self) -> http_body::SizeHint {
        match self.inner {
            Inner::PlainText(ref body) => HttpBody::size_hint(body),
            // the rest are "unknown", so default
            #[cfg(any(feature = "brotli", feature = "gzip", feature = "deflate"))]
            _ => http_body::SizeHint::default(),
        }
    }
}

impl Future for Pending {
    type Output = Result<Inner, std::io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use futures_util::StreamExt;

        match futures_core::ready!(Pin::new(&mut self.0).poll_peek(cx)) {
            Some(Ok(_)) => {
                // fallthrough
            }
            Some(Err(_e)) => {
                // error was just a ref, so we need to really poll to move it
                return Poll::Ready(Err(futures_core::ready!(
                    Pin::new(&mut self.0).poll_next(cx)
                )
                .expect("just peeked Some")
                .unwrap_err()));
            }
            None => return Poll::Ready(Ok(Inner::PlainText(Body::empty().into_stream()))),
        };

        let _body = std::mem::replace(
            &mut self.0,
            IoStream(Body::empty().into_stream()).peekable(),
        );

        match self.1 {
            #[cfg(feature = "brotli")]
            DecoderType::Brotli => Poll::Ready(Ok(Inner::Brotli(Box::pin(FramedRead::new(
                BrotliDecoder::new(StreamReader::new(_body)),
                BytesCodec::new(),
            ))))),
            #[cfg(feature = "gzip")]
            DecoderType::Gzip => Poll::Ready(Ok(Inner::Gzip(Box::pin(FramedRead::new(
                GzipDecoder::new(StreamReader::new(_body)),
                BytesCodec::new(),
            ))))),
            #[cfg(feature = "deflate")]
            DecoderType::Deflate => Poll::Ready(Ok(Inner::Deflate(Box::pin(FramedRead::new(
                ZlibDecoder::new(StreamReader::new(_body)),
                BytesCodec::new(),
            ))))),
        }
    }
}

impl Stream for IoStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match futures_core::ready!(Pin::new(&mut self.0).poll_next(cx)) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
            Some(Err(err)) => Poll::Ready(Some(Err(err.into_io()))),
            None => Poll::Ready(None),
        }
    }
}

// ===== impl Accepts =====

impl Accepts {
    pub(super) fn none() -> Self {
        Accepts {
            #[cfg(feature = "gzip")]
            gzip: false,
            #[cfg(feature = "brotli")]
            brotli: false,
            #[cfg(feature = "deflate")]
            deflate: false,
        }
    }

    pub(super) fn as_str(&self) -> Option<&'static str> {
        match (self.is_gzip(), self.is_brotli(), self.is_deflate()) {
            (true, true, true) => Some("gzip, br, deflate"),
            (true, true, false) => Some("gzip, br"),
            (true, false, true) => Some("gzip, deflate"),
            (false, true, true) => Some("br, deflate"),
            (true, false, false) => Some("gzip"),
            (false, true, false) => Some("br"),
            (false, false, true) => Some("deflate"),
            (false, false, false) => None,
        }
    }

    fn is_gzip(&self) -> bool {
        #[cfg(feature = "gzip")]
        {
            self.gzip
        }

        #[cfg(not(feature = "gzip"))]
        {
            false
        }
    }

    fn is_brotli(&self) -> bool {
        #[cfg(feature = "brotli")]
        {
            self.brotli
        }

        #[cfg(not(feature = "brotli"))]
        {
            false
        }
    }

    fn is_deflate(&self) -> bool {
        #[cfg(feature = "deflate")]
        {
            self.deflate
        }

        #[cfg(not(feature = "deflate"))]
        {
            false
        }
    }
}

impl Default for Accepts {
    fn default() -> Accepts {
        Accepts {
            #[cfg(feature = "gzip")]
            gzip: true,
            #[cfg(feature = "brotli")]
            brotli: true,
            #[cfg(feature = "deflate")]
            deflate: true,
        }
    }
}
