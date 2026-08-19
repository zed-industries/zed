use collections::BTreeMap;
use gpui_util::post_inc;
use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::Rc,
};

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
    next_subscriber_id: usize,
}

struct Subscriber<Callback> {
    active: Rc<Cell<bool>>,
    dropped: Rc<Cell<bool>>,
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
        let dropped = Rc::new(Cell::new(false));
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
                    dropped: dropped.clone(),
                    callback,
                },
            );
        let this = self.0.clone();

        let subscription = Subscription {
            unsubscribe: Some(Box::new(move || {
                dropped.set(true);

                let mut lock = this.borrow_mut();
                let Some(subscribers) = lock.subscribers.get_mut(&emitter_key) else {
                    return;
                };

                if let Some(subscribers) = subscribers {
                    subscribers.remove(&subscriber_id);
                    if subscribers.is_empty() {
                        lock.subscribers.remove(&emitter_key);
                    }
                }
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
            if !subscriber.active.get() {
                return true;
            }
            if subscriber.dropped.get() {
                return false;
            }
            let keep = f(&mut subscriber.callback);
            keep && !subscriber.dropped.get()
        });
        let mut lock = self.0.borrow_mut();

        // Add any new subscribers that were added while invoking the callback.
        if let Some(Some(new_subscribers)) = lock.subscribers.remove(emitter) {
            subscribers.extend(new_subscribers);
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

impl std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Global, TestApp};

    #[test]
    fn test_unsubscribe_during_callback_with_insert() {
        struct TestGlobal;
        impl Global for TestGlobal {}

        let mut app = TestApp::new();
        app.set_global(TestGlobal);

        let observer_a_count = Rc::new(Cell::new(0usize));
        let observer_b_count = Rc::new(Cell::new(0usize));

        let sub_a: Rc<RefCell<Option<Subscription>>> = Default::default();
        let sub_b: Rc<RefCell<Option<Subscription>>> = Default::default();

        // Observer A fires first (lower subscriber_id). It drops itself and
        // inserts a new observer for the same global.
        *sub_a.borrow_mut() = Some(app.update({
            let count = observer_a_count.clone();
            let sub_a = sub_a.clone();
            move |cx| {
                cx.observe_global::<TestGlobal>(move |cx| {
                    count.set(count.get() + 1);
                    sub_a.borrow_mut().take();
                    cx.observe_global::<TestGlobal>(|_| {}).detach();
                })
            }
        }));

        // Observer B fires second. It just drops itself.
        *sub_b.borrow_mut() = Some(app.update({
            let count = observer_b_count.clone();
            let sub_b = sub_b.clone();
            move |cx| {
                cx.observe_global::<TestGlobal>(move |_cx| {
                    count.set(count.get() + 1);
                    sub_b.borrow_mut().take();
                })
            }
        }));

        // Both fire once.
        app.update(|cx| cx.set_global(TestGlobal));
        assert_eq!(observer_a_count.get(), 1);
        assert_eq!(observer_b_count.get(), 1);

        // Neither should fire again — both dropped their subscriptions.
        app.update(|cx| cx.set_global(TestGlobal));
        assert_eq!(observer_a_count.get(), 1);
        assert_eq!(observer_b_count.get(), 1, "orphaned subscriber fired again");
    }

    #[test]
    fn test_callback_dropped_by_earlier_callback_does_not_fire() {
        struct TestGlobal;
        impl Global for TestGlobal {}

        let mut app = TestApp::new();
        app.set_global(TestGlobal);

        let observer_b_count = Rc::new(Cell::new(0usize));
        let sub_b: Rc<RefCell<Option<Subscription>>> = Default::default();

        // Observer A fires first and drops B's subscription.
        app.update({
            let sub_b = sub_b.clone();
            move |cx| {
                cx.observe_global::<TestGlobal>(move |_cx| {
                    sub_b.borrow_mut().take();
                })
                .detach();
            }
        });

        // Observer B fires second — but A already dropped it.
        *sub_b.borrow_mut() = Some(app.update({
            let count = observer_b_count.clone();
            move |cx| {
                cx.observe_global::<TestGlobal>(move |_cx| {
                    count.set(count.get() + 1);
                })
            }
        }));

        app.update(|cx| cx.set_global(TestGlobal));
        assert_eq!(
            observer_b_count.get(),
            0,
            "B should not fire — A dropped its subscription"
        );
    }

    #[test]
    fn test_self_drop_during_callback() {
        struct TestGlobal;
        impl Global for TestGlobal {}

        let mut app = TestApp::new();
        app.set_global(TestGlobal);

        let count = Rc::new(Cell::new(0usize));
        let sub: Rc<RefCell<Option<Subscription>>> = Default::default();

        *sub.borrow_mut() = Some(app.update({
            let count = count.clone();
            let sub = sub.clone();
            move |cx| {
                cx.observe_global::<TestGlobal>(move |_cx| {
                    count.set(count.get() + 1);
                    sub.borrow_mut().take();
                })
            }
        }));

        app.update(|cx| cx.set_global(TestGlobal));
        assert_eq!(count.get(), 1);

        app.update(|cx| cx.set_global(TestGlobal));
        assert_eq!(count.get(), 1, "should not fire after self-drop");
    }

    #[test]
    fn test_subscription_drop() {
        struct TestGlobal;
        impl Global for TestGlobal {}

        let mut app = TestApp::new();
        app.set_global(TestGlobal);

        let count = Rc::new(Cell::new(0usize));

        let subscription = app.update({
            let count = count.clone();
            move |cx| {
                cx.observe_global::<TestGlobal>(move |_cx| {
                    count.set(count.get() + 1);
                })
            }
        });

        drop(subscription);

        app.update(|cx| cx.set_global(TestGlobal));
        assert_eq!(count.get(), 0, "should not fire after drop");
    }
}
