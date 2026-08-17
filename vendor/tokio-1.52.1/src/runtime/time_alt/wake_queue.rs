use super::{Entry, EntryHandle, WakeQueueEntry};
use crate::util::linked_list;

type EntryList = linked_list::LinkedList<WakeQueueEntry, Entry>;

/// A queue of entries that need to be woken up.
#[derive(Debug)]
pub(crate) struct WakeQueue {
    list: EntryList,
}

impl Drop for WakeQueue {
    fn drop(&mut self) {
        // drain all entries without waking them up
        while let Some(hdl) = self.list.pop_front() {
            drop(hdl);
        }
    }
}

impl WakeQueue {
    pub(crate) fn new() -> Self {
        Self {
            list: EntryList::new(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - [`Entry::extra_pointers`] of `hdl` must not being used.
    pub(crate) unsafe fn push_front(&mut self, hdl: EntryHandle) {
        self.list.push_front(hdl);
    }

    /// Wakes all entries in the wake queue.
    pub(crate) fn wake_all(mut self) {
        while let Some(hdl) = self.list.pop_front() {
            hdl.wake();
        }
    }
}

#[cfg(test)]
mod tests;
