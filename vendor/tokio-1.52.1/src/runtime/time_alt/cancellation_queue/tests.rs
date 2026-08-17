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
fn single_thread() {
    model(|| {
        for i in 0..NUM_ITEMS {
            let (tx, mut rx) = new();

            for _ in 0..i {
                unsafe { tx.send(new_handle()) };
            }

            assert_eq!(rx.recv_all().count(), i);
        }
    });
}

#[test]
#[cfg(not(target_os = "wasi"))] // No thread on wasi.
fn multi_thread() {
    use crate::loom::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    use crate::loom::sync::Arc;
    use crate::loom::thread;

    #[cfg(loom)]
    const NUM_THREADS: usize = 3;
    #[cfg(not(loom))]
    const NUM_THREADS: usize = 8;

    model(|| {
        let (tx, mut rx) = new();
        let mut jhs = Vec::new();
        let sent = Arc::new(AtomicUsize::new(0));

        for _ in 0..NUM_THREADS {
            let tx = tx.clone();
            let sent = sent.clone();
            jhs.push(thread::spawn(move || {
                for _ in 0..NUM_ITEMS {
                    unsafe { tx.send(new_handle()) };
                    sent.fetch_add(1, SeqCst);
                }
            }));
        }

        let mut count = 0;
        loop {
            count += rx.recv_all().count();
            if sent.fetch_add(0, SeqCst) == NUM_ITEMS * NUM_THREADS {
                jhs.into_iter().for_each(|jh| {
                    jh.join().unwrap();
                });
                count += rx.recv_all().count();
                break;
            }
            thread::yield_now();
        }

        assert_eq!(count, NUM_ITEMS * NUM_THREADS);
    })
}

#[test]
fn drop_iter_should_not_leak_memory() {
    model(|| {
        let (tx, mut rx) = new();

        let hdls = (0..NUM_ITEMS).map(|_| new_handle()).collect::<Vec<_>>();
        for hdl in hdls.iter() {
            unsafe { tx.send(hdl.clone()) };
        }

        drop(rx.recv_all());

        assert!(hdls.into_iter().all(|hdl| hdl.inner_strong_count() == 1));
    });
}
