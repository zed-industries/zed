use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::Result;
use windows::Win32::{
    Graphics::DirectComposition::{
        COMPOSITION_FRAME_ID_COMPLETED, COMPOSITION_FRAME_STATS, DCompositionGetFrameId,
        DCompositionGetStatistics, DCompositionWaitForCompositorClock,
    },
    System::{Performance::QueryPerformanceFrequency, Threading::INFINITE},
};

static QPC_TICKS_PER_SECOND: LazyLock<u64> = LazyLock::new(|| {
    let mut frequency = 0;
    // On systems that run Windows XP or later, the function will always succeed and
    // will thus never return zero.
    unsafe { QueryPerformanceFrequency(&mut frequency).unwrap() };
    frequency as u64
});

const VSYNC_INTERVAL_THRESHOLD: Duration = Duration::from_millis(1);

pub(crate) struct VSyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool + Send>,
}

impl VSyncProvider {
    pub(crate) fn new() -> Result<Self> {
        let interval = get_dwm_interval_from_direct_composition()?;
        let f = Box::new(|| unsafe { DCompositionWaitForCompositorClock(None, INFINITE) == 0 });
        Ok(Self { interval, f })
    }

    pub(crate) fn wait_for_vsync(&self) {
        let vsync_start = Instant::now();
        let wait_succeeded = (self.f)();
        let elapsed = vsync_start.elapsed();
        // WaitForVBlank and DCompositionWaitForCompositorClock returns very early
        // instead of waiting until vblank when the monitor goes to sleep or is
        // unplugged (nothing to present due to desktop occlusion). We use 1ms as
        // a threshhold for the duration of the wait functions and fallback to
        // Sleep() if it returns before that. This could happen during normal
        // operation for the first call after the vsync thread becomes non-idle,
        // but it shouldn't happen often.
        if !wait_succeeded || elapsed < VSYNC_INTERVAL_THRESHOLD {
            log::warn!("VSyncProvider::wait_for_vsync() took shorter than expected");
            std::thread::sleep(self.interval);
        }
    }
}

fn get_dwm_interval_from_direct_composition() -> Result<Duration> {
    let frame_id = unsafe { DCompositionGetFrameId(COMPOSITION_FRAME_ID_COMPLETED) }?;
    let mut stats = COMPOSITION_FRAME_STATS::default();
    unsafe { DCompositionGetStatistics(frame_id, &mut stats, 0, None, None) }?;
    Ok(duration_from_qpc(stats.framePeriod))
}

fn duration_from_qpc(qpc: u64) -> Duration {
    let dwm_ticks_per_microsecond = *QPC_TICKS_PER_SECOND / 1_000_000;
    Duration::from_micros(qpc / dwm_ticks_per_microsecond)
}
