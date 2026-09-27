//! Criterion measurements beyond wall time, and a way to report several of
//! them from one benchmark run.
//!
//! Criterion analyzes exactly one scalar per benchmark. [`BenchMeasurement`]
//! wraps that *primary* measurement together with any number of *secondary*
//! measurements taken over the same iterations, whose per-iteration values a
//! benchmark harness records into a [`MetricReport`]. The measurements here
//! are process- or thread-scoped counters: Linux hardware performance
//! counters ([`HardwareCounter`]) and `getrusage` statistics
//! ([`ResourceCounter`]). Nothing in this crate depends on GPUI, so any
//! Criterion benchmark can use it:
//!
//! ```ignore
//! fn benches(c: &mut Criterion<bench_metrics::BenchMeasurement>) {
//!     let report = bench_metrics::MetricReport::new();
//!     c.bench_function("append", |b| report.iter(b, || work()));
//!     report.print("  ");
//! }
//!
//! criterion_group! {
//!     name = group;
//!     config = Criterion::default()
//!         .with_measurement(bench_metrics::BenchMeasurement::from_env_or_exit());
//!     targets = benches
//! }
//! criterion_main!(group);
//! ```

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use anyhow::{Result, anyhow};
use criterion::measurement::{Measurement, ValueFormatter};

/// The environment variable [`BenchMeasurement::from_env`] reads.
pub const BENCH_MEASUREMENT_ENV_VAR: &str = "BENCH_MEASUREMENT";

/// Which threads a counter covers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetricScope {
    /// Every thread in the benchmark process, including threads created
    /// after the counter was opened.
    Process,
    /// Only the thread that constructed the counter. In a Criterion benchmark
    /// that is the thread running the benchmark routine; in GPUI benchmarks it
    /// is also the foreground thread where task polls, layout, and paint run.
    CallingThread,
}

impl MetricScope {
    fn label(self) -> &'static str {
        match self {
            Self::Process => "the benchmark process",
            Self::CallingThread => "the calling thread",
        }
    }
}

/// Formats event counts with K/M/G prefixes and a unit noun.
#[derive(Clone, Copy)]
pub struct CountFormatter {
    unit: &'static str,
}

impl CountFormatter {
    /// Creates a formatter whose values are counts of `unit`, e.g. `"cycles"`.
    pub const fn new(unit: &'static str) -> Self {
        Self { unit }
    }
}

impl ValueFormatter for CountFormatter {
    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str {
        let (scale, prefix) = if typical_value < 1_000.0 {
            (1.0, "")
        } else if typical_value < 1_000_000.0 {
            (1_000.0, "K ")
        } else if typical_value < 1_000_000_000.0 {
            (1_000_000.0, "M ")
        } else {
            (1_000_000_000.0, "G ")
        };
        for value in values {
            *value /= scale;
        }
        // `&'static str` is required, so the prefixed unit is interned once.
        intern(prefix, self.unit)
    }

    fn scale_throughputs(
        &self,
        _typical_value: f64,
        throughput: &criterion::Throughput,
        values: &mut [f64],
    ) -> &'static str {
        let (units, per) = match throughput {
            criterion::Throughput::Bits(units) => (*units, "/bit"),
            criterion::Throughput::Bytes(units) | criterion::Throughput::BytesDecimal(units) => {
                (*units, "/byte")
            }
            criterion::Throughput::Elements(units)
            | criterion::Throughput::ElementsAndBytes {
                elements: units, ..
            } => (*units, "/element"),
        };
        for value in values {
            *value /= units as f64;
        }
        intern(self.unit, per)
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        self.unit
    }
}

/// Returns a `'static` concatenation of two strings, leaking each distinct
/// combination once. Criterion's `ValueFormatter` requires `&'static str`
/// units, and the set of combinations is a handful of fixed labels.
fn intern(first: &str, second: &str) -> &'static str {
    use std::sync::{Mutex, OnceLock};

    static INTERNED: OnceLock<Mutex<Vec<&'static str>>> = OnceLock::new();
    let combined = format!("{first}{second}");
    let mut interned = INTERNED
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = interned.iter().find(|existing| **existing == combined) {
        return existing;
    }
    let leaked: &'static str = Box::leak(combined.into_boxed_str());
    interned.push(leaked);
    leaked
}

// ---------------------------------------------------------------------------
// Linux hardware performance counters
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
pub use linux_perf::{CounterSnapshot, HardwareCounter, HardwareEvent};

/// Hardware performance counters read through Linux `perf_event_open`.
///
/// Everything here is Linux-only; the public types are re-exported at the
/// crate root. Items from the `perf-event2` crate are referenced through the
/// `perf_event` path so this module's own helpers are distinguishable from
/// the crate's.
#[cfg(target_os = "linux")]
mod linux_perf {
    use super::*;
    use crate::linux_perf;

    /// A hardware event counted by [`HardwareCounter`].
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum HardwareEvent {
        /// Retired userspace instructions: near-deterministic, so small
        /// regressions are detectable, but blind to waits and memory stalls.
        Instructions,
        /// Userspace CPU cycles. `instructions / cycles` is IPC; a falling IPC with
        /// flat instructions means worse cache or branch behavior.
        Cycles,
        /// Mispredicted branches.
        BranchMisses,
        /// Last-level cache misses, i.e. accesses served from memory.
        CacheMisses,
        /// Last-level cache accesses.
        CacheReferences,
    }

    impl HardwareEvent {
        fn generic(self) -> perf_event::events::Hardware {
            use perf_event::events::Hardware;
            match self {
                Self::Instructions => Hardware::INSTRUCTIONS,
                Self::Cycles => Hardware::CPU_CYCLES,
                Self::BranchMisses => Hardware::BRANCH_MISSES,
                Self::CacheMisses => Hardware::CACHE_MISSES,
                Self::CacheReferences => Hardware::CACHE_REFERENCES,
            }
        }

        /// The event's name under `/sys/bus/event_source/devices/<pmu>/events`.
        fn sysfs_name(self) -> &'static str {
            match self {
                Self::Instructions => "instructions",
                Self::Cycles => "cpu-cycles",
                Self::BranchMisses => "branch-misses",
                Self::CacheMisses => "cache-misses",
                Self::CacheReferences => "cache-references",
            }
        }

        /// The unit noun used in reports.
        pub fn unit(self) -> &'static str {
            match self {
                Self::Instructions => "instructions",
                Self::Cycles => "cycles",
                Self::BranchMisses => "branch misses",
                Self::CacheMisses => "cache misses",
                Self::CacheReferences => "cache references",
            }
        }
    }

    /// Criterion measurement for a Linux hardware performance counter, read
    /// through `perf_event_open`.
    ///
    /// Counters are opened once, at construction, and left enabled; each
    /// `Measurement::start`/`end` reads them and reports the difference, so the
    /// measured interval costs one `read` syscall per counter (a few hundred
    /// nanoseconds) rather than reopening counters per sample.
    ///
    /// With [`MetricScope::Process`], one inherited counter is opened for every
    /// thread that exists at construction. Inheritance extends each counter to the
    /// threads that thread later creates, and live descendants are summed on read,
    /// so the whole process is covered for the counter's lifetime. With
    /// [`MetricScope::CallingThread`], a single non-inherited counter observes the
    /// constructing thread only.
    ///
    /// Hybrid CPUs (separate performance and efficiency core PMUs) need one event
    /// per PMU, of which only the one matching the CPU a thread runs on can be
    /// scheduled; each PMU's encoding is read from sysfs. If the kernel multiplexes
    /// a conventional counter with other profiling, its count is scaled by
    /// `time_enabled / time_running` and a warning is printed once.
    pub struct HardwareCounter {
        event: HardwareEvent,
        scope: MetricScope,
        counters: RefCell<Vec<PerfCounter>>,
        formatter: CountFormatter,
        multiplexing_reported: Cell<bool>,
    }

    struct PerfCounter {
        counter: perf_event::Counter,
        /// Generic events on conventional CPUs are scaled when multiplexed. Hybrid
        /// per-PMU events are not, since a thread on the other core type shows as
        /// "not running" for that PMU by design.
        scale_for_multiplexing: bool,
    }

    /// One counter's state at `Measurement::start`.
    #[doc(hidden)]
    #[derive(Clone, Copy)]
    pub struct CounterSnapshot {
        count: u64,
        time_enabled: std::time::Duration,
        time_running: std::time::Duration,
    }

    impl HardwareCounter {
        /// Opens counters for `event` over `scope`.
        ///
        /// Fails with an actionable error when the kernel's perf security policy
        /// denies access, or with `NotFound`-style errors when the CPU or
        /// virtual machine does not expose the event.
        pub fn new(event: HardwareEvent, scope: MetricScope) -> Result<Self> {
            let hybrid_events = linux_perf::hybrid_pmu_events(event)?;
            let mut counters = Vec::new();
            match scope {
                MetricScope::CallingThread => {
                    Self::open(
                        event,
                        CounterTarget::CallingThread,
                        &hybrid_events,
                        &mut counters,
                    )?;
                }
                MetricScope::Process => {
                    for thread_id in linux_perf::process_thread_ids()? {
                        Self::open(
                            event,
                            CounterTarget::ThreadAndDescendants(thread_id),
                            &hybrid_events,
                            &mut counters,
                        )?;
                    }
                }
            }
            if counters.is_empty() {
                return Err(anyhow!(
                    "no Linux {} counter could be opened for {}: the CPU does not expose this event",
                    event.sysfs_name(),
                    scope.label()
                ));
            }
            for counter in &mut counters {
                counter
                    .counter
                    .enable()
                    .map_err(|error| Self::operation_error(event, "enable", error))?;
            }
            let this = Self {
                event,
                scope,
                counters: RefCell::new(counters),
                formatter: CountFormatter::new(event.unit()),
                multiplexing_reported: Cell::new(false),
            };
            this.verify_scheduled()?;
            Ok(this)
        }

        /// The counted event.
        pub fn event(&self) -> HardwareEvent {
            self.event
        }

        /// The covered threads.
        pub fn scope(&self) -> MetricScope {
            self.scope
        }

        /// Confirms the PMU actually schedules at least one counter. A virtual
        /// machine can accept `perf_event_open` yet never run the event; failing
        /// here is clearer than measuring zeros.
        fn verify_scheduled(&self) -> Result<()> {
            let before = self.snapshot();
            std::hint::black_box(
                (0..10_000_u64).fold(0_u64, |total, value| total.wrapping_add(value)),
            );
            let after = self.snapshot();
            let ran = before
                .iter()
                .zip(&after)
                .any(|(before, after)| after.time_running > before.time_running);
            if ran {
                Ok(())
            } else {
                Err(anyhow!(
                    "Linux opened the {} counters but the PMU never ran them; ensure hardware \
                     performance counters are available to this machine or CI runner",
                    self.event.sysfs_name()
                ))
            }
        }

        fn open(
            event: HardwareEvent,
            target: CounterTarget,
            hybrid_events: &[(u32, u64)],
            counters: &mut Vec<PerfCounter>,
        ) -> Result<()> {
            let opened: Vec<(std::io::Result<perf_event::Counter>, bool)> =
                if hybrid_events.is_empty() {
                    let mut builder = perf_event::Builder::new(event.generic());
                    target.configure(&mut builder);
                    vec![(builder.build(), true)]
                } else {
                    hybrid_events
                        .iter()
                        .map(|&(pmu_type, config)| {
                            let mut builder =
                                perf_event::Builder::new(perf_event::events::Raw::new(config));
                            builder.attrs_mut().type_ = pmu_type;
                            target.configure(&mut builder);
                            (builder.build(), false)
                        })
                        .collect()
                };
            for (result, scale_for_multiplexing) in opened {
                match result {
                    Ok(counter) => counters.push(PerfCounter {
                        counter,
                        scale_for_multiplexing,
                    }),
                    // The thread exited between enumeration and open.
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => return Err(Self::open_error(event, target, error)),
                }
            }
            Ok(())
        }

        fn open_error(
            event: HardwareEvent,
            target: CounterTarget,
            error: std::io::Error,
        ) -> anyhow::Error {
            anyhow!(
                "failed to open Linux {} counter for {target}: {error}. Grant this benchmark \
                 CAP_PERFMON or adjust /proc/sys/kernel/perf_event_paranoid according to your CI \
                 security policy",
                event.sysfs_name()
            )
        }

        fn operation_error(
            event: HardwareEvent,
            operation: &str,
            error: std::io::Error,
        ) -> anyhow::Error {
            anyhow!(
                "Linux {} counter {operation} failed: {error}",
                event.sysfs_name()
            )
        }

        fn snapshot(&self) -> Vec<CounterSnapshot> {
            self.counters
                .borrow_mut()
                .iter_mut()
                .map(|counter| {
                    let data = counter.counter.read_full().unwrap_or_else(|error| {
                        panic!("{}", Self::operation_error(self.event, "read", error))
                    });
                    CounterSnapshot {
                        count: data.count(),
                        time_enabled: data
                            .time_enabled()
                            .expect("time-enabled counter data was requested"),
                        time_running: data
                            .time_running()
                            .expect("time-running counter data was requested"),
                    }
                })
                .collect()
        }
    }

    impl Measurement for HardwareCounter {
        type Intermediate = Vec<CounterSnapshot>;
        type Value = f64;

        fn start(&self) -> Self::Intermediate {
            self.snapshot()
        }

        fn end(&self, start: Self::Intermediate) -> Self::Value {
            let end = self.snapshot();
            let counters = self.counters.borrow();
            let mut total = 0.0;
            let mut any_counter_ran = false;
            for ((counter, start), end) in counters.iter().zip(&start).zip(&end) {
                let count = end.count.saturating_sub(start.count) as f64;
                let time_enabled = end.time_enabled.saturating_sub(start.time_enabled);
                let time_running = end.time_running.saturating_sub(start.time_running);
                if time_running.is_zero() {
                    // Process-wide coverage includes threads that slept for the
                    // whole interval, and hybrid PMUs whose core type this thread
                    // never ran on.
                    continue;
                }
                any_counter_ran = true;
                if counter.scale_for_multiplexing && time_running < time_enabled {
                    if !self.multiplexing_reported.replace(true) {
                        eprintln!(
                            "{} counters were multiplexed by the kernel; counts are scaled using \
                             time_enabled/time_running",
                            self.event.sysfs_name()
                        );
                    }
                    total += count * time_enabled.as_secs_f64() / time_running.as_secs_f64();
                } else {
                    total += count;
                }
            }
            if !any_counter_ran {
                panic!(
                    "Linux {} counters did not run during the measured interval; ensure hardware \
                     performance counters are available to this machine or CI runner",
                    self.event.sysfs_name()
                );
            }
            total
        }

        fn add(&self, first: &Self::Value, second: &Self::Value) -> Self::Value {
            first + second
        }

        fn zero(&self) -> Self::Value {
            0.0
        }

        fn to_f64(&self, value: &Self::Value) -> f64 {
            *value
        }

        fn formatter(&self) -> &dyn ValueFormatter {
            &self.formatter
        }
    }

    #[derive(Clone, Copy)]
    enum CounterTarget {
        /// `pid = 0` to `perf_event_open`: the calling thread, with no inheritance.
        CallingThread,
        /// A specific thread plus every thread it creates afterwards.
        ThreadAndDescendants(i32),
    }

    impl CounterTarget {
        fn configure(self, builder: &mut perf_event::Builder) {
            use perf_event::ReadFormat;

            match self {
                Self::CallingThread => builder.observe_self().inherit(false),
                Self::ThreadAndDescendants(thread_id) => {
                    builder.observe_pid(thread_id).inherit(true)
                }
            };
            builder.read_format(ReadFormat::TOTAL_TIME_ENABLED | ReadFormat::TOTAL_TIME_RUNNING);
        }
    }

    impl std::fmt::Display for CounterTarget {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::CallingThread => write!(f, "the calling thread"),
                Self::ThreadAndDescendants(thread_id) => write!(f, "thread {thread_id}"),
            }
        }
    }

    fn process_thread_ids() -> Result<Vec<i32>> {
        let mut thread_ids = Vec::new();
        for entry in std::fs::read_dir("/proc/self/task")
            .map_err(|error| anyhow!("failed to enumerate benchmark process threads: {error}"))?
        {
            let entry = entry.map_err(|error| {
                anyhow!("failed to enumerate a benchmark process thread: {error}")
            })?;
            if let Some(thread_id) = entry
                .file_name()
                .to_str()
                .and_then(|thread_id| thread_id.parse().ok())
            {
                thread_ids.push(thread_id);
            }
        }
        Ok(thread_ids)
    }

    /// Returns `(pmu_type, config)` for `event` on each hybrid CPU PMU, or an
    /// empty list on conventional CPUs where the generic hardware event suffices.
    ///
    /// A PMU (performance monitoring unit) is the per-core block of counter
    /// registers; Linux exposes each as a device under
    /// `/sys/bus/event_source/devices/`. Conventional CPUs have one, `cpu`, and
    /// the generic events target it. Hybrid CPUs have `cpu_core` (performance
    /// cores) and `cpu_atom` (efficiency cores) with distinct event encodings,
    /// and the kernel refuses the generic event for a task that may run on
    /// either, so one raw event per PMU is needed.
    ///
    /// Each PMU publishes its events as `term=value,...` strings and the bit
    /// layout of each term under `format/`, e.g. `cache-misses` is
    /// `event=0x2e,umask=0x41` with `event` at `config:0-7` and `umask` at
    /// `config:8-15`.
    fn hybrid_pmu_events(event: HardwareEvent) -> Result<Vec<(u32, u64)>> {
        use anyhow::Context as _;

        let mut events = Vec::new();
        for pmu in ["cpu_core", "cpu_atom"] {
            let pmu_path = std::path::Path::new("/sys/bus/event_source/devices").join(pmu);
            if !pmu_path.exists() {
                continue;
            }
            let pmu_type = std::fs::read_to_string(pmu_path.join("type"))
                .with_context(|| format!("failed to read Linux {pmu} performance-counter type"))?
                .trim()
                .parse::<u32>()
                .with_context(|| format!("invalid Linux {pmu} performance-counter type"))?;
            let encoding =
                std::fs::read_to_string(pmu_path.join("events").join(event.sysfs_name()))
                    .with_context(|| {
                        format!("failed to read Linux {pmu} {} event", event.sysfs_name())
                    })?;
            let mut config = 0_u64;
            for term in encoding.trim().split(',') {
                let (name, value) = term.split_once('=').unwrap_or((term, "1"));
                let value = value
                    .strip_prefix("0x")
                    .map_or_else(|| value.parse::<u64>(), |hex| u64::from_str_radix(hex, 16))
                    .with_context(|| {
                        format!(
                            "invalid Linux {pmu} {} event term {term:?}",
                            event.sysfs_name()
                        )
                    })?;
                let layout = std::fs::read_to_string(pmu_path.join("format").join(name))
                    .with_context(|| format!("failed to read Linux {pmu} format for {name:?}"))?;
                let bits = layout.trim().strip_prefix("config:").ok_or_else(|| {
                    anyhow!(
                        "unsupported Linux {pmu} format {:?} for {name:?}",
                        layout.trim()
                    )
                })?;
                let (low, high) = match bits.split_once('-') {
                    Some((low, high)) => (low.parse::<u32>()?, high.parse::<u32>()?),
                    None => {
                        let bit = bits.parse::<u32>()?;
                        (bit, bit)
                    }
                };
                let width = high - low + 1;
                let mask = if width >= 64 {
                    u64::MAX
                } else {
                    (1_u64 << width) - 1
                };
                config |= (value & mask) << low;
            }
            events.push((pmu_type, config));
        }
        Ok(events)
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn perf_permission_error_is_actionable() {
            let error = HardwareCounter::open_error(
                HardwareEvent::Instructions,
                CounterTarget::ThreadAndDescendants(42),
                std::io::Error::from(std::io::ErrorKind::PermissionDenied),
            );
            let message = error.to_string();
            assert!(message.contains("CAP_PERFMON"));
            assert!(message.contains("perf_event_paranoid"));
            assert!(message.contains("thread 42"));
        }
    }
}

// ---------------------------------------------------------------------------
// getrusage statistics
// ---------------------------------------------------------------------------

/// A statistic reported by `getrusage`, counted by [`ResourceCounter`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceMetric {
    /// Times the thread blocked and gave up the CPU: lock waits, channel
    /// receives, sleeps, and I/O. Preemptions are not included, since they
    /// reflect the machine rather than the code.
    VoluntaryContextSwitches,
    /// Page faults served without disk I/O: first touches of freshly mapped
    /// memory, a privilege-free proxy for allocation pressure.
    MinorPageFaults,
}

impl ResourceMetric {
    /// The unit noun used in reports.
    pub fn unit(self) -> &'static str {
        match self {
            Self::VoluntaryContextSwitches => "context switches",
            Self::MinorPageFaults => "page faults",
        }
    }

    #[cfg(unix)]
    fn read(self, usage: &libc::rusage) -> u64 {
        let value = match self {
            Self::VoluntaryContextSwitches => usage.ru_nvcsw,
            Self::MinorPageFaults => usage.ru_minflt,
        };
        value.max(0) as u64
    }
}

/// Criterion measurement for a `getrusage` statistic.
///
/// Unlike hardware counters this needs no privileges and works on every Unix.
/// [`MetricScope::CallingThread`] uses `RUSAGE_THREAD`, which only Linux
/// provides; [`MetricScope::Process`] uses `RUSAGE_SELF`, which sums every
/// thread in the process.
pub struct ResourceCounter {
    metric: ResourceMetric,
    scope: MetricScope,
    formatter: CountFormatter,
}

impl ResourceCounter {
    /// Creates a counter for `metric` over `scope`.
    pub fn new(metric: ResourceMetric, scope: MetricScope) -> Result<Self> {
        let this = Self {
            metric,
            scope,
            formatter: CountFormatter::new(metric.unit()),
        };
        this.read()?;
        Ok(this)
    }

    /// The counted statistic.
    pub fn metric(&self) -> ResourceMetric {
        self.metric
    }

    /// The covered threads.
    pub fn scope(&self) -> MetricScope {
        self.scope
    }

    #[cfg(unix)]
    fn read(&self) -> Result<u64> {
        let who = match self.scope {
            MetricScope::Process => libc::RUSAGE_SELF,
            #[cfg(target_os = "linux")]
            MetricScope::CallingThread => libc::RUSAGE_THREAD,
            #[cfg(not(target_os = "linux"))]
            MetricScope::CallingThread => {
                return Err(anyhow!(
                    "getrusage statistics for {} need RUSAGE_THREAD, which only Linux provides",
                    self.scope.label()
                ));
            }
        };
        // SAFETY: `rusage` is plain old data that `getrusage` fully initializes
        // on success, and `who` is one of the constants the call accepts.
        let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
        let status = unsafe { libc::getrusage(who, &mut usage) };
        if status != 0 {
            return Err(anyhow!(
                "getrusage failed: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(self.metric.read(&usage))
    }

    #[cfg(not(unix))]
    fn read(&self) -> Result<u64> {
        Err(anyhow!(
            "getrusage statistics for {} are unavailable on this platform",
            self.scope.label()
        ))
    }
}

impl Measurement for ResourceCounter {
    type Intermediate = u64;
    type Value = f64;

    fn start(&self) -> Self::Intermediate {
        self.read().unwrap_or_else(|error| panic!("{error}"))
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        let end = self.read().unwrap_or_else(|error| panic!("{error}"));
        end.saturating_sub(start) as f64
    }

    fn add(&self, first: &Self::Value, second: &Self::Value) -> Self::Value {
        first + second
    }

    fn zero(&self) -> Self::Value {
        0.0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &self.formatter
    }
}

// ---------------------------------------------------------------------------
// Composite measurement
// ---------------------------------------------------------------------------

/// A Criterion measurement that records several metrics per sample.
///
/// Criterion analyzes one scalar per benchmark, fixed in the `Criterion<M>`
/// type. This type erases the concrete [`Measurement`] Criterion analyzes (the
/// *primary*) so benchmark code needs no type parameter, and takes any number
/// of *secondary* measurements over the same iterations. A harness records
/// their per-iteration values into a [`MetricReport`]; see
/// [`MetricReport::iter`] for plain Criterion benchmarks.
///
/// Every measurement already reduces to `f64` for Criterion's statistics, so
/// the erased value is that `f64` and each measurement's own formatter still
/// labels it.
pub struct BenchMeasurement {
    primary: Box<dyn ErasedMeasurement>,
    secondaries: Rc<SecondaryMeasurements>,
}

impl BenchMeasurement {
    /// Wraps the Criterion measurement whose values Criterion analyzes.
    pub fn new<M>(primary: M) -> Self
    where
        M: Measurement + 'static,
        M::Intermediate: 'static,
    {
        Self {
            primary: Box::new(ErasedMeasurementCell::new(primary)),
            secondaries: Rc::new(SecondaryMeasurements::default()),
        }
    }

    /// Adds a measurement taken alongside the primary on every sample.
    ///
    /// `name` labels the metric in reports, e.g. `"instructions"`.
    /// Secondaries start before and end after the primary, so their own
    /// readout stays outside the primary's measured interval.
    pub fn with_secondary<M>(mut self, name: &'static str, measurement: M) -> Self
    where
        M: Measurement + 'static,
        M::Intermediate: 'static,
    {
        self.secondaries_mut()
            .metrics
            .push(Rc::new(SecondaryMetric {
                name,
                measurement: Box::new(ErasedMeasurementCell::new(measurement)),
                total: Cell::new(0.0),
            }));
        self
    }

    /// Adds `name` to reports as the ratio of two secondaries' totals over
    /// each sample, e.g. `with_ratio("IPC", "instructions", "cycles")`.
    /// Unknown names are reported as unavailable rather than failing, so a
    /// ratio can be declared even when one input was skipped on this machine.
    pub fn with_ratio(
        mut self,
        name: &'static str,
        numerator: &'static str,
        denominator: &'static str,
    ) -> Self {
        self.secondaries_mut().ratios.push(RatioMetric {
            name,
            numerator,
            denominator,
        });
        self
    }

    fn secondaries_mut(&mut self) -> &mut SecondaryMeasurements {
        Rc::get_mut(&mut self.secondaries)
            .expect("secondaries are only shared once measurement starts")
    }

    /// Returns the measurement configured by [`BENCH_MEASUREMENT_ENV_VAR`].
    ///
    /// * unset: Criterion analyzes wall time; every counter this machine
    ///   supports is a secondary. On Linux with hardware counters available
    ///   that is process and calling-thread instructions, cycles, branch
    ///   misses, and cache misses; everywhere, voluntary context switches and
    ///   minor page faults from `getrusage`. Counters that fail to open are
    ///   skipped with a note printed once.
    /// * `wall-time`: wall time only, no counters.
    /// * `instructions`: Criterion analyzes process-wide retired instructions
    ///   and wall time becomes a secondary. Fails when counters are
    ///   unavailable so CI does not silently measure something else.
    pub fn from_env() -> Result<Self> {
        match std::env::var(BENCH_MEASUREMENT_ENV_VAR).as_deref() {
            Err(std::env::VarError::NotPresent) => {
                let measurement = Self::new(criterion::measurement::WallTime);
                Ok(measurement.with_default_secondaries(false))
            }
            Ok("wall-time") => Ok(Self::new(criterion::measurement::WallTime)),
            Ok("instructions") => {
                #[cfg(target_os = "linux")]
                {
                    let instructions =
                        HardwareCounter::new(HardwareEvent::Instructions, MetricScope::Process)?;
                    Ok(Self::new(instructions)
                        .with_default_secondaries(true)
                        .with_secondary("wall time", criterion::measurement::WallTime))
                }
                #[cfg(not(target_os = "linux"))]
                {
                    Err(anyhow!(
                        "retired-instruction benchmarks require Linux perf_event_open; run with \
                         {BENCH_MEASUREMENT_ENV_VAR}=wall-time on this platform"
                    ))
                }
            }
            Ok(value) => Err(anyhow!(
                "unsupported {BENCH_MEASUREMENT_ENV_VAR} value {value:?}; expected \"wall-time\" \
                 or \"instructions\""
            )),
            Err(error) => Err(anyhow!(
                "{BENCH_MEASUREMENT_ENV_VAR} is not valid Unicode: {error}"
            )),
        }
    }

    /// [`Self::from_env`], exiting the process with an actionable message when
    /// the requested measurement can't be used on this machine. For use in
    /// `criterion_group!` `config` expressions, where there is nothing to
    /// propagate an error to.
    pub fn from_env_or_exit() -> Self {
        Self::from_env().unwrap_or_else(|error| {
            eprintln!("failed to select benchmark measurement: {error:#}");
            std::process::exit(2);
        })
    }

    /// Adds every counter this machine supports. `skip_process_instructions`
    /// leaves out the process-wide instruction count when it is already the
    /// primary.
    #[cfg_attr(not(target_os = "linux"), expect(unused_variables))]
    fn with_default_secondaries(mut self, skip_process_instructions: bool) -> Self {
        #[cfg(target_os = "linux")]
        {
            // Every hardware counter needs the same perf_event_open access, so
            // a denial on the first one is reported once for all of them
            // rather than once per event.
            let probe = if skip_process_instructions {
                Ok(None)
            } else {
                HardwareCounter::new(HardwareEvent::Instructions, MetricScope::Process).map(Some)
            };
            match probe {
                Err(error) => note_unavailable("hardware counters", &error),
                Ok(instructions) => {
                    if let Some(instructions) = instructions {
                        self = self.with_secondary("instructions", instructions);
                    }
                    let hardware =
                        |this: Self, name, event, scope| match HardwareCounter::new(event, scope) {
                            Ok(counter) => this.with_secondary(name, counter),
                            Err(error) => {
                                note_unavailable(name, &error);
                                this
                            }
                        };
                    self = hardware(
                        self,
                        "foreground instructions",
                        HardwareEvent::Instructions,
                        MetricScope::CallingThread,
                    );
                    self = hardware(self, "cycles", HardwareEvent::Cycles, MetricScope::Process);
                    self = hardware(
                        self,
                        "branch misses",
                        HardwareEvent::BranchMisses,
                        MetricScope::Process,
                    );
                    self = hardware(
                        self,
                        "cache misses",
                        HardwareEvent::CacheMisses,
                        MetricScope::Process,
                    );
                    self = self.with_ratio("IPC", "instructions", "cycles").with_ratio(
                        "foreground share of instructions",
                        "foreground instructions",
                        "instructions",
                    );
                }
            }
        }
        let resource = |this: Self, name, metric, scope| match ResourceCounter::new(metric, scope) {
            Ok(counter) => this.with_secondary(name, counter),
            Err(error) => {
                note_unavailable(name, &error);
                this
            }
        };
        self = resource(
            self,
            "foreground context switches",
            ResourceMetric::VoluntaryContextSwitches,
            MetricScope::CallingThread,
        );
        self = resource(
            self,
            "page faults",
            ResourceMetric::MinorPageFaults,
            MetricScope::Process,
        );
        self
    }
}

/// Prints why a default secondary is unavailable, once per metric per process,
/// so a machine without perf access reports it without spamming every group.
fn note_unavailable(name: &str, error: &anyhow::Error) {
    use std::sync::{Mutex, OnceLock};

    static NOTED: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    let mut noted = NOTED
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if noted.iter().any(|noted| noted == name) {
        return;
    }
    noted.push(name.to_owned());
    eprintln!("benchmark metric {name:?} unavailable on this machine: {error:#}");
}

/// Object-safe view of a Criterion measurement.
///
/// Criterion always pairs `start` with `end` on the same thread and never
/// nests them, so the intermediate value can be parked in the measurement
/// between the two calls instead of flowing through the generic
/// `Intermediate` type.
trait ErasedMeasurement {
    fn start(&self);
    fn end(&self) -> f64;
    fn formatter(&self) -> &dyn ValueFormatter;
}

struct ErasedMeasurementCell<M: Measurement> {
    measurement: M,
    intermediate: RefCell<Option<M::Intermediate>>,
}

impl<M: Measurement> ErasedMeasurementCell<M> {
    fn new(measurement: M) -> Self {
        Self {
            measurement,
            intermediate: RefCell::new(None),
        }
    }
}

impl<M: Measurement> ErasedMeasurement for ErasedMeasurementCell<M> {
    fn start(&self) {
        let previous = self
            .intermediate
            .borrow_mut()
            .replace(self.measurement.start());
        assert!(
            previous.is_none(),
            "Measurement::start called again before Measurement::end"
        );
    }

    fn end(&self) -> f64 {
        let intermediate = self
            .intermediate
            .borrow_mut()
            .take()
            .expect("Measurement::end called without a matching Measurement::start");
        self.measurement.to_f64(&self.measurement.end(intermediate))
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        self.measurement.formatter()
    }
}

/// A secondary metric and its total since the report last drained it.
struct SecondaryMetric {
    name: &'static str,
    measurement: Box<dyn ErasedMeasurement>,
    total: Cell<f64>,
}

struct RatioMetric {
    name: &'static str,
    numerator: &'static str,
    denominator: &'static str,
}

#[derive(Default)]
struct SecondaryMeasurements {
    metrics: Vec<Rc<SecondaryMetric>>,
    ratios: Vec<RatioMetric>,
}

impl SecondaryMeasurements {
    /// Returns each metric's total since the previous call and resets it.
    fn take_totals(&self) -> Vec<(Rc<SecondaryMetric>, f64)> {
        self.metrics
            .iter()
            .map(|metric| (metric.clone(), metric.total.replace(0.0)))
            .collect()
    }
}

thread_local! {
    /// The secondaries of the `BenchMeasurement` most recently started on this
    /// thread. Criterion owns the measurement and hands benchmark code only a
    /// `Bencher`, so this is how a harness reaches the secondary totals after
    /// Criterion's iteration loop returns.
    static ACTIVE_SECONDARIES: RefCell<Option<Rc<SecondaryMeasurements>>> =
        const { RefCell::new(None) };
}

fn active_secondaries() -> Option<Rc<SecondaryMeasurements>> {
    ACTIVE_SECONDARIES.with(|active| active.borrow().clone())
}

impl Measurement for BenchMeasurement {
    type Intermediate = ();
    type Value = f64;

    fn start(&self) -> Self::Intermediate {
        ACTIVE_SECONDARIES.with(|active| *active.borrow_mut() = Some(self.secondaries.clone()));
        for metric in &self.secondaries.metrics {
            metric.measurement.start();
        }
        self.primary.start();
    }

    fn end(&self, (): Self::Intermediate) -> Self::Value {
        let value = self.primary.end();
        for metric in self.secondaries.metrics.iter().rev() {
            metric
                .total
                .set(metric.total.get() + metric.measurement.end());
        }
        value
    }

    fn add(&self, first: &Self::Value, second: &Self::Value) -> Self::Value {
        first + second
    }

    fn zero(&self) -> Self::Value {
        0.0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        self.primary.formatter()
    }
}

// ---------------------------------------------------------------------------
// Report
// ---------------------------------------------------------------------------

/// Per-iteration values of every secondary metric and ratio, one entry per
/// Criterion sample.
#[derive(Clone, Default)]
pub struct MetricReport {
    samples: Rc<RefCell<Vec<MetricSamples>>>,
}

struct MetricSamples {
    name: &'static str,
    /// `None` for ratios, which are unitless.
    formatter: Option<Rc<SecondaryMetric>>,
    values: Vec<f64>,
}

impl MetricReport {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether any sample has been recorded.
    pub fn is_empty(&self) -> bool {
        self.samples.borrow().is_empty()
    }

    /// Returns every recorded per-iteration value (or ratio) for `name`, in
    /// sample order, for programmatic checks on top of the printed summary.
    pub fn values(&self, name: &str) -> Option<Vec<f64>> {
        self.samples
            .borrow()
            .iter()
            .find(|samples| samples.name == name)
            .map(|samples| samples.values.clone())
    }

    /// Records the per-iteration value of every active secondary metric for
    /// the Criterion sample that just finished, using the totals accumulated
    /// since the previous call, and each ratio of those totals.
    ///
    /// `iterations` is counted by the caller because Criterion's `Bencher`
    /// does not expose it, and `iter_batched` with `BatchSize::PerIteration`
    /// ends the measurement once per iteration rather than once per sample.
    pub fn record_sample(&self, iterations: u64) {
        let Some(secondaries) = active_secondaries() else {
            return;
        };
        let totals = secondaries.take_totals();
        if iterations == 0 {
            return;
        }
        let mut samples = self.samples.borrow_mut();
        for (metric, total) in &totals {
            Self::push(
                &mut samples,
                metric.name,
                Some(metric),
                total / iterations as f64,
            );
        }
        for ratio in &secondaries.ratios {
            let total_named = |name| {
                totals
                    .iter()
                    .find(|(metric, _)| metric.name == name)
                    .map(|(_, total)| *total)
            };
            if let (Some(numerator), Some(denominator)) =
                (total_named(ratio.numerator), total_named(ratio.denominator))
                && denominator > 0.0
            {
                Self::push(&mut samples, ratio.name, None, numerator / denominator);
            }
        }
    }

    fn push(
        samples: &mut Vec<MetricSamples>,
        name: &'static str,
        metric: Option<&Rc<SecondaryMetric>>,
        value: f64,
    ) {
        match samples.iter_mut().find(|samples| samples.name == name) {
            Some(samples) => samples.values.push(value),
            None => samples.push(MetricSamples {
                name,
                formatter: metric.cloned(),
                values: vec![value],
            }),
        }
    }

    /// Discards secondary totals accumulated outside a measured interval,
    /// e.g. by Criterion iterating a different benchmark on this thread.
    pub fn discard_pending(&self) {
        if let Some(secondaries) = active_secondaries() {
            secondaries.take_totals();
        }
    }

    /// `Bencher::iter` for plain Criterion benchmarks: runs `routine` under
    /// Criterion's loop and records this sample's secondary metrics.
    pub fn iter<O>(
        &self,
        bencher: &mut criterion::Bencher<'_, BenchMeasurement>,
        mut routine: impl FnMut() -> O,
    ) {
        self.discard_pending();
        let mut iterations = 0;
        bencher.iter(|| {
            iterations += 1;
            routine()
        });
        self.record_sample(iterations);
    }

    /// Prints one line per metric, each prefixed with `indent`, to stderr.
    pub fn print(&self, indent: &str) {
        for samples in self.samples.borrow().iter() {
            let mut sorted = samples.values.clone();
            sorted.sort_by(f64::total_cmp);
            let Some((&min, &max)) = sorted.first().zip(sorted.last()) else {
                continue;
            };
            let median = sorted[sorted.len() / 2];
            match &samples.formatter {
                Some(metric) => {
                    let mut scaled = [median, min, max];
                    let unit = metric
                        .measurement
                        .formatter()
                        .scale_values(median, &mut scaled);
                    let [median, min, max] = scaled;
                    eprintln!(
                        "{indent}{} per iteration: median {median:.3} {unit} (min {min:.3}, max \
                         {max:.3}, samples {})",
                        samples.name,
                        sorted.len()
                    );
                }
                None => eprintln!(
                    "{indent}{}: median {median:.3} (min {min:.3}, max {max:.3}, samples {})",
                    samples.name,
                    sorted.len()
                ),
            }
        }
    }
}

/// A deterministic measurement for tests: counts increments of a shared
/// counter between `start` and `end`.
#[cfg(any(test, feature = "test-support"))]
pub struct FakeCounter {
    counter: std::sync::Arc<std::sync::atomic::AtomicU64>,
    formatter: CountFormatter,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeCounter {
    /// Creates a measurement reading `counter`.
    pub fn new(counter: std::sync::Arc<std::sync::atomic::AtomicU64>) -> Self {
        Self {
            counter,
            formatter: CountFormatter::new("ticks"),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Measurement for FakeCounter {
    type Intermediate = u64;
    type Value = u64;

    fn start(&self) -> u64 {
        self.counter.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn end(&self, start: u64) -> u64 {
        self.counter.load(std::sync::atomic::Ordering::SeqCst) - start
    }

    fn add(&self, first: &u64, second: &u64) -> u64 {
        first + second
    }

    fn zero(&self) -> u64 {
        0
    }

    fn to_f64(&self, value: &u64) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &self.formatter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    };

    fn fast_criterion(measurement: BenchMeasurement) -> criterion::Criterion<BenchMeasurement> {
        criterion::Criterion::default()
            .with_measurement(measurement)
            .without_plots()
            .sample_size(10)
            .warm_up_time(std::time::Duration::from_millis(1))
            .measurement_time(std::time::Duration::from_millis(1))
    }

    #[test]
    fn erased_measurement_preserves_wrapped_values() {
        let counter = Arc::new(AtomicU64::new(0));
        let measurement = BenchMeasurement::new(FakeCounter::new(counter.clone()));

        let intermediate = measurement.start();
        counter.fetch_add(5, Ordering::SeqCst);
        assert_eq!(measurement.end(intermediate), 5.0);

        let intermediate = measurement.start();
        counter.fetch_add(3, Ordering::SeqCst);
        assert_eq!(measurement.end(intermediate), 3.0);
    }

    #[test]
    fn report_normalizes_secondaries_per_iteration_and_computes_ratios() {
        let primary = Arc::new(AtomicU64::new(0));
        let secondary = Arc::new(AtomicU64::new(0));
        let report = MetricReport::new();
        let mut criterion = fast_criterion(
            BenchMeasurement::new(FakeCounter::new(primary.clone()))
                .with_secondary("ticks", FakeCounter::new(secondary.clone()))
                .with_secondary("double ticks", FakeCounter::new(secondary.clone()))
                .with_ratio("tick ratio", "double ticks", "ticks")
                .with_ratio("unavailable ratio", "ticks", "missing"),
        );

        criterion.bench_function("report", |bencher| {
            // Fixture work outside the measured loop is not attributed to any
            // iteration.
            secondary.fetch_add(1_000_000, Ordering::SeqCst);
            report.iter(bencher, || {
                primary.fetch_add(1, Ordering::SeqCst);
                secondary.fetch_add(11, Ordering::SeqCst);
            });
        });

        let samples = report.samples.borrow();
        let values = |name: &str| {
            samples
                .iter()
                .find(|samples| samples.name == name)
                .map(|samples| samples.values.clone())
        };
        let ticks = values("ticks").expect("ticks were recorded");
        assert!(!ticks.is_empty());
        assert!(
            ticks.iter().all(|value| *value == 11.0),
            "each sample's total should be divided by its iteration count: {ticks:?}"
        );
        let ratio = values("tick ratio").expect("ratio was recorded");
        assert!(
            ratio.iter().all(|value| *value == 1.0),
            "ratio of identical counters should be 1: {ratio:?}"
        );
        assert!(values("unavailable ratio").is_none());
    }

    #[test]
    fn count_formatter_scales_with_prefixes() {
        let formatter = CountFormatter::new("cycles");
        let mut values = [2_500_000.0];
        assert_eq!(formatter.scale_values(2_500_000.0, &mut values), "M cycles");
        assert_eq!(values, [2.5]);
        let mut values = [12.0];
        assert_eq!(formatter.scale_values(12.0, &mut values), "cycles");
        assert_eq!(formatter.scale_for_machines(&mut values), "cycles");
    }

    #[cfg(unix)]
    #[test]
    fn resource_counter_counts_page_faults_and_context_switches() {
        let faults = ResourceCounter::new(ResourceMetric::MinorPageFaults, MetricScope::Process)
            .expect("RUSAGE_SELF is always available");
        let start = faults.start();
        // Touch fresh pages: a 4 MiB allocation faults in ~1000 pages.
        let buffer = vec![1_u8; 4 << 20];
        std::hint::black_box(&buffer);
        assert!(faults.end(start) > 100.0);

        #[cfg(target_os = "linux")]
        {
            let switches = ResourceCounter::new(
                ResourceMetric::VoluntaryContextSwitches,
                MetricScope::CallingThread,
            )
            .expect("RUSAGE_THREAD is available on Linux");
            let start = switches.start();
            for _ in 0..5 {
                std::thread::sleep(std::time::Duration::from_micros(100));
            }
            assert!(switches.end(start) >= 5.0);
        }
    }

    /// Returns a closure measuring a background thread spinning `iterations`
    /// times, or `None` when this machine has no usable hardware counters.
    #[cfg(target_os = "linux")]
    fn background_spin_instructions(scope: MetricScope) -> Option<impl Fn(u64) -> f64> {
        let counter = match HardwareCounter::new(HardwareEvent::Instructions, scope) {
            Ok(counter) => counter,
            Err(error) => {
                let message = error.to_string();
                assert!(
                    message.contains("CAP_PERFMON") || message.contains("PMU never ran"),
                    "unavailable counters should have an actionable error: {message}"
                );
                return None;
            }
        };
        Some(move |iterations: u64| {
            let start = counter.start();
            let worker = std::thread::spawn(move || {
                let mut total = 0_u64;
                for value in 0..iterations {
                    total = std::hint::black_box(total.wrapping_add(value));
                }
                total
            });
            std::hint::black_box(worker.join().expect("spin thread should finish"));
            counter.end(start)
        })
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn process_instructions_include_threads_spawned_after_open() {
        let Some(measure) = background_spin_instructions(MetricScope::Process) else {
            return;
        };
        let idle = measure(0);
        let busy = measure(1_000_000);
        assert!(
            busy > idle + 500_000.0,
            "work on a thread spawned after the counter opened should be counted: \
             idle={idle}, busy={busy}"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn calling_thread_instructions_exclude_other_threads() {
        let Some(measure) = background_spin_instructions(MetricScope::CallingThread) else {
            return;
        };
        let idle = measure(0);
        let busy = measure(1_000_000);
        assert!(
            busy < idle + 100_000.0,
            "another thread's work should not count toward the calling thread: \
             idle={idle}, busy={busy}"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn hardware_events_open_or_fail_actionably() {
        for event in [
            HardwareEvent::Cycles,
            HardwareEvent::BranchMisses,
            HardwareEvent::CacheMisses,
            HardwareEvent::CacheReferences,
        ] {
            match HardwareCounter::new(event, MetricScope::Process) {
                Ok(counter) => {
                    let start = counter.start();
                    std::hint::black_box((0..100_000_u64).fold(0_u64, |t, v| t.wrapping_add(v)));
                    let count = counter.end(start);
                    assert!(count >= 0.0);
                }
                Err(error) => {
                    let message = error.to_string();
                    assert!(
                        message.contains("CAP_PERFMON")
                            || message.contains("does not expose")
                            || message.contains("PMU never ran"),
                        "{event:?}: {message}"
                    );
                }
            }
        }
    }
}
