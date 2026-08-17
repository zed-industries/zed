use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::io::AsyncWrite;
use futures_util::ready;
use futures_util::sink::SinkExt;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

use crate::util::js_to_io_error;

use super::IntoSink;

/// An [`AsyncWrite`] for the [`into_async_write`](super::WritableStream::into_async_write) method.
///
/// This `AsyncWrite` holds a writer, and therefore locks the [`WritableStream`](super::WritableStream).
/// When this `AsyncWrite` is dropped, it also drops its writer which in turn
/// [releases its lock](https://streams.spec.whatwg.org/#release-a-lock).
///
/// [`AsyncWrite`]: https://docs.rs/futures/0.3.30/futures/io/trait.AsyncWrite.html
#[must_use = "writers do nothing unless polled"]
#[derive(Debug)]
pub struct IntoAsyncWrite<'writer> {
    sink: IntoSink<'writer>,
}

impl<'writer> IntoAsyncWrite<'writer> {
    #[inline]
    pub(super) fn new(sink: IntoSink<'writer>) -> Self {
        Self { sink }
    }

    /// [Aborts](https://streams.spec.whatwg.org/#abort-a-writable-stream) the stream,
    /// signaling that the producer can no longer successfully write to the stream.
    pub async fn abort(self) -> Result<(), JsValue> {
        self.sink.abort().await
    }

    /// [Aborts](https://streams.spec.whatwg.org/#abort-a-writable-stream) the stream,
    /// signaling that the producer can no longer successfully write to the stream.
    pub async fn abort_with_reason(self, reason: &JsValue) -> Result<(), JsValue> {
        self.sink.abort_with_reason(reason).await
    }
}

impl<'writer> AsyncWrite for IntoAsyncWrite<'writer> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        ready!(self
            .as_mut()
            .sink
            .poll_ready_unpin(cx)
            .map_err(js_to_io_error))?;
        self.as_mut()
            .sink
            .start_send_unpin(Uint8Array::from(buf).into())
            .map_err(js_to_io_error)?;
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.as_mut()
            .sink
            .poll_flush_unpin(cx)
            .map_err(js_to_io_error)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.as_mut()
            .sink
            .poll_close_unpin(cx)
            .map_err(js_to_io_error)
    }
}
