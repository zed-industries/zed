//! Samples the foreground thread's resource usage to explain its stalls.
//!
//! The journal records how long foreground work took, but not why. The
//! watchdog runs on the hang monitor's thread and reads the foreground's CPU
//! time, page faults, and run-queue delay while the foreground is working, so
//! each stall can be split into phases: running on a CPU, waiting on a CPU,
//! waiting on storage, blocked on something else, or frozen along with the
//! whole process. The foreground does no work for it beyond the activity
//! counter the journal maintains.
//!
//! Samples are coalesced into segments of one cause as they're taken, so a
//! stall of any length costs a handful of segments.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use scheduler::Instant;

use super::thread_usage::{ThreadUsage, ThreadUsageReader};
use super::{StallCause, StallPhase, StallProfile};
use crate::profiler::journal::ForegroundActivity;

/// How often the foreground is sampled while it's working. Stall phases
/// are resolved to about this precision.
const TICK: Duration = Duration::from_millis(50);

/// How long the foreground must stay idle before the watchdog stops ticking
/// and waits for the next turn to wake it instead. Waking costs the
/// foreground a syscall, so this keeps bursts of short turns from paying it.
const IDLE_AFTER: Duration = Duration::from_secs(1);

/// A tick this late, with the foreground making no progress, means the
/// whole process was stopped: a debugger, SIGSTOP, a paused VM, or App Nap.
/// Scheduling delays on a loaded machine are far shorter.
const FREEZE_LATENESS: Duration = Duration::from_secs(1);

/// Off-CPU time attributed to each major fault when deciding whether a
/// phase was spent paging. Reading a page back from storage takes between
/// tens of microseconds and tens of milliseconds, so this is a rough middle.
const PAGING_TIME_PER_FAULT: Duration = Duration::from_millis(5);

/// Segments retained. Idle time and runs of one cause each coalesce into a
/// single segment, so this spans far longer than any stall worth reporting.
const MAX_SEGMENTS: usize = 4096;

/// Phases reported per stall.
const MAX_PHASES: usize = 8;

/// Samples a foreground thread and profiles its stalls.
pub(crate) struct Watchdog {
    activity: Arc<ForegroundActivity>,
    reader: ThreadUsageReader,
    sampler: Sampler,
    /// When the current wait should end, if it's a timed tick.
    tick_due: Option<Instant>,
}

impl Watchdog {
    pub(crate) fn new(activity: Arc<ForegroundActivity>, reader: ThreadUsageReader) -> Self {
        let mut watchdog = Self {
            activity,
            reader,
            sampler: Sampler::default(),
            tick_due: None,
        };
        watchdog.sample(Instant::now());
        watchdog
    }

    /// Blocks until the next sample is due, the foreground begins working
    /// after a quiet period, the thread is unparked, or `deadline` passes.
    pub(crate) fn wait(&mut self, deadline: Instant) {
        let now = Instant::now();
        match self.sampler.last {
            Some(last) if self.sampler.is_quiet(now) => {
                self.tick_due = None;
                self.activity
                    .wait_while_idle(last.state, deadline.saturating_duration_since(now));
            }
            last => {
                let tick_due = last.map_or(now, |last| last.at + TICK).min(deadline);
                self.tick_due = Some(tick_due);
                std::thread::park_timeout(tick_due.saturating_duration_since(now));
            }
        }
    }

    /// Records the foreground's usage at `now`.
    pub(crate) fn sample(&mut self, now: Instant) {
        let state = self.activity.state();
        let Some(usage) = self.reader.read() else {
            return;
        };
        let lateness = self
            .tick_due
            .take()
            .map_or(Duration::ZERO, |due| now.saturating_duration_since(due));
        self.sampler.record(now, state, usage, lateness);
    }

    /// Profiles foreground work that ran from `start` to `end`. `presenting`
    /// marks work that submitted a frame, whose blocked time is attributed to
    /// the platform's presentation.
    pub(crate) fn profile(
        &self,
        start: Instant,
        end: Instant,
        presenting: bool,
    ) -> Option<StallProfile> {
        self.sampler.profile(start, end, presenting)
    }
}

#[derive(Clone, Copy)]
struct Sample {
    at: Instant,
    state: u64,
    usage: ThreadUsage,
}

/// Usage accrued over a span of time.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct UsageDelta {
    cpu: Duration,
    system_cpu: Option<Duration>,
    major_faults: Option<u64>,
    run_delay: Option<Duration>,
}

impl UsageDelta {
    fn between(before: &ThreadUsage, after: &ThreadUsage) -> Self {
        Self {
            cpu: after.cpu.saturating_sub(before.cpu),
            system_cpu: after
                .system_cpu
                .zip(before.system_cpu)
                .map(|(after, before)| after.saturating_sub(before)),
            major_faults: after
                .major_faults
                .zip(before.major_faults)
                .map(|(after, before)| after.saturating_sub(before)),
            run_delay: after
                .run_delay
                .zip(before.run_delay)
                .map(|(after, before)| after.saturating_sub(before)),
        }
    }

    fn add(&mut self, other: &Self) {
        self.cpu += other.cpu;
        self.system_cpu = self.system_cpu.zip(other.system_cpu).map(|(a, b)| a + b);
        self.major_faults = self
            .major_faults
            .zip(other.major_faults)
            .map(|(a, b)| a + b);
        self.run_delay = self.run_delay.zip(other.run_delay).map(|(a, b)| a + b);
    }
}

/// A span of time with one cause and the usage accrued over it.
#[derive(Debug, Clone, Copy)]
struct Segment {
    start: Instant,
    end: Instant,
    /// `None` while the foreground was idle throughout.
    cause: Option<StallCause>,
    usage: UsageDelta,
}

#[derive(Default)]
struct Sampler {
    last: Option<Sample>,
    /// When the activity state was last seen to change.
    last_change_at: Option<Instant>,
    segments: VecDeque<Segment>,
}

impl Sampler {
    /// Whether the foreground has been idle long enough to stop ticking.
    fn is_quiet(&self, now: Instant) -> bool {
        self.last
            .is_some_and(|last| !ForegroundActivity::is_working(last.state))
            && self
                .last_change_at
                .is_none_or(|changed| now.saturating_duration_since(changed) >= IDLE_AFTER)
    }

    /// Records a sample taken `lateness` after it was due.
    fn record(&mut self, at: Instant, state: u64, usage: ThreadUsage, lateness: Duration) {
        let sample = Sample { at, state, usage };
        let Some(previous) = self.last.replace(sample) else {
            self.last_change_at = Some(at);
            return;
        };
        if state != previous.state {
            self.last_change_at = Some(at);
        }
        let Some(elapsed) = at
            .checked_duration_since(previous.at)
            .filter(|d| !d.is_zero())
        else {
            return;
        };
        let idle = state == previous.state && !ForegroundActivity::is_working(state);
        let usage = UsageDelta::between(&previous.usage, &usage);
        let cause = (!idle).then(|| classify(elapsed, &usage, lateness));

        if let Some(last) = self.segments.back_mut()
            && last.cause == cause
            && last.end == previous.at
        {
            last.end = at;
            last.usage.add(&usage);
            return;
        }
        if self.segments.len() == MAX_SEGMENTS {
            self.segments.pop_front();
        }
        self.segments.push_back(Segment {
            start: previous.at,
            end: at,
            cause,
            usage,
        });
    }

    fn profile(&self, start: Instant, end: Instant, presenting: bool) -> Option<StallProfile> {
        let attribute = |cause: StallCause| match cause {
            StallCause::Blocked if presenting => StallCause::PresentBlocked,
            cause => cause,
        };
        let mut sampled = Duration::ZERO;
        let mut usage = UsageDelta {
            system_cpu: Some(Duration::ZERO),
            major_faults: Some(0),
            run_delay: Some(Duration::ZERO),
            ..UsageDelta::default()
        };
        let mut phases: Vec<(StallCause, Duration)> = Vec::new();
        for segment in &self.segments {
            let overlap_start = segment.start.max(start);
            let overlap_end = segment.end.min(end);
            let Some(overlap) = overlap_end
                .checked_duration_since(overlap_start)
                .filter(|overlap| !overlap.is_zero())
            else {
                continue;
            };
            // Usage within a segment is spread evenly over it.
            let fraction =
                overlap.as_secs_f64() / segment.end.duration_since(segment.start).as_secs_f64();
            usage.add(&UsageDelta {
                cpu: segment.usage.cpu.mul_f64(fraction),
                system_cpu: segment.usage.system_cpu.map(|time| time.mul_f64(fraction)),
                major_faults: segment
                    .usage
                    .major_faults
                    .map(|faults| (faults as f64 * fraction).round() as u64),
                run_delay: segment.usage.run_delay.map(|time| time.mul_f64(fraction)),
            });
            sampled += overlap;
            // Idle time can only overlap the edges of foreground work, and
            // reads as blocked there: the work began or ended mid-segment.
            let cause = attribute(segment.cause.unwrap_or(StallCause::Blocked));
            match phases.last_mut() {
                Some((last_cause, duration)) if *last_cause == cause => *duration += overlap,
                _ => phases.push((cause, overlap)),
            }
        }
        if sampled.is_zero() {
            return None;
        }

        let mut totals: Vec<(StallCause, Duration)> = Vec::new();
        for (cause, duration) in &phases {
            match totals
                .iter_mut()
                .find(|(total_cause, _)| total_cause == cause)
            {
                Some((_, total)) => *total += *duration,
                None => totals.push((*cause, *duration)),
            }
        }
        let cause = totals
            .iter()
            .max_by_key(|(_, total)| *total)
            .map_or(StallCause::Blocked, |(cause, _)| *cause);

        Some(StallProfile {
            cause,
            sampled_ms: as_millis(sampled),
            cpu_ms: as_millis(usage.cpu),
            system_cpu_ms: usage.system_cpu.map(as_millis),
            major_faults: usage.major_faults,
            run_delay_ms: usage.run_delay.map(as_millis),
            phases_elided: phases.len().saturating_sub(MAX_PHASES),
            phases: phases
                .into_iter()
                .take(MAX_PHASES)
                .map(|(cause, duration)| StallPhase {
                    cause,
                    duration_ms: as_millis(duration),
                })
                .collect(),
        })
    }
}

/// Why the foreground made the progress it did over `elapsed`, during which
/// it accrued `usage` and after which the sample arrived `lateness` late.
fn classify(elapsed: Duration, usage: &UsageDelta, lateness: Duration) -> StallCause {
    let off_cpu = elapsed.saturating_sub(usage.cpu);
    let waited_for_cpu = usage.run_delay.is_some_and(|delay| delay * 2 >= off_cpu);
    if lateness >= FREEZE_LATENESS && usage.cpu * 10 <= elapsed && !waited_for_cpu {
        StallCause::Frozen
    } else if usage.cpu * 2 >= elapsed {
        StallCause::CpuBound
    } else if waited_for_cpu {
        StallCause::CpuStarved
    } else if usage.major_faults.is_some_and(|faults| {
        faults > 0
            && PAGING_TIME_PER_FAULT.saturating_mul(u32::try_from(faults).unwrap_or(u32::MAX)) * 2
                >= off_cpu
    }) {
        StallCause::Paging
    } else {
        StallCause::Blocked
    }
}

fn as_millis(duration: Duration) -> f64 {
    duration.as_micros() as f64 / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDLE: u64 = 0;
    const WORKING: u64 = 1;

    #[test]
    fn a_stall_is_split_into_phases_by_cause() {
        let start = Instant::now();
        let at = |ms: u64| start + Duration::from_millis(ms);
        let mut sampler = Sampler::default();
        let mut usage = ThreadUsage {
            system_cpu: Some(Duration::ZERO),
            major_faults: Some(0),
            run_delay: Some(Duration::ZERO),
            ..ThreadUsage::default()
        };
        sampler.record(at(0), IDLE, usage, Duration::ZERO);
        // Woken as the turn begins, then 200 ms of computing...
        sampler.record(at(1), WORKING, usage, Duration::ZERO);
        for ms in (50..=200).step_by(50) {
            usage.cpu += Duration::from_millis(50);
            sampler.record(at(ms), WORKING, usage, Duration::ZERO);
        }
        // ...then 100 ms waiting on something with no CPU...
        for ms in [250, 300] {
            sampler.record(at(ms), WORKING, usage, Duration::ZERO);
        }
        // ...then 100 ms waiting for a CPU...
        for ms in [350, 400] {
            usage.run_delay = usage
                .run_delay
                .map(|delay| delay + Duration::from_millis(45));
            sampler.record(at(ms), WORKING, usage, Duration::ZERO);
        }
        // ...then 50 ms reading pages back in, before going idle.
        usage.major_faults = Some(8);
        sampler.record(at(450), IDLE, usage, Duration::ZERO);

        let profile = sampler
            .profile(at(1), at(450), false)
            .expect("the stall was sampled");
        let phases: Vec<(StallCause, f64)> = profile
            .phases
            .iter()
            .map(|phase| (phase.cause, phase.duration_ms))
            .collect();
        assert_eq!(
            phases,
            [
                (StallCause::CpuBound, 199.0),
                (StallCause::Blocked, 100.0),
                (StallCause::CpuStarved, 100.0),
                (StallCause::Paging, 50.0),
            ]
        );
        assert_eq!(profile.cause, StallCause::CpuBound);
        assert_eq!(profile.sampled_ms, 449.0);
        assert_eq!(profile.cpu_ms, 200.0);
        assert_eq!(profile.major_faults, Some(8));
        assert_eq!(profile.run_delay_ms, Some(90.0));
    }

    #[test]
    fn blocked_presentation_and_process_freezes_are_distinguished() {
        let start = Instant::now();
        let at = |ms: u64| start + Duration::from_millis(ms);
        let usage = ThreadUsage::default();
        let mut sampler = Sampler::default();
        sampler.record(at(0), WORKING, usage, Duration::ZERO);
        sampler.record(at(300), WORKING, usage, Duration::ZERO);
        // The next tick, due at 350 ms, arrived seconds late with no progress.
        sampler.record(at(3000), WORKING, usage, at(3000) - at(350));

        let blocked = sampler.profile(at(0), at(300), false).expect("sampled");
        assert_eq!(blocked.cause, StallCause::Blocked);
        let presenting = sampler.profile(at(0), at(300), true).expect("sampled");
        assert_eq!(presenting.cause, StallCause::PresentBlocked);
        let frozen = sampler.profile(at(300), at(3000), false).expect("sampled");
        assert_eq!(frozen.cause, StallCause::Frozen);
    }

    #[test]
    fn idle_time_coalesces_and_the_watchdog_goes_quiet() {
        let start = Instant::now();
        let at = |ms: u64| start + Duration::from_millis(ms);
        let usage = ThreadUsage::default();
        let mut sampler = Sampler::default();
        sampler.record(at(0), IDLE, usage, Duration::ZERO);
        assert!(!sampler.is_quiet(at(500)));
        for ms in (50..=1000).step_by(50) {
            sampler.record(at(ms), IDLE, usage, Duration::ZERO);
        }
        assert!(sampler.is_quiet(at(1000)));
        assert_eq!(sampler.segments.len(), 1);

        // Short turns between ticks keep it ticking.
        sampler.record(at(1050), 2, usage, Duration::ZERO);
        assert!(!sampler.is_quiet(at(1100)));
        assert_eq!(sampler.segments.len(), 2);
        assert!(sampler.profile(at(0), at(1000), false).is_some());
        assert!(sampler.profile(at(2000), at(3000), false).is_none());
    }
}
