use std::{
    collections::HashMap,
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use gpui::FrameRequestSource;
use gpui_util::ResultExt;
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::{
        Dwm::{
            DWM_TIMING_INFO, DWMWA_EXTENDED_FRAME_BOUNDS, DwmFlush, DwmGetCompositionTimingInfo,
            DwmGetWindowAttribute,
        },
        Dxgi::{CreateDXGIFactory1, IDXGIFactory1, IDXGIOutput},
        Gdi::{HMONITOR, MONITOR_DEFAULTTONEAREST, MonitorFromWindow},
    },
    System::Performance::QueryPerformanceFrequency,
};

use crate::{monitor_refresh_interval, monitors_under};

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

/// How often a monitor's refresh is looked up again, to follow mode changes.
const MONITOR_REFRESH_TTL: Duration = Duration::from_secs(1);
/// How often a window's monitor is looked up again, to follow moves.
const WINDOW_MONITOR_TTL: Duration = Duration::from_millis(100);

/// The vblank clock of one monitor: the DXGI output that drives it. Each runs on its own thread,
/// so a window on a 144 Hz monitor next to a 240 Hz one draws on its own monitor's vblanks
/// instead of the compositor clock, which runs at the fastest display's rate.
pub(crate) struct MonitorClock {
    pub(crate) monitor: isize,
    output: IDXGIOutput,
}

// SAFETY: DXGI objects are free-threaded; each clock is used by the one thread it moves to.
unsafe impl Send for MonitorClock {}

impl MonitorClock {
    /// One clock per output of every adapter, so monitors on the integrated and the discrete
    /// GPU of a hybrid laptop each get theirs.
    pub(crate) fn all() -> Vec<Self> {
        let Some(factory) = (unsafe { CreateDXGIFactory1::<IDXGIFactory1>() }).log_err() else {
            return Vec::new();
        };
        let mut clocks = Vec::new();
        for adapter in (0..).map_while(|i| unsafe { factory.EnumAdapters1(i) }.ok()) {
            for output in (0..).map_while(|i| unsafe { adapter.EnumOutputs(i) }.ok()) {
                if let Ok(desc) = unsafe { output.GetDesc() }
                    && desc.AttachedToDesktop.as_bool()
                {
                    clocks.push(Self {
                        monitor: desc.Monitor.0 as isize,
                        output,
                    });
                }
            }
        }
        clocks
    }

    /// Blocks until the monitor's next vblank; an error means the output is gone.
    pub(crate) fn wait(&self) -> bool {
        unsafe { self.output.WaitForVBlank() }.is_ok()
    }
}

/// Which monitor's clock ticks each window: the fastest monitor its visible frame overlaps, so
/// a window across two monitors draws at the faster one's rate and neither part of it lags.
#[derive(Default)]
pub(crate) struct WindowMonitors {
    windows: HashMap<isize, (isize, Instant)>,
    refresh: HashMap<isize, (Option<Duration>, Instant)>,
}

impl WindowMonitors {
    pub(crate) fn of(&mut self, hwnd: HWND, now: Instant) -> isize {
        if let Some((monitor, checked)) = self.windows.get(&(hwnd.0 as isize))
            && now.duration_since(*checked) < WINDOW_MONITOR_TTL
        {
            return *monitor;
        }
        let mut bounds = RECT::default();
        // The visible frame: the invisible resize border would reach into a neighbouring monitor.
        let visible = unsafe {
            DwmGetWindowAttribute(
                hwnd,
                DWMWA_EXTENDED_FRAME_BOUNDS,
                &mut bounds as *mut RECT as *mut _,
                std::mem::size_of::<RECT>() as u32,
            )
        };
        let under = if visible.is_ok() {
            monitors_under(bounds)
        } else {
            Default::default()
        };
        let rated: Vec<(isize, Option<Duration>)> = under
            .into_iter()
            .map(|m| (m.0 as isize, self.refresh_of(m, now)))
            .collect();
        let monitor = fastest(&rated).unwrap_or_else(
            || unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) }.0 as isize,
        );
        self.windows.insert(hwnd.0 as isize, (monitor, now));
        monitor
    }

    fn refresh_of(&mut self, monitor: HMONITOR, now: Instant) -> Option<Duration> {
        let entry = self
            .refresh
            .entry(monitor.0 as isize)
            .or_insert_with(|| (monitor_refresh_interval(monitor), now));
        if now.duration_since(entry.1) > MONITOR_REFRESH_TTL {
            *entry = (monitor_refresh_interval(monitor), now);
        }
        entry.0
    }

    /// Forgets windows that are gone.
    pub(crate) fn retain(&mut self, live: impl Fn(isize) -> bool) {
        self.windows.retain(|hwnd, _| live(*hwnd));
    }
}

/// The monitor with the shortest refresh interval; one whose rate is unknown counts as slowest.
fn fastest(monitors: &[(isize, Option<Duration>)]) -> Option<isize> {
    monitors
        .iter()
        .min_by_key(|(_, interval)| interval.unwrap_or(Duration::MAX))
        .map(|(monitor, _)| *monitor)
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
    fn a_window_across_monitors_takes_the_fastest_one() {
        let ms = |v: f64| Some(Duration::from_secs_f64(v / 1000.));
        assert_eq!(fastest(&[(1, ms(6.94)), (2, ms(4.17))]), Some(2));
        assert_eq!(fastest(&[(1, None), (2, ms(16.67))]), Some(2));
        assert_eq!(fastest(&[(1, ms(6.94))]), Some(1));
        assert_eq!(fastest(&[]), None);
    }

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
}
