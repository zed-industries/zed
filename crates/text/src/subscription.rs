use crate::{Edit, Patch};
use parking_lot::Mutex;
use std::{
    mem,
    sync::{Arc, Weak},
};

#[derive(Default)]
pub struct Topic<T>(Mutex<Vec<Weak<Mutex<Patch<T>>>>>);

#[derive(Default)]
pub struct BatchTopic(Vec<Weak<Mutex<Vec<PatchBatch>>>>);

#[derive(Clone, Debug)]
pub struct PatchBatch {
    pub version: clock::Global,
    pub patch: Patch<usize>,
}

pub struct BatchSubscription(Arc<Mutex<Vec<PatchBatch>>>);

impl std::fmt::Debug for BatchSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchSubscription").finish_non_exhaustive()
    }
}

impl BatchTopic {
    pub fn subscribe(&mut self) -> BatchSubscription {
        let subscription = BatchSubscription(Arc::default());
        self.0.push(Arc::downgrade(&subscription.0));
        subscription
    }

    pub fn publish_mut(&mut self, version: &clock::Global, patch: &Patch<usize>) {
        if patch.is_empty() {
            return;
        }
        self.0.retain(|subscription| {
            if let Some(subscription) = subscription.upgrade() {
                subscription.lock().push(PatchBatch {
                    version: version.clone(),
                    patch: patch.clone(),
                });
                true
            } else {
                false
            }
        });
    }

    pub fn has_subscribers(&self) -> bool {
        !self.0.is_empty()
    }
}

impl BatchSubscription {
    pub fn drain(&self) -> Vec<PatchBatch> {
        mem::take(&mut *self.0.lock())
    }
}

pub struct Subscription<T>(Arc<Mutex<Patch<T>>>);

impl<T: Default, TDelta> Topic<T>
where
    T: 'static
        + Copy
        + Ord
        + std::ops::Sub<T, Output = TDelta>
        + std::ops::Add<TDelta, Output = T>
        + std::ops::AddAssign<TDelta>
        + Default,
    TDelta: Ord + Copy,
{
    pub fn subscribe(&mut self) -> Subscription<T> {
        let subscription = Subscription(Default::default());
        self.0.get_mut().push(Arc::downgrade(&subscription.0));
        subscription
    }

    pub fn publish(&self, edits: impl Clone + IntoIterator<Item = Edit<T>>) {
        publish(&mut self.0.lock(), edits);
    }

    pub fn publish_mut(&mut self, edits: impl Clone + IntoIterator<Item = Edit<T>>) {
        publish(self.0.get_mut(), edits);
    }
}

impl<T: Default> Subscription<T> {
    pub fn consume(&self) -> Patch<T> {
        mem::take(&mut *self.0.lock())
    }
}

fn publish<T, TDelta>(
    subscriptions: &mut Vec<Weak<Mutex<Patch<T>>>>,
    edits: impl Clone + IntoIterator<Item = Edit<T>>,
) where
    T: 'static
        + Copy
        + Ord
        + std::ops::Sub<T, Output = TDelta>
        + std::ops::Add<TDelta, Output = T>
        + std::ops::AddAssign<TDelta>
        + Default,
    TDelta: Ord + Copy,
{
    subscriptions.retain(|subscription| {
        if let Some(subscription) = subscription.upgrade() {
            let mut patch = subscription.lock();
            *patch = patch.compose(edits.clone());
            true
        } else {
            false
        }
    });
}
