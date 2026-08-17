//! Bindings and conversions for
//! [readable streams](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream).
use futures_util::io::AsyncRead;
use futures_util::Stream;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub use byob_reader::ReadableStreamBYOBReader;
pub use default_reader::ReadableStreamDefaultReader;
pub use into_async_read::IntoAsyncRead;
pub use into_stream::IntoStream;
use into_underlying_source::IntoUnderlyingSource;
pub use pipe_options::PipeOptions;

use crate::queuing_strategy::QueuingStrategy;
use crate::readable::into_underlying_byte_source::IntoUnderlyingByteSource;
use crate::util::promise_to_void_future;
use crate::writable::WritableStream;

mod byob_reader;
mod default_reader;
mod into_async_read;
mod into_stream;
mod into_underlying_byte_source;
mod into_underlying_source;
mod pipe_options;
pub mod sys;

/// A [`ReadableStream`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream).
///
/// `ReadableStream`s can be created from a [raw JavaScript stream](sys::ReadableStream) with
/// [`from_raw`](Self::from_raw), or from a Rust [`Stream`] with [`from_stream`](Self::from_stream).
///
/// They can be converted into a [raw JavaScript stream](sys::ReadableStream) with
/// [`into_raw`](Self::into_raw), or into a Rust [`Stream`] with [`into_stream`](Self::into_stream).
///
/// If the browser supports [readable byte streams](https://streams.spec.whatwg.org/#readable-byte-stream),
/// then they can be created from a Rust [`AsyncRead`] with [`from_async_read`](Self::from_async_read),
/// or converted into one with [`into_async_read`](Self::into_async_read).
///
/// [`Stream`]: https://docs.rs/futures/0.3.30/futures/stream/trait.Stream.html
/// [`AsyncRead`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html
#[derive(Debug)]
pub struct ReadableStream {
    raw: sys::ReadableStream,
}

impl ReadableStream {
    /// Creates a new `ReadableStream` from a [JavaScript stream](sys::ReadableStream).
    #[inline]
    pub fn from_raw(raw: sys::ReadableStream) -> Self {
        Self { raw }
    }

    /// Creates a new `ReadableStream` from a [`Stream`].
    ///
    /// Items and errors must be represented as raw [`JsValue`]s.
    /// Use [`map`], [`map_ok`] and/or [`map_err`] to convert a stream's items to a `JsValue`
    /// before passing it to this function.
    ///
    /// [`Stream`]: https://docs.rs/futures/0.3.30/futures/stream/trait.Stream.html
    /// [`map`]: https://docs.rs/futures/0.3.30/futures/stream/trait.StreamExt.html#method.map
    /// [`map_ok`]: https://docs.rs/futures/0.3.30/futures/stream/trait.TryStreamExt.html#method.map_ok
    /// [`map_err`]: https://docs.rs/futures/0.3.30/futures/stream/trait.TryStreamExt.html#method.map_err
    pub fn from_stream<St>(stream: St) -> Self
    where
        St: Stream<Item = Result<JsValue, JsValue>> + 'static,
    {
        let source = IntoUnderlyingSource::new(Box::new(stream));
        // Set HWM to 0 to prevent the JS ReadableStream from buffering chunks in its queue,
        // since the original Rust stream is better suited to handle that.
        let strategy = QueuingStrategy::new(0.0);
        let raw =
            sys::ReadableStreamExt::new_with_into_underlying_source(source, strategy.into_raw())
                .unchecked_into();
        Self::from_raw(raw)
    }

    /// Creates a new `ReadableStream` from an [`AsyncRead`].
    ///
    /// This creates a readable byte stream whose `autoAllocateChunkSize` is `default_buffer_len`.
    /// Therefore, if a default reader is used to consume the stream, the given `async_read`
    /// will be [polled][AsyncRead::poll_read] with a buffer of this size. If a BYOB reader is used,
    /// then it will be polled with a buffer of the same size as the BYOB read request instead.
    ///
    /// **Panics** if readable byte streams are not supported by the browser.
    ///
    /// [`AsyncRead`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html
    /// [AsyncRead::poll_read]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html#tymethod.poll_read
    // TODO Non-panicking variant?
    pub fn from_async_read<R>(async_read: R, default_buffer_len: usize) -> Self
    where
        R: AsyncRead + 'static,
    {
        let source = IntoUnderlyingByteSource::new(Box::new(async_read), default_buffer_len);
        let raw = sys::ReadableStreamExt::new_with_into_underlying_byte_source(source)
            .expect_throw("readable byte streams not supported")
            .unchecked_into();
        Self::from_raw(raw)
    }

    /// Creates a new `ReadableStream` wrapping the provided [iterable] or [async iterable].
    ///
    /// This can be used to adapt various kinds of objects into a readable stream,
    /// such as an [array], an [async generator] or a [Node.js readable stream][Readable].
    ///
    /// **Panics** if `ReadableStream.from()` is not supported by the browser,
    /// or if the given object is not a valid iterable or async iterable.
    /// For a non-panicking variant, use [`try_from`](Self::try_from).
    ///
    /// [iterable]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_iterable_protocol
    /// [async iterable]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_async_iterator_and_async_iterable_protocols
    /// [array]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
    /// [async generator]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator
    /// [Readable]: https://nodejs.org/api/stream.html#class-streamreadable
    pub fn from(async_iterable: Object) -> Self {
        Self::try_from(async_iterable).unwrap_throw()
    }

    /// Try to create a new `ReadableStream` wrapping the provided [iterable] or [async iterable].
    ///
    /// This can be used to adapt various kinds of objects into a readable stream,
    /// such as an [array], an [async generator] or a [Node.js readable stream][Readable].
    ///
    /// If `ReadableStream.from()` is not supported by the browser,
    /// or if the given object is not a valid iterable or async iterable,
    /// then this returns an error.
    ///
    /// [iterable]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_iterable_protocol
    /// [async iterable]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_async_iterator_and_async_iterable_protocols
    /// [array]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
    /// [async generator]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator
    /// [Readable]: https://nodejs.org/api/stream.html#class-streamreadable
    pub fn try_from(async_iterable: Object) -> Result<Self, js_sys::Error> {
        let raw = sys::ReadableStreamExt::from_async_iterable(&async_iterable)?.unchecked_into();
        Ok(Self::from_raw(raw))
    }

    /// Acquires a reference to the underlying [JavaScript stream](sys::ReadableStream).
    #[inline]
    pub fn as_raw(&self) -> &sys::ReadableStream {
        &self.raw
    }

    /// Consumes this `ReadableStream`, returning the underlying [JavaScript stream](sys::ReadableStream).
    #[inline]
    pub fn into_raw(self) -> sys::ReadableStream {
        self.raw
    }

    /// Returns `true` if the stream is [locked to a reader](https://streams.spec.whatwg.org/#lock).
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.as_raw().locked()
    }

    /// [Cancels](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// signaling a loss of interest in the stream by a consumer.
    ///
    /// If the stream is currently locked to a reader, then this returns an error.
    pub async fn cancel(&mut self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().cancel()).await
    }

    /// [Cancels](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// signaling a loss of interest in the stream by a consumer.
    ///
    /// The supplied `reason` will be given to the underlying source, which may or may not use it.
    ///
    /// If the stream is currently locked to a reader, then this returns an error.
    pub async fn cancel_with_reason(&mut self, reason: &JsValue) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().cancel_with_reason(reason)).await
    }

    /// Creates a [default reader](ReadableStreamDefaultReader) and
    /// [locks](https://streams.spec.whatwg.org/#lock) the stream to the new reader.
    ///
    /// While the stream is locked, no other reader can be acquired until this one is released.
    ///
    /// **Panics** if the stream is already locked to a reader. For a non-panicking variant,
    /// use [`try_get_reader`](Self::try_get_reader).
    #[inline]
    pub fn get_reader(&mut self) -> ReadableStreamDefaultReader {
        self.try_get_reader()
            .expect_throw("already locked to a reader")
    }

    /// Try to create a [default reader](ReadableStreamDefaultReader) and
    /// [lock](https://streams.spec.whatwg.org/#lock) the stream to the new reader.
    ///
    /// While the stream is locked, no other reader can be acquired until this one is released.
    ///
    /// If the stream is already locked to a reader, then this returns an error.
    pub fn try_get_reader(&mut self) -> Result<ReadableStreamDefaultReader, js_sys::Error> {
        ReadableStreamDefaultReader::new(self)
    }

    /// Creates a [BYOB reader](ReadableStreamBYOBReader) and
    /// [locks](https://streams.spec.whatwg.org/#lock) the stream to the new reader.
    ///
    /// While the stream is locked, no other reader can be acquired until this one is released.
    ///
    /// **Panics** if the stream is already locked to a reader, or if this stream is not a readable
    /// byte stream. For a non-panicking variant, use [`try_get_reader`](Self::try_get_reader).
    pub fn get_byob_reader(&mut self) -> ReadableStreamBYOBReader {
        self.try_get_byob_reader()
            .expect_throw("already locked to a reader, or not a readable byte stream")
    }

    /// Try to create a [BYOB reader](ReadableStreamBYOBReader) and
    /// [lock](https://streams.spec.whatwg.org/#lock) the stream to the new reader.
    ///
    /// While the stream is locked, no other reader can be acquired until this one is released.
    ///
    /// If the stream is already locked to a reader, then this returns an error.
    pub fn try_get_byob_reader(&mut self) -> Result<ReadableStreamBYOBReader, js_sys::Error> {
        ReadableStreamBYOBReader::new(self)
    }

    /// [Pipes](https://streams.spec.whatwg.org/#piping) this readable stream to a given
    /// writable stream.
    ///
    /// Piping a stream will [lock](https://streams.spec.whatwg.org/#lock) it for the duration
    /// of the pipe, preventing any other consumer from acquiring a reader.
    ///
    /// This returns `()` if the pipe completes successfully, or `Err(error)` if any `error`
    /// was encountered during the process.
    pub async fn pipe_to<'a>(&'a mut self, dest: &'a mut WritableStream) -> Result<(), JsValue> {
        self.pipe_to_with_options(dest, &PipeOptions::default())
            .await
    }

    /// [Pipes](https://streams.spec.whatwg.org/#piping) this readable stream to a given
    /// writable stream.
    ///
    /// Piping a stream will [lock](https://streams.spec.whatwg.org/#lock) it for the duration
    /// of the pipe, preventing any other consumer from acquiring a reader.
    ///
    /// Errors and closures of the source and destination streams propagate as follows:
    /// * An error in the source readable stream will [abort](https://streams.spec.whatwg.org/#abort-a-writable-stream)
    ///   the destination writable stream, unless [`options.prevent_abort`](PipeOptions::prevent_abort)
    ///   is `true`.
    /// * An error in the destination writable stream will [cancel](https://streams.spec.whatwg.org/#cancel-a-readable-stream)
    ///   the source readable stream, unless [`options.prevent_cancel`](PipeOptions::prevent_cancel)
    ///   is `true`.
    /// * When the source readable stream closes, the destination writable stream will be closed,
    ///   unless [`options.prevent_close`](PipeOptions::prevent_close) is `true`.
    /// * If the destination writable stream starts out closed or closing, the source readable stream
    ///   will be [canceled](https://streams.spec.whatwg.org/#cancel-a-readable-stream),
    ///   unless unless [`options.prevent_cancel`](PipeOptions::prevent_cancel) is `true`.
    ///
    /// This returns `()` if the pipe completes successfully, or `Err(error)` if any `error`
    /// was encountered during the process.
    pub async fn pipe_to_with_options<'a>(
        &'a mut self,
        dest: &'a mut WritableStream,
        options: &PipeOptions,
    ) -> Result<(), JsValue> {
        let promise = self
            .as_raw()
            .pipe_to_with_options(dest.as_raw(), &options.clone().into_raw());
        promise_to_void_future(promise).await
    }

    /// [Tees](https://streams.spec.whatwg.org/#tee-a-readable-stream) this readable stream,
    /// returning the two resulting branches as new [`ReadableStream`] instances.
    ///
    /// Teeing a stream will [lock](https://streams.spec.whatwg.org/#lock) it, preventing any other
    /// consumer from acquiring a reader.
    /// To [cancel](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// cancel both of the resulting branches; a composite cancellation reason will then be
    /// propagated to the stream's underlying source.
    ///
    /// Note that the chunks seen in each branch will be the same object.
    /// If the chunks are not immutable, this could allow interference between the two branches.
    ///
    /// **Panics** if the stream is already locked to a reader. For a non-panicking variant,
    /// use [`try_tee`](Self::try_tee).
    pub fn tee(self) -> (ReadableStream, ReadableStream) {
        self.try_tee().expect_throw("already locked to a reader")
    }

    /// Tries to [tee](https://streams.spec.whatwg.org/#tee-a-readable-stream) this readable stream,
    /// returning the two resulting branches as new [`ReadableStream`] instances.
    ///
    /// Teeing a stream will [lock](https://streams.spec.whatwg.org/#lock) it, preventing any other
    /// consumer from acquiring a reader.
    /// To [cancel](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// cancel both of the resulting branches; a composite cancellation reason will then be
    /// propagated to the stream's underlying source.
    ///
    /// Note that the chunks seen in each branch will be the same object.
    /// If the chunks are not immutable, this could allow interference between the two branches.
    ///
    /// If the stream is already locked to a reader, then this returns an error
    /// along with the original `ReadableStream`.
    pub fn try_tee(self) -> Result<(ReadableStream, ReadableStream), (js_sys::Error, Self)> {
        let branches = self
            .as_raw()
            .unchecked_ref::<sys::ReadableStreamExt>()
            .try_tee()
            .map_err(|err| (err, self))?;
        debug_assert_eq!(branches.length(), 2);
        let (left, right) = (branches.get(0), branches.get(1));
        Ok((
            Self::from_raw(left.unchecked_into()),
            Self::from_raw(right.unchecked_into()),
        ))
    }

    /// Converts this `ReadableStream` into a [`Stream`].
    ///
    /// Items and errors are represented by their raw [`JsValue`].
    /// Use [`map`], [`map_ok`] and/or [`map_err`] on the returned stream to convert them to a more
    /// appropriate type.
    ///
    /// **Panics** if the stream is already locked to a reader. For a non-panicking variant,
    /// use [`try_into_stream`](Self::try_into_stream).
    ///
    /// [`Stream`]: https://docs.rs/futures/0.3.30/futures/stream/trait.Stream.html
    /// [`map`]: https://docs.rs/futures/0.3.30/futures/stream/trait.StreamExt.html#method.map
    /// [`map_ok`]: https://docs.rs/futures/0.3.30/futures/stream/trait.TryStreamExt.html#method.map_ok
    /// [`map_err`]: https://docs.rs/futures/0.3.30/futures/stream/trait.TryStreamExt.html#method.map_err
    #[inline]
    pub fn into_stream(self) -> IntoStream<'static> {
        self.try_into_stream()
            .expect_throw("already locked to a reader")
    }

    /// Try to convert this `ReadableStream` into a [`Stream`].
    ///
    /// Items and errors are represented by their raw [`JsValue`].
    /// Use [`map`], [`map_ok`] and/or [`map_err`] on the returned stream to convert them to a more
    /// appropriate type.
    ///
    /// If the stream is already locked to a reader, then this returns an error
    /// along with the original `ReadableStream`.
    ///
    /// [`Stream`]: https://docs.rs/futures/0.3.30/futures/stream/trait.Stream.html
    /// [`map`]: https://docs.rs/futures/0.3.30/futures/stream/trait.StreamExt.html#method.map
    /// [`map_ok`]: https://docs.rs/futures/0.3.30/futures/stream/trait.TryStreamExt.html#method.map_ok
    /// [`map_err`]: https://docs.rs/futures/0.3.30/futures/stream/trait.TryStreamExt.html#method.map_err
    pub fn try_into_stream(mut self) -> Result<IntoStream<'static>, (js_sys::Error, Self)> {
        let reader = ReadableStreamDefaultReader::new(&mut self).map_err(|err| (err, self))?;
        Ok(IntoStream::new(reader, true))
    }

    /// Converts this `ReadableStream` into an [`AsyncRead`].
    ///
    /// **Panics** if the stream is already locked to a reader, or if this stream is not a readable
    /// byte stream. For a non-panicking variant, use [`try_into_async_read`](Self::try_into_async_read).
    ///
    /// [`AsyncRead`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html
    #[inline]
    pub fn into_async_read(self) -> IntoAsyncRead<'static> {
        self.try_into_async_read()
            .expect_throw("already locked to a reader, or not a readable byte stream")
    }

    /// Try to convert this `ReadableStream` into an [`AsyncRead`].
    ///
    /// If the stream is already locked to a reader, or if this stream is not a readable byte
    /// stream, then this returns an error along with the original `ReadableStream`.
    ///
    /// [`AsyncRead`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html
    pub fn try_into_async_read(mut self) -> Result<IntoAsyncRead<'static>, (js_sys::Error, Self)> {
        let reader = ReadableStreamBYOBReader::new(&mut self).map_err(|err| (err, self))?;
        Ok(IntoAsyncRead::new(reader, true))
    }
}

impl<St> From<St> for ReadableStream
where
    St: Stream<Item = Result<JsValue, JsValue>> + 'static,
{
    /// Equivalent to [`from_stream`](Self::from_stream).
    #[inline]
    fn from(stream: St) -> Self {
        Self::from_stream(stream)
    }
}
