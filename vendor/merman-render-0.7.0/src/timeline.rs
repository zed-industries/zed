use crate::Result;
use crate::config::{
    config_bool as cfg_bool, config_f64 as cfg_f64, config_f64_css_px as cfg_f64_css_px,
    config_string as cfg_string,
};
use crate::model::{
    Bounds, TimelineDiagramLayout, TimelineLineLayout, TimelineNodeLayout, TimelineSectionLayout,
    TimelineTaskLayout,
};
use crate::text::{TextMeasurer, TextStyle, normalize_font_key};
use merman_core::diagrams::timeline::{TimelineDiagramRenderModel, TimelineRenderTask};
use std::borrow::Cow;

const MAX_SECTIONS: i64 = 12;

const BASE_MARGIN: f64 = 50.0;
const NODE_PADDING: f64 = 20.0;
const TASK_STEP_X: f64 = 200.0;
const TASK_CONTENT_WIDTH_DEFAULT: f64 = 150.0;
const EVENT_VERTICAL_OFFSET_FROM_TASK_Y: f64 = 200.0;
const EVENT_GAP_Y: f64 = 10.0;

const TITLE_Y: f64 = 20.0;
const DEFAULT_VIEWBOX_PADDING: f64 = 50.0;

fn timeline_text_style(effective_config: &serde_json::Value) -> TextStyle {
    let font_family = cfg_string(effective_config, &["themeVariables", "fontFamily"])
        .or_else(|| cfg_string(effective_config, &["fontFamily"]))
        .map(|s| s.trim().trim_end_matches(';').trim().to_string())
        .filter(|s| !s.is_empty());
    let font_size =
        crate::config::config_theme_or_root_font_size_px(effective_config, 16.0).max(1.0);
    TextStyle {
        font_family,
        font_size,
        font_weight: None,
    }
}

fn timeline_font_key(style: &TextStyle) -> String {
    style
        .font_family
        .as_deref()
        .map(normalize_font_key)
        .unwrap_or_default()
}

fn timeline_uses_fira_sans_browser_fallback(style: &TextStyle) -> bool {
    timeline_font_key(style) == "firasans"
}

fn timeline_measurement_style(style: &TextStyle) -> Cow<'_, TextStyle> {
    if timeline_uses_fira_sans_browser_fallback(style) {
        // Edge/Chromium in the pinned Mermaid CLI environment resolves bare `Fira Sans` to the
        // generic sans-serif face. Use the same metrics for Timeline's SVG getBBox probes.
        let mut style = style.clone();
        style.font_family = Some("sans-serif".to_string());
        Cow::Owned(style)
    } else {
        Cow::Borrowed(style)
    }
}

fn section_index(full_section: i64) -> i64 {
    (full_section % MAX_SECTIONS) - 1
}

fn section_class(full_section: i64) -> String {
    format!("section-{}", section_index(full_section))
}

fn next_char_at(text: &str, idx: usize) -> Option<char> {
    text.get(idx..)?.chars().next()
}

fn wrap_tokens(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut buf = String::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let Some(ch) = next_char_at(text, i) else {
            break;
        };
        if ch.is_whitespace() {
            if !buf.is_empty() {
                out.push(std::mem::take(&mut buf));
            }
            // Coalesce any whitespace run into a single token.
            while i < bytes.len() {
                let Some(c) = next_char_at(text, i) else {
                    break;
                };
                if !c.is_whitespace() {
                    break;
                }
                i += c.len_utf8();
            }
            out.push(" ".to_string());
            continue;
        }

        let Some(rest) = text.get(i..) else {
            break;
        };
        if rest.starts_with("<br>") || rest.starts_with("<br/>") || rest.starts_with("<br />") {
            if !buf.is_empty() {
                out.push(std::mem::take(&mut buf));
            }
            if rest.starts_with("<br>") {
                i += "<br>".len();
            } else if rest.starts_with("<br/>") {
                i += "<br/>".len();
            } else {
                i += "<br />".len();
            }
            out.push("<br>".to_string());
            continue;
        }

        buf.push(ch);
        i += ch.len_utf8();
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

fn join_trim(tokens: &[String]) -> String {
    tokens.join(" ").trim().to_string()
}

fn svg_collapse_whitespace_for_measure(s: &str) -> Cow<'_, str> {
    // Mermaid timeline wrap decisions use `tspan.getComputedTextLength()`, which measures the
    // rendered text. SVG text collapses whitespace runs unless `xml:space="preserve"` is set.
    // Mermaid does not set `xml:space`, so we mirror that collapsing here.
    let mut out: Option<String> = None;
    let mut last_space = false;
    let mut saw_non_space = false;

    for ch in s.chars() {
        if ch.is_whitespace() {
            if !saw_non_space || last_space {
                continue;
            }
            out.get_or_insert_with(|| String::with_capacity(s.len()))
                .push(' ');
            last_space = true;
        } else {
            saw_non_space = true;
            out.get_or_insert_with(|| String::with_capacity(s.len()))
                .push(ch);
            last_space = false;
        }
    }

    let Some(mut out) = out else {
        return Cow::Borrowed(s.trim());
    };
    if out.ends_with(' ') {
        out.pop();
    }
    Cow::Owned(out)
}

fn wrap_lines(
    text: &str,
    max_width: f64,
    style: &TextStyle,
    measurer: &dyn TextMeasurer,
) -> Vec<String> {
    let tokens = wrap_tokens(text);
    if tokens.is_empty() {
        return vec![String::new()];
    }

    let mut lines: Vec<String> = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let measurement_style = timeline_measurement_style(style);

    for tok in tokens {
        cur.push(tok.clone());
        let candidate = join_trim(&cur);
        let candidate = svg_collapse_whitespace_for_measure(&candidate);
        // Mermaid uses `getComputedTextLength()` here, but our headless measurer cannot reproduce
        // the exact font fallback behavior of the Puppeteer baselines. Empirically, the single-run
        // SVG `getBBox().width` probe is a closer and more stable approximation for wrap
        // boundaries in timeline fixtures.
        let candidate_width =
            measurer.measure_svg_simple_text_bbox_width_px(&candidate, measurement_style.as_ref());
        if candidate_width > max_width || tok == "<br>" {
            cur.pop();
            lines.push(join_trim(&cur));
            if tok == "<br>" {
                cur = vec![String::new()];
            } else {
                cur = vec![tok];
            }
        }
    }

    lines.push(join_trim(&cur));
    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

fn timeline_first_line_bbox_height_px(style: &TextStyle) -> f64 {
    if timeline_uses_fira_sans_browser_fallback(style) && (style.font_size - 17.0).abs() <= 0.01 {
        return 25.0;
    }

    let first_line_em = 1.1875;
    (style.font_size.max(1.0) * first_line_em).floor()
}

fn text_bbox_height(lines: &[String], style: &TextStyle) -> f64 {
    // Mermaid timeline measures SVG `<text>.getBBox().height` (see upstream `svgDraw.js`).
    //
    // In the pinned Mermaid CLI/browser baselines, the bbox behaves as:
    // - first line: ~1.1875em (Trebuchet MS default)
    // - additional lines: 1.1em each
    let font_size = style.font_size.max(1.0);
    let lines = lines.iter().filter(|l| !l.trim().is_empty()).count();
    if lines == 0 {
        return 0.0;
    }
    // Empirically, browser `getBBox().height` for SVG `<text>` in upstream timeline fixtures can
    // "snap" the first-line height down to an integer for non-default font sizes (notably when
    // the diagram sets `themeVariables.fontSize`). Mirror that so our layout stays on the same
    // lattice as upstream `mmdc` baselines.
    let first = timeline_first_line_bbox_height_px(style);
    let additional = (lines.saturating_sub(1) as f64) * font_size * 1.1;
    first + additional
}

fn virtual_node_height(
    text: &str,
    content_width: f64,
    style: &TextStyle,
    layout_font_size: f64,
    padding: f64,
    measurer: &dyn TextMeasurer,
) -> (f64, Vec<String>) {
    // Mermaid timeline `wrap()` compares `tspan.getComputedTextLength()` against `node.width`
    // (the configured inner width, excluding padding).
    let lines = wrap_lines(text, content_width.max(1.0), style, measurer);
    let bbox_h = text_bbox_height(&lines, style);
    // Mermaid timeline uses `conf.fontSize` (top-level `config.fontSize`) for the extra vertical
    // offset, even when the actual rendered font size comes from `themeVariables.fontSize`.
    let h = bbox_h + layout_font_size.max(1.0) * 1.1 * 0.5 + padding;
    (h, lines)
}

#[derive(Debug, Clone, Copy)]
struct TimelineNodeRequest<'a> {
    kind: &'a str,
    label: &'a str,
    full_section: i64,
    x: f64,
    y: f64,
    content_width: f64,
    max_height: f64,
    style: &'a TextStyle,
    layout_font_size: f64,
}

fn compute_node(
    request: TimelineNodeRequest<'_>,
    measurer: &dyn TextMeasurer,
) -> TimelineNodeLayout {
    let TimelineNodeRequest {
        kind,
        label,
        full_section,
        x,
        y,
        content_width,
        max_height,
        style,
        layout_font_size,
    } = request;
    let (h0, label_lines) = virtual_node_height(
        label,
        content_width,
        style,
        layout_font_size,
        NODE_PADDING,
        measurer,
    );
    let height = h0.max(max_height).max(1.0);
    let width = (content_width + NODE_PADDING * 2.0).max(1.0);
    TimelineNodeLayout {
        x,
        y,
        width,
        height,
        content_width: content_width.max(1.0),
        padding: NODE_PADDING,
        section_class: section_class(full_section),
        label: label.to_string(),
        label_lines,
        kind: kind.to_string(),
    }
}

fn bounds_from_nodes_and_lines<'a, 'b>(
    nodes: impl IntoIterator<Item = &'a TimelineNodeLayout>,
    lines: impl IntoIterator<Item = &'b TimelineLineLayout>,
) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    let mut any = false;
    for n in nodes {
        any = true;
        min_x = min_x.min(n.x);
        min_y = min_y.min(n.y);
        max_x = max_x.max(n.x + n.width);
        max_y = max_y.max(n.y + n.height);
    }
    for l in lines {
        any = true;
        min_x = min_x.min(l.x1.min(l.x2));
        min_y = min_y.min(l.y1.min(l.y2));
        max_x = max_x.max(l.x1.max(l.x2));
        max_y = max_y.max(l.y1.max(l.y2));
    }

    if any {
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    }
}

fn timeline_svg_bbox_x_with_ascii_overhang_override_px(
    font_key: &str,
    font_size_px: f64,
    text: &str,
) -> Option<(f64, f64)> {
    crate::generated::timeline_text_overrides_11_12_2::
        lookup_timeline_svg_bbox_x_with_ascii_overhang_px(font_key, font_size_px, text)
}

fn expand_bounds_for_node_text(
    min_x: &mut f64,
    _min_y: &mut f64,
    max_x: &mut f64,
    _max_y: &mut f64,
    nodes: &[TimelineNodeLayout],
    style: &TextStyle,
    measurer: &dyn TextMeasurer,
) {
    for n in nodes {
        if n.kind == "title-bounds" {
            continue;
        }

        let anchor_x = n.x + n.width / 2.0;
        for line in &n.label_lines {
            if line.trim().is_empty() {
                continue;
            }
            // Timeline node labels can overflow the node shape. Mermaid computes the final
            // viewport from SVG `getBBox()`, which includes glyph overhang and can be asymmetric
            // even for ASCII strings (observed in upstream fixtures).
            let (left, right) = timeline_svg_bbox_x_with_ascii_overhang_override_px(
                style.font_family.as_deref().unwrap_or_default(),
                style.font_size,
                line,
            )
            .unwrap_or_else(|| measurer.measure_svg_text_bbox_x_with_ascii_overhang(line, style));
            *min_x = (*min_x).min(anchor_x - left);
            *max_x = (*max_x).max(anchor_x + right);
        }
    }
}

pub fn layout_timeline_diagram(
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<TimelineDiagramLayout> {
    let model: TimelineDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_timeline_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_timeline_diagram_typed(
    model: &TimelineDiagramRenderModel,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<TimelineDiagramLayout> {
    let _ = (model.acc_title.as_deref(), model.acc_descr.as_deref());

    let text_style = timeline_text_style(effective_config);
    let render_font_size = text_style.font_size;
    let layout_font_size = cfg_f64_css_px(effective_config, &["fontSize"])
        .unwrap_or(render_font_size)
        .max(1.0);

    let left_margin = cfg_f64(effective_config, &["timeline", "leftMargin"])
        .unwrap_or(150.0)
        .max(0.0);
    let disable_multicolor =
        cfg_bool(effective_config, &["timeline", "disableMulticolor"]).unwrap_or(false);
    // Mermaid's Timeline renderer hardcodes the text-wrap width to `150` (see upstream
    // `drawTasks`/`drawEvents`: node objects use `width: 150` and `wrap(..., node.width)`),
    // even though the config schema exposes a `timeline.width` field.
    //
    // For upstream parity, treat `timeline.width` as a no-op and keep the wrap width constant.
    let task_content_width = TASK_CONTENT_WIDTH_DEFAULT;
    let _ = cfg_f64(effective_config, &["timeline", "width"]);

    let mut max_section_height: f64 = 0.0;
    for section in &model.sections {
        let (h, _lines) = virtual_node_height(
            section,
            task_content_width,
            &text_style,
            layout_font_size,
            NODE_PADDING,
            measurer,
        );
        max_section_height = max_section_height.max(h + 20.0);
    }

    let mut max_task_height: f64 = 0.0;
    let mut max_event_line_length: f64 = 0.0;
    for task in &model.tasks {
        // Upstream Mermaid's Timeline renderer computes `maxTaskHeight` by passing the entire
        // task object into `getVirtualNodeHeight(...)`, which stringifies to `"[object Object]"`.
        // This inflates `maxTaskHeight` when all task labels are short; replicate for parity.
        let virtual_task_label = "[object Object]";
        let (h, _lines) = virtual_node_height(
            virtual_task_label,
            task_content_width,
            &text_style,
            layout_font_size,
            NODE_PADDING,
            measurer,
        );
        max_task_height = max_task_height.max(h + 20.0);

        let mut task_event_len: f64 = 0.0;
        for ev in &task.events {
            let (eh, _lines) = virtual_node_height(
                ev,
                task_content_width,
                &text_style,
                layout_font_size,
                NODE_PADDING,
                measurer,
            );
            task_event_len += eh;
        }
        if !task.events.is_empty() {
            task_event_len += (task.events.len().saturating_sub(1) as f64) * EVENT_GAP_Y;
        }
        max_event_line_length = max_event_line_length.max(task_event_len);
    }

    let base_x = BASE_MARGIN + left_margin;
    let base_y = BASE_MARGIN;

    let mut sections: Vec<TimelineSectionLayout> = Vec::new();
    let mut orphan_tasks: Vec<TimelineTaskLayout> = Vec::new();

    let mut all_nodes_pre_title: Vec<TimelineNodeLayout> = Vec::new();
    let mut all_lines_pre_title: Vec<TimelineLineLayout> = Vec::new();

    let has_sections = !model.sections.is_empty();

    if has_sections {
        let mut master_x = base_x;
        let section_y = base_y;

        for (section_number, section_label) in model.sections.iter().enumerate() {
            let section_number = section_number as i64;
            let tasks_for_section: Vec<&TimelineRenderTask> = model
                .tasks
                .iter()
                .filter(|t| t.section == *section_label)
                .collect();
            let tasks_for_section_count = tasks_for_section.len().max(1);

            let content_width = TASK_STEP_X * (tasks_for_section_count as f64) - 50.0;
            let section_node = compute_node(
                TimelineNodeRequest {
                    kind: "section",
                    label: section_label,
                    full_section: section_number,
                    x: master_x,
                    y: section_y,
                    content_width,
                    max_height: max_section_height,
                    style: &text_style,
                    layout_font_size,
                },
                measurer,
            );
            all_nodes_pre_title.push(section_node.clone());

            let mut tasks: Vec<TimelineTaskLayout> = Vec::new();
            let mut task_x = master_x;
            let task_y = section_y + max_section_height + 50.0;

            for task in &tasks_for_section {
                let full_section = section_number;
                let task_node = compute_node(
                    TimelineNodeRequest {
                        kind: "task",
                        label: &task.task,
                        full_section,
                        x: task_x,
                        y: task_y,
                        content_width: task_content_width,
                        max_height: max_task_height,
                        style: &text_style,
                        layout_font_size,
                    },
                    measurer,
                );
                all_nodes_pre_title.push(task_node.clone());

                let connector = TimelineLineLayout {
                    kind: "task-events".to_string(),
                    x1: task_x + (task_node.width / 2.0),
                    y1: task_y + max_task_height,
                    x2: task_x + (task_node.width / 2.0),
                    y2: task_y + max_task_height + 100.0 + max_event_line_length + 100.0,
                };
                all_lines_pre_title.push(connector.clone());

                let mut events: Vec<TimelineNodeLayout> = Vec::new();
                let mut event_y = task_y + EVENT_VERTICAL_OFFSET_FROM_TASK_Y;
                for ev in &task.events {
                    let event_node = compute_node(
                        TimelineNodeRequest {
                            kind: "event",
                            label: ev,
                            full_section,
                            x: task_x,
                            y: event_y,
                            content_width: task_content_width,
                            max_height: 50.0,
                            style: &text_style,
                            layout_font_size,
                        },
                        measurer,
                    );
                    event_y += event_node.height + EVENT_GAP_Y;
                    all_nodes_pre_title.push(event_node.clone());
                    events.push(event_node);
                }

                tasks.push(TimelineTaskLayout {
                    node: task_node,
                    connector,
                    events,
                });

                task_x += TASK_STEP_X;
            }

            sections.push(TimelineSectionLayout {
                node: section_node,
                tasks,
            });

            master_x += TASK_STEP_X * (tasks_for_section_count as f64);
        }
    } else {
        let mut master_x = base_x;
        let master_y = base_y;
        let mut section_color: i64 = 0;

        for task in &model.tasks {
            let task_node = compute_node(
                TimelineNodeRequest {
                    kind: "task",
                    label: &task.task,
                    full_section: section_color,
                    x: master_x,
                    y: master_y,
                    content_width: task_content_width,
                    max_height: max_task_height,
                    style: &text_style,
                    layout_font_size,
                },
                measurer,
            );
            all_nodes_pre_title.push(task_node.clone());

            let connector = TimelineLineLayout {
                kind: "task-events".to_string(),
                x1: master_x + (task_node.width / 2.0),
                y1: master_y + max_task_height,
                x2: master_x + (task_node.width / 2.0),
                y2: master_y + max_task_height + 100.0 + max_event_line_length + 100.0,
            };
            all_lines_pre_title.push(connector.clone());

            let mut events: Vec<TimelineNodeLayout> = Vec::new();
            let mut event_y = master_y + EVENT_VERTICAL_OFFSET_FROM_TASK_Y;
            for ev in &task.events {
                let event_node = compute_node(
                    TimelineNodeRequest {
                        kind: "event",
                        label: ev,
                        full_section: section_color,
                        x: master_x,
                        y: event_y,
                        content_width: task_content_width,
                        max_height: 50.0,
                        style: &text_style,
                        layout_font_size,
                    },
                    measurer,
                );
                event_y += event_node.height + EVENT_GAP_Y;
                all_nodes_pre_title.push(event_node.clone());
                events.push(event_node);
            }

            orphan_tasks.push(TimelineTaskLayout {
                node: task_node,
                connector,
                events,
            });

            master_x += TASK_STEP_X;
            if !disable_multicolor {
                section_color += 1;
            }
        }
    }

    let pre_title_bounds = bounds_from_nodes_and_lines(&all_nodes_pre_title, &all_lines_pre_title);
    let has_pre_title_content = pre_title_bounds.is_some();
    let (mut pre_min_x, mut pre_min_y, mut pre_max_x, mut pre_max_y) =
        pre_title_bounds.unwrap_or((0.0, 0.0, 0.0, 0.0));
    if has_pre_title_content {
        expand_bounds_for_node_text(
            &mut pre_min_x,
            &mut pre_min_y,
            &mut pre_max_x,
            &mut pre_max_y,
            &all_nodes_pre_title,
            &text_style,
            measurer,
        );
    }
    let pre_title_box_width = if has_pre_title_content {
        (pre_max_x - pre_min_x).max(1.0)
    } else {
        0.0
    };

    let title = model
        .title
        .as_deref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let title_x = pre_title_box_width / 2.0 - left_margin;

    let depth_y = if has_sections {
        max_section_height + max_task_height + 150.0
    } else {
        max_task_height + 100.0
    };

    let activity_line = TimelineLineLayout {
        kind: "activity".to_string(),
        x1: left_margin,
        y1: depth_y,
        x2: pre_title_box_width + 3.0 * left_margin,
        y2: depth_y,
    };

    let mut all_nodes_full: Vec<TimelineNodeLayout> = all_nodes_pre_title.clone();
    let mut all_lines_full: Vec<TimelineLineLayout> = all_lines_pre_title.clone();
    all_lines_full.push(activity_line.clone());

    if let Some(t) = title.as_deref() {
        // Approximate the title bounds so the viewBox can include it (Mermaid uses a bold 4ex).
        //
        // Note: `ex` depends on the font x-height; for Mermaid's default theme at 16px, `4ex`
        // resolves to ~31px in upstream fixtures.
        let title_font_size = render_font_size * 1.9375;
        let title_style = TextStyle {
            font_family: text_style.font_family.clone(),
            font_size: title_font_size,
            font_weight: Some("bold".to_string()),
        };
        let metrics = measurer.measure(t, &title_style);
        all_nodes_full.push(TimelineNodeLayout {
            x: title_x,
            y: TITLE_Y - title_style.font_size,
            width: metrics.width.max(1.0),
            height: title_style.font_size.max(1.0),
            content_width: metrics.width.max(1.0),
            padding: 0.0,
            section_class: "section-root".to_string(),
            label: t.to_string(),
            label_lines: vec![t.to_string()],
            kind: "title-bounds".to_string(),
        });
    }

    let (mut full_min_x, mut full_min_y, mut full_max_x, mut full_max_y) =
        bounds_from_nodes_and_lines(&all_nodes_full, &all_lines_full)
            .unwrap_or((pre_min_x, pre_min_y, pre_max_x, pre_max_y));
    expand_bounds_for_node_text(
        &mut full_min_x,
        &mut full_min_y,
        &mut full_max_x,
        &mut full_max_y,
        &all_nodes_full,
        &text_style,
        measurer,
    );

    let viewbox_padding =
        cfg_f64(effective_config, &["timeline", "padding"]).unwrap_or(DEFAULT_VIEWBOX_PADDING);
    let vb_min_x = full_min_x - viewbox_padding;
    let vb_min_y = full_min_y - viewbox_padding;
    let vb_max_x = full_max_x + viewbox_padding;
    let vb_max_y = full_max_y + viewbox_padding;

    Ok(TimelineDiagramLayout {
        bounds: Some(Bounds {
            min_x: vb_min_x,
            min_y: vb_min_y,
            max_x: vb_max_x,
            max_y: vb_max_y,
        }),
        left_margin,
        base_x,
        base_y,
        pre_title_box_width,
        sections,
        orphan_tasks,
        activity_line,
        title,
        title_x,
        title_y: TITLE_Y,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use merman_core::{Engine, ParseOptions};
    use std::path::PathBuf;

    const LONG_WORD: &str = "SupercalifragilisticexpialidociousSupercalifragilisticexpialidocious";

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
    }

    #[test]
    fn fira_sans_17_timeline_metrics_match_mermaid_browser_wrap() {
        let style = TextStyle {
            font_family: Some("Fira Sans".to_string()),
            font_size: 17.0,
            font_weight: None,
        };
        let measurer = crate::text::VendoredFontMetricsTextMeasurer::default();

        let (height, lines) = virtual_node_height(
            "Quality Management System (4)",
            TASK_CONTENT_WIDTH_DEFAULT,
            &style,
            16.0,
            NODE_PADDING,
            &measurer,
        );

        assert_eq!(lines, ["Quality", "Management", "System   (4)"]);
        assert!(
            (height - 91.2).abs() < 0.001,
            "expected Fira/Sans browser Timeline height to be 91.2px, got {height}"
        );
    }

    #[test]
    fn svg_override_paths_cover_long_word_bbox() {
        assert_eq!(
            timeline_svg_bbox_x_with_ascii_overhang_override_px(
                "\"trebuchet ms\", verdana, arial, sans-serif",
                16.0,
                LONG_WORD,
            ),
            Some((235.3203125, 235.3203125))
        );
        assert_eq!(
            timeline_svg_bbox_x_with_ascii_overhang_override_px("", 16.0, LONG_WORD),
            Some((235.3203125, 235.3203125))
        );
        assert_eq!(
            timeline_svg_bbox_x_with_ascii_overhang_override_px("courier", 16.0, "Line 2"),
            None
        );
    }

    #[test]
    fn long_word_wrap_keeps_upstream_activity_line_extent() {
        let path = workspace_root()
            .join("fixtures")
            .join("timeline")
            .join("upstream_long_word_wrap.mmd");
        let text = std::fs::read_to_string(&path).expect("fixture");

        let engine = Engine::new();
        let parsed =
            futures::executor::block_on(engine.parse_diagram(&text, ParseOptions::default()))
                .expect("parse ok")
                .expect("diagram detected");
        let out =
            crate::layout_parsed(&parsed, &crate::LayoutOptions::default()).expect("layout ok");
        let crate::model::LayoutDiagram::TimelineDiagram(layout) = out.layout else {
            panic!("expected TimelineDiagram layout");
        };

        let actual = layout.activity_line.x2;
        assert!(
            (actual - 920.640625).abs() < 0.0001,
            "expected long-word timeline activity line extent to stay aligned with upstream, got {actual}"
        );
    }

    #[test]
    fn empty_timeline_does_not_invent_pre_title_width() {
        let engine = Engine::new();
        let parsed =
            futures::executor::block_on(engine.parse_diagram("timeline", ParseOptions::default()))
                .expect("parse ok")
                .expect("diagram detected");
        let out =
            crate::layout_parsed(&parsed, &crate::LayoutOptions::default()).expect("layout ok");
        let crate::model::LayoutDiagram::TimelineDiagram(layout) = out.layout else {
            panic!("expected TimelineDiagram layout");
        };

        assert_eq!(layout.pre_title_box_width, 0.0);
        assert_eq!(layout.activity_line.x1, 150.0);
        assert_eq!(layout.activity_line.x2, 450.0);
        let bounds = layout.bounds.expect("bounds");
        assert_eq!(bounds.min_x, 100.0);
        assert_eq!(bounds.min_y, 50.0);
        assert_eq!(bounds.max_x, 500.0);
        assert_eq!(bounds.max_y, 150.0);
    }
}
