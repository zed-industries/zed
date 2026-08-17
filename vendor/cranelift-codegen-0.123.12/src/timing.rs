//! Pass timing.
//!
//! This modules provides facilities for timing the execution of individual compilation passes.

use alloc::boxed::Box;
use core::any::Any;
use core::fmt;

// Each pass that can be timed is predefined with the `define_passes!` macro. Each pass has a
// snake_case name and a plain text description used when printing out the timing report.
//
// This macro defines:
//
// - A C-style enum containing all the pass names and a `None` variant.
// - A usize constant with the number of defined passes.
// - A const array of pass descriptions.
// - A public function per pass used to start the timing of that pass.
macro_rules! define_passes {
    ($($pass:ident: $desc:expr,)+) => {
        /// A single profiled pass.
        #[expect(non_camel_case_types, reason = "macro-generated code")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Pass {
            $(#[doc=$desc] $pass,)+
            /// No active pass.
            None,
        }

        /// The amount of profiled passes.
        pub const NUM_PASSES: usize = Pass::None as usize;

        const DESCRIPTIONS: [&str; NUM_PASSES] = [ $($desc),+ ];

        $(
            #[doc=$desc]
            #[must_use]
            pub fn $pass() -> Box<dyn Any> {
                start_pass(Pass::$pass)
            }
        )+
    }
}

// Pass definitions.
define_passes! {
    // All these are used in other crates but defined here so they appear in the unified
    // `PassTimes` output.
    process_file: "Processing test file",
    parse_text: "Parsing textual Cranelift IR",
    wasm_translate_module: "Translate WASM module",
    wasm_translate_function: "Translate WASM function",

    verifier: "Verify Cranelift IR",

    compile: "Compilation passes",
    try_incremental_cache: "Try loading from incremental cache",
    store_incremental_cache: "Store in incremental cache",
    flowgraph: "Control flow graph",
    domtree: "Dominator tree",
    loop_analysis: "Loop analysis",
    preopt: "Pre-legalization rewriting",
    egraph: "Egraph based optimizations",
    gvn: "Global value numbering",
    licm: "Loop invariant code motion",
    unreachable_code: "Remove unreachable blocks",
    remove_constant_phis: "Remove constant phi-nodes",

    vcode_lower: "VCode lowering",
    vcode_emit: "VCode emission",
    vcode_emit_finish: "VCode emission finalization",

    regalloc: "Register allocation",
    regalloc_checker: "Register allocation symbolic verification",
    layout_renumber: "Layout full renumbering",

    canonicalize_nans: "Canonicalization of NaNs",
}

impl Pass {
    fn idx(self) -> usize {
        self as usize
    }

    /// Description of the pass.
    pub fn description(self) -> &'static str {
        match DESCRIPTIONS.get(self.idx()) {
            Some(s) => s,
            None => "<no pass>",
        }
    }
}

impl fmt::Display for Pass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// A profiler.
pub trait Profiler {
    /// Start a profiling pass.
    ///
    /// Will return a token which when dropped indicates the end of the pass.
    ///
    /// Multiple passes can be active at the same time, but they must be started and stopped in a
    /// LIFO fashion.
    fn start_pass(&self, pass: Pass) -> Box<dyn Any>;
}

/// The default profiler. You can get the results using [`take_current`].
pub struct DefaultProfiler;

#[cfg(not(feature = "timing"))]
pub(crate) use disabled::*;
#[cfg(feature = "timing")]
pub use enabled::*;

#[cfg(feature = "timing")]
mod enabled {
    use super::{DESCRIPTIONS, DefaultProfiler, NUM_PASSES, Pass, Profiler};
    use std::any::Any;
    use std::boxed::Box;
    use std::cell::{Cell, RefCell};
    use std::fmt;
    use std::mem;
    use std::time::Duration;
    use std::time::Instant;

    // Information about passes in a single thread.
    thread_local! {
        static PROFILER: RefCell<Box<dyn Profiler>> = RefCell::new(Box::new(DefaultProfiler));
    }

    /// Set the profiler for the current thread.
    ///
    /// Returns the old profiler.
    pub fn set_thread_profiler(new_profiler: Box<dyn Profiler>) -> Box<dyn Profiler> {
        PROFILER.with(|profiler| std::mem::replace(&mut *profiler.borrow_mut(), new_profiler))
    }

    /// Start timing `pass` as a child of the currently running pass, if any.
    ///
    /// This function is called by the publicly exposed pass functions.
    pub fn start_pass(pass: Pass) -> Box<dyn Any> {
        PROFILER.with(|profiler| profiler.borrow().start_pass(pass))
    }

    /// Accumulated timing information for a single pass.
    #[derive(Default, Copy, Clone)]
    struct PassTime {
        /// Total time spent running this pass including children.
        total: Duration,

        /// Time spent running in child passes.
        child: Duration,
    }

    /// Accumulated timing for all passes.
    pub struct PassTimes {
        pass: [PassTime; NUM_PASSES],
    }

    impl PassTimes {
        /// Add `other` to the timings of this `PassTimes`.
        pub fn add(&mut self, other: &Self) {
            for (a, b) in self.pass.iter_mut().zip(&other.pass[..]) {
                a.total += b.total;
                a.child += b.child;
            }
        }

        /// Returns the total amount of time taken by all the passes measured.
        pub fn total(&self) -> Duration {
            self.pass.iter().map(|p| p.total - p.child).sum()
        }
    }

    impl Default for PassTimes {
        fn default() -> Self {
            Self {
                pass: [Default::default(); NUM_PASSES],
            }
        }
    }

    impl fmt::Display for PassTimes {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "======== ========  ==================================")?;
            writeln!(f, "   Total     Self  Pass")?;
            writeln!(f, "-------- --------  ----------------------------------")?;
            for (time, desc) in self.pass.iter().zip(&DESCRIPTIONS[..]) {
                // Omit passes that haven't run.
                if time.total == Duration::default() {
                    continue;
                }

                // Write a duration as secs.millis, trailing space.
                fn fmtdur(mut dur: Duration, f: &mut fmt::Formatter) -> fmt::Result {
                    // Round to nearest ms by adding 500us.
                    dur += Duration::new(0, 500_000);
                    let ms = dur.subsec_millis();
                    write!(f, "{:4}.{:03} ", dur.as_secs(), ms)
                }

                fmtdur(time.total, f)?;
                if let Some(s) = time.total.checked_sub(time.child) {
                    fmtdur(s, f)?;
                }
                writeln!(f, " {desc}")?;
            }
            writeln!(f, "======== ========  ==================================")
        }
    }

    // Information about passes in a single thread.
    thread_local! {
        static PASS_TIME: RefCell<PassTimes> = RefCell::new(Default::default());
    }

    /// Take the current accumulated pass timings and reset the timings for the current thread.
    ///
    /// Only applies when [`DefaultProfiler`] is used.
    ///
    /// # Panics
    ///
    /// Panics when any pass timing token is currently active.
    pub fn take_current() -> PassTimes {
        assert_eq!(
            CURRENT_PASS.with(|p| p.get()),
            Pass::None,
            "Cannot take_current() on profiler while a pass is active"
        );
        PASS_TIME.with(|rc| mem::take(&mut *rc.borrow_mut()))
    }

    // Information about passes in a single thread.
    thread_local! {
        static CURRENT_PASS: Cell<Pass> = const { Cell::new(Pass::None) };
        static LAST_INSTANT: Cell<Option<Instant>> = Cell::new(None);
    }

    /// Provides a guaranteed-never-decreasing `Instant`. This is
    /// supposed to be guaranteed by the operating system APIs that
    /// are used to implement `Instant::now()`, but apparently
    /// sometimes kernel bugs may result in this moving backward, and
    /// if it does, our invariants here are violated, possibly
    /// resulting in panics when we process nested time spans. So we
    /// backstop against any bugs by tracking the last-returned
    /// `Instant` and never regressing before it.
    fn monotonic_instant() -> Instant {
        let now = match LAST_INSTANT.get() {
            None => Instant::now(),
            Some(prev) => {
                let system_now = Instant::now();
                core::cmp::max(prev, system_now)
            }
        };
        LAST_INSTANT.set(Some(now));
        now
    }

    impl Profiler for DefaultProfiler {
        fn start_pass(&self, pass: Pass) -> Box<dyn Any> {
            let prev = CURRENT_PASS.with(|p| p.replace(pass));
            log::debug!("timing: Starting {pass}, (during {prev})");
            Box::new(DefaultTimingToken {
                start: monotonic_instant(),
                pass,
                prev,
            })
        }
    }

    /// A timing token is responsible for timing the currently running pass. Timing starts when it
    /// is created and ends when it is dropped.
    ///
    /// Multiple passes can be active at the same time, but they must be started and stopped in a
    /// LIFO fashion.
    struct DefaultTimingToken {
        /// Start time for this pass.
        start: Instant,

        // Pass being timed by this token.
        pass: Pass,

        // The previously active pass which will be restored when this token is dropped.
        prev: Pass,
    }

    /// Dropping a timing token indicated the end of the pass.
    impl Drop for DefaultTimingToken {
        fn drop(&mut self) {
            let now = monotonic_instant();
            let duration = now.duration_since(self.start);
            log::debug!("timing: Ending {}: {}ms", self.pass, duration.as_millis());
            let old_cur = CURRENT_PASS.with(|p| p.replace(self.prev));
            assert_eq!(self.pass, old_cur, "Timing tokens dropped out of order");
            PASS_TIME.with(|rc| {
                let mut table = rc.borrow_mut();
                table.pass[self.pass.idx()].total += duration;
                if let Some(parent) = table.pass.get_mut(self.prev.idx()) {
                    parent.child += duration;
                }
            })
        }
    }
}

#[cfg(not(feature = "timing"))]
mod disabled {
    use super::{DefaultProfiler, Pass, Profiler};
    use alloc::boxed::Box;
    use core::any::Any;

    impl Profiler for DefaultProfiler {
        fn start_pass(&self, _pass: Pass) -> Box<dyn Any> {
            Box::new(())
        }
    }

    pub(crate) fn start_pass(_pass: Pass) -> Box<dyn Any> {
        Box::new(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn display() {
        assert_eq!(Pass::None.to_string(), "<no pass>");
        assert_eq!(Pass::regalloc.to_string(), "Register allocation");
    }
}
