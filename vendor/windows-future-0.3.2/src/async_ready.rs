use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

struct ReadyState<T: Async> {
    set_completed: AtomicBool,
    result: Result<T::Output>,
}

impl<T: Async> ReadyState<T> {
    fn new(result: Result<T::Output>) -> Self {
        Self {
            set_completed: AtomicBool::new(false),
            result,
        }
    }

    fn status(&self) -> AsyncStatus {
        if self.result.is_ok() {
            AsyncStatus::Completed
        } else {
            AsyncStatus::Error
        }
    }

    // The "Ready" implementations don't need to store the handler since the handler is invoked immediately
    // but still need to confirm that `SetCompleted` is called at most once.
    fn invoke_completed(&self, sender: &T, handler: Ref<T::CompletedHandler>) -> Result<()> {
        if !self.set_completed.swap(true, Ordering::SeqCst) {
            sender.invoke_completed(handler.ok()?, self.status());
            Ok(())
        } else {
            Err(Error::from_hresult(HRESULT(0x80000018u32 as i32))) // E_ILLEGAL_DELEGATE_ASSIGNMENT
        }
    }

    // The `From` implementation is not used here since we don't want to transfer any error object to the calling thread.
    // That happens when `GetResults` is called.
    fn error_code(&self) -> HRESULT {
        match &self.result {
            Ok(_) => HRESULT(0),
            Err(error) => error.code(),
        }
    }
}

#[implement(IAsyncAction, IAsyncInfo)]
struct ReadyAction(ReadyState<IAsyncAction>);

#[implement(IAsyncOperation<T>, IAsyncInfo)]
struct ReadyOperation<T>(ReadyState<IAsyncOperation<T>>)
where
    T: RuntimeType + 'static;

#[implement(IAsyncActionWithProgress<P>, IAsyncInfo)]
struct ReadyActionWithProgress<P>(ReadyState<IAsyncActionWithProgress<P>>)
where
    P: RuntimeType + 'static;

#[implement(IAsyncOperationWithProgress<T, P>, IAsyncInfo)]
struct ReadyOperationWithProgress<T, P>(ReadyState<IAsyncOperationWithProgress<T, P>>)
where
    T: RuntimeType + 'static,
    P: RuntimeType + 'static;

impl IAsyncInfo_Impl for ReadyAction_Impl {
    fn Id(&self) -> Result<u32> {
        Ok(1)
    }
    fn Status(&self) -> Result<AsyncStatus> {
        Ok(self.0.status())
    }
    fn ErrorCode(&self) -> Result<HRESULT> {
        Ok(self.0.error_code())
    }
    fn Cancel(&self) -> Result<()> {
        Ok(())
    }
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl<T: RuntimeType> IAsyncInfo_Impl for ReadyOperation_Impl<T> {
    fn Id(&self) -> Result<u32> {
        Ok(1)
    }
    fn Status(&self) -> Result<AsyncStatus> {
        Ok(self.0.status())
    }
    fn ErrorCode(&self) -> Result<HRESULT> {
        Ok(self.0.error_code())
    }
    fn Cancel(&self) -> Result<()> {
        Ok(())
    }
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl<P: RuntimeType> IAsyncInfo_Impl for ReadyActionWithProgress_Impl<P> {
    fn Id(&self) -> Result<u32> {
        Ok(1)
    }
    fn Status(&self) -> Result<AsyncStatus> {
        Ok(self.0.status())
    }
    fn ErrorCode(&self) -> Result<HRESULT> {
        Ok(self.0.error_code())
    }
    fn Cancel(&self) -> Result<()> {
        Ok(())
    }
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncInfo_Impl for ReadyOperationWithProgress_Impl<T, P> {
    fn Id(&self) -> Result<u32> {
        Ok(1)
    }
    fn Status(&self) -> Result<AsyncStatus> {
        Ok(self.0.status())
    }
    fn ErrorCode(&self) -> Result<HRESULT> {
        Ok(self.0.error_code())
    }
    fn Cancel(&self) -> Result<()> {
        Ok(())
    }
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl IAsyncAction_Impl for ReadyAction_Impl {
    fn SetCompleted(&self, handler: Ref<AsyncActionCompletedHandler>) -> Result<()> {
        self.0.invoke_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionCompletedHandler> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.result.clone()
    }
}

impl<T: RuntimeType> IAsyncOperation_Impl<T> for ReadyOperation_Impl<T> {
    fn SetCompleted(&self, handler: Ref<AsyncOperationCompletedHandler<T>>) -> Result<()> {
        self.0.invoke_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncOperationCompletedHandler<T>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<T> {
        self.0.result.clone()
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress_Impl<P> for ReadyActionWithProgress_Impl<P> {
    fn SetCompleted(&self, handler: Ref<AsyncActionWithProgressCompletedHandler<P>>) -> Result<()> {
        self.0.invoke_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionWithProgressCompletedHandler<P>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.result.clone()
    }
    fn SetProgress(&self, _: Ref<AsyncActionProgressHandler<P>>) -> Result<()> {
        Ok(())
    }
    fn Progress(&self) -> Result<AsyncActionProgressHandler<P>> {
        Err(Error::empty())
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress_Impl<T, P>
    for ReadyOperationWithProgress_Impl<T, P>
{
    fn SetCompleted(
        &self,
        handler: Ref<AsyncOperationWithProgressCompletedHandler<T, P>>,
    ) -> Result<()> {
        self.0.invoke_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncOperationWithProgressCompletedHandler<T, P>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<T> {
        self.0.result.clone()
    }
    fn SetProgress(&self, _: Ref<AsyncOperationProgressHandler<T, P>>) -> Result<()> {
        Ok(())
    }
    fn Progress(&self) -> Result<AsyncOperationProgressHandler<T, P>> {
        Err(Error::empty())
    }
}

impl IAsyncAction {
    /// Creates an `IAsyncAction` that is immediately ready with a value.
    pub fn ready(result: Result<()>) -> Self {
        ReadyAction(ReadyState::new(result)).into()
    }
}

impl<T: RuntimeType> IAsyncOperation<T> {
    /// Creates an `IAsyncOperation<T>` that is immediately ready with a value.
    pub fn ready(result: Result<T>) -> Self {
        ReadyOperation(ReadyState::new(result)).into()
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress<P> {
    /// Creates an `IAsyncActionWithProgress<P>` that is immediately ready with a value.
    pub fn ready(result: Result<()>) -> Self {
        ReadyActionWithProgress(ReadyState::new(result)).into()
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
    /// Creates an `IAsyncOperationWithProgress<T, P>` that is immediately ready with a value.
    pub fn ready(result: Result<T>) -> Self {
        ReadyOperationWithProgress(ReadyState::new(result)).into()
    }
}
