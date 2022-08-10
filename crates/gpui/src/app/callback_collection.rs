use std::sync::Arc;
use std::{hash::Hash, sync::Weak};

use parking_lot::Mutex;

use collections::{btree_map, BTreeMap, HashMap};

use crate::MutableAppContext;

pub type Mapping<K, F> = Mutex<HashMap<K, BTreeMap<usize, Option<F>>>>;

pub struct CallbackCollection<K: Hash + Eq, F> {
    internal: Arc<Mapping<K, F>>,
}

impl<K: Hash + Eq, F> Clone for CallbackCollection<K, F> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<K: Hash + Eq + Copy, F> Default for CallbackCollection<K, F> {
    fn default() -> Self {
        CallbackCollection {
            internal: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl<K: Hash + Eq + Copy, F> CallbackCollection<K, F> {
    pub fn downgrade(&self) -> Weak<Mapping<K, F>> {
        Arc::downgrade(&self.internal)
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.internal.lock().is_empty()
    }

    pub fn add_callback(&mut self, id: K, subscription_id: usize, callback: F) {
        self.internal
            .lock()
            .entry(id)
            .or_default()
            .insert(subscription_id, Some(callback));
    }

    pub fn remove(&mut self, id: K) {
        self.internal.lock().remove(&id);
    }

    pub fn add_or_remove_callback(&mut self, id: K, subscription_id: usize, callback: F) {
        match self
            .internal
            .lock()
            .entry(id)
            .or_default()
            .entry(subscription_id)
        {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(Some(callback));
            }

            btree_map::Entry::Occupied(entry) => {
                // TODO: This seems like it should never be called because no code
                // should ever attempt to remove an existing callback
                debug_assert!(entry.get().is_none());
                entry.remove();
            }
        }
    }

    pub fn emit_and_cleanup<C: FnMut(&mut F, &mut MutableAppContext) -> bool>(
        &mut self,
        id: K,
        cx: &mut MutableAppContext,
        mut call_callback: C,
    ) {
        let callbacks = self.internal.lock().remove(&id);
        if let Some(callbacks) = callbacks {
            for (subscription_id, callback) in callbacks {
                if let Some(mut callback) = callback {
                    let alive = call_callback(&mut callback, cx);
                    if alive {
                        match self
                            .internal
                            .lock()
                            .entry(id)
                            .or_default()
                            .entry(subscription_id)
                        {
                            btree_map::Entry::Vacant(entry) => {
                                entry.insert(Some(callback));
                            }
                            btree_map::Entry::Occupied(entry) => {
                                entry.remove();
                            }
                        }
                    }
                }
            }
        }
    }
}
