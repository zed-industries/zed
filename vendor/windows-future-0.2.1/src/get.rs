use super::*;

impl IAsyncAction {
    /// Waits for the `IAsyncAction` to finish.
    pub fn get(&self) -> Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = Waiter::new()?;
            self.SetCompleted(&AsyncActionCompletedHandler::new(move |_, _| {
                // This is safe because the waiter will only be dropped after being signaled.
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}

impl<T: RuntimeType> IAsyncOperation<T> {
    /// Waits for the `IAsyncOperation<T>` to finish.
    pub fn get(&self) -> Result<T> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = Waiter::new()?;
            self.SetCompleted(&AsyncOperationCompletedHandler::new(move |_, _| {
                // This is safe because the waiter will only be dropped after being signaled.
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress<P> {
    /// Waits for the `IAsyncActionWithProgress<P>` to finish.
    pub fn get(&self) -> Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = Waiter::new()?;
            self.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
                move |_, _| {
                    // This is safe because the waiter will only be dropped after being signaled.
                    unsafe {
                        signaler.signal();
                    }
                    Ok(())
                },
            ))?;
        }
        self.GetResults()
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
    /// Waits for the `IAsyncOperationWithProgress<T, P>` to finish.
    pub fn get(&self) -> Result<T> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = Waiter::new()?;
            self.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
                move |_, _| {
                    // This is safe because the waiter will only be dropped after being signaled.
                    unsafe {
                        signaler.signal();
                    }
                    Ok(())
                },
            ))?;
        }
        self.GetResults()
    }
}
