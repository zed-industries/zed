use crate::runtime::scheduler::driver;
use crate::runtime::time_alt::cancellation_queue::{Receiver, Sender};
use crate::runtime::time_alt::{EntryHandle, RegistrationQueue, WakeQueue, Wheel};
use std::time::Duration;

pub(crate) fn min_duration(a: Option<Duration>, b: Option<Duration>) -> Option<Duration> {
    match (a, b) {
        (Some(dur_a), Some(dur_b)) => Some(std::cmp::min(dur_a, dur_b)),
        (Some(dur_a), None) => Some(dur_a),
        (None, Some(dur_b)) => Some(dur_b),
        (None, None) => None,
    }
}

pub(crate) fn process_registration_queue(
    registration_queue: &mut RegistrationQueue,
    wheel: &mut Wheel,
    tx: &Sender,
    wake_queue: &mut WakeQueue,
) {
    while let Some(hdl) = registration_queue.pop_front() {
        if hdl.deadline() <= wheel.elapsed() {
            unsafe {
                wake_queue.push_front(hdl);
            }
        } else {
            // Safety: the entry is not registered yet
            unsafe {
                wheel.insert(hdl, tx.clone());
            }
        }
    }
}

pub(crate) fn insert_inject_timers(
    wheel: &mut Wheel,
    tx: &Sender,
    inject: Vec<EntryHandle>,
    wake_queue: &mut WakeQueue,
) {
    for hdl in inject {
        if hdl.deadline() <= wheel.elapsed() {
            unsafe {
                wake_queue.push_front(hdl);
            }
        } else {
            // Safety: the entry is not registered yet
            unsafe {
                wheel.insert(hdl, tx.clone());
            }
        }
    }
}

pub(crate) fn remove_cancelled_timers(wheel: &mut Wheel, rx: &mut Receiver) {
    for hdl in rx.recv_all() {
        debug_assert!(hdl.is_cancelled());

        if hdl.deadline() > wheel.elapsed() {
            // Safety: the entry is registered in THIS wheel
            unsafe {
                wheel.remove(hdl);
            }
        }
    }
}

pub(crate) fn next_expiration_time(wheel: &Wheel, drv_hdl: &driver::Handle) -> Option<Duration> {
    drv_hdl.with_time(|maybe_time_hdl| {
        let Some(time_hdl) = maybe_time_hdl else {
            // time driver is not enabled, nothing to do.
            return None;
        };

        let clock = drv_hdl.clock();
        let time_source = time_hdl.time_source();

        wheel.next_expiration_time().map(|tick| {
            let now = time_source.now(clock);
            time_source.tick_to_duration(tick.saturating_sub(now))
        })
    })
}

#[cfg(feature = "test-util")]
pub(crate) fn pre_auto_advance(drv_hdl: &driver::Handle, duration: Option<Duration>) -> bool {
    drv_hdl.with_time(|maybe_time_hdl| {
        if maybe_time_hdl.is_none() {
            // time driver is not enabled, nothing to do.
            return false;
        }

        if duration.is_some() {
            let clock = drv_hdl.clock();
            if clock.can_auto_advance() {
                return true;
            }

            false
        } else {
            false
        }
    })
}

pub(crate) fn process_expired_timers(
    wheel: &mut Wheel,
    drv_hdl: &driver::Handle,
    wake_queue: &mut WakeQueue,
) {
    drv_hdl.with_time(|maybe_time_hdl| {
        let Some(time_hdl) = maybe_time_hdl else {
            // time driver is not enabled, nothing to do.
            return;
        };

        let clock = drv_hdl.clock();
        let time_source = time_hdl.time_source();

        let now = time_source.now(clock);
        time_hdl.process_at_time_alt(wheel, now, wake_queue);
    });
}

pub(crate) fn shutdown_local_timers(
    wheel: &mut Wheel,
    rx: &mut Receiver,
    inject: Vec<EntryHandle>,
    drv_hdl: &driver::Handle,
) {
    drv_hdl.with_time(|maybe_time_hdl| {
        let Some(time_hdl) = maybe_time_hdl else {
            // time driver is not enabled, nothing to do.
            return;
        };

        remove_cancelled_timers(wheel, rx);
        time_hdl.shutdown_alt(wheel);

        let mut wake_queue = WakeQueue::new();
        // simply wake all unregistered timers
        for hdl in inject.into_iter().filter(|hdl| !hdl.is_cancelled()) {
            unsafe {
                wake_queue.push_front(hdl);
            }
        }

        wake_queue.wake_all();
    });
}

#[cfg(feature = "test-util")]
pub(crate) fn post_auto_advance(drv_hdl: &driver::Handle, duration: Option<Duration>) {
    drv_hdl.with_time(|maybe_time_hdl| {
        let Some(time_hdl) = maybe_time_hdl else {
            // time driver is not enabled, nothing to do.
            return;
        };

        if let Some(park_duration) = duration {
            let clock = drv_hdl.clock();
            if clock.can_auto_advance() && !time_hdl.did_wake() {
                if let Err(msg) = clock.advance(park_duration) {
                    panic!("{msg}");
                }
            }
        }
    })
}

#[cfg(not(feature = "test-util"))]
pub(crate) fn pre_auto_advance(_drv_hdl: &driver::Handle, _duration: Option<Duration>) -> bool {
    false
}

#[cfg(not(feature = "test-util"))]
pub(crate) fn post_auto_advance(_drv_hdl: &driver::Handle, _duration: Option<Duration>) {
    // No-op in non-test util builds
}
