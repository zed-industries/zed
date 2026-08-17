use crate::loom::sync::atomic::AtomicPtr;
use crate::runtime::task::{Header, Notified, RawTask};

use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::SeqCst;

/// An atomic cell which can contain a pointer to a [`Notified`] task.
///
/// This is similar to the `crate::util::AtomicCell` type, but specialized to
/// hold a task pointer --- this type "remembers" the task's scheduler generic
/// when a task is stored in the cell, so that the pointer can be turned back
/// into a [`Notified`] task with the correct generic type when it is retrieved.
pub(crate) struct AtomicNotified<S: 'static> {
    task: AtomicPtr<Header>,
    _scheduler: PhantomData<S>,
}

impl<S: 'static> AtomicNotified<S> {
    pub(crate) fn empty() -> Self {
        Self {
            task: AtomicPtr::new(ptr::null_mut()),
            _scheduler: PhantomData,
        }
    }

    pub(crate) fn swap(&self, task: Option<Notified<S>>) -> Option<Notified<S>> {
        let new = task
            .map(|t| t.into_raw().header_ptr().as_ptr())
            .unwrap_or_else(ptr::null_mut);
        let old = self.task.swap(new, SeqCst);
        NonNull::new(old).map(|ptr| unsafe {
            // Safety: since we only allow tasks with the same scheduler type to
            // be placed in this cell, we know that the pointed task's scheduler
            // type matches the type parameter S.
            Notified::from_raw(RawTask::from_raw(ptr))
        })
    }

    pub(crate) fn take(&self) -> Option<Notified<S>> {
        self.swap(None)
    }

    pub(crate) fn is_some(&self) -> bool {
        !self.task.load(SeqCst).is_null()
    }
}

unsafe impl<S: Send> Send for AtomicNotified<S> {}
unsafe impl<S: Send> Sync for AtomicNotified<S> {}

impl<S> Drop for AtomicNotified<S> {
    fn drop(&mut self) {
        // Ensure the task reference is dropped if this cell is dropped.
        let _ = self.take();
    }
}
