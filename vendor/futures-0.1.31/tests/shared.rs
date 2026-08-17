#![allow(bare_trait_objects, unknown_lints)]

extern crate futures;

mod support;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

use futures::sync::oneshot;
use futures::prelude::*;
use futures::future;

fn send_shared_oneshot_and_wait_on_multiple_threads(threads_number: u32) {
    let (tx, rx) = oneshot::channel::<u32>();
    let f = rx.shared();
    let threads = (0..threads_number).map(|_| {
        let cloned_future = f.clone();
        thread::spawn(move || {
            assert_eq!(*cloned_future.wait().unwrap(), 6);
        })
    }).collect::<Vec<_>>();
    tx.send(6).unwrap();
    assert_eq!(*f.wait().unwrap(), 6);
    for f in threads {
        f.join().unwrap();
    }
}

#[test]
fn one_thread() {
    send_shared_oneshot_and_wait_on_multiple_threads(1);
}

#[test]
fn two_threads() {
    send_shared_oneshot_and_wait_on_multiple_threads(2);
}

#[test]
fn many_threads() {
    send_shared_oneshot_and_wait_on_multiple_threads(1000);
}

#[test]
fn drop_on_one_task_ok() {
    let (tx, rx) = oneshot::channel::<u32>();
    let f1 = rx.shared();
    let f2 = f1.clone();

    let (tx2, rx2) = oneshot::channel::<u32>();

    let t1 = thread::spawn(|| {
        let f = f1.map_err(|_| ()).map(|x| *x).select(rx2.map_err(|_| ()));
        drop(f.wait());
    });

    let (tx3, rx3) = oneshot::channel::<u32>();

    let t2 = thread::spawn(|| {
        let _ = f2.map(|x| tx3.send(*x).unwrap()).map_err(|_| ()).wait();
    });

    tx2.send(11).unwrap(); // cancel `f1`
    t1.join().unwrap();

    tx.send(42).unwrap(); // Should cause `f2` and then `rx3` to get resolved.
    let result = rx3.wait().unwrap();
    assert_eq!(result, 42);
    t2.join().unwrap();
}

#[test]
fn drop_in_poll() {
    let slot = Rc::new(RefCell::new(None));
    let slot2 = slot.clone();
    let future = future::poll_fn(move || {
        drop(slot2.borrow_mut().take().unwrap());
        Ok::<_, u32>(1.into())
    }).shared();
    let future2 = Box::new(future.clone()) as Box<Future<Item=_, Error=_>>;
    *slot.borrow_mut() = Some(future2);
    assert_eq!(*future.wait().unwrap(), 1);
}

#[test]
fn peek() {
    let core = ::support::local_executor::Core::new();

    let (tx0, rx0) = oneshot::channel::<u32>();
    let f1 = rx0.shared();
    let f2 = f1.clone();

    // Repeated calls on the original or clone do not change the outcome.
    for _ in 0..2 {
        assert!(f1.peek().is_none());
        assert!(f2.peek().is_none());
    }

    // Completing the underlying future has no effect, because the value has not been `poll`ed in.
    tx0.send(42).unwrap();
    for _ in 0..2 {
        assert!(f1.peek().is_none());
        assert!(f2.peek().is_none());
    }

    // Once the Shared has been polled, the value is peekable on the clone.
    core.spawn(f1.map(|_|()).map_err(|_|()));
    core.run(future::ok::<(),()>(())).unwrap();
    for _ in 0..2 {
        assert_eq!(42, *f2.peek().unwrap().unwrap());
    }
}

#[test]
fn polled_then_ignored() {
    let core = ::support::local_executor::Core::new();

    let (tx0, rx0) = oneshot::channel::<u32>();
    let f1 = rx0.shared();
    let f2 = f1.clone();

    let (tx1, rx1) = oneshot::channel::<u32>();
    let (tx2, rx2) = oneshot::channel::<u32>();
    let (tx3, rx3) = oneshot::channel::<u32>();

    core.spawn(f1.map(|n| tx3.send(*n).unwrap()).map_err(|_|()));

    core.run(future::ok::<(),()>(())).unwrap(); // Allow f1 to be polled.

    core.spawn(f2.map_err(|_| ()).map(|x| *x).select(rx2.map_err(|_| ())).map_err(|_| ())
        .and_then(|(_, f2)| rx3.map_err(|_| ()).map(move |n| {drop(f2); tx1.send(n).unwrap()})));

    core.run(future::ok::<(),()>(())).unwrap();  // Allow f2 to be polled.

    tx2.send(11).unwrap(); // Resolve rx2, causing f2 to no longer get polled.

    core.run(future::ok::<(),()>(())).unwrap(); // Let the send() propagate.

    tx0.send(42).unwrap(); // Should cause f1, then rx3, and then rx1 to resolve.

    assert_eq!(core.run(rx1).unwrap(), 42);
}

#[test]
fn recursive_poll() {
    use futures::sync::mpsc;
    use futures::Stream;

    let core = ::support::local_executor::Core::new();
    let (tx0, rx0) = mpsc::unbounded::<Box<Future<Item=(),Error=()>>>();
    let run_stream = rx0.for_each(|f| f);

    let (tx1, rx1) = oneshot::channel::<()>();

    let f1 = run_stream.shared();
    let f2 = f1.clone();
    let f3 = f1.clone();
    tx0.unbounded_send(Box::new(
        f1.map(|_|()).map_err(|_|())
            .select(rx1.map_err(|_|()))
            .map(|_| ()).map_err(|_|()))).unwrap();

    core.spawn(f2.map(|_|()).map_err(|_|()));

    // Call poll() on the spawned future. We want to be sure that this does not trigger a
    // deadlock or panic due to a recursive lock() on a mutex.
    core.run(future::ok::<(),()>(())).unwrap();

    tx1.send(()).unwrap(); // Break the cycle.
    drop(tx0);
    core.run(f3).unwrap();
}

#[test]
fn recursive_poll_with_unpark() {
    use futures::sync::mpsc;
    use futures::{Stream, task};

    let core = ::support::local_executor::Core::new();
    let (tx0, rx0) = mpsc::unbounded::<Box<Future<Item=(),Error=()>>>();
    let run_stream = rx0.for_each(|f| f);

    let (tx1, rx1) = oneshot::channel::<()>();

    let f1 = run_stream.shared();
    let f2 = f1.clone();
    let f3 = f1.clone();
    tx0.unbounded_send(Box::new(future::lazy(move || {
        task::current().notify();
        f1.map(|_|()).map_err(|_|())
            .select(rx1.map_err(|_|()))
            .map(|_| ()).map_err(|_|())
    }))).unwrap();

    core.spawn(f2.map(|_|()).map_err(|_|()));

    // Call poll() on the spawned future. We want to be sure that this does not trigger a
    // deadlock or panic due to a recursive lock() on a mutex.
    core.run(future::ok::<(),()>(())).unwrap();

    tx1.send(()).unwrap(); // Break the cycle.
    drop(tx0);
    core.run(f3).unwrap();
}

#[test]
fn shared_future_that_wakes_itself_until_pending_is_returned() {
    use futures::Async;
    use std::cell::Cell;

    let core = ::support::local_executor::Core::new();

    let proceed = Cell::new(false);
    let fut = futures::future::poll_fn(|| {
        Ok::<_, ()>(if proceed.get() {
            Async::Ready(())
        } else {
            futures::task::current().notify();
            Async::NotReady
        })
    })
    .shared()
    .map(|_| ())
    .map_err(|_| ());

    // The join future can only complete if the second future gets a chance to run after the first
    // has returned pending
    let second = futures::future::lazy(|| {
        proceed.set(true);
        Ok::<_, ()>(())
    });

    core.run(fut.join(second)).unwrap();
}
