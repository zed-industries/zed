//! SVG text bbox and font-key helpers.

use super::{TextStyle, round_to_1_64_px};

pub(crate) const FLOWCHART_DEFAULT_FONT_KEY: &str = "trebuchetms,verdana,arial,sans-serif";

const SVG_DEFAULT_FIRST_LINE_BBOX_EM: f64 = 1.1875;
const SVG_COURIER_FIRST_LINE_BBOX_EM: f64 = 1.125;
const SVG_DEFAULT_TITLE_ASCENT_EM: f64 = 0.9444444444;
const SVG_DEFAULT_TITLE_DESCENT_EM: f64 = 0.262;
const SVG_COURIER_TITLE_ASCENT_EM: f64 = 0.8333333333333334;
const SVG_COURIER_TITLE_DESCENT_EM: f64 = 0.25;

pub(crate) fn normalize_font_key(s: &str) -> String {
    s.chars()
        .filter_map(|ch| {
            if ch.is_whitespace() || ch == '"' || ch == '\'' || ch == ';' {
                None
            } else {
                Some(ch.to_ascii_lowercase())
            }
        })
        .collect()
}

pub(crate) fn font_key_uses_courier_metrics(font_key: &str) -> bool {
    font_key
        .split(',')
        .any(|token| matches!(token, "courier" | "couriernew") || token.contains("monospace"))
}

pub(crate) fn style_uses_courier_metrics(style: &TextStyle) -> bool {
    style
        .font_family
        .as_deref()
        .map(normalize_font_key)
        .is_some_and(|font_key| font_key_uses_courier_metrics(&font_key))
}

pub(crate) fn svg_bbox_round_px_ties_to_even(v: f64) -> f64 {
    if !v.is_finite() {
        return 0.0;
    }
    let floor = v.floor();
    let frac = v - floor;
    if frac < 0.5 {
        floor
    } else if frac > 0.5 {
        floor + 1.0
    } else if (floor as i64) % 2 == 0 {
        floor
    } else {
        floor + 1.0
    }
}

pub(crate) fn svg_wrapped_first_line_bbox_height_px(style: &TextStyle) -> f64 {
    let first_line_em = if style_uses_courier_metrics(style) {
        SVG_COURIER_FIRST_LINE_BBOX_EM
    } else {
        SVG_DEFAULT_FIRST_LINE_BBOX_EM
    };
    svg_bbox_round_px_ties_to_even(style.font_size.max(1.0) * first_line_em)
}

pub(crate) fn flowchart_svg_edge_label_background_y_px(style: &TextStyle) -> f64 {
    let baseline_box_h =
        svg_bbox_round_px_ties_to_even(style.font_size.max(1.0) * SVG_COURIER_FIRST_LINE_BBOX_EM);
    baseline_box_h - svg_wrapped_first_line_bbox_height_px(style)
}

pub(crate) fn svg_title_bbox_vertical_extents_px(style: &TextStyle) -> (f64, f64) {
    let font_size = style.font_size.max(1.0);
    let (ascent_em, descent_em) = if style_uses_courier_metrics(style) {
        (SVG_COURIER_TITLE_ASCENT_EM, SVG_COURIER_TITLE_DESCENT_EM)
    } else {
        (SVG_DEFAULT_TITLE_ASCENT_EM, SVG_DEFAULT_TITLE_DESCENT_EM)
    };
    (font_size * ascent_em, font_size * descent_em)
}

pub(crate) fn svg_create_text_bbox_y_offset_px(style: &TextStyle) -> f64 {
    round_to_1_64_px(style.font_size.max(1.0) / 16.0)
}
