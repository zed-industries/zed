use super::*;
use crate::loom::thread;

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
fn wake_up_in_the_same_thread() {
    model(|| {
        let mut counts = Vec::new();

        let mut reg_queue = RegistrationQueue::new();

        for _ in 0..NUM_ITEMS {
            let (hdl, count) = new_handle();
            counts.push(count);
            unsafe {
                reg_queue.push_front(hdl);
            }
        }

        let mut wake_queue = WakeQueue::new();
        for _ in 0..NUM_ITEMS {
            if let Some(hdl) = reg_queue.pop_front() {
                unsafe {
                    wake_queue.push_front(hdl);
                }
            }
        }
        assert!(reg_queue.pop_front().is_none());
        wake_queue.wake_all();

        assert!(counts.into_iter().all(|c| c.get() == 1));
    });
}

#[test]
fn cancel_in_the_same_thread() {
    model(|| {
        let mut counts = Vec::new();
        let (cancel_tx, mut cancel_rx) = cancellation_queue::new();

        let mut reg_queue = RegistrationQueue::new();

        for _ in 0..NUM_ITEMS {
            let (hdl, count) = new_handle();
            hdl.register_cancel_tx(cancel_tx.clone());
            counts.push(count);
            unsafe {
                reg_queue.push_front(hdl.clone());
            }
            hdl.cancel();
        }

        // drain the registration queue
        while let Some(hdl) = reg_queue.pop_front() {
            drop(hdl);
        }

        let mut wake_queue = WakeQueue::new();
        for hdl in cancel_rx.recv_all() {
            unsafe {
                wake_queue.push_front(hdl);
            }
        }
        wake_queue.wake_all();

        assert!(counts.into_iter().all(|c| c.get() == 0));
    });
}

#[test]
fn wake_up_in_the_different_thread() {
    model(|| {
        let mut counts = Vec::new();

        let mut hdls = Vec::new();
        let mut reg_queue = RegistrationQueue::new();

        for _ in 0..NUM_ITEMS {
            let (hdl, count) = new_handle();
            counts.push(count);
            hdls.push(hdl.clone());
            unsafe {
                reg_queue.push_front(hdl);
            }
        }

        // wake up all handles in a different thread
        thread::spawn(move || {
            let mut wake_queue = WakeQueue::new();
            for _ in 0..NUM_ITEMS {
                if let Some(hdl) = reg_queue.pop_front() {
                    unsafe {
                        wake_queue.push_front(hdl);
                    }
                }
            }
            assert!(reg_queue.pop_front().is_none());
            wake_queue.wake_all();
            assert!(counts.into_iter().all(|c| c.get() == 1));
        })
        .join()
        .unwrap();
    });
}

#[test]
fn cancel_in_the_different_thread() {
    model(|| {
        let mut counts = Vec::new();
        let (cancel_tx, mut cancel_rx) = cancellation_queue::new();
        let mut hdls = Vec::new();
        let mut reg_queue = RegistrationQueue::new();

        for _ in 0..NUM_ITEMS {
            let (hdl, count) = new_handle();
            hdl.register_cancel_tx(cancel_tx.clone());
            counts.push(count);
            hdls.push(hdl.clone());
            unsafe {
                reg_queue.push_front(hdl);
            }
        }

        // this thread cancel all handles concurrently
        let jh = thread::spawn(move || {
            // cancel all handles
            for hdl in hdls {
                hdl.cancel();
            }
        });

        // cancellation queue concurrently
        while let Some(hdl) = reg_queue.pop_front() {
            drop(hdl);
        }

        let mut wake_queue = WakeQueue::new();
        for hdl in cancel_rx.recv_all() {
            unsafe {
                wake_queue.push_front(hdl);
            }
        }
        wake_queue.wake_all();
        assert!(counts.into_iter().all(|c| c.get() == 0));

        jh.join().unwrap();
    })
}
