use futures::future::{FusedFuture, Future};
use futures::task::Context;
use futures_intrusive::sync::LocalManualResetEvent;
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

macro_rules! gen_event_tests {
    ($mod_name:ident, $event_type:ident) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn synchronous() {
                let event = $event_type::new(false);

                assert!(!event.is_set());
                event.set();
                assert!(event.is_set());
                event.reset();
                assert!(!event.is_set());
            }

            #[test]
            fn immediately_ready_event() {
                let event = $event_type::new(true);
                let waker = &panic_waker();
                let cx = &mut Context::from_waker(&waker);

                assert!(event.is_set());

                let poll = event.wait();
                pin_mut!(poll);
                assert!(!poll.as_mut().is_terminated());

                assert!(poll.as_mut().poll(cx).is_ready());
                assert!(poll.as_mut().is_terminated());
            }

            #[test]
            fn cancel_mid_wait() {
                let event = $event_type::new(false);
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                {
                    // Cancel a wait in between other waits
                    // In order to arbitrarily drop a non movable future we have to box and pin it
                    let mut poll1 = Box::pin(event.wait());
                    let mut poll2 = Box::pin(event.wait());
                    let mut poll3 = Box::pin(event.wait());
                    let mut poll4 = Box::pin(event.wait());
                    let mut poll5 = Box::pin(event.wait());

                    assert!(poll1.as_mut().poll(cx).is_pending());
                    assert!(poll2.as_mut().poll(cx).is_pending());
                    assert!(poll3.as_mut().poll(cx).is_pending());
                    assert!(poll4.as_mut().poll(cx).is_pending());
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(!poll1.is_terminated());
                    assert!(!poll2.is_terminated());
                    assert!(!poll3.is_terminated());
                    assert!(!poll4.is_terminated());
                    assert!(!poll5.is_terminated());

                    // Cancel 2 futures. Only the remaining ones should get completed
                    drop(poll2);
                    drop(poll4);

                    assert!(poll1.as_mut().poll(cx).is_pending());
                    assert!(poll3.as_mut().poll(cx).is_pending());
                    assert!(poll5.as_mut().poll(cx).is_pending());

                    assert_eq!(count, 0);
                    event.set();

                    assert!(poll1.as_mut().poll(cx).is_ready());
                    assert!(poll3.as_mut().poll(cx).is_ready());
                    assert!(poll5.as_mut().poll(cx).is_ready());
                    assert!(poll1.is_terminated());
                    assert!(poll3.is_terminated());
                    assert!(poll5.is_terminated());
                }

                assert_eq!(count, 3);
            }

            #[test]
            fn cancel_end_wait() {
                let event = $event_type::new(false);
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let poll1 = event.wait();
                let poll2 = event.wait();
                let poll3 = event.wait();
                let poll4 = event.wait();

                pin_mut!(poll1);
                pin_mut!(poll2);
                pin_mut!(poll3);
                pin_mut!(poll4);

                assert!(poll1.as_mut().poll(cx).is_pending());
                assert!(poll2.as_mut().poll(cx).is_pending());

                // Start polling some wait handles which get cancelled
                // before new ones are attached
                {
                    let poll5 = event.wait();
                    let poll6 = event.wait();
                    pin_mut!(poll5);
                    pin_mut!(poll6);
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(poll6.as_mut().poll(cx).is_pending());
                }

                assert!(poll3.as_mut().poll(cx).is_pending());
                assert!(poll4.as_mut().poll(cx).is_pending());

                event.set();

                assert!(poll1.as_mut().poll(cx).is_ready());
                assert!(poll2.as_mut().poll(cx).is_ready());
                assert!(poll3.as_mut().poll(cx).is_ready());
                assert!(poll4.as_mut().poll(cx).is_ready());

                assert_eq!(count, 4);
            }

            #[test]
            fn poll_from_multiple_executors() {
                let (waker_1, count_1) = new_count_waker();
                let (waker_2, count_2) = new_count_waker();
                let event = $event_type::new(false);

                let cx_1 = &mut Context::from_waker(&waker_1);
                let cx_2 = &mut Context::from_waker(&waker_2);

                let fut = event.wait();
                pin_mut!(fut);

                assert!(fut.as_mut().poll(cx_1).is_pending());
                assert!(fut.as_mut().poll(cx_2).is_pending());

                event.set();
                assert!(event.is_set());
                assert_eq!(count_1, 0);
                assert_eq!(count_2, 1);

                assert!(fut.as_mut().poll(cx_2).is_ready());
                assert!(fut.as_mut().is_terminated());
            }
        }
    };
}

gen_event_tests!(local_manual_reset_event_tests, LocalManualResetEvent);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures::executor::block_on;
    use futures_intrusive::sync::ManualResetEvent;
    use std::sync::Arc;
    use std::thread;
    use std::time;

    gen_event_tests!(manual_reset_event_tests, ManualResetEvent);

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn event_futures_are_send() {
        let event = ManualResetEvent::new(false);
        is_sync(&event);
        {
            let wait_fut = event.wait();
            is_send(&wait_fut);
            pin_mut!(wait_fut);
            is_send(&wait_fut);
        }
        is_send_value(event);
    }

    #[test]
    fn multithreaded_smoke() {
        let event = Arc::new(ManualResetEvent::new(false));

        let waiters: Vec<thread::JoinHandle<time::Instant>> = [1..4]
            .iter()
            .map(|_| {
                let ev = event.clone();
                thread::spawn(move || {
                    block_on(ev.wait());
                    time::Instant::now()
                })
            })
            .collect();

        let start = time::Instant::now();

        thread::sleep(time::Duration::from_millis(100));
        event.set();

        for waiter in waiters.into_iter() {
            let end_time = waiter.join().unwrap();
            let diff = end_time - start;
            assert!(diff > time::Duration::from_millis(50));
        }
    }
}
