use crate::{Edit, Patch};
use parking_lot::Mutex;
use std::{
    mem,
    sync::{Arc, Weak},
};

#[derive(Default)]
pub struct Topic(Mutex<Vec<Weak<Mutex<Patch<usize>>>>>);

pub struct Subscription(Arc<Mutex<Patch<usize>>>);

impl Topic {
    pub fn subscribe(&mut self) -> Subscription {
        let subscription = Subscription(Default::default());
        self.0.get_mut().push(Arc::downgrade(&subscription.0));
        subscription
    }

    pub fn publish(&self, edits: impl Clone + IntoIterator<Item = Edit<usize>>) {
        publish(&mut self.0.lock(), edits);
    }

    pub fn publish_mut(&mut self, edits: impl Clone + IntoIterator<Item = Edit<usize>>) {
        publish(self.0.get_mut(), edits);
    }
}

impl Subscription {
    pub fn consume(&self) -> Patch<usize> {
        mem::take(&mut *self.0.lock())
    }
}

fn publish(
    subscriptions: &mut Vec<Weak<Mutex<Patch<usize>>>>,
    edits: impl Clone + IntoIterator<Item = Edit<usize>>,
) {
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
