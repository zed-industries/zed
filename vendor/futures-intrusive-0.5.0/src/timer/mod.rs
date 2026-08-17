//! Asynchronous timers.
//!
//! This module provides a timer implementation which returns awaitable
//! `Future`s.
//! The timer can work with a configurable clock source. In order to utilize
//! the system clock, a global instance `StdClock` can be utilized.

mod clock;
pub use self::clock::{Clock, MockClock};

#[cfg(feature = "std")]
pub use self::clock::StdClock;

mod timer;

pub use self::timer::{
    GenericTimerService, LocalTimer, LocalTimerFuture, LocalTimerService,
    Timer, TimerFuture,
};

#[cfg(feature = "std")]
pub use self::timer::TimerService;
