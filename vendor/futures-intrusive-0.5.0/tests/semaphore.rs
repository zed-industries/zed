use futures::future::{FusedFuture, Future};
use futures::task::{Context, Poll};
use futures_intrusive::sync::LocalSemaphore;
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

macro_rules! gen_semaphore_tests {
    ($mod_name:ident, $semaphore_type:ident) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn uncontended_acquire() {
                for is_fair in &[true, false] {
                    let waker = &panic_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 2);
                    assert_eq!(2, sem.permits());

                    {
                        let sem_fut = sem.acquire(1);
                        pin_mut!(sem_fut);
                        match sem_fut.as_mut().poll(cx) {
                            Poll::Pending => panic!("Expect semaphore to get acquired"),
                            Poll::Ready(_guard) => {
                                assert_eq!(1, sem.permits());
                            },
                        };
                        assert!(sem_fut.as_mut().is_terminated());
                        assert_eq!(2, sem.permits());
                    }

                    assert_eq!(2, sem.permits());

                    {
                        let sem_fut = sem.acquire(2);
                        pin_mut!(sem_fut);
                        match sem_fut.as_mut().poll(cx) {
                            Poll::Pending => panic!("Expect semaphore to get acquired"),
                            Poll::Ready(_guard) => {
                                assert_eq!(0, sem.permits());
                            },
                        };
                        assert!(sem_fut.as_mut().is_terminated());
                    }

                    assert_eq!(2, sem.permits());
                }
            }

            #[test]
            fn manual_release_via_disarm() {
                for is_fair in &[true, false] {
                    let waker = &panic_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 2);
                    assert_eq!(2, sem.permits());

                    {
                        let sem_fut = sem.acquire(1);
                        pin_mut!(sem_fut);
                        match sem_fut.as_mut().poll(cx) {
                            Poll::Pending => panic!("Expect semaphore to get acquired"),
                            Poll::Ready(mut guard) => {
                                assert_eq!(1, sem.permits());
                                guard.disarm();
                            },
                        };
                        assert!(sem_fut.as_mut().is_terminated());
                        assert_eq!(1, sem.permits());
                    }

                    assert_eq!(1, sem.permits());

                    {
                        let sem_fut = sem.acquire(1);
                        pin_mut!(sem_fut);
                        match sem_fut.as_mut().poll(cx) {
                            Poll::Pending => panic!("Expect semaphore to get acquired"),
                            Poll::Ready(mut guard) => {
                                assert_eq!(0, sem.permits());
                                guard.disarm();
                            },
                        };
                        assert!(sem_fut.as_mut().is_terminated());
                    }

                    assert_eq!(0, sem.permits());

                    sem.release(2);
                    assert_eq!(2, sem.permits());
                }
            }

            #[test]
            #[should_panic]
            fn poll_after_completion_should_panic() {
                for is_fair in &[true, false] {
                    let waker = &panic_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 2);

                    let sem_fut = sem.acquire(2);
                    pin_mut!(sem_fut);
                    match sem_fut.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired"),
                        Poll::Ready(guard) => guard,
                    };
                    assert!(sem_fut.as_mut().is_terminated());

                    let _ = sem_fut.poll(cx);
                }
            }

            #[test]
            fn contended_acquire() {
                for is_fair in &[false, true] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 3);

                    let sem_fut1 = sem.acquire(3);
                    pin_mut!(sem_fut1);

                    // Acquire the semaphore
                    let guard1 = match sem_fut1.poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 1"),
                        Poll::Ready(guard) => guard
                    };

                    // The next acquire attempts must fail
                    let sem_fut2 = sem.acquire(1);
                    pin_mut!(sem_fut2);
                    assert!(sem_fut2.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut2.as_mut().is_terminated());
                    let sem_fut3 = sem.acquire(2);
                    pin_mut!(sem_fut3);
                    assert!(sem_fut3.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut3.as_mut().is_terminated());
                    let sem_fut4 = sem.acquire(2);
                    pin_mut!(sem_fut4);
                    assert!(sem_fut4.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut4.as_mut().is_terminated());
                    assert_eq!(count, 0);

                    // Release - semaphore should be available again and allow
                    // fut2 and fut3 to complete
                    assert_eq!(0, sem.permits());
                    drop(guard1);
                    assert_eq!(3, sem.permits());
                    // At least one task should be awoken.
                    if *is_fair {
                        assert_eq!(count, 1);
                    }
                    else {
                        assert_eq!(count, 2);
                    }

                    let guard2 = match sem_fut2.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 2"),
                        Poll::Ready(guard) => guard
                    };
                    assert!(sem_fut2.as_mut().is_terminated());
                    assert_eq!(2, sem.permits());
                    // In the fair case, the next task should be woken up here
                    assert_eq!(count, 2);

                    let guard3 = match sem_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 3"),
                        Poll::Ready(guard) => guard
                    };
                    assert!(sem_fut3.as_mut().is_terminated());
                    assert_eq!(0, sem.permits());

                    assert!(sem_fut4.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut4.as_mut().is_terminated());

                    // Release - some permits should be available again
                    drop(guard2);
                    assert_eq!(1, sem.permits());
                    assert_eq!(count, 2);

                    assert!(sem_fut4.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut4.as_mut().is_terminated());

                    // After releasing the permits from fut3, there should be
                    // enough permits for fut4 getting woken.
                    drop(guard3);
                    assert_eq!(3, sem.permits());
                    assert_eq!(count, 3);

                    let guard4 = match sem_fut4.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 4"),
                        Poll::Ready(guard) => guard
                    };
                    assert!(sem_fut4.as_mut().is_terminated());

                    drop(guard4);
                    assert_eq!(3, sem.permits());
                    assert_eq!(count, 3);
                }
            }

            #[test]
            fn acquire_synchronously() {
                for is_fair in &[true] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 3);

                    let sem_fut1 = sem.acquire(3);
                    pin_mut!(sem_fut1);

                    // Acquire the semaphore
                    let guard1 = match sem_fut1.poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 1"),
                        Poll::Ready(guard) => guard
                    };

                    // Some failing acquire attempts
                    assert!(sem.try_acquire(1).is_none());

                    // Add an async waiter
                    let mut sem_fut2 = Box::pin(sem.acquire(1));
                    assert!(sem_fut2.as_mut().poll(cx).is_pending());
                    assert_eq!(count, 0);

                    // Release - semaphore should be available again
                    drop(guard1);
                    assert_eq!(3, sem.permits());

                    // In the fair case we shouldn't be able to obtain the
                    // semaphore asynchronously. In the unfair case it should
                    // be possible.
                    if *is_fair {
                        assert!(sem.try_acquire(1).is_none());

                        // Cancel async acquire attempt
                        drop(sem_fut2);
                        // Now the semaphore should be acquireable
                    }

                    let guard = sem.try_acquire(1).unwrap();
                    assert_eq!(2, sem.permits());
                    let mut guard2 = sem.try_acquire(2).unwrap();
                    assert_eq!(0, sem.permits());
                    guard2.disarm();
                    sem.release(2);
                    drop(guard);
                }
            }

            #[test]
            fn acquire_0_permits_without_other_waiters() {
                for is_fair in &[false, true] {
                    let (waker, _count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 3);

                    // Acquire the semaphore
                    let guard1 = sem.try_acquire(3).unwrap();
                    assert_eq!(0, sem.permits());

                    let sem_fut2 = sem.acquire(0);
                    pin_mut!(sem_fut2);
                    let guard2 = match sem_fut2.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 2"),
                        Poll::Ready(guard) => guard
                    };

                    drop(guard2);
                    assert_eq!(0, sem.permits());
                    drop(guard1);
                    assert_eq!(3, sem.permits());
                }
            }

            #[test]
            fn acquire_0_permits_with_other_waiters() {
                for is_fair in &[false, true] {
                    let (waker, _count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 3);

                    // Acquire the semaphore
                    let guard1 = sem.try_acquire(3).unwrap();

                    assert_eq!(0, sem.permits());

                    let sem_fut2 = sem.acquire(1);
                    pin_mut!(sem_fut2);
                    assert!(sem_fut2.as_mut().poll(cx).is_pending());

                    let sem_fut3 = sem.acquire(0);
                    pin_mut!(sem_fut3);
                    let guard3 = match sem_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 3"),
                        Poll::Ready(guard) => guard
                    };

                    drop(guard3);
                    assert_eq!(0, sem.permits());
                    drop(guard1);
                    assert_eq!(3, sem.permits());

                    let guard2 = match sem_fut2.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired 2"),
                        Poll::Ready(guard) => guard
                    };
                    assert_eq!(2, sem.permits());
                    drop(guard2);
                }
            }

            #[test]
            fn cancel_wait_for_semaphore() {
                for is_fair in &[true, false] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 5);

                    // Acquire the semaphore
                    let guard1 = sem.try_acquire(5).unwrap();

                    // The second and third lock attempt must fail
                    let mut sem_fut2 = Box::pin(sem.acquire(1));
                    let mut sem_fut3 = Box::pin(sem.acquire(1));

                    assert!(sem_fut2.as_mut().poll(cx).is_pending());
                    assert!(sem_fut3.as_mut().poll(cx).is_pending());

                    // Before the semaphore gets available, cancel one acquire attempt
                    drop(sem_fut2);

                    // Unlock - semaphore should be available again.
                    // fut2 should have been notified
                    drop(guard1);
                    assert_eq!(count, 1);

                    // Unlock - semaphore should be available again
                    match sem_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired"),
                        Poll::Ready(guard) => guard
                    };
                }
            }

            #[test]
            fn unlock_next_when_notification_is_not_used() {
                for is_fair in &[true, false] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let sem = $semaphore_type::new(*is_fair, 2);

                    let guard1 = sem.try_acquire(2).unwrap();

                    // The second and third acquire attempt must fail
                    let mut sem_fut2 = Box::pin(sem.acquire(1));
                    let mut sem_fut3 = Box::pin(sem.acquire(1));

                    assert!(sem_fut2.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut2.as_mut().is_terminated());
                    assert!(sem_fut3.as_mut().poll(cx).is_pending());
                    assert!(!sem_fut3.as_mut().is_terminated());
                    assert_eq!(count, 0);

                    // Release - semaphore should be available again. fut2 should have been notified
                    drop(guard1);
                    if *is_fair {
                        assert_eq!(count, 1);
                    }
                    else {
                        assert_eq!(count, 2);
                    }

                    // We don't use the notification. Expect the next waiting task to be woken up
                    drop(sem_fut2);
                    assert_eq!(count, 2);

                    match sem_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect semaphore to get acquired"),
                        Poll::Ready(guard) => guard
                    };
                }
            }

            #[test]
            fn new_waiters_on_unfair_semaphore_can_acquire_future_while_one_task_is_notified() {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let sem = $semaphore_type::new(false, 3);

                // Acquire the semaphore
                let guard1 = sem.try_acquire(3).unwrap();

                // The second and third acquire attempt must fail
                let mut sem_fut2 = Box::pin(sem.acquire(3));
                let mut sem_fut3 = Box::pin(sem.acquire(3));

                assert!(sem_fut2.as_mut().poll(cx).is_pending());

                // Release - Semaphore should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Acquire fut3 in between. This should succeed
                let guard3 = match sem_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(guard) => guard
                };
                // Now fut2 can't use it's notification and is still pending
                assert!(sem_fut2.as_mut().poll(cx).is_pending());

                // When we drop fut3, the semaphore should signal that it's available for fut2,
                // which needs to have re-registered
                drop(guard3);
                assert_eq!(count, 2);
                match sem_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(_guard) => {},
                };
            }

            #[test]
            fn waiters_on_unfair_semaphore_can_acquire_future_through_repolling_if_one_task_is_notified() {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let sem = $semaphore_type::new(false, 3);

                // Acquire the semaphore
                let guard1 = sem.try_acquire(3).unwrap();

                // The second and third acquire attempt must fail
                let mut sem_fut2 = Box::pin(sem.acquire(3));
                let mut sem_fut3 = Box::pin(sem.acquire(3));
                // Start polling both futures, which means both are waiters
                assert!(sem_fut2.as_mut().poll(cx).is_pending());
                assert!(sem_fut3.as_mut().poll(cx).is_pending());

                // Release - semaphore should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Acquire fut3 in between. This should succeed
                let guard3 = match sem_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(guard) => guard
                };
                // Now fut2 can't use it's notification and is still pending
                assert!(sem_fut2.as_mut().poll(cx).is_pending());

                // When we drop fut3, the mutex should signal that it's available for fut2,
                // which needs to have re-registered
                drop(guard3);
                assert_eq!(count, 2);
                match sem_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(_guard) => {},
                };
            }

            #[test]
            fn new_waiters_on_fair_semaphore_cant_acquire_future_while_one_task_is_notified() {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let sem = $semaphore_type::new(true, 3);

                // Acquire the semaphore
                let guard1 = sem.try_acquire(3).unwrap();

                // The second and third acquire attempt must fail
                let mut sem_fut2 = Box::pin(sem.acquire(3));
                let mut sem_fut3 = Box::pin(sem.acquire(3));

                assert!(sem_fut2.as_mut().poll(cx).is_pending());

                // Release - semaphore should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Try to acquire fut3 in between. This should fail
                assert!(sem_fut3.as_mut().poll(cx).is_pending());

                // fut2 should be be able to get acquired
                match sem_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(_guard) => {},
                };

                // Now fut3 should have been signaled and should be able to get acquired
                assert_eq!(count, 2);
                match sem_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(_guard) => {},
                };
            }

            #[test]
            fn waiters_on_fair_semaphore_cant_acquire_future_through_repolling_if_one_task_is_notified() {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let sem = $semaphore_type::new(true, 3);

                // Acquire the semaphore
                let guard1 = sem.try_acquire(3).unwrap();

                // The second and third acquire attempt must fail
                let mut sem_fut2 = Box::pin(sem.acquire(3));
                let mut sem_fut3 = Box::pin(sem.acquire(3));

                assert!(sem_fut2.as_mut().poll(cx).is_pending());
                assert!(sem_fut3.as_mut().poll(cx).is_pending());

                // Release - semaphore should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Acquire fut3 in between. This should fail, since fut2 should get the permits first
                assert!(sem_fut3.as_mut().poll(cx).is_pending());

                // fut2 should be acquired
                match sem_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(_guard) => {},
                };

                // Now fut3 should be able to get acquired
                assert_eq!(count, 2);

                match sem_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect semaphore to get acquired"),
                    Poll::Ready(_guard) => {},
                };
            }

            #[test]
            fn poll_from_multiple_executors() {
                for is_fair in &[true, false] {
                    let (waker_1, count_1) = new_count_waker();
                    let (waker_2, count_2) = new_count_waker();
                    let sem = $semaphore_type::new(*is_fair, 3);

                    // Acquire the semaphore
                    let guard = sem.try_acquire(3).unwrap();

                    let fut = sem.acquire(1);
                    pin_mut!(fut);

                    let cx_1 = &mut Context::from_waker(&waker_1);
                    let cx_2 = &mut Context::from_waker(&waker_2);
                    assert!(fut.as_mut().poll(cx_1).is_pending());
                    assert!(fut.as_mut().poll(cx_2).is_pending());

                    drop(guard);
                    assert_eq!(count_1, 0);
                    assert_eq!(count_2, 1);

                    assert!(fut.as_mut().poll(cx_2).is_ready());
                    assert!(fut.as_mut().is_terminated());
                }
            }
        }
    }
}

gen_semaphore_tests!(local_semaphore_tests, LocalSemaphore);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures::FutureExt;
    use futures_intrusive::sync::{Semaphore, SharedSemaphore};

    gen_semaphore_tests!(semaphore_tests, Semaphore);
    gen_semaphore_tests!(shared_semaphore_tests, SharedSemaphore);

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn semaphore_futures_are_send() {
        let sem = Semaphore::new(true, 3);
        is_sync(&sem);
        {
            let wait_fut = sem.acquire(3);
            is_send(&wait_fut);
            pin_mut!(wait_fut);
            is_send(&wait_fut);

            let waker = &panic_waker();
            let cx = &mut Context::from_waker(&waker);
            pin_mut!(wait_fut);
            let res = wait_fut.poll_unpin(cx);
            let releaser = match res {
                Poll::Ready(v) => v,
                Poll::Pending => panic!("Expected to be ready"),
            };
            is_send(&releaser);
            is_send_value(releaser);
        }
        is_send_value(sem);
    }
}
