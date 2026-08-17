#![cfg(feature = "use_std")]
#![allow(bare_trait_objects, unknown_lints)]

extern crate futures;

mod support;

use futures::prelude::*;
use futures::unsync::oneshot;
use futures::unsync::mpsc::{self, SendError};
use futures::future::lazy;
use futures::stream::{iter_ok, unfold};

use support::local_executor::Core;

#[test]
fn mpsc_send_recv() {
    let (tx, rx) = mpsc::channel::<i32>(1);
    let mut rx = rx.wait();

    tx.send(42).wait().unwrap();

    assert_eq!(rx.next(), Some(Ok(42)));
    assert_eq!(rx.next(), None);
}

#[test]
fn mpsc_rx_notready() {
    let (_tx, mut rx) = mpsc::channel::<i32>(1);

    lazy(|| {
        assert_eq!(rx.poll().unwrap(), Async::NotReady);
        Ok(()) as Result<(), ()>
    }).wait().unwrap();
}

#[test]
fn mpsc_rx_end() {
    let (_, mut rx) = mpsc::channel::<i32>(1);

    lazy(|| {
        assert_eq!(rx.poll().unwrap(), Async::Ready(None));
        Ok(()) as Result<(), ()>
    }).wait().unwrap();
}

#[test]
fn mpsc_tx_clone_weak_rc() {
    let (tx, mut rx) = mpsc::channel::<i32>(1); // rc = 1

    let tx_clone = tx.clone(); // rc = 2
    lazy(|| {
        assert_eq!(rx.poll().unwrap(), Async::NotReady);
        Ok(()) as Result<(), ()>
    }).wait().unwrap();

    drop(tx); // rc = 1
    lazy(|| {
        assert_eq!(rx.poll().unwrap(), Async::NotReady);
        Ok(()) as Result<(), ()>
    }).wait().unwrap();

    drop(tx_clone); // rc = 0
    lazy(|| {
        assert_eq!(rx.poll().unwrap(), Async::Ready(None));
        Ok(()) as Result<(), ()>
    }).wait().unwrap();
}

#[test]
fn mpsc_tx_notready() {
    let (tx, _rx) = mpsc::channel::<i32>(1);
    let tx = tx.send(1).wait().unwrap();
    lazy(move || {
        assert!(tx.send(2).poll().unwrap().is_not_ready());
        Ok(()) as Result<(), ()>
    }).wait().unwrap();
}

#[test]
fn mpsc_tx_err() {
    let (tx, _) = mpsc::channel::<i32>(1);
    lazy(move || {
        assert!(tx.send(2).poll().is_err());
        Ok(()) as Result<(), ()>
    }).wait().unwrap();
}

#[test]
fn mpsc_backpressure() {
    let (tx, rx) = mpsc::channel::<i32>(1);
    lazy(move || {
        iter_ok(vec![1, 2, 3])
            .forward(tx)
            .map_err(|e: SendError<i32>| panic!("{}", e))
            .join(rx.take(3).collect().map(|xs| {
                assert_eq!(xs, [1, 2, 3]);
            }))
    }).wait().unwrap();
}

#[test]
fn mpsc_unbounded() {
    let (tx, rx) = mpsc::unbounded::<i32>();
    lazy(move || {
        iter_ok(vec![1, 2, 3])
            .forward(tx)
            .map_err(|e: SendError<i32>| panic!("{}", e))
            .join(rx.take(3).collect().map(|xs| {
                assert_eq!(xs, [1, 2, 3]);
            }))
    }).wait().unwrap();
}

#[test]
fn mpsc_recv_unpark() {
    let core = Core::new();
    let (tx, rx) = mpsc::channel::<i32>(1);
    let tx2 = tx.clone();
    core.spawn(rx.collect().map(|xs| assert_eq!(xs, [1, 2])));
    core.spawn(lazy(move || tx.send(1).map(|_| ()).map_err(|e| panic!("{}", e))));
    core.run(lazy(move || tx2.send(2))).unwrap();
}

#[test]
fn mpsc_send_unpark() {
    let core = Core::new();
    let (tx, rx) = mpsc::channel::<i32>(1);
    let (donetx, donerx) = oneshot::channel();
    core.spawn(iter_ok(vec![1, 2]).forward(tx)
        .then(|x: Result<_, SendError<i32>>| {
            assert!(x.is_err());
            donetx.send(()).unwrap();
            Ok(())
        }));
    core.spawn(lazy(move || { let _ = rx; Ok(()) }));
    core.run(donerx).unwrap();
}

#[test]
fn spawn_sends_items() {
    let core = Core::new();
    let stream = unfold(0, |i| Some(Ok::<_,u8>((i, i + 1))));
    let rx = mpsc::spawn(stream, &core, 1);
    assert_eq!(core.run(rx.take(4).collect()).unwrap(),
               [0, 1, 2, 3]);
}

#[test]
fn spawn_kill_dead_stream() {
    use std::thread;
    use std::time::Duration;
    use futures::future::Either;

    // a stream which never returns anything (maybe a remote end isn't
    // responding), but dropping it leads to observable side effects
    // (like closing connections, releasing limited resources, ...)
    #[derive(Debug)]
    struct Dead {
        // when dropped you should get Err(oneshot::Canceled) on the
        // receiving end
        done: oneshot::Sender<()>,
    }
    impl Stream for Dead {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
            Ok(Async::NotReady)
        }
    }

    // need to implement a timeout for the test, as it would hang
    // forever right now
    let (timeout_tx, timeout_rx) = futures::sync::oneshot::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let _ = timeout_tx.send(());
    });

    let core = Core::new();
    let (done_tx, done_rx) = oneshot::channel();
    let stream = Dead{done: done_tx};
    let rx = mpsc::spawn(stream, &core, 1);
    let res = core.run(
        Ok::<_, ()>(())
        .into_future()
        .then(move |_| {
            // now drop the spawned stream: maybe some timeout exceeded,
            // or some connection on this end was closed by the remote
            // end.
            drop(rx);
            // and wait for the spawned stream to release its resources
            done_rx
        })
        .select2(timeout_rx)
    );
    match res {
        Err(Either::A((oneshot::Canceled, _))) => (),
        _ => {
            panic!("dead stream wasn't canceled");
        },
    }
}


/// Test case for PR #768 (issue #766).
/// The issue was:
/// Given that an empty channel is polled by the Receiver, and the only Sender
/// gets dropped without sending anything, then the Receiver would get stuck.

#[test]
fn dropped_sender_of_unused_channel_notifies_receiver() {
    let core = Core::new();
    type FUTURE = Box<futures::Future<Item=u8, Error=()>>;

    // Constructs the channel which we want to test, and two futures which
    // act on that channel.
    let pair = |reverse| -> Vec<FUTURE> {
        // This is the channel which we want to test.
        let (tx, rx) = mpsc::channel::<u8>(1);
        let mut futures: Vec<FUTURE> = vec![
            Box::new(futures::stream::iter_ok(vec![])
                .forward(tx)
                .map_err(|_: mpsc::SendError<u8>| ())
                .map(|_| 42)
            ),
            Box::new(rx.fold((), |_, _| Ok(()))
                .map(|_| 24)
            ),
        ];
        if reverse {
            futures.reverse();
        }
        futures
    };

    let make_test_future = |reverse| -> Box<Future<Item=Vec<u8>, Error=()>> {
        let f = futures::future::join_all(pair(reverse));

        // Use a timeout. This is not meant to test the `sync::oneshot` but
        // merely uses it to implement this timeout.
        let (timeout_tx, timeout_rx) = futures::sync::oneshot::channel::<Vec<u8>>();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            let x = timeout_tx.send(vec![0]);
            assert!(x.is_err(), "Test timed out.");
        });

        Box::new(f.select(timeout_rx.map_err(|_|()))
            .map_err(|x| x.0)
            .map(|x| x.0)
        )
    };

    // The order of the tested futures is important to test fix of PR #768.
    // We want future_2 to poll on the Receiver before the Sender is dropped.
    let result = core.run(make_test_future(false));
    assert!(result.is_ok());
    assert_eq!(vec![42, 24], result.unwrap());

    // Test also the other ordering:
    let result = core.run(make_test_future(true));
    assert!(result.is_ok());
    assert_eq!(vec![24, 42], result.unwrap());
}
