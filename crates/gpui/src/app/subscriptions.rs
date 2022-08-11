use std::marker::PhantomData;
use std::mem;
use std::sync::Arc;
use std::{hash::Hash, sync::Weak};

use parking_lot::Mutex;

use collections::{btree_map, BTreeMap, HashMap};

use crate::MutableAppContext;

//TODO:
//- [ ] for storage managers that make sense for it, add a generic effect construction function.
//- [x] Remove weird functionality duplication between storage types. Subscription manager should be fully generic and not have
//      unreachable!() impls
//  - This can be done by making subscription manager generic over a *composite* key type, that contains the resource ID + sub ID
//  - This also applies to the overly-generic callback types. Only the keyed fallible storage needs the aliveness, why should
//    handle_entity_release_effect need to know to return false?
//- [ ] Move subscription ID handling into the subscription managers somehow.
// ^ All three of these probably involve the same collection of refactorings at once.

///A subscription to some event source in GPUI. By default subscriptions last as long as
//their view or model does, but this trait may allow you change that at some point.
#[must_use]
pub trait Subscription {
    ///Drop this handle to the subscription. GPUI will keep your subscription alive and
    ///continue to call it until it's source is dropped.
    fn detach(&mut self);
}

///This is the most common subscription storage type. Subscriptions are tied to entity (K)
///And can be dropped in place when no longer needed.
pub struct MultiCall<K: Hash + Eq + Copy, F> {
    ///The usize is a subscription id
    internal: Mutex<HashMap<K, BTreeMap<usize, Option<F>>>>,
}

///As of 8/11/22, this is only used for release observations. These callbacks are automatically collected
///After firing and so there's no need for the Option.
pub struct SingleCall<K: Hash + Eq + Copy, F> {
    internal: Mutex<HashMap<K, BTreeMap<usize, F>>>,
}

///As of 8/11/22, this is only used for action callbacks. Actions are fired and received globally, and
///so are not tied to a specific GPUI resource like the others.
pub struct Unkeyed<F> {
    internal: Mutex<BTreeMap<usize, F>>,
}

///This provides common functionality across the different kinds of storage mechanisms available for subcriptions
pub trait SubcriptionManager<K: Hash + Eq + Copy, F> {
    ///Allocates a new storage mechanism. This is a heavy operation, as it usually allocates an Arc, Mutex, and the backing storage.
    fn new() -> Arc<Self>;

    ///Drops a specific subscription callback that's registered on a specific key.
    fn drop(&self, subscription_key_id: K);

    ///Checks if this subscription manager contains any callbacks.
    fn is_empty(self: Arc<Self>) -> bool;
}

impl<K: Hash + Eq + Copy, F> SubcriptionManager<(K, usize), F> for SingleCall<K, F> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            internal: Default::default(),
        })
    }

    fn drop(&self, key_id: (K, usize)) {
        if let Some(subscriptions) = self.internal.lock().get_mut(&key_id.0) {
            subscriptions.remove(&key_id.1);
        }
    }

    fn is_empty(self: Arc<Self>) -> bool {
        self.internal.lock().is_empty()
    }
}

impl<K: Hash + Eq + Copy, F> SingleCall<K, F> {
    pub fn emit<C: FnMut(&mut F, &mut MutableAppContext)>(
        &mut self,
        key_id: K,
        cx: &mut MutableAppContext,
        call_callback: C,
    ) {
        let callbacks = self.internal.lock().remove(&key_id);

        if let Some(callbacks) = callbacks {
            for callback in callbacks.values_mut() {
                call_callback(callback, cx);
            }
        }
    }

    pub fn add_callback(&self, key_id: K, subscription_id: usize, callback: F) {
        self.internal
            .lock()
            .entry(key_id)
            .or_default()
            .insert(subscription_id, callback);
    }
}

impl<K: Hash + Eq + Copy, F> SubcriptionManager<(K, usize), F> for MultiCall<K, F> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            internal: Default::default(),
        })
    }

    fn drop(&self, key: (K, usize)) {
        match self.internal.lock().entry(key.0).or_default().entry(key.1) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(None);
            }
            btree_map::Entry::Occupied(entry) => {
                entry.remove();
            }
        }
    }

    fn is_empty(self: Arc<Self>) -> bool {
        self.internal.lock().is_empty()
    }
}

impl<K: Hash + Eq + Copy, F> MultiCall<K, F> {
    pub fn emit<C>(&mut self, key_id: K, cx: &mut MutableAppContext, call_callback: C)
    where
        C: FnMut(&mut F, &mut MutableAppContext) -> bool,
    {
        let callbacks = self.internal.lock().remove(&key_id);
        if let Some(callbacks) = callbacks {
            for (subscription_id, callback) in callbacks {
                if let Some(mut callback) = callback {
                    let alive = call_callback(&mut callback, cx);
                    if alive {
                        match self
                            .internal
                            .lock()
                            .entry(key_id)
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

    pub fn add_callback(&self, key_id: K, subscription_id: usize, callback: F) {
        self.internal
            .lock()
            .entry(key_id)
            .or_default()
            .insert(subscription_id, Some(callback));
    }

    pub fn remove_key(&mut self, key_id: K) {
        self.internal.lock().remove(&key_id);
    }

    pub fn toggle_callback(&mut self, key_id: K, subscription_id: usize, callback: F) {
        match self
            .internal
            .lock()
            .entry(key_id)
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
}

impl<F> SubcriptionManager<usize, F> for Unkeyed<F> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            internal: Default::default(),
        })
    }

    fn drop(&self, subscription_id: usize) {
        self.internal.lock().remove(&subscription_id);
    }

    fn is_empty(self: Arc<Self>) -> bool {
        self.internal.lock().is_empty()
    }
}

impl<F> Unkeyed<F> {
    pub fn add_callback(&self, subscription_id: usize, callback: F) {
        self.internal.lock().insert(subscription_id, callback);
    }

    pub fn emit<C: FnMut(&mut F, &mut MutableAppContext)>(
        &mut self,
        cx: &mut MutableAppContext,
        call_callback: C,
    ) {
        let mut callbacks = mem::take(&mut *self.internal.lock());
        for callback in callbacks.values_mut() {
            call_callback(callback, cx);
        }
        self.internal.lock().extend(callbacks);
    }
}

///The implementation of the trait that the user will receive.
pub struct InternalSubscription<K: Hash + Eq + Copy, F, S: SubcriptionManager<K, F>> {
    ///The path to the key for this subscription
    key: K,
    ///A weak reference to the backing storage, for removal.
    subscriptions: Option<Weak<S>>,
    ///PhantomData so Rust is happy.
    _callback: PhantomData<F>,
}

impl<K: Hash + Eq + Copy, F, S: SubcriptionManager<K, F>> InternalSubscription<K, F, S> {
    pub fn new(key: K, subscriptions: &Arc<S>) -> Self {
        Self {
            key,
            subscriptions: Some(Arc::downgrade(subscriptions)),
            _callback: PhantomData,
        }
    }
}

impl<K: Hash + Eq + Copy, F, S: SubcriptionManager<K, F>> Subscription
    for InternalSubscription<K, F, S>
{
    fn detach(&mut self) {
        self.subscriptions.take();
    }
}

impl<K: Hash + Eq + Copy, F, S: SubcriptionManager<K, F>> Drop for InternalSubscription<K, F, S> {
    fn drop(&mut self) {
        if let Some(subscriptions) = self.subscriptions.as_ref().and_then(Weak::upgrade) {
            (*subscriptions).drop(self.key);
        }
    }
}
