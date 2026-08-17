use core::pin::Pin;
use core::task::{Context, Poll};

use futures_util::io::{AsyncRead, Error};
use futures_util::ready;
use futures_util::FutureExt;
use js_sys::{Object, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::util::{checked_cast_to_usize, clamp_to_u32, js_to_io_error};

use super::sys::ReadableStreamReadResult;
use super::ReadableStreamBYOBReader;

/// An [`AsyncRead`] for the [`into_async_read`](super::ReadableStream::into_async_read) method.
///
/// This `AsyncRead` holds a reader, and therefore locks the [`ReadableStream`](super::ReadableStream).
/// When this `AsyncRead` is dropped, it also drops its reader which in turn
/// [releases its lock](https://streams.spec.whatwg.org/#release-a-lock).
///
/// [`AsyncRead`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncRead.html
#[must_use = "readers do nothing unless polled"]
#[derive(Debug)]
pub struct IntoAsyncRead<'reader> {
    reader: Option<ReadableStreamBYOBReader<'reader>>,
    buffer: Option<Uint8Array>,
    fut: Option<JsFuture>,
    cancel_on_drop: bool,
}

impl<'reader> IntoAsyncRead<'reader> {
    #[inline]
    pub(super) fn new(reader: ReadableStreamBYOBReader, cancel_on_drop: bool) -> IntoAsyncRead {
        IntoAsyncRead {
            reader: Some(reader),
            buffer: None,
            fut: None,
            cancel_on_drop,
        }
    }

    /// [Cancels](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// signaling a loss of interest in the stream by a consumer.
    pub async fn cancel(mut self) -> Result<(), JsValue> {
        match self.reader.take() {
            Some(mut reader) => reader.cancel().await,
            None => Ok(()),
        }
    }

    /// [Cancels](https://streams.spec.whatwg.org/#cancel-a-readable-stream) the stream,
    /// signaling a loss of interest in the stream by a consumer.
    pub async fn cancel_with_reason(mut self, reason: &JsValue) -> Result<(), JsValue> {
        match self.reader.take() {
            Some(mut reader) => reader.cancel_with_reason(reason).await,
            None => Ok(()),
        }
    }

    #[inline]
    fn discard_reader(mut self: Pin<&mut Self>) {
        self.reader = None;
        self.buffer = None;
    }
}

impl<'reader> AsyncRead for IntoAsyncRead<'reader> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        let read_fut = match self.fut.as_mut() {
            Some(fut) => fut,
            None => {
                // No pending read, start reading the next bytes
                let buf_len = clamp_to_u32(buf.len());
                let buffer = match self.buffer.take() {
                    // Re-use the internal buffer if it is large enough,
                    // otherwise allocate a new one
                    Some(buffer) if buffer.byte_length() >= buf_len => buffer,
                    _ => Uint8Array::new_with_length(buf_len),
                };
                // Limit to output buffer size
                let buffer = buffer.subarray(0, buf_len).unchecked_into::<Object>();
                match &self.reader {
                    Some(reader) => {
                        // Read into internal buffer and store its future
                        let fut =
                            JsFuture::from(reader.as_raw().read_with_array_buffer_view(&buffer));
                        self.fut.insert(fut)
                    }
                    None => {
                        // Reader was already dropped
                        return Poll::Ready(Ok(0));
                    }
                }
            }
        };

        // Poll the future for the pending read
        let js_result = ready!(read_fut.poll_unpin(cx));
        self.fut = None;

        // Read completed
        Poll::Ready(match js_result {
            Ok(js_value) => {
                let result = ReadableStreamReadResult::from(js_value);
                if result.get_done().unwrap_or_default() {
                    // End of stream
                    self.discard_reader();
                    Ok(0)
                } else {
                    // Cannot be canceled, so view must exist
                    let filled_view = result.get_value().unchecked_into::<Uint8Array>();
                    // Copy bytes to output buffer
                    let filled_len = checked_cast_to_usize(filled_view.byte_length());
                    debug_assert!(filled_len <= buf.len());
                    filled_view.copy_to(&mut buf[0..filled_len]);
                    // Re-construct internal buffer with the new ArrayBuffer
                    self.buffer = Some(Uint8Array::new(&filled_view.buffer()));
                    Ok(filled_len)
                }
            }
            Err(js_value) => {
                // Error
                self.discard_reader();
                Err(js_to_io_error(js_value))
            }
        })
    }
}

impl<'reader> Drop for IntoAsyncRead<'reader> {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            if let Some(reader) = self.reader.take() {
                let on_rejected = Closure::once(|_| {});
                let _ = reader.as_raw().cancel().catch(&on_rejected);
                on_rejected.forget();
            }
        }
    }
}
