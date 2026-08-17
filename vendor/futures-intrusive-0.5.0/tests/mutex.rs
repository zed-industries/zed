use futures::future::{FusedFuture, Future};
use futures::task::{Context, Poll};
use futures_intrusive::sync::LocalMutex;
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

macro_rules! gen_mutex_tests {
    ($mod_name:ident, $mutex_type:ident) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn uncontended_lock() {
                for is_fair in &[true, false] {
                    let waker = &panic_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let mtx = $mutex_type::new(5, *is_fair);
                    assert_eq!(false, mtx.is_locked());

                    {
                        let mutex_fut = mtx.lock();
                        pin_mut!(mutex_fut);
                        match mutex_fut.as_mut().poll(cx) {
                            Poll::Pending => panic!("Expect mutex to get locked"),
                            Poll::Ready(mut guard) => {
                                assert_eq!(true, mtx.is_locked());
                                assert_eq!(5, *guard);
                                *guard += 7;
                                assert_eq!(12, *guard);
                            }
                        };
                        assert!(mutex_fut.as_mut().is_terminated());
                    }

                    assert_eq!(false, mtx.is_locked());

                    {
                        let mutex_fut = mtx.lock();
                        pin_mut!(mutex_fut);
                        match mutex_fut.as_mut().poll(cx) {
                            Poll::Pending => panic!("Expect mutex to get locked"),
                            Poll::Ready(guard) => {
                                assert_eq!(true, mtx.is_locked());
                                assert_eq!(12, *guard);
                            }
                        };
                    }

                    assert_eq!(false, mtx.is_locked());
                }
            }

            #[test]
            #[should_panic]
            fn poll_after_completion_should_panic() {
                for is_fair in &[true, false] {
                    let waker = &panic_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let mtx = $mutex_type::new(5, *is_fair);

                    let mutex_fut = mtx.lock();
                    pin_mut!(mutex_fut);
                    let guard = match mutex_fut.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get locked"),
                        Poll::Ready(guard) => guard,
                    };
                    assert_eq!(5, *guard);
                    assert!(mutex_fut.as_mut().is_terminated());

                    let _ = mutex_fut.poll(cx);
                }
            }

            #[test]
            fn contended_lock() {
                for is_fair in &[true, false] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let mtx = $mutex_type::new(5, *is_fair);

                    let mutex_fut1 = mtx.lock();
                    pin_mut!(mutex_fut1);

                    // Lock the mutex
                    let mut guard1 = match mutex_fut1.poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get locked"),
                        Poll::Ready(guard) => guard,
                    };
                    *guard1 = 27;

                    // The second and third lock attempt must fail
                    let mutex_fut2 = mtx.lock();
                    pin_mut!(mutex_fut2);
                    assert!(mutex_fut2.as_mut().poll(cx).is_pending());
                    assert!(!mutex_fut2.as_mut().is_terminated());
                    let mutex_fut3 = mtx.lock();
                    pin_mut!(mutex_fut3);
                    assert!(mutex_fut3.as_mut().poll(cx).is_pending());
                    assert!(!mutex_fut3.as_mut().is_terminated());
                    assert_eq!(count, 0);

                    // Unlock - mutex should be available again
                    drop(guard1);
                    assert_eq!(count, 1);
                    let mut guard2 = match mutex_fut2.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get locked"),
                        Poll::Ready(guard) => guard,
                    };
                    assert_eq!(27, *guard2);
                    *guard2 = 72;
                    assert!(mutex_fut2.as_mut().is_terminated());
                    assert!(mutex_fut3.as_mut().poll(cx).is_pending());
                    assert!(!mutex_fut3.as_mut().is_terminated());
                    assert_eq!(count, 1);

                    // Unlock - mutex should be available again
                    drop(guard2);
                    assert_eq!(count, 2);
                    let guard3 = match mutex_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get locked"),
                        Poll::Ready(guard) => guard,
                    };
                    assert_eq!(72, *guard3);
                    assert!(mutex_fut3.as_mut().is_terminated());

                    drop(guard3);
                    assert_eq!(count, 2);
                }
            }

            #[test]
            fn lock_synchronously() {
                for is_fair in &[true] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let mtx = $mutex_type::new(5, *is_fair);

                    let mutex_fut1 = mtx.lock();
                    pin_mut!(mutex_fut1);

                    // Lock the mutex
                    let mut guard1 = match mutex_fut1.poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get acquired 1"),
                        Poll::Ready(guard) => guard,
                    };
                    *guard1 = 7;
                    assert_eq!(true, mtx.is_locked());

                    // Synchronous lock attempt fails
                    assert!(mtx.try_lock().is_none());

                    // Add an async waiter
                    let mut mutex_fut2 = Box::pin(mtx.lock());
                    assert!(mutex_fut2.as_mut().poll(cx).is_pending());
                    assert_eq!(count, 0);

                    // Release - mutex should be available again
                    drop(guard1);
                    assert_eq!(false, mtx.is_locked());

                    // In the fair case we shouldn't be able to obtain the
                    // mutex asynchronously. In the unfair case it should
                    // be possible.
                    if *is_fair {
                        assert!(mtx.try_lock().is_none());

                        // Cancel async lock attempt
                        drop(mutex_fut2);
                        // Now the mutex should be lockable
                    }

                    let guard = mtx.try_lock().unwrap();
                    assert_eq!(true, mtx.is_locked());
                    assert_eq!(*guard, 7);
                    drop(guard);
                }
            }

            #[test]
            fn cancel_wait_for_mutex() {
                for is_fair in &[true, false] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let mtx = $mutex_type::new(5, *is_fair);

                    // Lock the mutex
                    let mut guard1 = mtx.try_lock().unwrap();
                    *guard1 = 27;

                    // The second and third lock attempt must fail
                    let mut mutex_fut2 = Box::pin(mtx.lock());
                    let mut mutex_fut3 = Box::pin(mtx.lock());

                    assert!(mutex_fut2.as_mut().poll(cx).is_pending());
                    assert!(mutex_fut3.as_mut().poll(cx).is_pending());

                    // Before the mutex gets available, cancel one lock attempt
                    drop(mutex_fut2);

                    // Unlock - mutex should be available again. Mutex2 should have been notified
                    drop(guard1);
                    assert_eq!(count, 1);

                    // Unlock - mutex should be available again
                    match mutex_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get locked"),
                        Poll::Ready(guard) => guard,
                    };
                }
            }

            #[test]
            fn unlock_next_when_notification_is_not_used() {
                for is_fair in &[true, false] {
                    let (waker, count) = new_count_waker();
                    let cx = &mut Context::from_waker(&waker);
                    let mtx = $mutex_type::new(5, *is_fair);

                    // Lock the mutex
                    let mut guard1 = mtx.try_lock().unwrap();
                    *guard1 = 27;

                    // The second and third lock attempt must fail
                    let mut mutex_fut2 = Box::pin(mtx.lock());
                    let mut mutex_fut3 = Box::pin(mtx.lock());

                    assert!(mutex_fut2.as_mut().poll(cx).is_pending());
                    assert!(!mutex_fut2.as_mut().is_terminated());

                    assert!(mutex_fut3.as_mut().poll(cx).is_pending());
                    assert!(!mutex_fut3.as_mut().is_terminated());
                    assert_eq!(count, 0);

                    // Unlock - mutex should be available again. Mutex2 should have been notified
                    drop(guard1);
                    assert_eq!(count, 1);

                    // We don't use the notification. Expect the next waiting task to be woken up
                    drop(mutex_fut2);
                    assert_eq!(count, 2);

                    // Unlock - mutex should be available again
                    match mutex_fut3.as_mut().poll(cx) {
                        Poll::Pending => panic!("Expect mutex to get locked"),
                        Poll::Ready(guard) => guard,
                    };
                }
            }

            #[test]
            fn new_waiters_on_unfair_mutex_can_acquire_future_while_one_task_is_notified() {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let mtx = $mutex_type::new(5, false);

                // Lock the mutex
                let mut guard1 = mtx.try_lock().unwrap();
                *guard1 = 27;

                // The second and third lock attempt must fail
                let mut mutex_fut2 = Box::pin(mtx.lock());
                let mut mutex_fut3 = Box::pin(mtx.lock());

                assert!(mutex_fut2.as_mut().poll(cx).is_pending());

                // Unlock - mutex should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Lock fut3 in between. This should succeed
                let guard3 = match mutex_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(guard) => guard,
                };
                // Now fut2 can't use it's notification and is still pending
                assert!(mutex_fut2.as_mut().poll(cx).is_pending());

                // When we drop fut3, the mutex should signal that it's available for fut2,
                // which needs to have re-registered
                drop(guard3);
                assert_eq!(count, 2);
                match mutex_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(_guard) => {}
                };
            }

            #[test]
            fn waiters_on_unfair_mutex_can_acquire_future_through_repolling_if_one_task_is_notified(
            ) {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let mtx = $mutex_type::new(5, false);

                // Lock the mutex
                let mut guard1 = mtx.try_lock().unwrap();
                *guard1 = 27;

                // The second and third lock attempt must fail
                let mut mutex_fut2 = Box::pin(mtx.lock());
                let mut mutex_fut3 = Box::pin(mtx.lock());

                assert!(mutex_fut2.as_mut().poll(cx).is_pending());
                assert!(mutex_fut3.as_mut().poll(cx).is_pending());

                // Unlock - mutex should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Lock fut3 in between. This should succeed
                let guard3 = match mutex_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(guard) => guard,
                };
                // Now fut2 can't use it's notification and is still pending
                assert!(mutex_fut2.as_mut().poll(cx).is_pending());

                // When we drop fut3, the mutex should signal that it's available for fut2,
                // which needs to have re-registered
                drop(guard3);
                assert_eq!(count, 2);
                match mutex_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(_guard) => {}
                };
            }

            #[test]
            fn new_waiters_on_fair_mutex_cant_acquire_future_while_one_task_is_notified() {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let mtx = $mutex_type::new(5, true);

                // Lock the mutex
                let mut guard1 = mtx.try_lock().unwrap();
                *guard1 = 27;

                // The second and third lock attempt must fail
                let mut mutex_fut2 = Box::pin(mtx.lock());
                let mut mutex_fut3 = Box::pin(mtx.lock());

                assert!(mutex_fut2.as_mut().poll(cx).is_pending());

                // Unlock - mutex should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Lock fut3 in between. This should fail
                assert!(mutex_fut3.as_mut().poll(cx).is_pending());

                // fut2 should be lockable
                match mutex_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(_guard) => {}
                };

                // Now fut3 should have been signaled and be lockable
                assert_eq!(count, 2);
                match mutex_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(_guard) => {}
                };
            }

            #[test]
            fn waiters_on_fair_mutex_cant_acquire_future_through_repolling_if_one_task_is_notified()
            {
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                let mtx = $mutex_type::new(5, true);

                // Lock the mutex
                let mut guard1 = mtx.try_lock().unwrap();
                *guard1 = 27;

                // The second and third lock attempt must fail
                let mut mutex_fut2 = Box::pin(mtx.lock());
                let mut mutex_fut3 = Box::pin(mtx.lock());

                assert!(mutex_fut2.as_mut().poll(cx).is_pending());
                assert!(mutex_fut3.as_mut().poll(cx).is_pending());

                // Unlock - mutex should be available again. fut2 should have been notified
                drop(guard1);
                assert_eq!(count, 1);

                // Lock fut3 in between. This should fail, since fut2 should get the mutex first
                assert!(mutex_fut3.as_mut().poll(cx).is_pending());

                // fut2 should be lockable
                match mutex_fut2.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(_guard) => {}
                };

                // Now fut3 should be lockable
                assert_eq!(count, 2);

                match mutex_fut3.as_mut().poll(cx) {
                    Poll::Pending => panic!("Expect mutex to get locked"),
                    Poll::Ready(_guard) => {}
                };
            }

            #[test]
            fn poll_from_multiple_executors() {
                for is_fair in &[true, false] {
                    let (waker_1, count_1) = new_count_waker();
                    let (waker_2, count_2) = new_count_waker();
                    let mtx = $mutex_type::new(5, *is_fair);

                    // Lock the mutex
                    let mut guard1 = mtx.try_lock().unwrap();
                    *guard1 = 27;

                    let cx_1 = &mut Context::from_waker(&waker_1);
                    let cx_2 = &mut Context::from_waker(&waker_2);

                    let fut = mtx.lock();
                    pin_mut!(fut);

                    assert!(fut.as_mut().poll(cx_1).is_pending());
                    assert!(fut.as_mut().poll(cx_2).is_pending());

                    drop(guard1);
                    assert_eq!(count_1, 0);
                    assert_eq!(count_2, 1);

                    assert!(fut.as_mut().poll(cx_2).is_ready());
                    assert!(fut.as_mut().is_terminated());
                }
            }
        }
    };
}

gen_mutex_tests!(local_mutex_tests, LocalMutex);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures::FutureExt;
    use futures_intrusive::sync::Mutex;

    gen_mutex_tests!(mutex_tests, Mutex);

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn mutex_futures_are_send() {
        let mutex = Mutex::new(true, true);
        is_sync(&mutex);
        {
            let lock_fut = mutex.lock();
            is_send(&lock_fut);
            pin_mut!(lock_fut);
            is_send(&lock_fut);

            let waker = &panic_waker();
            let cx = &mut Context::from_waker(&waker);
            pin_mut!(lock_fut);
            let res = lock_fut.poll_unpin(cx);
            let guard = match res {
                Poll::Ready(v) => v,
                Poll::Pending => panic!("Expected to be ready"),
            };
            is_send(&guard);
            is_send_value(guard);
        }
        is_send_value(mutex);
    }
}
