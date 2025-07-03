use collections::{BTreeMap, BTreeSet};
use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    mem,
    rc::Rc,
};
use util::post_inc;

pub(crate) struct SubscriberSet<EmitterKey, Callback>(
    Rc<RefCell<SubscriberSetState<EmitterKey, Callback>>>,
);

impl<EmitterKey, Callback> Clone for SubscriberSet<EmitterKey, Callback> {
    fn clone(&self) -> Self {
        SubscriberSet(self.0.clone())
    }
}

struct SubscriberSetState<EmitterKey, Callback> {
    subscribers: BTreeMap<EmitterKey, Option<BTreeMap<usize, Subscriber<Callback>>>>,
    dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
    next_subscriber_id: usize,
}

struct Subscriber<Callback> {
    active: Rc<Cell<bool>>,
    callback: Callback,
}

impl<EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
where
    EmitterKey: 'static + Ord + Clone + Debug,
    Callback: 'static,
{
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SubscriberSetState {
            subscribers: Default::default(),
            dropped_subscribers: Default::default(),
            next_subscriber_id: 0,
        })))
    }

    /// Inserts a new [`Subscription`] for the given `emitter_key`. By default, subscriptions
    /// are inert, meaning that they won't be listed when calling `[SubscriberSet::remove]` or `[SubscriberSet::retain]`.
    /// This method returns a tuple of a [`Subscription`] and an `impl FnOnce`, and you can use the latter
    /// to activate the [`Subscription`].
    pub fn insert(
        &self,
        emitter_key: EmitterKey,
        callback: Callback,
    ) -> (Subscription, impl FnOnce() + use<EmitterKey, Callback>) {
        let active = Rc::new(Cell::new(false));
        let mut lock = self.0.borrow_mut();
        let subscriber_id = post_inc(&mut lock.next_subscriber_id);
        lock.subscribers
            .entry(emitter_key.clone())
            .or_default()
            .get_or_insert_with(Default::default)
            .insert(
                subscriber_id,
                Subscriber {
                    active: active.clone(),
                    callback,
                },
            );
        let this = self.0.clone();

        let subscription = Subscription {
            unsubscribe: Some(Box::new(move || {
                let mut lock = this.borrow_mut();
                let Some(subscribers) = lock.subscribers.get_mut(&emitter_key) else {
                    // remove was called with this emitter_key
                    return;
                };

                if let Some(subscribers) = subscribers {
                    subscribers.remove(&subscriber_id);
                    if subscribers.is_empty() {
                        lock.subscribers.remove(&emitter_key);
                    }
                    return;
                }

                // We didn't manage to remove the subscription, which means it was dropped
                // while invoking the callback. Mark it as dropped so that we can remove it
                // later.
                lock.dropped_subscribers
                    .insert((emitter_key, subscriber_id));
            })),
        };
        (subscription, move || active.set(true))
    }

    pub fn remove(
        &self,
        emitter: &EmitterKey,
    ) -> impl IntoIterator<Item = Callback> + use<EmitterKey, Callback> {
        let subscribers = self.0.borrow_mut().subscribers.remove(emitter);
        subscribers
            .unwrap_or_default()
            .map(|s| s.into_values())
            .into_iter()
            .flatten()
            .filter_map(|subscriber| {
                if subscriber.active.get() {
                    Some(subscriber.callback)
                } else {
                    None
                }
            })
    }

    /// Call the given callback for each subscriber to the given emitter.
    /// If the callback returns false, the subscriber is removed.
    pub fn retain<F>(&self, emitter: &EmitterKey, mut f: F)
    where
        F: FnMut(&mut Callback) -> bool,
    {
        let Some(mut subscribers) = self
            .0
            .borrow_mut()
            .subscribers
            .get_mut(emitter)
            .and_then(|s| s.take())
        else {
            return;
        };

        subscribers.retain(|_, subscriber| {
            if subscriber.active.get() {
                f(&mut subscriber.callback)
            } else {
                true
            }
        });
        let mut lock = self.0.borrow_mut();

        // Add any new subscribers that were added while invoking the callback.
        if let Some(Some(new_subscribers)) = lock.subscribers.remove(emitter) {
            subscribers.extend(new_subscribers);
        }

        // Remove any dropped subscriptions that were dropped while invoking the callback.
        for (dropped_emitter, dropped_subscription_id) in mem::take(&mut lock.dropped_subscribers) {
            debug_assert_eq!(*emitter, dropped_emitter);
            subscribers.remove(&dropped_subscription_id);
        }

        if !subscribers.is_empty() {
            lock.subscribers.insert(emitter.clone(), Some(subscribers));
        }
    }
}

/// A handle to a subscription created by GPUI. When dropped, the subscription
/// is cancelled and the callback will no longer be invoked.
#[must_use]
pub struct Subscription {
    unsubscribe: Option<Box<dyn FnOnce() + 'static>>,
}

impl Subscription {
    /// Creates a new subscription with a callback that gets invoked when
    /// this subscription is dropped.
    pub fn new(unsubscribe: impl 'static + FnOnce()) -> Self {
        Self {
            unsubscribe: Some(Box::new(unsubscribe)),
        }
    }

    /// Detaches the subscription from this handle. The callback will
    /// continue to be invoked until the entities it has been
    /// subscribed to are dropped
    pub fn detach(mut self) {
        self.unsubscribe.take();
    }

    /// Joins two subscriptions into a single subscription. Detach will
    /// detach both interior subscriptions.
    pub fn join(mut subscription_a: Self, mut subscription_b: Self) -> Self {
        let a_unsubscribe = subscription_a.unsubscribe.take();
        let b_unsubscribe = subscription_b.unsubscribe.take();
        Self {
            unsubscribe: Some(Box::new(move || {
                if let Some(self_unsubscribe) = a_unsubscribe {
                    self_unsubscribe();
                }
                if let Some(other_unsubscribe) = b_unsubscribe {
                    other_unsubscribe();
                }
            })),
        }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(unsubscribe) = self.unsubscribe.take() {
            unsubscribe();
        }
    }
}
