extern crate futures;

use std::sync::mpsc;
use std::thread;

use futures::prelude::*;
use futures::future::{lazy, ok};
use futures::sync::oneshot::*;

mod support;
use support::*;

#[test]
fn smoke_poll() {
    let (mut tx, rx) = channel::<u32>();
    let mut task = futures::executor::spawn(lazy(|| {
        assert!(tx.poll_cancel().unwrap().is_not_ready());
        assert!(tx.poll_cancel().unwrap().is_not_ready());
        drop(rx);
        assert!(tx.poll_cancel().unwrap().is_ready());
        assert!(tx.poll_cancel().unwrap().is_ready());
        ok::<(), ()>(())
    }));
    assert!(task.poll_future_notify(&notify_noop(), 0).unwrap().is_ready());
}

#[test]
fn cancel_notifies() {
    let (tx, rx) = channel::<u32>();
    let (tx2, rx2) = mpsc::channel();

    WaitForCancel { tx: tx }.then(move |v| tx2.send(v)).forget();
    drop(rx);
    rx2.recv().unwrap().unwrap();
}

struct WaitForCancel {
    tx: Sender<u32>,
}

impl Future for WaitForCancel {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        self.tx.poll_cancel()
    }
}

#[test]
fn cancel_lots() {
    let (tx, rx) = mpsc::channel::<(Sender<_>, mpsc::Sender<_>)>();
    let t = thread::spawn(move || {
        for (tx, tx2) in rx {
            WaitForCancel { tx: tx }.then(move |v| tx2.send(v)).forget();
        }

    });

    for _ in 0..20000 {
        let (otx, orx) = channel::<u32>();
        let (tx2, rx2) = mpsc::channel();
        tx.send((otx, tx2)).unwrap();
        drop(orx);
        rx2.recv().unwrap().unwrap();
    }
    drop(tx);

    t.join().unwrap();
}

#[test]
fn close() {
    let (mut tx, mut rx) = channel::<u32>();
    rx.close();
    assert!(rx.poll().is_err());
    assert!(tx.poll_cancel().unwrap().is_ready());
}

#[test]
fn close_wakes() {
    let (tx, mut rx) = channel::<u32>();
    let (tx2, rx2) = mpsc::channel();
    let t = thread::spawn(move || {
        rx.close();
        rx2.recv().unwrap();
    });
    WaitForCancel { tx: tx }.wait().unwrap();
    tx2.send(()).unwrap();
    t.join().unwrap();
}

#[test]
fn is_canceled() {
    let (tx, rx) = channel::<u32>();
    assert!(!tx.is_canceled());
    drop(rx);
    assert!(tx.is_canceled());
}

#[test]
fn cancel_sends() {
    let (tx, rx) = mpsc::channel::<Sender<_>>();
    let t = thread::spawn(move || {
        for otx in rx {
            let _ = otx.send(42);
        }
    });

    for _ in 0..20000 {
        let (otx, mut orx) = channel::<u32>();
        tx.send(otx).unwrap();

        orx.close();
        // Not necessary to wrap in a task because the implementation of oneshot
        // never calls `task::current()` if the channel has been closed already.
        let _ = orx.poll();
    }

    drop(tx);
    t.join().unwrap();
}

#[test]
fn spawn_sends_items() {
    let core = local_executor::Core::new();
    let future = ok::<_, ()>(1);
    let rx = spawn(future, &core);
    assert_eq!(core.run(rx).unwrap(), 1);
}

#[test]
fn spawn_kill_dead_stream() {
    use std::thread;
    use std::time::Duration;
    use futures::future::Either;
    use futures::sync::oneshot;

    // a future which never returns anything (forever accepting incoming
    // connections), but dropping it leads to observable side effects
    // (like closing listening sockets, releasing limited resources,
    // ...)
    #[derive(Debug)]
    struct Dead {
        // when dropped you should get Err(oneshot::Canceled) on the
        // receiving end
        done: oneshot::Sender<()>,
    }
    impl Future for Dead {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            Ok(Async::NotReady)
        }
    }

    // need to implement a timeout for the test, as it would hang
    // forever right now
    let (timeout_tx, timeout_rx) = oneshot::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let _ = timeout_tx.send(());
    });

    let core = local_executor::Core::new();
    let (done_tx, done_rx) = oneshot::channel();
    let future = Dead{done: done_tx};
    let rx = spawn(future, &core);
    let res = core.run(
        Ok::<_, ()>(())
        .into_future()
        .then(move |_| {
            // now drop the spawned future: maybe some timeout exceeded,
            // or some connection on this end was closed by the remote
            // end.
            drop(rx);
            // and wait for the spawned future to release its resources
            done_rx
        })
        .select2(timeout_rx)
    );
    match res {
        Err(Either::A((oneshot::Canceled, _))) => (),
        Ok(Either::B(((), _))) => {
            panic!("dead future wasn't canceled (timeout)");
        },
        _ => {
            panic!("dead future wasn't canceled (unexpected result)");
        },
    }
}

#[test]
fn spawn_dont_kill_forgot_dead_stream() {
    use std::thread;
    use std::time::Duration;
    use futures::future::Either;
    use futures::sync::oneshot;

    // a future which never returns anything (forever accepting incoming
    // connections), but dropping it leads to observable side effects
    // (like closing listening sockets, releasing limited resources,
    // ...)
    #[derive(Debug)]
    struct Dead {
        // when dropped you should get Err(oneshot::Canceled) on the
        // receiving end
        done: oneshot::Sender<()>,
    }
    impl Future for Dead {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            Ok(Async::NotReady)
        }
    }

    // need to implement a timeout for the test, as it would hang
    // forever right now
    let (timeout_tx, timeout_rx) = oneshot::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let _ = timeout_tx.send(());
    });

    let core = local_executor::Core::new();
    let (done_tx, done_rx) = oneshot::channel();
    let future = Dead{done: done_tx};
    let rx = spawn(future, &core);
    let res = core.run(
        Ok::<_, ()>(())
        .into_future()
        .then(move |_| {
            // forget the spawned future: should keep running, i.e. hit
            // the timeout below.
            rx.forget();
            // and wait for the spawned future to release its resources
            done_rx
        })
        .select2(timeout_rx)
    );
    match res {
        Err(Either::A((oneshot::Canceled, _))) => {
            panic!("forgotten dead future was canceled");
        },
        Ok(Either::B(((), _))) => (), // reached timeout
        _ => {
            panic!("forgotten dead future was canceled (unexpected result)");
        },
    }
}
