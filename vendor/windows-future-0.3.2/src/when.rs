use super::*;

impl IAsyncAction {
    /// Calls `op(result)` when the `IAsyncAction` completes.
    pub fn when<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(Result<()>) + Send + 'static,
    {
        Async::when(self, op)
    }
}

impl<T: RuntimeType> IAsyncOperation<T> {
    /// Calls `op(result)` when the `IAsyncOperation<T>` completes.
    pub fn when<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(Result<T>) + Send + 'static,
    {
        Async::when(self, op)
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress<P> {
    /// Calls `op(result)` when the `IAsyncActionWithProgress<P>` completes.
    pub fn when<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(Result<()>) + Send + 'static,
    {
        Async::when(self, op)
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
    /// Calls `op(result)` when the `IAsyncOperationWithProgress<T, P>` completes.
    pub fn when<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(Result<T>) + Send + 'static,
    {
        Async::when(self, op)
    }
}
