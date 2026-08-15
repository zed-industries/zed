//! Lets tests skip the display map's test-only invariants, so that corrupted
//! state propagates the way it does in production builds, where no
//! invariants run, and downstream symptoms become observable.

use std::cell::Cell;

thread_local! {
    static SIMULATING: Cell<bool> = const { Cell::new(false) };
}

/// True when the display map's test-only invariants should be skipped.
/// Enabled process-wide by the `SIMULATE_PRODUCTION` environment variable
/// (for randomized seed hunts driven from the command line) or per-thread by
/// [`SimulateProductionGuard`].
pub fn is_simulating_production() -> bool {
    SIMULATING.with(|flag| flag.get()) || std::env::var("SIMULATE_PRODUCTION").is_ok()
}

/// Skips the display map's test-only invariants on the current thread while
/// held. GPUI tests drive all display map layers from the test's own thread,
/// so this doesn't leak into tests running in parallel the way setting the
/// environment variable would.
pub struct SimulateProductionGuard;

impl SimulateProductionGuard {
    pub fn new() -> Self {
        SIMULATING.with(|flag| flag.set(true));
        Self
    }
}

impl Drop for SimulateProductionGuard {
    fn drop(&mut self) {
        SIMULATING.with(|flag| flag.set(false));
    }
}
