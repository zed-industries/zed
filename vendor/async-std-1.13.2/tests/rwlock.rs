use std::cell::Cell;
use std::num::Wrapping;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use async_std::prelude::*;
use async_std::sync::RwLock;
use async_std::task;
use futures::channel::mpsc;

#[cfg(not(target_os = "unknown"))]
use async_std::task::spawn;
#[cfg(target_os = "unknown")]
use async_std::task::spawn_local as spawn;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Generates a random number in `0..n`.
pub fn random(n: u32) -> u32 {
    thread_local! {
        static RNG: Cell<Wrapping<u32>> = Cell::new(Wrapping(1_406_868_647));
    }

    RNG.with(|rng| {
        // This is the 32-bit variant of Xorshift.
        //
        // Source: https://en.wikipedia.org/wiki/Xorshift
        let mut x = rng.get();
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        rng.set(x);

        // This is a fast alternative to `x % n`.
        //
        // Author: Daniel Lemire
        // Source: https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
        ((x.0 as u64).wrapping_mul(n as u64) >> 32) as u32
    })
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn smoke() {
    task::block_on(async {
        let lock = RwLock::new(());
        drop(lock.read().await);
        drop(lock.write().await);
        drop((lock.read().await, lock.read().await));
        drop(lock.write().await);
    });
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn try_write() {
    task::block_on(async {
        let lock = RwLock::new(0isize);
        let read_guard = lock.read().await;
        assert!(lock.try_write().is_none());
        drop(read_guard);
    });
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn into_inner() {
    let lock = RwLock::new(10);
    assert_eq!(lock.into_inner(), 10);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn into_inner_and_drop() {
    struct Counter(Arc<AtomicUsize>);

    impl Drop for Counter {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    let cnt = Arc::new(AtomicUsize::new(0));
    let lock = RwLock::new(Counter(cnt.clone()));
    assert_eq!(cnt.load(Ordering::SeqCst), 0);

    {
        let _inner = lock.into_inner();
        assert_eq!(cnt.load(Ordering::SeqCst), 0);
    }

    assert_eq!(cnt.load(Ordering::SeqCst), 1);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn get_mut() {
    let mut lock = RwLock::new(10);
    *lock.get_mut() = 20;
    assert_eq!(lock.into_inner(), 20);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn contention() {
    const N: u32 = 10;
    const M: usize = 1000;

    let (tx, mut rx) = mpsc::unbounded();
    let tx = Arc::new(tx);
    let rw = Arc::new(RwLock::new(()));

    // Spawn N tasks that randomly acquire the lock M times.
    for _ in 0..N {
        let tx = tx.clone();
        let rw = rw.clone();

        spawn(async move {
            for _ in 0..M {
                if random(N) == 0 {
                    drop(rw.write().await);
                } else {
                    drop(rw.read().await);
                }
            }
            tx.unbounded_send(()).unwrap();
        });
    }

    task::block_on(async move {
        for _ in 0..N {
            rx.next().await.unwrap();
        }
    });
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn writer_and_readers() {
    #[derive(Default)]
    struct Yield(Cell<bool>);

    impl Future for Yield {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0.get() {
                Poll::Ready(())
            } else {
                self.0.set(true);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    let lock = Arc::new(RwLock::new(0i32));
    let (tx, mut rx) = mpsc::unbounded();

    // Spawn a writer task.
    spawn({
        let lock = lock.clone();
        async move {
            let mut lock = lock.write().await;
            for _ in 0..10 {
                let tmp = *lock;
                *lock = -1;
                Yield::default().await;
                *lock = tmp + 1;
            }
            tx.unbounded_send(()).unwrap();
        }
    });

    // Readers try to catch the writer in the act.
    let mut readers = Vec::new();
    for _ in 0..5 {
        let lock = lock.clone();
        readers.push(spawn(async move {
            let lock = lock.read().await;
            assert!(*lock >= 0);
        }));
    }

    task::block_on(async move {
        // Wait for readers to pass their asserts.
        for r in readers {
            r.await;
        }

        // Wait for writer to finish.
        rx.next().await.unwrap();
        let lock = lock.read().await;
        assert_eq!(*lock, 10);
    });
}
