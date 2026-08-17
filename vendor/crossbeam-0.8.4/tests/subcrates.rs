//! Makes sure subcrates are properly re-exported.

use crossbeam::select;

#[test]
fn channel() {
    let (s, r) = crossbeam::channel::bounded(1);

    select! {
        send(s, 0) -> res => res.unwrap(),
        recv(r) -> res => assert!(res.is_ok()),
    }
}

#[test]
fn deque() {
    let w = crossbeam::deque::Worker::new_fifo();
    w.push(1);
    let _ = w.pop();
}

#[test]
#[cfg_attr(miri, ignore)] // Miri ICE: https://github.com/crossbeam-rs/crossbeam/pull/870#issuecomment-1189209073
fn epoch() {
    crossbeam::epoch::pin();
}

#[test]
fn queue() {
    let a = crossbeam::queue::ArrayQueue::new(10);
    let _ = a.push(1);
    let _ = a.pop();
}

#[test]
fn utils() {
    crossbeam::utils::CachePadded::new(7);

    crossbeam::scope(|scope| {
        scope.spawn(|_| ());
    })
    .unwrap();

    crossbeam::thread::scope(|scope| {
        scope.spawn(|_| ());
    })
    .unwrap();
}
