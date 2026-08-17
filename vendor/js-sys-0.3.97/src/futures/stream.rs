//! Converting JavaScript `AsyncIterator`s to Rust `Stream`s.
//!
//! Analogous to the promise to future conversion, this module allows
//! turning objects implementing the async iterator protocol into `Stream`s
//! that produce values that can be awaited from.
//!

use super::JsFuture;
use crate::{AsyncIterator, IteratorNext};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::stream::Stream;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::{prelude::*, JsGeneric};

/// A `Stream` that yields values from an underlying `AsyncIterator`.
pub struct JsStream<T = JsValue> {
    iter: AsyncIterator<T>,
    next: Option<JsFuture<IteratorNext<T>>>,
    done: bool,
}

impl<T: JsGeneric + FromWasmAbi> JsStream<T> {
    fn next_future(&self) -> Result<JsFuture<IteratorNext<T>>, JsValue> {
        self.iter.next_iterator().map(JsFuture::from)
    }
}

impl<T> From<AsyncIterator<T>> for JsStream<T> {
    fn from(iter: AsyncIterator<T>) -> Self {
        JsStream {
            iter,
            next: None,
            done: false,
        }
    }
}

impl<T: JsGeneric + FromWasmAbi + Unpin> Stream for JsStream<T> {
    type Item = Result<T, JsValue>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if self.done {
            return Poll::Ready(None);
        }

        let future = match self.next.as_mut() {
            Some(val) => val,
            None => match self.next_future() {
                Ok(val) => {
                    self.next = Some(val);
                    self.next.as_mut().unwrap()
                }
                Err(e) => {
                    self.done = true;
                    return Poll::Ready(Some(Err(e)));
                }
            },
        };

        match Pin::new(future).poll(cx) {
            Poll::Ready(res) => match res {
                Ok(next) => {
                    if next.done() {
                        self.done = true;
                        Poll::Ready(None)
                    } else {
                        self.next.take();
                        Poll::Ready(Some(Ok(next.value())))
                    }
                }
                Err(e) => {
                    self.done = true;
                    Poll::Ready(Some(Err(e)))
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
