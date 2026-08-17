use super::*;

// An `Async` represents a WinRT async execution object or type. There are precisely four such types:
// - IAsyncAction
// - IAsyncActionWithProgress
// - IAsyncOperation
// - IAsyncOperationWithProgress
//
// All four implementations are provided here and there is thus no need to implement this trait.
// This trait provides an abstraction over the relevant differences so that the various async
// capabilities in this crate can be reused for all implementations.
pub trait Async: Interface {
    // The type of value produced on completion.
    type Output: Clone;

    // The type of the delegate use for completion notification.
    type CompletedHandler: Interface;

    // Sets the handler or callback to invoke when execution completes. This handler can only be set once.
    fn set_completed<F: Fn(&Self) + Send + 'static>(&self, handler: F) -> Result<()>;

    // Calls the given handler with the current object and status.
    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus);

    // Returns the value produced on completion. This should only be called when execution completes.
    fn get_results(&self) -> Result<Self::Output>;

    // Gets the current status of async execution. This calls `QueryInterface` so should be used sparingly.
    fn status(&self) -> Result<AsyncStatus>;

    // Waits for the async execution to finish and then returns the results.
    fn join(&self) -> Result<Self::Output> {
        if self.status()? == AsyncStatus::Started {
            let (_waiter, signaler) = Waiter::new()?;
            self.set_completed(move |_| {
                // This is safe because the waiter will only be dropped after being signaled.
                unsafe {
                    signaler.signal();
                }
            })?;
        }
        self.get_results()
    }

    // Calls `op(result)` when async execution completes.
    fn when<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(Result<Self::Output>) + Send + 'static,
    {
        if self.status()? == AsyncStatus::Started {
            // The `set_completed` closure is guaranteed to only be called once, like `FnOnce`, by the async pattern,
            // but Rust doesn't know that so `RefCell` is used to pass `op` in to the closure.
            let op = core::cell::RefCell::new(Some(op));
            self.set_completed(move |sender| {
                if let Some(op) = op.take() {
                    op(sender.get_results());
                }
            })?;
        } else {
            op(self.get_results());
        }
        Ok(())
    }
}

impl Async for IAsyncAction {
    type Output = ();
    type CompletedHandler = AsyncActionCompletedHandler;

    fn set_completed<F: Fn(&Self) + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, _| {
            handler(sender.ok()?);
            Ok(())
        }))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

impl<T: RuntimeType> Async for IAsyncOperation<T> {
    type Output = T;
    type CompletedHandler = AsyncOperationCompletedHandler<T>;

    fn set_completed<F: Fn(&Self) + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncOperationCompletedHandler::new(move |sender, _| {
            handler(sender.ok()?);
            Ok(())
        }))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

impl<P: RuntimeType> Async for IAsyncActionWithProgress<P> {
    type Output = ();
    type CompletedHandler = AsyncActionWithProgressCompletedHandler<P>;

    fn set_completed<F: Fn(&Self) + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
            move |sender, _| {
                handler(sender.ok()?);
                Ok(())
            },
        ))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

impl<T: RuntimeType, P: RuntimeType> Async for IAsyncOperationWithProgress<T, P> {
    type Output = T;
    type CompletedHandler = AsyncOperationWithProgressCompletedHandler<T, P>;

    fn set_completed<F: Fn(&Self) + Send + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
            move |sender, _| {
                handler(sender.ok()?);
                Ok(())
            },
        ))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}
