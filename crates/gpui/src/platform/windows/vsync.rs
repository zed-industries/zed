use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use util::ResultExt;
use windows::{
    Win32::{
        Foundation::{HANDLE, HWND},
        Graphics::{
            DirectComposition::{
                COMPOSITION_FRAME_ID_COMPLETED, COMPOSITION_FRAME_ID_TYPE, COMPOSITION_FRAME_STATS,
                COMPOSITION_TARGET_ID,
            },
            Dwm::{DWM_TIMING_INFO, DwmFlush, DwmGetCompositionTimingInfo},
        },
        System::{
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            Performance::QueryPerformanceFrequency,
            Threading::INFINITE,
        },
    },
    core::{HRESULT, s},
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

// Here we are using dynamic loading of DirectComposition functions,
// or the app will refuse to start on windows systems that do not support DirectComposition.
type DCompositionGetFrameId =
    unsafe extern "system" fn(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> HRESULT;
type DCompositionGetStatistics = unsafe extern "system" fn(
    frameid: u64,
    framestats: *mut COMPOSITION_FRAME_STATS,
    targetidcount: u32,
    targetids: *mut COMPOSITION_TARGET_ID,
    actualtargetidcount: *mut u32,
) -> HRESULT;
type DCompositionWaitForCompositorClock =
    unsafe extern "system" fn(count: u32, handles: *const HANDLE, timeoutinms: u32) -> u32;

pub(crate) struct VSyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool>,
}

impl VSyncProvider {
    pub(crate) fn new() -> Self {
        if let Some((get_frame_id, get_statistics, wait_for_comp_clock)) =
            initialize_direct_composition()
                .context("Retrieving DirectComposition functions")
                .log_with_level(log::Level::Warn)
        {
            let interval = get_dwm_interval_from_direct_composition(get_frame_id, get_statistics)
                .context("Failed to get DWM interval from DirectComposition")
                .log_err()
                .unwrap_or(DEFAULT_VSYNC_INTERVAL);
            log::info!(
                "DirectComposition is supported for VSync, interval: {:?}",
                interval
            );
            let f = Box::new(move || unsafe {
                wait_for_comp_clock(0, std::ptr::null(), INFINITE) == 0
            });
            Self { interval, f }
        } else {
            let interval = get_dwm_interval()
                .context("Failed to get DWM interval")
                .log_err()
                .unwrap_or(DEFAULT_VSYNC_INTERVAL);
            log::info!(
                "DirectComposition is not supported for VSync, falling back to DWM, interval: {:?}",
                interval
            );
            let f = Box::new(|| unsafe { DwmFlush().is_ok() });
            Self { interval, f }
        }
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
            log::trace!("VSyncProvider::wait_for_vsync() took less time than expected");
            std::thread::sleep(self.interval);
        }
    }
}

fn initialize_direct_composition() -> Result<(
    DCompositionGetFrameId,
    DCompositionGetStatistics,
    DCompositionWaitForCompositorClock,
)> {
    unsafe {
        // Load DLL at runtime since older Windows versions don't have dcomp.
        let hmodule = GetModuleHandleA(s!("dcomp.dll")).context("Loading dcomp.dll")?;
        let get_frame_id_addr = GetProcAddress(hmodule, s!("DCompositionGetFrameId"))
            .context("Function DCompositionGetFrameId not found")?;
        let get_statistics_addr = GetProcAddress(hmodule, s!("DCompositionGetStatistics"))
            .context("Function DCompositionGetStatistics not found")?;
        let wait_for_compositor_clock_addr =
            GetProcAddress(hmodule, s!("DCompositionWaitForCompositorClock"))
                .context("Function DCompositionWaitForCompositorClock not found")?;
        let get_frame_id: DCompositionGetFrameId = std::mem::transmute(get_frame_id_addr);
        let get_statistics: DCompositionGetStatistics = std::mem::transmute(get_statistics_addr);
        let wait_for_compositor_clock: DCompositionWaitForCompositorClock =
            std::mem::transmute(wait_for_compositor_clock_addr);
        Ok((get_frame_id, get_statistics, wait_for_compositor_clock))
    }
}

fn get_dwm_interval_from_direct_composition(
    get_frame_id: DCompositionGetFrameId,
    get_statistics: DCompositionGetStatistics,
) -> Result<Duration> {
    let mut frame_id = 0;
    unsafe { get_frame_id(COMPOSITION_FRAME_ID_COMPLETED, &mut frame_id) }.ok()?;
    let mut stats = COMPOSITION_FRAME_STATS::default();
    unsafe {
        get_statistics(
            frame_id,
            &mut stats,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    }
    .ok()?;
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
