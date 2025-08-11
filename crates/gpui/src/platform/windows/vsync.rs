use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use util::ResultExt;
use windows::Win32::{
    Foundation::HWND,
    Graphics::{
        DirectComposition::{
            COMPOSITION_FRAME_ID_COMPLETED, COMPOSITION_FRAME_STATS, DCompositionGetFrameId,
            DCompositionGetStatistics, DCompositionWaitForCompositorClock,
        },
        Dwm::{DWM_TIMING_INFO, DwmFlush, DwmGetCompositionTimingInfo},
    },
    System::{Performance::QueryPerformanceFrequency, Threading::INFINITE},
};

use crate::WindowsVersion;

static QPC_TICKS_PER_SECOND: LazyLock<u64> = LazyLock::new(|| {
    let mut frequency = 0;
    // On systems that run Windows XP or later, the function will always succeed and
    // will thus never return zero.
    unsafe { QueryPerformanceFrequency(&mut frequency).unwrap() };
    frequency as u64
});

const VSYNC_INTERVAL_THRESHOLD: Duration = Duration::from_millis(1);
const DEFAULT_VSYNC_INTERVAL: Duration = Duration::from_micros(16_666); // ~60Hz

pub(crate) struct VSyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool>,
}

impl VSyncProvider {
    pub(crate) fn new(windows_version: WindowsVersion) -> Self {
        let interval: Duration;
        let f: Box<dyn Fn() -> bool>;
        match windows_version {
            WindowsVersion::Win10 => {
                interval = get_dwm_interval()
                    .context("Failed to get DWM interval")
                    .log_err()
                    .unwrap_or(DEFAULT_VSYNC_INTERVAL);
                f = Box::new(|| unsafe { DwmFlush().is_ok() });
            }
            WindowsVersion::Win11 => {
                interval = get_dwm_interval_from_direct_composition()
                    .context("Failed to get DWM interval")
                    .log_err()
                    .unwrap_or(DEFAULT_VSYNC_INTERVAL);
                f = Box::new(|| unsafe { DCompositionWaitForCompositorClock(None, INFINITE) == 0 });
            }
        }
        log::info!("VSyncProvider initialized with interval: {:?}", interval);
        Self { interval, f }
    }

    pub(crate) fn wait_for_vsync(&self) {
        let vsync_start = Instant::now();
        let wait_succeeded = (self.f)();
        let elapsed = vsync_start.elapsed();
        // DwmFlush and DCompositionWaitForCompositorClock returns very early
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
    Ok(retrieve_duration(stats.framePeriod, *QPC_TICKS_PER_SECOND))
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
