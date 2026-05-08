use super::{
    rows::{OverlayAction, SourceFilterAction},
    snapshot::OverlaySnapshot,
};
use crate::{Bounds, Hitbox, HitboxBehavior, Pixels, Point, Size, Window, point, px, size};

#[derive(Clone, Debug)]
pub(in crate::devtools) struct PreparedOverlay {
    pub(super) snapshot: OverlaySnapshot,
    pub(super) hud_bounds: Bounds<Pixels>,
    pub(super) hud_hitbox: Hitbox,
    pub(super) row_hitboxes: Vec<OverlayRowHitbox>,
}

#[derive(Clone, Debug)]
pub(super) struct OverlayRowHitbox {
    pub(super) hitbox: Hitbox,
    pub(super) action: OverlayAction,
}

pub(super) fn prepaint_overlay(
    window: &mut Window,
    snapshot: OverlaySnapshot,
    hud_origin: Option<Point<Pixels>>,
) -> PreparedOverlay {
    let hud_bounds = hud_bounds(snapshot.rows.len(), window.viewport_size(), hud_origin);
    let hud_hitbox = window.insert_hitbox(hud_bounds, HitboxBehavior::Normal);
    let mut row_hitboxes = Vec::new();
    for (row_index, row) in snapshot.rows.iter().enumerate() {
        for (action_index, action) in row.actions.iter().copied().enumerate() {
            let hitbox = window.insert_hitbox(
                hud_button_bounds(
                    hud_bounds,
                    row_index,
                    &row.actions,
                    &row.action_group_breaks,
                    action_index,
                ),
                HitboxBehavior::BlockMouse,
            );
            row_hitboxes.push(OverlayRowHitbox { hitbox, action });
        }
    }

    PreparedOverlay {
        snapshot,
        hud_bounds,
        hud_hitbox,
        row_hitboxes,
    }
}

fn hud_bounds(
    row_count: usize,
    viewport_size: Size<Pixels>,
    hud_origin: Option<Point<Pixels>>,
) -> Bounds<Pixels> {
    let margin = px(12.);
    let padding = hud_padding();
    let hud_width = px(680.);
    let line_height = hud_line_height();
    let hud_height = padding * 2. + line_height * (row_count as f32);
    let hud_size = size(hud_width, hud_height);
    let default_origin = point(
        (viewport_size.width - hud_width - margin).max(margin),
        margin,
    );
    let origin = hud_origin.unwrap_or(default_origin);
    Bounds::new(clamp_hud_origin(origin, viewport_size, hud_size), hud_size)
}

pub(super) fn clamp_hud_origin(
    origin: Point<Pixels>,
    viewport_size: Size<Pixels>,
    hud_size: Size<Pixels>,
) -> Point<Pixels> {
    let visible_handle_size = px(28.);
    let min = point(
        visible_handle_size - hud_size.width,
        visible_handle_size - hud_size.height,
    );
    let max = point(
        viewport_size.width - visible_handle_size,
        viewport_size.height - visible_handle_size,
    );
    let max = max.max(&min);
    origin.clamp(&min, &max)
}

fn hud_button_bounds(
    hud_bounds: Bounds<Pixels>,
    row_index: usize,
    actions: &[OverlayAction],
    group_breaks: &[usize],
    action_index: usize,
) -> Bounds<Pixels> {
    let padding = hud_padding();
    let line_height = hud_line_height();
    let action_offset = (0..action_index).fold(px(0.), |offset, i| {
        let gap = if group_breaks.contains(&i) {
            hud_group_gap()
        } else {
            hud_button_gap()
        };
        offset + actions[i].width() + gap
    });
    let button_width = actions
        .get(action_index)
        .map(|action| action.width())
        .unwrap_or(px(0.));
    Bounds::new(
        point(
            hud_bounds.origin.x + padding - px(2.) + action_offset,
            hud_bounds.origin.y + padding + line_height * (row_index as f32) - px(1.),
        ),
        size(button_width, line_height),
    )
}

pub(super) fn hud_row_bounds(hud_bounds: Bounds<Pixels>, row_index: usize) -> Bounds<Pixels> {
    let padding = hud_padding();
    let line_height = hud_line_height();
    Bounds::new(
        point(
            hud_bounds.origin.x + padding - px(2.),
            hud_bounds.origin.y + padding + line_height * (row_index as f32) - px(1.),
        ),
        size(hud_bounds.size.width - padding * 2. + px(4.), line_height),
    )
}

pub(super) fn hud_action_text_offset(actions: &[OverlayAction], group_breaks: &[usize]) -> Pixels {
    if actions.is_empty() {
        px(0.)
    } else {
        actions
            .iter()
            .enumerate()
            .fold(px(0.), |offset, (i, action)| {
                let gap = if group_breaks.contains(&i) {
                    hud_group_gap()
                } else {
                    hud_button_gap()
                };
                offset + action.width() + gap
            })
            + px(3.)
    }
}

/// Indent applied to `ColumnHeader` rows so their column labels line up with
/// data-row text (which is offset by the `[hide, pin]`/`[show, unpin]`
/// buttons that precede it).
pub(super) fn data_row_text_indent() -> Pixels {
    let hide = OverlayAction::toolbar("hide", false, SourceFilterAction::ResetFilters);
    let pin = OverlayAction::toolbar("pin", false, SourceFilterAction::ResetFilters);
    hud_action_text_offset(&[hide, pin], &[])
}

pub(super) fn hud_padding() -> Pixels {
    px(8.)
}

pub(super) fn hud_line_height() -> Pixels {
    px(14.)
}

fn hud_button_gap() -> Pixels {
    px(4.)
}

fn hud_group_gap() -> Pixels {
    px(14.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_hud_origin_keeps_a_handle_visible() {
        let viewport_size = size(px(100.), px(80.));
        let hud_size = size(px(460.), px(140.));

        assert_eq!(
            clamp_hud_origin(point(px(-1000.), px(1000.)), viewport_size, hud_size),
            point(px(28.) - hud_size.width, viewport_size.height - px(28.))
        );
    }

    #[test]
    fn dragged_hud_bounds_use_the_requested_origin() {
        let bounds = hud_bounds(4, size(px(800.), px(600.)), Some(point(px(120.), px(140.))));

        assert_eq!(bounds.origin, point(px(120.), px(140.)));
    }
}
