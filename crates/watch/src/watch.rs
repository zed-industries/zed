mod error;

pub use error::*;
use parking_lot::{RwLock, RwLockReadGuard, RwLockUpgradableReadGuard};
use std::{
    collections::BTreeMap,
    mem,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
};

pub fn channel<T>(value: T) -> (Sender<T>, Receiver<T>) {
    let state = Arc::new(RwLock::new(State {
        value,
        wakers: BTreeMap::new(),
        next_waker_id: WakerId::default(),
        version: 0,
        closed: false,
    }));

    (
        Sender {
            state: state.clone(),
        },
        Receiver {
            state,
            observed_version: 0,
        },
    )
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct WakerId(usize);

impl WakerId {
    fn post_inc(&mut self) -> Self {
        let id = *self;
        self.0 = id.0.wrapping_add(1);
        *self
    }
}

struct State<T> {
    value: T,
    wakers: BTreeMap<WakerId, Waker>,
    next_waker_id: WakerId,
    version: usize,
    closed: bool,
}

pub struct Sender<T> {
    state: Arc<RwLock<State<T>>>,
}

impl<T> Sender<T> {
    pub fn receiver(&self) -> Receiver<T> {
        let observed_version = self.state.read().version;
        Receiver {
            state: self.state.clone(),
            observed_version,
        }
    }

    pub fn send(&mut self, value: T) -> Result<(), NoReceiverError> {
        if let Some(state) = Arc::get_mut(&mut self.state) {
            let state = state.get_mut();
            state.value = value;
            debug_assert_eq!(state.wakers.len(), 0);
            Err(NoReceiverError)
        } else {
            let mut state = self.state.write();
            state.value = value;
            state.version = state.version.wrapping_add(1);
            let wakers = mem::take(&mut state.wakers);
            drop(state);

            for (_, waker) in wakers {
                waker.wake();
            }

            Ok(())
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut state = self.state.write();
        state.closed = true;
        for (_, waker) in mem::take(&mut state.wakers) {
            waker.wake();
        }
    }
}

#[derive(Clone)]
pub struct Receiver<T> {
    state: Arc<RwLock<State<T>>>,
    observed_version: usize,
}

struct Changed<'a, T> {
    receiver: &'a mut Receiver<T>,
    pending_waker_id: Option<WakerId>,
}

impl<T> Future for Changed<'_, T> {
    type Output = Result<(), NoSenderError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = &mut *self;

        let state = this.receiver.state.upgradable_read();
        if state.closed {
            Poll::Ready(Err(NoSenderError))
        } else if state.version == this.receiver.observed_version {
            let mut state = RwLockUpgradableReadGuard::upgrade(state);

            // Remove the pending waker from the state. This should happen
            // automatically when the waker was woken up by the sender, but if
            // this future was polled again without an explicit call to `wake`
            // by the sender, we need to remove it manually.
            if let Some(pending_waker_id) = this.pending_waker_id.take() {
                state.wakers.remove(&pending_waker_id);
            }

            // Insert a waker so that this future can be woken up when the state changes.
            let waker_id = state.next_waker_id.post_inc();
            state.wakers.insert(waker_id, cx.waker().clone());
            this.pending_waker_id = Some(waker_id);

            Poll::Pending
        } else {
            // If the current version is different from what we observed, it
            // means that the sender updated the value. In this case, we don't
            // need to clear the pending waker because the sender has already
            // cleared it.
            this.pending_waker_id = None;
            this.receiver.observed_version = state.version;
            Poll::Ready(Ok(()))
        }
    }
}

impl<T> Drop for Changed<'_, T> {
    fn drop(&mut self) {
        // If this future gets dropped before the waker has a chance of being
        // woken up, we need to clear it to avoid a memory leak.
        if let Some(waker_id) = self.pending_waker_id {
            let mut state = self.receiver.state.write();
            state.wakers.remove(&waker_id);
        }
    }
}

impl<T> Receiver<T> {
    pub fn borrow(&self) -> parking_lot::MappedRwLockReadGuard<T> {
        RwLockReadGuard::map(self.state.read(), |state| &state.value)
    }

    pub fn changed(&mut self) -> impl Future<Output = Result<(), NoSenderError>> {
        Changed {
            receiver: self,
            pending_waker_id: None,
        }
    }
}

impl<T: Clone> Receiver<T> {
    pub async fn recv(&mut self) -> Result<T, NoSenderError> {
        self.changed().await?;
        Ok(self.borrow().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};

    #[gpui::test]
    async fn test_basic_watch() {
        let (mut sender, mut receiver) = channel(0);
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(receiver.recv().await, Ok(1));

        assert_eq!(sender.send(2), Ok(()));
        assert_eq!(sender.send(3), Ok(()));
        assert_eq!(receiver.recv().await, Ok(3));

        drop(receiver);
        assert_eq!(sender.send(4), Err(NoReceiverError));

        let mut receiver = sender.receiver();
        assert_eq!(sender.send(5), Ok(()));
        assert_eq!(receiver.recv().await, Ok(5));

        drop(sender);
        assert_eq!(receiver.recv().await, Err(NoSenderError));
    }

    #[gpui::test(iterations = 100)]
    async fn test_watch_random(cx: &mut TestAppContext) {
        let next_id = Arc::new(AtomicUsize::new(1));
        let closed = Arc::new(AtomicBool::new(false));
        let (mut tx, rx) = channel(0);
        let mut tasks = Vec::new();

        tasks.push(cx.background_spawn({
            let executor = cx.executor().clone();
            let next_id = next_id.clone();
            let closed = closed.clone();
            async move {
                for _ in 0..16 {
                    executor.simulate_random_delay().await;
                    let id = next_id.fetch_add(1, SeqCst);
                    zlog::info!("sending {}", id);
                    tx.send(id).ok();
                }
                closed.store(true, SeqCst);
            }
        }));

        for receiver_id in 0..16 {
            let executor = cx.executor().clone();
            let next_id = next_id.clone();
            let closed = closed.clone();
            let mut rx = rx.clone();
            let mut prev_observed_value = *rx.borrow();
            tasks.push(cx.background_spawn(async move {
                for _ in 0..16 {
                    executor.simulate_random_delay().await;

                    zlog::info!("{}: receiving", receiver_id);
                    match rx.recv().await {
                        Ok(value) => {
                            zlog::info!("{}: received {}", receiver_id, value);
                            assert!(!closed.load(SeqCst));
                            assert_eq!(value, next_id.load(SeqCst) - 1);
                            assert_ne!(value, prev_observed_value);
                            prev_observed_value = value;
                        }
                        Err(_) => {
                            zlog::info!("{}: closed", receiver_id);
                            assert!(closed.load(SeqCst));
                            break;
                        }
                    }
                }
            }));
        }

        for task in tasks {
            task.await;
        }
    }

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }
}
