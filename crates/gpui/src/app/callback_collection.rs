use std::sync::Arc;
use std::{hash::Hash, sync::Weak};

use parking_lot::Mutex;

use collections::{BTreeMap, HashMap, HashSet};

use crate::MutableAppContext;

// Problem 5: Current bug callbacks currently called many times after being dropped
// update
//   notify
//   observe (push effect)
//     subscription {id : 5}
//     pending: [Effect::Notify, Effect::observe { id: 5 }]
//   drop observation subscription (write None into subscriptions)
// flush effects
//   notify
//   observe
// Problem 6: Key-value pair is leaked if you drop a callback while calling it, and then never call that set of callbacks again
// -----------------
// Problem 1: Many very similar subscription enum variants
// Problem 2: Subscriptions and CallbackCollections use a shared mutex to update the callback status
// Problem 3: Current implementation is error prone with regard to uninitialized callbacks or dropping during callback
// Problem 4: Calling callbacks requires removing all of them from the list and adding them back

// Solution 1 CallbackState:
//      Add more state to the CallbackCollection map to communicate dropped and initialized status
//      Solves: P5
// Solution 2 DroppedSubscriptionList:
//      Store a parallel set of dropped subscriptions in the Mapping which stores the key and subscription id for all dropped subscriptions
//      which can be
// Solution 3 GarbageCollection:
//      Use some type of traditional garbage collection to handle dropping of callbacks
//          atomic flag per callback which is looped over in remove dropped entities

// TODO:
//   - Move subscription id counter to CallbackCollection
//   - Consider adding a reverse map in Mapping from subscription id to key so that the dropped subscriptions
//     can be a hashset of usize and the Subscription doesn't need the key
//   - Investigate why the remaining two types of callback lists can't use the same callback collection and subscriptions
pub struct Subscription<K: Clone + Hash + Eq, F> {
    key: K,
    id: usize,
    mapping: Option<Weak<Mutex<Mapping<K, F>>>>,
}

struct Mapping<K, F> {
    callbacks: HashMap<K, BTreeMap<usize, F>>,
    dropped_subscriptions: HashSet<(K, usize)>,
}

impl<K, F> Default for Mapping<K, F> {
    fn default() -> Self {
        Self {
            callbacks: Default::default(),
            dropped_subscriptions: Default::default(),
        }
    }
}

pub(crate) struct CallbackCollection<K: Clone + Hash + Eq, F> {
    internal: Arc<Mutex<Mapping<K, F>>>,
}

impl<K: Clone + Hash + Eq, F> Clone for CallbackCollection<K, F> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<K: Clone + Hash + Eq + Copy, F> Default for CallbackCollection<K, F> {
    fn default() -> Self {
        CallbackCollection {
            internal: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl<K: Clone + Hash + Eq + Copy, F> CallbackCollection<K, F> {
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.internal.lock().callbacks.is_empty()
    }

    pub fn subscribe(&mut self, key: K, subscription_id: usize) -> Subscription<K, F> {
        Subscription {
            key,
            id: subscription_id,
            mapping: Some(Arc::downgrade(&self.internal)),
        }
    }

    pub fn count(&mut self, key: K) -> usize {
        self.internal
            .lock()
            .callbacks
            .get(&key)
            .map_or(0, |callbacks| callbacks.len())
    }

    pub fn add_callback(&mut self, key: K, subscription_id: usize, callback: F) {
        let mut this = self.internal.lock();
        if !this.dropped_subscriptions.contains(&(key, subscription_id)) {
            this.callbacks
                .entry(key)
                .or_default()
                .insert(subscription_id, callback);
        }
    }

    pub fn remove(&mut self, key: K) {
        self.internal.lock().callbacks.remove(&key);
    }

    pub fn emit<C: FnMut(&mut F, &mut MutableAppContext) -> bool>(
        &mut self,
        key: K,
        cx: &mut MutableAppContext,
        mut call_callback: C,
    ) {
        let callbacks = self.internal.lock().callbacks.remove(&key);
        if let Some(callbacks) = callbacks {
            for (subscription_id, mut callback) in callbacks {
                if !self
                    .internal
                    .lock()
                    .dropped_subscriptions
                    .contains(&(key, subscription_id))
                {
                    if call_callback(&mut callback, cx) {
                        self.add_callback(key, subscription_id, callback);
                    }
                }
            }
        }
    }

    pub fn gc(&mut self) {
        let mut this = self.internal.lock();

        for (key, id) in std::mem::take(&mut this.dropped_subscriptions) {
            if let Some(callbacks) = this.callbacks.get_mut(&key) {
                callbacks.remove(&id);
            }
        }
    }
}

impl<K: Clone + Hash + Eq, F> Subscription<K, F> {
    pub fn detach(&mut self) {
        self.mapping.take();
    }
}

impl<K: Clone + Hash + Eq, F> Drop for Subscription<K, F> {
    // If the callback has been initialized (no callback in the list for the key and id),
    // add this subscription id and key to the dropped subscriptions list
    // Otherwise, just remove the associated callback from the callback collection
    fn drop(&mut self) {
        if let Some(mapping) = self.mapping.as_ref().and_then(|mapping| mapping.upgrade()) {
            let mut mapping = mapping.lock();
            if let Some(callbacks) = mapping.callbacks.get_mut(&self.key) {
                if callbacks.remove(&self.id).is_some() {
                    return;
                }
            }
            mapping
                .dropped_subscriptions
                .insert((self.key.clone(), self.id));
        }
    }
}
