//! Timer event source
//!
//! The [`Timer`] is an event source that will fire its event after a certain amount of time
//! specified at creation. Its timing is tracked directly by the event loop core logic, and it does
//! not consume any system resource.
//!
//! As of calloop v0.11.0, the event loop always uses high-precision timers. However, the timer
//! precision varies between operating systems; for instance, the scheduler granularity on Windows
//! is about 16 milliseconds. If you need to rely on good precision timers in general, you may need
//! to enable realtime features of your OS to ensure your thread is quickly woken up by the system
//! scheduler.
//!
//! The provided event is an [`Instant`] representing the deadline for which this timer has fired
//! (which can be earlier than the current time depending on the event loop congestion).
//!
//! The callback associated with this event source is expected to return a [`TimeoutAction`], which
//! can be used to implement self-repeating timers by telling calloop to reprogram the same timer
//! for a later timeout after it has fired.

/*
 * This module provides two main types:
 *
 * - `Timer` is the user-facing type that represents a timer event source
 * - `TimerWheel` is an internal data structure for tracking registered timeouts, it is used by
 *   the polling logic in sys/mod.rs
 */

use std::{
    cell::RefCell,
    collections::BinaryHeap,
    rc::Rc,
    task::Waker,
    time::{Duration, Instant},
};

use crate::{EventSource, LoopHandle, Poll, PostAction, Readiness, Token, TokenFactory};

#[derive(Debug)]
struct Registration {
    token: Token,
    wheel: Rc<RefCell<TimerWheel>>,
    counter: u32,
}

/// A timer event source
///
/// When registered to the event loop, it will trigger an event once its deadline is reached.
/// If the deadline is in the past relative to the moment of its insertion in the event loop,
/// the `TImer` will trigger an event as soon as the event loop is dispatched.
#[derive(Debug)]
pub struct Timer {
    registration: Option<Registration>,
    deadline: Option<Instant>,
}

impl Timer {
    /// Create a timer that will fire immediately when inserted in the event loop
    pub fn immediate() -> Timer {
        Self::from_deadline(Instant::now())
    }

    /// Create a timer that will fire after a given duration from now
    pub fn from_duration(duration: Duration) -> Timer {
        Self::from_deadline_inner(Instant::now().checked_add(duration))
    }

    /// Create a timer that will fire at a given instant
    pub fn from_deadline(deadline: Instant) -> Timer {
        Self::from_deadline_inner(Some(deadline))
    }

    fn from_deadline_inner(deadline: Option<Instant>) -> Timer {
        Timer {
            registration: None,
            deadline,
        }
    }

    /// Changes the deadline of this timer to an [`Instant`]
    ///
    /// If the `Timer` is currently registered in the event loop, it needs to be
    /// re-registered for this change to take effect.
    pub fn set_deadline(&mut self, deadline: Instant) {
        self.deadline = Some(deadline);
    }

    /// Changes the deadline of this timer to a [`Duration`] from now
    ///
    /// If the `Timer` is currently registered in the event loop, it needs to be
    /// re-registered for this change to take effect.
    pub fn set_duration(&mut self, duration: Duration) {
        self.deadline = Instant::now().checked_add(duration);
    }

    /// Get the current deadline of this `Timer`
    ///
    /// Returns `None` if the timer has overflowed.
    pub fn current_deadline(&self) -> Option<Instant> {
        self.deadline
    }
}

impl EventSource for Timer {
    type Event = Instant;
    type Metadata = ();
    type Ret = TimeoutAction;
    type Error = std::io::Error;

    fn process_events<F>(
        &mut self,
        _: Readiness,
        token: Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        if let (Some(ref registration), Some(ref deadline)) = (&self.registration, &self.deadline) {
            if registration.token != token {
                return Ok(PostAction::Continue);
            }
            let new_deadline = match callback(*deadline, &mut ()) {
                TimeoutAction::Drop => return Ok(PostAction::Remove),
                TimeoutAction::ToInstant(instant) => instant,
                TimeoutAction::ToDuration(duration) => match Instant::now().checked_add(duration) {
                    Some(new_deadline) => new_deadline,
                    None => {
                        // The timer has overflowed, meaning we have no choice but to drop it.
                        self.deadline = None;
                        return Ok(PostAction::Remove);
                    }
                },
            };
            // If we received an event, we MUST have a valid counter value
            registration.wheel.borrow_mut().insert_reuse(
                registration.counter,
                new_deadline,
                registration.token,
            );
            self.deadline = Some(new_deadline);
        }
        Ok(PostAction::Continue)
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> crate::Result<()> {
        // Only register a deadline if we haven't overflowed.
        if let Some(deadline) = self.deadline {
            let wheel = poll.timers.clone();
            let token = token_factory.token();
            let counter = wheel.borrow_mut().insert(deadline, token);
            self.registration = Some(Registration {
                token,
                wheel,
                counter,
            });
        }

        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> crate::Result<()> {
        self.unregister(poll)?;
        self.register(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
        if let Some(registration) = self.registration.take() {
            poll.timers.borrow_mut().cancel(registration.counter);
        }
        Ok(())
    }
}

/// Action to reschedule a timeout if necessary
#[derive(Debug)]
pub enum TimeoutAction {
    /// Don't reschedule this timer
    Drop,
    /// Reschedule this timer to a given [`Instant`]
    ToInstant(Instant),
    /// Reschedule this timer to a given [`Duration`] in the future
    ToDuration(Duration),
}

// Internal representation of a timeout registered in the TimerWheel
#[derive(Debug)]
struct TimeoutData {
    deadline: Instant,
    token: RefCell<Option<Token>>,
    counter: u32,
}

// A data structure for tracking registered timeouts
#[derive(Debug)]
pub(crate) struct TimerWheel {
    heap: BinaryHeap<TimeoutData>,
    counter: u32,
}

impl TimerWheel {
    pub(crate) fn new() -> TimerWheel {
        TimerWheel {
            heap: BinaryHeap::new(),
            counter: 0,
        }
    }

    pub(crate) fn insert(&mut self, deadline: Instant, token: Token) -> u32 {
        self.heap.push(TimeoutData {
            deadline,
            token: RefCell::new(Some(token)),
            counter: self.counter,
        });
        let ret = self.counter;
        self.counter += 1;
        ret
    }

    pub(crate) fn insert_reuse(&mut self, counter: u32, deadline: Instant, token: Token) {
        self.heap.push(TimeoutData {
            deadline,
            token: RefCell::new(Some(token)),
            counter,
        });
    }

    pub(crate) fn cancel(&mut self, counter: u32) {
        if self
            .heap
            .peek()
            .map(|data| data.counter == counter)
            .unwrap_or(false)
        {
            self.heap.pop();
            return;
        };

        self.heap
            .iter()
            .rev()
            .find(|data| data.counter == counter)
            .map(|data| data.token.take());
    }

    pub(crate) fn next_expired(&mut self, now: Instant) -> Option<(u32, Token)> {
        loop {
            // check if there is an expired item
            if let Some(data) = self.heap.peek() {
                if data.deadline > now {
                    return None;
                }
                // there is an expired timeout, continue the
                // loop body
            } else {
                return None;
            }

            // There is an item in the heap, this unwrap cannot blow
            let data = self.heap.pop().unwrap();
            if let Some(token) = data.token.into_inner() {
                return Some((data.counter, token));
            }
            // otherwise this timeout was cancelled, continue looping
        }
    }

    pub(crate) fn next_deadline(&self) -> Option<std::time::Instant> {
        self.heap.peek().map(|data| data.deadline)
    }
}

// trait implementations for TimeoutData

impl std::cmp::Ord for TimeoutData {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // earlier values have priority
        self.deadline.cmp(&other.deadline).reverse()
    }
}

impl std::cmp::PartialOrd for TimeoutData {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// This impl is required for PartialOrd but actually never used
// and the type is private, so ignore its coverage
impl std::cmp::PartialEq for TimeoutData {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}

impl std::cmp::Eq for TimeoutData {}

// Logic for timer futures

/// A future that resolves once a certain timeout is expired
pub struct TimeoutFuture {
    deadline: Option<Instant>,
    waker: Rc<RefCell<Option<Waker>>>,
}

impl std::fmt::Debug for TimeoutFuture {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeoutFuture")
            .field("deadline", &self.deadline)
            .finish_non_exhaustive()
    }
}

impl TimeoutFuture {
    /// Create a future that resolves after a given duration
    pub fn from_duration<Data>(handle: &LoopHandle<'_, Data>, duration: Duration) -> TimeoutFuture {
        Self::from_deadline_inner(handle, Instant::now().checked_add(duration))
    }

    /// Create a future that resolves at a given instant
    pub fn from_deadline<Data>(handle: &LoopHandle<'_, Data>, deadline: Instant) -> TimeoutFuture {
        Self::from_deadline_inner(handle, Some(deadline))
    }

    /// Create a future that resolves at a given instant
    fn from_deadline_inner<Data>(
        handle: &LoopHandle<'_, Data>,
        deadline: Option<Instant>,
    ) -> TimeoutFuture {
        let timer = Timer::from_deadline_inner(deadline);
        let waker = Rc::new(RefCell::new(None::<Waker>));
        handle
            .insert_source(timer, {
                let waker = waker.clone();
                move |_, &mut (), _| {
                    if let Some(waker) = waker.borrow_mut().clone() {
                        waker.wake()
                    }
                    TimeoutAction::Drop
                }
            })
            .unwrap();

        TimeoutFuture { deadline, waker }
    }
}

impl std::future::Future for TimeoutFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.deadline {
            None => return std::task::Poll::Pending,

            Some(deadline) => {
                if Instant::now() >= deadline {
                    return std::task::Poll::Ready(());
                }
            }
        }

        *self.waker.borrow_mut() = Some(cx.waker().clone());
        std::task::Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::time::Duration;

    #[test]
    fn simple_timer() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        event_loop
            .handle()
            .insert_source(
                Timer::from_duration(Duration::from_millis(100)),
                |_, &mut (), dispatched| {
                    *dispatched = true;
                    TimeoutAction::Drop
                },
            )
            .unwrap();

        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        // not yet dispatched
        assert!(!dispatched);

        event_loop
            .dispatch(Some(Duration::from_millis(150)), &mut dispatched)
            .unwrap();
        // now dispatched
        assert!(dispatched);
    }

    #[test]
    fn simple_timer_instant() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        event_loop
            .handle()
            .insert_source(
                Timer::from_duration(Duration::from_millis(100)),
                |_, &mut (), dispatched| {
                    *dispatched = true;
                    TimeoutAction::Drop
                },
            )
            .unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(150)), &mut dispatched)
            .unwrap();
        // now dispatched
        assert!(dispatched);
    }

    #[test]
    fn immediate_timer() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        event_loop
            .handle()
            .insert_source(Timer::immediate(), |_, &mut (), dispatched| {
                *dispatched = true;
                TimeoutAction::Drop
            })
            .unwrap();

        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        // now dispatched
        assert!(dispatched);
    }

    // We cannot actually test high precision timers, as they are only high precision in release mode
    // This test is here to ensure that the high-precision codepath are executed and work as intended
    // even if we cannot test if they are actually high precision
    #[test]
    fn high_precision_timer() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        event_loop
            .handle()
            .insert_source(
                Timer::from_duration(Duration::from_millis(100)),
                |_, &mut (), dispatched| {
                    *dispatched = true;
                    TimeoutAction::Drop
                },
            )
            .unwrap();

        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        // not yet dispatched
        assert!(!dispatched);

        event_loop
            .dispatch(Some(Duration::from_micros(10200)), &mut dispatched)
            .unwrap();
        // yet not dispatched
        assert!(!dispatched);

        event_loop
            .dispatch(Some(Duration::from_millis(100)), &mut dispatched)
            .unwrap();
        // now dispatched
        assert!(dispatched);
    }

    #[test]
    fn cancel_timer() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        let token = event_loop
            .handle()
            .insert_source(
                Timer::from_duration(Duration::from_millis(100)),
                |_, &mut (), dispatched| {
                    *dispatched = true;
                    TimeoutAction::Drop
                },
            )
            .unwrap();

        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        // not yet dispatched
        assert!(!dispatched);

        event_loop.handle().remove(token);

        event_loop
            .dispatch(Some(Duration::from_millis(150)), &mut dispatched)
            .unwrap();
        // still not dispatched
        assert!(!dispatched);
    }

    #[test]
    fn repeating_timer() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = 0;

        event_loop
            .handle()
            .insert_source(
                Timer::from_duration(Duration::from_millis(500)),
                |_, &mut (), dispatched| {
                    *dispatched += 1;
                    TimeoutAction::ToDuration(Duration::from_millis(500))
                },
            )
            .unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(250)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 0);

        event_loop
            .dispatch(Some(Duration::from_millis(510)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 1);

        event_loop
            .dispatch(Some(Duration::from_millis(510)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 2);

        event_loop
            .dispatch(Some(Duration::from_millis(510)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 3);
    }

    #[cfg(feature = "executor")]
    #[test]
    fn timeout_future() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = 0;

        let timeout_1 =
            TimeoutFuture::from_duration(&event_loop.handle(), Duration::from_millis(500));
        let timeout_2 =
            TimeoutFuture::from_duration(&event_loop.handle(), Duration::from_millis(1500));
        // This one should never go off.
        let timeout_3 = TimeoutFuture::from_duration(&event_loop.handle(), Duration::MAX);

        let (exec, sched) = crate::sources::futures::executor().unwrap();
        event_loop
            .handle()
            .insert_source(exec, move |(), &mut (), got| {
                *got += 1;
            })
            .unwrap();

        sched.schedule(timeout_1).unwrap();
        sched.schedule(timeout_2).unwrap();
        sched.schedule(timeout_3).unwrap();

        // We do a 0-timeout dispatch after every regular dispatch to let the timeout triggers
        // flow back to the executor

        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 0);

        event_loop
            .dispatch(Some(Duration::from_millis(1000)), &mut dispatched)
            .unwrap();
        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 1);

        event_loop
            .dispatch(Some(Duration::from_millis(1100)), &mut dispatched)
            .unwrap();
        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 2);
    }

    #[test]
    fn no_overflow() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = 0;

        event_loop
            .handle()
            .insert_source(
                Timer::from_duration(Duration::from_millis(500)),
                |_, &mut (), dispatched| {
                    *dispatched += 1;
                    TimeoutAction::Drop
                },
            )
            .unwrap();

        event_loop
            .handle()
            .insert_source(Timer::from_duration(Duration::MAX), |_, &mut (), _| {
                panic!("This timer should never go off")
            })
            .unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(250)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 0);

        event_loop
            .dispatch(Some(Duration::from_millis(510)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 1);

        event_loop
            .dispatch(Some(Duration::from_millis(510)), &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 1);
    }
}
