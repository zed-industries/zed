extern crate futures;

use futures::prelude::*;
use futures::future;
use futures::unsync::oneshot::{channel, Canceled, spawn};

mod support;
use support::local_executor;

#[test]
fn smoke() {
    let (tx, rx) = channel();
    tx.send(33).unwrap();
    assert_eq!(rx.wait().unwrap(), 33);
}

#[test]
fn canceled() {
    let (_, rx) = channel::<()>();
    assert_eq!(rx.wait().unwrap_err(), Canceled);
}

#[test]
fn poll_cancel() {
    let (mut tx, _) = channel::<()>();
    assert!(tx.poll_cancel().unwrap().is_ready());
}

#[test]
fn tx_complete_rx_unparked() {
    let (tx, rx) = channel();

    let res = rx.join(future::lazy(move || {
        tx.send(55).unwrap();
        Ok(11)
    }));
    assert_eq!(res.wait().unwrap(), (55, 11));
}

#[test]
fn tx_dropped_rx_unparked() {
    let (tx, rx) = channel::<i32>();

    let res = rx.join(future::lazy(move || {
        let _tx = tx;
        Ok(11)
    }));
    assert_eq!(res.wait().unwrap_err(), Canceled);
}


#[test]
fn is_canceled() {
    let (tx, rx) = channel::<u32>();
    assert!(!tx.is_canceled());
    drop(rx);
    assert!(tx.is_canceled());
}

#[test]
fn spawn_sends_items() {
    let core = local_executor::Core::new();
    let future = future::ok::<_, ()>(1);
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
