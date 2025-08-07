use std::{sync::LazyLock, time::Duration};

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
    unsafe { QueryPerformanceFrequency(&mut frequency) };
    frequency as u64
});

pub(crate) struct VsyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool + Send>,
}

impl VsyncProvider {
    pub fn new() -> Result<Self> {
        let interval = get_dwm_interval_from_direct_composition()?;
        let f = Box::new(|| unsafe { DCompositionWaitForCompositorClock(None, INFINITE) == 0 });
        Ok(Self { interval, f })
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
