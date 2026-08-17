use std::marker::PhantomData;

use js_sys::{Object, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::util::{checked_cast_to_usize, clamp_to_u32, promise_to_void_future};

use super::{sys, IntoAsyncRead, ReadableStream};

/// A [`ReadableStreamBYOBReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)
/// that can be used to read chunks from a [`ReadableStream`](ReadableStream).
///
/// This is returned by the [`get_byob_reader`](ReadableStream::get_byob_reader) method.
///
/// When the reader is dropped, it automatically [releases its lock](https://streams.spec.whatwg.org/#release-a-lock).
#[derive(Debug)]
pub struct ReadableStreamBYOBReader<'stream> {
    raw: sys::ReadableStreamBYOBReader,
    _stream: PhantomData<&'stream mut ReadableStream>,
}

impl<'stream> ReadableStreamBYOBReader<'stream> {
    pub(crate) fn new(stream: &mut ReadableStream) -> Result<Self, js_sys::Error> {
        let reader_options = sys::ReadableStreamGetReaderOptions::new();
        reader_options.set_mode(sys::ReadableStreamReaderMode::Byob);
        Ok(Self {
            raw: stream
                .as_raw()
                .unchecked_ref::<sys::ReadableStreamExt>()
                .try_get_reader_with_options(&reader_options)?
                .unchecked_into(),
            _stream: PhantomData,
        })
    }

    /// Acquires a reference to the underlying [JavaScript reader](sys::ReadableStreamBYOBReader).
    #[inline]
    pub fn as_raw(&self) -> &sys::ReadableStreamBYOBReader {
        &self.raw
    }

    /// Waits for the stream to become closed.
    ///
    /// This returns an error if the stream ever errors, or if the reader's lock is
    /// [released](https://streams.spec.whatwg.org/#release-a-lock) before the stream finishes
    /// closing.
    pub async fn closed(&self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().closed()).await
    }

    /// [Cancels](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// signaling a loss of interest in the stream by a consumer.
    ///
    /// Equivalent to [`ReadableStream.cancel`](ReadableStream::cancel).
    pub async fn cancel(&mut self) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().cancel()).await
    }

    /// [Cancels](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// signaling a loss of interest in the stream by a consumer.
    ///
    /// Equivalent to [`ReadableStream.cancel_with_reason`](ReadableStream::cancel_with_reason).
    pub async fn cancel_with_reason(&mut self, reason: &JsValue) -> Result<(), JsValue> {
        promise_to_void_future(self.as_raw().cancel_with_reason(reason)).await
    }

    /// Reads the next chunk from the stream's internal queue into `dst`,
    /// and returns the number of bytes read.
    ///
    /// * If some bytes were read into `dst`, this returns `Ok(bytes_read)`.
    /// * If the stream closes and no more bytes are available, this returns `Ok(0)`.
    /// * If the stream cancels, this returns `Ok(0)`.
    /// * If the stream encounters an `error`, this returns `Err(error)`.
    ///
    /// This always allocated a new temporary `Uint8Array` with the same size as `dst` to hold
    /// the result before copying to `dst`. We cannot pass a view on the backing WebAssembly memory
    /// directly, because:
    /// * `reader.read(view)` needs to transfer `view.buffer`, but `WebAssembly.Memory` buffers
    ///    are non-transferable.
    /// * `view.buffer` can be invalidated if the WebAssembly memory grows while `read(view)`
    ///    is still in progress.
    ///
    /// Therefore, it is necessary to use a separate buffer living in the JavaScript heap.
    /// To avoid repeated allocations for repeated reads,
    /// use [`read_with_buffer`](Self::read_with_buffer).
    pub async fn read(&mut self, dst: &mut [u8]) -> Result<usize, JsValue> {
        let buffer = Uint8Array::new_with_length(clamp_to_u32(dst.len()));
        let (bytes_read, _) = self.read_with_buffer(dst, buffer).await?;
        Ok(bytes_read)
    }

    /// Reads the next chunk from the stream's internal queue into `dst`,
    /// and returns the number of bytes read.
    ///
    /// The given `buffer` is used to store the bytes before they are copied to `dst`.
    /// This buffer is returned back together with the result, so it can be re-used for subsequent
    /// reads without extra allocations. Note that the underlying `ArrayBuffer` is transferred
    /// in the process, so any other views on the original buffer will become unusable.
    ///
    /// * If some bytes were read into `dst`, this returns `Ok((bytes_read, Some(buffer)))`.
    /// * If the stream closes and no more bytes are available, this returns `Ok((0, Some(buffer)))`.
    /// * If the stream cancels, this returns `Ok((0, None))`. In this case, the given buffer is
    ///   not returned.
    /// * If the stream encounters an `error`, this returns `Err(error)`.
    pub async fn read_with_buffer(
        &mut self,
        dst: &mut [u8],
        buffer: Uint8Array,
    ) -> Result<(usize, Option<Uint8Array>), JsValue> {
        // Save the original buffer's byte offset and length.
        let buffer_offset = buffer.byte_offset();
        let buffer_len = buffer.byte_length();
        // Limit view to destination slice's length.
        let dst_len = clamp_to_u32(dst.len());
        let view = buffer.subarray(0, dst_len).unchecked_into::<Object>();
        // Read into view. This transfers `buffer.buffer()`.
        let promise = self.as_raw().read_with_array_buffer_view(&view);
        let js_result = JsFuture::from(promise).await?;
        let result = sys::ReadableStreamReadResult::from(js_result);
        let js_value = result.get_value();
        let filled_view = if js_value.is_undefined() {
            // No new view was returned. The stream must have been canceled.
            assert!(result.get_done().unwrap_or_default());
            return Ok((0, None));
        } else {
            js_value.unchecked_into::<Uint8Array>()
        };
        let filled_len = checked_cast_to_usize(filled_view.byte_length());
        debug_assert!(filled_len <= dst.len());
        // Re-construct the original Uint8Array with the new ArrayBuffer.
        let new_buffer = Uint8Array::new_with_byte_offset_and_length(
            &filled_view.buffer(),
            buffer_offset,
            buffer_len,
        );
        if result.get_done().unwrap_or_default() {
            debug_assert_eq!(filled_len, 0);
        } else {
            filled_view.copy_to(&mut dst[0..filled_len]);
        }
        Ok((filled_len, Some(new_buffer)))
    }

    /// [Releases](https://streams.spec.whatwg.org/#release-a-lock) this reader's lock on the
    /// corresponding stream.
    ///
    /// [As of January 2022](https://github.com/whatwg/streams/commit/d5f92d9f17306d31ba6b27424d23d58e89bf64a5),
    /// the Streams standard allows the lock to be released even when there are still pending read
    /// requests. Such requests will automatically become rejected, and this function will always
    /// succeed.
    ///
    /// However, if the Streams implementation is not yet up-to-date with this change, then
    /// releasing the lock while there are pending read requests will **panic**. For a non-panicking
    /// variant, use [`try_release_lock`](Self::try_release_lock).
    #[inline]
    pub fn release_lock(mut self) {
        self.release_lock_mut()
    }

    fn release_lock_mut(&mut self) {
        self.as_raw().release_lock()
    }

    /// Try to [release](https://streams.spec.whatwg.org/#release-a-lock) this reader's lock on the
    /// corresponding stream.
    ///
    /// [As of January 2022](https://github.com/whatwg/streams/commit/d5f92d9f17306d31ba6b27424d23d58e89bf64a5),
    /// the Streams standard allows the lock to be released even when there are still pending read
    /// requests. Such requests will automatically become rejected, and this function will always
    /// return `Ok(())`.
    ///
    /// However, if the Streams implementation is not yet up-to-date with this change, then
    /// the lock cannot be released while there are pending read requests. Attempting to do so will
    /// return an error and leave the reader locked to the stream.
    #[inline]
    pub fn try_release_lock(self) -> Result<(), (js_sys::Error, Self)> {
        self.as_raw()
            .unchecked_ref::<sys::ReadableStreamReaderExt>()
            .try_release_lock()
            .map_err(|err| (err, self))
    }

    /// Converts this `ReadableStreamBYOBReader` into an [`AsyncRead`].
    ///
    /// This is similar to [`ReadableStream.into_async_read`](ReadableStream::into_async_read),
    /// except that after the returned `AsyncRead` is dropped, the original `ReadableStream` is
    /// still usable. This allows reading only a few bytes from the `AsyncRead`, while still
    /// allowing another reader to read the remaining bytes later on.
    ///
    /// [`AsyncRead`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html
    #[inline]
    pub fn into_async_read(self) -> IntoAsyncRead<'stream> {
        IntoAsyncRead::new(self, false)
    }
}

impl Drop for ReadableStreamBYOBReader<'_> {
    fn drop(&mut self) {
        self.release_lock_mut();
    }
}
