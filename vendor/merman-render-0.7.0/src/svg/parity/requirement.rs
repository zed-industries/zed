use super::state::StateRoughRectSpec;
use super::*;
use merman_core::diagrams::requirement::RequirementDiagramRenderModel;

// Requirement diagram SVG renderer implementation (split from parity.rs).

pub(super) fn render_requirement_diagram_svg(
    layout: &RequirementDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: RequirementDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    render_requirement_diagram_svg_model(layout, &model, effective_config, diagram_title, options)
}

pub(super) fn render_requirement_diagram_svg_model(
    layout: &RequirementDiagramLayout,
    model: &RequirementDiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    fn requirement_marker_id(diagram_id: &str, suffix: &str) -> String {
        format!("{diagram_id}_requirement-{suffix}")
    }

    fn mermaid_markdown_to_html(raw: &str) -> String {
        // Mermaid runs requirement node labels through its `markdownToHTML()` pipeline when
        // `htmlLabels=true` (via `createText(...)`). Keep the XHTML fragment XML-safe while still
        // preserving inline HTML like `<br/>` and Mermaid's minimal emphasis/strong subset.
        let decoded = decode_mermaid_entities_for_render_text(raw);
        crate::text::mermaid_markdown_to_xhtml_label_fragment(decoded.as_ref(), true)
    }

    fn requirement_label_uses_markdown_inline(raw: &str) -> bool {
        // Inline emphasis/strong spans, explicit `<br>` breaks, or multiline field content all go
        // through Mermaid's HTML-label Markdown path.
        let lower = raw.to_ascii_lowercase();
        raw.contains('*') || raw.contains('_') || raw.contains('\n') || lower.contains("<br")
    }

    #[derive(Debug, Clone, Copy)]
    enum LabelContent<'a> {
        Text(&'a str),
        Html(&'a str),
    }

    #[derive(Debug, Clone, Copy)]
    struct LabelForeignObject<'a> {
        content: LabelContent<'a>,
        width: f64,
        height: f64,
        span_class: &'a str,
        span_style: Option<&'a str>,
        div_class: Option<&'a str>,
        div_style_prefix: Option<&'a str>,
        max_width_px: i64,
    }

    fn mk_label_foreign_object(out: &mut String, spec: LabelForeignObject<'_>) {
        let LabelForeignObject {
            content,
            width,
            height,
            span_class,
            span_style,
            div_class,
            div_style_prefix,
            max_width_px,
        } = spec;
        let div_class_attr = div_class
            .map(|c| format!(r#" class="{c}""#))
            .unwrap_or_default();
        let span_style_attr = span_style
            .map(|s| format!(r#" style="{}""#, escape_xml(s)))
            .unwrap_or_default();
        let div_style_prefix = div_style_prefix.unwrap_or("");
        let _ = write!(
            out,
            r#"<foreignObject height="{h}" width="{w}"><div{div_class_attr} style="{div_style_prefix}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {max_width}px; text-align: center;"><span class="{span_class}"{span_style_attr}>"#,
            w = fmt(width),
            h = fmt(height),
            div_class_attr = div_class_attr,
            span_class = escape_xml(span_class),
            span_style_attr = span_style_attr,
            div_style_prefix = escape_xml(div_style_prefix),
            max_width = max_width_px,
        );
        match content {
            LabelContent::Text(text) => {
                out.push_str("<p>");
                escape_xml_into(out, text);
                out.push_str("</p>");
            }
            LabelContent::Html(html) => out.push_str(html),
        }
        out.push_str("</span></div></foreignObject>");
    }

    fn rough_double_line_path_d(x1: f64, y1: f64, x2: f64, y2: f64) -> String {
        let cx1 = (x1 + x2) / 2.0;
        let cy1 = (y1 + y2) / 2.0;
        let mut out = String::new();
        let _ = write!(
            &mut out,
            "M{x1} {y1} C{cx0} {cy0} {cx1} {cy1} {x2} {y2} M{x1b} {y1b} C{cx0b} {cy0b} {cx1b} {cy1b} {x2b} {y2b}",
            x1 = fmt_path(x1),
            y1 = fmt_path(y1),
            cx0 = fmt_path((x1 * 2.0 + x2) / 3.0),
            cy0 = fmt_path((y1 * 2.0 + y2) / 3.0),
            cx1 = fmt_path((x1 + x2 * 2.0) / 3.0),
            cy1 = fmt_path((y1 + y2 * 2.0) / 3.0),
            x2 = fmt_path(x2),
            y2 = fmt_path(y2),
            x1b = fmt_path(x1),
            y1b = fmt_path(y1),
            cx0b = fmt_path(cx1),
            cy0b = fmt_path(cy1),
            cx1b = fmt_path(cx1 + (x2 - x1) * 0.1),
            cy1b = fmt_path(cy1 + (y2 - y1) * 0.1),
            x2b = fmt_path(x2),
            y2b = fmt_path(y2),
        );
        out
    }

    fn rough_rect_stroke_path_d(x: f64, y: f64, w: f64, h: f64) -> String {
        let x2 = x + w;
        let y2 = y + h;
        let mut out = String::new();
        out.push_str(&rough_double_line_path_d(x, y, x2, y));
        out.push(' ');
        out.push_str(&rough_double_line_path_d(x2, y, x2, y2));
        out.push(' ');
        out.push_str(&rough_double_line_path_d(x2, y2, x, y2));
        out.push(' ');
        out.push_str(&rough_double_line_path_d(x, y2, x, y));
        out
    }

    fn is_prototype_pollution_id(id: &str) -> bool {
        id == "__proto__"
    }

    fn parse_node_style_overrides(
        css_styles: &[String],
    ) -> (
        String, // labelStyles (span/g)
        String, // labelStyles as a `<div style="...">` prefix
        String, // nodeStyles
        Option<String>,
        Option<String>,
        Option<f64>,
    ) {
        // Mirror Mermaid `styles2String(node)` output:
        // - De-duplicate by key (`Map` semantics) while preserving first insertion order.
        // - Split into label vs node styles via Mermaid `isLabelStyle`.
        // - Append ` !important` when emitting style strings.
        fn is_label_style(key: &str) -> bool {
            matches!(
                key,
                "color"
                    | "font-size"
                    | "font-family"
                    | "font-weight"
                    | "font-style"
                    | "text-decoration"
                    | "text-align"
                    | "text-transform"
                    | "line-height"
                    | "letter-spacing"
                    | "word-spacing"
                    | "text-shadow"
                    | "text-overflow"
                    | "white-space"
                    | "word-wrap"
                    | "word-break"
                    | "overflow-wrap"
                    | "hyphens"
            )
        }

        let mut styles: IndexMap<String, String> = IndexMap::new();
        for raw in css_styles {
            let s = raw.trim().trim_end_matches(';');
            let Some((k, v)) = s.split_once(':') else {
                continue;
            };
            let k = k.trim().to_string();
            let mut v = v.trim().to_string();
            if k.is_empty() || v.is_empty() {
                continue;
            }
            if let Some((vv, _)) = v.split_once("!important") {
                v = vv.trim().to_string();
            }

            // JS `Map#set` overwrites the value without changing the key order.
            if let Some(existing) = styles.get_mut(&k) {
                *existing = v;
            } else {
                styles.insert(k, v);
            }
        }

        let mut label_kv: Vec<(&str, &str)> = Vec::new();
        let mut node_kv: Vec<(&str, &str)> = Vec::new();
        for (k, v) in &styles {
            if is_label_style(k.trim().to_ascii_lowercase().as_str()) {
                label_kv.push((k.as_str(), v.as_str()));
            } else {
                node_kv.push((k.as_str(), v.as_str()));
            }
        }

        let label_styles = label_kv
            .iter()
            .map(|(k, v)| format!("{k}:{v} !important"))
            .collect::<Vec<_>>()
            .join(";");
        let label_div_style_prefix = label_kv
            .iter()
            .map(|(k, v)| format!("{k}: {v} !important; "))
            .collect::<Vec<_>>()
            .join("");
        let node_styles = node_kv
            .iter()
            .map(|(k, v)| format!("{k}:{v} !important"))
            .collect::<Vec<_>>()
            .join(";");

        let fill = styles.get("fill").cloned();
        let stroke = styles.get("stroke").cloned();
        let stroke_width = styles
            .get("stroke-width")
            .and_then(|v| v.trim_end_matches("px").trim().parse::<f64>().ok());

        (
            label_styles,
            label_div_style_prefix,
            node_styles,
            fill,
            stroke,
            stroke_width,
        )
    }

    let diagram_id = options.diagram_id.as_deref().unwrap_or("requirement");
    let look = config_diagram_look(effective_config);
    let look = look.as_str();
    let look_attr = format!(r#" data-look="{}""#, escape_xml(look));
    let node_id_prefix = if diagram_id.is_empty() {
        String::new()
    } else {
        format!("{}-", diagram_id)
    };

    let relationships = &model.relationships;
    let req_by_id: std::collections::HashMap<&str, _> = model
        .requirements
        .iter()
        .map(|n| (n.name.as_str(), n))
        .collect();
    let el_by_id: std::collections::HashMap<&str, _> = model
        .elements
        .iter()
        .map(|n| (n.name.as_str(), n))
        .collect();

    let measurer = crate::text::VendoredFontMetricsTextMeasurer::default();
    let font_family = Some(crate::config::config_font_family_or_first_array_css(
        effective_config,
    ));
    let font_size = crate::config::config_theme_or_root_font_size_px(effective_config, 16.0);
    let theme = SvgTheme::new(effective_config);
    let default_fill_color = theme.color("requirementBackground", "#ECECFF");
    let default_stroke_color = theme.color("nodeBorder", "#9370DB");
    let hand_drawn_seed = effective_config
        .get("handDrawnSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let calc_style = TextStyle {
        font_family: font_family.clone(),
        font_size,
        font_weight: None,
    };
    let html_style_regular = TextStyle {
        font_family: font_family.clone(),
        font_size,
        font_weight: None,
    };
    let html_style_bold = TextStyle {
        font_family,
        font_size,
        font_weight: Some("bold".to_string()),
    };

    fn calculate_text_width_like_mermaid_px(
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
        text: &str,
    ) -> i64 {
        // Mermaid `calculateTextWidth` uses SVG `<text>` bbox widths, rounds to integers, and takes
        // the maximum width across `sans-serif` and the configured `fontFamily`.
        let mut sans = style.clone();
        sans.font_family = Some("sans-serif".to_string());
        sans.font_weight = None;

        let mut fam = style.clone();
        fam.font_weight = None;

        let (l1, r1) = measurer.measure_svg_title_bbox_x(text, &sans);
        let (l2, r2) = measurer.measure_svg_title_bbox_x(text, &fam);
        let w1 = (l1 + r1).max(0.0);
        let w2 = (l2 + r2).max(0.0);

        w1.max(w2).round() as i64
    }

    #[derive(Clone, Debug)]
    struct RequirementNodeLabelLine {
        display_text: String,
        display_html: Option<String>,
        max_width_px: i64,
        html_width: f64,
        html_height: f64,
        y_offset: f64,
        bold: bool,
        // Type/name are centered; body labels are left-aligned to the box inner padding.
        keep_centered: bool,
    }

    fn measure_node_label_line(
        measurer: &dyn TextMeasurer,
        html_style_regular: &TextStyle,
        html_style_bold: &TextStyle,
        calc_style: &TextStyle,
        display_text: &str,
        calc_text: &str,
        bold: bool,
    ) -> Option<(f64, f64, i64)> {
        if display_text.trim().is_empty() {
            return None;
        }

        let html_style = if bold {
            html_style_bold
        } else {
            html_style_regular
        };
        let font_size = html_style.font_size.max(1.0);
        let looks_like_markdown_inline = requirement_label_uses_markdown_inline(display_text);
        let measured = if looks_like_markdown_inline {
            crate::text::measure_markdown_with_flowchart_bold_deltas(
                measurer,
                display_text,
                html_style,
                None,
                crate::text::WrapMode::HtmlLike,
            )
        } else {
            measurer.measure_wrapped(
                display_text,
                html_style,
                None,
                crate::text::WrapMode::HtmlLike,
            )
        };
        let height = measured.height.max(1.0);
        let width = if let Some(em) =
            crate::generated::requirement_text_overrides_11_12_2::lookup_requirement_html_label_width_em(
                display_text,
                bold,
            )
        {
            (em * font_size).max(1.0)
        } else {
            measured.width.max(1.0)
        };
        let max_w = if let Some(px) =
            crate::generated::requirement_text_overrides_11_12_2::lookup_requirement_calc_max_width_px(
                calc_text,
            )
        {
            px
        } else {
            let calc_input =
                if calc_text.contains('\n') || calc_text.to_ascii_lowercase().contains("<br") {
                    crate::flowchart::flowchart_label_plain_text_for_layout(calc_text, "text", true)
                } else {
                    calc_text.to_string()
                };
            let calc_w = calculate_text_width_like_mermaid_px(measurer, calc_style, &calc_input);
            (calc_w + 50).max(0)
        };
        Some((width, height, max_w))
    }

    fn requirement_edge_id(src: &str, dst: &str, idx: usize) -> String {
        format!("{src}-{dst}-{idx}")
    }

    let mut edge_rel_type_by_id: std::collections::HashMap<String, &str> =
        std::collections::HashMap::new();
    for rel in relationships {
        // Match upstream edge id collisions (counter is always 0).
        let edge_id = requirement_edge_id(&rel.src, &rel.dst, 0);
        edge_rel_type_by_id.insert(edge_id, rel.rel_type.as_str());
    }

    let bounds = layout.bounds.clone().unwrap_or_else(|| {
        if layout.nodes.is_empty() && layout.edges.is_empty() {
            Bounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 0.0,
                max_y: 0.0,
            }
        } else {
            compute_layout_bounds(&[], &layout.nodes, &layout.edges).unwrap_or(Bounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 100.0,
                max_y: 100.0,
            })
        }
    });
    let viewport_padding = 8.0;
    let vb_x = bounds.min_x - viewport_padding;
    let vb_y = bounds.min_y - viewport_padding;
    let vb_w = ((bounds.max_x - bounds.min_x) + 2.0 * viewport_padding).max(1.0);
    let vb_h = ((bounds.max_y - bounds.min_y) + 2.0 * viewport_padding).max(1.0);
    fn js_to_precision_fixed(v: f64, precision: i32) -> String {
        // Match JavaScript `Number(v).toPrecision(precision)` for the range of SVG widths we use
        // in Mermaid fixtures (fixed notation, no exponent branch needed).
        if !v.is_finite() {
            return "0".to_string();
        }
        if v == 0.0 {
            let decimals = (precision - 1).max(0) as usize;
            return format!("{:.*}", decimals, 0.0);
        }

        let abs = v.abs();
        let exponent = abs.log10().floor() as i32;
        let decimals = (precision - (exponent + 1)).max(0) as usize;
        format!("{:.*}", decimals, v)
    }
    let max_width_style = js_to_precision_fixed(vb_w, 6);

    let mut out = String::new();

    let vb_x_attr = fmt_string(vb_x);
    let vb_y_attr = fmt_string(vb_y);
    let mut vb_w_attr = fmt_string(vb_w);
    let mut vb_h_attr = fmt_string(vb_h);
    let mut max_width_style_attr = max_width_style.clone();
    let mut viewbox_attr = format!("{vb_x_attr} {vb_y_attr} {vb_w_attr} {vb_h_attr}");
    apply_root_viewport_override(
        diagram_id,
        &mut viewbox_attr,
        &mut vb_w_attr,
        &mut vb_h_attr,
        &mut max_width_style_attr,
        crate::generated::requirement_root_overrides_11_12_2::lookup_requirement_root_viewport_override,
    );

    let mut aria_labelledby: Option<String> = None;
    let mut aria_describedby: Option<String> = None;
    let mut a11y_nodes = String::new();
    if let Some(t) = model
        .acc_title
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty())
    {
        let title_id = format!("chart-title-{diagram_id}");
        aria_labelledby = Some(escape_xml(&title_id));
        let _ = write!(
            &mut a11y_nodes,
            r#"<title id="{}">{}</title>"#,
            escape_xml(&title_id),
            escape_xml(t)
        );
    }
    if let Some(d) = model
        .acc_descr
        .as_deref()
        .map(str::trim)
        .filter(|d| !d.is_empty())
    {
        let desc_id = format!("chart-desc-{diagram_id}");
        aria_describedby = Some(escape_xml(&desc_id));
        let _ = write!(
            &mut a11y_nodes,
            r#"<desc id="{}">{}</desc>"#,
            escape_xml(&desc_id),
            escape_xml(d)
        );
    }

    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            class: Some("requirementDiagram"),
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(&format!(
                "max-width: {max_width_style_attr}px; background-color: white;"
            )),
            viewbox_attr: Some(&viewbox_attr),
            aria_labelledby: aria_labelledby.as_deref(),
            aria_describedby: aria_describedby.as_deref(),
            ..root_svg::SvgRootAttrs::new(diagram_id, "requirement")
        },
    );

    out.push_str(&a11y_nodes);

    let _ = write!(
        &mut out,
        r#"<style>{}</style>"#,
        requirement_css(diagram_id, effective_config)
    );

    out.push_str("<g>");

    // Markers.
    let contains_marker_id = requirement_marker_id(diagram_id, "requirement_containsStart");
    let arrow_marker_id = requirement_marker_id(diagram_id, "requirement_arrowEnd");
    let _ = write!(
        &mut out,
        r#"<defs><marker id="{id}" refX="0" refY="10" markerWidth="20" markerHeight="20" orient="auto"><g><circle cx="10" cy="10" r="9" fill="none"/><line x1="1" x2="19" y1="10" y2="10"/><line y1="1" y2="19" x1="10" x2="10"/></g></marker></defs>"#,
        id = escape_xml(&contains_marker_id)
    );
    let _ = write!(
        &mut out,
        r#"<defs><marker id="{id}" refX="20" refY="10" markerWidth="20" markerHeight="20" orient="auto"><path d="M0,0&#10;      L20,10&#10;      M20,10&#10;      L0,20"/></marker></defs>"#,
        id = escape_xml(&arrow_marker_id)
    );

    out.push_str(r#"<g class="root">"#);
    out.push_str(r#"<g class="clusters"/>"#);

    out.push_str(r#"<g class="edgePaths">"#);
    for e in &layout.edges {
        let rel_type = edge_rel_type_by_id.get(&e.id).copied().unwrap_or("");
        let is_contains = rel_type == "contains";
        let pattern = if is_contains { "solid" } else { "dashed" };
        let class = format!("edge-thickness-normal edge-pattern-{pattern} relationshipLine");
        let style = if is_contains {
            "fill:none;;;;fill:none;"
        } else {
            "fill:none;stroke-dasharray: 10,7;;;fill:none;stroke-dasharray: 10,7"
        };

        let d = curve_basis_path_d(&e.points);
        let data_points_b64 =
            base64::engine::general_purpose::STANDARD.encode(json_stringify_points(&e.points));

        let marker_attr = if is_contains {
            format!(
                r#" marker-start="url(#{})""#,
                escape_xml(&contains_marker_id)
            )
        } else {
            format!(r#" marker-end="url(#{})""#, escape_xml(&arrow_marker_id))
        };

        let _ = write!(
            &mut out,
            r#"<path d="{d}" id="{dom_id}" class="{class}" style="{style}" data-edge="true" data-et="edge" data-id="{id}" data-points="{data_points}"{look_attr}{marker_attr}/>"#,
            d = escape_xml(&d),
            dom_id = escape_xml(&format!("{}{}", node_id_prefix, e.id)),
            id = escape_xml(&e.id),
            class = escape_xml(&class),
            style = escape_xml(style),
            data_points = escape_xml(&data_points_b64),
            look_attr = look_attr.as_str(),
            marker_attr = marker_attr,
        );
    }
    out.push_str("</g>");

    out.push_str(r#"<g class="edgeLabels">"#);
    for e in &layout.edges {
        let rel_type = edge_rel_type_by_id.get(&e.id).copied().unwrap_or("");
        if rel_type.trim().is_empty() {
            continue;
        }
        let label_text = format!("<<{rel_type}>>");
        let label_calc = format!("&lt;&lt;{rel_type}&gt;&gt;");
        let max_width_px = measure_node_label_line(
            &measurer,
            &html_style_regular,
            &html_style_bold,
            &calc_style,
            &label_text,
            &label_calc,
            false,
        )
        .map(|(_, _, max_w)| max_w)
        .unwrap_or(200);

        let (x, y, w, h) = e
            .label
            .as_ref()
            .map(|l| (l.x, l.y, l.width, l.height))
            .unwrap_or_else(|| {
                let mid = e
                    .points
                    .get(1)
                    .cloned()
                    .unwrap_or(crate::model::LayoutPoint { x: 0.0, y: 0.0 });
                (mid.x, mid.y, 0.0, 0.0)
            });
        let _ = write!(
            &mut out,
            r#"<g class="edgeLabel" transform="translate({x}, {y})"><g class="label" data-id="{id}" transform="translate({lx}, {ly})">"#,
            x = fmt(x),
            y = fmt(y),
            id = escape_xml(&e.id),
            lx = fmt(-w / 2.0),
            ly = fmt(-h / 2.0),
        );
        mk_label_foreign_object(
            &mut out,
            LabelForeignObject {
                content: LabelContent::Text(&label_text),
                width: w,
                height: h,
                span_class: "edgeLabel",
                span_style: None,
                div_class: Some("labelBkg"),
                div_style_prefix: None,
                max_width_px,
            },
        );
        out.push_str("</g></g>");
    }
    out.push_str("</g>");

    out.push_str(r#"<g class="nodes">"#);
    for n in &layout.nodes {
        if n.id == "__proto__" {
            continue;
        }
        let cx = n.x + n.width / 2.0;
        let cy = n.y + n.height / 2.0;

        let mut node_classes: Vec<&str> = Vec::new();
        let mut css_styles: &[String] = &[];
        let mut label_lines: Vec<RequirementNodeLabelLine> = Vec::new();
        let mut type_height = 0.0;
        let mut name_height = 0.0;
        let mut has_body = false;
        if let Some(req) = req_by_id.get(n.id.as_str()) {
            node_classes = req.classes.iter().map(String::as_str).collect();
            css_styles = &req.css_styles;
            let style_bold = crate::requirement::requirement_styles_force_bold(css_styles);

            let type_display = format!("<<{}>>", req.node_type);
            let type_calc = format!("&lt;&lt;{}&gt;&gt;", req.node_type);
            let Some((w, h, max_w)) = measure_node_label_line(
                &measurer,
                &html_style_regular,
                &html_style_bold,
                &calc_style,
                &type_display,
                &type_calc,
                style_bold,
            ) else {
                return Err(Error::InvalidModel {
                    message: format!("missing requirement type label for {}", req.name),
                });
            };
            type_height = h;
            label_lines.push(RequirementNodeLabelLine {
                display_text: type_display,
                display_html: None,
                max_width_px: max_w,
                html_width: w,
                html_height: h,
                y_offset: 0.0,
                bold: false,
                keep_centered: true,
            });

            let Some((w, h, max_w)) = measure_node_label_line(
                &measurer,
                &html_style_regular,
                &html_style_bold,
                &calc_style,
                &req.name,
                &req.name,
                true,
            ) else {
                return Err(Error::InvalidModel {
                    message: format!("missing requirement name label for {}", req.name),
                });
            };
            name_height = h;
            label_lines.push(RequirementNodeLabelLine {
                display_text: req.name.clone(),
                display_html: None,
                max_width_px: max_w,
                html_width: w,
                html_height: h,
                y_offset: type_height,
                bold: true,
                keep_centered: true,
            });

            let gap = 20.0;
            let mut y_offset = type_height + name_height + gap;

            let id_line = req.requirement_id.trim();
            if !id_line.is_empty() {
                let t = format!("ID: {}", id_line);
                if let Some((w, h, max_w)) = measure_node_label_line(
                    &measurer,
                    &html_style_regular,
                    &html_style_bold,
                    &calc_style,
                    &t,
                    &t,
                    style_bold,
                ) {
                    label_lines.push(RequirementNodeLabelLine {
                        display_text: t,
                        display_html: None,
                        max_width_px: max_w,
                        html_width: w,
                        html_height: h,
                        y_offset,
                        bold: false,
                        keep_centered: false,
                    });
                    y_offset += h;
                    has_body = true;
                }
            }
            let text_line = req.text.trim();
            if !text_line.is_empty() {
                let t = format!("Text: {}", text_line);
                if let Some((w, h, max_w)) = measure_node_label_line(
                    &measurer,
                    &html_style_regular,
                    &html_style_bold,
                    &calc_style,
                    &t,
                    &t,
                    style_bold,
                ) {
                    label_lines.push(RequirementNodeLabelLine {
                        display_text: t,
                        display_html: None,
                        max_width_px: max_w,
                        html_width: w,
                        html_height: h,
                        y_offset,
                        bold: false,
                        keep_centered: false,
                    });
                    y_offset += h;
                    has_body = true;
                }
            }
            let risk_line = req.risk.trim();
            if !risk_line.is_empty() {
                let t = format!("Risk: {}", risk_line);
                if let Some((w, h, max_w)) = measure_node_label_line(
                    &measurer,
                    &html_style_regular,
                    &html_style_bold,
                    &calc_style,
                    &t,
                    &t,
                    style_bold,
                ) {
                    label_lines.push(RequirementNodeLabelLine {
                        display_text: t,
                        display_html: None,
                        max_width_px: max_w,
                        html_width: w,
                        html_height: h,
                        y_offset,
                        bold: false,
                        keep_centered: false,
                    });
                    y_offset += h;
                    has_body = true;
                }
            }
            let verify_line = req.verify_method.trim();
            if !verify_line.is_empty() {
                let t = format!("Verification: {}", verify_line);
                if let Some((w, h, max_w)) = measure_node_label_line(
                    &measurer,
                    &html_style_regular,
                    &html_style_bold,
                    &calc_style,
                    &t,
                    &t,
                    style_bold,
                ) {
                    label_lines.push(RequirementNodeLabelLine {
                        display_text: t,
                        display_html: None,
                        max_width_px: max_w,
                        html_width: w,
                        html_height: h,
                        y_offset,
                        bold: false,
                        keep_centered: false,
                    });
                    has_body = true;
                }
            }
        } else if let Some(el) = el_by_id.get(n.id.as_str()) {
            node_classes = el.classes.iter().map(String::as_str).collect();
            css_styles = &el.css_styles;
            let style_bold = crate::requirement::requirement_styles_force_bold(css_styles);

            let type_display = "<<Element>>".to_string();
            let type_calc = "&lt;&lt;Element&gt;&gt;".to_string();
            let Some((w, h, max_w)) = measure_node_label_line(
                &measurer,
                &html_style_regular,
                &html_style_bold,
                &calc_style,
                &type_display,
                &type_calc,
                style_bold,
            ) else {
                return Err(Error::InvalidModel {
                    message: format!("missing element type label for {}", el.name),
                });
            };
            type_height = h;
            label_lines.push(RequirementNodeLabelLine {
                display_text: type_display,
                display_html: None,
                max_width_px: max_w,
                html_width: w,
                html_height: h,
                y_offset: 0.0,
                bold: false,
                keep_centered: true,
            });

            let Some((w, h, max_w)) = measure_node_label_line(
                &measurer,
                &html_style_regular,
                &html_style_bold,
                &calc_style,
                &el.name,
                &el.name,
                true,
            ) else {
                return Err(Error::InvalidModel {
                    message: format!("missing element name label for {}", el.name),
                });
            };
            name_height = h;
            label_lines.push(RequirementNodeLabelLine {
                display_text: el.name.clone(),
                display_html: None,
                max_width_px: max_w,
                html_width: w,
                html_height: h,
                y_offset: type_height,
                bold: true,
                keep_centered: true,
            });

            let gap = 20.0;
            let mut y_offset = type_height + name_height + gap;

            let type_line = el.element_type.trim();
            if !type_line.is_empty() {
                let t = format!("Type: {}", type_line);
                if let Some((w, h, max_w)) = measure_node_label_line(
                    &measurer,
                    &html_style_regular,
                    &html_style_bold,
                    &calc_style,
                    &t,
                    &t,
                    style_bold,
                ) {
                    label_lines.push(RequirementNodeLabelLine {
                        display_text: t,
                        display_html: None,
                        max_width_px: max_w,
                        html_width: w,
                        html_height: h,
                        y_offset,
                        bold: false,
                        keep_centered: false,
                    });
                    y_offset += h;
                    has_body = true;
                }
            }
            let doc_line = el.doc_ref.trim();
            if !doc_line.is_empty() {
                let t = format!("Doc Ref: {}", doc_line);
                if let Some((w, h, max_w)) = measure_node_label_line(
                    &measurer,
                    &html_style_regular,
                    &html_style_bold,
                    &calc_style,
                    &t,
                    &t,
                    style_bold,
                ) {
                    label_lines.push(RequirementNodeLabelLine {
                        display_text: t,
                        display_html: None,
                        max_width_px: max_w,
                        html_width: w,
                        html_height: h,
                        y_offset,
                        bold: false,
                        keep_centered: false,
                    });
                    has_body = true;
                }
            }
        }

        if !node_classes.contains(&"default") {
            node_classes.insert(0, "default");
        }
        let classes_str = if node_classes.is_empty() {
            "node default".to_string()
        } else {
            format!("node {}", node_classes.join(" "))
        };
        let id_attr = if is_prototype_pollution_id(&n.id) {
            String::new()
        } else {
            format!(
                r#" id="{}{}""#,
                escape_xml(&node_id_prefix),
                escape_xml(&n.id)
            )
        };

        let _ = write!(
            &mut out,
            r#"<g class="{class}"{id_attr}{look_attr} transform="translate({cx}, {cy})">"#,
            class = escape_xml(&classes_str),
            id_attr = id_attr,
            look_attr = look_attr.as_str(),
            cx = fmt(cx),
            cy = fmt(cy),
        );

        let (
            label_styles,
            label_div_style_prefix,
            node_styles,
            fill_override,
            stroke_override,
            stroke_width_override,
        ) = parse_node_style_overrides(css_styles);
        let fill_color = fill_override.as_deref().unwrap_or(&default_fill_color);
        let stroke_color = stroke_override.as_deref().unwrap_or(&default_stroke_color);
        let stroke_width = stroke_width_override.unwrap_or(1.3);

        let x = -n.width / 2.0;
        let y = -n.height / 2.0;
        let fill_path = format!(
            "M{} {} L{} {} L{} {} L{} {}",
            fmt(x),
            fmt(y),
            fmt(x + n.width),
            fmt(y),
            fmt(x + n.width),
            fmt(y + n.height),
            fmt(x),
            fmt(y + n.height)
        );
        let stroke_path = roughjs_paths_for_rect(StateRoughRectSpec {
            x,
            y,
            w: n.width,
            h: n.height,
            fill: fill_color,
            stroke: stroke_color,
            stroke_width: stroke_width as f32,
            seed: hand_drawn_seed,
        })
        .map(|(_, stroke_d)| stroke_d)
        .unwrap_or_else(|| rough_rect_stroke_path_d(x, y, n.width, n.height));

        let _ = write!(
            &mut out,
            r#"<g class="{class}" style="{style}">"#,
            class = "basic label-container outer-path",
            style = escape_xml(&node_styles)
        );
        let _ = write!(
            &mut out,
            r##"<path d="{d}" stroke="none" stroke-width="0" fill="{fill}"/>"##,
            d = escape_xml(&fill_path),
            fill = escape_xml(fill_color),
        );
        let _ = write!(
            &mut out,
            r##"<path d="{d}" stroke="{stroke}" stroke-width="{stroke_width}" fill="none" stroke-dasharray="0 0"/>"##,
            d = escape_xml(&stroke_path),
            stroke = escape_xml(stroke_color),
            stroke_width = fmt(stroke_width),
        );
        out.push_str("</g>");

        // Labels.
        let padding = 20.0;
        for line in label_lines.iter_mut() {
            if line.display_html.is_none()
                && requirement_label_uses_markdown_inline(&line.display_text)
            {
                line.display_html = Some(mermaid_markdown_to_html(&line.display_text));
            }
        }
        for line in &label_lines {
            let label_x = if line.keep_centered {
                -line.html_width / 2.0
            } else {
                x + padding / 2.0
            };
            let label_y = y + line.y_offset - line.html_height / 2.0 + padding;
            let style = if line.bold {
                format!("{label_styles}; font-weight: bold;")
            } else {
                label_styles.clone()
            };
            let span_style = if style.trim().is_empty() {
                None
            } else {
                Some(style.as_str())
            };
            let div_style_prefix = {
                let mut p = String::new();
                if !label_div_style_prefix.is_empty() {
                    p.push_str(&label_div_style_prefix);
                }
                if line.bold {
                    p.push_str("font-weight: bold; ");
                }
                if p.is_empty() { None } else { Some(p) }
            };
            let div_style_prefix = div_style_prefix.as_deref();
            let _ = write!(
                &mut out,
                r#"<g class="label" style="{style}" transform="translate({x}, {y})">"#,
                style = escape_xml(&style),
                x = fmt(label_x),
                y = fmt(label_y),
            );
            mk_label_foreign_object(
                &mut out,
                LabelForeignObject {
                    content: line
                        .display_html
                        .as_deref()
                        .map(LabelContent::Html)
                        .unwrap_or_else(|| LabelContent::Text(&line.display_text)),
                    width: line.html_width,
                    height: line.html_height,
                    span_class: "markdown-node-label nodeLabel",
                    span_style,
                    div_class: None,
                    div_style_prefix,
                    max_width_px: line.max_width_px,
                },
            );
            out.push_str("</g>");
        }

        if has_body {
            let gap = 20.0;
            let divider_y = y + type_height + name_height + gap;
            let divider_d = if let Some(stroke) = roughjs_parse_hex_color_to_srgba(stroke_color) {
                if let Ok(mut opts) = roughr::core::OptionsBuilder::default()
                    .seed(hand_drawn_seed)
                    .roughness(0.0)
                    .fill_style(roughr::core::FillStyle::Solid)
                    .stroke(stroke)
                    .stroke_width(stroke_width as f32)
                    .stroke_line_dash(vec![0.0, 0.0])
                    .stroke_line_dash_offset(0.0)
                    .fill_line_dash(vec![0.0, 0.0])
                    .fill_line_dash_offset(0.0)
                    .disable_multi_stroke(false)
                    .disable_multi_stroke_fill(false)
                    .build()
                {
                    roughjs_ops_to_svg_path_d(&roughr::renderer::line::<f64>(
                        x,
                        divider_y,
                        x + n.width,
                        divider_y,
                        &mut opts,
                    ))
                } else {
                    rough_double_line_path_d(x, divider_y, x + n.width, divider_y)
                }
            } else {
                rough_double_line_path_d(x, divider_y, x + n.width, divider_y)
            };
            let _ = write!(
                &mut out,
                r##"<g class="divider" style="{style}"><path d="{d}" stroke="{stroke}" stroke-width="{stroke_width}" fill="none" stroke-dasharray="0 0"/></g>"##,
                style = escape_xml(&node_styles),
                d = escape_xml(&divider_d),
                stroke = escape_xml(stroke_color),
                stroke_width = fmt(stroke_width),
            );
        }

        out.push_str("</g>");
    }
    out.push_str("</g>");

    out.push_str("</g></g>");

    let diagram_title = diagram_title.map(str::trim).filter(|t| !t.is_empty());
    if let Some(title) = diagram_title {
        let vb_x_f = vb_x_attr.parse::<f64>().unwrap_or(vb_x);
        let vb_y_f = vb_y_attr.parse::<f64>().unwrap_or(vb_y);
        let vb_w_f = vb_w_attr.parse::<f64>().unwrap_or(vb_w);
        // Mermaid places requirement diagram titles at a fixed offset below the viewBox top.
        let title_x = vb_x_f + vb_w_f / 2.0;
        let title_y = vb_y_f + 23.0;
        let _ = write!(
            &mut out,
            r#"<text text-anchor="middle" x="{x}" y="{y}" class="requirementDiagramTitleText">{txt}</text>"#,
            x = fmt(title_x),
            y = fmt(title_y),
            txt = escape_xml(title),
        );
    }

    push_requirement_shadow_defs(&mut out, diagram_id, effective_config);

    out.push_str("</svg>\n");
    Ok(out)
}

fn push_requirement_shadow_defs(
    out: &mut String,
    diagram_id: &str,
    effective_config: &serde_json::Value,
) {
    let flood_color = effective_config
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
