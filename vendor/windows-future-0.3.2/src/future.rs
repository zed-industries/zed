use super::*;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

// The `AsyncFuture` is needed to store some extra state needed to keep async execution up to date with possible changes
// to Rust execution context. Each async type implements `IntoFuture` rather than implementing `Future` directly so that
// this adapter may be used.
pub struct AsyncFuture<A: Async> {
    // Represents the async execution and provides the virtual methods for setting up a `Completed` handler and
    // calling `GetResults` when execution is completed.
    inner: A,

    // Provides the `Status` virtual method and saves repeated calls to `QueryInterface` during polling.
    status: IAsyncInfo,

    // A shared waker is needed to keep the `Completed` handler updated.
    // - `Option` is used to avoid allocations for async objects that have already completed.
    // - `Arc` is used to share the `Waker` with the `Completed` handler and potentially replace the `Waker`
    //   since we don't have the ability to replace the `Completed` handler itself.
    // - `Mutex` is used to synchronize replacing the `Waker` across threads.
    waker: Option<Arc<Mutex<Waker>>>,
}

impl<A: Async> AsyncFuture<A> {
    fn new(inner: A) -> Self {
        Self {
            // All four async interfaces implement `IAsyncInfo` so this `cast` will always succeed.
            status: inner.cast().unwrap(),
            inner,
            waker: None,
        }
    }
}

unsafe impl<A: Async> Send for AsyncFuture<A> {}
unsafe impl<A: Async> Sync for AsyncFuture<A> {}
impl<A: Async> Unpin for AsyncFuture<A> {}

impl<A: Async> Future for AsyncFuture<A> {
    type Output = Result<A::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // A status of `Started` just means async execution is still in flight. Since WinRT async is always
        // "hot start", if its not `Started` then its ready for us to call `GetResults` so we can skip all of
        // the remaining set up.
        if self.status.Status()? != AsyncStatus::Started {
            return Poll::Ready(self.inner.get_results());
        }

        if let Some(shared_waker) = &self.waker {
            // We have a shared waker which means we're either getting polled again or been transferred to
            // another execution context. As we can't tell the difference, we need to update the shared waker
            // to make sure we've got the "current" waker.
            let mut guard = shared_waker.lock().unwrap();
            guard.clone_from(cx.waker());

            // It may be possible that the `Completed` handler acquired the lock and signaled the old waker
            // before we managed to acquire the lock to update it with the current waker. We check the status
            // again here just in case this happens.
            if self.status.Status()? != AsyncStatus::Started {
                return Poll::Ready(self.inner.get_results());
            }
        } else {
            // If we don't have a saved waker it means this is the first time we're getting polled and should
            // create the shared waker and set up a `Completed` handler.
            let shared_waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(shared_waker.clone());

            // Note that the handler can only be set once, which is why we need a shared waker in the first
            // place. On the other hand, the handler will get called even if async execution has already
            // completed, so we can just return `Pending` after setting the Completed handler.
            self.inner.set_completed(move |_| {
                shared_waker.lock().unwrap().wake_by_ref();
            })?;
        };

        Poll::Pending
    }
}

//
// The four `IntoFuture` trait implementations.
//

impl IntoFuture for IAsyncAction {
    type Output = Result<()>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

impl<T: RuntimeType> IntoFuture for IAsyncOperation<T> {
    type Output = Result<T>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

impl<P: RuntimeType> IntoFuture for IAsyncActionWithProgress<P> {
    type Output = Result<()>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

impl<T: RuntimeType, P: RuntimeType> IntoFuture for IAsyncOperationWithProgress<T, P> {
    type Output = Result<T>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}
