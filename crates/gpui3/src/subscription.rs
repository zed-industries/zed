use collections::{BTreeMap, BTreeSet};
use parking_lot::Mutex;
use std::{fmt::Debug, mem, sync::Arc};
use util::post_inc;

#[derive(Clone)]
pub(crate) struct SubscriberSet<EmitterKey, Callback>(
    Arc<Mutex<SubscriberSetState<EmitterKey, Callback>>>,
);

struct SubscriberSetState<EmitterKey, Callback> {
    subscribers: BTreeMap<EmitterKey, BTreeMap<usize, Callback>>,
    dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
    next_subscriber_id: usize,
}

impl<EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
where
    EmitterKey: 'static + Ord + Clone + Debug,
    Callback: 'static,
{
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(SubscriberSetState {
            subscribers: Default::default(),
            dropped_subscribers: Default::default(),
            next_subscriber_id: 0,
        })))
    }

    pub fn insert(&self, emitter: EmitterKey, callback: Callback) -> Subscription {
        let mut lock = self.0.lock();
        let subscriber_id = post_inc(&mut lock.next_subscriber_id);
        lock.subscribers
            .entry(emitter.clone())
            .or_default()
            .insert(subscriber_id, callback);
        let this = self.0.clone();
        Subscription {
            unsubscribe: Some(Box::new(move || {
                let mut lock = this.lock();
                if let Some(subscribers) = lock.subscribers.get_mut(&emitter) {
                    subscribers.remove(&subscriber_id);
                    if subscribers.is_empty() {
                        lock.subscribers.remove(&emitter);
                        return;
                    }
                }

                // We didn't manage to remove the subscription, which means it was dropped
                // while invoking the callback. Mark it as dropped so that we can remove it
                // later.
                lock.dropped_subscribers.insert((emitter, subscriber_id));
            })),
        }
    }

    pub fn retain<F>(&self, emitter: &EmitterKey, mut f: F)
    where
        F: FnMut(&mut Callback) -> bool,
    {
        let entry = self.0.lock().subscribers.remove_entry(emitter);
        if let Some((emitter, mut subscribers)) = entry {
            subscribers.retain(|_, callback| f(callback));
            let mut lock = self.0.lock();

            // Add any new subscribers that were added while invoking the callback.
            if let Some(new_subscribers) = lock.subscribers.remove(&emitter) {
                subscribers.extend(new_subscribers);
            }

            // Remove any dropped subscriptions that were dropped while invoking the callback.
            for (dropped_emitter, dropped_subscription_id) in
                mem::take(&mut lock.dropped_subscribers)
            {
                debug_assert_eq!(emitter, dropped_emitter);
                subscribers.remove(&dropped_subscription_id);
            }

            if !subscribers.is_empty() {
                lock.subscribers.insert(emitter, subscribers);
            }
        }
    }
}

#[must_use]
pub struct Subscription {
    unsubscribe: Option<Box<dyn FnOnce()>>,
}

impl Subscription {
    pub fn detach(mut self) {
        self.unsubscribe.take();
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(unsubscribe) = self.unsubscribe.take() {
            unsubscribe();
        }
    }
}
