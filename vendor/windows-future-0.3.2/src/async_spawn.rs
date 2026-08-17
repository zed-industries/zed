use super::*;
use std::sync::Mutex;

struct State<T: Async> {
    result: Option<Result<T::Output>>,
    completed: Option<T::CompletedHandler>,
    completed_assigned: bool,
}

impl<T: Async> State<T> {
    fn status(&self) -> AsyncStatus {
        match &self.result {
            None => AsyncStatus::Started,
            Some(Ok(_)) => AsyncStatus::Completed,
            Some(Err(_)) => AsyncStatus::Error,
        }
    }

    fn error_code(&self) -> HRESULT {
        match &self.result {
            Some(Err(error)) => error.code(),
            _ => HRESULT(0),
        }
    }

    fn get_results(&self) -> Result<T::Output> {
        match &self.result {
            Some(result) => result.clone(),
            None => Err(Error::from_hresult(HRESULT(0x8000000Eu32 as i32))), // E_ILLEGAL_METHOD_CALL
        }
    }
}

struct SyncState<T: Async>(Mutex<State<T>>);

impl<T: Async> SyncState<T> {
    fn new() -> Self {
        Self(Mutex::new(State {
            result: None,
            completed: None,
            completed_assigned: false,
        }))
    }

    fn status(&self) -> AsyncStatus {
        self.0.lock().unwrap().status()
    }

    fn error_code(&self) -> HRESULT {
        self.0.lock().unwrap().error_code()
    }

    fn get_results(&self) -> Result<T::Output> {
        self.0.lock().unwrap().get_results()
    }

    fn set_completed(&self, sender: &T, handler: Ref<T::CompletedHandler>) -> Result<()> {
        let mut guard = self.0.lock().unwrap();

        if guard.completed_assigned {
            Err(Error::from_hresult(HRESULT(0x80000018u32 as i32))) // E_ILLEGAL_DELEGATE_ASSIGNMENT
        } else {
            guard.completed_assigned = true;
            let status = guard.status();
            let handler = handler.ok()?;

            if status == AsyncStatus::Started {
                guard.completed = Some(handler.clone());
            } else {
                drop(guard);
                sender.invoke_completed(handler, status);
            }

            Ok(())
        }
    }

    fn spawn<F>(&self, sender: &T, f: F)
    where
        F: FnOnce() -> Result<T::Output> + Send + 'static,
    {
        let result = f();
        let mut guard = self.0.lock().unwrap();
        debug_assert!(guard.result.is_none());
        guard.result = Some(result);
        let status = guard.status();
        let completed = guard.completed.take();

        drop(guard);

        if let Some(completed) = completed {
            sender.invoke_completed(&completed, status);
        }
    }
}

unsafe impl<T: Async> Send for SyncState<T> {}

#[implement(IAsyncAction, IAsyncInfo)]
struct Action(SyncState<IAsyncAction>);

#[implement(IAsyncOperation<T>, IAsyncInfo)]
struct Operation<T>(SyncState<IAsyncOperation<T>>)
where
    T: RuntimeType + 'static;

#[implement(IAsyncActionWithProgress<P>, IAsyncInfo)]
struct ActionWithProgress<P>(SyncState<IAsyncActionWithProgress<P>>)
where
    P: RuntimeType + 'static;

#[implement(IAsyncOperationWithProgress<T, P>, IAsyncInfo)]
struct OperationWithProgress<T, P>(SyncState<IAsyncOperationWithProgress<T, P>>)
where
    T: RuntimeType + 'static,
    P: RuntimeType + 'static;

impl IAsyncInfo_Impl for Action_Impl {
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

impl<T: RuntimeType> IAsyncInfo_Impl for Operation_Impl<T> {
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

impl<P: RuntimeType> IAsyncInfo_Impl for ActionWithProgress_Impl<P> {
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

impl<T: RuntimeType, P: RuntimeType> IAsyncInfo_Impl for OperationWithProgress_Impl<T, P> {
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

impl IAsyncAction_Impl for Action_Impl {
    fn SetCompleted(&self, handler: Ref<AsyncActionCompletedHandler>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionCompletedHandler> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.get_results()
    }
}

impl<T: RuntimeType> IAsyncOperation_Impl<T> for Operation_Impl<T> {
    fn SetCompleted(&self, handler: Ref<AsyncOperationCompletedHandler<T>>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncOperationCompletedHandler<T>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<T> {
        self.0.get_results()
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress_Impl<P> for ActionWithProgress_Impl<P> {
    fn SetCompleted(&self, handler: Ref<AsyncActionWithProgressCompletedHandler<P>>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionWithProgressCompletedHandler<P>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.get_results()
    }
    fn SetProgress(&self, _: Ref<AsyncActionProgressHandler<P>>) -> Result<()> {
        Ok(())
    }
    fn Progress(&self) -> Result<AsyncActionProgressHandler<P>> {
        Err(Error::empty())
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress_Impl<T, P>
    for OperationWithProgress_Impl<T, P>
{
    fn SetCompleted(
        &self,
        handler: Ref<AsyncOperationWithProgressCompletedHandler<T, P>>,
    ) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncOperationWithProgressCompletedHandler<T, P>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<T> {
        self.0.get_results()
    }
    fn SetProgress(&self, _: Ref<AsyncOperationProgressHandler<T, P>>) -> Result<()> {
        Ok(())
    }
    fn Progress(&self) -> Result<AsyncOperationProgressHandler<T, P>> {
        Err(Error::empty())
    }
}

impl IAsyncAction {
    /// Creates an `IAsyncAction` that waits for the closure to execute on the Windows thread pool.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let object = ComObject::new(Action(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

impl<T: RuntimeType> IAsyncOperation<T> {
    /// Creates an `IAsyncOperation<T>` that waits for the closure to execute on the Windows thread pool.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        let object = ComObject::new(Operation(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress<P> {
    /// Creates an `IAsyncActionWithProgress<P>` that waits for the closure to execute on the Windows thread pool.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let object = ComObject::new(ActionWithProgress(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
    /// Creates an `IAsyncOperationWithProgress<T, P>` that waits for the closure to execute on the Windows thread pool.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        let object = ComObject::new(OperationWithProgress(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}
