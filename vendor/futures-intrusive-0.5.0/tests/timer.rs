use core::time::Duration;
use futures::future::{FusedFuture, Future};
use futures::task::Context;
use futures_intrusive::timer::{LocalTimerService, MockClock};
use futures_test::task::{new_count_waker, panic_waker};
use pin_utils::pin_mut;

macro_rules! gen_timer_tests {
    ($mod_name:ident, $timer_type:ident, $timer_trait_type:ident) => {
        mod $mod_name {
            use super::*;
            use futures_intrusive::timer::$timer_trait_type;

            #[test]
            fn start_and_expire_timers() {
                static TEST_CLOCK: MockClock = MockClock::new();
                TEST_CLOCK.set_time(200);
                let timer = $timer_type::new(&TEST_CLOCK);
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);
                assert!(timer.next_expiration().is_none());

                let fut = timer.deadline(999);
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_pending());
                assert_eq!(Some(999), timer.next_expiration());

                let fut2 = timer.delay(Duration::from_millis(300));
                pin_mut!(fut2);
                assert!(fut2.as_mut().poll(cx).is_pending());
                assert_eq!(Some(500), timer.next_expiration());

                let fut3 = timer.delay(Duration::from_millis(500));
                pin_mut!(fut3);
                assert!(fut3.as_mut().poll(cx).is_pending());
                assert_eq!(Some(500), timer.next_expiration());

                TEST_CLOCK.set_time(500);
                timer.check_expirations();
                assert_eq!(count, 1);
                assert!(fut.as_mut().poll(cx).is_pending());
                assert!(fut2.as_mut().poll(cx).is_ready());
                assert!(fut3.as_mut().poll(cx).is_pending());
                assert_eq!(Some(700), timer.next_expiration());

                TEST_CLOCK.set_time(699);
                timer.check_expirations();
                assert_eq!(count, 1);

                TEST_CLOCK.set_time(700);
                timer.check_expirations();
                assert_eq!(count, 2);

                assert!(fut.as_mut().poll(cx).is_pending());
                assert!(fut3.as_mut().poll(cx).is_ready());
                assert_eq!(Some(999), timer.next_expiration());

                TEST_CLOCK.set_time(1000);
                timer.check_expirations();
                assert_eq!(count, 3);

                assert!(fut.as_mut().poll(cx).is_ready());
                assert_eq!(None, timer.next_expiration());
            }

            #[test]
            fn immediately_ready_timer() {
                static TEST_CLOCK: MockClock = MockClock::new();
                TEST_CLOCK.set_time(400);
                let timer = $timer_type::new(&TEST_CLOCK);
                let waker = &panic_waker();
                let cx = &mut Context::from_waker(&waker);

                let fut = timer.delay(Duration::from_millis(0));
                pin_mut!(fut);
                assert!(fut.as_mut().poll(cx).is_ready());

                for ts in 389..=400 {
                    let fut2 = timer.deadline(ts);
                    pin_mut!(fut2);
                    assert!(fut2.as_mut().poll(cx).is_ready());
                }
            }

            #[test]
            fn can_use_timer_as_trait_object() {
                static TEST_CLOCK: MockClock = MockClock::new();
                TEST_CLOCK.set_time(340);
                let timer = $timer_type::new(&TEST_CLOCK);
                let (waker, _count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let mut inner = |dyn_timer: &dyn $timer_trait_type| {
                    let fut = dyn_timer.delay(Duration::from_millis(10));
                    pin_mut!(fut);
                    assert!(fut.as_mut().poll(cx).is_pending());
                    TEST_CLOCK.set_time(350);
                    timer.check_expirations();

                    assert!(fut.as_mut().poll(cx).is_ready());
                };

                inner(&timer);
            }

            #[test]
            fn cancel_mid_wait() {
                static TEST_CLOCK: MockClock = MockClock::new();
                TEST_CLOCK.set_time(1300);
                let timer = $timer_type::new(&TEST_CLOCK);
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                {
                    // Cancel a wait in between other waits
                    // In order to arbitrarily drop a non movable future we have to box and pin it
                    let mut poll1 = Box::pin(timer.deadline(1400));
                    let mut poll2 = Box::pin(timer.deadline(1500));
                    let mut poll3 = Box::pin(timer.deadline(1600));
                    let mut poll4 = Box::pin(timer.deadline(1700));
                    let mut poll5 = Box::pin(timer.deadline(1800));

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
                    TEST_CLOCK.set_time(1800);
                    timer.check_expirations();

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
                static TEST_CLOCK: MockClock = MockClock::new();
                TEST_CLOCK.set_time(2300);
                let timer = $timer_type::new(&TEST_CLOCK);
                let (waker, count) = new_count_waker();
                let cx = &mut Context::from_waker(&waker);

                let poll1 = timer.deadline(2400);
                let poll2 = timer.deadline(2500);
                let poll3 = timer.deadline(2600);
                let poll4 = timer.deadline(2700);

                pin_mut!(poll1);
                pin_mut!(poll2);
                pin_mut!(poll3);
                pin_mut!(poll4);

                assert!(poll1.as_mut().poll(cx).is_pending());
                assert!(poll2.as_mut().poll(cx).is_pending());

                // Start polling some wait handles which get cancelled
                // before new ones are attached
                {
                    let poll5 = timer.deadline(2350);
                    let poll6 = timer.deadline(2650);
                    pin_mut!(poll5);
                    pin_mut!(poll6);
                    assert!(poll5.as_mut().poll(cx).is_pending());
                    assert!(poll6.as_mut().poll(cx).is_pending());
                }

                assert!(poll3.as_mut().poll(cx).is_pending());
                assert!(poll4.as_mut().poll(cx).is_pending());

                TEST_CLOCK.set_time(2700);
                timer.check_expirations();

                assert!(poll1.as_mut().poll(cx).is_ready());
                assert!(poll2.as_mut().poll(cx).is_ready());
                assert!(poll3.as_mut().poll(cx).is_ready());
                assert!(poll4.as_mut().poll(cx).is_ready());

                assert_eq!(count, 4);
            }

            #[test]
            fn poll_from_multiple_executors() {
                static TEST_CLOCK: MockClock = MockClock::new();
                TEST_CLOCK.set_time(2300);

                let timer = $timer_type::new(&TEST_CLOCK);

                let (waker_1, count_1) = new_count_waker();
                let (waker_2, count_2) = new_count_waker();
                let cx_1 = &mut Context::from_waker(&waker_1);
                let cx_2 = &mut Context::from_waker(&waker_2);

                let fut = timer.deadline(2400);
                pin_mut!(fut);

                assert!(fut.as_mut().poll(cx_1).is_pending());
                assert!(fut.as_mut().poll(cx_2).is_pending());

                TEST_CLOCK.set_time(2700);
                timer.check_expirations();
                assert_eq!(count_1, 0);
                assert_eq!(count_2, 1);

                assert!(fut.as_mut().poll(cx_2).is_ready());
                assert!(fut.as_mut().is_terminated());
            }
        }
    };
}

gen_timer_tests!(local_timer_service_tests, LocalTimerService, LocalTimer);

#[cfg(feature = "std")]
mod if_std {
    use super::*;
    use futures_intrusive::timer::{Timer, TimerService};

    gen_timer_tests!(timer_service_tests, TimerService, Timer);

    fn is_send<T: Send>(_: &T) {}

    fn is_send_value<T: Send>(_: T) {}

    fn is_sync<T: Sync>(_: &T) {}

    #[test]
    fn timer_futures_are_send() {
        static TEST_CLOCK: MockClock = MockClock::new();
        TEST_CLOCK.set_time(2300);
        let timer = TimerService::new(&TEST_CLOCK);
        is_sync(&timer);
        {
            let deadline = timer.deadline(2400);
            is_send(&deadline);
            pin_mut!(deadline);
            is_send(&deadline);
            let delay_fut = timer.delay(Duration::from_millis(1000));
            is_send(&delay_fut);
            pin_mut!(delay_fut);
            is_send(&delay_fut);
        }
        is_send_value(timer);
    }
}
