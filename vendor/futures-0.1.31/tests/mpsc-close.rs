extern crate futures;

use std::sync::{Arc, Weak};
use std::thread;
use std::time::{Duration, Instant};

use futures::prelude::*;
use futures::sync::mpsc::*;
use futures::task;

#[test]
fn smoke() {
    let (mut sender, receiver) = channel(1);

    let t = thread::spawn(move ||{
        while let Ok(s) = sender.send(42).wait() {
            sender = s;
        }
    });

    receiver.take(3).for_each(|_| Ok(())).wait().unwrap();

    t.join().unwrap()
}

// Stress test that `try_send()`s occurring concurrently with receiver
// close/drops don't appear as successful sends.
#[test]
fn stress_try_send_as_receiver_closes() {
    const AMT: usize = 10000;
    // To provide variable timing characteristics (in the hopes of
    // reproducing the collision that leads to a race), we busy-re-poll
    // the test MPSC receiver a variable number of times before actually
    // stopping.  We vary this countdown between 1 and the following
    // value.
    const MAX_COUNTDOWN: usize = 20;
    // When we detect that a successfully sent item is still in the
    // queue after a disconnect, we spin for up to 100ms to confirm that
    // it is a persistent condition and not a concurrency illusion.
    const SPIN_TIMEOUT_S: u64 = 10;
    const SPIN_SLEEP_MS: u64 = 10;
    struct TestRx {
        rx: Receiver<Arc<()>>,
        // The number of times to query `rx` before dropping it.
        poll_count: usize
    }
    struct TestTask {
        command_rx: Receiver<TestRx>,
        test_rx: Option<Receiver<Arc<()>>>,
        countdown: usize,
    }
    impl TestTask {
        /// Create a new TestTask
        fn new() -> (TestTask, Sender<TestRx>) {
            let (command_tx, command_rx) = channel::<TestRx>(0);
            (
                TestTask {
                    command_rx: command_rx,
                    test_rx: None,
                    countdown: 0, // 0 means no countdown is in progress.
                },
                command_tx,
            )
        }
    }
    impl Future for TestTask {
        type Item = ();
        type Error = ();
        fn poll(&mut self) -> Poll<(), ()> {
            // Poll the test channel, if one is present.
            if let Some(ref mut rx) = self.test_rx {
                if let Ok(Async::Ready(v)) = rx.poll() {
                   let _ = v.expect("test finished unexpectedly!");
                }
                self.countdown -= 1;
                // Busy-poll until the countdown is finished.
                task::current().notify();
            }
            // Accept any newly submitted MPSC channels for testing.
            match self.command_rx.poll()? {
                Async::Ready(Some(TestRx { rx, poll_count })) => {
                    self.test_rx = Some(rx);
                    self.countdown = poll_count;
                    task::current().notify();
                },
                Async::Ready(None) => return Ok(Async::Ready(())),
                _ => {},
            }
            if self.countdown == 0 {
                // Countdown complete -- drop the Receiver.
                self.test_rx = None;
            }
            Ok(Async::NotReady)
        }
    }
    let (f, mut cmd_tx) = TestTask::new();
    let bg = thread::spawn(move || f.wait());
    for i in 0..AMT {
        let (mut test_tx, rx) = channel(0);
        let poll_count = i % MAX_COUNTDOWN;
        cmd_tx.try_send(TestRx { rx: rx, poll_count: poll_count }).unwrap();
        let mut prev_weak: Option<Weak<()>> = None;
        let mut attempted_sends = 0;
        let mut successful_sends = 0;
        loop {
            // Create a test item.
            let item = Arc::new(());
            let weak = Arc::downgrade(&item);
            match test_tx.try_send(item) {
                Ok(_) => {
                    prev_weak = Some(weak);
                    successful_sends += 1;
                }
                Err(ref e) if e.is_full() => {}
                Err(ref e) if e.is_disconnected() => {
                    // Test for evidence of the race condition.
                    if let Some(prev_weak) = prev_weak {
                        if prev_weak.upgrade().is_some() {
                            // The previously sent item is still allocated.
                            // However, there appears to be some aspect of the
                            // concurrency that can legitimately cause the Arc
                            // to be momentarily valid.  Spin for up to 100ms
                            // waiting for the previously sent item to be
                            // dropped.
                            let t0 = Instant::now();
                            let mut spins = 0;
                            loop {
                                if prev_weak.upgrade().is_none() {
                                    break;
                                }
                                assert!(t0.elapsed() < Duration::from_secs(SPIN_TIMEOUT_S),
                                    "item not dropped on iteration {} after \
                                     {} sends ({} successful). spin=({})",
                                    i, attempted_sends, successful_sends, spins
                                );
                                spins += 1;
                                thread::sleep(Duration::from_millis(SPIN_SLEEP_MS));
                            }
                        }
                    }
                    break;
                }
                Err(ref e) => panic!("unexpected error: {}", e),
            }
            attempted_sends += 1;
        }
    }
    drop(cmd_tx);
    bg.join()
        .expect("background thread join")
        .expect("background thread result");
}
