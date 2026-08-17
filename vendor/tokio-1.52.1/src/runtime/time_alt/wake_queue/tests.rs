use super::*;

use futures_test::task::{new_count_waker, AwokenCount};

#[cfg(loom)]
const NUM_ITEMS: usize = 16;

#[cfg(not(loom))]
const NUM_ITEMS: usize = 64;

fn new_handle() -> (EntryHandle, AwokenCount) {
    let (waker, count) = new_count_waker();
    (EntryHandle::new(0, waker), count)
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
        let mut queue = WakeQueue::new();
        let mut counts = Vec::new();

        for _ in 0..NUM_ITEMS {
            let (hdl, count) = new_handle();
            counts.push(count);
            unsafe {
                queue.push_front(hdl);
            }
        }
        assert!(!queue.is_empty());
        queue.wake_all();
        assert!(counts.into_iter().all(|c| c.get() == 1));
    });
}

#[test]
fn drop_should_not_leak_memory() {
    model(|| {
        let mut queue = WakeQueue::new();

        let mut hdls = vec![];
        let mut counts = vec![];
        for _ in 0..NUM_ITEMS {
            let (hdl, count) = new_handle();
            hdls.push(hdl);
            counts.push(count);
        }

        for hdl in hdls.iter() {
            unsafe { queue.push_front(hdl.clone()) };
        }

        drop(queue);

        assert!(hdls.into_iter().all(|hdl| hdl.inner_strong_count() == 1));
        // drop should not wake any entries
        assert!(counts.into_iter().all(|count| count.get() == 0));
    });
}
