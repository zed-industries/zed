//! Runtime frame-loop instrumentation, enabled with `GPUI_FRAME_DEBUG=1`.
//!
//! Answers three questions about a running app without changing rendering
//! behavior, to guide damage-tracking work:
//!
//! 1. *Who* is invalidating windows (attributed to `cx.notify()` /
//!    `window.refresh()` call sites via `#[track_caller]`).
//! 2. *How often* windows draw and how often already-drawn frames are
//!    re-presented (each present re-renders the full scene on the GPU).
//! 3. *How much* of the window actually changed each draw, by running
//!    [`SceneDamage::between`](crate::SceneDamage::between) in
//!    measure-only mode, along with the diff's own CPU cost.
//!
//! A summary is logged (at `info` level, target `gpui::frame_debug`) roughly
//! every 2 seconds per window, but only when frames occurred, so an idle
//! window logs nothing.

use crate::{SceneDamage, WindowId};
use scheduler::Instant;
use std::{
    collections::HashMap,
    panic::Location,
    sync::{Mutex, OnceLock},
    time::Duration,
};

/// Whether frame debugging was requested via `GPUI_FRAME_DEBUG=1`.
#[inline]
pub(crate) fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        let enabled =
            std::env::var("GPUI_FRAME_DEBUG").is_ok_and(|value| value != "0" && !value.is_empty());
        if enabled {
            // Make captures self-describing for A/B comparisons.
            log::info!(
                target: "gpui::frame_debug",
                "frame debugging enabled; present skip: {}, strict order damage: {}",
                crate::window::present_skip_enabled(),
                crate::scene_damage::strict_order_damage(),
            );
        }
        enabled
    })
}

const SUMMARY_INTERVAL: Duration = Duration::from_secs(2);

struct Stats {
    started_at: Instant,
    invalidations: HashMap<&'static Location<'static>, u64>,
    windows: HashMap<WindowId, WindowStats>,
}

impl Stats {
    fn new() -> Self {
        Self {
            started_at: Instant::now(),
            invalidations: HashMap::default(),
            windows: HashMap::default(),
        }
    }
}

#[derive(Default)]
struct WindowStats {
    draws: u64,
    draw_time: Duration,
    /// Frames where the scene diff reported no change at all.
    unchanged_draws: u64,
    full_damage_draws: u64,
    /// Sum over frames of damage area as a fraction of the viewport area.
    damage_area_fraction: f64,
    diff_time: Duration,
    /// `present()` calls for an unchanged scene (platform-required
    /// presentation or input-rate sustain), each of which re-renders the
    /// full scene on the GPU.
    represents: u64,
    /// Presents that were skipped because the scene was unchanged.
    skipped_presents: u64,
}

static STATS: Mutex<Option<Stats>> = Mutex::new(None);

fn with_stats(f: impl FnOnce(&mut Stats)) {
    if let Ok(mut stats) = STATS.lock() {
        f(stats.get_or_insert_with(Stats::new));
    }
}

/// Records that `caller` invalidated a window. Call sites are aggregated and
/// reported in the periodic summary.
#[inline]
pub(crate) fn record_invalidation(caller: &'static Location<'static>) {
    if !enabled() {
        return;
    }
    std::hint::cold_path();
    with_stats(|stats| {
        *stats.invalidations.entry(caller).or_default() += 1;
    });
}

/// Records a completed `Window::draw` along with the damage measured for
/// that frame.
pub(crate) fn record_draw(
    window_id: WindowId,
    damage: SceneDamage,
    diff_time: Duration,
    viewport_area: f64,
    draw_time: Duration,
) {
    debug_assert!(enabled());
    with_stats(|stats| {
        let window = stats.windows.entry(window_id).or_default();
        window.draws += 1;
        window.draw_time += draw_time;
        window.diff_time += diff_time;
        match damage {
            SceneDamage::Unchanged => window.unchanged_draws += 1,
            SceneDamage::Full => {
                window.full_damage_draws += 1;
                window.damage_area_fraction += 1.0;
            }
            SceneDamage::Rects(rects) => {
                // Rects may overlap slightly, so this can over-count; fine
                // for statistics.
                let area: f64 = rects
                    .as_slice()
                    .iter()
                    .map(|rect| f64::from(rect.size.width.0) * f64::from(rect.size.height.0))
                    .sum();
                if viewport_area > 0.0 {
                    window.damage_area_fraction += (area / viewport_area).min(1.0);
                }
            }
        }
    });
    maybe_log_summary();
}

/// Records a `present()` of an unchanged scene (the `needs_present` path in
/// the frame loop), which still re-renders the full scene on the GPU.
pub(crate) fn record_represent(window_id: WindowId) {
    if !enabled() {
        return;
    }
    with_stats(|stats| {
        let window = stats.windows.entry(window_id).or_default();
        window.represents += 1;
    });
    maybe_log_summary();
}

/// Records a present that was skipped because the scene was unchanged.
pub(crate) fn record_skipped_present(window_id: WindowId) {
    if !enabled() {
        return;
    }
    with_stats(|stats| {
        let window = stats.windows.entry(window_id).or_default();
        window.skipped_presents += 1;
    });
    maybe_log_summary();
}

fn maybe_log_summary() {
    static LAST_SUMMARY: Mutex<Option<Instant>> = Mutex::new(None);
    let now = Instant::now();
    {
        let Ok(mut last_summary) = LAST_SUMMARY.lock() else {
            return;
        };
        let due = last_summary.is_none_or(|last| now.duration_since(last) >= SUMMARY_INTERVAL);
        if !due {
            return;
        }
        *last_summary = Some(now);
    }

    let Ok(mut stats) = STATS.lock() else {
        return;
    };
    let Some(stats) = stats.take() else {
        return;
    };

    let elapsed_seconds = now.duration_since(stats.started_at).as_secs_f64().max(0.001);
    for (window_id, window) in &stats.windows {
        let changed_draws = window.draws - window.unchanged_draws;
        let mean_damage_percent = if changed_draws > 0 {
            window.damage_area_fraction / changed_draws as f64 * 100.0
        } else {
            0.0
        };
        log::info!(
            target: "gpui::frame_debug",
            "window {} over {:.1}s: {} draws ({:.1}/s; {} unchanged, {} full-damage), \
             {} re-presents, {} skipped presents, \
             mean damage {:.1}% of viewport, \
             draw avg {:?}, scene diff avg {:?}",
            window_id.as_u64(),
            elapsed_seconds,
            window.draws,
            window.draws as f64 / elapsed_seconds,
            window.unchanged_draws,
            window.full_damage_draws,
            window.represents,
            window.skipped_presents,
            mean_damage_percent,
            window.draw_time.checked_div(window.draws as u32).unwrap_or_default(),
            window.diff_time.checked_div(window.draws as u32).unwrap_or_default(),
        );
    }

    let mut invalidations: Vec<_> = stats.invalidations.into_iter().collect();
    invalidations.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));
    for (caller, count) in invalidations.iter().take(10) {
        log::info!(
            target: "gpui::frame_debug",
            "  invalidated by {caller}: {count}x",
        );
    }
}
