use super::*;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

// An `Async` represents a WinRT async execution object or type. There are precisely four such types:
// - IAsyncAction
// - IAsyncActionWithProgress
// - IAsyncOperation
// - IAsyncOperationWithProgress
//
// All four implementations are provided here and there is thus no need to implement this trait.
// This trait provides an abstraction over the relevant differences so that the `AsyncFuture`
// implementation below can be reused for all of them.
pub trait Async: Interface {
    // The type of value produced on completion.
    type Output: Clone;

    // The type of the delegate use for completion notification.
    type CompletedHandler: Interface;

    // Sets the handler or callback to invoke when execution completes. This handler can only be set once.
    fn set_completed<F: Fn() + Send + 'static>(&self, handler: F) -> Result<()>;

    // Calls the given handler with the current object and status.
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus);

    // Returns the value produced on completion. This should only be called when execution completes.
    fn get_results(&self) -> Result<Self::Output>;
}

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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
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
            self.inner.set_completed(move || {
                shared_waker.lock().unwrap().wake_by_ref();
            })?;
        };

        Poll::Pending
    }
}

//
// The four `Async` trait implementations.
//

impl Async for IAsyncAction {
    type Output = ();
    type CompletedHandler = AsyncActionCompletedHandler;

    fn set_completed<F: Fn() + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncActionCompletedHandler::new(move |_, _| {
            handler();
            Ok(())
        }))
    }

    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }
}

impl<T: RuntimeType> Async for IAsyncOperation<T> {
    type Output = T;
    type CompletedHandler = AsyncOperationCompletedHandler<T>;

    fn set_completed<F: Fn() + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncOperationCompletedHandler::new(move |_, _| {
            handler();
            Ok(())
        }))
    }

    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }
}

impl<P: RuntimeType> Async for IAsyncActionWithProgress<P> {
    type Output = ();
    type CompletedHandler = AsyncActionWithProgressCompletedHandler<P>;

    fn set_completed<F: Fn() + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
            move |_, _| {
                handler();
                Ok(())
            },
        ))
    }

    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }
}

impl<T: RuntimeType, P: RuntimeType> Async for IAsyncOperationWithProgress<T, P> {
    type Output = T;
    type CompletedHandler = AsyncOperationWithProgressCompletedHandler<T, P>;

    fn set_completed<F: Fn() + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
            move |_, _| {
                handler();
                Ok(())
            },
        ))
    }

    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
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
