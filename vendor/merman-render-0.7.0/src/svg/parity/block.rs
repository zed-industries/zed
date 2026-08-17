use super::*;
use crate::block::{
    BlockArrowPoint as ArrowPoint, block_arrow_points, block_label_is_effectively_empty,
};
use crate::model::LayoutPoint;

// Block diagram SVG renderer implementation (split from parity.rs).

pub(super) fn render_block_diagram_svg(
    layout: &BlockDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model = crate::block::block_model_from_semantic(semantic)?;
    render_block_diagram_svg_model(layout, &model, effective_config, options)
}

pub(super) fn render_block_diagram_svg_model(
    layout: &BlockDiagramLayout,
    model: &merman_core::diagrams::block::BlockDiagramRenderModel,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    fn decode_block_label_html(raw: &str) -> String {
        // Mermaid's block diagram labels are rendered via an HTML foreignObject label helper,
        // which decodes HTML entities (notably `&nbsp;`).
        raw.replace("&nbsp;", "\u{00A0}")
    }

    #[derive(Clone)]
    struct RenderNode {
        label: String,
        block_type: String,
        classes: Vec<String>,
        styles: Vec<String>,
        directions: Vec<String>,
    }

    fn collect_nodes(
        root: &crate::block::BlockNode,
        out: &mut std::collections::HashMap<String, RenderNode>,
    ) {
        let mut stack = vec![root];
        while let Some(n) = stack.pop() {
            if let Some(existing) = out.get_mut(&n.id) {
                if !n.label.is_empty() {
                    existing.label = n.label.clone();
                }
                if !n.block_type.is_empty() && n.block_type != "na" {
                    existing.block_type = n.block_type.clone();
                }
                if !n.classes.is_empty() {
                    existing.classes = n.classes.clone();
                }
                if !n.styles.is_empty() {
                    existing.styles = n.styles.clone();
                }
                if !n.directions.is_empty() {
                    existing.directions = n.directions.clone();
                }
            } else {
                out.insert(
                    n.id.clone(),
                    RenderNode {
                        label: n.label.clone(),
                        block_type: n.block_type.clone(),
                        classes: n.classes.clone(),
                        styles: n.styles.clone(),
                        directions: n.directions.clone(),
                    },
                );
            }
            for child in n.children.iter().rev() {
                stack.push(child);
            }
        }
    }

    let node_padding = config_f64(effective_config, &["block", "padding"]).unwrap_or(8.0);
    let mut nodes_by_id: std::collections::HashMap<String, RenderNode> =
        std::collections::HashMap::new();
    for n in &model.blocks_flat {
        collect_nodes(n, &mut nodes_by_id);
    }
    let layout_nodes_by_id: std::collections::HashMap<String, LayoutNode> = layout
        .nodes
        .iter()
        .cloned()
        .map(|n| (n.id.clone(), n))
        .collect();

    fn marker_id(diagram_id: &str, marker: &str) -> String {
        format!("{diagram_id}_block-{marker}")
    }

    fn marker_url(diagram_id: &str, marker: &str) -> String {
        format!("url(#{})", marker_id(diagram_id, marker))
    }

    fn dom_id(diagram_id: &str, raw_id: &str) -> String {
        if diagram_id.is_empty() {
            raw_id.to_string()
        } else {
            format!("{diagram_id}-{raw_id}")
        }
    }

    fn edge_marker_end(arrow: Option<&str>) -> Option<&'static str> {
        match arrow.unwrap_or("").trim() {
            "arrow_point" => Some("pointEnd"),
            "arrow_circle" => Some("circleEnd"),
            "arrow_cross" => Some("crossEnd"),
            "arrow_open" | "" => None,
            _ => Some("pointEnd"),
        }
    }

    fn edge_marker_start(arrow: Option<&str>) -> Option<&'static str> {
        match arrow.unwrap_or("").trim() {
            "arrow_point" => Some("pointStart"),
            "arrow_circle" => Some("circleStart"),
            "arrow_cross" => Some("crossStart"),
            "arrow_open" | "" => None,
            _ => None,
        }
    }

    fn parse_hex_rgb_u8(v: &str) -> Option<(u8, u8, u8)> {
        let v = v.trim();
        let hex = v.strip_prefix('#')?;
        match hex.len() {
            6 => Some((
                u8::from_str_radix(&hex[0..2], 16).ok()?,
                u8::from_str_radix(&hex[2..4], 16).ok()?,
                u8::from_str_radix(&hex[4..6], 16).ok()?,
            )),
            3 => Some((
                u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?,
                u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?,
                u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?,
            )),
            _ => None,
        }
    }

    fn push_ordered_decl(out: &mut Vec<(String, String)>, key: &str, raw: &str) {
        if let Some((_, value)) = out.iter_mut().find(|(existing, _)| existing == key) {
            *value = raw.to_string();
            return;
        }
        out.push((key.to_string(), raw.to_string()));
    }

    fn compile_block_inline_styles(styles: &[String]) -> (String, String, String) {
        let mut box_decls: Vec<(String, String)> = Vec::new();
        let mut text_decls: Vec<(String, String)> = Vec::new();

        for raw in styles {
            let trimmed = raw.trim().trim_end_matches(';').trim();
            if trimmed.is_empty() {
                continue;
            }
            let Some((key, value)) = parse_style_decl(trimmed) else {
                let decoded = decode_mermaid_entities_for_render_text(trimmed);
                let decoded = decoded.as_ref().trim();
                if !decoded.is_empty() {
                    push_ordered_decl(&mut box_decls, decoded, decoded);
                }
                continue;
            };
            if is_rect_style_key(key) {
                push_ordered_decl(&mut box_decls, key, trimmed);
            }
            if is_text_style_key(key) {
                let _ = value;
                push_ordered_decl(&mut text_decls, key, trimmed);
            }
        }

        let style_attr = |decls: &[(String, String)]| -> String {
            let mut out = String::new();
            for (_, raw) in decls {
                out.push_str(raw);
                out.push(';');
            }
            out
        };

        let mut div_prefix = String::new();
        for (key, raw) in &text_decls {
            if key == "color" {
                let value = raw.split_once(':').map(|(_, v)| v.trim()).unwrap_or("");
                if let Some((r, g, b)) = parse_hex_rgb_u8(value) {
                    let _ = write!(&mut div_prefix, "color: rgb({r}, {g}, {b}); ");
                } else if !value.is_empty() {
                    let _ = write!(&mut div_prefix, "color: {}; ", value.to_ascii_lowercase());
                }
            } else {
                div_prefix.push_str(raw);
                div_prefix.push_str("; ");
            }
        }

        (style_attr(&box_decls), style_attr(&text_decls), div_prefix)
    }

    fn block_edge_start_marker_inset(arrow: Option<&str>) -> f64 {
        match arrow.unwrap_or("").trim() {
            "arrow_point" => 4.5,
            _ => 0.0,
        }
    }

    fn block_edge_end_marker_inset(arrow: Option<&str>) -> f64 {
        match arrow.unwrap_or("").trim() {
            "arrow_point" => 4.0,
            _ => 0.0,
        }
    }

    fn move_point_towards(point: &LayoutPoint, target: &LayoutPoint, distance: f64) -> LayoutPoint {
        if distance.abs() <= 1e-12 {
            return point.clone();
        }
        let dx = target.x - point.x;
        let dy = target.y - point.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len <= 1e-12 {
            return point.clone();
        }
        LayoutPoint {
            x: point.x + dx / len * distance,
            y: point.y + dy / len * distance,
        }
    }

    fn block_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
        let id = escape_xml(diagram_id);
        let theme = PresentationTheme::new(effective_config).node_diagram();
        let font_family = theme.common.font_family_css.as_str();
        let font_size = theme.common.font_size_px;
        let text_color = theme.common.text_color.as_str();
        let node_text_color = theme.node_text_color.as_str();
        let title_color = theme.title_color.as_str();
        let main_bkg = theme.main_bkg.as_str();
        let node_border = theme.node_border.as_str();
        let line_color = theme.common.line_color.as_str();
        let arrowhead_color = theme.arrowhead_color.as_str();
        let stroke_width = theme.stroke_width.as_str();
        let edge_label_background = theme.edge_label_background.as_str();
        let cluster_bkg = theme.cluster_bkg.as_str();
        let cluster_border = theme.cluster_border.as_str();
        let cluster_bkg =
            css_rgba_fade(&cluster_bkg, 0.5).unwrap_or_else(|| cluster_bkg.to_string());
        let cluster_border =
            css_rgba_fade(&cluster_border, 0.2).unwrap_or_else(|| cluster_border.to_string());

        let mut out = String::new();
        let _ = write!(
            &mut out,
            r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
            id.as_str(),
            font_family,
            fmt(font_size),
            node_text_color
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
            r#"#{} .label{{font-family:{};color:{};}}#{} p{{margin:0;}}#{} .label text,#{} span,#{} p{{fill:{};color:{};}}"#,
            id.as_str(),
            font_family,
            node_text_color,
            id.as_str(),
            id.as_str(),
            id.as_str(),
            id.as_str(),
            node_text_color,
            node_text_color
        );
        let _ = write!(
            &mut out,
            r#"#{} .cluster-label text{{fill:{};}}#{} .cluster-label span,#{} .cluster-label p{{color:{};}}"#,
            id.as_str(),
            title_color,
            id.as_str(),
            id.as_str(),
            title_color
        );
        let _ = write!(
            &mut out,
            r#"#{} .node rect,#{} .node circle,#{} .node ellipse,#{} .node polygon,#{} .node path{{fill:{};stroke:{};stroke-width:1px;}}#{} .flowchart-label text{{text-anchor:middle;}}#{} .node .label{{text-align:center;}}#{} .node.clickable{{cursor:pointer;}}"#,
            id.as_str(),
            id.as_str(),
            id.as_str(),
            id.as_str(),
            id.as_str(),
            main_bkg,
            node_border,
            id.as_str(),
            id.as_str(),
            id.as_str()
        );
        let _ = write!(
            &mut out,
            r#"#{} .arrowheadPath,#{} .arrowMarkerPath{{fill:{};stroke:{};}}#{} .edgePath .path{{stroke:{};stroke-width:2.0px;}}#{} .flowchart-link{{stroke:{};fill:none;}}"#,
            id.as_str(),
            id.as_str(),
            arrowhead_color,
            line_color,
            id.as_str(),
            line_color,
            id.as_str(),
            line_color
        );
        let _ = write!(
            &mut out,
            r#"#{} .edgeLabel{{background-color:{};text-align:center;}}#{} .edgeLabel p{{margin:0;padding:0;display:inline;}}#{} .edgeLabel rect{{opacity:0.5;background-color:{};fill:{};}}#{} .labelBkg{{background-color:{}}}"#,
            id.as_str(),
            edge_label_background,
            id.as_str(),
            id.as_str(),
            edge_label_background,
            edge_label_background,
            id.as_str(),
            edge_label_background
        );
        let _ = write!(
            &mut out,
            r#"#{} .node .cluster{{fill:{};stroke:{};stroke-width:1px;}}#{} .cluster text{{fill:{};}}#{} .cluster span,#{} .cluster p{{color:{};}}#{} .flowchartTitleText{{text-anchor:middle;font-size:18px;fill:{};}}#{} :root{{--mermaid-font-family:{};}}"#,
            id.as_str(),
            cluster_bkg,
            cluster_border,
            id.as_str(),
            title_color,
            id.as_str(),
            id.as_str(),
            title_color,
            id.as_str(),
            text_color,
            id.as_str(),
            font_family
        );
        out
    }

    fn intersect_line(
        p1: &LayoutPoint,
        p2: &LayoutPoint,
        q1: &LayoutPoint,
        q2: &LayoutPoint,
    ) -> Option<LayoutPoint> {
        let a1 = p2.y - p1.y;
        let b1 = p1.x - p2.x;
        let c1 = p2.x * p1.y - p1.x * p2.y;

        let r3 = a1 * q1.x + b1 * q1.y + c1;
        let r4 = a1 * q2.x + b1 * q2.y + c1;
        if r3 != 0.0 && r4 != 0.0 && r3 * r4 > 0.0 {
            return None;
        }

        let a2 = q2.y - q1.y;
        let b2 = q1.x - q2.x;
        let c2 = q2.x * q1.y - q1.x * q2.y;

        let r1 = a2 * p1.x + b2 * p1.y + c2;
        let r2 = a2 * p2.x + b2 * p2.y + c2;
        if r1 != 0.0 && r2 != 0.0 && r1 * r2 > 0.0 {
            return None;
        }

        let denom = a1 * b2 - a2 * b1;
        if denom.abs() <= 1e-12 {
            return None;
        }

        let offset = (denom / 2.0).abs();

        let num_x = b1 * c2 - b2 * c1;
        let x = if num_x < 0.0 {
            (num_x - offset) / denom
        } else {
            (num_x + offset) / denom
        };

        let num_y = a2 * c1 - a1 * c2;
        let y = if num_y < 0.0 {
            (num_y - offset) / denom
        } else {
            (num_y + offset) / denom
        };

        Some(LayoutPoint { x, y })
    }

    fn intersect_rect(node: &LayoutNode, point: &LayoutPoint) -> LayoutPoint {
        let dx = point.x - node.x;
        let dy = point.y - node.y;
        let mut w = node.width / 2.0;
        let mut h = node.height / 2.0;

        let (sx, sy) = if dy.abs() * w > dx.abs() * h {
            if dy < 0.0 {
                h = -h;
            }
            let sx = if dy == 0.0 { 0.0 } else { (h * dx) / dy };
            (sx, h)
        } else {
            if dx < 0.0 {
                w = -w;
            }
            let sy = if dx == 0.0 { 0.0 } else { (w * dy) / dx };
            (w, sy)
        };

        LayoutPoint {
            x: node.x + sx,
            y: node.y + sy,
        }
    }

    fn intersect_circle(node: &LayoutNode, point: &LayoutPoint) -> LayoutPoint {
        let dx = point.x - node.x;
        let dy = point.y - node.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist <= 1e-12 {
            return LayoutPoint {
                x: node.x,
                y: node.y,
            };
        }
        let radius = (node.width.min(node.height) / 2.0).max(0.0);
        LayoutPoint {
            x: node.x + dx / dist * radius,
            y: node.y + dy / dist * radius,
        }
    }

    fn intersect_cylinder(node: &LayoutNode, point: &LayoutPoint) -> LayoutPoint {
        let mut pos = intersect_rect(node, point);
        let x = pos.x - node.x;

        let width = node.width.max(1.0);
        let rx = width / 2.0;
        let ry = rx / (2.5 + width / 50.0);

        if rx != 0.0
            && (x.abs() < width / 2.0
                || ((x.abs() - width / 2.0).abs() < 1e-12
                    && (pos.y - node.y).abs() > node.height / 2.0 - ry))
        {
            let mut y = ry * ry * (1.0 - (x * x) / (rx * rx));
            if y > 0.0 {
                y = y.sqrt();
            } else {
                y = 0.0;
            }
            y = ry - y;
            if point.y - node.y > 0.0 {
                y = -y;
            }
            pos.y += y;
        }

        pos
    }

    fn intersect_polygon(
        node: &LayoutNode,
        poly_points: &[ArrowPoint],
        point: &LayoutPoint,
    ) -> LayoutPoint {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        for entry in poly_points {
            min_x = min_x.min(entry.x);
            min_y = min_y.min(entry.y);
        }

        let left = node.x - node.width / 2.0 - min_x;
        let top = node.y - node.height / 2.0 - min_y;

        let mut intersections = Vec::new();
        for idx in 0..poly_points.len() {
            let p1 = &poly_points[idx];
            let p2 = &poly_points[(idx + 1) % poly_points.len()];
            let q1 = LayoutPoint {
                x: left + p1.x,
                y: top + p1.y,
            };
            let q2 = LayoutPoint {
                x: left + p2.x,
                y: top + p2.y,
            };
            if let Some(intersection) = intersect_line(
                &LayoutPoint {
                    x: node.x,
                    y: node.y,
                },
                point,
                &q1,
                &q2,
            ) {
                intersections.push(intersection);
            }
        }

        intersections
            .into_iter()
            .min_by(|p, q| {
                let p_dist = (p.x - point.x).powi(2) + (p.y - point.y).powi(2);
                let q_dist = (q.x - point.x).powi(2) + (q.y - point.y).powi(2);
                p_dist.total_cmp(&q_dist)
            })
            .unwrap_or(LayoutPoint {
                x: node.x,
                y: node.y,
            })
    }

    fn block_polygon_points(
        node: &LayoutNode,
        render_node: &RenderNode,
        node_padding: f64,
    ) -> Option<Vec<ArrowPoint>> {
        let bbox_w = node.label_width.unwrap_or(0.0).max(0.0);
        let bbox_h = node.label_height.unwrap_or(0.0).max(0.0);
        let rect_w = (bbox_w + node_padding).max(1.0);
        let rect_h = (bbox_h + node_padding).max(1.0);

        match render_node.block_type.as_str() {
            "diamond" => {
                let side = (rect_w + rect_h).max(1.0);
                Some(vec![
                    ArrowPoint {
                        x: side / 2.0,
                        y: 0.0,
                    },
                    ArrowPoint {
                        x: side,
                        y: -side / 2.0,
                    },
                    ArrowPoint {
                        x: side / 2.0,
                        y: -side,
                    },
                    ArrowPoint {
                        x: 0.0,
                        y: -side / 2.0,
                    },
                ])
            }
            "hexagon" => {
                let shoulder = rect_h / 4.0;
                let hex_w = (bbox_w + 2.0 * shoulder + node_padding).max(1.0);
                Some(vec![
                    ArrowPoint {
                        x: shoulder,
                        y: 0.0,
                    },
                    ArrowPoint {
                        x: hex_w - shoulder,
                        y: 0.0,
                    },
                    ArrowPoint {
                        x: hex_w,
                        y: -rect_h / 2.0,
                    },
                    ArrowPoint {
                        x: hex_w - shoulder,
                        y: -rect_h,
                    },
                    ArrowPoint {
                        x: shoulder,
                        y: -rect_h,
                    },
                    ArrowPoint {
                        x: 0.0,
                        y: -rect_h / 2.0,
                    },
                ])
            }
            "rect_left_inv_arrow" => Some(vec![
                ArrowPoint {
                    x: -rect_h / 2.0,
                    y: 0.0,
                },
                ArrowPoint { x: rect_w, y: 0.0 },
                ArrowPoint {
                    x: rect_w,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: -rect_h / 2.0,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: 0.0,
                    y: -rect_h / 2.0,
                },
            ]),
            "subroutine" => Some(vec![
                ArrowPoint { x: 0.0, y: 0.0 },
                ArrowPoint { x: rect_w, y: 0.0 },
                ArrowPoint {
                    x: rect_w,
                    y: -rect_h,
                },
                ArrowPoint { x: 0.0, y: -rect_h },
                ArrowPoint { x: 0.0, y: 0.0 },
                ArrowPoint { x: -8.0, y: 0.0 },
                ArrowPoint {
                    x: rect_w + 8.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w + 8.0,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: -8.0,
                    y: -rect_h,
                },
                ArrowPoint { x: -8.0, y: 0.0 },
            ]),
            "lean_right" => Some(vec![
                ArrowPoint {
                    x: (-2.0 * rect_h) / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w - rect_h / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w + (2.0 * rect_h) / 6.0,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: rect_h / 6.0,
                    y: -rect_h,
                },
            ]),
            "lean_left" => Some(vec![
                ArrowPoint {
                    x: (2.0 * rect_h) / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w + rect_h / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w - (2.0 * rect_h) / 6.0,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: -rect_h / 6.0,
                    y: -rect_h,
                },
            ]),
            "trapezoid" => Some(vec![
                ArrowPoint {
                    x: (-2.0 * rect_h) / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w + (2.0 * rect_h) / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w - rect_h / 6.0,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: rect_h / 6.0,
                    y: -rect_h,
                },
            ]),
            "inv_trapezoid" => Some(vec![
                ArrowPoint {
                    x: rect_h / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w - rect_h / 6.0,
                    y: 0.0,
                },
                ArrowPoint {
                    x: rect_w + (2.0 * rect_h) / 6.0,
                    y: -rect_h,
                },
                ArrowPoint {
                    x: (-2.0 * rect_h) / 6.0,
                    y: -rect_h,
                },
            ]),
            "block_arrow" => Some(block_arrow_points(
                &render_node.directions,
                bbox_w,
                bbox_h,
                node_padding,
            )),
            _ => None,
        }
    }

    fn block_intersect_node(
        node: &LayoutNode,
        render_node: &RenderNode,
        point: &LayoutPoint,
        node_padding: f64,
    ) -> LayoutPoint {
        match render_node.block_type.as_str() {
            "circle" | "doublecircle" => intersect_circle(node, point),
            "cylinder" => intersect_cylinder(node, point),
            "diamond"
            | "hexagon"
            | "rect_left_inv_arrow"
            | "subroutine"
            | "lean_right"
            | "lean_left"
            | "trapezoid"
            | "inv_trapezoid"
            | "block_arrow" => block_polygon_points(node, render_node, node_padding)
                .map(|points| intersect_polygon(node, &points, point))
                .unwrap_or_else(|| intersect_rect(node, point)),
            _ => intersect_rect(node, point),
        }
    }

    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");

    let bounds = layout.bounds.clone().unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    });
    let diagram_padding = config_f64(effective_config, &["block", "diagramPadding"])
        .unwrap_or(5.0)
        .max(0.0);

    let vb_min_x = bounds.min_x - diagram_padding;
    let vb_min_y = bounds.min_y - diagram_padding;
    let vb_w = (bounds.max_x - bounds.min_x + diagram_padding * 2.0).max(1.0);
    let vb_h = (bounds.max_y - bounds.min_y + diagram_padding * 2.0).max(1.0);

    let mut out = String::new();
    let viewbox_attr = format!(
        "{} {} {} {}",
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w.max(1.0)),
        fmt(vb_h.max(1.0))
    );
    let max_w_style = fmt_max_width_px(vb_w.max(1.0));
    let style_attr = format!("max-width: {max_w_style}px; background-color: white;");
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(&viewbox_attr),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "block")
        },
    );
    out.push_str("<style>");
    out.push_str(&block_css(diagram_id, effective_config));
    out.push_str("</style><g/>");

    let _ = write!(
        &mut out,
        r#"<marker id="{}" class="marker block" viewBox="0 0 10 10" refX="6" refY="5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M 0 0 L 10 5 L 0 10 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        escape_xml(&marker_id(diagram_id, "pointEnd"))
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{}" class="marker block" viewBox="0 0 10 10" refX="4.5" refY="5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M 0 5 L 10 10 L 10 0 z" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        escape_xml(&marker_id(diagram_id, "pointStart"))
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{}" class="marker block" viewBox="0 0 10 10" refX="11" refY="5" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><circle cx="5" cy="5" r="5" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        escape_xml(&marker_id(diagram_id, "circleEnd"))
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{}" class="marker block" viewBox="0 0 10 10" refX="-1" refY="5" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><circle cx="5" cy="5" r="5" class="arrowMarkerPath" style="stroke-width: 1; stroke-dasharray: 1, 0;"/></marker>"#,
        escape_xml(&marker_id(diagram_id, "circleStart"))
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{}" class="marker cross block" viewBox="0 0 11 11" refX="12" refY="5.2" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><path d="M 1,1 l 9,9 M 10,1 l -9,9" class="arrowMarkerPath" style="stroke-width: 2; stroke-dasharray: 1, 0;"/></marker>"#,
        escape_xml(&marker_id(diagram_id, "crossEnd"))
    );
    let _ = write!(
        &mut out,
        r#"<marker id="{}" class="marker cross block" viewBox="0 0 11 11" refX="-1" refY="5.2" markerUnits="userSpaceOnUse" markerWidth="11" markerHeight="11" orient="auto"><path d="M 1,1 l 9,9 M 10,1 l -9,9" class="arrowMarkerPath" style="stroke-width: 2; stroke-dasharray: 1, 0;"/></marker>"#,
        escape_xml(&marker_id(diagram_id, "crossStart"))
    );

    out.push_str(r#"<g class="block">"#);

    for n in &layout.nodes {
        let Some(node) = nodes_by_id.get(&n.id) else {
            continue;
        };

        let class_str = if node.classes.is_empty() {
            "default".to_string()
        } else {
            node.classes.join(" ")
        };
        let class_str = format!("{class_str} flowchart-label");
        let (node_box_style, node_text_style, node_div_style_prefix) =
            compile_block_inline_styles(&node.styles);

        let width = n.width.max(1.0);
        let height = n.height.max(1.0);
        let x = -width / 2.0;
        let y = -height / 2.0;

        let id_attr = format!(r#" id="{}""#, escape_attr(&dom_id(diagram_id, &n.id)));
        let _ = write!(
            &mut out,
            r#"<g class="node default {}"{} transform="translate({}, {})">"#,
            escape_attr(&class_str),
            id_attr,
            fmt(n.x),
            fmt(n.y)
        );

        fn emit_polygon(
            out: &mut String,
            points: &[ArrowPoint],
            base_w: f64,
            base_h: f64,
            style_attr: &str,
        ) {
            out.push_str(r#"<polygon points=""#);
            for (idx, point) in points.iter().enumerate() {
                if idx > 0 {
                    out.push(' ');
                }
                let _ = write!(out, "{},{}", fmt_display(point.x), fmt_display(point.y));
            }
            let _ = write!(
                out,
                r#"" class="label-container" style="{}" transform="translate({},{})"/>"#,
                escape_attr(style_attr),
                fmt_display(-base_w / 2.0),
                fmt_display(base_h / 2.0)
            );
        }

        let bbox_w = n.label_width.unwrap_or(0.0).max(0.0);
        let bbox_h = n.label_height.unwrap_or(0.0).max(0.0);
        let rect_w = (bbox_w + node_padding).max(1.0);
        let rect_h = (bbox_h + node_padding).max(1.0);

        match node.block_type.as_str() {
            "circle" => {
                let _ = write!(
                    &mut out,
                    r#"<circle style="{}" rx="0" ry="0" r="{}" width="{}" height="{}"/>"#,
                    escape_attr(&node_box_style),
                    fmt(rect_w / 2.0),
                    fmt(rect_w),
                    fmt(rect_h)
                );
            }
            "doublecircle" => {
                let outer_w = rect_w + 10.0;
                let outer_h = rect_h + 10.0;
                let _ = write!(
                    &mut out,
                    r#"<g class="default flowchart-label"><circle style="{}" rx="0" ry="0" r="{}" width="{}" height="{}"/><circle style="{}" rx="0" ry="0" r="{}" width="{}" height="{}"/></g>"#,
                    escape_attr(&node_box_style),
                    fmt(outer_w / 2.0),
                    fmt(outer_w),
                    fmt(outer_h),
                    escape_attr(&node_box_style),
                    fmt(rect_w / 2.0),
                    fmt(rect_w),
                    fmt(rect_h)
                );
            }
            "stadium" => {
                let stadium_w = (bbox_w + rect_h / 4.0 + node_padding).max(1.0);
                let _ = write!(
                    &mut out,
                    r#"<rect rx="{}" ry="{}" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                    fmt(rect_h / 2.0),
                    fmt(rect_h / 2.0),
                    escape_attr(&node_box_style),
                    fmt(-stadium_w / 2.0),
                    fmt(-rect_h / 2.0),
                    fmt(stadium_w),
                    fmt(rect_h)
                );
            }
            "cylinder" => {
                let rx = rect_w / 2.0;
                let ry = rx / (2.5 + rect_w / 50.0);
                let body_h = (bbox_h + ry + node_padding).max(1.0);
                let _ = write!(
                    &mut out,
                    r#"<path d="M {},{} a {},{} 0,0,0 {} 0 a {},{} 0,0,0 {} 0 l 0,{} a {},{} 0,0,0 {} 0 l 0,{}" style="{}" transform="translate({},{})"/>"#,
                    fmt_display(0.0),
                    fmt_display(ry),
                    fmt_display(rx),
                    fmt_display(ry),
                    fmt_display(rect_w),
                    fmt_display(rx),
                    fmt_display(ry),
                    fmt_display(-rect_w),
                    fmt_display(body_h),
                    fmt_display(rx),
                    fmt_display(ry),
                    fmt_display(rect_w),
                    fmt_display(-body_h),
                    escape_attr(&node_box_style),
                    fmt_display(-rect_w / 2.0),
                    fmt_display(-(body_h / 2.0 + ry))
                );
            }
            "diamond" => {
                let side = (rect_w + rect_h).max(1.0);
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: side / 2.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: side,
                            y: -side / 2.0,
                        },
                        ArrowPoint {
                            x: side / 2.0,
                            y: -side,
                        },
                        ArrowPoint {
                            x: 0.0,
                            y: -side / 2.0,
                        },
                    ],
                    side,
                    side,
                    &node_box_style,
                );
            }
            "hexagon" => {
                let shoulder = rect_h / 4.0;
                let hex_w = (bbox_w + 2.0 * shoulder + node_padding).max(1.0);
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: shoulder,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: hex_w - shoulder,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: hex_w,
                            y: -rect_h / 2.0,
                        },
                        ArrowPoint {
                            x: hex_w - shoulder,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: shoulder,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: 0.0,
                            y: -rect_h / 2.0,
                        },
                    ],
                    hex_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "rect_left_inv_arrow" => {
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: -rect_h / 2.0,
                            y: 0.0,
                        },
                        ArrowPoint { x: rect_w, y: 0.0 },
                        ArrowPoint {
                            x: rect_w,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: -rect_h / 2.0,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: 0.0,
                            y: -rect_h / 2.0,
                        },
                    ],
                    rect_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "subroutine" => {
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint { x: 0.0, y: 0.0 },
                        ArrowPoint { x: rect_w, y: 0.0 },
                        ArrowPoint {
                            x: rect_w,
                            y: -rect_h,
                        },
                        ArrowPoint { x: 0.0, y: -rect_h },
                        ArrowPoint { x: 0.0, y: 0.0 },
                        ArrowPoint { x: -8.0, y: 0.0 },
                        ArrowPoint {
                            x: rect_w + 8.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w + 8.0,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: -8.0,
                            y: -rect_h,
                        },
                        ArrowPoint { x: -8.0, y: 0.0 },
                    ],
                    rect_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "lean_right" => {
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: (-2.0 * rect_h) / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w - rect_h / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w + (2.0 * rect_h) / 6.0,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: rect_h / 6.0,
                            y: -rect_h,
                        },
                    ],
                    rect_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "lean_left" => {
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: (2.0 * rect_h) / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w + rect_h / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w - (2.0 * rect_h) / 6.0,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: -rect_h / 6.0,
                            y: -rect_h,
                        },
                    ],
                    rect_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "trapezoid" => {
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: (-2.0 * rect_h) / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w + (2.0 * rect_h) / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w - rect_h / 6.0,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: rect_h / 6.0,
                            y: -rect_h,
                        },
                    ],
                    rect_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "inv_trapezoid" => {
                emit_polygon(
                    &mut out,
                    &[
                        ArrowPoint {
                            x: rect_h / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w - rect_h / 6.0,
                            y: 0.0,
                        },
                        ArrowPoint {
                            x: rect_w + (2.0 * rect_h) / 6.0,
                            y: -rect_h,
                        },
                        ArrowPoint {
                            x: (-2.0 * rect_h) / 6.0,
                            y: -rect_h,
                        },
                    ],
                    rect_w,
                    rect_h,
                    &node_box_style,
                );
            }
            "composite" => {
                let _ = write!(
                    &mut out,
                    r#"<rect class="basic cluster composite label-container" rx="0" ry="0" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                    escape_attr(&node_box_style),
                    fmt(x),
                    fmt(y),
                    fmt(width),
                    fmt(height)
                );
            }
            "block_arrow" => {
                let h = (bbox_h + 2.0 * node_padding).max(1.0);
                let m = h / 2.0;
                let w = (bbox_w + 2.0 * m + node_padding).max(1.0);
                let pts = block_arrow_points(&node.directions, bbox_w, bbox_h, node_padding);

                out.push_str(r#"<polygon points=""#);
                for (idx, p) in pts.iter().enumerate() {
                    if idx > 0 {
                        out.push(' ');
                    }
                    let _ = write!(&mut out, "{},{}", fmt_display(p.x), fmt_display(p.y));
                }
                let _ = write!(
                    &mut out,
                    r#"" class="label-container" style="{}" transform="translate({},{})"/>"#,
                    escape_attr(&node_box_style),
                    fmt_display(-w / 2.0),
                    fmt_display(h / 2.0)
                );
            }
            "round" => {
                let _ = write!(
                    &mut out,
                    r#"<rect class="basic label-container" rx="5" ry="5" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                    escape_attr(&node_box_style),
                    fmt(x),
                    fmt(y),
                    fmt(width),
                    fmt(height)
                );
            }
            _ => {
                let _ = write!(
                    &mut out,
                    r#"<rect class="basic label-container" rx="0" ry="0" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                    escape_attr(&node_box_style),
                    fmt(x),
                    fmt(y),
                    fmt(width),
                    fmt(height)
                );
            }
        }

        let label = decode_block_label_html(&node.label);
        let label_effectively_empty =
            node.label.is_empty() || block_label_is_effectively_empty(&label);
        let (label_tx, label_ty, label_w, label_h) = if label_effectively_empty {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            let label_w = n.label_width.unwrap_or(0.0).max(0.0);
            let label_h = n.label_height.unwrap_or(0.0).max(0.0);
            (-label_w / 2.0, -label_h / 2.0, label_w, label_h)
        };
        let span_style_attr = if node_text_style.is_empty() {
            String::new()
        } else {
            format!(r#" style="{}""#, escape_attr(&node_text_style))
        };
        let label_markup = if node.label.is_empty() {
            String::new()
        } else {
            format!("<p>{}</p>", escape_xml(&label))
        };
        let _ = write!(
            &mut out,
            r#"<g class="label" style="{}" transform="translate({}, {})"><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}display: table-cell; white-space: nowrap; line-height: 1.5;"><span class="nodeLabel"{}>{}</span></div></foreignObject></g>"#,
            escape_attr(&node_text_style),
            fmt(label_tx),
            fmt(label_ty),
            fmt(label_w),
            fmt(label_h),
            escape_attr(&node_div_style_prefix),
            span_style_attr,
            label_markup
        );

        out.push_str("</g>");
    }

    for e in &model.edges {
        let Some(le) = layout.edges.iter().find(|x| x.id == e.id) else {
            continue;
        };
        let mut edge_points = match (
            layout_nodes_by_id.get(&e.start),
            layout_nodes_by_id.get(&e.end),
            nodes_by_id.get(&e.start),
            nodes_by_id.get(&e.end),
        ) {
            (Some(from), Some(to), Some(from_render), Some(to_render)) => {
                let mid = le.points.get(1).cloned().unwrap_or(LayoutPoint {
                    x: from.x + (to.x - from.x) / 2.0,
                    y: from.y + (to.y - from.y) / 2.0,
                });
                vec![
                    block_intersect_node(from, from_render, &mid, node_padding),
                    mid.clone(),
                    block_intersect_node(to, to_render, &mid, node_padding),
                ]
            }
            _ => le.points.clone(),
        };
        if edge_points.len() >= 2 {
            let start_inset = block_edge_start_marker_inset(e.arrow_type_start.as_deref());
            if start_inset > 0.0 {
                edge_points[0] = move_point_towards(&edge_points[0], &edge_points[1], start_inset);
            }
            let end_inset = block_edge_end_marker_inset(e.arrow_type_end.as_deref());
            if end_inset > 0.0 {
                let last = edge_points.len() - 1;
                edge_points[last] =
                    move_point_towards(&edge_points[last], &edge_points[last - 1], end_inset);
            }
        }
        let d = curve_basis_path_d(&edge_points);
        let class_attr = "edge-thickness-normal edge-pattern-solid edge-thickness-normal edge-pattern-solid flowchart-link LS-a1 LE-b1";
        let _ = write!(
            &mut out,
            r#"<path d="{}" id="{}" class="{}""#,
            escape_attr(&d),
            escape_attr(&dom_id(diagram_id, &e.id)),
            escape_attr(class_attr)
        );

        if let Some(m) = edge_marker_start(e.arrow_type_start.as_deref()) {
            let _ = write!(
                &mut out,
                r#" marker-start="{}""#,
                escape_attr(&marker_url(diagram_id, m))
            );
        }
        if let Some(m) = edge_marker_end(e.arrow_type_end.as_deref()) {
            let _ = write!(
                &mut out,
                r#" marker-end="{}""#,
                escape_attr(&marker_url(diagram_id, m))
            );
        }
        out.push_str("/>");
    }

    for e in &model.edges {
        let Some(le) = layout.edges.iter().find(|x| x.id == e.id) else {
            continue;
        };
        let Some(lbl) = le.label.as_ref().filter(|_| !e.label.trim().is_empty()) else {
            continue;
        };

        let _ = write!(
            &mut out,
            r#"<g class="edgeLabel" transform="translate({}, {})"><g class="label" transform="translate({}, {})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="stroke: rgb(51, 51, 51); stroke-width: 1.5px; display: table-cell; white-space: nowrap; line-height: 1.5;"><span class="edgeLabel" style="stroke: #333; stroke-width: 1.5px;color:none;"><p>{}</p></span></div></foreignObject></g></g>"#,
            fmt(lbl.x),
            fmt(lbl.y),
            fmt(-lbl.width / 2.0),
            fmt(-lbl.height / 2.0),
            fmt(lbl.width),
            fmt(lbl.height),
            escape_xml(&decode_block_label_html(&e.label))
        );
    }

    out.push_str("</g></svg>\n");
    Ok(out)
}
