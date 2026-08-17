use core::pin::Pin;
use core::task::{Context, Poll};

use futures_util::Sink;
use futures_util::{ready, FutureExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use super::WritableStreamDefaultWriter;

/// A [`Sink`] for the [`into_sink`](super::WritableStream::into_sink) method.
///
/// This sink holds a writer, and therefore locks the [`WritableStream`](super::WritableStream).
/// When this sink is dropped, it also drops its writer which in turn
/// [releases its lock](https://streams.spec.whatwg.org/#release-a-lock).
///
/// [`Sink`]: https://docs.rs/futures/0.3.30/futures/sink/trait.Sink.html
#[must_use = "sinks do nothing unless polled"]
#[derive(Debug)]
pub struct IntoSink<'writer> {
    writer: Option<WritableStreamDefaultWriter<'writer>>,
    ready_fut: Option<JsFuture>,
    write_fut: Option<JsFuture>,
    close_fut: Option<JsFuture>,
}

impl<'writer> IntoSink<'writer> {
    #[inline]
    pub(super) fn new(writer: WritableStreamDefaultWriter) -> IntoSink {
        IntoSink {
            writer: Some(writer),
            ready_fut: None,
            write_fut: None,
            close_fut: None,
        }
    }

    /// [Aborts](https://streams.spec.whatwg.org/#abort-a-writable-stream) the stream,
    /// signaling that the producer can no longer successfully write to the stream.
    pub async fn abort(mut self) -> Result<(), JsValue> {
        match self.writer.take() {
            Some(mut writer) => writer.abort().await,
            None => Ok(()),
        }
    }

    /// [Aborts](https://streams.spec.whatwg.org/#abort-a-writable-stream) the stream,
    /// signaling that the producer can no longer successfully write to the stream.
    pub async fn abort_with_reason(mut self, reason: &JsValue) -> Result<(), JsValue> {
        match self.writer.take() {
            Some(mut writer) => writer.abort_with_reason(reason).await,
            None => Ok(()),
        }
    }
}

impl<'writer> Sink<JsValue> for IntoSink<'writer> {
    type Error = JsValue;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let ready_fut = match self.ready_fut.as_mut() {
            Some(fut) => fut,
            None => match &self.writer {
                Some(writer) => {
                    // No pending ready future yet, create one from ready promise
                    let fut = JsFuture::from(writer.as_raw().ready());
                    self.ready_fut.insert(fut)
                }
                None => {
                    // Writer was already dropped
                    // TODO Return error?
                    return Poll::Ready(Ok(()));
                }
            },
        };

        // Poll the ready future
        let js_result = ready!(ready_fut.poll_unpin(cx));
        self.ready_fut = None;

        // Ready future completed
        Poll::Ready(match js_result {
            Ok(js_value) => {
                debug_assert!(js_value.is_undefined());
                Ok(())
            }
            Err(js_value) => {
                // Error, drop writer
                self.writer = None;
                Err(js_value)
            }
        })
    }

    fn start_send(mut self: Pin<&mut Self>, item: JsValue) -> Result<(), Self::Error> {
        match &self.writer {
            Some(writer) => {
                let fut = JsFuture::from(writer.as_raw().write_with_chunk(&item));
                // Set or replace the pending write future
                self.write_fut = Some(fut);
                Ok(())
            }
            None => {
                // TODO Return error?
                Ok(())
            }
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let write_fut = match self.write_fut.as_mut() {
            Some(fut) => fut,
            None => {
                // If we're not writing, then there's nothing to flush
                return Poll::Ready(Ok(()));
            }
        };

        // Poll the write future
        let js_result = ready!(write_fut.poll_unpin(cx));
        self.write_fut = None;

        // Write future completed
        Poll::Ready(match js_result {
            Ok(js_value) => {
                debug_assert!(js_value.is_undefined());
                Ok(())
            }
            Err(js_value) => {
                // Error, drop writer
                self.writer = None;
                Err(js_value)
            }
        })
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let close_fut = match self.close_fut.as_mut() {
            Some(fut) => fut,
            None => match &self.writer {
                Some(writer) => {
                    // No pending close future
                    // Start closing the stream and create future from close promise
                    let fut = JsFuture::from(writer.as_raw().close());
                    self.close_fut.insert(fut)
                }
                None => {
                    // Writer was already dropped
                    // TODO Return error?
                    return Poll::Ready(Ok(()));
                }
            },
        };

        // Poll the close future
        let js_result = ready!(close_fut.poll_unpin(cx));
        self.close_fut = None;

        // Close future completed
        self.writer = None;
        Poll::Ready(match js_result {
            Ok(js_value) => {
                debug_assert!(js_value.is_undefined());
                Ok(())
            }
            Err(js_value) => Err(js_value),
        })
    }
}
