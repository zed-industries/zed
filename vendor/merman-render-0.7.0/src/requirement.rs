use crate::config::config_f64;
use crate::json::from_value_ref;
use crate::model::{
    Bounds, LayoutEdge, LayoutLabel, LayoutNode, LayoutPoint, RequirementDiagramLayout,
};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use crate::{Error, Result};
use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, RankDir};
use merman_core::diagrams::requirement::RequirementDiagramRenderModel;
use serde_json::Value;

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

#[derive(Debug, Clone)]
struct RequirementLabelMetrics {
    width: f64,
    height: f64,
}

fn requirement_label_uses_markdown_html(raw: &str) -> bool {
    let lower = raw.to_ascii_lowercase();
    raw.contains('*') || raw.contains('_') || raw.contains('\n') || lower.contains("<br")
}

pub(crate) fn requirement_styles_force_bold(css_styles: &[String]) -> bool {
    css_styles.iter().any(|raw| {
        let s = raw.trim().trim_end_matches(';');
        let Some((key, value)) = s.split_once(':') else {
            return false;
        };
        if !key.trim().eq_ignore_ascii_case("font-weight") {
            return false;
        }
        let value = value
            .split_once("!important")
            .map(|(v, _)| v)
            .unwrap_or(value)
            .trim()
            .to_ascii_lowercase();
        value == "bold"
            || value == "bolder"
            || value
                .parse::<u16>()
                .map(|weight| weight >= 600)
                .unwrap_or(false)
    })
}

fn measure_requirement_label_metrics(
    measurer: &dyn TextMeasurer,
    html_style: &TextStyle,
    display_text: &str,
    bold: bool,
) -> Option<RequirementLabelMetrics> {
    if display_text.trim().is_empty() {
        return None;
    }

    let font_size = html_style.font_size.max(1.0);
    let looks_like_markdown_inline = requirement_label_uses_markdown_html(display_text);
    let measured = if looks_like_markdown_inline {
        crate::text::measure_markdown_with_flowchart_bold_deltas(
            measurer,
            display_text,
            html_style,
            None,
            WrapMode::HtmlLike,
        )
    } else {
        measurer.measure_wrapped(display_text, html_style, None, WrapMode::HtmlLike)
    };
    let height = measured.height.max(1.0);
    let width = if let Some(em) =
        crate::generated::requirement_text_overrides_11_12_2::lookup_requirement_html_label_width_em(
            display_text,
            bold,
        ) {
        (em * font_size).max(1.0)
    } else {
        measured.width.max(1.0)
    };

    Some(RequirementLabelMetrics { width, height })
}

#[derive(Debug, Clone)]
struct RequirementBoxLayout {
    width: f64,
    height: f64,
}

fn requirement_box_layout(
    measurer: &dyn TextMeasurer,
    html_style_regular: &TextStyle,
    html_style_bold: &TextStyle,
    lines: &[(String, bool)],
    gap: f64,
    padding: f64,
) -> RequirementBoxLayout {
    // Mirrors Mermaid `requirementBox.ts` label stacking and bbox-based sizing.
    let mut html_metrics: Vec<Option<RequirementLabelMetrics>> = Vec::with_capacity(lines.len());
    let mut max_w: f64 = 0.0;

    // First pass: label bbox widths/heights (HTML).
    for (display, bold) in lines {
        let html_style = if *bold {
            html_style_bold
        } else {
            html_style_regular
        };
        let m = measure_requirement_label_metrics(measurer, html_style, display, *bold);
        if let Some(m) = &m {
            max_w = max_w.max(m.width);
        }
        html_metrics.push(m);
    }

    let total_w = max_w + padding;

    // Second pass: vertical extents.
    let mut min_y = 0.0;
    let mut max_y = 0.0;
    let mut y_offset = 0.0;

    for (idx, m) in html_metrics.iter().enumerate() {
        let Some(m) = m else {
            continue;
        };

        if idx == 0 {
            min_y = -m.height / 2.0;
            max_y = m.height / 2.0;
            y_offset = m.height;
            continue;
        }
        if idx == 1 {
            let top = -m.height / 2.0 + y_offset;
            let bottom = m.height / 2.0 + y_offset;
            min_y = min_y.min(top);
            max_y = max_y.max(bottom);
            y_offset += m.height + gap;
            continue;
        }

        let top = -m.height / 2.0 + y_offset;
        let bottom = m.height / 2.0 + y_offset;
        min_y = min_y.min(top);
        max_y = max_y.max(bottom);
        y_offset += m.height;
    }

    let bbox_h = (max_y - min_y).max(1.0);
    let total_h = bbox_h + padding;

    RequirementBoxLayout {
        width: total_w.max(1.0),
        height: total_h.max(1.0),
    }
}

fn requirement_edge_id(src: &str, dst: &str, idx: usize) -> String {
    format!("{src}-{dst}-{idx}")
}

fn prefixed_nonempty_line(prefix: &str, value: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        String::new()
    } else {
        format!("{prefix}{value}")
    }
}

pub fn layout_requirement_diagram(
    model: &Value,
    effective_config: &Value,
    text_measurer: &dyn TextMeasurer,
) -> Result<RequirementDiagramLayout> {
    let model: RequirementDiagramRenderModel = from_value_ref(model)?;
    layout_requirement_diagram_typed(&model, effective_config, text_measurer)
}

pub fn layout_requirement_diagram_typed(
    model: &RequirementDiagramRenderModel,
    effective_config: &Value,
    text_measurer: &dyn TextMeasurer,
) -> Result<RequirementDiagramLayout> {
    let direction = if model.direction.trim().is_empty() {
        normalize_dir("TB")
    } else {
        normalize_dir(&model.direction)
    };

    let nodesep = config_f64(effective_config, &["nodeSpacing"])
        .or_else(|| config_f64(effective_config, &["flowchart", "nodeSpacing"]))
        .unwrap_or(50.0);
    let ranksep = config_f64(effective_config, &["rankSpacing"])
        .or_else(|| config_f64(effective_config, &["flowchart", "rankSpacing"]))
        .unwrap_or(50.0);

    let font_family = Some(crate::config::config_font_family_or_first_array_css(
        effective_config,
    ));
    let font_size = crate::config::config_theme_or_root_font_size_px(effective_config, 16.0);
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

    let padding = 20.0;
    let gap = 20.0;

    let mut g = Graph::<NodeLabel, EdgeLabel, GraphLabel>::new(GraphOptions {
        directed: true,
        multigraph: true,
        compound: true,
    });
    g.set_graph(GraphLabel {
        rankdir: rank_dir_from(&direction),
        nodesep,
        ranksep,
        marginx: 8.0,
        marginy: 8.0,
        ..Default::default()
    });

    for r in &model.requirements {
        // Mermaid's underlying graph data structures historically used plain JS objects in a few
        // places. The `__proto__` id can still trigger prototype pollution safeguards, effectively
        // dropping the node from the rendered graph. Mirror the upstream SVG baselines.
        if r.name == "__proto__" {
            continue;
        }

        let type_disp = format!("<<{}>>", r.node_type);
        let style_bold = requirement_styles_force_bold(&r.css_styles);
        let mut lines: Vec<(String, bool)> = Vec::new();
        lines.push((type_disp, style_bold));
        lines.push((r.name.clone(), true));

        let id_line = prefixed_nonempty_line("ID: ", &r.requirement_id);
        lines.push((id_line, style_bold));

        let text_line = prefixed_nonempty_line("Text: ", &r.text);
        lines.push((text_line, style_bold));

        let risk_line = prefixed_nonempty_line("Risk: ", &r.risk);
        lines.push((risk_line, style_bold));

        let verify_line = prefixed_nonempty_line("Verification: ", &r.verify_method);
        lines.push((verify_line, style_bold));

        let box_layout = requirement_box_layout(
            text_measurer,
            &html_style_regular,
            &html_style_bold,
            &lines,
            gap,
            padding,
        );
        g.set_node(
            r.name.clone(),
            NodeLabel {
                width: box_layout.width,
                height: box_layout.height,
                ..Default::default()
            },
        );
    }

    for e in &model.elements {
        if e.name == "__proto__" {
            continue;
        }

        let type_disp = "<<Element>>".to_string();
        let style_bold = requirement_styles_force_bold(&e.css_styles);
        let mut lines: Vec<(String, bool)> = Vec::new();
        lines.push((type_disp, style_bold));
        lines.push((e.name.clone(), true));

        let type_line = e.element_type.trim().to_string();
        let type_line = if type_line.is_empty() {
            String::new()
        } else {
            format!("Type: {type_line}")
        };
        lines.push((type_line, style_bold));

        let doc_line = prefixed_nonempty_line("Doc Ref: ", &e.doc_ref);
        lines.push((doc_line, style_bold));

        let box_layout = requirement_box_layout(
            text_measurer,
            &html_style_regular,
            &html_style_bold,
            &lines,
            gap,
            padding,
        );
        g.set_node(
            e.name.clone(),
            NodeLabel {
                width: box_layout.width,
                height: box_layout.height,
                ..Default::default()
            },
        );
    }

    for rel in &model.relationships {
        if !g.has_node(&rel.src) {
            return Err(Error::InvalidModel {
                message: format!("relationship src node not found: {}", rel.src),
            });
        }
        if !g.has_node(&rel.dst) {
            return Err(Error::InvalidModel {
                message: format!("relationship dst node not found: {}", rel.dst),
            });
        }

        // Mermaid's requirement diagram edge ids currently collide for multiple relationships
        // between the same nodes (the upstream DB resets the counter for every relation).
        // Downstream graphlib layout will overwrite earlier edges; only the last relation survives.
        // Mirror this behavior to match the upstream SVG baselines.
        let edge_id = requirement_edge_id(&rel.src, &rel.dst, 0);

        let label_display = format!("<<{}>>", rel.rel_type);
        let metrics = measure_requirement_label_metrics(
            text_measurer,
            &html_style_regular,
            &label_display,
            false,
        )
        .unwrap_or(RequirementLabelMetrics {
            width: 0.0,
            height: 0.0,
        });

        let el = EdgeLabel {
            width: metrics.width.max(0.0),
            height: metrics.height.max(0.0),
            labelpos: LabelPos::C,
            // Dagre defaults to 10 when unspecified.
            labeloffset: 10.0,
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        };
        g.set_edge_named(rel.src.clone(), rel.dst.clone(), Some(edge_id), Some(el));
    }

    dugong::layout_dagreish(&mut g);

    let mut out_nodes: Vec<LayoutNode> = Vec::new();
    for v in g.nodes() {
        let Some(n) = g.node(v) else {
            continue;
        };
        let (Some(cx), Some(cy)) = (n.x, n.y) else {
            continue;
        };
        out_nodes.push(LayoutNode {
            id: v.to_string(),
            x: cx - n.width / 2.0,
            y: cy - n.height / 2.0,
            width: n.width,
            height: n.height,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });
    }

    let mut out_edges: Vec<LayoutEdge> = Vec::new();
    for ek in g.edge_keys() {
        let Some(e) = g.edge_by_key(&ek) else {
            continue;
        };

        let points = e
            .points
            .iter()
            .map(|p| LayoutPoint { x: p.x, y: p.y })
            .collect::<Vec<_>>();

        let label = match (e.x, e.y) {
            (Some(x), Some(y)) if e.width > 0.0 && e.height > 0.0 => Some(LayoutLabel {
                x,
                y,
                width: e.width,
                height: e.height,
            }),
            _ => None,
        };

        out_edges.push(LayoutEdge {
            id: ek
                .name
                .clone()
                .unwrap_or_else(|| format!("{}-{}", ek.v, ek.w)),
            from: ek.v.clone(),
            to: ek.w.clone(),
            from_cluster: None,
            to_cluster: None,
            points,
            label,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: None,
            end_marker: None,
            stroke_dasharray: None,
        });
    }

    fn bounds_for_nodes_edges(nodes: &[LayoutNode], edges: &[LayoutEdge]) -> Option<Bounds> {
        if nodes.is_empty() && edges.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for n in nodes {
            min_x = min_x.min(n.x);
            min_y = min_y.min(n.y);
            max_x = max_x.max(n.x + n.width);
            max_y = max_y.max(n.y + n.height);
        }
        for e in edges {
            for p in &e.points {
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                max_x = max_x.max(p.x);
                max_y = max_y.max(p.y);
            }
            if let Some(l) = &e.label {
                min_x = min_x.min(l.x - l.width / 2.0);
                max_x = max_x.max(l.x + l.width / 2.0);
                min_y = min_y.min(l.y - l.height / 2.0);
                max_y = max_y.max(l.y + l.height / 2.0);
            }
        }

        if !min_x.is_finite() || !min_y.is_finite() || !max_x.is_finite() || !max_y.is_finite() {
            return None;
        }
        Some(Bounds {
            min_x,
            min_y,
            max_x,
            max_y,
        })
    }

    let bounds = bounds_for_nodes_edges(&out_nodes, &out_edges);

    Ok(RequirementDiagramLayout {
        nodes: out_nodes,
        edges: out_edges,
        bounds,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn requirement_styles_detect_bold_font_weight() {
        assert!(super::requirement_styles_force_bold(&[
            "fill:#f9f".to_string(),
            " font-weight:bold".to_string(),
        ]));
        assert!(super::requirement_styles_force_bold(&[
            "font-weight: 700 !important".to_string(),
        ]));
        assert!(!super::requirement_styles_force_bold(&[
            "font-weight: normal".to_string(),
            "stroke:blue".to_string(),
        ]));
    }
}
