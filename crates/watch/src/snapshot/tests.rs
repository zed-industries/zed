use super::*;
use futures::task::{ArcWake, waker};
use std::{
    future::Future,
    sync::{
        Arc, Barrier,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

#[derive(Default)]
struct WakeCounter(AtomicUsize);

impl ArcWake for WakeCounter {
    fn wake_by_ref(counter: &Arc<Self>) {
        counter.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn counting_waker() -> (Waker, Arc<WakeCounter>) {
    let counter = Arc::new(WakeCounter::default());
    (waker(counter.clone()), counter)
}

fn poll_changed<T>(receiver: &mut Receiver<T>, waker: &Waker) -> Poll<Result<(), NoSenderError>> {
    let mut changed = Box::pin(receiver.changed());
    changed.as_mut().poll(&mut Context::from_waker(waker))
}

#[test]
fn coalesces_publications_to_the_latest_snapshot() {
    let (mut sender, mut receiver) = channel(0);
    let (waker, _) = counting_waker();

    assert_eq!(sender.send(1), Ok(()));
    assert_eq!(sender.send(2), Ok(()));
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Ready(Ok(())));
    assert_eq!(*receiver.snapshot(), 2);
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Pending);
}

#[test]
fn equal_values_are_distinct_publications() {
    let (mut sender, mut receiver) = channel(7);
    let (waker, _) = counting_waker();

    assert_eq!(sender.send(7), Ok(()));
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Ready(Ok(())));
    assert_eq!(sender.send(7), Ok(()));
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Ready(Ok(())));
}

#[test]
fn initial_value_is_seen() {
    let (_sender, mut receiver) = channel(3);
    let (waker, _) = counting_waker();

    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Pending);
    assert_eq!(*receiver.snapshot(), 3);
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Pending);
}

#[test]
fn snapshot_marks_the_latest_publication_as_seen() {
    let (mut sender, mut receiver) = channel(0);
    let (waker, _) = counting_waker();
    assert_eq!(sender.send(1), Ok(()));
    assert_eq!(*receiver.snapshot(), 1);
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Pending);
    drop(sender);
    assert_eq!(
        poll_changed(&mut receiver, &waker),
        Poll::Ready(Err(NoSenderError))
    );
}

#[test]
fn clone_copies_seen_version_but_fresh_receiver_sees_current_version() {
    let (mut sender, mut receiver) = channel(0);
    assert_eq!(sender.send(1), Ok(()));
    let mut clone = receiver.clone();
    let mut fresh = sender.receiver();
    let (waker, _) = counting_waker();

    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Ready(Ok(())));
    assert_eq!(poll_changed(&mut clone, &waker), Poll::Ready(Ok(())));
    assert_eq!(poll_changed(&mut fresh, &waker), Poll::Pending);
}

#[test]
fn retained_snapshots_do_not_require_clone_or_block_publication() {
    #[derive(Debug, Eq, PartialEq)]
    struct NotClone(usize);

    let (mut sender, mut receiver) = channel(NotClone(1));
    let retained = receiver.snapshot();
    assert_eq!(sender.send(NotClone(2)), Ok(()));
    let latest = receiver.snapshot();
    drop(sender);
    drop(receiver);

    assert_eq!(*retained, NotClone(1));
    assert_eq!(*latest, NotClone(2));
}

#[test]
fn send_without_receivers_still_stores_the_value() {
    let (mut sender, receiver) = channel(1);
    drop(receiver);

    assert_eq!(sender.send(2), Err(NoReceiverError));
    let mut receiver = sender.receiver();
    let (waker, _) = counting_waker();
    assert_eq!(*receiver.snapshot(), 2);
    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Pending);
}

#[test]
fn final_publication_precedes_channel_closure() {
    let (mut sender, mut receiver) = channel(0);
    let (waker, _) = counting_waker();
    assert_eq!(sender.send(1), Ok(()));
    drop(sender);

    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Ready(Ok(())));
    assert_eq!(*receiver.snapshot(), 1);
    assert_eq!(
        poll_changed(&mut receiver, &waker),
        Poll::Ready(Err(NoSenderError))
    );
}

#[test]
fn sender_drop_wakes_a_pending_change() {
    let (sender, mut receiver) = channel(0);
    let (waker, counter) = counting_waker();
    let mut changed = Box::pin(receiver.changed());
    assert_eq!(
        changed.as_mut().poll(&mut Context::from_waker(&waker)),
        Poll::Pending
    );

    drop(sender);

    assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    assert_eq!(
        changed.as_mut().poll(&mut Context::from_waker(&waker)),
        Poll::Ready(Err(NoSenderError))
    );
}

#[test]
fn constant_receiver_never_changes() {
    #[derive(Debug, PartialEq)]
    struct NotClone;

    let mut receiver = Receiver::constant(NotClone);
    let (waker, counter) = counting_waker();

    assert_eq!(poll_changed(&mut receiver, &waker), Poll::Pending);
    assert_eq!(*receiver.snapshot(), NotClone);
    assert_eq!(counter.0.load(Ordering::SeqCst), 0);
}

#[test]
fn dropping_changed_unregisters_its_waker_and_repoll_replaces_it() {
    let (mut sender, mut receiver) = channel(0);
    let (first_waker, first_counter) = counting_waker();
    let (second_waker, second_counter) = counting_waker();

    {
        let mut changed = Box::pin(receiver.changed());
        assert_eq!(
            changed
                .as_mut()
                .poll(&mut Context::from_waker(&first_waker)),
            Poll::Pending
        );
        assert_eq!(
            changed
                .as_mut()
                .poll(&mut Context::from_waker(&second_waker)),
            Poll::Pending
        );
        assert_eq!(sender.send(1), Ok(()));
        assert_eq!(first_counter.0.load(Ordering::SeqCst), 0);
        assert_eq!(second_counter.0.load(Ordering::SeqCst), 1);
    }

    assert_eq!(
        poll_changed(&mut receiver, &second_waker),
        Poll::Ready(Ok(()))
    );
    let (cancelled_waker, cancelled_counter) = counting_waker();
    {
        let mut changed = Box::pin(receiver.changed());
        assert_eq!(
            changed
                .as_mut()
                .poll(&mut Context::from_waker(&cancelled_waker)),
            Poll::Pending
        );
    }
    assert_eq!(sender.send(2), Ok(()));
    assert_eq!(cancelled_counter.0.load(Ordering::SeqCst), 0);
}

#[test]
fn publication_wakes_all_registered_receivers() {
    let (mut sender, mut first_receiver) = channel(0);
    let mut second_receiver = first_receiver.clone();
    let mut third_receiver = first_receiver.clone();
    let (first_waker, first_counter) = counting_waker();
    let (second_waker, second_counter) = counting_waker();
    let (third_waker, third_counter) = counting_waker();
    let mut first_changed = Box::pin(first_receiver.changed());
    let mut second_changed = Box::pin(second_receiver.changed());
    let mut third_changed = Box::pin(third_receiver.changed());

    assert_eq!(
        first_changed
            .as_mut()
            .poll(&mut Context::from_waker(&first_waker)),
        Poll::Pending
    );
    assert_eq!(
        second_changed
            .as_mut()
            .poll(&mut Context::from_waker(&second_waker)),
        Poll::Pending
    );
    assert_eq!(
        third_changed
            .as_mut()
            .poll(&mut Context::from_waker(&third_waker)),
        Poll::Pending
    );

    assert_eq!(sender.send(1), Ok(()));
    assert_eq!(first_counter.0.load(Ordering::SeqCst), 1);
    assert_eq!(second_counter.0.load(Ordering::SeqCst), 1);
    assert_eq!(third_counter.0.load(Ordering::SeqCst), 1);
}

#[test]
fn receiver_churn_removes_waker_entries() {
    let (mut sender, receiver) = channel(0);

    for _ in 0..1_000 {
        drop(receiver.clone());
    }

    assert_eq!(sender.state.receivers.load().len(), 1);
    drop(receiver);
    assert!(sender.state.receivers.load().is_empty());
    assert_eq!(sender.send(1), Err(NoReceiverError));
}

#[test]
fn threaded_recv_delivers_each_acknowledged_publication_and_closure() {
    let (mut sender, mut receiver) = channel(0);
    let (acknowledgment_sender, acknowledgment_receiver) = mpsc::channel();
    let (completed_sender, completed_receiver) = mpsc::channel();
    thread::spawn(move || {
        futures::executor::block_on(async move {
            for expected in 1..=1_000 {
                assert_eq!(receiver.recv().await, Ok(expected));
                acknowledgment_sender
                    .send(())
                    .expect("publisher is waiting for acknowledgment");
            }
            assert_eq!(receiver.recv().await, Err(NoSenderError));
        });
        completed_sender.send(()).expect("test is waiting");
    });
    for value in 1..=1_000 {
        assert_eq!(sender.send(value), Ok(()));
        acknowledgment_receiver
            .recv_timeout(Duration::from_secs(10))
            .expect("receiver missed a publication");
    }
    drop(sender);
    completed_receiver
        .recv_timeout(Duration::from_secs(10))
        .expect("receiver missed channel closure");
}

#[test]
fn concurrent_send_poll_register_and_drop_completes_within_deadline() {
    const RECEIVER_COUNT: usize = 4;
    const ITERATIONS: usize = 2_000;

    let (mut sender, receiver) = channel(0);
    let receivers = (0..RECEIVER_COUNT)
        .map(|_| receiver.clone())
        .collect::<Vec<_>>();
    drop(receiver);
    let sender_guard = sender.receiver();

    let barrier = Arc::new(Barrier::new(RECEIVER_COUNT + 1));
    let (completed_sender, completed_receiver) = mpsc::channel();

    for mut receiver in receivers {
        let barrier = barrier.clone();
        let completed_sender = completed_sender.clone();
        thread::spawn(move || {
            let (waker, _) = counting_waker();
            barrier.wait();
            for iteration in 0..ITERATIONS {
                if poll_changed(&mut receiver, &waker) == Poll::Ready(Err(NoSenderError)) {
                    assert_eq!(*receiver.snapshot(), ITERATIONS);
                }
                if iteration % 3 == 0 {
                    let mut clone = receiver.clone();
                    if poll_changed(&mut clone, &waker) == Poll::Ready(Err(NoSenderError)) {
                        assert_eq!(*clone.snapshot(), ITERATIONS);
                    }
                }
            }
            completed_sender
                .send(())
                .expect("completion receiver exists");
        });
    }

    thread::spawn(move || {
        let _sender_guard = sender_guard;
        barrier.wait();
        for value in 1..=ITERATIONS {
            assert_eq!(sender.send(value), Ok(()));
        }
        completed_sender
            .send(())
            .expect("completion receiver exists");
    });

    for _ in 0..=RECEIVER_COUNT {
        completed_receiver
            .recv_timeout(Duration::from_secs(10))
            .expect("concurrent channel operations exceeded deadline");
    }
}
