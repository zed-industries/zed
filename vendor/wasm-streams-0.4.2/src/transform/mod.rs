//! Bindings and conversions for
//! [transform streams](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream).
use crate::readable::ReadableStream;
use crate::writable::WritableStream;

pub mod sys;

/// A [`TransformStream`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream).
///
/// `TransformStream`s can be created from a [raw JavaScript stream](sys::TransformStream) with
/// [`from_raw`](Self::from_raw), and can be converted back with [`into_raw`](Self::into_raw).
///
/// Use [`readable`](Self::readable) and [`writable`](Self::writable) to access the readable and
/// writable side of the transform stream.
/// These can then be converted into a Rust [`Stream`] and [`Sink`] respectively
/// using [`into_stream`](super::ReadableStream::into_stream)
/// and [`into_sink`](super::WritableStream::into_sink).
///
/// [`Stream`]: https://docs.rs/futures/0.3.30/futures/stream/trait.Stream.html
/// [`Sink`]: https://docs.rs/futures/0.3.30/futures/sink/trait.Sink.html
#[derive(Debug)]
pub struct TransformStream {
    raw: sys::TransformStream,
}

impl TransformStream {
    /// Creates a new `TransformStream` from a [JavaScript stream](sys::TransformStream).
    #[inline]
    pub fn from_raw(raw: sys::TransformStream) -> Self {
        Self { raw }
    }

    /// Acquires a reference to the underlying [JavaScript stream](sys::TransformStream).
    #[inline]
    pub fn as_raw(&self) -> &sys::TransformStream {
        &self.raw
    }

    /// Consumes this `TransformStream`, returning the underlying [JavaScript stream](sys::TransformStream).
    #[inline]
    pub fn into_raw(self) -> sys::TransformStream {
        self.raw
    }

    /// Returns the readable side of the transform stream.
    #[inline]
    pub fn readable(&self) -> ReadableStream {
        ReadableStream::from_raw(self.as_raw().readable())
    }

    /// Returns the writable side of the transform stream.
    #[inline]
    pub fn writable(&self) -> WritableStream {
        WritableStream::from_raw(self.as_raw().writable())
    }
}
