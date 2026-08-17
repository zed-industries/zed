#![allow(clippy::bool_assert_comparison)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;

use async_channel::{bounded, RecvError, SendError, TryRecvError, TrySendError};
use easy_parallel::Parallel;
use futures_lite::{future, prelude::*};

fn ms(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

#[test]
fn smoke() {
    let (s, r) = bounded(1);

    future::block_on(s.send(7)).unwrap();
    assert_eq!(r.try_recv(), Ok(7));

    future::block_on(s.send(8)).unwrap();
    assert_eq!(future::block_on(r.recv()), Ok(8));

    assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn smoke_blocking() {
    let (s, r) = bounded(1);

    s.send_blocking(7).unwrap();
    assert_eq!(r.try_recv(), Ok(7));

    s.send_blocking(8).unwrap();
    assert_eq!(future::block_on(r.recv()), Ok(8));

    future::block_on(s.send(9)).unwrap();
    assert_eq!(r.recv_blocking(), Ok(9));

    assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
}

#[test]
fn capacity() {
    for i in 1..10 {
        let (s, r) = bounded::<()>(i);
        assert_eq!(s.capacity(), Some(i));
        assert_eq!(r.capacity(), Some(i));
    }
}

#[test]
fn len_empty_full() {
    let (s, r) = bounded(2);

    assert_eq!(s.len(), 0);
    assert_eq!(s.is_empty(), true);
    assert_eq!(s.is_full(), false);
    assert_eq!(r.len(), 0);
    assert_eq!(r.is_empty(), true);
    assert_eq!(r.is_full(), false);

    future::block_on(s.send(())).unwrap();

    assert_eq!(s.len(), 1);
    assert_eq!(s.is_empty(), false);
    assert_eq!(s.is_full(), false);
    assert_eq!(r.len(), 1);
    assert_eq!(r.is_empty(), false);
    assert_eq!(r.is_full(), false);

    future::block_on(s.send(())).unwrap();

    assert_eq!(s.len(), 2);
    assert_eq!(s.is_empty(), false);
    assert_eq!(s.is_full(), true);
    assert_eq!(r.len(), 2);
    assert_eq!(r.is_empty(), false);
    assert_eq!(r.is_full(), true);

    future::block_on(r.recv()).unwrap();

    assert_eq!(s.len(), 1);
    assert_eq!(s.is_empty(), false);
    assert_eq!(s.is_full(), false);
    assert_eq!(r.len(), 1);
    assert_eq!(r.is_empty(), false);
    assert_eq!(r.is_full(), false);
}

#[test]
fn try_recv() {
    let (s, r) = bounded(100);

    Parallel::new()
        .add(move || {
            assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
            sleep(ms(1500));
            assert_eq!(r.try_recv(), Ok(7));
            sleep(ms(500));
            assert_eq!(r.try_recv(), Err(TryRecvError::Closed));
        })
        .add(move || {
            sleep(ms(1000));
            future::block_on(s.send(7)).unwrap();
        })
        .run();
}

#[test]
fn recv() {
    let (s, r) = bounded(100);

    Parallel::new()
        .add(move || {
            assert_eq!(future::block_on(r.recv()), Ok(7));
            sleep(ms(1000));
            assert_eq!(future::block_on(r.recv()), Ok(8));
            sleep(ms(1000));
            assert_eq!(future::block_on(r.recv()), Ok(9));
            assert_eq!(future::block_on(r.recv()), Err(RecvError));
        })
        .add(move || {
            sleep(ms(1500));
            future::block_on(s.send(7)).unwrap();
            future::block_on(s.send(8)).unwrap();
            future::block_on(s.send(9)).unwrap();
        })
        .run();
}

#[test]
fn try_send() {
    let (s, r) = bounded(1);

    Parallel::new()
        .add(move || {
            assert_eq!(s.try_send(1), Ok(()));
            assert_eq!(s.try_send(2), Err(TrySendError::Full(2)));
            sleep(ms(1500));
            assert_eq!(s.try_send(3), Ok(()));
            sleep(ms(500));
            assert_eq!(s.try_send(4), Err(TrySendError::Closed(4)));
        })
        .add(move || {
            sleep(ms(1000));
            assert_eq!(r.try_recv(), Ok(1));
            assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
            assert_eq!(future::block_on(r.recv()), Ok(3));
        })
        .run();
}

#[test]
fn send() {
    let (s, r) = bounded(1);

    Parallel::new()
        .add(|| {
            future::block_on(s.send(7)).unwrap();
            sleep(ms(1000));
            future::block_on(s.send(8)).unwrap();
            sleep(ms(1000));
            future::block_on(s.send(9)).unwrap();
            sleep(ms(1000));
            future::block_on(s.send(10)).unwrap();
        })
        .add(|| {
            sleep(ms(1500));
            assert_eq!(future::block_on(r.recv()), Ok(7));
            assert_eq!(future::block_on(r.recv()), Ok(8));
            assert_eq!(future::block_on(r.recv()), Ok(9));
        })
        .run();
}

#[test]
fn send_after_close() {
    let (s, r) = bounded(100);

    future::block_on(s.send(1)).unwrap();
    future::block_on(s.send(2)).unwrap();
    future::block_on(s.send(3)).unwrap();

    drop(r);

    assert_eq!(future::block_on(s.send(4)), Err(SendError(4)));
    assert_eq!(s.try_send(5), Err(TrySendError::Closed(5)));
    assert_eq!(future::block_on(s.send(6)), Err(SendError(6)));
}

#[test]
fn recv_after_close() {
    let (s, r) = bounded(100);

    future::block_on(s.send(1)).unwrap();
    future::block_on(s.send(2)).unwrap();
    future::block_on(s.send(3)).unwrap();

    drop(s);

    assert_eq!(future::block_on(r.recv()), Ok(1));
    assert_eq!(future::block_on(r.recv()), Ok(2));
    assert_eq!(future::block_on(r.recv()), Ok(3));
    assert_eq!(future::block_on(r.recv()), Err(RecvError));
}

#[test]
fn len() {
    const COUNT: usize = 25_000;
    const CAP: usize = 1000;

    let (s, r) = bounded(CAP);

    assert_eq!(s.len(), 0);
    assert_eq!(r.len(), 0);

    for _ in 0..CAP / 10 {
        for i in 0..50 {
            future::block_on(s.send(i)).unwrap();
            assert_eq!(s.len(), i + 1);
        }

        for i in 0..50 {
            future::block_on(r.recv()).unwrap();
            assert_eq!(r.len(), 50 - i - 1);
        }
    }

    assert_eq!(s.len(), 0);
    assert_eq!(r.len(), 0);

    for i in 0..CAP {
        future::block_on(s.send(i)).unwrap();
        assert_eq!(s.len(), i + 1);
    }

    for _ in 0..CAP {
        future::block_on(r.recv()).unwrap();
    }

    assert_eq!(s.len(), 0);
    assert_eq!(r.len(), 0);

    Parallel::new()
        .add(|| {
            for i in 0..COUNT {
                assert_eq!(future::block_on(r.recv()), Ok(i));
                let len = r.len();
                assert!(len <= CAP);
            }
        })
        .add(|| {
            for i in 0..COUNT {
                future::block_on(s.send(i)).unwrap();
                let len = s.len();
                assert!(len <= CAP);
            }
        })
        .run();

    assert_eq!(s.len(), 0);
    assert_eq!(r.len(), 0);
}

#[test]
fn receiver_count() {
    let (s, r) = bounded::<()>(5);
    let receiver_clones: Vec<_> = (0..20).map(|_| r.clone()).collect();

    assert_eq!(s.receiver_count(), 21);
    assert_eq!(r.receiver_count(), 21);

    drop(receiver_clones);

    assert_eq!(s.receiver_count(), 1);
    assert_eq!(r.receiver_count(), 1);
}

#[test]
fn sender_count() {
    let (s, r) = bounded::<()>(5);
    let sender_clones: Vec<_> = (0..20).map(|_| s.clone()).collect();

    assert_eq!(s.sender_count(), 21);
    assert_eq!(r.sender_count(), 21);

    drop(sender_clones);

    assert_eq!(s.receiver_count(), 1);
    assert_eq!(r.receiver_count(), 1);
}

#[test]
fn close_wakes_sender() {
    let (s, r) = bounded(1);

    Parallel::new()
        .add(move || {
            assert_eq!(future::block_on(s.send(())), Ok(()));
            assert_eq!(future::block_on(s.send(())), Err(SendError(())));
        })
        .add(move || {
            sleep(ms(1000));
            drop(r);
        })
        .run();
}

#[test]
fn close_wakes_receiver() {
    let (s, r) = bounded::<()>(1);

    Parallel::new()
        .add(move || {
            assert_eq!(future::block_on(r.recv()), Err(RecvError));
        })
        .add(move || {
            sleep(ms(1000));
            drop(s);
        })
        .run();
}

#[test]
fn forget_blocked_sender() {
    let (s1, r) = bounded(2);
    let s2 = s1.clone();

    Parallel::new()
        .add(move || {
            assert!(future::block_on(s1.send(3)).is_ok());
            assert!(future::block_on(s1.send(7)).is_ok());
            let mut s1_fut = s1.send(13);
            // Poll but keep the future alive.
            assert_eq!(future::block_on(future::poll_once(&mut s1_fut)), None);
            sleep(ms(500));
        })
        .add(move || {
            sleep(ms(100));
            assert!(future::block_on(s2.send(42)).is_ok());
        })
        .add(move || {
            sleep(ms(200));
            assert_eq!(future::block_on(r.recv()), Ok(3));
            assert_eq!(future::block_on(r.recv()), Ok(7));
            sleep(ms(100));
            assert_eq!(r.try_recv(), Ok(42));
        })
        .run();
}

#[test]
fn forget_blocked_receiver() {
    let (s, r1) = bounded(2);
    let r2 = r1.clone();

    Parallel::new()
        .add(move || {
            let mut r1_fut = r1.recv();
            // Poll but keep the future alive.
            assert_eq!(future::block_on(future::poll_once(&mut r1_fut)), None);
            sleep(ms(500));
        })
        .add(move || {
            sleep(ms(100));
            assert_eq!(future::block_on(r2.recv()), Ok(3));
        })
        .add(move || {
            sleep(ms(200));
            assert!(future::block_on(s.send(3)).is_ok());
            assert!(future::block_on(s.send(7)).is_ok());
            sleep(ms(100));
            assert!(s.try_send(42).is_ok());
        })
        .run();
}

#[test]
fn spsc() {
    const COUNT: usize = 100_000;

    let (s, r) = bounded(3);

    Parallel::new()
        .add(move || {
            for i in 0..COUNT {
                assert_eq!(future::block_on(r.recv()), Ok(i));
            }
            assert_eq!(future::block_on(r.recv()), Err(RecvError));
        })
        .add(move || {
            for i in 0..COUNT {
                future::block_on(s.send(i)).unwrap();
            }
        })
        .run();
}

#[test]
fn mpmc() {
    const COUNT: usize = 25_000;
    const THREADS: usize = 4;

    let (s, r) = bounded::<usize>(3);
    let v = (0..COUNT).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();

    Parallel::new()
        .each(0..THREADS, |_| {
            for _ in 0..COUNT {
                let n = future::block_on(r.recv()).unwrap();
                v[n].fetch_add(1, Ordering::SeqCst);
            }
        })
        .each(0..THREADS, |_| {
            for i in 0..COUNT {
                future::block_on(s.send(i)).unwrap();
            }
        })
        .run();

    for c in v {
        assert_eq!(c.load(Ordering::SeqCst), THREADS);
    }
}

#[test]
fn mpmc_stream() {
    const COUNT: usize = 25_000;
    const THREADS: usize = 4;

    let (s, r) = bounded::<usize>(3);
    let v = (0..COUNT).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();
    let v = &v;

    Parallel::new()
        .each(0..THREADS, {
            let mut r = r;
            move |_| {
                for _ in 0..COUNT {
                    let n = future::block_on(r.next()).unwrap();
                    v[n].fetch_add(1, Ordering::SeqCst);
                }
            }
        })
        .each(0..THREADS, |_| {
            for i in 0..COUNT {
                future::block_on(s.send(i)).unwrap();
            }
        })
        .run();

    for c in v {
        assert_eq!(c.load(Ordering::SeqCst), THREADS);
    }
}

#[test]
fn weak() {
    let (s, r) = bounded::<usize>(3);

    // Create a weak sender/receiver pair.
    let (weak_s, weak_r) = (s.downgrade(), r.downgrade());

    // Upgrade and send.
    {
        let s = weak_s.upgrade().unwrap();
        s.send_blocking(3).unwrap();
        let r = weak_r.upgrade().unwrap();
        assert_eq!(r.recv_blocking(), Ok(3));
    }

    // Drop the original sender/receiver pair.
    drop((s, r));

    // Try to upgrade again.
    {
        assert!(weak_s.upgrade().is_none());
        assert!(weak_r.upgrade().is_none());
    }
}
