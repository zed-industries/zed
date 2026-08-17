use super::*;
use crate::model::IshikawaTextLayout;

pub(super) fn render_ishikawa_diagram_svg(
    layout: &IshikawaDiagramLayout,
    _semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("ishikawa");
    let viewbox_attr = format!(
        "{} {} {} {}",
        fmt(layout.viewbox_x),
        fmt(layout.viewbox_y),
        fmt(layout.total_width),
        fmt(layout.total_height)
    );
    let fixed_width = fmt_string(layout.total_width);
    let fixed_height = fmt_string(layout.total_height);
    let max_width = fmt_string(layout.total_width);
    let style_attr = format!("max-width: {max_width}px; background-color: white;");

    let mut out = String::new();
    if layout.use_max_width {
        root_svg::push_svg_root_open(
            &mut out,
            root_svg::SvgRootAttrs {
                width: root_svg::SvgRootWidth::Percent100,
                style_attr: Some(style_attr.as_str()),
                viewbox_attr: Some(viewbox_attr.as_str()),
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, "ishikawa")
            },
        );
    } else {
        root_svg::push_svg_root_open(
            &mut out,
            root_svg::SvgRootAttrs {
                width: root_svg::SvgRootWidth::Fixed(fixed_width.as_str()),
                height_attr: Some(fixed_height.as_str()),
                style_attr: Some("background-color: white;"),
                viewbox_attr: Some(viewbox_attr.as_str()),
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, "ishikawa")
            },
        );
    }

    let css = ishikawa_css(layout, effective_config);
    let marker_id = format!("ishikawa-arrow-{diagram_id}");
    let _ = write!(&mut out, "<style>{css}</style>");
    out.push_str("<g/>");
    let _ = write!(&mut out, r#"<g class="ishikawa"><defs><marker id=""#);
    escape_xml_into(&mut out, &marker_id);
    out.push_str(
        r#"" viewBox="0 0 10 10" refX="0" refY="5" markerWidth="6" markerHeight="6" orient="auto"><path d="M 10 0 L 0 5 L 10 10 Z" class="ishikawa-arrow"></path></marker></defs>"#,
    );

    if let Some(head) = &layout.head {
        let _ = write!(
            &mut out,
            r#"<g class="ishikawa-head-group" transform="translate({}, {})"><path class="ishikawa-head" d=""#,
            fmt(head.x),
            fmt(head.y)
        );
        escape_attr_into(&mut out, &head.path_d);
        out.push_str(r#""></path>"#);
        push_ishikawa_head_text(&mut out, &head.label, -head.x, -head.y);
        out.push_str("</g>");
    }

    for label_box in &layout.label_boxes {
        let _ = write!(
            &mut out,
            r#"<rect class="ishikawa-label-box" x="{}" y="{}" width="{}" height="{}"></rect>"#,
            fmt(label_box.x),
            fmt(label_box.y),
            fmt(label_box.width),
            fmt(label_box.height)
        );
    }

    for line in &layout.lines {
        let _ = write!(
            &mut out,
            r#"<line class="{}" x1="{}" y1="{}" x2="{}" y2="{}""#,
            escape_attr_display(&line.class_name),
            fmt(line.x1),
            fmt(line.y1),
            fmt(line.x2),
            fmt(line.y2)
        );
        if line.marker_start {
            let _ = write!(
                &mut out,
                r#" marker-start="url(#{})""#,
                escape_attr_display(&marker_id)
            );
        }
        out.push_str("></line>");
    }

    for label in &layout.labels {
        push_text_with_offset(&mut out, label, 0.0, 0.0);
    }

    out.push_str("</g></svg>\n");
    Ok(out)
}

fn push_ishikawa_head_text(out: &mut String, text: &IshikawaTextLayout, dx: f64, dy: f64) {
    let mut shifted = text.clone();
    shifted.anchor = "start".to_string();
    shifted.x = 0.0;
    shifted.y += dy;
    let transform_x = text.x + dx - (text.bbox.max_x - text.bbox.min_x) / 2.0;
    let transform_y = text.y + dy - shifted.y;
    let first_y =
        shifted.y - ((shifted.lines.len().saturating_sub(1)) as f64 * shifted.line_height) / 2.0;
    let _ = write!(
        out,
        r#"<text class="{}" text-anchor="{}" x="{}" y="{}" transform="translate({},{})">"#,
        escape_attr_display(&shifted.class_name),
        escape_attr_display(&shifted.anchor),
        fmt(shifted.x),
        fmt(first_y),
        fmt(transform_x),
        fmt(transform_y)
    );
    for (idx, line) in shifted.lines.iter().enumerate() {
        let _ = write!(
            out,
            r#"<tspan x="{}" dy="{}">"#,
            fmt(shifted.x),
            if idx == 0 {
                "0".to_string()
            } else {
                fmt_string(shifted.line_height)
            }
        );
        escape_xml_into(out, line);
        out.push_str("</tspan>");
    }
    out.push_str("</text>");
}

fn push_text_with_offset(out: &mut String, text: &IshikawaTextLayout, dx: f64, dy: f64) {
    let first_y =
        text.y + dy - ((text.lines.len().saturating_sub(1)) as f64 * text.line_height) / 2.0;
    let _ = write!(
        out,
        r#"<text class="{}" text-anchor="{}" x="{}" y="{}">"#,
        escape_attr_display(&text.class_name),
        escape_attr_display(&text.anchor),
        fmt(text.x + dx),
        fmt(first_y)
    );
    if text.lines.is_empty() {
        escape_xml_into(out, &text.text);
    } else {
        for (idx, line) in text.lines.iter().enumerate() {
            let _ = write!(
                out,
                r#"<tspan x="{}" dy="{}">"#,
                fmt(text.x + dx),
                if idx == 0 {
                    "0".to_string()
                } else {
                    fmt_string(text.line_height)
                }
            );
            escape_xml_into(out, line);
            out.push_str("</tspan>");
        }
    }
    out.push_str("</text>");
}

fn ishikawa_css(layout: &IshikawaDiagramLayout, effective_config: &serde_json::Value) -> String {
    let theme = PresentationTheme::new(effective_config).ishikawa();
    let font_size = crate::config::config_css_number_or_string(effective_config, &["fontSize"])
        .unwrap_or_else(|| format!("{}px", fmt_string(layout.font_size)));

    format!(
        ".ishikawa .ishikawa-spine,.ishikawa .ishikawa-branch,.ishikawa .ishikawa-sub-branch {{ stroke: {line_color}; stroke-width: 2; fill: none; }}\
.ishikawa .ishikawa-sub-branch {{ stroke-width: 1; }}\
.ishikawa .ishikawa-arrow {{ fill: {line_color}; }}\
.ishikawa .ishikawa-head {{ fill: {main_bkg}; stroke: {line_color}; stroke-width: 2; }}\
.ishikawa .ishikawa-label-box {{ fill: {main_bkg}; stroke: {line_color}; stroke-width: 2; }}\
.ishikawa text {{ font-family: {font_family}; font-size: {font_size}; fill: {text_color}; }}\
.ishikawa .ishikawa-head-label {{ font-weight: 600; text-anchor: middle; dominant-baseline: middle; font-size: 14px; }}\
.ishikawa .ishikawa-label {{ text-anchor: end; }}\
.ishikawa .ishikawa-label.cause {{ text-anchor: middle; dominant-baseline: middle; }}\
.ishikawa .ishikawa-label.align {{ text-anchor: end; dominant-baseline: middle; }}\
.ishikawa .ishikawa-label.up {{ dominant-baseline: baseline; }}\
.ishikawa .ishikawa-label.down {{ dominant-baseline: hanging; }}",
        line_color = theme.line_color,
        main_bkg = theme.main_bkg,
        font_family = theme.font_family,
        text_color = theme.text_color
    )
}
