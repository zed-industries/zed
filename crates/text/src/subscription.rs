use crate::{Edit, Patch};
use parking_lot::Mutex;
use std::{
    mem,
    sync::{Arc, Weak},
};

#[derive(Default)]
pub struct Topic<T>(Mutex<Vec<Weak<Mutex<Patch<T>>>>>);

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
