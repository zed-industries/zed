use super::{
    input::register_input_handlers,
    layout::{
        PreparedOverlay, data_row_text_indent, hud_action_text_offset, hud_line_height,
        hud_padding, hud_row_bounds,
    },
    rows::OverlayRowKind,
};
use crate::{
    App, BorderStyle, Hsla, Pixels, Point, SharedString, TextAlign, TextRun, Window, fill, font,
    hsla, outline, point, px, quad, rgba,
};

#[derive(Clone, Copy, Debug)]
struct HeatStyle {
    hue: f32,
    fill_alpha: f32,
    border_alpha: f32,
    border_width: f32,
}

pub(super) fn paint_prepared_overlay(
    window: &mut Window,
    cx: &mut App,
    prepared_overlay: &PreparedOverlay,
) {
    for heat in &prepared_overlay.snapshot.heats {
        let style = heat_style(heat.rate, heat.opacity);
        window.paint_quad(quad(
            heat.bounds,
            px(2.),
            hsla(style.hue, 0.96, 0.54, style.fill_alpha),
            px(style.border_width),
            hsla(style.hue, 0.96, 0.58, style.border_alpha),
            heat.border_style,
        ));
    }

    for reuse_outline in &prepared_overlay.snapshot.reuse_outlines {
        window.paint_quad(outline(
            reuse_outline.bounds,
            hsla(0.42, 0.88, 0.62, 0.68 * reuse_outline.opacity),
            BorderStyle::Solid,
        ));
    }

    for flash in &prepared_overlay.snapshot.flashes {
        window.paint_quad(quad(
            flash.bounds,
            px(2.),
            hsla(0.54, 0.96, 0.52, 0.10 * flash.opacity),
            px(1.),
            hsla(0.54, 0.96, 0.62, 0.78 * flash.opacity),
            BorderStyle::default(),
        ));
    }

    paint_hud(window, cx, prepared_overlay);
}

fn heat_style(rate: usize, opacity: f32) -> HeatStyle {
    let (hue, fill_alpha, border_alpha, border_width) = if rate >= 60 {
        (0.0, 0.26, 0.98, 3.0)
    } else if rate >= 30 {
        (0.025, 0.21, 0.92, 2.5)
    } else if rate >= 15 {
        (0.075, 0.16, 0.82, 1.9)
    } else if rate >= 5 {
        (0.13, 0.12, 0.68, 1.35)
    } else {
        (0.54, 0.10, 0.58, 1.0)
    };

    HeatStyle {
        hue,
        fill_alpha: fill_alpha * opacity,
        border_alpha: border_alpha * opacity,
        border_width,
    }
}

fn paint_hud(window: &mut Window, cx: &mut App, prepared_overlay: &PreparedOverlay) {
    if prepared_overlay.snapshot.rows.is_empty() {
        return;
    }

    let padding = hud_padding();
    let line_height = hud_line_height();
    let bounds = prepared_overlay.hud_bounds;

    window.paint_quad(fill(bounds, rgba(0x111827dd)));
    window.paint_quad(outline(
        bounds,
        hsla(0.58, 0.68, 0.68, 0.72),
        BorderStyle::default(),
    ));

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let row_bounds = hud_row_bounds(bounds, line_index);
        match row.kind {
            OverlayRowKind::SectionBar | OverlayRowKind::Toolbar => {
                window.paint_quad(fill(row_bounds, rgba(0x273244aa)));
            }
            OverlayRowKind::ColumnHeader => {
                window.paint_quad(fill(row_bounds, rgba(0x1a2233aa)));
            }
            OverlayRowKind::Header | OverlayRowKind::Data | OverlayRowKind::Spacer => {}
        }
    }

    for row_hitbox in &prepared_overlay.row_hitboxes {
        let fill_color = if row_hitbox.hitbox.is_hovered(window) {
            rgba(0x38bdf84a)
        } else if row_hitbox.action.active {
            rgba(0x0ea5e94a)
        } else {
            rgba(0x1f29374a)
        };
        window.paint_quad(fill(row_hitbox.hitbox.bounds, fill_color));
        window.paint_quad(outline(
            row_hitbox.hitbox.bounds,
            hsla(0.58, 0.68, 0.68, 0.52),
            BorderStyle::default(),
        ));
        let button_text_origin = point(
            row_hitbox.hitbox.origin.x + px(5.),
            row_hitbox.hitbox.origin.y + px(1.),
        );
        paint_text_line_with_color(
            window,
            cx,
            button_text_origin,
            row_hitbox.action.label,
            line_height,
            hsla(0.58, 0.38, 0.98, 0.98),
        );
    }

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let text_color = match row.kind {
            OverlayRowKind::Header => hsla(0.58, 0.44, 0.94, 1.),
            OverlayRowKind::ColumnHeader => hsla(0.58, 0.30, 0.74, 0.92),
            OverlayRowKind::SectionBar | OverlayRowKind::Toolbar => hsla(0.12, 0.62, 0.76, 1.),
            OverlayRowKind::Data => hsla(0.58, 0.38, 0.92, 0.96),
            OverlayRowKind::Spacer => continue,
        };
        let row_indent = match row.kind {
            OverlayRowKind::ColumnHeader => data_row_text_indent(),
            _ => hud_action_text_offset(&row.actions, &row.action_group_breaks),
        };
        let origin = point(
            bounds.origin.x + padding + row_indent,
            bounds.origin.y + padding + line_height * (line_index as f32),
        );
        paint_text_line_with_color(window, cx, origin, &row.text, line_height, text_color);
    }

    register_input_handlers(window, prepared_overlay);
}

fn paint_text_line_with_color(
    window: &mut Window,
    cx: &mut App,
    origin: Point<Pixels>,
    line: &str,
    line_height: Pixels,
    color: Hsla,
) {
    let font_size = px(11.);
    let text_run = TextRun {
        len: line.len(),
        font: font(".SystemUIFont"),
        color,
        ..TextRun::default()
    };
    let shaped_line = window.text_system().shape_line(
        SharedString::from(line.to_string()),
        font_size,
        &[text_run],
        None,
    );
    if let Err(error) = shaped_line.paint(origin, line_height, TextAlign::Left, None, window, cx) {
        log::debug!("failed to paint GPUI devtools HUD text: {error:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_style_uses_expected_rate_bands() {
        assert_eq!(heat_style(4, 1.).hue, 0.54);
        assert_eq!(heat_style(5, 1.).hue, 0.13);
        assert_eq!(heat_style(15, 1.).hue, 0.075);
        assert_eq!(heat_style(30, 1.).hue, 0.025);
        assert_eq!(heat_style(60, 1.).hue, 0.0);
    }
}
