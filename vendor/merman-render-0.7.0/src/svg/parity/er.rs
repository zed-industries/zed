use super::*;

// ER diagram SVG renderer implementation (split from parity.rs).

pub(super) fn render_er_diagram_debug_svg(
    layout: &ErDiagramLayout,
    options: &SvgRenderOptions,
) -> String {
    let mut nodes = layout.nodes.clone();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut edges = layout.edges.clone();
    edges.sort_by(|a, b| a.id.cmp(&b.id));

    // Mermaid `setupViewPortForSVG` uses `svg.node().getBBox()`. In Chromium, ER edge labels are
    // rendered via `<foreignObject>` and do not reliably contribute to the root SVG bbox. Exclude
    // edge label boxes from our bounds computation so `viewBox` / translation matches upstream.
    let mut edges_for_bounds = edges.clone();
    for e in &mut edges_for_bounds {
        e.label = None;
        e.start_label_left = None;
        e.start_label_right = None;
        e.end_label_left = None;
        e.end_label_right = None;
    }

    let bounds = compute_layout_bounds(&[], &nodes, &edges_for_bounds).unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    });
    let pad = options.viewbox_padding.max(0.0);
    let vb_min_x = bounds.min_x - pad;
    let vb_min_y = bounds.min_y - pad;
    let vb_w = (bounds.max_x - bounds.min_x) + pad * 2.0;
    let vb_h = (bounds.max_y - bounds.min_y) + pad * 2.0;

    let mut out = String::new();
    let _ = writeln!(
        &mut out,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}">"#,
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w.max(1.0)),
        fmt(vb_h.max(1.0))
    );
    out.push_str(
        r#"<style>
 .node-box { fill: none; stroke: #2563eb; stroke-width: 1; }
 .node-label { fill: #1f2937; font-family: ui-sans-serif, system-ui, sans-serif; font-size: 11px; text-anchor: middle; dominant-baseline: middle; }
 .edge { fill: none; stroke: #111827; stroke-width: 1; }
 .edge-label-box { fill: #fef3c7; stroke: #92400e; stroke-width: 1; opacity: 0.6; }
 .debug-cross { stroke: #ef4444; stroke-width: 1; }
 </style>
 "#,
    );

    // Ported from Mermaid `@11.12.2` `erMarkers.js` (debug-only for now).
    out.push_str(
        r##"<defs>
  <marker id="MD_PARENT_START" refX="0" refY="7" markerWidth="190" markerHeight="240" orient="auto">
    <path d="M 18,7 L9,13 L1,7 L9,1 Z" fill="#111827" />
  </marker>
  <marker id="MD_PARENT_END" refX="19" refY="7" markerWidth="20" markerHeight="28" orient="auto">
    <path d="M 18,7 L9,13 L1,7 L9,1 Z" fill="#111827" />
  </marker>

  <marker id="ONLY_ONE_START" refX="0" refY="9" markerWidth="18" markerHeight="18" orient="auto">
    <path stroke="#111827" fill="none" d="M9,0 L9,18 M15,0 L15,18" />
  </marker>
  <marker id="ONLY_ONE_END" refX="18" refY="9" markerWidth="18" markerHeight="18" orient="auto">
    <path stroke="#111827" fill="none" d="M3,0 L3,18 M9,0 L9,18" />
  </marker>

  <marker id="ZERO_OR_ONE_START" refX="0" refY="9" markerWidth="30" markerHeight="18" orient="auto">
    <circle stroke="#111827" fill="white" cx="21" cy="9" r="6" />
    <path stroke="#111827" fill="none" d="M9,0 L9,18" />
  </marker>
  <marker id="ZERO_OR_ONE_END" refX="30" refY="9" markerWidth="30" markerHeight="18" orient="auto">
    <circle stroke="#111827" fill="white" cx="9" cy="9" r="6" />
    <path stroke="#111827" fill="none" d="M21,0 L21,18" />
  </marker>

  <marker id="ONE_OR_MORE_START" refX="18" refY="18" markerWidth="45" markerHeight="36" orient="auto">
    <path stroke="#111827" fill="none" d="M0,18 Q 18,0 36,18 Q 18,36 0,18 M42,9 L42,27" />
  </marker>
  <marker id="ONE_OR_MORE_END" refX="27" refY="18" markerWidth="45" markerHeight="36" orient="auto">
    <path stroke="#111827" fill="none" d="M3,9 L3,27 M9,18 Q27,0 45,18 Q27,36 9,18" />
  </marker>

  <marker id="ZERO_OR_MORE_START" refX="18" refY="18" markerWidth="57" markerHeight="36" orient="auto">
    <circle stroke="#111827" fill="white" cx="48" cy="18" r="6" />
    <path stroke="#111827" fill="none" d="M0,18 Q18,0 36,18 Q18,36 0,18" />
  </marker>
  <marker id="ZERO_OR_MORE_END" refX="39" refY="18" markerWidth="57" markerHeight="36" orient="auto">
    <circle stroke="#111827" fill="white" cx="9" cy="18" r="6" />
    <path stroke="#111827" fill="none" d="M21,18 Q39,0 57,18 Q39,36 21,18" />
  </marker>
</defs>
"##,
    );

    if options.include_edges {
        out.push_str(r#"<g class="edges">"#);
        for e in &edges {
            if e.points.len() >= 2 {
                let _ = write!(&mut out, r#"<polyline class="edge""#);
                if let Some(dash) = &e.stroke_dasharray {
                    let _ = write!(
                        &mut out,
                        r#" stroke-dasharray="{}""#,
                        escape_xml_display(dash)
                    );
                }
                if let Some(m) = &e.start_marker {
                    let _ = write!(
                        &mut out,
                        r#" marker-start="url(#{})""#,
                        escape_xml_display(m)
                    );
                }
                if let Some(m) = &e.end_marker {
                    let _ = write!(&mut out, r#" marker-end="url(#{})""#, escape_xml_display(m));
                }
                out.push_str(r#" points=""#);
                push_points_attr(&mut out, &e.points);
                out.push_str(r#"" />"#);
            }

            if let Some(lbl) = &e.label {
                let x = lbl.x - lbl.width / 2.0;
                let y = lbl.y - lbl.height / 2.0;
                let _ = write!(
                    &mut out,
                    r#"<rect class="edge-label-box" x="{}" y="{}" width="{}" height="{}" />"#,
                    fmt(x),
                    fmt(y),
                    fmt(lbl.width.max(1.0)),
                    fmt(lbl.height.max(1.0))
                );
                if options.include_edge_id_labels {
                    let _ = write!(
                        &mut out,
                        r#"<text class="node-label" x="{}" y="{}">{}</text>"#,
                        fmt(lbl.x),
                        fmt(lbl.y),
                        escape_xml_display(&e.id)
                    );
                }
            }
        }
        out.push_str("</g>\n");
    }

    if options.include_nodes {
        out.push_str(r#"<g class="nodes">"#);
        for n in &nodes {
            render_node(&mut out, n);
        }
        out.push_str("</g>\n");
    }

    out.push_str("</svg>\n");
    out
}

fn compile_er_entity_styles(
    entity: &crate::er::ErEntity,
    classes: &std::collections::BTreeMap<String, crate::er::ErClassDef>,
) -> (Vec<String>, Vec<String>) {
    let mut compiled_box: Vec<String> = Vec::new();
    let mut compiled_text: Vec<String> = Vec::new();
    let mut seen_classes: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for class_name in entity.css_classes.split_whitespace() {
        if !seen_classes.insert(class_name) {
            continue;
        }
        let Some(def) = classes.get(class_name) else {
            continue;
        };
        for s in &def.styles {
            let t = s.trim();
            if t.is_empty() {
                continue;
            }
            compiled_box.push(t.to_string());
        }
        for s in &def.text_styles {
            let t = s.trim();
            if t.is_empty() {
                continue;
            }
            compiled_text.push(t.to_string());
        }
    }

    let mut rect_map: std::collections::BTreeMap<String, String> =
        std::collections::BTreeMap::new();
    let mut text_map: std::collections::BTreeMap<String, String> =
        std::collections::BTreeMap::new();

    // Box styles: classDef styles + `style` statements.
    for s in compiled_box.iter().chain(entity.css_styles.iter()) {
        let Some((k, v)) = parse_style_decl(s) else {
            continue;
        };
        if is_rect_style_key(k) {
            rect_map.insert(k.to_string(), v.to_string());
        }
        // Mermaid treats `color:` as the HTML label text color (even if it comes from the style list).
        if k == "color" {
            text_map.insert("color".to_string(), v.to_string());
        }
    }

    // Text styles: classDef textStyles + `style` statements (only text-related keys).
    for s in compiled_text.iter().chain(entity.css_styles.iter()) {
        let Some((k, v)) = parse_style_decl(s) else {
            continue;
        };
        if !is_text_style_key(k) {
            continue;
        }
        if k == "color" {
            text_map.insert("color".to_string(), v.to_string());
        } else {
            text_map.insert(k.to_string(), v.to_string());
        }
    }

    let mut rect_decls: Vec<String> = Vec::new();
    for k in [
        "fill",
        "stroke",
        "stroke-width",
        "stroke-dasharray",
        "opacity",
        "fill-opacity",
        "stroke-opacity",
    ] {
        if let Some(v) = rect_map.get(k) {
            rect_decls.push(format!("{k}:{v}"));
        }
    }

    let mut text_decls: Vec<String> = Vec::new();
    for k in [
        "color",
        "font-family",
        "font-size",
        "font-weight",
        "opacity",
    ] {
        if let Some(v) = text_map.get(k) {
            text_decls.push(format!("{k}:{v}"));
        }
    }

    (rect_decls, text_decls)
}

fn style_decls_with_important_join(decls: &[String], join: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for d in decls {
        let Some((k, v)) = parse_style_decl(d) else {
            continue;
        };
        out.push(format!("{k}:{v} !important"));
    }
    out.join(join)
}

fn style_decls_with_important(decls: &[String]) -> String {
    style_decls_with_important_join(decls, "; ")
}

fn last_style_value(decls: &[String], key: &str) -> Option<String> {
    for d in decls.iter().rev() {
        let Some((k, v)) = parse_style_decl(d) else {
            continue;
        };
        if k == key {
            return Some(v.to_string());
        }
    }
    None
}

fn concat_style_keys(decls: &[String], keys: &[&str]) -> String {
    let mut out = String::new();
    for k in keys {
        if let Some(v) = last_style_value(decls, k) {
            out.push_str(k);
            out.push(':');
            out.push_str(&v);
        }
    }
    out
}

fn parse_px_f64(v: &str) -> Option<f64> {
    let raw = v.trim().trim_end_matches(';').trim();
    let raw = raw.trim_end_matches("px").trim();
    if raw.is_empty() {
        return None;
    }
    raw.parse::<f64>().ok()
}

fn is_label_coordinate_in_path(point: crate::model::LayoutPoint, d_attr: &str) -> bool {
    // Mermaid `@11.12.2`:
    // - `packages/mermaid/src/utils.ts:isLabelCoordinateInPath`
    // - `packages/mermaid/src/rendering-util/rendering-elements/edges.js`
    //
    // This is intentionally a very rough heuristic: it rounds the mid point and checks whether
    // either the rounded x or y shows up in the rounded SVG path `d` string.
    let rounded_x = point.x.round() as i64;
    let rounded_y = point.y.round() as i64;

    let sanitized_d = round_decimal_numbers_in_path(d_attr);

    sanitized_d.contains(&rounded_x.to_string()) || sanitized_d.contains(&rounded_y.to_string())
}

fn round_decimal_numbers_in_path(d_attr: &str) -> String {
    let mut out = String::new();
    let mut copied_until = 0usize;
    let mut cursor = 0usize;
    let mut changed = false;

    while cursor < d_attr.len() {
        if let Some(end) = decimal_number_match_end_at(d_attr, cursor) {
            if !changed {
                out = String::with_capacity(d_attr.len());
                changed = true;
            }
            out.push_str(&d_attr[copied_until..cursor]);
            let v = d_attr[cursor..end].parse::<f64>().unwrap_or(0.0);
            out.push_str(&(v.round() as i64).to_string());
            copied_until = end;
            cursor = end;
            continue;
        }

        let Some(ch) = d_attr[cursor..].chars().next() else {
            break;
        };
        cursor += ch.len_utf8();
    }

    if changed {
        out.push_str(&d_attr[copied_until..]);
        out
    } else {
        d_attr.to_string()
    }
}

fn decimal_number_match_end_at(s: &str, start: usize) -> Option<usize> {
    let digit_start = start;
    let mut cursor = consume_ascii_digits_in_path(s, start);
    if cursor == digit_start || !s.get(cursor..)?.starts_with('.') {
        return None;
    }

    let fraction_start = cursor + 1;
    cursor = consume_ascii_digits_in_path(s, fraction_start);
    if cursor == fraction_start {
        return None;
    }

    Some(cursor)
}

fn consume_ascii_digits_in_path(s: &str, mut cursor: usize) -> usize {
    while let Some(b) = s.as_bytes().get(cursor) {
        if !b.is_ascii_digit() {
            break;
        }
        cursor += 1;
    }
    cursor
}

fn calc_label_position(points: &[crate::model::LayoutPoint]) -> Option<(f64, f64)> {
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

pub(super) fn render_er_diagram_svg(
    layout: &ErDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: crate::er::ErModel = crate::json::from_value_ref(semantic)?;
    render_er_diagram_svg_model(
        layout,
        &model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

fn er_font_family_css(effective_config: &serde_json::Value) -> String {
    crate::config::config_font_family_css(effective_config)
}

pub(super) fn render_er_diagram_svg_model(
    layout: &ErDiagramLayout,
    model: &merman_core::diagrams::er::ErDiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    // Mermaid's internal diagram type for ER is `er` (not `erDiagram`), and marker ids are derived
    // from this type (e.g. `<diagramId>_er-zeroOrMoreEnd`).
    let diagram_type = "er";
    let is_elk_layout = effective_config
        .get("layout")
        .and_then(|v| v.as_str())
        .is_some_and(|s| s.eq_ignore_ascii_case("elk"));
    let data_look = config_diagram_look(effective_config);
    let data_look = data_look.as_str();

    // Mermaid's computed theme variables are not currently present in `effective_config`.
    // Use Mermaid default theme fallbacks so Stage-B SVGs match upstream defaults more closely.
    let _stroke = theme_color(effective_config, "lineColor", "#333333");
    let node_border = theme_color(effective_config, "nodeBorder", "#9370DB");
    let main_bkg = theme_color(effective_config, "mainBkg", "#ECECFF");
    let _tertiary = theme_color(
        effective_config,
        "tertiaryColor",
        "hsl(80, 100%, 96.2745098039%)",
    );
    let text_color = theme_color(effective_config, "textColor", "#333333");
    let _node_text_color = theme_color(effective_config, "nodeTextColor", &text_color);
    let font_family = er_font_family_css(effective_config);
    // Mermaid ER unified output inherits the root SVG font-size, so `themeVariables.fontSize`
    // wins when present (including Mermaid's common `"NNpx"` form).
    let font_size = crate::config::config_theme_or_root_font_size_px_opt(effective_config)
        .or_else(|| config_f64_css_px(effective_config, &["er", "fontSize"]))
        .unwrap_or(16.0)
        .max(1.0);
    let title_top_margin = effective_config
        .get("er")
        .and_then(|v| v.get("titleTopMargin"))
        .and_then(|v| v.as_f64())
        .or_else(|| {
            effective_config
                .get("titleTopMargin")
                .and_then(|v| v.as_f64())
        })
        .unwrap_or(25.0)
        .max(0.0);
    let use_max_width = effective_config
        .get("er")
        .and_then(|v| v.get("useMaxWidth"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let label_style = crate::text::TextStyle {
        font_family: Some(font_family.clone()),
        font_size,
        font_weight: None,
    };
    let attr_style = crate::text::TextStyle {
        font_family: Some(font_family.clone()),
        font_size: font_size.max(1.0),
        font_weight: None,
    };
    fn parse_trailing_index(id: &str) -> Option<i64> {
        let (_, tail) = id.rsplit_once('-')?;
        tail.parse::<i64>().ok()
    }
    fn er_node_sort_key(id: &str) -> (i64, i64) {
        if id.contains("---") {
            return (1, parse_trailing_index(id).unwrap_or(i64::MAX));
        }
        (0, parse_trailing_index(id).unwrap_or(i64::MAX))
    }

    let mut nodes = layout.nodes.clone();
    nodes.sort_by_key(|n| er_node_sort_key(&n.id));

    let mut edges = layout.edges.clone();
    fn er_edge_sort_key(id: &str) -> (i64, i64) {
        let Some(rest) = id.strip_prefix("er-rel-") else {
            return (i64::MAX, i64::MAX);
        };
        let mut digits_len = 0usize;
        for ch in rest.chars() {
            if !ch.is_ascii_digit() {
                break;
            }
            digits_len += ch.len_utf8();
        }
        if digits_len == 0 {
            return (i64::MAX, i64::MAX);
        }
        let Ok(idx) = rest[..digits_len].parse::<i64>() else {
            return (i64::MAX, i64::MAX);
        };
        let suffix = &rest[digits_len..];
        let variant = match suffix {
            "-cyclic-0" => 0,
            "" => 1,
            "-cyclic-2" => 2,
            _ => 99,
        };
        (idx, variant)
    }
    edges.sort_by_key(|e| er_edge_sort_key(&e.id));

    let include_md_parent = edges.iter().any(|e| {
        matches!(
            e.start_marker.as_deref(),
            Some("MD_PARENT_START") | Some("MD_PARENT_END")
        ) || matches!(
            e.end_marker.as_deref(),
            Some("MD_PARENT_START") | Some("MD_PARENT_END")
        )
    });

    let diagram_title = diagram_title.map(str::trim).filter(|t| !t.is_empty());
    let is_empty_diagram = nodes.is_empty() && edges.is_empty() && diagram_title.is_none();

    let bounds = compute_layout_bounds(&[], &nodes, &edges).unwrap_or({
        if is_empty_diagram {
            Bounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 0.0,
                max_y: 0.0,
            }
        } else {
            Bounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 100.0,
                max_y: 100.0,
            }
        }
    });

    let mut content_bounds = bounds.clone();
    if let Some(title) = diagram_title {
        let title_style = crate::text::TextStyle {
            font_family: Some(font_family.clone()),
            font_size,
            font_weight: None,
        };
        let measure = measurer.measure(title, &title_style);
        // ER titles inherit the root font-size in upstream CSS. Chromium's SVG bbox for this
        // inherited title sits on a 1/32px width lattice and includes 4px of vertical overhang
        // beyond the shared single-line text height.
        let title_width = ((measure.width.max(1.0) * 32.0).floor()) / 32.0;
        let title_height = measure.height + 4.0;
        let w = (content_bounds.max_x - content_bounds.min_x).max(1.0);
        let title_x = content_bounds.min_x + w / 2.0;
        let title_y = -title_top_margin;
        let title_min_x = title_x - title_width / 2.0;
        let title_max_x = title_x + title_width / 2.0;
        // Approximate the SVG text bbox using the measured height above the baseline.
        let title_min_y = title_y - title_height;
        let title_max_y = title_y;
        content_bounds.min_x = content_bounds.min_x.min(title_min_x);
        content_bounds.max_x = content_bounds.max_x.max(title_max_x);
        content_bounds.min_y = content_bounds.min_y.min(title_min_y);
        content_bounds.max_y = content_bounds.max_y.max(title_max_y);
    }

    let pad = options.viewbox_padding.max(0.0);
    let mut out = String::new();
    let (
        translate_x,
        translate_y,
        mut viewbox_attr,
        mut w_attr,
        mut h_attr,
        mut max_w_style,
        root_width_for_title,
    ) = if is_empty_diagram {
        let empty_span = (pad * 2.0).max(1.0);
        let empty_span_attr = fmt_string(empty_span);
        (
            0.0,
            0.0,
            format!(
                "{} {} {} {}",
                fmt_string(-pad),
                fmt_string(-pad),
                empty_span_attr,
                empty_span_attr
            ),
            empty_span_attr.clone(),
            empty_span_attr.clone(),
            fmt_max_width_px(empty_span),
            empty_span,
        )
    } else {
        let content_w = (content_bounds.max_x - content_bounds.min_x).max(1.0);
        let content_h = (content_bounds.max_y - content_bounds.min_y).max(1.0);
        let vb_w = content_w + pad * 2.0;
        let vb_h = content_h + pad * 2.0;
        let translate_x = pad - content_bounds.min_x;
        let translate_y = pad - content_bounds.min_y;

        // Upstream Mermaid viewports are driven by browser `getBBox()` values which frequently land on
        // a single-precision lattice. Snap the root viewport width/height to that lattice to keep
        // `parity-root` comparisons stable at high decimal precision.
        let vb_w_attr = ((vb_w.max(1.0)) as f32) as f64;
        let vb_h_attr = ((vb_h.max(1.0)) as f32) as f64;
        let w_attr = fmt_string(vb_w_attr);
        let h_attr = fmt_string(vb_h_attr);
        (
            translate_x,
            translate_y,
            format!("0 0 {} {}", w_attr, h_attr),
            w_attr,
            h_attr,
            fmt_max_width_px(vb_w_attr),
            vb_w_attr,
        )
    };
    apply_root_viewport_override(
        diagram_id,
        &mut viewbox_attr,
        &mut w_attr,
        &mut h_attr,
        &mut max_w_style,
        crate::generated::er_root_overrides_11_12_2::lookup_er_root_viewport_override,
    );

    let has_acc_title = model.acc_title.as_ref().is_some_and(|s| !s.is_empty());
    let has_acc_descr = model.acc_descr.as_ref().is_some_and(|s| !s.is_empty());
    let aria_labelledby = has_acc_title.then(|| format!("chart-title-{}", escape_xml(diagram_id)));
    let aria_describedby = has_acc_descr.then(|| format!("chart-desc-{}", escape_xml(diagram_id)));
    if use_max_width {
        root_svg::push_svg_root_open(
            &mut out,
            root_svg::SvgRootAttrs {
                class: Some("erDiagram"),
                width: root_svg::SvgRootWidth::Percent100,
                style_attr: Some(&format!(
                    "max-width: {max_w_style}px; background-color: white;"
                )),
                viewbox_attr: Some(&viewbox_attr),
                aria_labelledby: aria_labelledby.as_deref(),
                aria_describedby: aria_describedby.as_deref(),
                ..root_svg::SvgRootAttrs::new(diagram_id, diagram_type)
            },
        );
    } else {
        root_svg::push_svg_root_open(
            &mut out,
            root_svg::SvgRootAttrs {
                class: Some("erDiagram"),
                width: root_svg::SvgRootWidth::Fixed(&w_attr),
                height_attr: Some(&h_attr),
                style_attr: Some("background-color: white;"),
                viewbox_attr: Some(&viewbox_attr),
                aria_labelledby: aria_labelledby.as_deref(),
                aria_describedby: aria_describedby.as_deref(),
                ..root_svg::SvgRootAttrs::new(diagram_id, diagram_type)
            },
        );
    }

    if has_acc_title {
        let _ = write!(
            &mut out,
            r#"<title id="chart-title-{}">{}"#,
            escape_xml(diagram_id),
            escape_xml(model.acc_title.as_deref().unwrap_or_default())
        );
        out.push_str("</title>");
    }
    if has_acc_descr {
        let _ = write!(
            &mut out,
            r#"<desc id="chart-desc-{}">{}"#,
            escape_xml(diagram_id),
            escape_xml(model.acc_descr.as_deref().unwrap_or_default())
        );
        out.push_str("</desc>");
    }

    let _ = write!(
        &mut out,
        r#"<style>{}</style>"#,
        er_css(diagram_id, effective_config)
    );

    // Mermaid wraps diagram content (defs + root) in a single `<g>` element.
    out.push_str("<g>");

    // Markers ported from Mermaid `@11.12.2` `erMarkers.js`.
    // Note: ids follow Mermaid marker rules: `${diagramId}_${diagramType}-${markerType}{Start|End}`.
    // Mermaid's ER unified renderer enables four marker types by default; include MD_PARENT only if used.
    let diagram_id_esc = escape_xml(diagram_id);
    let diagram_type_esc = escape_xml(diagram_type);

    // Mermaid emits one `<defs>` wrapper per marker.
    if include_md_parent {
        let _ = writeln!(
            &mut out,
            r#"<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-mdParentStart" class="marker mdParent er" refX="0" refY="7" markerWidth="190" markerHeight="240" orient="auto"><path d="M 18,7 L9,13 L1,7 L9,1 Z"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-mdParentEnd" class="marker mdParent er" refX="19" refY="7" markerWidth="20" markerHeight="28" orient="auto"><path d="M 18,7 L9,13 L1,7 L9,1 Z"/></marker></defs>"#
        );
    }

    let _ = writeln!(
        &mut out,
        r#"<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-onlyOneStart" class="marker onlyOne er" refX="0" refY="9" markerWidth="18" markerHeight="18" orient="auto"><path d="M9,0 L9,18 M15,0 L15,18"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-onlyOneEnd" class="marker onlyOne er" refX="18" refY="9" markerWidth="18" markerHeight="18" orient="auto"><path d="M3,0 L3,18 M9,0 L9,18"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-zeroOrOneStart" class="marker zeroOrOne er" refX="0" refY="9" markerWidth="30" markerHeight="18" orient="auto"><circle fill="white" cx="21" cy="9" r="6"/><path d="M9,0 L9,18"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-zeroOrOneEnd" class="marker zeroOrOne er" refX="30" refY="9" markerWidth="30" markerHeight="18" orient="auto"><circle fill="white" cx="9" cy="9" r="6"/><path d="M21,0 L21,18"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-oneOrMoreStart" class="marker oneOrMore er" refX="18" refY="18" markerWidth="45" markerHeight="36" orient="auto"><path d="M0,18 Q 18,0 36,18 Q 18,36 0,18 M42,9 L42,27"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-oneOrMoreEnd" class="marker oneOrMore er" refX="27" refY="18" markerWidth="45" markerHeight="36" orient="auto"><path d="M3,9 L3,27 M9,18 Q27,0 45,18 Q27,36 9,18"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-zeroOrMoreStart" class="marker zeroOrMore er" refX="18" refY="18" markerWidth="57" markerHeight="36" orient="auto"><circle fill="white" cx="48" cy="18" r="6"/><path d="M0,18 Q18,0 36,18 Q18,36 0,18"/></marker></defs>
<defs><marker id="{diagram_id_esc}_{diagram_type_esc}-zeroOrMoreEnd" class="marker zeroOrMore er" refX="39" refY="18" markerWidth="57" markerHeight="36" orient="auto"><circle fill="white" cx="9" cy="18" r="6"/><path d="M21,18 Q39,0 57,18 Q39,36 21,18"/></marker></defs>"#
    );

    let mut entity_by_id: std::collections::HashMap<&str, &crate::er::ErEntity> =
        std::collections::HashMap::new();
    for e in model.entities.values() {
        entity_by_id.insert(e.id.as_str(), e);
    }

    fn er_rel_idx_from_edge_id(edge_id: &str) -> Option<usize> {
        let rest = edge_id.strip_prefix("er-rel-")?;
        let mut digits_len = 0usize;
        for ch in rest.chars() {
            if !ch.is_ascii_digit() {
                break;
            }
            digits_len += ch.len_utf8();
        }
        if digits_len == 0 {
            return None;
        }
        rest[..digits_len].parse::<usize>().ok()
    }

    fn er_edge_dom_id(edge_id: &str, relationships: &[crate::er::ErRelationship]) -> String {
        let Some(idx) = er_rel_idx_from_edge_id(edge_id) else {
            return edge_id.to_string();
        };
        let Some(rel) = relationships.get(idx) else {
            return edge_id.to_string();
        };
        let rest = edge_id.strip_prefix("er-rel-").unwrap_or("");
        let idx_prefix = idx.to_string();
        let suffix = rest.strip_prefix(&idx_prefix).unwrap_or("");
        let base = if rel.entity_a == rel.entity_b {
            match suffix {
                "-cyclic-0" => format!("{}-cyclic-special-1", rel.entity_a),
                "" => format!("{}-cyclic-special-mid", rel.entity_a),
                "-cyclic-2" => format!("{}-cyclic-special-2", rel.entity_a),
                _ => format!("{}-cyclic-special-mid", rel.entity_a),
            }
        } else {
            format!("id_{}_{}_{}", rel.entity_a, rel.entity_b, idx)
        };
        base
    }

    if is_elk_layout {
        // Mermaid's ER diagram output changes shape when `layout=elk` is enabled: markers are
        // emitted in their own wrapper `<g>`, and the rest of the content is written as sibling
        // top-level groups (`edges/edgePaths`, `subgraphs`, `nodes`, `edgeLabels`).
        out.push_str("</g>\n");
        out.push_str(r#"<g class="subgraphs"/>"#);
    } else {
        let _ = writeln!(&mut out, r#"<g class="root">"#);
        out.push_str(r#"<g class="clusters"/>"#);
    }

    if is_elk_layout {
        out.push_str(r#"<g class="edges edgePaths">"#);
    } else {
        out.push_str(r#"<g class="edgePaths">"#);
    }
    if options.include_edges {
        for e in &edges {
            if e.points.len() < 2 {
                continue;
            }
            let edge_dom_id = er_edge_dom_id(&e.id, &model.relationships);
            let edge_svg_id = format!("{diagram_id}-{edge_dom_id}");
            let is_dashed = e.stroke_dasharray.as_deref() == Some("8,8");
            let pattern_class = if is_dashed {
                "edge-pattern-dashed"
            } else {
                "edge-pattern-solid"
            };
            let line_classes = format!("edge-thickness-normal {pattern_class} relationshipLine");
            let shifted: Vec<crate::model::LayoutPoint> = e
                .points
                .iter()
                .map(|p| crate::model::LayoutPoint {
                    x: p.x + translate_x,
                    y: p.y + translate_y,
                })
                .collect();
            let data_points = base64::engine::general_purpose::STANDARD
                .encode(serde_json::to_vec(&shifted).unwrap_or_default());
            let mut curve_points = shifted.clone();
            if curve_points.len() == 2 {
                let a = &curve_points[0];
                let b = &curve_points[1];
                curve_points.insert(
                    1,
                    crate::model::LayoutPoint {
                        x: (a.x + b.x) / 2.0,
                        y: (a.y + b.y) / 2.0,
                    },
                );
            }
            let d = curve_basis_path_d(&curve_points);

            let _ = write!(
                &mut out,
                r#"<path d="{}" id="{}" class="{}" data-edge="true" data-et="edge" data-id="{}" data-points="{}" data-look="{}""#,
                escape_xml(&d),
                escape_xml(&edge_svg_id),
                escape_xml(&line_classes),
                escape_xml(&edge_dom_id),
                escape_xml(&data_points),
                escape_xml(&data_look)
            );
            if let Some(m) = &e.start_marker {
                let marker = er_unified_marker_id(diagram_id, diagram_type, m);
                let _ = write!(&mut out, r#" marker-start="url(#{})""#, escape_xml(&marker));
            }
            if let Some(m) = &e.end_marker {
                let marker = er_unified_marker_id(diagram_id, diagram_type, m);
                let _ = write!(&mut out, r#" marker-end="url(#{})""#, escape_xml(&marker));
            }
            out.push_str(" />");
        }
    }
    out.push_str("</g>");

    out.push_str(r#"<g class="edgeLabels">"#);
    if options.include_edges {
        for e in &edges {
            let rel_idx = er_rel_idx_from_edge_id(&e.id)
                .and_then(|idx| model.relationships.get(idx).map(|r| (idx, r)));

            let rel_text_raw = rel_idx.map(|(_, r)| r.role_a.as_str()).unwrap_or("");
            let rel_text = rel_text_raw.trim();
            let edge_dom_id = er_edge_dom_id(&e.id, &model.relationships);

            let has_label_text = !rel_text.is_empty();
            let has_whitespace_only_label = !rel_text_raw.is_empty() && rel_text.is_empty();
            let (w, h, mut cx, mut cy) = if has_label_text {
                if let Some(lbl) = &e.label {
                    (
                        lbl.width.max(0.0),
                        lbl.height.max(0.0),
                        lbl.x + translate_x,
                        lbl.y + translate_y,
                    )
                } else {
                    (0.0, 0.0, 0.0, 0.0)
                }
            } else {
                (0.0, 0.0, 0.0, 0.0)
            };

            if has_label_text && w > 0.0 && h > 0.0 {
                // Mermaid positions edge labels using Dagre's `edge.x/edge.y` by default, but it
                // recomputes the label position along the polyline when the edge path `d` doesn't
                // contain the midpoint coordinates (see `edges.js:isLabelCoordinateInPath`).
                //
                // Replicate that behavior here to match upstream DOM parity for certain curved
                // edges (notably parallel relationship edges in ER diagrams).
                let shifted: Vec<crate::model::LayoutPoint> = e
                    .points
                    .iter()
                    .map(|p| crate::model::LayoutPoint {
                        x: p.x + translate_x,
                        y: p.y + translate_y,
                    })
                    .collect();
                if !shifted.is_empty() {
                    let mid_idx = shifted.len() / 2;
                    let mid = shifted[mid_idx].clone();
                    let mut curve_points = shifted.clone();
                    if curve_points.len() == 2 {
                        let a = &curve_points[0];
                        let b = &curve_points[1];
                        curve_points.insert(
                            1,
                            crate::model::LayoutPoint {
                                x: (a.x + b.x) / 2.0,
                                y: (a.y + b.y) / 2.0,
                            },
                        );
                    }
                    let d = curve_basis_path_d(&curve_points);
                    if !is_label_coordinate_in_path(mid, &d) {
                        if let Some((x, y)) = calc_label_position(&shifted) {
                            cx = x;
                            cy = y;
                        }
                    }
                }
            }

            if has_label_text && w > 0.0 && h > 0.0 {
                // Mermaid ER relationship labels follow Mermaid's effective HTML-label
                // resolution: root `htmlLabels`, then `flowchart.htmlLabels`, then default
                // `true`. When both are unset, upstream still emits HTML `<foreignObject>`
                // labels through `createText(...)`.
                let edge_html_labels = crate::er::er_relationship_html_labels(effective_config);

                let _ = write!(
                    &mut out,
                    r#"<g class="edgeLabel" transform="translate({}, {})">"#,
                    fmt(cx),
                    fmt(cy)
                );
                let _ = write!(
                    &mut out,
                    r#"<g class="label" data-id="{}" transform="translate({}, {})">"#,
                    escape_xml_display(&edge_dom_id),
                    fmt(-w / 2.0),
                    fmt(-h / 2.0)
                );
                if edge_html_labels {
                    let _ = write!(
                        &mut out,
                        r#"<foreignObject width="{}" height="{}">"#,
                        fmt(w),
                        fmt(h)
                    );
                    out.push_str(r#"<div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;"><span class="edgeLabel">"#);
                    // Mermaid ER relationship labels use the generic HTML edge-label path, so they
                    // inherit `markdownToHTML()` semantics (Markdown emphasis + inline `<br/>`
                    // handling) rather than rendering literal `**...**` marker text.
                    let html =
                        crate::text::mermaid_markdown_to_xhtml_label_fragment(rel_text, true);
                    out.push_str(&html);
                    out.push_str(r#"</span></div></foreignObject></g></g>"#);
                } else {
                    out.push_str("<g>");
                    let _ = write!(
                        &mut out,
                        r#"<rect class="background" style="" x="{}" y="-1" width="{}" height="{}"/>"#,
                        fmt(-w / 2.0),
                        fmt(w),
                        fmt(h)
                    );
                    crate::svg::parity::flowchart::write_flowchart_svg_text_centered(
                        &mut out, rel_text, true,
                    );
                    out.push_str("</g></g></g>");
                }
            } else {
                let edge_html_labels = crate::er::er_relationship_html_labels(effective_config);
                if edge_html_labels {
                    // Mermaid emits a `translate(undefined,NaN)` transform for relationship labels
                    // that are whitespace-only (but not for fully empty strings). Preserve that
                    // oddity for DOM parity in `structure` mode (see upstream Cypress fixture
                    // `*_blank_or_empty_labels_007`).
                    if has_whitespace_only_label {
                        out.push_str(r#"<g class="edgeLabel" transform="translate(undefined,NaN)"><g class="label""#);
                    } else {
                        out.push_str(r#"<g class="edgeLabel"><g class="label""#);
                    }
                    let _ = write!(
                        &mut out,
                        r#" data-id="{}""#,
                        escape_xml_display(&edge_dom_id)
                    );
                    out.push_str(r#" transform="translate(0, 0)"><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;"><span class="edgeLabel"></span></div></foreignObject></g></g>"#);
                } else {
                    if has_whitespace_only_label {
                        out.push_str(r#"<g class="edgeLabel" transform="translate(undefined,NaN)"><g class="label""#);
                    } else {
                        out.push_str(r#"<g class="edgeLabel"><g class="label""#);
                    }
                    let _ = write!(
                        &mut out,
                        r#" data-id="{}""#,
                        escape_xml_display(&edge_dom_id)
                    );
                    out.push_str(r#" transform="translate(0, 0)"><g><rect class="background" style="" x="0" y="-1" width="0" height="0"/>"#);
                    crate::svg::parity::flowchart::write_flowchart_svg_text_centered(
                        &mut out, "", true,
                    );
                    out.push_str("</g></g></g>");
                }
            }
        }
    }
    out.push_str("</g>\n");

    // Entities drawn after relationships so they cover markers when overlapping.
    out.push_str(r#"<g class="nodes">"#);
    for n in &nodes {
        let Some(entity) = entity_by_id.get(n.id.as_str()).copied() else {
            if n.id.contains("---") {
                let cx = n.x + translate_x;
                let cy = n.y + translate_y;
                let _ = write!(
                    &mut out,
                    r#"<g class="label edgeLabel" id="{}" transform="translate({}, {})">"#,
                    escape_xml(&n.id),
                    fmt(cx),
                    fmt(cy)
                );
                out.push_str(r#"<rect width="0.1" height="0.1"/>"#);
                out.push_str(r#"<g class="label" style="" transform="translate(0, 0)"><rect/><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 10px; text-align: center;"><span class="nodeLabel"></span></div></foreignObject></g></g>"#);
            }
            continue;
        };

        let (rect_style_decls, text_style_decls) = compile_er_entity_styles(entity, &model.classes);
        let rect_style_attr = if rect_style_decls.is_empty() {
            r#"style="""#.to_string()
        } else {
            format!(
                r#"style="{}""#,
                escape_xml(&style_decls_with_important(&rect_style_decls))
            )
        };
        let label_style_attr = if text_style_decls.is_empty() {
            r#"style="""#.to_string()
        } else {
            format!(
                r#"style="{}""#,
                escape_xml(&style_decls_with_important(&text_style_decls))
            )
        };

        let measure = crate::er::measure_entity_box(
            entity,
            measurer,
            &label_style,
            &attr_style,
            effective_config,
        );
        let w = n.width.max(1.0);
        let h = n.height.max(1.0);
        if (measure.width - w).abs() > 1e-3 || (measure.height - h).abs() > 1e-3 {
            return Err(Error::InvalidModel {
                message: format!(
                    "ER entity measured size mismatch for {}: layout=({},{}), measure=({}, {})",
                    n.id, w, h, measure.width, measure.height
                ),
            });
        }

        let cx = n.x + translate_x;
        let cy = n.y + translate_y;
        let ox = -w / 2.0;
        let oy = -h / 2.0;

        let group_class = if entity.css_classes.trim().is_empty() {
            "node".to_string()
        } else {
            format!("node {}", entity.css_classes.trim())
        };
        let _ = write!(
            &mut out,
            r#"<g id="{}-{}" class="{}" data-look="{}" transform="translate({}, {})">"#,
            escape_xml(diagram_id),
            escape_xml(&entity.id),
            escape_xml(&group_class),
            escape_xml(&data_look),
            fmt(cx),
            fmt(cy)
        );

        if entity.attributes.is_empty() {
            let _ = write!(
                &mut out,
                r#"<rect class="basic label-container" {} x="{}" y="{}" width="{}" height="{}"/>"#,
                rect_style_attr,
                fmt(ox),
                fmt(oy),
                fmt(w),
                fmt(h)
            );
            let html_labels = effective_config
                .get("htmlLabels")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let wrap_mode = if html_labels {
                crate::text::WrapMode::HtmlLike
            } else {
                crate::text::WrapMode::SvgLike
            };
            let label_metrics =
                measurer.measure_wrapped(&measure.label_text, &label_style, None, wrap_mode);
            let lw = if wrap_mode == crate::text::WrapMode::HtmlLike {
                measure.label_html_width.max(0.0)
            } else {
                label_metrics.width.max(0.0)
            };
            let lh = label_metrics.height.max(0.0);

            let _ = write!(
                &mut out,
                r#"<g class="label" transform="translate({}, {})" {}><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: center;">{}</div></foreignObject></g>"#,
                fmt(-lw / 2.0),
                fmt(-lh / 2.0),
                label_style_attr,
                fmt(lw),
                fmt(lh),
                measure.label_max_width_px.max(0),
                html_label_content(&measure.label_text, "", true)
            );
            out.push_str("</g>");
            continue;
        }

        fn fallback_rough_line_path_d(x0: f64, y0: f64, x1: f64, y1: f64) -> String {
            let c1x = x0 + (x1 - x0) * 0.25;
            let c1y = y0 + (y1 - y0) * 0.25;
            let c2x = x0 + (x1 - x0) * 0.75;
            let c2y = y0 + (y1 - y0) * 0.75;
            let d1 = format!(
                "M{} {} C{} {}, {} {}, {} {}",
                fmt_path(x0),
                fmt_path(y0),
                fmt_path(c1x),
                fmt_path(c1y),
                fmt_path(c2x),
                fmt_path(c2y),
                fmt_path(x1),
                fmt_path(y1)
            );
            let c1x2 = x0 + (x1 - x0) * 0.35;
            let c1y2 = y0 + (y1 - y0) * 0.15;
            let c2x2 = x0 + (x1 - x0) * 0.65;
            let c2y2 = y0 + (y1 - y0) * 0.85;
            let d2 = format!(
                "M{} {} C{} {}, {} {}, {} {}",
                fmt_path(x0),
                fmt_path(y0),
                fmt_path(c1x2),
                fmt_path(c1y2),
                fmt_path(c2x2),
                fmt_path(c2y2),
                fmt_path(x1),
                fmt_path(y1)
            );
            format!("{d1} {d2}")
        }

        fn fallback_rough_rect_border_path_d(x0: f64, y0: f64, x1: f64, y1: f64) -> String {
            let top = fallback_rough_line_path_d(x0, y0, x1, y0);
            let right = fallback_rough_line_path_d(x1, y0, x1, y1);
            let bottom = fallback_rough_line_path_d(x1, y1, x0, y1);
            let left = fallback_rough_line_path_d(x0, y1, x0, y0);
            format!("{top} {right} {bottom} {left}")
        }

        fn html_label_content(
            text: &str,
            span_style_attr: &str,
            markdown_node_label: bool,
        ) -> String {
            let span_class = if markdown_node_label {
                "nodeLabel markdown-node-label"
            } else {
                "nodeLabel"
            };
            let decoded = decode_mermaid_entities_for_render_text(text);
            let text = decoded.as_ref().trim();
            if text.is_empty() {
                return format!(r#"<span class="{}"{}></span>"#, span_class, span_style_attr);
            }

            let lower = text.to_ascii_lowercase();
            let has_inline_html =
                lower.contains("<br") || lower.contains("<strong") || lower.contains("<em");
            let has_inline_code = text.contains('`');
            let has_markdown = crate::er::er_label_has_structural_markdown(text);

            // Mermaid's DOM serialization for generics (`type<T>`) avoids nested HTML tags.
            // When the generic source also used markdown delimiters, the delimiters affect
            // parsing but are not emitted as literal text.
            if (text.contains('<') || text.contains('>')) && !has_inline_html {
                if let Some(plain) = crate::er::er_generic_markdown_plain_text(text) {
                    return escape_xml(&plain);
                }
                return escape_xml(text);
            }

            if has_inline_code {
                let html_out = crate::text::mermaid_markdown_to_xhtml_label_fragment(text, true);
                return format!(
                    r#"<span class="{}"{}>{}</span>"#,
                    span_class, span_style_attr, html_out
                );
            }

            if has_markdown || has_inline_html {
                let mut html_out = String::new();
                let parser = pulldown_cmark::Parser::new_ext(
                    text,
                    pulldown_cmark::Options::ENABLE_TABLES
                        | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
                        | pulldown_cmark::Options::ENABLE_TASKLISTS,
                )
                .map(|ev| match ev {
                    pulldown_cmark::Event::SoftBreak => pulldown_cmark::Event::HardBreak,
                    other => other,
                });
                pulldown_cmark::html::push_html(&mut html_out, parser);
                let html_out = html_out.trim().to_string();
                let html_out = html_out
                    .replace("<br>", "<br />")
                    .replace("<br/>", "<br />")
                    .replace("<br >", "<br />");

                return format!(
                    r#"<span class="{}"{}>{}</span>"#,
                    span_class, span_style_attr, html_out
                );
            }

            format!(
                r#"<span class="{}"{}><p>{}</p></span>"#,
                span_class,
                span_style_attr,
                escape_xml(text)
            )
        }

        fn parse_hex_color_rgb(s: &str) -> Option<(u8, u8, u8)> {
            let s = s.trim();
            let hex = s.strip_prefix('#')?;
            if hex.len() == 3 {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                return Some((r, g, b));
            }
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                return Some((r, g, b));
            }
            None
        }

        let label_div_color_prefix = last_style_value(&text_style_decls, "color")
            .and_then(|v| parse_hex_color_rgb(&v))
            .map(|(r, g, b)| format!("color: rgb({r}, {g}, {b}) !important; "))
            .unwrap_or_default();
        let span_style_attr = if text_style_decls.is_empty() {
            String::new()
        } else {
            format!(
                r#" style="{}""#,
                escape_xml(&style_decls_with_important(&text_style_decls))
            )
        };

        // Mermaid ER attribute tables (erBox.ts) use HTML labels (`foreignObject`) and paths for the table rows.
        let name_row_h = (measure.label_height + measure.text_padding).max(1.0);
        let box_x0 = ox;
        let box_y0 = oy;
        let box_x1 = ox + w;
        let box_y1 = oy + h;
        let sep_y = oy + name_row_h;

        let box_fill =
            last_style_value(&rect_style_decls, "fill").unwrap_or_else(|| main_bkg.clone());
        let box_stroke =
            last_style_value(&rect_style_decls, "stroke").unwrap_or_else(|| node_border.clone());
        let box_stroke_width = last_style_value(&rect_style_decls, "stroke-width")
            .and_then(|v| parse_px_f64(&v))
            .unwrap_or(1.3)
            .max(0.0);

        let stroke_width_attr = fmt(box_stroke_width);

        let group_style = concat_style_keys(&rect_style_decls, &["fill", "stroke", "stroke-width"]);
        let group_style_attr = if group_style.is_empty() {
            r#"style="""#.to_string()
        } else {
            format!(r#"style="{}""#, escape_xml(&group_style))
        };

        let mut override_decls: Vec<String> = Vec::new();
        if let Some(v) = last_style_value(&rect_style_decls, "stroke") {
            override_decls.push(format!("stroke:{v}"));
        }
        if let Some(v) = last_style_value(&rect_style_decls, "stroke-width") {
            override_decls.push(format!("stroke-width:{v}"));
        }
        let override_style = if override_decls.is_empty() {
            None
        } else {
            Some(style_decls_with_important(&override_decls))
        };
        let override_style_attr = override_style
            .as_deref()
            .map(|s| format!(r#" style="{}""#, escape_xml(s)))
            .unwrap_or_default();

        let hand_drawn_seed = effective_config
            .get("handDrawnSeed")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        // Mermaid erBox.ts uses Rough.js with `roughness=0` for default (non-handDrawn) nodes.
        //
        // Even with roughness=0, Rough.js still depends on seeded randomness via `divergePoint`.
        // For strict SVG parity we use the same Rough.js algorithm (v4.6.6) here instead of a
        // generic sketchy-stroke renderer.
        fn roughjs46_next_f64(seed: &mut u32) -> f64 {
            if *seed == 0 {
                // Mermaid (Rough.js) falls back to `Math.random()` when seed=0. We keep our SVG
                // stable in that case by returning 0, which yields `divergePoint=0.2`.
                return 0.0;
            }
            // Rough.js v4.6.6 (bin/math.js):
            //   this.seed = Math.imul(48271, this.seed)
            //   return ((2**31 - 1) & this.seed) / 2**31
            let prod = seed.wrapping_mul(48_271);
            *seed = prod & 0x7fff_ffff;
            (*seed as f64) / 2_147_483_648.0
        }

        fn roughjs46_diverge_point(seed: &mut u32) -> f64 {
            0.2 + roughjs46_next_f64(seed) * 0.2
        }

        fn roughjs46_double_line_path_d(
            seed: &mut u32,
            x0: f64,
            y0: f64,
            x1: f64,
            y1: f64,
        ) -> String {
            let mut out = String::new();
            let dx = x1 - x0;
            let dy = y1 - y0;

            for _ in 0..2 {
                let d = roughjs46_diverge_point(seed);
                // Rough.js `_line()` continues to call into `_offsetOpt()` even when `roughness=0`
                // (the random terms get multiplied by zero, but the PRNG state still advances).
                //
                // In Rough.js v4.6.6 `_line()` uses:
                // - 2 random() calls for `midDispX/midDispY` offsetOpt
                // - 2 random() calls for moveTo (x1/y1)
                // - 6 random() calls for bcurveTo (cp1/cp2/x2/y2)
                // Total: 10 random() calls after divergePoint.
                for _ in 0..10 {
                    let _ = roughjs46_next_f64(seed);
                }
                let cx1 = x0 + dx * d;
                let cy1 = y0 + dy * d;
                let cx2 = x0 + dx * 2.0 * d;
                let cy2 = y0 + dy * 2.0 * d;
                let _ = write!(
                    &mut out,
                    "M{} {} C{} {}, {} {}, {} {} ",
                    x0, y0, cx1, cy1, cx2, cy2, x1, y1
                );
            }

            out.trim_end().to_string()
        }

        fn rough_rect_border_path_d(seed: u64, x0: f64, y0: f64, x1: f64, y1: f64) -> String {
            let w = (x1 - x0).max(0.0);
            let h = (y1 - y0).max(0.0);
            if seed == 0 {
                return fallback_rough_rect_border_path_d(x0, y0, x1, y1);
            }
            let mut s = seed as u32;

            // Rough.js v4.6.6 renderer.rectangle -> polygon -> linearPath:
            //   segments: (x,y)->(x+w,y)->(x+w,y+h)->(x,y+h)->(x,y)
            let mut out = String::new();
            let x2 = x0 + w;
            let y2 = y0 + h;

            let segs = [
                (x0, y0, x2, y0),
                (x2, y0, x2, y2),
                (x2, y2, x0, y2),
                (x0, y2, x0, y0),
            ];
            for (ax, ay, bx, by) in segs {
                let d = roughjs46_double_line_path_d(&mut s, ax, ay, bx, by);
                out.push_str(&d);
                out.push(' ');
            }

            out.trim_end().to_string()
        }

        fn roughjs46_rect_fill_path_d(x0: f64, y0: f64, x1: f64, y1: f64) -> String {
            format!(
                "M{} {} L{} {} L{} {} L{} {}",
                x0, y0, x1, y0, x1, y1, x0, y1
            )
        }

        fn thin_divider_rect_bounds(x0: f64, y0: f64, x1: f64, y1: f64) -> (f64, f64, f64, f64) {
            let half = 0.00005;
            if (y1 - y0).abs() <= (x1 - x0).abs() {
                (x0, y0 - half, x1, y0 + half)
            } else {
                (x0 - half, y0, x0 + half, y1)
            }
        }

        // Base box (fill + border)
        let _ = write!(&mut out, r#"<g {} class="outer-path">"#, group_style_attr);
        let _ = write!(
            &mut out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="{}"{} />"#,
            roughjs46_rect_fill_path_d(box_x0, box_y0, box_x1, box_y1),
            escape_xml(&box_fill),
            override_style_attr
        );
        let _ = write!(
            &mut out,
            r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="0 0"{} />"#,
            rough_rect_border_path_d(hand_drawn_seed, box_x0, box_y0, box_x1, box_y1),
            escape_xml(&box_stroke),
            stroke_width_attr,
            override_style_attr
        );
        out.push_str("</g>");

        // Row rectangles
        let odd_fill = theme_color(effective_config, "rowOdd", "hsl(240, 100%, 100%)");
        let even_fill = theme_color(
            effective_config,
            "rowEven",
            "hsl(240, 100%, 97.2745098039%)",
        );
        let mut y = sep_y;
        for (idx, row) in measure.rows.iter().enumerate() {
            let row_h = row.height.max(1.0);
            let y0 = y;
            let y1 = y + row_h;
            y = y1;
            let is_odd = idx % 2 == 0;
            let row_class = if is_odd {
                "row-rect-odd"
            } else {
                "row-rect-even"
            };
            let row_fill = if is_odd {
                odd_fill.as_str()
            } else {
                even_fill.as_str()
            };
            let _ = write!(
                &mut out,
                r#"<g {} class="{}">"#,
                group_style_attr, row_class
            );
            let row_override_style_attr =
                if !is_odd && last_style_value(&rect_style_decls, "fill").is_some() {
                    let mut decls: Vec<String> = Vec::new();
                    if let Some(v) = last_style_value(&rect_style_decls, "fill") {
                        decls.push(format!("fill:{v}"));
                    }
                    if let Some(v) = last_style_value(&rect_style_decls, "stroke") {
                        decls.push(format!("stroke:{v}"));
                    }
                    if let Some(v) = last_style_value(&rect_style_decls, "stroke-width") {
                        decls.push(format!("stroke-width:{v}"));
                    }
                    if decls.is_empty() {
                        override_style_attr.clone()
                    } else {
                        let s = style_decls_with_important_join(&decls, ";");
                        format!(r#" style="{}""#, escape_xml(&s))
                    }
                } else {
                    override_style_attr.clone()
                };
            let _ = write!(
                &mut out,
                r#"<path d="{}" stroke="none" stroke-width="0" fill="{}"{} />"#,
                roughjs46_rect_fill_path_d(box_x0, y0, box_x1, y1),
                escape_xml(row_fill),
                row_override_style_attr
            );
            let _ = write!(
                &mut out,
                r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="0 0"{} />"#,
                rough_rect_border_path_d(hand_drawn_seed, box_x0, y0, box_x1, y1),
                escape_xml(&node_border),
                stroke_width_attr,
                row_override_style_attr
            );
            out.push_str("</g>");
        }

        // HTML labels
        let line_h = (font_size * 1.5).max(1.0);
        let mut pad = config_f64(effective_config, &["er", "diagramPadding"]).unwrap_or(20.0);
        // Keep parity with Mermaid's erBox.ts `if (!config.htmlLabels) { PADDING *= 1.25; }`:
        // when `htmlLabels` is unset (undefined), upstream still applies the 1.25 multiplier.
        let html_labels_raw = effective_config
            .get("htmlLabels")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !html_labels_raw {
            pad *= 1.25;
        }

        fn er_calc_text_input_for_calculate_text_width(text: &str) -> String {
            // Mermaid erBox.ts measures `calculateTextWidth` on the pre-workaround string, which
            // can include literal `&lt;` / `&gt;` for generics.
            if text.contains('<') || text.contains('>') {
                text.replace('<', "&lt;").replace('>', "&gt;")
            } else {
                text.to_string()
            }
        }

        let name_w = measure.label_html_width.max(0.0);
        let name_x = -name_w / 2.0;
        let name_y = oy + name_row_h / 2.0 - line_h / 2.0;
        let name_mw_px = crate::er::calculate_text_width_like_mermaid_px(
            measurer,
            &label_style,
            &er_calc_text_input_for_calculate_text_width(&measure.label_text),
        ) + 100;
        let _ = write!(
            &mut out,
            r#"<g class="label name" transform="translate({}, {})" {}><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: start;">{}"#,
            fmt(name_x),
            fmt(name_y),
            label_style_attr,
            fmt(name_w),
            fmt(line_h),
            escape_xml(&label_div_color_prefix),
            name_mw_px.max(0),
            html_label_content(&measure.label_text, &span_style_attr, false)
        );
        out.push_str("</div></foreignObject></g>");

        let type_col_w = measure.type_col_w.max(0.0);
        let name_col_w = measure.name_col_w.max(0.0);
        let key_col_w = measure.key_col_w.max(0.0);
        let _comment_col_w = measure.comment_col_w.max(0.0);

        let left_text_x = ox + pad / 2.0;
        let type_left = left_text_x;
        let name_left = left_text_x + type_col_w;
        let key_left = left_text_x + type_col_w + name_col_w;
        let comment_left = left_text_x + type_col_w + name_col_w + key_col_w;

        let mut row_top = sep_y;
        for row in &measure.rows {
            let row_h = row.height.max(1.0);
            let cell_y = row_top + row_h / 2.0 - line_h / 2.0;

            let type_w = crate::er::er_html_label_metrics(&row.type_text, measurer, &attr_style)
                .width
                .max(0.0);
            let name_w = crate::er::er_html_label_metrics(&row.name_text, measurer, &attr_style)
                .width
                .max(0.0);
            let keys_w = crate::er::er_html_label_metrics(&row.key_text, measurer, &attr_style)
                .width
                .max(0.0);
            let comment_w =
                crate::er::er_html_label_metrics(&row.comment_text, measurer, &attr_style)
                    .width
                    .max(0.0);

            let type_mw_px = crate::er::calculate_text_width_like_mermaid_px(
                measurer,
                &attr_style,
                &er_calc_text_input_for_calculate_text_width(&row.type_text),
            ) + 100;
            let name_mw_px = crate::er::calculate_text_width_like_mermaid_px(
                measurer,
                &attr_style,
                &er_calc_text_input_for_calculate_text_width(&row.name_text),
            ) + 100;
            let keys_mw_px = crate::er::calculate_text_width_like_mermaid_px(
                measurer,
                &attr_style,
                &er_calc_text_input_for_calculate_text_width(&row.key_text),
            ) + 100;
            let comment_mw_px = crate::er::calculate_text_width_like_mermaid_px(
                measurer,
                &attr_style,
                &er_calc_text_input_for_calculate_text_width(&row.comment_text),
            ) + 100;

            let _ = write!(
                &mut out,
                r#"<g class="label attribute-type" transform="translate({}, {})" {}><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: start;">{}"#,
                fmt(type_left),
                fmt(cell_y),
                label_style_attr,
                fmt(type_w),
                fmt(line_h),
                escape_xml(&label_div_color_prefix),
                type_mw_px.max(0),
                html_label_content(&row.type_text, &span_style_attr, false)
            );
            out.push_str("</div></foreignObject></g>");

            let _ = write!(
                &mut out,
                r#"<g class="label attribute-name" transform="translate({}, {})" {}><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: start;">{}"#,
                fmt(name_left),
                fmt(cell_y),
                label_style_attr,
                fmt(name_w),
                fmt(line_h),
                escape_xml(&label_div_color_prefix),
                name_mw_px.max(0),
                html_label_content(&row.name_text, &span_style_attr, false)
            );
            out.push_str("</div></foreignObject></g>");

            let _ = write!(
                &mut out,
                r#"<g class="label attribute-keys" transform="translate({}, {})" {}><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: start;">{}"#,
                fmt(key_left),
                fmt(cell_y),
                label_style_attr,
                fmt(keys_w),
                fmt(if row.key_text.trim().is_empty() {
                    0.0
                } else {
                    line_h
                }),
                escape_xml(&label_div_color_prefix),
                keys_mw_px.max(0),
                html_label_content(&row.key_text, &span_style_attr, false)
            );
            out.push_str("</div></foreignObject></g>");

            let _ = write!(
                &mut out,
                r#"<g class="label attribute-comment" transform="translate({}, {})" {}><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: start;">{}"#,
                fmt(comment_left),
                fmt(cell_y),
                label_style_attr,
                fmt(comment_w),
                fmt(if row.comment_text.trim().is_empty() {
                    0.0
                } else {
                    line_h
                }),
                escape_xml(&label_div_color_prefix),
                comment_mw_px.max(0),
                html_label_content(&row.comment_text, &span_style_attr, false)
            );
            out.push_str("</div></foreignObject></g>");

            row_top += row_h;
        }

        // Dividers (header separator + column boundaries)
        let divider_style = override_style_attr.clone();
        let divider_path_attrs = format!(
            r#" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="0 0"{}"#,
            escape_xml(&box_stroke),
            stroke_width_attr,
            divider_style
        );
        // Mermaid `erBox.ts` draws the header separator twice:
        // - once as the explicit "Name line"
        // - once via the later `yOffsets` pass (which always contains `0`)
        fn write_divider_group(
            out: &mut String,
            hand_drawn_seed: u64,
            x0: f64,
            y0: f64,
            x1: f64,
            y1: f64,
            fill: &str,
            divider_path_attrs: &str,
        ) {
            let (rx0, ry0, rx1, ry1) = thin_divider_rect_bounds(x0, y0, x1, y1);
            let _ = write!(
                out,
                r#"<g class="divider"><path d="{}" stroke="none" stroke-width="0" fill="{}" fill-rule="evenodd"/><path d="{}"{} /></g>"#,
                roughjs46_rect_fill_path_d(rx0, ry0, rx1, ry1),
                escape_xml(fill),
                rough_rect_border_path_d(hand_drawn_seed, rx0, ry0, rx1, ry1),
                divider_path_attrs
            );
        }

        write_divider_group(
            &mut out,
            hand_drawn_seed,
            box_x0,
            sep_y,
            box_x1,
            sep_y,
            &box_fill,
            &divider_path_attrs,
        );

        let mut divider_xs: Vec<f64> = Vec::new();
        divider_xs.push(ox + type_col_w);
        if measure.has_key {
            divider_xs.push(ox + type_col_w + name_col_w);
        }
        if measure.has_comment {
            divider_xs.push(ox + type_col_w + name_col_w + key_col_w);
        }
        for x in divider_xs {
            write_divider_group(
                &mut out,
                hand_drawn_seed,
                x,
                sep_y,
                x,
                box_y1,
                &box_fill,
                &divider_path_attrs,
            );
        }

        write_divider_group(
            &mut out,
            hand_drawn_seed,
            box_x0,
            sep_y,
            box_x1,
            sep_y,
            &box_fill,
            &divider_path_attrs,
        );

        out.push_str("</g>");
    }
    out.push_str("</g>\n");

    if !is_elk_layout {
        out.push_str("</g>\n</g>\n");
    }

    push_er_gradient(&mut out, diagram_id, effective_config);

    if let Some(title) = diagram_title {
        // Mermaid `utils.insertTitle(...)` appends the title after rendering the graph content.
        // - `text-anchor="middle"`
        // - `x = bounds.x + bounds.width / 2`
        // - `y = -titleTopMargin` (default: 25)
        let title_top_margin = effective_config
            .get("er")
            .and_then(|v| v.get("titleTopMargin"))
            .and_then(|v| v.as_f64())
            .unwrap_or(25.0);

        let (vb_min_x, vb_w) = viewbox_attr
            .split_whitespace()
            .collect::<Vec<_>>()
            .get(0..3)
            .and_then(|p| Some((p[0].parse::<f64>().ok()?, p[2].parse::<f64>().ok()?)))
            .unwrap_or((0.0, root_width_for_title));

        let _ = write!(
            &mut out,
            r#"<text text-anchor="middle" x="{}" y="{}" class="erDiagramTitleText">{}"#,
            fmt(vb_min_x + vb_w / 2.0),
            fmt(-title_top_margin),
            escape_xml(title)
        );
        out.push_str("</text>\n");
    }

    push_er_shadow_defs(&mut out, diagram_id, effective_config);

    out.push_str("</svg>\n");
    Ok(out)
}

fn push_er_shadow_defs(
    out: &mut String,
    diagram_id: &str,
    effective_config_value: &serde_json::Value,
) {
    let flood_color = effective_config_value
        .get("theme")
        .and_then(|v| v.as_str())
        .filter(|theme| theme.contains("dark"))
        .map(|_| "#FFFFFF")
        .unwrap_or("#000000");
    let diagram_id = escape_xml(diagram_id);
    let _ = write!(
        out,
        r#"<defs><filter id="{}-drop-shadow" height="130%" width="130%"><feDropShadow dx="4" dy="4" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs><defs><filter id="{}-drop-shadow-small" height="150%" width="150%"><feDropShadow dx="2" dy="2" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs>"#,
        diagram_id.as_str(),
        flood_color,
        diagram_id.as_str(),
        flood_color
    );
}

fn push_er_gradient(
    out: &mut String,
    diagram_id: &str,
    effective_config_value: &serde_json::Value,
) {
    if !config_bool(effective_config_value, &["themeVariables", "useGradient"]).unwrap_or(false) {
        return;
    }

    let gradient_start =
        config_string(effective_config_value, &["themeVariables", "gradientStart"])
            .or_else(|| {
                config_string(
                    effective_config_value,
                    &["themeVariables", "primaryBorderColor"],
                )
            })
            .unwrap_or_else(|| "#9370DB".to_string());
    let gradient_stop = config_string(effective_config_value, &["themeVariables", "gradientStop"])
        .or_else(|| {
            config_string(
                effective_config_value,
                &["themeVariables", "secondaryBorderColor"],
            )
        })
        .unwrap_or_else(|| gradient_start.clone());

    let diagram_id = escape_xml(diagram_id);
    let gradient_start = escape_xml(&gradient_start);
    let gradient_stop = escape_xml(&gradient_stop);
    let _ = write!(
        out,
        r#"<linearGradient id="{}-gradient" gradientUnits="objectBoundingBox" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" stop-opacity="1"/><stop offset="100%" stop-color="{}" stop-opacity="1"/></linearGradient>"#,
        diagram_id.as_str(),
        gradient_start.as_str(),
        gradient_stop.as_str()
    );
}

fn er_unified_marker_id(diagram_id: &str, diagram_type: &str, upstream_marker: &str) -> String {
    let upstream_marker = upstream_marker.trim();
    let (base, suffix) = if let Some(v) = upstream_marker.strip_suffix("_START") {
        (v, "Start")
    } else if let Some(v) = upstream_marker.strip_suffix("_END") {
        (v, "End")
    } else {
        return upstream_marker.to_string();
    };

    let marker_type = match base {
        "ONLY_ONE" => "onlyOne",
        "ZERO_OR_ONE" => "zeroOrOne",
        "ONE_OR_MORE" => "oneOrMore",
        "ZERO_OR_MORE" => "zeroOrMore",
        "MD_PARENT" => "mdParent",
        _ => return upstream_marker.to_string(),
    };

    format!("{diagram_id}_{diagram_type}-{marker_type}{suffix}")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn er_font_family_css_uses_mermaid_default_fallback_for_raw_config() {
        assert_eq!(
            super::er_font_family_css(&json!({})),
            crate::config::MERMAID_DEFAULT_FONT_FAMILY_CSS
        );
    }

    #[test]
    fn er_font_family_css_prefers_theme_variables() {
        assert_eq!(
            super::er_font_family_css(&json!({
                "fontFamily": "Courier, monospace",
                "themeVariables": {
                    "fontFamily": "\"IBM Plex Sans\", Arial, sans-serif"
                }
            })),
            r#""IBM Plex Sans",Arial,sans-serif"#
        );
    }

    #[test]
    fn er_label_coordinate_path_decimal_rounding_without_regex() {
        assert_eq!(
            super::round_decimal_numbers_in_path("M-10.5 20.6 .5 10. 3.4.5"),
            "M-11 21 .5 10. 3.5"
        );

        assert!(super::is_label_coordinate_in_path(
            crate::model::LayoutPoint { x: -11.0, y: 99.0 },
            "M-10.5 20.6"
        ));
    }
}
