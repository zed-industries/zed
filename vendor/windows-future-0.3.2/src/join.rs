use super::*;

impl IAsyncAction {
    /// Waits for the `IAsyncAction` to finish.
    pub fn join(&self) -> Result<()> {
        Async::join(self)
    }
}

impl<T: RuntimeType> IAsyncOperation<T> {
    /// Waits for the `IAsyncOperation<T>` to finish.
    pub fn join(&self) -> Result<T> {
        Async::join(self)
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress<P> {
    /// Waits for the `IAsyncActionWithProgress<P>` to finish.
    pub fn join(&self) -> Result<()> {
        Async::join(self)
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
    /// Waits for the `IAsyncOperationWithProgress<T, P>` to finish.
    pub fn join(&self) -> Result<T> {
        Async::join(self)
    }
}
