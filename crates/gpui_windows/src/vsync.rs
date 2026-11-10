use std::{
    collections::VecDeque,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use gpui::FrameRequestSource;
use gpui_util::ResultExt;
use parking_lot::{Condvar, Mutex};
use windows::Win32::{
    Foundation::HWND,
    Graphics::Dwm::{DWM_TIMING_INFO, DwmFlush, DwmGetCompositionTimingInfo},
    System::Performance::QueryPerformanceFrequency,
};

static QPC_TICKS_PER_SECOND: LazyLock<u64> = LazyLock::new(|| {
    let mut frequency = 0;
    // On systems that run Windows XP or later, the function will always succeed and
    // will thus never return zero.
    unsafe { QueryPerformanceFrequency(&mut frequency).unwrap() };
    frequency as u64
});

const VSYNC_INTERVAL_THRESHOLD: Duration = Duration::from_millis(1);
const DEFAULT_VSYNC_INTERVAL: Duration = Duration::from_micros(16_666); // ~60Hz

/// Wakes the `VSyncProvider` thread when a window wants a frame.
///
/// The vsync thread only invalidates windows that asked for a frame, and sleeps here while no
/// window wants one, so an idle app neither waits on vblanks nor handles `WM_PAINT`.
pub(crate) struct FrameScheduler {
    demand: Mutex<bool>,
    condvar: Condvar,
}

impl FrameScheduler {
    pub(crate) fn new() -> Self {
        Self {
            demand: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }

    /// Wakes the vsync thread for at least one more vblank.
    pub(crate) fn wake(&self) {
        let mut demand = self.demand.lock();
        if !*demand {
            *demand = true;
            self.condvar.notify_all();
        }
    }

    /// Wakes a vsync thread so it notices that its `stop` flag was set.
    pub(crate) fn notify_stop(&self) {
        let _demand = self.demand.lock();
        self.condvar.notify_all();
    }

    /// Blocks until [`Self::wake`] is called, consuming the wake. Returns false, without consuming
    /// a wake, once `stop` is set: a newer vsync thread may be waiting on the same scheduler.
    pub(crate) fn wait(&self, stop: &AtomicBool) -> bool {
        let mut demand = self.demand.lock();
        loop {
            if stop.load(Ordering::Acquire) {
                return false;
            }
            if *demand {
                *demand = false;
                return true;
            }
            self.condvar.wait(&mut demand);
        }
    }
}

/// A window's requests for frames, shared between the window and the vsync thread.
pub(crate) struct WindowFrameRequest {
    /// Set when the window wants a frame on the next vblank; the vsync thread clears it when it
    /// invalidates the window.
    wanted: AtomicBool,
    /// Set from a frame request until the frame is drawn, on the UI thread.
    pending: AtomicBool,
    /// Frames are requested on every vblank until then, for GPUI's "keep presenting after
    /// high-rate input" logic, which only runs when a frame is requested.
    sustain_until: Mutex<Option<Instant>>,
    scheduler: Arc<FrameScheduler>,
}

impl WindowFrameRequest {
    pub(crate) fn new(scheduler: Arc<FrameScheduler>) -> Self {
        Self {
            wanted: AtomicBool::new(false),
            pending: AtomicBool::new(false),
            sustain_until: Mutex::new(None),
            scheduler,
        }
    }

    /// Asks for `WM_PAINT` on the next vblank.
    pub(crate) fn request(&self) {
        self.pending.store(true, Ordering::Relaxed);
        if !self.wanted.swap(true, Ordering::AcqRel) {
            self.scheduler.wake();
        }
    }

    /// Wakes the vsync thread without asking for a frame, so it checks for a lost device.
    pub(crate) fn wake_vsync(&self) {
        self.scheduler.wake();
    }

    /// Whether a frame was requested and not drawn yet.
    pub(crate) fn is_pending(&self) -> bool {
        self.pending.load(Ordering::Relaxed)
    }

    /// Called on the UI thread right before the window's frame callback runs. A request made by
    /// the callback itself (the window is still dirty, or animating) sets it again.
    pub(crate) fn begin_frame(&self) {
        self.pending.store(false, Ordering::Relaxed);
    }

    /// Asks for a frame on every vblank until `until`.
    pub(crate) fn sustain(&self, until: Instant) {
        let mut sustain_until = self.sustain_until.lock();
        if sustain_until.is_none_or(|current| current < until) {
            *sustain_until = Some(until);
        }
        drop(sustain_until);
        self.scheduler.wake();
    }

    /// Called by the vsync thread on each vblank. Returns whether to invalidate the window now,
    /// and whether it wants frames on the following vblanks too.
    pub(crate) fn take(&self, now: Instant) -> (bool, bool) {
        let wanted = self.wanted.swap(false, Ordering::AcqRel);
        let mut sustain_until = self.sustain_until.lock();
        let sustained = match *sustain_until {
            Some(until) if now < until => true,
            Some(_) => {
                *sustain_until = None;
                false
            }
            None => false,
        };
        (wanted || sustained, sustained)
    }
}

/// Detects input arriving at a high rate, as GPUI's own `InputRateTracker` does, so frames keep
/// being requested while GPUI keeps presenting after such input.
pub(crate) struct InputRateTracker {
    timestamps: VecDeque<Instant>,
}

impl InputRateTracker {
    const WINDOW: Duration = Duration::from_millis(100);
    const INPUTS_PER_SECOND: u128 = 60;
    pub(crate) const SUSTAIN: Duration = Duration::from_secs(1);

    pub(crate) fn new() -> Self {
        Self {
            timestamps: VecDeque::new(),
        }
    }

    /// Records an input that changed the window, and returns whether input now arrives at a
    /// high rate.
    pub(crate) fn record_input(&mut self, now: Instant) -> bool {
        self.timestamps.push_back(now);
        while let Some(&oldest) = self.timestamps.front() {
            if now.duration_since(oldest) > Self::WINDOW {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
        let min_events = Self::INPUTS_PER_SECOND * Self::WINDOW.as_millis() / 1000;
        self.timestamps.len() as u128 >= min_events
    }
}

pub(crate) struct VSyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool>,
}

impl VSyncProvider {
    pub(crate) fn new() -> Self {
        let interval = get_dwm_interval()
            .context("Failed to get DWM interval")
            .log_err()
            .unwrap_or(DEFAULT_VSYNC_INTERVAL);
        let f = Box::new(|| unsafe { DwmFlush().is_ok() });
        Self { interval, f }
    }

    pub(crate) fn wait_for_vsync(&self) -> FrameRequestSource {
        let vsync_start = Instant::now();
        let wait_succeeded = (self.f)();
        let elapsed = vsync_start.elapsed();
        // DwmFlush and DCompositionWaitForCompositorClock returns very early
        // instead of waiting until vblank when the monitor goes to sleep or is
        // unplugged (nothing to present due to desktop occlusion). We use 1ms as
        // a threshold for the duration of the wait functions and fallback to
        // Sleep() if it returns before that. This could happen during normal
        // operation for the first call after the vsync thread becomes non-idle,
        // but it shouldn't happen often.
        if !wait_succeeded || elapsed < VSYNC_INTERVAL_THRESHOLD {
            log::trace!("VSyncProvider::wait_for_vsync() took less time than expected");
            std::thread::sleep(self.interval);
            FrameRequestSource::LocalSchedule
        } else {
            FrameRequestSource::NativeCallback
        }
    }
}

fn get_dwm_interval() -> Result<Duration> {
    let mut timing_info = DWM_TIMING_INFO {
        cbSize: std::mem::size_of::<DWM_TIMING_INFO>() as u32,
        ..Default::default()
    };
    unsafe { DwmGetCompositionTimingInfo(HWND::default(), &mut timing_info) }?;
    let interval = retrieve_duration(timing_info.qpcRefreshPeriod, *QPC_TICKS_PER_SECOND);
    // Check for interval values that are impossibly low. A 29 microsecond
    // interval was seen (from a qpcRefreshPeriod of 60).
    if interval < VSYNC_INTERVAL_THRESHOLD {
        Ok(retrieve_duration(
            timing_info.rateRefresh.uiDenominator as u64,
            timing_info.rateRefresh.uiNumerator as u64,
        ))
    } else {
        Ok(interval)
    }
}

#[inline]
fn retrieve_duration(counts: u64, ticks_per_second: u64) -> Duration {
    let ticks_per_microsecond = ticks_per_second / 1_000_000;
    Duration::from_micros(counts / ticks_per_microsecond)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_compositor_wait_reports_native_callback() {
        let provider = VSyncProvider {
            interval: Duration::ZERO,
            f: Box::new(|| {
                std::thread::sleep(VSYNC_INTERVAL_THRESHOLD * 2);
                true
            }),
        };

        assert_eq!(
            provider.wait_for_vsync(),
            FrameRequestSource::NativeCallback
        );
    }

    #[test]
    fn failed_compositor_wait_reports_local_schedule_even_after_threshold() {
        let provider = VSyncProvider {
            interval: Duration::ZERO,
            f: Box::new(|| {
                std::thread::sleep(VSYNC_INTERVAL_THRESHOLD * 2);
                false
            }),
        };

        assert_eq!(provider.wait_for_vsync(), FrameRequestSource::LocalSchedule);
    }

    #[test]
    fn frame_request_is_served_once() {
        let scheduler = Arc::new(FrameScheduler::new());
        let request = WindowFrameRequest::new(scheduler.clone());
        let now = Instant::now();
        assert_eq!(request.take(now), (false, false));

        request.request();
        request.request();
        assert!(request.is_pending());
        // Both requests woke the vsync thread once.
        let stop = AtomicBool::new(false);
        assert!(scheduler.wait(&stop));
        assert!(!*scheduler.demand.lock());
        assert_eq!(request.take(now), (true, false));
        assert_eq!(request.take(now), (false, false));

        // Drawing clears the pending frame; a request made while drawing sets it again.
        request.begin_frame();
        assert!(!request.is_pending());
        request.request();
        assert!(request.is_pending());
    }

    #[test]
    fn sustained_frames_end_at_their_deadline() {
        let scheduler = Arc::new(FrameScheduler::new());
        let request = WindowFrameRequest::new(scheduler);
        let now = Instant::now();
        request.sustain(now + Duration::from_secs(1));
        // An earlier deadline doesn't shorten it.
        request.sustain(now + Duration::from_millis(10));
        assert_eq!(request.take(now), (true, true));
        assert_eq!(
            request.take(now + Duration::from_millis(500)),
            (true, true)
        );
        assert_eq!(request.take(now + Duration::from_secs(1)), (false, false));
        assert_eq!(request.take(now + Duration::from_secs(2)), (false, false));
    }

    #[test]
    fn stopping_leaves_the_wake_for_the_next_thread() {
        let scheduler = FrameScheduler::new();
        scheduler.wake();
        assert!(!scheduler.wait(&AtomicBool::new(true)));
        assert!(scheduler.wait(&AtomicBool::new(false)));
    }

    #[test]
    fn stopping_wakes_a_waiting_thread() {
        let scheduler = Arc::new(FrameScheduler::new());
        let stop = Arc::new(AtomicBool::new(false));
        let thread = std::thread::spawn({
            let scheduler = scheduler.clone();
            let stop = stop.clone();
            move || scheduler.wait(&stop)
        });
        std::thread::sleep(Duration::from_millis(20));
        stop.store(true, Ordering::Release);
        scheduler.notify_stop();
        assert!(!thread.join().unwrap());
    }

    #[test]
    fn input_rate_matches_gpui() {
        let start = Instant::now();
        let mut tracker = InputRateTracker::new();
        // 60 inputs a second is 6 in 100 ms.
        for i in 0..5 {
            assert!(!tracker.record_input(start + Duration::from_millis(i * 16)));
        }
        assert!(tracker.record_input(start + Duration::from_millis(80)));

        let mut slow = InputRateTracker::new();
        for i in 0..20 {
            assert!(!slow.record_input(start + Duration::from_millis(i * 30)));
        }
    }
}
