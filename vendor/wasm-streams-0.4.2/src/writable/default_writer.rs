use std::marker::PhantomData;

use wasm_bindgen::{throw_val, JsValue};

use crate::util::promise_to_void_future;

use super::{sys, IntoAsyncWrite, IntoSink, WritableStream};

/// A [`WritableStreamDefaultWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)
/// that can be used to write chunks to a [`WritableStream`](WritableStream).
///
/// This is returned by the [`get_writer`](WritableStream::get_writer) method.
///
/// When the writer is dropped, it automatically [releases its lock](https://streams.spec.whatwg.org/#release-a-lock).
#[derive(Debug)]
pub struct WritableStreamDefaultWriter<'stream> {
    raw: sys::WritableStreamDefaultWriter,
    _stream: PhantomData<&'stream mut WritableStream>,
}

impl<'stream> WritableStreamDefaultWriter<'stream> {
    pub(crate) fn new(stream: &mut WritableStream) -> Result<Self, js_sys::Error> {
        Ok(Self {
            raw: stream.as_raw().get_writer()?,
            _stream: PhantomData,
        })
    }

    /// Acquires a reference to the underlying [JavaScript writer](sys::WritableStreamDefaultWriter).
    #[inline]
    pub fn as_raw(&self) -> &sys::WritableStreamDefaultWriter {
        &self.raw
    }

    /// Waits for the stream to become closed.
    ///
    /// This returns an error if the stream ever errors, or if the writer's lock is
    /// [released](https://streams.spec.whatwg.org/#release-a-lock) before the stream finishes
    /// closing.
    pub async fn closed(&self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().closed()).await
    }

    /// Returns the desired size to fill the stream's internal queue.
    ///
    /// * It can be negative, if the queue is over-full.
    ///   A producer can use this information to determine the right amount of data to write.
    /// * It will be `None` if the stream cannot be successfully written to
    ///   (due to either being errored, or having an abort queued up).
    /// * It will return zero if the stream is closed.
    #[inline]
    pub fn desired_size(&self) -> Option<f64> {
        self.as_raw()
            .desired_size()
            .unwrap_or_else(|error| throw_val(error))
    }

    /// Waits until the desired size to fill the stream's internal queue transitions
    /// from non-positive to positive, signaling that it is no longer applying backpressure.
    ///
    /// Once the desired size to fill the stream's internal queue dips back to zero or below,
    /// this will return a new future that stays pending until the next transition.
    ///
    /// This returns an error if the stream ever errors, or if the writer's lock is
    /// [released](https://streams.spec.whatwg.org/#release-a-lock) before the stream finishes
    /// closing.
    pub async fn ready(&self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().ready()).await
    }

    /// [Aborts](https://streams.spec.whatwg.org/#abort-a-writable-stream) the stream,
    /// signaling that the producer can no longer successfully write to the stream.
    ///
    /// Equivalent to [`WritableStream.abort`](WritableStream::abort).
    pub async fn abort(&mut self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().abort()).await
    }

    /// [Aborts](https://streams.spec.whatwg.org/#abort-a-writable-stream) the stream with the
    /// given `reason`, signaling that the producer can no longer successfully write to the stream.
    ///
    /// Equivalent to [`WritableStream.abort_with_reason`](WritableStream::abort_with_reason).
    pub async fn abort_with_reason(&mut self, reason: &JsValue) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().abort_with_reason(reason)).await
    }

    /// Writes the given `chunk` to the writable stream, by waiting until any previous writes
    /// have finished successfully, and then sending the chunk to the underlying sink's `write()`
    /// method.
    ///
    /// This returns `Ok(())` upon a successful write, or `Err(error)` if the write fails or stream
    /// becomes errored before the writing process is initiated.
    ///
    /// Note that what "success" means is up to the underlying sink; it might indicate simply
    /// that the chunk has been accepted, and not necessarily that it is safely saved to
    /// its ultimate destination.
    pub async fn write(&mut self, chunk: JsValue) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().write_with_chunk(&chunk)).await
    }

    /// Closes the stream.
    ///
    /// The underlying sink will finish processing any previously-written chunks, before invoking
    /// its close behavior. During this time any further attempts to write will fail
    /// (without erroring the stream).
    ///
    /// This returns `Ok(())` if all remaining chunks are successfully written and the stream
    /// successfully closes, or `Err(error)` if an error is encountered during this process.
    pub async fn close(&mut self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().close()).await
    }

    /// Converts this `WritableStreamDefaultWriter` into a [`Sink`].
    ///
    /// This is similar to [`WritableStream.into_sink`](WritableStream::into_sink),
    /// except that after the returned `Sink` is dropped, the original `WritableStream` is still
    /// usable. This allows writing only a few chunks through the `Sink`, while still allowing
    /// another writer to write more chunks later on.
    ///
    /// [`Sink`]: https://docs.rs/futures/0.3.30/futures/sink/trait.Sink.html
    #[inline]
    pub fn into_sink(self) -> IntoSink<'stream> {
        IntoSink::new(self)
    }

    /// Converts this `WritableStreamDefaultWriter` into an [`AsyncWrite`].
    ///
    /// The writable stream must accept [`Uint8Array`](js_sys::Uint8Array) chunks.
    ///
    /// This is similar to [`WritableStream.into_async_write`](WritableStream::into_async_write),
    /// except that after the returned `AsyncWrite` is dropped, the original `WritableStream` is
    /// still usable. This allows writing only a few bytes through the `AsyncWrite`, while still
    /// allowing another writer to write more bytes later on.
    ///
    /// [`AsyncWrite`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncWrite.html
    #[inline]
    pub fn into_async_write(self) -> IntoAsyncWrite<'stream> {
        IntoAsyncWrite::new(self.into_sink())
    }
}

impl Drop for WritableStreamDefaultWriter<'_> {
    fn drop(&mut self) {
        self.as_raw().release_lock()
    }
}
