use crate::util::post_inc;
use crate::MutableAppContext;
use collections::{btree_map, BTreeMap, HashMap};
use parking_lot::Mutex;
use std::marker::PhantomData;
use std::mem;
use std::sync::Arc;
use std::{hash::Hash, sync::Weak};

//Goals:
// - Generic implementation, for code reuse and simplicity
//  - Original implementation ignored this
// - Hide implementation details, like storage mechanism and generics
//  - I used a trait for this, which makes ownership hard because the size is always changing. I am *required* to put it in a box if so.
// - Use an owned type whose dropping deallocates a subscription (unless detached)
//  - GPUI used static dispatch with an enum + match + code duplication

//TODO:
//- [ ] Decide what to do about: test_dropping_subscriptions_during_callback
//- [ ] for storage managers that make sense for it, add a generic effect construction function.
//- [x] Remove weird functionality duplication between storage types. Subscription manager should be fully generic and not have
//      unreachable!() impls
//  - This can be done by making subscription manager generic over a *composite* key type, that contains the resource ID + sub ID
//  - This also applies to the overly-generic callback types. Only the keyed fallible storage needs the aliveness, why should
//    handle_entity_release_effect need to know to return false?
//- [x] Move subscription ID handling into the subscription managers somehow.
// ^ All three of these probably involve the same collection of refactorings at once.

///A subscription to some event source in GPUI. By default subscriptions last as long as
//their view or model does, but this trait may allow you change that at some point.
pub trait Subscription {
    ///Drop this handle to the subscription. GPUI will keep your subscription alive and
    ///continue to call it until it's source is dropped.
    fn detach(&mut self);
}

///This is the most common subscription storage type. Subscriptions are tied to entity (K)
///And can be dropped in place when no longer needed.
pub struct MultiCall<K: Hash + Eq + Copy, F> {
    ///The usize is a subscription id
    internal: Mutex<InternalStorage<HashMap<K, BTreeMap<usize, Option<F>>>>>,
}

///As of 8/11/22, this is only used for release observations. These callbacks are automatically collected
///After firing and so there's no need for the Option.
pub struct SingleCall<K: Hash + Eq + Copy, F> {
    internal: Mutex<InternalStorage<HashMap<K, BTreeMap<usize, F>>>>,
}

///As of 8/11/22, this is only used for action callbacks. Actions are fired and received globally, and
///so are not tied to a specific GPUI resource like the others.
pub struct Unkeyed<F> {
    internal: Mutex<InternalStorage<BTreeMap<usize, F>>>,
}

///This wraps up the subscription ID handling code into a single type
#[derive(Default)]
struct InternalStorage<T: Default> {
    next_id: usize,
    storage: T,
}

impl<T: Default> InternalStorage<T> {
    ///Get the next subscription id for this storage
    fn next_id(&mut self) -> usize {
        post_inc(&mut self.next_id)
    }
}

///This provides common functionality across the different kinds of storage mechanisms available for subcriptions
pub trait SubcriptionManager<K: Hash + Eq + Copy, F> {
    ///Allocates a new storage mechanism. This is a heavy operation, as it usually allocates an Arc, Mutex, and the backing storage.
    fn new() -> Arc<Self>;

    ///Drops a specific subscription callback that's registered on a specific key.
    fn drop_subscription(&self, subscription_key_id: K);

    ///Checks if this subscription manager contains any callbacks.
    fn is_empty(self: &Arc<Self>) -> bool;
}

impl<K: Hash + Eq + Copy, F> SubcriptionManager<(K, usize), F> for SingleCall<K, F> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            internal: Default::default(),
        })
    }

    fn drop_subscription(&self, key_id: (K, usize)) {
        if let Some(subscriptions) = self.internal.lock().storage.get_mut(&key_id.0) {
            subscriptions.remove(&key_id.1);
        }
    }

    fn is_empty(self: &Arc<Self>) -> bool {
        self.internal.lock().storage.is_empty()
    }
}

impl<K: Hash + Eq + Copy, F> SingleCall<K, F> {
    pub fn emit<C: FnMut(F, &mut MutableAppContext)>(
        &self,
        key: K,
        cx: &mut MutableAppContext,
        mut call_callback: C,
    ) {
        let callbacks = self.internal.lock().storage.remove(&key);

        if let Some(callbacks) = callbacks {
            for (_, callback) in callbacks.into_iter() {
                call_callback(callback, cx);
            }
        }
    }

    pub fn add_subscription(self: &Arc<Self>, key: K, f: F) -> impl Subscription {
        let mut internal = self.internal.lock();
        let id = internal.next_id();
        internal.storage.entry(key).or_default().insert(id, f);
        drop(internal);

        InternalSubscription::new((key, id), self)
    }
}

impl<K: Hash + Eq + Copy, F> SubcriptionManager<(K, usize), F> for MultiCall<K, F> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            internal: Default::default(),
        })
    }

    fn drop_subscription(&self, key: (K, usize)) {
        match self
            .internal
            .lock()
            .storage
            .entry(key.0)
            .or_default()
            .entry(key.1)
        {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(None);
            }
            btree_map::Entry::Occupied(entry) => {
                entry.remove();
            }
        }
    }

    fn is_empty(self: &Arc<Self>) -> bool {
        self.internal.lock().storage.is_empty()
    }
}

impl<K: Hash + Eq + Copy, F> MultiCall<K, F> {
    pub fn emit<C>(&self, key_id: K, cx: &mut MutableAppContext, mut call_callback: C)
    where
        C: FnMut(&mut F, &mut MutableAppContext) -> bool,
    {
        let callbacks = self.internal.lock().storage.remove(&key_id);
        if let Some(callbacks) = callbacks {
            for (subscription_id, callback) in callbacks {
                if let Some(mut callback) = callback {
                    let alive = call_callback(&mut callback, cx);
                    if alive {
                        match self
                            .internal
                            .lock()
                            .storage
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

    pub fn add_subscription(self: &Arc<Self>, key: K, f: F) -> impl Subscription {
        let mut internal = self.internal.lock();
        let id = internal.next_id();
        internal.storage.entry(key).or_default().insert(id, Some(f));
        drop(internal);

        InternalSubscription::new((key, id), self)
    }

    pub fn remove_key(&self, key_id: K) {
        self.internal.lock().storage.remove(&key_id);
    }

    //TODO: Figure out how to remove
    pub fn toggle_callback(&self, key_id: K, subscription_id: usize, callback: F) {
        match self
            .internal
            .lock()
            .storage
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

    fn drop_subscription(&self, subscription_id: usize) {
        self.internal.lock().storage.remove(&subscription_id);
    }

    fn is_empty(self: &Arc<Self>) -> bool {
        self.internal.lock().storage.is_empty()
    }
}

impl<F> Unkeyed<F> {
    pub fn emit<C: FnMut(&mut F, &mut MutableAppContext)>(
        &self,
        cx: &mut MutableAppContext,
        mut call_callback: C,
    ) {
        let mut callbacks = mem::take(&mut self.internal.lock().storage);
        for callback in callbacks.values_mut() {
            call_callback(callback, cx);
        }
        self.internal.lock().storage.extend(callbacks);
    }

    pub fn add_subscription(self: &Arc<Self>, f: F) -> impl Subscription {
        let mut internal = self.internal.lock();
        let id = internal.next_id();
        internal.storage.insert(id, f);
        drop(internal);

        InternalSubscription::new(id, self)
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
        //This drops the weak arc, meaning that dropping an InternalSubscription won't drop the underlying subscription
        self.subscriptions.take();
    }
}

impl<K: Hash + Eq + Copy, F, S: SubcriptionManager<K, F>> Drop for InternalSubscription<K, F, S> {
    fn drop(&mut self) {
        if let Some(subscriptions) = self.subscriptions.as_ref().and_then(Weak::upgrade) {
            (*subscriptions).drop_subscription(self.key);
        }
    }
}

// //*****************************************************************************
// //The following code de-generics everything above so we don't have to box and
// //dynamic dispatch on every single subscription handler. This is very annoying.
// //This comes at the cost of every kind of subscription being as heavy as the
// //largest version of IntetrnalSubscription.
// //
// //But alas, memory is cheap and time (latency) is not.
// //*****************************************************************************

// pub type ActionCallback =
//     dyn FnMut(&mut dyn AnyView, &dyn Action, &mut MutableAppContext, usize, usize);
// pub type GlobalActionCallback = dyn FnMut(&dyn Action, &mut MutableAppContext);
// pub type SubscriptionCallback = Box<dyn FnMut(&dyn Any, &mut MutableAppContext) -> bool>;
// pub type GlobalSubscriptionCallback = Box<dyn FnMut(&dyn Any, &mut MutableAppContext)>;
// pub type ObservationCallback = Box<dyn FnMut(&mut MutableAppContext) -> bool>;
// pub type FocusObservationCallback = Box<dyn FnMut(bool, &mut MutableAppContext) -> bool>;
// pub type GlobalObservationCallback = Box<dyn FnMut(&mut MutableAppContext)>;
// pub type ReleaseObservationCallback = Box<dyn FnOnce(&dyn Any, &mut MutableAppContext)>;
// pub type ActionObservationCallback = Box<dyn FnMut(TypeId, &mut MutableAppContext)>;
// pub type WindowActivationCallback = Box<dyn FnMut(bool, &mut MutableAppContext) -> bool>;
// pub type WindowFullscreenCallback = Box<dyn FnMut(bool, &mut MutableAppContext) -> bool>;
// pub type DeserializeActionCallback = fn(json: &str) -> anyhow::Result<Box<dyn Action>>;
// pub type WindowShouldCloseSubscriptionCallback = Box<dyn FnMut(&mut MutableAppContext) -> bool>;

// #[must_use]
// pub enum Subscription {
//     FocusObservations(
//         InternalSubscription<
//             (usize, usize),
//             FocusObservationCallback,
//             MultiCall<usize, FocusObservationCallback>,
//         >,
//     ),
//     GlobalSubscriptions(
//         InternalSubscription<
//             (TypeId, usize),
//             GlobalSubscriptionCallback,
//             MultiCall<TypeId, GlobalSubscriptionCallback>,
//         >,
//     ),
//     GlobalObservations(
//         InternalSubscription<
//             (TypeId, usize),
//             GlobalObservationCallback,
//             MultiCall<TypeId, GlobalObservationCallback>,
//         >,
//     ),
//     Subscriptions(
//         InternalSubscription<
//             (usize, usize),
//             SubscriptionCallback,
//             MultiCall<usize, SubscriptionCallback>,
//         >,
//     ),
//     Observations(
//         InternalSubscription<
//             (usize, usize),
//             ObservationCallback,
//             MultiCall<usize, ObservationCallback>,
//         >,
//     ),
//     WindowActivationObservations(
//         InternalSubscription<
//             (usize, usize),
//             WindowActivationCallback,
//             MultiCall<usize, WindowActivationCallback>,
//         >,
//     ),
//     WindowFullscreenObservations(
//         InternalSubscription<
//             (usize, usize),
//             WindowFullscreenCallback,
//             MultiCall<usize, WindowFullscreenCallback>,
//         >,
//     ),
//     ReleaseObservations(
//         InternalSubscription<
//             (usize, usize),
//             ReleaseObservationCallback,
//             SingleCall<usize, ReleaseObservationCallback>,
//         >,
//     ),
//     ActionDispatchObservations(
//         InternalSubscription<usize, ActionObservationCallback, Unkeyed<ActionObservationCallback>>,
//     ),
// }

// impl Detachable for Subscription {
//     fn detach(&mut self) {
//         match self {
//             Subscription::FocusObservations(internal) => internal.detach(),
//             Subscription::GlobalSubscriptions(internal) => internal.detach(),
//             Subscription::GlobalObservations(internal) => internal.detach(),
//             Subscription::Subscriptions(internal) => internal.detach(),
//             Subscription::Observations(internal) => internal.detach(),
//             Subscription::WindowActivationObservations(internal) => internal.detach(),
//             Subscription::WindowFullscreenObservations(internal) => internal.detach(),
//             Subscription::ReleaseObservations(internal) => internal.detach(),
//             Subscription::ActionDispatchObservations(internal) => internal.detach(),
//         }
//     }
// }
