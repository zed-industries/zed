use std::cell::Cell;
use std::ptr;

use crate::task::{LocalsMap, Task, TaskId};
use crate::utils::abort_on_panic;

thread_local! {
    /// A pointer to the currently running task.
    static CURRENT: Cell<*const TaskLocalsWrapper> = Cell::new(ptr::null_mut());
}

/// A wrapper to store task local data.
pub(crate) struct TaskLocalsWrapper {
    /// The actual task details.
    task: Task,

    /// The map holding task-local values.
    locals: LocalsMap,
}

impl TaskLocalsWrapper {
    /// Creates a new task handle.
    ///
    /// If the task is unnamed, the inner representation of the task will be lazily allocated on
    /// demand.
    #[inline]
    pub(crate) fn new(task: Task) -> Self {
        Self {
            task,
            locals: LocalsMap::new(),
        }
    }

    /// Gets the task's unique identifier.
    #[inline]
    pub fn id(&self) -> TaskId {
        self.task.id()
    }

    /// Returns a reference to the inner `Task`.
    pub(crate) fn task(&self) -> &Task {
        &self.task
    }

    /// Returns the map holding task-local values.
    pub(crate) fn locals(&self) -> &LocalsMap {
        &self.locals
    }

    /// Set a reference to the current task.
    pub(crate) unsafe fn set_current<F, R>(task: *const TaskLocalsWrapper, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        CURRENT.with(|current| {
            let old_task = current.replace(task);
            defer! {
                current.set(old_task);
            }
            f()
        })
    }

    /// Gets a reference to the current task.
    pub(crate) fn get_current<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&TaskLocalsWrapper) -> R,
    {
        let res = CURRENT.try_with(|current| unsafe { current.get().as_ref().map(f) });
        match res {
            Ok(Some(val)) => Some(val),
            Ok(None) | Err(_) => None,
        }
    }
}

impl Drop for TaskLocalsWrapper {
    fn drop(&mut self) {
        // Abort the process if dropping task-locals panics.
        abort_on_panic(|| {
            unsafe { self.locals.clear() };
        });
    }
}
