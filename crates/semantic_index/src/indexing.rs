use collections::HashSet;
use parking_lot::Mutex;
use project::ProjectEntryId;
use smol::channel;
use std::sync::{Arc, Weak};

/// The set of entries that are currently being indexed.
pub struct IndexingEntrySet {
    entry_ids: Mutex<HashSet<ProjectEntryId>>,
    tx: channel::Sender<()>,
}

/// When dropped, removes the entry from the set of entries that are being indexed.
#[derive(Clone)]
pub(crate) struct IndexingEntryHandle {
    entry_id: ProjectEntryId,
    set: Weak<IndexingEntrySet>,
}

impl IndexingEntrySet {
    pub fn new(tx: channel::Sender<()>) -> Self {
        Self {
            entry_ids: Default::default(),
            tx,
        }
    }

    pub fn insert(self: &Arc<Self>, entry_id: ProjectEntryId) -> IndexingEntryHandle {
        self.entry_ids.lock().insert(entry_id);
        self.tx.send_blocking(()).ok();
        IndexingEntryHandle {
            entry_id,
            set: Arc::downgrade(self),
        }
    }

    pub fn len(&self) -> usize {
        self.entry_ids.lock().len()
    }
}

impl Drop for IndexingEntryHandle {
    fn drop(&mut self) {
        if let Some(set) = self.set.upgrade() {
            set.tx.send_blocking(()).ok();
            set.entry_ids.lock().remove(&self.entry_id);
        }
    }
}
