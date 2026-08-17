use cap_std::time::{Duration, Instant, SystemClock};
use cap_std::{AmbientAuthority, ambient_authority};
use cap_time_ext::{MonotonicClockExt as _, SystemClockExt as _};

pub struct WasiClocksCtx {
    pub wall_clock: Box<dyn HostWallClock + Send>,
    pub monotonic_clock: Box<dyn HostMonotonicClock + Send>,
}

impl Default for WasiClocksCtx {
    fn default() -> Self {
        Self {
            wall_clock: wall_clock(),
            monotonic_clock: monotonic_clock(),
        }
    }
}

pub trait WasiClocksView: Send {
    fn clocks(&mut self) -> &mut WasiClocksCtx;
}

impl WasiClocksView for WasiClocksCtx {
    fn clocks(&mut self) -> &mut WasiClocksCtx {
        self
    }
}

pub trait HostWallClock: Send {
    fn resolution(&self) -> Duration;
    fn now(&self) -> Duration;
}

pub trait HostMonotonicClock: Send {
    fn resolution(&self) -> u64;
    fn now(&self) -> u64;
}

pub struct WallClock {
    /// The underlying system clock.
    clock: cap_std::time::SystemClock,
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new(ambient_authority())
    }
}

impl WallClock {
    pub fn new(ambient_authority: AmbientAuthority) -> Self {
        Self {
            clock: cap_std::time::SystemClock::new(ambient_authority),
        }
    }
}

impl HostWallClock for WallClock {
    fn resolution(&self) -> Duration {
        self.clock.resolution()
    }

    fn now(&self) -> Duration {
        // WASI defines wall clocks to return "Unix time".
        self.clock
            .now()
            .duration_since(SystemClock::UNIX_EPOCH)
            .unwrap()
    }
}

pub struct MonotonicClock {
    /// The underlying system clock.
    clock: cap_std::time::MonotonicClock,

    /// The `Instant` this clock was created. All returned times are
    /// durations since that time.
    initial: Instant,
}

impl Default for MonotonicClock {
    fn default() -> Self {
        Self::new(ambient_authority())
    }
}

impl MonotonicClock {
    pub fn new(ambient_authority: AmbientAuthority) -> Self {
        let clock = cap_std::time::MonotonicClock::new(ambient_authority);
        let initial = clock.now();
        Self { clock, initial }
    }
}

impl HostMonotonicClock for MonotonicClock {
    fn resolution(&self) -> u64 {
        self.clock.resolution().as_nanos().try_into().unwrap()
    }

    fn now(&self) -> u64 {
        // Unwrap here and in `resolution` above; a `u64` is wide enough to
        // hold over 584 years of nanoseconds.
        self.clock
            .now()
            .duration_since(self.initial)
            .as_nanos()
            .try_into()
            .unwrap()
    }
}

pub fn monotonic_clock() -> Box<dyn HostMonotonicClock + Send> {
    Box::new(MonotonicClock::default())
}

pub fn wall_clock() -> Box<dyn HostWallClock + Send> {
    Box::new(WallClock::default())
}
