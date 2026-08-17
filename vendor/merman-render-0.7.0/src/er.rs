use crate::config::{config_bool, config_f64, config_f64_css_px};
use crate::model::{Bounds, ErDiagramLayout, LayoutEdge, LayoutLabel, LayoutNode, LayoutPoint};
use crate::text::{TextMeasurer, TextMetrics, TextStyle, WrapMode};
use crate::{Error, Result};
use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, RankDir};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) type ErModel = merman_core::diagrams::er::ErDiagramRenderModel;
pub(crate) type ErEntity = merman_core::diagrams::er::ErEntityRenderModel;
pub(crate) type ErRelationship = merman_core::diagrams::er::ErRelationshipRenderModel;
pub(crate) type ErClassDef = merman_core::diagrams::er::ErClassDefRenderModel;

fn normalize_dir(direction: &str) -> String {
    match direction.trim().to_uppercase().as_str() {
        "TB" | "TD" => "TB".to_string(),
        "BT" => "BT".to_string(),
        "LR" => "LR".to_string(),
        "RL" => "RL".to_string(),
        other => other.to_string(),
    }
}

fn rank_dir_from(direction: &str) -> RankDir {
    match normalize_dir(direction).as_str() {
        "TB" => RankDir::TB,
        "BT" => RankDir::BT,
        "LR" => RankDir::LR,
        "RL" => RankDir::RL,
        _ => RankDir::TB,
    }
}

pub(crate) fn er_generic_markdown_plain_text(text: &str) -> Option<String> {
    if !(text.contains('<') || text.contains('>')) {
        return None;
    }
    let lower = text.to_ascii_lowercase();
    if lower.contains("<br") || lower.contains("<strong") || lower.contains("<em") {
        return None;
    }
    if !(text.contains('*') || text.contains('_') || text.contains('`')) {
        return None;
    }

    let mut plain = text.trim().to_string();
    loop {
        let next = plain
            .strip_prefix("**")
            .and_then(|s| s.strip_suffix("**"))
            .or_else(|| plain.strip_prefix("__").and_then(|s| s.strip_suffix("__")))
            .or_else(|| plain.strip_prefix('*').and_then(|s| s.strip_suffix('*')))
            .or_else(|| plain.strip_prefix('_').and_then(|s| s.strip_suffix('_')))
            .or_else(|| plain.strip_prefix('`').and_then(|s| s.strip_suffix('`')));

        let Some(next) = next else {
            break;
        };
        plain = next.trim().to_string();
    }

    if plain.is_empty() || plain == text {
        None
    } else {
        Some(plain)
    }
}

pub(crate) fn er_label_has_structural_markdown(text: &str) -> bool {
    if !(text.contains('*') || text.contains('_') || text.contains('`') || text.contains('<')) {
        return false;
    }

    let parser = pulldown_cmark::Parser::new_ext(
        text,
        pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS,
    );
    parser.into_iter().any(|ev| {
        matches!(
            ev,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Strong)
                | pulldown_cmark::Event::Start(pulldown_cmark::Tag::Emphasis)
                | pulldown_cmark::Event::Code(_)
                | pulldown_cmark::Event::Html(_)
                | pulldown_cmark::Event::InlineHtml(_)
        )
    })
}

pub(crate) fn er_html_label_metrics(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> TextMetrics {
    let text = text.trim();
    if text.is_empty() {
        return TextMetrics {
            width: 0.0,
            height: 0.0,
            line_count: 0,
        };
    }

    let lower = text.to_ascii_lowercase();
    let has_inline_html =
        lower.contains("<br") || lower.contains("<strong") || lower.contains("<em");

    let apply_width_override = |metrics: &mut TextMetrics| {
        if let Some(width) =
            crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(style.font_size, text)
        {
            metrics.width = width.max(0.0);
        }
    };

    let generic_markdown_plain = er_generic_markdown_plain_text(text);
    let measure_text = generic_markdown_plain.as_deref().unwrap_or(text);

    if (measure_text.contains('<') || measure_text.contains('>')) && !has_inline_html {
        let mut metrics = measurer.measure_wrapped(measure_text, style, None, WrapMode::HtmlLike);
        apply_width_override(&mut metrics);
        return metrics;
    }

    let has_markdown = er_label_has_structural_markdown(text);
    let mut metrics = if has_markdown || has_inline_html {
        crate::text::measure_markdown_with_flowchart_bold_deltas(
            measurer,
            measure_text,
            style,
            None,
            WrapMode::HtmlLike,
        )
    } else {
        measurer.measure_wrapped(measure_text, style, None, WrapMode::HtmlLike)
    };
    apply_width_override(&mut metrics);
    if text.contains('`') {
        let svg_bbox_w = measurer.measure_svg_simple_text_bbox_width_px(text, style);
        metrics.width = crate::text::round_to_1_64_px(metrics.width.max(svg_bbox_w));
    }
    metrics
}

pub(crate) fn calculate_text_width_like_mermaid_px(
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    text: &str,
) -> i64 {
    // Mermaid `calculateTextWidth` uses SVG `drawSimpleText(...).getBBox().width` and rounds to
    // integers. It probes both `sans-serif` and the configured `fontFamily`, but typically
    // takes the larger width to avoid underestimation when the configured family cannot render
    // in the current user agent.
    let mut sans = style.clone();
    sans.font_family = Some("sans-serif".to_string());
    sans.font_weight = None;

    let mut fam = style.clone();
    fam.font_weight = None;

    let w_fam = measurer.measure_svg_simple_text_bbox_width_px(text, &fam);
    let w_sans = measurer.measure_svg_simple_text_bbox_width_px(text, &sans);
    let w = match (
        w_fam.is_finite() && w_fam > 0.0,
        w_sans.is_finite() && w_sans > 0.0,
    ) {
        (true, true) => w_fam.max(w_sans),
        (true, false) => w_fam,
        (false, true) => w_sans,
        (false, false) => 0.0,
    };
    if !w.is_finite() {
        return 0;
    }
    // Our headless SVG bbox approximation uses a power-of-two grid internally. Nudge by half of a
    // 1/256px step to avoid systematic round-down at the `.5` boundary that can affect
    // `minEntityWidth` clamping in Mermaid's `erBox.ts` `drawRect` branch.
    (w + (1.0 / 512.0)).round() as i64
}

fn er_text_style(effective_config: &Value) -> TextStyle {
    let font_family = Some(crate::config::config_font_family_css(effective_config));
    // Mermaid ER unified renderer inherits the root SVG font-size, so `themeVariables.fontSize`
    // wins when present (including Mermaid's common `"NNpx"` form).
    let font_size = crate::config::config_theme_or_root_font_size_px_opt(effective_config)
        .or_else(|| config_f64_css_px(effective_config, &["er", "fontSize"]))
        .unwrap_or(16.0);
    TextStyle {
        font_family,
        font_size,
        font_weight: None,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ErEntityMeasureRow {
    pub type_text: String,
    pub name_text: String,
    pub key_text: String,
    pub comment_text: String,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ErEntityMeasure {
    pub width: f64,
    pub height: f64,
    pub text_padding: f64,
    pub label_text: String,
    pub label_html_width: f64,
    pub label_height: f64,
    pub label_max_width_px: i64,
    pub has_key: bool,
    pub has_comment: bool,
    pub type_col_w: f64,
    pub name_col_w: f64,
    pub key_col_w: f64,
    pub comment_col_w: f64,
    pub rows: Vec<ErEntityMeasureRow>,
}

pub(crate) fn measure_entity_box(
    entity: &ErEntity,
    measurer: &dyn TextMeasurer,
    label_style: &TextStyle,
    attr_style: &TextStyle,
    effective_config: &Value,
) -> ErEntityMeasure {
    // Mermaid measures ER attribute table text via HTML labels (`foreignObject`) and browser font
    // metrics. Our headless measurer is an approximation; keep the math as close as possible to
    // upstream and avoid introducing arbitrary scaling factors.
    const ATTR_TEXT_WIDTH_SCALE: f64 = 1.0;

    // Mermaid's ER renderer (erBox.ts) uses `config.htmlLabels` inconsistently:
    // - It passes `useHtmlLabels: config.htmlLabels` into `createText`, where `undefined`
    //   effectively behaves as `true` due to JS default parameters.
    // - It uses `if (!config.htmlLabels) { PADDING *= 1.25; TEXT_PADDING *= 1.25; }`, where
    //   `undefined` behaves as `false` and triggers the multiplier even when HTML labels are used.
    //
    // Upstream SVG fixtures at Mermaid@11.12.2 reflect this quirk. The padding multiplier still
    // keys off the raw truthiness (`undefined` behaves like `false`) even though the rendered labels
    // use HTML `<foreignObject>` output by default.
    let html_labels_raw = config_bool(effective_config, &["htmlLabels"]).unwrap_or(false);

    // Mermaid ER unified shape (`erBox.ts`) uses:
    // - PADDING = config.er.diagramPadding (default 20 in Mermaid 11.12.2 schema defaults)
    // - TEXT_PADDING = config.er.entityPadding (default 15)
    let mut padding = config_f64(effective_config, &["er", "diagramPadding"]).unwrap_or(20.0);
    let mut text_padding = config_f64(effective_config, &["er", "entityPadding"]).unwrap_or(15.0);
    let min_w = config_f64(effective_config, &["er", "minEntityWidth"]).unwrap_or(100.0);
    let wrapping_width_px = config_f64(effective_config, &["flowchart", "wrappingWidth"])
        .unwrap_or(200.0)
        .round()
        .max(0.0) as i64;

    let label_text = if entity.alias.trim().is_empty() {
        entity.label.as_str()
    } else {
        entity.alias.as_str()
    }
    .to_string();
    let label_metrics = er_html_label_metrics(&label_text, measurer, label_style);
    let label_html_width = crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(
        label_style.font_size,
        &label_text,
    )
    .unwrap_or_else(|| label_metrics.width.max(0.0));

    // No attributes: use `drawRect`-like padding rules from Mermaid erBox.ts.
    if entity.attributes.is_empty() {
        let label_pad_x = padding;
        let label_pad_y = padding * 1.5;
        // Mermaid's `drawRect` branch clamps to `minEntityWidth` based on `calculateTextWidth()`,
        // not on the HTML label bbox. Preserve that quirk: upstream can end up with nodes that are
        // narrower than `minEntityWidth` when `calculateTextWidth()` is larger than the HTML bbox
        // used by `drawRect`.
        let calc_w = calculate_text_width_like_mermaid_px(measurer, label_style, &label_text);
        let clamp_to_min_w = crate::generated::er_text_overrides_11_12_2::
            lookup_entity_drawrect_clamp_to_min_entity_width(&label_text)
            .unwrap_or((calc_w as f64 + label_pad_x * 2.0) < min_w);
        let width = if clamp_to_min_w {
            min_w
        } else {
            label_html_width + label_pad_x * 2.0
        };
        let height = label_metrics.height + label_pad_y * 2.0;
        return ErEntityMeasure {
            width: width.max(1.0),
            height: height.max(1.0),
            text_padding,
            label_text,
            label_html_width,
            label_height: label_metrics.height.max(0.0),
            label_max_width_px: if clamp_to_min_w {
                min_w.round().max(0.0) as i64
            } else {
                wrapping_width_px
            },
            has_key: false,
            has_comment: false,
            type_col_w: 0.0,
            name_col_w: 0.0,
            key_col_w: 0.0,
            comment_col_w: 0.0,
            rows: Vec::new(),
        };
    }

    // Mermaid erBox.ts only applies the `* 1.25` multiplier after the "drawRect" early-return.
    // Keep that behavior: nodes without an attribute table should *not* inherit the multiplier.
    if !html_labels_raw {
        padding *= 1.25;
        text_padding *= 1.25;
    }

    let mut rows: Vec<ErEntityMeasureRow> = Vec::new();

    let mut max_type_raw_w: f64 = 0.0;
    let mut max_name_raw_w: f64 = 0.0;
    let mut max_keys_raw_w: f64 = 0.0;
    let mut max_comment_raw_w: f64 = 0.0;

    let mut max_type_col_w: f64 = 0.0;
    let mut max_name_col_w: f64 = 0.0;
    let mut max_keys_col_w: f64 = 0.0;
    let mut max_comment_col_w: f64 = 0.0;

    let mut total_rows_h = 0.0;

    for a in &entity.attributes {
        let ty = merman_core::common::parse_generic_types(&a.ty);
        let type_m = er_html_label_metrics(&ty, measurer, attr_style);
        let name_m = er_html_label_metrics(&a.name, measurer, attr_style);

        let type_w = crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(
            attr_style.font_size,
            &ty,
        )
        .unwrap_or(type_m.width)
            * ATTR_TEXT_WIDTH_SCALE;
        let name_w = crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(
            attr_style.font_size,
            &a.name,
        )
        .unwrap_or(name_m.width)
            * ATTR_TEXT_WIDTH_SCALE;
        max_type_raw_w = max_type_raw_w.max(type_w);
        max_name_raw_w = max_name_raw_w.max(name_w);
        max_type_col_w = max_type_col_w.max(type_w + padding);
        max_name_col_w = max_name_col_w.max(name_w + padding);

        let key_text = a.keys.join(",");
        let keys_m = er_html_label_metrics(&key_text, measurer, attr_style);
        let keys_w = crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(
            attr_style.font_size,
            &key_text,
        )
        .unwrap_or(keys_m.width)
            * ATTR_TEXT_WIDTH_SCALE;
        max_keys_raw_w = max_keys_raw_w.max(keys_w);
        max_keys_col_w = max_keys_col_w.max(keys_w + padding);

        let comment_text = a.comment.clone();
        let comment_m = er_html_label_metrics(&comment_text, measurer, attr_style);
        let comment_w = crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(
            attr_style.font_size,
            &comment_text,
        )
        .unwrap_or(comment_m.width)
            * ATTR_TEXT_WIDTH_SCALE;
        max_comment_raw_w = max_comment_raw_w.max(comment_w);
        max_comment_col_w = max_comment_col_w.max(comment_w + padding);

        let row_h = type_m
            .height
            .max(name_m.height)
            .max(keys_m.height)
            .max(comment_m.height)
            + text_padding;

        rows.push(ErEntityMeasureRow {
            type_text: ty,
            name_text: a.name.clone(),
            key_text,
            comment_text,
            height: row_h.max(1.0),
        });
        total_rows_h += row_h.max(1.0);
    }

    let mut total_width_sections = 4usize;
    let mut has_key = true;
    let mut has_comment = true;
    if max_keys_col_w <= padding {
        has_key = false;
        max_keys_col_w = 0.0;
        total_width_sections = total_width_sections.saturating_sub(1);
    }
    if max_comment_col_w <= padding {
        has_comment = false;
        max_comment_col_w = 0.0;
        total_width_sections = total_width_sections.saturating_sub(1);
    }

    // Mermaid adds extra padding to attribute components to accommodate the entity name width.
    // Mermaid uses the HTML label bbox (`getBoundingClientRect`) as `nameBBox.width`.
    let name_w_min = label_html_width + padding * 2.0;
    let mut max_width = max_type_col_w + max_name_col_w + max_keys_col_w + max_comment_col_w;
    if name_w_min - max_width > 0.0 && total_width_sections > 0 {
        let diff = name_w_min - max_width;
        let per = diff / total_width_sections as f64;
        max_type_col_w += per;
        max_name_col_w += per;
        if has_key {
            max_keys_col_w += per;
        }
        if has_comment {
            max_comment_col_w += per;
        }
        max_width = max_type_col_w + max_name_col_w + max_keys_col_w + max_comment_col_w;
    }

    let shape_bbox_w = label_html_width
        .max(max_type_raw_w)
        .max(max_name_raw_w)
        .max(max_keys_raw_w)
        .max(max_comment_raw_w);

    let width = (shape_bbox_w + padding * 2.0).max(max_width);
    let name_h = label_metrics.height + text_padding;
    let height = total_rows_h + name_h;

    ErEntityMeasure {
        width: width.max(1.0),
        height: height.max(1.0),
        text_padding,
        label_text,
        label_html_width,
        label_height: label_metrics.height.max(0.0),
        label_max_width_px: wrapping_width_px,
        has_key,
        has_comment,
        type_col_w: max_type_col_w.max(0.0),
        name_col_w: max_name_col_w.max(0.0),
        key_col_w: max_keys_col_w.max(0.0),
        comment_col_w: max_comment_col_w.max(0.0),
        rows,
    }
}

fn entity_box_dimensions(
    entity: &ErEntity,
    measurer: &dyn TextMeasurer,
    label_style: &TextStyle,
    attr_style: &TextStyle,
    effective_config: &Value,
) -> (f64, f64) {
    let m = measure_entity_box(entity, measurer, label_style, attr_style, effective_config);
    (m.width, m.height)
}

fn edge_label_metrics(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    html_labels: bool,
) -> (f64, f64) {
    let text = text.trim();
    if text.is_empty() {
        return (0.0, 0.0);
    }

    let lower = text.to_ascii_lowercase();
    let has_inline_html =
        lower.contains("<br") || lower.contains("<strong") || lower.contains("<em");
    let has_markdown = text.contains('*') || text.contains('_');
    let wrap_mode = if html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };

    // Mermaid ER relationship labels follow Mermaid's effective HTML-label resolution:
    // root `htmlLabels` first, then `flowchart.htmlLabels`, then default `true`.
    // - HTML mode uses the generic HTML edge-label path (`foreignObject`, line-height 1.5)
    // - SVG mode uses `createFormattedText(...)` (`<text>/<tspan>`, line-height 1.1)
    // Markdown emphasis is tokenized in both branches before the final DOM shape is emitted.
    if has_markdown || has_inline_html {
        let m = crate::text::measure_markdown_with_flowchart_bold_deltas(
            measurer, text, style, None, wrap_mode,
        );
        return (m.width.max(0.0), m.height.max(0.0));
    }

    let m = measurer.measure_wrapped(text, style, None, wrap_mode);
    let w = if html_labels {
        crate::generated::er_text_overrides_11_12_2::lookup_html_width_px(style.font_size, text)
            .unwrap_or(m.width)
    } else {
        m.width
    };
    (w.max(0.0), m.height.max(0.0))
}

fn parse_er_rel_idx_from_edge_name(name: &str) -> Option<usize> {
    let rest = name.strip_prefix("er-rel-")?;
    let mut end = 0usize;
    for (idx, ch) in rest.char_indices() {
        if !ch.is_ascii_digit() {
            break;
        }
        end = idx + ch.len_utf8();
    }
    if end == 0 {
        return None;
    }
    rest[..end].parse::<usize>().ok()
}

fn is_er_self_loop_dummy_node_id(id: &str) -> bool {
    // Mermaid's dagre renderer creates self-loop helper nodes using `${nodeId}---${nodeId}---{1|2}`.
    id.contains("---")
}

pub(crate) fn er_relationship_html_labels(effective_config: &Value) -> bool {
    // Mermaid ER relationship labels follow `getEffectiveHtmlLabels(config)` from
    // `rendering-elements/edges.js`:
    // - root `htmlLabels` wins when explicitly set
    // - otherwise `flowchart.htmlLabels` is used
    // - otherwise the default is `true`
    //
    // This intentionally differs from the entity padding quirk, which keys off raw
    // `!config.htmlLabels` in `erBox.ts`.
    config_bool(effective_config, &["htmlLabels"])
        .or_else(|| config_bool(effective_config, &["flowchart", "htmlLabels"]))
        .unwrap_or(true)
}

#[derive(Debug, Clone)]
struct LayoutEdgeParts {
    id: String,
    from: String,
    to: String,
    points: Vec<LayoutPoint>,
    label: Option<LayoutLabel>,
    start_marker: Option<String>,
    end_marker: Option<String>,
    stroke_dasharray: Option<String>,
}

fn calc_label_position(points: &[LayoutPoint]) -> Option<(f64, f64)> {
    if points.is_empty() {
        return None;
    }
    if points.len() == 1 {
        return Some((points[0].x, points[0].y));
    }

    let mut total = 0.0;
    for i in 1..points.len() {
        let dx = points[i].x - points[i - 1].x;
        let dy = points[i].y - points[i - 1].y;
        total += (dx * dx + dy * dy).sqrt();
    }
    let mut remaining = total / 2.0;
    for i in 1..points.len() {
        let p0 = &points[i - 1];
        let p1 = &points[i];
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let seg = (dx * dx + dy * dy).sqrt();
        if seg == 0.0 {
            continue;
        }
        if seg < remaining {
            remaining -= seg;
            continue;
        }
        let t = (remaining / seg).clamp(0.0, 1.0);
        return Some((p0.x + t * dx, p0.y + t * dy));
    }
    Some((points.last()?.x, points.last()?.y))
}

type Rect = merman_core::geom::Box2;

fn intersect_segment_with_rect(
    p0: &LayoutPoint,
    p1: &LayoutPoint,
    rect: Rect,
) -> Option<LayoutPoint> {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    if dx == 0.0 && dy == 0.0 {
        return None;
    }

    let mut candidates: Vec<(f64, LayoutPoint)> = Vec::new();
    let eps = 1e-9;
    let min_x = rect.min_x();
    let max_x = rect.max_x();
    let min_y = rect.min_y();
    let max_y = rect.max_y();

    if dx.abs() > eps {
        for x_edge in [min_x, max_x] {
            let t = (x_edge - p0.x) / dx;
            if t < -eps || t > 1.0 + eps {
                continue;
            }
            let y = p0.y + t * dy;
            if y + eps >= min_y && y <= max_y + eps {
                candidates.push((t, LayoutPoint { x: x_edge, y }));
            }
        }
    }

    if dy.abs() > eps {
        for y_edge in [min_y, max_y] {
            let t = (y_edge - p0.y) / dy;
            if t < -eps || t > 1.0 + eps {
                continue;
            }
            let x = p0.x + t * dx;
            if x + eps >= min_x && x <= max_x + eps {
                candidates.push((t, LayoutPoint { x, y: y_edge }));
            }
        }
    }

    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    candidates
        .into_iter()
        .find(|(t, _)| *t >= 0.0)
        .map(|(_, p)| p)
}

fn clip_edge_endpoints(points: &mut [LayoutPoint], from: Rect, to: Rect) {
    if points.len() < 2 {
        return;
    }
    if from.contains_point(points[0].x, points[0].y) {
        if let Some(p) = intersect_segment_with_rect(&points[0], &points[1], from) {
            points[0] = p;
        }
    }
    let last = points.len() - 1;
    if to.contains_point(points[last].x, points[last].y) {
        if let Some(p) = intersect_segment_with_rect(&points[last], &points[last - 1], to) {
            points[last] = p;
        }
    }
}

fn er_marker_id(card: &str, suffix: &str) -> Option<String> {
    match card {
        "ONLY_ONE" => Some(format!("ONLY_ONE_{suffix}")),
        "ZERO_OR_ONE" => Some(format!("ZERO_OR_ONE_{suffix}")),
        "ONE_OR_MORE" => Some(format!("ONE_OR_MORE_{suffix}")),
        "ZERO_OR_MORE" => Some(format!("ZERO_OR_MORE_{suffix}")),
        // Mermaid CLI ER output does not emit a dedicated MD_PARENT marker.
        "MD_PARENT" => None,
        _ => None,
    }
}

pub fn layout_er_diagram(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<ErDiagramLayout> {
    let model: ErModel = crate::json::from_value_ref(semantic)?;
    layout_er_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_er_diagram_typed(
    model: &merman_core::diagrams::er::ErDiagramRenderModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<ErDiagramLayout> {
    let nodesep = config_f64(effective_config, &["er", "nodeSpacing"]).unwrap_or(140.0);
    let ranksep = config_f64(effective_config, &["er", "rankSpacing"]).unwrap_or(80.0);
    let dir = rank_dir_from(&model.direction);

    let label_style = er_text_style(effective_config);
    let attr_style = TextStyle {
        font_family: label_style.font_family.clone(),
        font_size: label_style.font_size.max(1.0),
        font_weight: None,
    };
    let rel_label_style = TextStyle {
        font_family: label_style.font_family.clone(),
        // Mermaid ER relationship labels stay at a fixed 14px in the emitted stylesheet.
        font_size: 14.0,
        font_weight: None,
    };
    let rel_html_labels = er_relationship_html_labels(effective_config);

    let mut g = Graph::<NodeLabel, EdgeLabel, GraphLabel>::new(GraphOptions {
        directed: true,
        multigraph: true,
        // Mermaid's dagre adapter always enables `compound: true` (even if there are no clusters).
        // This also makes the ranker behavior match upstream for disconnected ER graphs.
        compound: true,
    });
    g.set_graph(GraphLabel {
        rankdir: dir,
        nodesep,
        ranksep,
        // Dagre's default `acyclicer` is "greedy" (Mermaid relies on this default).
        acyclicer: Some("greedy".to_string()),
        ..Default::default()
    });

    fn parse_entity_counter_from_id(id: &str) -> Option<usize> {
        let (_prefix, tail) = id.rsplit_once('-')?;
        tail.parse::<usize>().ok()
    }

    // Nodes.
    let mut entities_in_layout_order: Vec<&ErEntity> = model.entities.values().collect();
    entities_in_layout_order.sort_by(|a, b| {
        let a_key = (parse_entity_counter_from_id(&a.id), a.id.as_str());
        let b_key = (parse_entity_counter_from_id(&b.id), b.id.as_str());
        a_key.cmp(&b_key)
    });

    for e in entities_in_layout_order {
        let (w, h) =
            entity_box_dimensions(e, measurer, &label_style, &attr_style, effective_config);
        g.set_node(
            e.id.clone(),
            NodeLabel {
                width: w,
                height: h,
                ..Default::default()
            },
        );
    }

    // Edges. Mermaid ER uses edge labels ("roleA") and the unified renderer routes through the
    // generic dagre pipeline, which accounts for label bbox in spacing. Mirror that by giving
    // dagre real label sizes here.
    for (idx, r) in model.relationships.iter().enumerate() {
        if g.node(&r.entity_a).is_none() || g.node(&r.entity_b).is_none() {
            return Err(Error::InvalidModel {
                message: format!(
                    "relationship references missing entities: {} -> {}",
                    r.entity_a, r.entity_b
                ),
            });
        }

        // Mermaid's dagre renderer splits self-loops into three edges and introduces two helper
        // nodes (labelRect). Mermaid initializes them at 10x10, but after `updateNodeBounds(...)`
        // an empty labelRect collapses to ~0.1x0.1 and that is what Dagre uses for spacing.
        // Match that here for layout parity.
        if r.entity_a == r.entity_b {
            let node_id = r.entity_a.as_str();
            let special_1 = format!("{node_id}---{node_id}---1");
            let special_2 = format!("{node_id}---{node_id}---2");

            if g.node(&special_1).is_none() {
                g.set_node(
                    special_1.clone(),
                    NodeLabel {
                        width: 0.1,
                        height: 0.1,
                        ..Default::default()
                    },
                );
            }
            if g.node(&special_2).is_none() {
                g.set_node(
                    special_2.clone(),
                    NodeLabel {
                        width: 0.1,
                        height: 0.1,
                        ..Default::default()
                    },
                );
            }

            let (label_w, label_h) = if r.role_a.trim().is_empty() {
                (0.0, 0.0)
            } else {
                edge_label_metrics(&r.role_a, measurer, &rel_label_style, rel_html_labels)
            };

            // First segment: keep start marker, no label.
            g.set_edge_named(
                r.entity_a.clone(),
                special_1.clone(),
                Some(format!("er-rel-{idx}-cyclic-0")),
                Some(EdgeLabel {
                    width: 0.0,
                    height: 0.0,
                    labelpos: LabelPos::C,
                    labeloffset: 10.0,
                    minlen: 1,
                    weight: 1.0,
                    ..Default::default()
                }),
            );

            // Mid segment: carries the relationship label, no markers.
            g.set_edge_named(
                special_1.clone(),
                special_2.clone(),
                Some(format!("er-rel-{idx}")),
                Some(EdgeLabel {
                    width: label_w.max(0.0),
                    height: label_h.max(0.0),
                    labelpos: LabelPos::C,
                    labeloffset: 10.0,
                    minlen: 1,
                    weight: 1.0,
                    ..Default::default()
                }),
            );

            // Last segment: keep end marker, no label.
            g.set_edge_named(
                special_2.clone(),
                r.entity_a.clone(),
                Some(format!("er-rel-{idx}-cyclic-2")),
                Some(EdgeLabel {
                    width: 0.0,
                    height: 0.0,
                    labelpos: LabelPos::C,
                    labeloffset: 10.0,
                    minlen: 1,
                    weight: 1.0,
                    ..Default::default()
                }),
            );

            continue;
        }

        let name = format!("er-rel-{idx}");
        let (label_w, label_h) = if r.role_a.trim().is_empty() {
            (0.0, 0.0)
        } else {
            edge_label_metrics(&r.role_a, measurer, &rel_label_style, rel_html_labels)
        };
        g.set_edge_named(
            r.entity_a.clone(),
            r.entity_b.clone(),
            Some(name),
            Some(EdgeLabel {
                width: label_w.max(0.0),
                height: label_h.max(0.0),
                labelpos: LabelPos::C,
                labeloffset: 10.0,
                minlen: 1,
                weight: 1.0,
                ..Default::default()
            }),
        );
    }

    dugong::layout_dagreish(&mut g);

    let mut nodes: Vec<LayoutNode> = Vec::new();
    for id in g.node_ids() {
        let Some(n) = g.node(&id) else {
            continue;
        };
        nodes.push(LayoutNode {
            id: id.clone(),
            x: n.x.unwrap_or(0.0),
            y: n.y.unwrap_or(0.0),
            width: n.width,
            height: n.height,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });
    }
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut node_rect_by_id: HashMap<String, Rect> = HashMap::new();
    for n in &nodes {
        node_rect_by_id.insert(n.id.clone(), Rect::from_center(n.x, n.y, n.width, n.height));
    }

    let mut edges: Vec<LayoutEdgeParts> = Vec::new();
    for key in g.edge_keys() {
        let Some(e) = g.edge_by_key(&key) else {
            continue;
        };
        let mut points = e
            .points
            .iter()
            .map(|p| LayoutPoint { x: p.x, y: p.y })
            .collect::<Vec<_>>();

        let id = key
            .name
            .clone()
            .unwrap_or_else(|| format!("edge:{}:{}", key.v, key.w));

        let rel_idx = key
            .name
            .as_ref()
            .and_then(|name| parse_er_rel_idx_from_edge_name(name))
            .and_then(|idx| model.relationships.get(idx).map(|_| idx));

        let rel = rel_idx.and_then(|idx| model.relationships.get(idx));
        let role = rel.map(|r| r.role_a.clone()).unwrap_or_default();

        let (base_start_marker, base_end_marker, stroke_dasharray) = if let Some(rel) = rel {
            let card_a = rel.rel_spec.card_a.as_str();
            let card_b = rel.rel_spec.card_b.as_str();
            let rel_type = rel.rel_spec.rel_type.as_str();
            let start_marker = er_marker_id(card_b, "START");
            let end_marker = er_marker_id(card_a, "END");
            let stroke_dasharray = if rel_type == "NON_IDENTIFYING" {
                Some("8,8".to_string())
            } else {
                None
            };
            (start_marker, end_marker, stroke_dasharray)
        } else {
            (None, None, None)
        };

        if !is_er_self_loop_dummy_node_id(&key.v) && !is_er_self_loop_dummy_node_id(&key.w) {
            if let (Some(from_rect), Some(to_rect)) = (
                node_rect_by_id.get(&key.v).copied(),
                node_rect_by_id.get(&key.w).copied(),
            ) {
                clip_edge_endpoints(&mut points, from_rect, to_rect);
            }
        }

        let (start_marker, end_marker) =
            if is_er_self_loop_dummy_node_id(&key.v) && is_er_self_loop_dummy_node_id(&key.w) {
                (None, None)
            } else if id.ends_with("-cyclic-0") {
                (base_start_marker, None)
            } else if id.ends_with("-cyclic-2") {
                (None, base_end_marker)
            } else {
                (base_start_marker, base_end_marker)
            };

        let label =
            if role.trim().is_empty() || id.ends_with("-cyclic-0") || id.ends_with("-cyclic-2") {
                None
            } else {
                let (w, h) = edge_label_metrics(&role, measurer, &rel_label_style, rel_html_labels);
                // Mermaid uses Dagre's computed edge label center (`edge.x/edge.y`) rather than a
                // polyline midpoint. Prefer those coordinates when present.
                let (x, y) =
                    e.x.zip(e.y)
                        .or_else(|| calc_label_position(&points))
                        .unwrap_or((0.0, 0.0));
                Some(LayoutLabel {
                    x,
                    y,
                    width: w.max(1.0),
                    height: h.max(1.0),
                })
            };

        edges.push(LayoutEdgeParts {
            id,
            from: key.v.clone(),
            to: key.w.clone(),
            points,
            label,
            start_marker,
            end_marker,
            stroke_dasharray,
        });
    }
    edges.sort_by(|a, b| a.id.cmp(&b.id));

    let mut out_edges: Vec<LayoutEdge> = Vec::new();
    for e in edges {
        out_edges.push(LayoutEdge {
            id: e.id,
            from: e.from,
            to: e.to,
            from_cluster: None,
            to_cluster: None,
            points: e.points,
            label: e.label,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: e.start_marker,
            end_marker: e.end_marker,
            stroke_dasharray: e.stroke_dasharray,
        });
    }

    let bounds = {
        let mut points: Vec<(f64, f64)> = Vec::new();
        for n in &nodes {
            let hw = n.width / 2.0;
            let hh = n.height / 2.0;
            points.push((n.x - hw, n.y - hh));
            points.push((n.x + hw, n.y + hh));
        }
        for e in &out_edges {
            for p in &e.points {
                points.push((p.x, p.y));
            }
            if let Some(l) = &e.label {
                let hw = l.width / 2.0;
                let hh = l.height / 2.0;
                points.push((l.x - hw, l.y - hh));
                points.push((l.x + hw, l.y + hh));
            }
        }
        Bounds::from_points(points)
    };

    Ok(ErDiagramLayout {
        nodes,
        edges: out_edges,
        bounds,
    })
}

#[cfg(test)]
mod tests {
    use crate::text::{TextStyle, VendoredFontMetricsTextMeasurer};
    use serde_json::json;

    fn default_style() -> TextStyle {
        TextStyle {
            font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
            font_size: 16.0,
            font_weight: None,
        }
    }

    #[test]
    fn er_html_label_metrics_use_er_owned_width_overrides() {
        let measurer = VendoredFontMetricsTextMeasurer::default();
        let metrics = super::er_html_label_metrics("type<T>", &measurer, &default_style());

        assert_eq!(metrics.width, 57.953125);
        assert_eq!(metrics.height, 24.0);
    }

    #[test]
    fn er_label_markdown_detection_requires_structural_markup() {
        assert!(super::er_label_has_structural_markdown("last*Name*"));
        assert!(super::er_label_has_structural_markdown("__phone__"));
        assert!(super::er_label_has_structural_markdown("`code`"));
        assert!(!super::er_label_has_structural_markdown("*id"));
        assert!(!super::er_label_has_structural_markdown("driver_license"));
    }

    #[test]
    fn er_generic_markdown_plain_text_strips_wrapping_delimiters_only() {
        assert_eq!(
            super::er_generic_markdown_plain_text("*string(99)<T<<~>>>*").as_deref(),
            Some("string(99)<T<<~>>>")
        );
        assert_eq!(
            super::er_generic_markdown_plain_text("string(99)<T<<~>>>"),
            None
        );
    }

    #[test]
    fn er_relationship_htmllabels_follow_root_then_flowchart_config() {
        assert!(super::er_relationship_html_labels(&json!({})));
        assert!(super::er_relationship_html_labels(&json!({
            "flowchart": { "htmlLabels": true }
        })));
        assert!(!super::er_relationship_html_labels(&json!({
            "flowchart": { "htmlLabels": false }
        })));
        assert!(super::er_relationship_html_labels(&json!({
            "htmlLabels": true,
            "flowchart": { "htmlLabels": false }
        })));
        assert!(!super::er_relationship_html_labels(&json!({
            "htmlLabels": false,
            "flowchart": { "htmlLabels": true }
        })));
    }

    #[test]
    fn er_text_style_uses_theme_font_family_css() {
        let style = super::er_text_style(&json!({
            "fontFamily": "Courier, monospace",
            "themeVariables": {
                "fontFamily": "\"IBM Plex Sans\", Arial, sans-serif"
            }
        }));

        assert_eq!(
            style.font_family.as_deref(),
            Some(r#""IBM Plex Sans",Arial,sans-serif"#)
        );
    }
}
