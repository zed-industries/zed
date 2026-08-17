use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::io;
use crate::task::{JoinHandle, Task, TaskLocalsWrapper};

/// Task builder that configures the settings of a new task.
#[derive(Debug, Default)]
pub struct Builder {
    pub(crate) name: Option<String>,
}

impl Builder {
    /// Creates a new builder.
    #[inline]
    pub fn new() -> Builder {
        Builder { name: None }
    }

    /// Configures the name of the task.
    #[inline]
    pub fn name(mut self, name: String) -> Builder {
        self.name = Some(name);
        self
    }

    fn build<F, T>(self, future: F) -> SupportTaskLocals<F>
    where
        F: Future<Output = T>,
    {
        let name = self.name.map(Arc::new);

        // Create a new task handle.
        let task = Task::new(name);

        #[cfg(not(target_os = "unknown"))]
        once_cell::sync::Lazy::force(&crate::rt::RUNTIME);

        let tag = TaskLocalsWrapper::new(task);

        SupportTaskLocals { tag, future }
    }

    /// Spawns a task with the configured settings.
    #[cfg(not(target_os = "unknown"))]
    pub fn spawn<F, T>(self, future: F) -> io::Result<JoinHandle<T>>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let wrapped = self.build(future);

        kv_log_macro::trace!("spawn", {
            task_id: wrapped.tag.id().0,
            parent_task_id: TaskLocalsWrapper::get_current(|t| t.id().0).unwrap_or(0),
        });

        let task = wrapped.tag.task().clone();
        let handle = async_global_executor::spawn(wrapped);

        Ok(JoinHandle::new(handle, task))
    }

    /// Spawns a task locally with the configured settings.
    #[cfg(all(not(target_os = "unknown"), feature = "unstable"))]
    pub fn local<F, T>(self, future: F) -> io::Result<JoinHandle<T>>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        let wrapped = self.build(future);

        kv_log_macro::trace!("spawn_local", {
            task_id: wrapped.tag.id().0,
            parent_task_id: TaskLocalsWrapper::get_current(|t| t.id().0).unwrap_or(0),
        });

        let task = wrapped.tag.task().clone();
        let handle = async_global_executor::spawn_local(wrapped);

        Ok(JoinHandle::new(handle, task))
    }

    /// Spawns a task locally with the configured settings.
    #[cfg(all(target_arch = "wasm32", feature = "unstable"))]
    pub fn local<F, T>(self, future: F) -> io::Result<JoinHandle<T>>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        use futures_channel::oneshot::channel;
        let (sender, receiver) = channel();

        let wrapped = self.build(async move {
            let res = future.await;
            let _ = sender.send(res);
        });
        kv_log_macro::trace!("spawn_local", {
            task_id: wrapped.tag.id().0,
            parent_task_id: TaskLocalsWrapper::get_current(|t| t.id().0).unwrap_or(0),
        });

        let task = wrapped.tag.task().clone();
        wasm_bindgen_futures::spawn_local(wrapped);

        Ok(JoinHandle::new(receiver, task))
    }

    /// Spawns a task locally with the configured settings.
    #[cfg(all(target_arch = "wasm32", not(feature = "unstable")))]
    pub(crate) fn local<F, T>(self, future: F) -> io::Result<JoinHandle<T>>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        use futures_channel::oneshot::channel;
        let (sender, receiver) = channel();

        let wrapped = self.build(async move {
            let res = future.await;
            let _ = sender.send(res);
        });

        kv_log_macro::trace!("spawn_local", {
            task_id: wrapped.tag.id().0,
            parent_task_id: TaskLocalsWrapper::get_current(|t| t.id().0).unwrap_or(0),
        });

        let task = wrapped.tag.task().clone();
        wasm_bindgen_futures::spawn_local(wrapped);

        Ok(JoinHandle::new(receiver, task))
    }

    /// Spawns a task with the configured settings, blocking on its execution.
    #[cfg(not(target_os = "unknown"))]
    pub fn blocking<F, T>(self, future: F) -> T
    where
        F: Future<Output = T>,
    {
        use std::cell::Cell;

        let wrapped = self.build(future);

        // Log this `block_on` operation.
        kv_log_macro::trace!("block_on", {
            task_id: wrapped.tag.id().0,
            parent_task_id: TaskLocalsWrapper::get_current(|t| t.id().0).unwrap_or(0),
        });

        thread_local! {
            /// Tracks the number of nested block_on calls.
            static NUM_NESTED_BLOCKING: Cell<usize> = Cell::new(0);
        }

        // Run the future as a task.
        NUM_NESTED_BLOCKING.with(|num_nested_blocking| {
            let count = num_nested_blocking.get();
            let should_run = count == 0;
            // increase the count
            num_nested_blocking.replace(count + 1);

            unsafe {
                TaskLocalsWrapper::set_current(&wrapped.tag, || {
                    let res = if should_run {
                        // The first call should run the executor
                        async_global_executor::block_on(wrapped)
                    } else {
                        futures_lite::future::block_on(wrapped)
                    };
                    num_nested_blocking.replace(num_nested_blocking.get() - 1);
                    res
                })
            }
        })
    }
}

pin_project! {
    /// Wrapper to add support for task locals.
    struct SupportTaskLocals<F> {
        tag: TaskLocalsWrapper,
        #[pin]
        future: F,
    }
}

impl<F: Future> Future for SupportTaskLocals<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            TaskLocalsWrapper::set_current(&self.tag, || {
                let this = self.project();
                this.future.poll(cx)
            })
        }
    }
}
