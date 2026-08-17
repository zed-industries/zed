use super::*;

use futures::task::noop_waker;

#[cfg(loom)]
const NUM_ITEMS: usize = 16;

#[cfg(not(loom))]
const NUM_ITEMS: usize = 64;

fn new_handle() -> EntryHandle {
    EntryHandle::new(0, noop_waker())
}

fn model<F: Fn() + Send + Sync + 'static>(f: F) {
    #[cfg(loom)]
    loom::model(f);

    #[cfg(not(loom))]
    f();
}

#[test]
fn sanity() {
    model(|| {
        let mut queue = RegistrationQueue::new();
        for _ in 0..NUM_ITEMS {
            unsafe {
                queue.push_front(new_handle());
            }
        }
        for _ in 0..NUM_ITEMS {
            assert!(queue.pop_front().is_some());
        }
        assert!(queue.pop_front().is_none());
    });
}

#[test]
fn drop_should_not_leak_memory() {
    model(|| {
        let mut queue = RegistrationQueue::new();

        let hdls = (0..NUM_ITEMS).map(|_| new_handle()).collect::<Vec<_>>();
        for hdl in hdls.iter() {
            unsafe { queue.push_front(hdl.clone()) };
        }

        drop(queue);

        assert!(hdls.into_iter().all(|hdl| hdl.inner_strong_count() == 1));
    });
}
