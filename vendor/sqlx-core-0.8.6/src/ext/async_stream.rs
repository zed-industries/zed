//! A minimalist clone of the `async-stream` crate in 100% safe code, without proc macros.
//!
//! This was created initially to get around some weird compiler errors we were getting with
//! `async-stream`, and now it'd just be more work to replace.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use futures_core::future::BoxFuture;
use futures_core::stream::Stream;
use futures_core::FusedFuture;
use futures_util::future::Fuse;
use futures_util::FutureExt;

use crate::error::Error;

pub struct TryAsyncStream<'a, T> {
    yielder: Yielder<T>,
    future: Fuse<BoxFuture<'a, Result<(), Error>>>,
}

impl<'a, T> TryAsyncStream<'a, T> {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: FnOnce(Yielder<T>) -> Fut + Send,
        Fut: 'a + Future<Output = Result<(), Error>> + Send,
        T: 'a + Send,
    {
        let yielder = Yielder::new();

        let future = f(yielder.duplicate()).boxed().fuse();

        Self { future, yielder }
    }
}

pub struct Yielder<T> {
    // This mutex should never have any contention in normal operation.
    // We're just using it because `Rc<Cell<Option<T>>>` would not be `Send`.
    value: Arc<Mutex<Option<T>>>,
}

impl<T> Yielder<T> {
    fn new() -> Self {
        Yielder {
            value: Arc::new(Mutex::new(None)),
        }
    }

    // Don't want to expose a `Clone` impl
    fn duplicate(&self) -> Self {
        Yielder {
            value: self.value.clone(),
        }
    }

    /// NOTE: may deadlock the task if called from outside the future passed to `TryAsyncStream`.
    pub async fn r#yield(&self, val: T) {
        let replaced = self
            .value
            .lock()
            .expect("BUG: panicked while holding a lock")
            .replace(val);

        debug_assert!(
            replaced.is_none(),
            "BUG: previously yielded value not taken"
        );

        let mut yielded = false;

        // Allows the generating future to suspend its execution without changing the task priority,
        // which would happen with `tokio::task::yield_now()`.
        //
        // Note that because this has no way to schedule a wakeup, this could deadlock the task
        // if called in the wrong place.
        futures_util::future::poll_fn(|_cx| {
            if !yielded {
                yielded = true;
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await
    }

    fn take(&self) -> Option<T> {
        self.value
            .lock()
            .expect("BUG: panicked while holding a lock")
            .take()
    }
}

impl<'a, T> Stream for TryAsyncStream<'a, T> {
    type Item = Result<T, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.future.is_terminated() {
            return Poll::Ready(None);
        }

        match self.future.poll_unpin(cx) {
            Poll::Ready(Ok(())) => {
                // Future returned without yielding another value,
                // or else it would have returned `Pending` instead.
                Poll::Ready(None)
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => self
                .yielder
                .take()
                .map_or(Poll::Pending, |val| Poll::Ready(Some(Ok(val)))),
        }
    }
}

#[macro_export]
macro_rules! try_stream {
    ($($block:tt)*) => {
        $crate::ext::async_stream::TryAsyncStream::new(move |yielder| ::tracing::Instrument::in_current_span(async move {
            // Anti-footgun: effectively pins `yielder` to this future to prevent any accidental
            // move to another task, which could deadlock.
            let yielder = &yielder;

            macro_rules! r#yield {
                ($v:expr) => {{
                    yielder.r#yield($v).await;
                }}
            }

            $($block)*
        }))
    }
}
