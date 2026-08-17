use std::time::{Instant, Duration};
use flume::*;

#[test]
fn send_recv() {
    let (tx, rx) = unbounded();
    for i in 0..1000 { tx.send(i).unwrap(); }
    for i in 0..1000 { assert_eq!(rx.try_recv().unwrap(), i); }
    assert!(rx.try_recv().is_err());
}

#[test]
fn iter() {
    let (tx, rx) = unbounded();
    for i in 0..1000 { tx.send(i).unwrap(); }
    drop(tx);
    assert_eq!(rx.iter().sum::<u32>(), (0..1000).sum());
}

#[test]
fn try_iter() {
    let (tx, rx) = unbounded();
    for i in 0..1000 { tx.send(i).unwrap(); }
    assert_eq!(rx.try_iter().sum::<u32>(), (0..1000).sum());
}

#[test]
fn iter_threaded() {
    let (tx, rx) = unbounded();
    for i in 0..1000 {
        let tx = tx.clone();
        std::thread::spawn(move || tx.send(i).unwrap());
    }
    drop(tx);
    assert_eq!(rx.iter().sum::<u32>(), (0..1000).sum());
}

#[cfg_attr(any(target_os = "macos", windows), ignore)] // FIXME #41
#[test]
fn send_timeout() {
    let dur = Duration::from_millis(350);
    let max_error = Duration::from_millis(5);
    let dur_min = dur.checked_sub(max_error).unwrap();
    let dur_max = dur.checked_add(max_error).unwrap();

    let (tx, rx) = bounded(1);

    assert!(tx.send_timeout(42, dur).is_ok());

    let then = Instant::now();
    assert!(tx.send_timeout(43, dur).is_err());
    let now = Instant::now();

    let this = now.duration_since(then);
    if !(dur_min < this && this < dur_max) {
        panic!("timeout exceeded: {:?}", this);
    }

    assert_eq!(rx.drain().count(), 1);

    drop(rx);

    assert!(tx.send_timeout(42, Duration::from_millis(350)).is_err());
}

#[cfg_attr(any(target_os = "macos", windows), ignore)] // FIXME #41
#[test]
fn recv_timeout() {
    let dur = Duration::from_millis(350);
    let max_error = Duration::from_millis(5);
    let dur_min = dur.checked_sub(max_error).unwrap();
    let dur_max = dur.checked_add(max_error).unwrap();

    let (tx, rx) = unbounded();
    let then = Instant::now();
    assert!(rx.recv_timeout(dur).is_err());
    let now = Instant::now();

    let this = now.duration_since(then);
    if !(dur_min < this && this < dur_max) {
        panic!("timeout exceeded: {:?}", this);
    }

    tx.send(42).unwrap();
    assert_eq!(rx.recv_timeout(dur), Ok(42));
    assert!(Instant::now().duration_since(now) < max_error);
}

#[cfg_attr(any(target_os = "macos", windows), ignore)] // FIXME #41
#[test]
fn recv_deadline() {
    let dur = Duration::from_millis(350);
    let max_error = Duration::from_millis(5);
    let dur_min = dur.checked_sub(max_error).unwrap();
    let dur_max = dur.checked_add(max_error).unwrap();

    let (tx, rx) = unbounded();
    let then = Instant::now();
    assert!(rx.recv_deadline(then.checked_add(dur).unwrap()).is_err());
    let now = Instant::now();

    let this = now.duration_since(then);
    if !(dur_min < this && this < dur_max) {
        panic!("timeout exceeded: {:?}", this);
    }

    tx.send(42).unwrap();
    assert_eq!(rx.recv_deadline(now.checked_add(dur).unwrap()), Ok(42));
    assert!(Instant::now().duration_since(now) < max_error);
}

#[test]
fn recv_timeout_missed_send() {
    let (tx, rx) = bounded(10);

    assert!(rx.recv_timeout(Duration::from_millis(100)).is_err());

    tx.send(42).unwrap();

    assert_eq!(rx.recv(), Ok(42));
}

#[test]
fn disconnect_tx() {
    let (tx, rx) = unbounded::<()>();
    drop(tx);
    assert!(rx.recv().is_err());
}

#[test]
fn disconnect_rx() {
    let (tx, rx) = unbounded();
    drop(rx);
    assert!(tx.send(0).is_err());
}

#[test]
fn drain() {
    let (tx, rx) = unbounded();

    for i in 0..100 {
        tx.send(i).unwrap();
    }

    assert_eq!(rx.drain().sum::<u32>(), (0..100).sum());

    for i in 0..100 {
        tx.send(i).unwrap();
    }

    for i in 0..100 {
        tx.send(i).unwrap();
    }

    rx.recv().unwrap();

    (1u32..100).chain(0..100).zip(rx).for_each(|(l, r)| assert_eq!(l, r));
}

#[test]
fn try_send() {
    let (tx, rx) = bounded(5);

    for i in 0..5 {
        tx.try_send(i).unwrap();
    }

    assert!(tx.try_send(42).is_err());

    assert_eq!(rx.recv(), Ok(0));

    assert_eq!(tx.try_send(42), Ok(()));

    assert_eq!(rx.recv(), Ok(1));
    drop(rx);

    assert!(tx.try_send(42).is_err());
}

#[test]
fn send_bounded() {
    let (tx, rx) = bounded(5);

    for _ in 0..5 {
        tx.send(42).unwrap();
    }

    let _ = rx.recv().unwrap();

    tx.send(42).unwrap();

    assert!(tx.try_send(42).is_err());

    rx.drain();

    let mut ts = Vec::new();
    for _ in 0..100 {
        let tx = tx.clone();
        ts.push(std::thread::spawn(move || {
            for i in 0..10000 {
                tx.send(i).unwrap();
            }
        }));
    }

    drop(tx);

    assert_eq!(rx.iter().sum::<u64>(), (0..10000).sum::<u64>() * 100);

    for t in ts {
        t.join().unwrap();
    }

    assert!(rx.recv().is_err());
}

#[test]
fn rendezvous() {
    let (tx, rx) = bounded(0);

    for i in 0..5 {
        let tx = tx.clone();
        let t = std::thread::spawn(move || {
            assert!(tx.try_send(()).is_err());

            let then = Instant::now();
            tx.send(()).unwrap();
            let now = Instant::now();

            assert!(now.duration_since(then) > Duration::from_millis(100), "iter = {}", i);
        });

        std::thread::sleep(Duration::from_millis(1000));
        rx.recv().unwrap();

        t.join().unwrap();
    }
}

#[test]
fn hydra() {
    let thread_num = 32;
    let msg_num = 1000;

    let (main_tx, main_rx) = unbounded::<()>();

    let mut txs = Vec::new();
    for _ in 0..thread_num {
        let main_tx = main_tx.clone();
        let (tx, rx) = unbounded();
        txs.push(tx);

        std::thread::spawn(move || {
            for msg in rx.iter() {
                main_tx.send(msg).unwrap();
            }
        });
    }

    drop(main_tx);

    for _ in 0..10 {
        for tx in &txs {
            for _ in 0..msg_num {
                tx.send(Default::default()).unwrap();
            }
        }

        for _ in 0..thread_num {
            for _ in 0..msg_num {
                main_rx.recv().unwrap();
            }
        }
    }

    drop(txs);
    assert!(main_rx.recv().is_err());
}

#[test]
fn robin() {
    let thread_num = 32;
    let msg_num = 10;

    let (mut main_tx, main_rx) = bounded::<()>(1);

    for _ in 0..thread_num {
        let (mut tx, rx) = bounded(100);
        std::mem::swap(&mut tx, &mut main_tx);

        std::thread::spawn(move || {
            for msg in rx.iter() {
                tx.send(msg).unwrap();
            }
        });
    }

    for _ in 0..10 {
        let main_tx = main_tx.clone();
        std::thread::spawn(move || {
            for _ in 0..msg_num {
                main_tx.send(Default::default()).unwrap();
            }
        });

        for _ in 0..msg_num {
            main_rx.recv().unwrap();
        }
    }
}

#[cfg(feature = "select")]
#[test]
fn select_general() {
    #[derive(Debug, PartialEq)]
    struct Foo(usize);

    let (tx0, rx0) = bounded(1);
    let (tx1, rx1) = unbounded();

    for (i, t) in vec![tx0.clone(), tx1].into_iter().enumerate() {
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(250));
            let _ = t.send(Foo(i));
        });
    }

    let x = Selector::new()
        .recv(&rx0, |x| x)
        .recv(&rx1, |x| x)
        .wait()
        .unwrap();

    if x == Foo(0) {
        assert!(rx1.recv().unwrap() == Foo(1));
    } else {
        assert!(rx0.recv().unwrap() == Foo(0));
    }

    tx0.send(Foo(42)).unwrap();

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(rx0.recv().unwrap(), Foo(42));
        assert_eq!(rx0.recv().unwrap(), Foo(43));

    });

    Selector::new()
        .send(&tx0, Foo(43), |x| x)
        .wait()
        .unwrap();

    t.join().unwrap();
}

struct MessageWithoutDebug(u32);

#[test]
// This is a 'does it build' test, to make sure that the error types can turn
// into a std::error::Error without requiring the payload (which is not used
// there) to impl Debug.
fn std_error_without_debug() {
    let (tx, rx) = unbounded::<MessageWithoutDebug>();

    match tx.send(MessageWithoutDebug(1)) {
        Ok(_) => {}
        Err(e) => {
            let _std_err: &dyn std::error::Error = &e;
        }
    }

    match rx.recv() {
        Ok(_) => {}
        Err(e) => {
            let _std_err: &dyn std::error::Error = &e;
        }
    }

    match tx.try_send(MessageWithoutDebug(2)) {
        Ok(_) => {}
        Err(e) => {
            let _std_err: &dyn std::error::Error = &e;
        }
    }

    match rx.try_recv() {
        Ok(_) => {}
        Err(e) => {
            let _std_err: &dyn std::error::Error = &e;
        }
    }

    match tx.send_timeout(MessageWithoutDebug(3), Duration::from_secs(1000000)) {
        Ok(_) => {}
        Err(e) => {
            let _std_err: &dyn std::error::Error = &e;
        }
    }

    match rx.recv_timeout(Duration::from_secs(10000000)) {
        Ok(_) => {}
        Err(e) => {
            let _std_err: &dyn std::error::Error = &e;
        }
    }
}

#[test]
fn weak_close() {
    let (tx, rx) = unbounded::<()>();
    let weak = tx.downgrade();
    drop(tx);
    assert!(weak.upgrade().is_none());
    assert!(rx.is_disconnected());
    assert!(rx.try_recv().is_err());
}

#[test]
fn weak_upgrade() {
    let (tx, rx) = unbounded();
    let weak = tx.downgrade();
    let tx2 = weak.upgrade().unwrap();
    drop(tx);
    assert!(!rx.is_disconnected());
    tx2.send(()).unwrap();
    assert!(rx.try_recv().is_ok());
}
