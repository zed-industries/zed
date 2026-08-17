use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;

use futures_util::future::{abortable, AbortHandle, TryFutureExt};
use futures_util::stream::{Stream, TryStreamExt};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use super::sys;

type JsValueStream = dyn Stream<Item = Result<JsValue, JsValue>>;

#[wasm_bindgen]
pub(crate) struct IntoUnderlyingSource {
    inner: Rc<RefCell<Inner>>,
    pull_handle: Option<AbortHandle>,
}

impl IntoUnderlyingSource {
    pub fn new(stream: Box<JsValueStream>) -> Self {
        IntoUnderlyingSource {
            inner: Rc::new(RefCell::new(Inner::new(stream))),
            pull_handle: None,
        }
    }
}

#[allow(clippy::await_holding_refcell_ref)]
#[wasm_bindgen]
impl IntoUnderlyingSource {
    pub fn pull(&mut self, controller: sys::ReadableStreamDefaultController) -> Promise {
        let inner = self.inner.clone();
        let fut = async move {
            // This mutable borrow can never panic, since the ReadableStream always queues
            // each operation on the underlying source.
            let mut inner = inner.try_borrow_mut().unwrap_throw();
            inner.pull(controller).await
        };

        // Allow aborting the future from cancel().
        let (fut, handle) = abortable(fut);
        // Ignore errors from aborting the future.
        let fut = fut.unwrap_or_else(|_| Ok(JsValue::undefined()));

        self.pull_handle = Some(handle);
        future_to_promise(fut)
    }

    pub fn cancel(self) {
        // The stream has been canceled, drop everything.
        drop(self);
    }
}

impl Drop for IntoUnderlyingSource {
    fn drop(&mut self) {
        // Abort the pending pull, if any.
        if let Some(handle) = self.pull_handle.take() {
            handle.abort();
        }
    }
}

struct Inner {
    stream: Option<Pin<Box<JsValueStream>>>,
}

impl Inner {
    fn new(stream: Box<JsValueStream>) -> Self {
        Inner {
            stream: Some(stream.into()),
        }
    }

    async fn pull(
        &mut self,
        controller: sys::ReadableStreamDefaultController,
    ) -> Result<JsValue, JsValue> {
        // The stream should still exist, since pull() will not be called again
        // after the stream has closed or encountered an error.
        let stream = self.stream.as_mut().unwrap_throw();
        match stream.try_next().await {
            Ok(Some(chunk)) => controller.enqueue_with_chunk(&chunk)?,
            Ok(None) => {
                // The stream has closed, drop it.
                self.stream = None;
                controller.close()?;
            }
            Err(err) => {
                // The stream encountered an error, drop it.
                self.stream = None;
                return Err(err);
            }
        };
        Ok(JsValue::undefined())
    }
}
