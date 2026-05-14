use super::rows::{OverlayRow, hud_rows};
use crate::{BorderStyle, Bounds, Pixels, WindowId};
use scheduler::Instant;

use super::super::event_age;
use super::super::format::short_type_name;
use super::super::{FLASH_DURATION, GPUI_DEVTOOLS, state::RenderHeatCause};

#[derive(Clone, Debug)]
pub(super) struct OverlaySnapshot {
    pub(super) heats: Vec<HeatOverlay>,
    pub(super) reuse_outlines: Vec<ReuseOutlineOverlay>,
    pub(super) flashes: Vec<FlashOverlay>,
    pub(super) rows: Vec<OverlayRow>,
}

#[derive(Clone, Debug)]
pub(super) struct HeatOverlay {
    pub(super) bounds: Bounds<Pixels>,
    pub(super) rate: usize,
    pub(super) opacity: f32,
    pub(super) border_style: BorderStyle,
}

#[derive(Clone, Debug)]
pub(super) struct ReuseOutlineOverlay {
    pub(super) bounds: Bounds<Pixels>,
    pub(super) opacity: f32,
}

#[derive(Clone, Debug)]
pub(super) struct FlashOverlay {
    pub(super) bounds: Bounds<Pixels>,
    pub(super) label: Option<&'static str>,
    pub(super) opacity: f32,
}

pub(super) fn update_and_snapshot_overlay(window_id: WindowId) -> OverlaySnapshot {
    let snapshot_started_at = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    let now = devtools.paused_at.unwrap_or_else(Instant::now);
    let show_flashes = devtools.show_flashes;
    let show_flash_labels = devtools.show_flash_labels;
    let show_heat = devtools.show_heat;
    let hidden_render_sources = devtools.hidden_render_sources.clone();
    let (heats, reuse_outlines, flashes) = devtools
        .windows
        .get_mut(&window_id)
        .map(|window_state| {
            let mut expired_heat_entities = Vec::new();
            let mut heats = Vec::new();
            if show_heat {
                for (entity_id, heat) in &mut window_state.render_heat {
                    if hidden_render_sources.contains(&heat.source) || heat.expired(now) {
                        expired_heat_entities.push(*entity_id);
                        continue;
                    }

                    let current_rate = heat.prune(now);
                    let rate = current_rate.max(heat.last_rate);
                    let opacity = heat.opacity(now);
                    if rate == 0 || opacity == 0. {
                        continue;
                    }

                    let Some(bounds) = heat
                        .bounds
                        .or_else(|| window_state.view_bounds.get(entity_id).copied())
                    else {
                        continue;
                    };

                    heats.push(HeatOverlay {
                        bounds,
                        rate,
                        opacity,
                        border_style: match heat.cause {
                            RenderHeatCause::Render => BorderStyle::Solid,
                            RenderHeatCause::Refresh => BorderStyle::Dashed,
                        },
                    });
                }
            }
            for entity_id in expired_heat_entities {
                window_state.render_heat.remove(&entity_id);
            }

            let mut expired_reuse_entities = Vec::new();
            let mut reuse_outlines = Vec::new();
            if show_heat {
                for (entity_id, reuse_outline) in &window_state.reuse_outlines {
                    if hidden_render_sources.contains(&reuse_outline.source)
                        || reuse_outline.expired(now)
                    {
                        expired_reuse_entities.push(*entity_id);
                        continue;
                    }

                    let opacity = reuse_outline.opacity(now);
                    if opacity == 0. {
                        continue;
                    }

                    let Some(bounds) = reuse_outline
                        .bounds
                        .or_else(|| window_state.view_bounds.get(entity_id).copied())
                    else {
                        continue;
                    };

                    reuse_outlines.push(ReuseOutlineOverlay { bounds, opacity });
                }
            }
            for entity_id in expired_reuse_entities {
                window_state.reuse_outlines.remove(&entity_id);
            }

            window_state.active_flashes.retain(|_, flash| {
                event_age(now, flash.timestamp).is_none_or(|age| age <= FLASH_DURATION)
            });

            let flashes = if show_flashes {
                window_state
                    .active_flashes
                    .iter()
                    .filter_map(|(entity_id, flash)| {
                        if hidden_render_sources.contains(&flash.source) {
                            return None;
                        }

                        let bounds = window_state.view_bounds.get(entity_id).copied()?;
                        let elapsed = event_age(now, flash.timestamp)?;
                        let opacity = 1. - elapsed.as_secs_f32() / FLASH_DURATION.as_secs_f32();
                        Some(FlashOverlay {
                            bounds,
                            label: show_flash_labels
                                .then_some(short_type_name(flash.source.entity_type)),
                            opacity: opacity.clamp(0., 1.),
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            };

            (heats, reuse_outlines, flashes)
        })
        .unwrap_or_default();

    let rows = hud_rows(&devtools, window_id, now);
    let snapshot = OverlaySnapshot {
        heats,
        reuse_outlines,
        flashes,
        rows,
    };
    devtools.record_snapshot_duration(snapshot_started_at, snapshot_started_at.elapsed());
    snapshot
}
