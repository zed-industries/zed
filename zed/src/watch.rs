// TODO: This implementation is actually broken in that it will only

use gpui::{Entity, ModelContext, View, ViewContext};
use smol::{channel, lock::RwLock};
use std::ops::Deref;
use std::sync::Arc;

pub struct Sender<T> {
    value: Arc<RwLock<T>>,
    updated: channel::Sender<()>,
}

#[derive(Clone)]
pub struct Receiver<T> {
    value: Arc<RwLock<T>>,
    updated: channel::Receiver<()>,
}

impl<T> Sender<T> {
    pub async fn update<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        let result = f(&mut *self.value.write().await);
        self.updated.send(()).await.unwrap();
        result
    }
}

impl<T> Receiver<T> {
    pub async fn updated(&self) {
        let _ = self.updated.recv().await;
    }

    pub async fn read<'a>(&'a self) -> impl 'a + Deref<Target = T> {
        self.value.read().await
    }
}

// TODO: These implementations are broken because they only handle a single update.
impl<T: 'static + Clone> Receiver<T> {
    pub fn notify_model_on_change<M: 'static + Entity>(&self, ctx: &mut ModelContext<M>) {
        let watch = self.clone();
        ctx.spawn(async move { watch.updated().await }, |_, _, ctx| {
            ctx.notify()
        })
        .detach();
    }

    pub fn notify_view_on_change<V: 'static + View>(&self, ctx: &mut ViewContext<V>) {
        let watch = self.clone();
        ctx.spawn(async move { watch.updated().await }, |_, _, ctx| {
            ctx.notify()
        })
        .detach();
    }
}

pub fn channel<T>(value: T) -> (Sender<T>, Receiver<T>) {
    let value = Arc::new(RwLock::new(value));
    let (s, r) = channel::unbounded();
    let sender = Sender {
        value: value.clone(),
        updated: s,
    };
    let receiver = Receiver { value, updated: r };
    (sender, receiver)
}
