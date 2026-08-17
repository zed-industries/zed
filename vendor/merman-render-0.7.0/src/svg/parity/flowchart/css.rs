//! Flowchart CSS generation.

use super::*;

pub(in crate::svg::parity) fn flowchart_css(
    diagram_id: &str,
    effective_config: &serde_json::Value,
    font_family: &str,
    font_size: f64,
    class_defs: &IndexMap<String, Vec<String>>,
) -> String {
    let id = escape_xml(diagram_id);
    let theme = PresentationTheme::new(effective_config).node_diagram();
    let stroke = theme.common.line_color.as_str();
    let arrowhead_color = theme.arrowhead_color.as_str();
    let node_border = theme.node_border.as_str();
    let main_bkg = theme.main_bkg.as_str();
    let text_color = theme.common.text_color.as_str();
    let node_text_color = theme.node_text_color.as_str();
    let title_color = theme.title_color.as_str();
    let stroke_width = theme.stroke_width.as_str();
    let error_bkg = theme.common.error_bkg.as_str();
    let error_text = theme.common.error_text.as_str();
    let edge_label_background = theme.edge_label_background.as_str();
    let tertiary = theme.tertiary.as_str();
    let cluster_bkg = theme.cluster_bkg.as_str();
    let cluster_border = theme.cluster_border.as_str();

    fn flowchart_label_bkg_from_edge_label_background(edge_label_background: &str) -> String {
        fn parse_hex_channel(hex: &str) -> Option<u8> {
            u8::from_str_radix(hex, 16).ok()
        }

        fn parse_hex_rgb(s: &str) -> Option<(f64, f64, f64)> {
            let s = s.trim();
            let hex = s.strip_prefix('#')?;
            match hex.len() {
                3 => {
                    let r = parse_hex_channel(&hex[0..1].repeat(2))? as f64;
                    let g = parse_hex_channel(&hex[1..2].repeat(2))? as f64;
                    let b = parse_hex_channel(&hex[2..3].repeat(2))? as f64;
                    Some((r, g, b))
                }
                6 => {
                    let r = parse_hex_channel(&hex[0..2])? as f64;
                    let g = parse_hex_channel(&hex[2..4])? as f64;
                    let b = parse_hex_channel(&hex[4..6])? as f64;
                    Some((r, g, b))
                }
                _ => None,
            }
        }

        fn parse_csv_f64(s: &str) -> Option<Vec<f64>> {
            let mut out = Vec::new();
            for p in s.split(',') {
                let p = p.trim();
                if p.is_empty() {
                    return None;
                }
                out.push(p.parse::<f64>().ok()?);
            }
            Some(out)
        }

        fn parse_rgb_like(s: &str, prefix: &str) -> Option<(f64, f64, f64)> {
            let inner = s.trim().strip_prefix(prefix)?.strip_suffix(')')?;
            let parts = parse_csv_f64(inner)?;
            if parts.len() < 3 {
                return None;
            }
            Some((parts[0], parts[1], parts[2]))
        }

        fn parse_named_color(s: &str) -> Option<(f64, f64, f64)> {
            match s.trim().to_ascii_lowercase().as_str() {
                "black" => Some((0.0, 0.0, 0.0)),
                "white" => Some((255.0, 255.0, 255.0)),
                _ => None,
            }
        }

        fn parse_hsl_to_rgb(s: &str) -> Option<(f64, f64, f64)> {
            let inner = s.trim().strip_prefix("hsl(")?.strip_suffix(')')?;
            let mut parts = inner.split(',').map(|p| p.trim());
            let h = parts.next()?.parse::<f64>().ok()?;
            let s = parts
                .next()?
                .strip_suffix('%')?
                .trim()
                .parse::<f64>()
                .ok()?;
            let l = parts
                .next()?
                .strip_suffix('%')?
                .trim()
                .parse::<f64>()
                .ok()?;

            let h = (h / 360.0) % 1.0;
            let s = (s / 100.0).clamp(0.0, 1.0);
            let l = (l / 100.0).clamp(0.0, 1.0);

            if s == 0.0 {
                let v = (l * 255.0).round();
                return Some((v, v, v));
            }

            fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
                if t < 0.0 {
                    t += 1.0;
                }
                if t > 1.0 {
                    t -= 1.0;
                }
                if t < 1.0 / 6.0 {
                    return p + (q - p) * 6.0 * t;
                }
                if t < 1.0 / 2.0 {
                    return q;
                }
                if t < 2.0 / 3.0 {
                    return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
                }
                p
            }

            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            let r = hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0;
            let g = hue_to_rgb(p, q, h) * 255.0;
            let b = hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0;
            Some((r, g, b))
        }

        let rgb = parse_hex_rgb(edge_label_background)
            .or_else(|| parse_rgb_like(edge_label_background, "rgb("))
            .or_else(|| parse_rgb_like(edge_label_background, "rgba("))
            .or_else(|| parse_hsl_to_rgb(edge_label_background))
            .or_else(|| parse_named_color(edge_label_background));

        let (r, g, b) = rgb.unwrap_or((232.0, 232.0, 232.0));
        let r = r.round().clamp(0.0, 255.0) as i64;
        let g = g.round().clamp(0.0, 255.0) as i64;
        let b = b.round().clamp(0.0, 255.0) as i64;
        format!("rgba({r}, {g}, {b}, 0.5)")
    }

    let label_bkg = flowchart_label_bkg_from_edge_label_background(&edge_label_background);

    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
        id.as_str(),
        font_family,
        fmt(font_size),
        text_color
    );
    out.push_str(
        r#"@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}"#,
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id.as_str(),
        id.as_str()
    );
    let _ = write!(
        &mut out,
        r#"#{} .error-icon{{fill:{};}}#{} .error-text{{fill:{};stroke:{};}}"#,
        id.as_str(),
        error_bkg,
        id.as_str(),
        error_text,
        error_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-thickness-normal{{stroke-width:{}px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id.as_str(),
        stroke_width,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str()
    );
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:{};stroke:{};}}#{} .marker.cross{{stroke:{};}}"#,
        id.as_str(),
        stroke,
        stroke,
        id.as_str(),
        stroke
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{}px;}}#{} p{{margin:0;}}#{} .label{{font-family:{};color:{};}}"#,
        id.as_str(),
        font_family,
        fmt(font_size),
        id.as_str(),
        id.as_str(),
        font_family,
        node_text_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .cluster-label text{{fill:{};}}#{} .cluster-label span{{color:{};}}#{} .cluster-label span p{{background-color:transparent;}}#{} .label text,#{} span{{fill:{};color:{};}}"#,
        id.as_str(),
        title_color,
        id.as_str(),
        title_color,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        node_text_color,
        node_text_color
    );
    let _ = write!(
        &mut out,
        r#"#{id} .node rect,#{id} .node circle,#{id} .node ellipse,#{id} .node polygon,#{id} .node path{{fill:{main_bkg};stroke:{node_border};stroke-width:{stroke_width}px;}}#{id} .rough-node .label text,#{id} .node .label text,#{id} .image-shape .label,#{id} .icon-shape .label{{text-anchor:middle;}}#{id} .node .katex path{{fill:#000;stroke:#000;stroke-width:1px;}}#{id} .rough-node .label,#{id} .node .label,#{id} .image-shape .label,#{id} .icon-shape .label{{text-align:center;}}#{id} .node.clickable{{cursor:pointer;}}"#
    );
    let _ = write!(
        &mut out,
        r#"#{} .root .anchor path{{fill:{}!important;stroke-width:0;stroke:{};}}#{} .arrowheadPath{{fill:{};}}#{} .edgePath .path{{stroke:{};stroke-width:{}px;}}#{} .flowchart-link{{stroke:{};fill:none;}}"#,
        id.as_str(),
        stroke,
        stroke,
        id.as_str(),
        arrowhead_color,
        id.as_str(),
        stroke,
        stroke_width,
        id.as_str(),
        stroke
    );
    let _ = write!(
        &mut out,
        r#"#{} .edgeLabel{{background-color:{};text-align:center;}}#{} .edgeLabel p{{background-color:{};}}#{} .edgeLabel rect{{opacity:0.5;background-color:{};fill:{};}}#{} .labelBkg{{background-color:{};}}"#,
        id.as_str(),
        edge_label_background,
        id.as_str(),
        edge_label_background,
        id.as_str(),
        edge_label_background,
        edge_label_background,
        id.as_str(),
        label_bkg
    );
    let _ = write!(
        &mut out,
        r#"#{} .cluster rect{{fill:{};stroke:{};stroke-width:1px;}}#{} .cluster text{{fill:{};}}#{} .cluster span{{color:{};}}#{} div.mermaidTooltip{{position:absolute;text-align:center;max-width:200px;padding:2px;font-family:{};font-size:12px;background:{};border:1px solid {};border-radius:2px;pointer-events:none;z-index:100;}}#{} .flowchartTitleText{{text-anchor:middle;font-size:18px;fill:{};}}#{} rect.text{{fill:none;stroke-width:0;}}"#,
        escape_xml(diagram_id),
        cluster_bkg,
        cluster_border,
        escape_xml(diagram_id),
        title_color,
        escape_xml(diagram_id),
        title_color,
        escape_xml(diagram_id),
        font_family,
        tertiary,
        cluster_border,
        escape_xml(diagram_id),
        text_color,
        escape_xml(diagram_id)
    );
    let _ = write!(
        &mut out,
        r#"#{} .icon-shape,#{} .image-shape{{background-color:{};text-align:center;}}#{} .icon-shape p,#{} .image-shape p{{background-color:{};padding:2px;}}#{} .icon-shape .label rect,#{} .image-shape .label rect{{opacity:0.5;background-color:{};fill:{};}}#{} .label-icon{{display:inline-block;height:1em;overflow:visible;vertical-align:-0.125em;}}#{} .node .label-icon path{{fill:currentColor;stroke:revert;stroke-width:revert;}}#{} :root{{--mermaid-font-family:{};}}"#,
        id.as_str(),
        id.as_str(),
        edge_label_background,
        id.as_str(),
        id.as_str(),
        edge_label_background,
        id.as_str(),
        id.as_str(),
        edge_label_background,
        edge_label_background,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        font_family
    );

    // Mermaid `createCssStyles(...)` chooses different selectors based on `htmlLabels`.
    // - HTML labels: `.classDef > *` + `.classDef span`
    // - SVG labels: `.classDef rect|polygon|ellipse|circle|path`
    let html_labels = crate::flowchart::flowchart_effective_html_labels(effective_config);
    let shape_elements: &[&str] = &["rect", "polygon", "ellipse", "circle", "path"];

    for (class, decls) in class_defs {
        if decls.is_empty() {
            continue;
        }
        let mut style = String::new();
        let mut text_color: Option<String> = None;
        for d in decls {
            let Some((k, v)) = parse_style_decl(d) else {
                continue;
            };
            let _ = write!(&mut style, "{}:{}!important;", k, v);
            if k == "color" {
                text_color = Some(v.to_string());
            }
        }
        if style.is_empty() {
            continue;
        }
        if html_labels {
            // Mermaid (via Stylis) ends up serializing the `>` combinator inside `<style>` as
            // `&gt;` in the final SVG string (see upstream baselines).
            let _ = write!(
                &mut out,
                r#"#{} .{}&gt;*{{{}}}#{} .{} span{{{}}}"#,
                id.as_str(),
                escape_xml(class),
                style,
                id.as_str(),
                escape_xml(class),
                style
            );
        } else {
            for css_element in shape_elements {
                let _ = write!(
                    &mut out,
                    r#"#{} .{} {}{{{}}}"#,
                    id.as_str(),
                    escape_xml(class),
                    css_element,
                    style
                );
            }
        }
        if let Some(c) = text_color.as_deref() {
            let _ = write!(
                &mut out,
                r#"#{} .{} tspan{{fill:{}!important;}}"#,
                id.as_str(),
                escape_xml(class),
                escape_xml(c)
            );
        }
    }

    out
}

#[inline]
pub(super) fn write_flowchart_edge_class_attr(out: &mut String, edge: &crate::flowchart::FlowEdge) {
    // Mermaid includes a 2-part class tuple (thickness/pattern) for flowchart edge paths. The
    // second tuple is `edge-thickness-normal edge-pattern-solid` in Mermaid@11.12.2 baselines,
    // even for dotted/thick strokes.
    let (thickness_1, pattern_1) = match edge.stroke.as_deref() {
        Some("thick") => ("edge-thickness-thick", "edge-pattern-solid"),
        Some("invisible") => ("edge-thickness-invisible", "edge-pattern-solid"),
        Some("dotted") => ("edge-thickness-normal", "edge-pattern-dotted"),
        _ => ("edge-thickness-normal", "edge-pattern-solid"),
    };

    if thickness_1 == "edge-thickness-invisible" {
        // Mermaid@11.12.2 does *not* include the second tuple nor `flowchart-link` for invisible
        // edges.
        out.push_str(thickness_1);
        out.push(' ');
        out.push_str(pattern_1);
        return;
    }

    out.push_str(thickness_1);
    out.push(' ');
    out.push_str(pattern_1);
    out.push_str(" edge-thickness-normal edge-pattern-solid flowchart-link");

    // Mermaid attaches animation classes directly on the edge path element when enabled via
    // edge-id `@{ ... }` blocks (e.g. `e1@{ animate: true }` or `e1@{ animation: fast }`).
    if edge.animate == Some(false) {
        return;
    }
    let animation_class = match edge.animation.as_deref() {
        Some("slow") => Some("edge-animation-slow"),
        Some(_) => Some("edge-animation-fast"),
        None => match edge.animate {
            Some(true) => Some("edge-animation-fast"),
            _ => None,
        },
    };
    if let Some(cls) = animation_class {
        out.push(' ');
        out.push_str(cls);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn neutral_named_white_edge_label_background_fades_to_white() {
        let css = flowchart_css(
            "theme_neutral",
            &json!({
                "themeVariables": {
                    "edgeLabelBackground": "white"
                }
            }),
            "\"trebuchet ms\",verdana,arial,sans-serif",
            16.0,
            &IndexMap::new(),
        );

        assert!(
            css.contains("#theme_neutral .labelBkg{background-color:rgba(255, 255, 255, 0.5);}")
        );
    }

    #[test]
    fn unknown_edge_label_background_keeps_mermaid_default_fade() {
        let css = flowchart_css(
            "theme_unknown_color",
            &json!({
                "themeVariables": {
                    "edgeLabelBackground": "not-a-css-color"
                }
            }),
            "\"trebuchet ms\",verdana,arial,sans-serif",
            16.0,
            &IndexMap::new(),
        );

        assert!(css.contains(
            "#theme_unknown_color .labelBkg{background-color:rgba(232, 232, 232, 0.5);}"
        ));
    }
}
