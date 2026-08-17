use super::{Entry, EntryHandle, RegistrationQueueEntry};
use crate::util::linked_list;

type EntryList = linked_list::LinkedList<RegistrationQueueEntry, Entry>;

/// A queue of entries that need to be registered in the timer wheel.
#[derive(Debug)]
pub(crate) struct RegistrationQueue {
    list: EntryList,
}

impl Drop for RegistrationQueue {
    fn drop(&mut self) {
        // drain all entries without waking them up
        while let Some(hdl) = self.list.pop_front() {
            drop(hdl);
        }
    }
}

impl RegistrationQueue {
    pub(crate) fn new() -> Self {
        Self {
            list: EntryList::new(),
        }
    }

    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - [`Entry::extra_pointers`] of `hdl` must not being used.
    pub(crate) unsafe fn push_front(&mut self, hdl: EntryHandle) {
        self.list.push_front(hdl);
    }

    pub(crate) fn pop_front(&mut self) -> Option<EntryHandle> {
        self.list.pop_front()
    }
}

#[cfg(test)]
mod tests;
