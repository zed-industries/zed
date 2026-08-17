use std::fmt::Write as _;

use crate::architecture_metrics::{
    ARCHITECTURE_CREATE_TEXT_DEFAULT_WRAP_WIDTH_PX, ARCHITECTURE_SERVICE_LABEL_BOTTOM_EXTENSION_PX,
    architecture_create_text_bbox_height_px, architecture_create_text_bbox_y_range_px,
};
use crate::model::Bounds;
use crate::text::{TextMeasurer, VendoredFontMetricsTextMeasurer};

use super::super::{escape_xml, fmt};
use super::geometry::{
    arrow_points, arrow_shift, bounds_from_rect, edge_id, extend_bounds, is_arch_dir_x,
    is_arch_dir_y,
};
use super::labels::{svg_line_plain_text, wrap_svg_words_to_lines, write_svg_text_lines};
use super::model::ArchitectureModelAccess;
use super::settings::ArchitectureRenderSettings;
use crate::model::ArchitectureDiagramLayout;

pub(super) struct ArchitectureEdgeRenderContext<'a, M: ArchitectureModelAccess> {
    pub(super) out: &'a mut String,
    pub(super) diagram_id: &'a str,
    pub(super) layout: &'a ArchitectureDiagramLayout,
    pub(super) model: &'a M,
    pub(super) node_xy: &'a rustc_hash::FxHashMap<&'a str, (f64, f64)>,
    pub(super) settings: &'a ArchitectureRenderSettings,
    pub(super) text_measurer: &'a VendoredFontMetricsTextMeasurer,
    pub(super) content_bounds: &'a mut Option<Bounds>,
    pub(super) junction_bounds: &'a rustc_hash::FxHashMap<&'a str, Bounds>,
}

struct ArchitectureEdgeLabelPlan {
    lines: Vec<super::labels::SvgLine>,
    bounds: Bounds,
    dominant_baseline: &'static str,
    transform: String,
}

#[derive(Clone, Copy)]
struct ArchitectureEdgePoints {
    start_x: f64,
    start_y: f64,
    mid_x: f64,
    mid_y: f64,
    end_x: f64,
    end_y: f64,
}

struct ArchitectureArrowGeometry {
    points: String,
    transform: String,
    bounds: Bounds,
}

fn architecture_dir_unit(dir: char) -> (f64, f64) {
    match dir {
        'L' => (1.0, 0.0),
        'R' => (-1.0, 0.0),
        'T' => (0.0, 1.0),
        'B' => (0.0, -1.0),
        _ => (1.0, 0.0),
    }
}

fn architecture_arrow_geometry(
    dir: char,
    anchor_x: f64,
    anchor_y: f64,
    adjacent_x: f64,
    adjacent_y: f64,
    arrow_size: f64,
) -> ArchitectureArrowGeometry {
    let half_arrow_size = arrow_size / 2.0;
    let dx = anchor_x - adjacent_x;
    let dy = anchor_y - adjacent_y;
    let len = (dx * dx + dy * dy).sqrt();
    let (ux, uy) = if len > 1e-6 {
        (dx / len, dy / len)
    } else {
        architecture_dir_unit(dir)
    };

    let port_x_shift = if is_arch_dir_x(dir) {
        arrow_shift(dir, anchor_x, arrow_size)
    } else {
        anchor_x - half_arrow_size
    };
    let port_y_shift = if is_arch_dir_y(dir) {
        arrow_shift(dir, anchor_y, arrow_size)
    } else {
        anchor_y - half_arrow_size
    };

    if ux.abs() < 1e-6 || uy.abs() < 1e-6 {
        return ArchitectureArrowGeometry {
            points: arrow_points(dir, arrow_size),
            transform: format!("translate({},{})", fmt(port_x_shift), fmt(port_y_shift)),
            bounds: bounds_from_rect(port_x_shift, port_y_shift, arrow_size, arrow_size),
        };
    }

    // Mermaid positions Architecture arrows as standalone polygons rather than SVG markers.
    // Rotate that polygon by the actual final segment, so diagonal crosslinks point along the
    // edge instead of only following the requested port side.
    let tip_x = anchor_x + 2.0 * ux;
    let tip_y = anchor_y + 2.0 * uy;
    let base_x = tip_x - arrow_size * ux;
    let base_y = tip_y - arrow_size * uy;
    let perp_x = -uy * half_arrow_size;
    let perp_y = ux * half_arrow_size;
    let p0 = (base_x + perp_x, base_y + perp_y);
    let p1 = (base_x - perp_x, base_y - perp_y);
    let p2 = (tip_x, tip_y);
    let exact_bounds = Bounds {
        min_x: p0.0.min(p1.0).min(p2.0),
        min_y: p0.1.min(p1.1).min(p2.1),
        max_x: p0.0.max(p1.0).max(p2.0),
        max_y: p0.1.max(p1.1).max(p2.1),
    };
    let port_bounds = bounds_from_rect(port_x_shift, port_y_shift, arrow_size, arrow_size);
    let angle = (-ux).atan2(uy).to_degrees();

    ArchitectureArrowGeometry {
        points: arrow_points('T', arrow_size),
        transform: format!(
            "translate({},{}) rotate({},{},{})",
            fmt(tip_x - half_arrow_size),
            fmt(tip_y - arrow_size),
            fmt(angle),
            fmt(half_arrow_size),
            fmt(arrow_size)
        ),
        bounds: Bounds {
            min_x: exact_bounds.min_x.min(port_bounds.min_x),
            min_y: exact_bounds.min_y.min(port_bounds.min_y),
            max_x: exact_bounds.max_x.max(port_bounds.max_x),
            max_y: exact_bounds.max_y.max(port_bounds.max_y),
        },
    }
}

fn architecture_edge_label_plan(
    edge: super::model::ArchitectureEdgeRef<'_>,
    points: ArchitectureEdgePoints,
    settings: &ArchitectureRenderSettings,
    text_measurer: &VendoredFontMetricsTextMeasurer,
) -> Option<ArchitectureEdgeLabelPlan> {
    let label = edge.title.map(str::trim).filter(|t| !t.is_empty())?;
    let axis = match (is_arch_dir_x(edge.lhs_dir), is_arch_dir_x(edge.rhs_dir)) {
        (true, true) => "X",
        (false, false) => "Y",
        _ => "XY",
    };

    let wrap_width = match axis {
        "X" => (points.start_x - points.end_x).abs(),
        "Y" => (points.start_y - points.end_y).abs() / 1.5,
        _ => (points.start_x - points.end_x).abs() / 2.0,
    };
    let wrap_width = if wrap_width.is_finite() && wrap_width > 0.0 {
        wrap_width
    } else {
        ARCHITECTURE_CREATE_TEXT_DEFAULT_WRAP_WIDTH_PX
    };
    let lines = wrap_svg_words_to_lines(label, wrap_width, text_measurer, &settings.text_style);

    let mut bbox_w = 0.0f64;
    for line in &lines {
        let s = svg_line_plain_text(line);
        let m = text_measurer.measure_wrapped(
            s.as_str(),
            &settings.text_style,
            None,
            crate::text::WrapMode::SvgLike,
        );
        bbox_w = bbox_w.max(m.width);
    }
    let line_count = lines.len().max(1);
    let bbox_h = architecture_create_text_bbox_height_px(settings.svg_font_size_px, line_count);
    let half_bbox_h = bbox_h / 2.0;
    let half_bbox_w = bbox_w / 2.0;
    let (bbox_y_min, bbox_y_max) =
        architecture_create_text_bbox_y_range_px(settings.svg_font_size_px, line_count);

    let (dominant_baseline, transform) = match axis {
        "Y" => (
            "middle",
            format!(
                r#"translate({}, {}) rotate(-90)"#,
                fmt(points.mid_x),
                fmt(points.mid_y)
            ),
        ),
        "XY" => {
            let pair = format!("{}{}", edge.lhs_dir, edge.rhs_dir);
            let (xf, yf): (f64, f64) = match pair.as_str() {
                "LT" | "TL" => (1.0, 1.0),
                "BL" | "LB" => (1.0, -1.0),
                "BR" | "RB" => (-1.0, -1.0),
                _ => (-1.0, 1.0),
            };
            let angle = (-xf * yf * 45.0f64).round() as i64;

            // Rotated bbox at 45° (w' == h' == (w+h)*sqrt(2)/2).
            let diag = (bbox_w + bbox_h) * std::f64::consts::FRAC_1_SQRT_2;
            let t2x = xf * diag / 2.0;
            let t2y = yf * diag / 2.0;
            // Mermaid CLI serializes newline characters inside attribute values as XML entities
            // (`&#10;`). Emit those explicitly so our SVG matches the upstream baselines.
            let sep = "&#10;";

            (
                "auto",
                format!(
                    "translate({}, {}){sep}                translate({}, {}){sep}                rotate({}, 0, {})",
                    fmt(points.mid_x),
                    fmt(points.mid_y - half_bbox_h),
                    fmt(t2x),
                    fmt(t2y),
                    angle,
                    fmt(half_bbox_h),
                    sep = sep
                ),
            )
        }
        _ => (
            "middle",
            format!(r#"translate({}, {})"#, fmt(points.mid_x), fmt(points.mid_y)),
        ),
    };

    let bounds = match axis {
        "X" => Bounds {
            min_x: points.mid_x - half_bbox_w,
            min_y: points.mid_y + bbox_y_min,
            max_x: points.mid_x + half_bbox_w,
            max_y: points.mid_y + bbox_y_max,
        },
        "Y" => Bounds {
            min_x: points.mid_x + bbox_y_min,
            min_y: points.mid_y - half_bbox_w,
            max_x: points.mid_x + bbox_y_max,
            max_y: points.mid_y + half_bbox_w,
        },
        _ => {
            // |cos(45°)| == |sin(45°)| == sqrt(1/2)
            let a = (bbox_w + bbox_h) * std::f64::consts::FRAC_1_SQRT_2;
            Bounds {
                min_x: points.mid_x - a / 2.0,
                min_y: points.mid_y - a / 2.0,
                max_x: points.mid_x + a / 2.0,
                max_y: points.mid_y + a / 2.0,
            }
        }
    };

    Some(ArchitectureEdgeLabelPlan {
        lines,
        bounds,
        dominant_baseline,
        transform,
    })
}

pub(super) fn push_architecture_edges<M: ArchitectureModelAccess>(
    ctx: &mut ArchitectureEdgeRenderContext<'_, M>,
) {
    let out = &mut *ctx.out;
    let diagram_id = ctx.diagram_id;
    let layout = ctx.layout;
    let model = ctx.model;
    let node_xy = ctx.node_xy;
    let settings = ctx.settings;
    let text_measurer = ctx.text_measurer;
    let content_bounds = &mut *ctx.content_bounds;
    let junction_bounds = ctx.junction_bounds;

    let group_edge_shift = settings.padding_px + 4.0;
    let group_edge_label_bottom_px = ARCHITECTURE_SERVICE_LABEL_BOTTOM_EXTENSION_PX;
    let is_junction = |id: &str| junction_bounds.contains_key(id);

    let layout_edge_points: Vec<(f64, f64, f64, f64, f64, f64)> = layout
        .edges
        .iter()
        .map(|e| {
            // Architecture layout edges are expected to be 3-point polylines.
            // Be defensive and fall back to zeros if the snapshot is malformed.
            let p0 = e.points.first().map(|p| (p.x, p.y)).unwrap_or((0.0, 0.0));
            let pm = e.points.get(1).map(|p| (p.x, p.y)).unwrap_or((0.0, 0.0));
            let p2 = e.points.last().map(|p| (p.x, p.y)).unwrap_or((0.0, 0.0));
            (p0.0, p0.1, pm.0, pm.1, p2.0, p2.1)
        })
        .collect();

    let edge_points =
        |edge_idx: usize, edge: super::model::ArchitectureEdgeRef<'_>| -> ArchitectureEdgePoints {
            // Prefer layout-provided points: this is where we model Mermaid/Cytoscape edge routing.
            //
            // The layout points represent raw Cytoscape endpoints; Mermaid applies group/junction
            // endpoint shifts later, during SVG emission.
            let (raw_start_x, raw_start_y, mid_x, mid_y, raw_end_x, raw_end_y) = layout_edge_points
                .get(edge_idx)
                .copied()
                .unwrap_or_else(|| {
                    let (sx, sy) = node_xy.get(edge.lhs_id).copied().unwrap_or((0.0, 0.0));
                    let (tx, ty) = node_xy.get(edge.rhs_id).copied().unwrap_or((0.0, 0.0));

                    let (sx, sy) = match edge.lhs_dir {
                        'L' => (sx, sy + settings.half_icon),
                        'R' => (sx + settings.icon_size_px, sy + settings.half_icon),
                        'T' => (sx + settings.half_icon, sy),
                        'B' => (sx + settings.half_icon, sy + settings.icon_size_px),
                        _ => (sx + settings.half_icon, sy + settings.half_icon),
                    };
                    let (tx, ty) = match edge.rhs_dir {
                        'L' => (tx, ty + settings.half_icon),
                        'R' => (tx + settings.icon_size_px, ty + settings.half_icon),
                        'T' => (tx + settings.half_icon, ty),
                        'B' => (tx + settings.half_icon, ty + settings.icon_size_px),
                        _ => (tx + settings.half_icon, ty + settings.half_icon),
                    };

                    let (mx, my) = if (sx - tx).abs() > 1e-6 && (sy - ty).abs() > 1e-6 {
                        // Match upstream Mermaid: choose the bend based on the *source* dir.
                        if is_arch_dir_y(edge.lhs_dir) {
                            (sx, ty)
                        } else {
                            (tx, sy)
                        }
                    } else {
                        ((sx + tx) / 2.0, (sy + ty) / 2.0)
                    };
                    (sx, sy, mx, my, tx, ty)
                });

            let mut start_x = raw_start_x;
            let mut start_y = raw_start_y;
            let mut end_x = raw_end_x;
            let mut end_y = raw_end_y;

            let lhs_group = edge.lhs_group.unwrap_or(false);
            if lhs_group {
                if is_arch_dir_x(edge.lhs_dir) {
                    start_x += if edge.lhs_dir == 'L' {
                        -group_edge_shift
                    } else {
                        group_edge_shift
                    };
                } else {
                    start_y += if edge.lhs_dir == 'T' {
                        -group_edge_shift
                    } else {
                        group_edge_shift + group_edge_label_bottom_px
                    };
                }
            }
            if !lhs_group && is_junction(edge.lhs_id) {
                if is_arch_dir_x(edge.lhs_dir) {
                    start_x += if edge.lhs_dir == 'L' {
                        settings.half_icon
                    } else {
                        -settings.half_icon
                    };
                } else {
                    start_y += if edge.lhs_dir == 'T' {
                        settings.half_icon
                    } else {
                        -settings.half_icon
                    };
                }
            }

            let rhs_group = edge.rhs_group.unwrap_or(false);
            if rhs_group {
                if is_arch_dir_x(edge.rhs_dir) {
                    end_x += if edge.rhs_dir == 'L' {
                        -group_edge_shift
                    } else {
                        group_edge_shift
                    };
                } else {
                    end_y += if edge.rhs_dir == 'T' {
                        -group_edge_shift
                    } else {
                        group_edge_shift + group_edge_label_bottom_px
                    };
                }
            }
            if !rhs_group && is_junction(edge.rhs_id) {
                if is_arch_dir_x(edge.rhs_dir) {
                    end_x += if edge.rhs_dir == 'L' {
                        settings.half_icon
                    } else {
                        -settings.half_icon
                    };
                } else {
                    end_y += if edge.rhs_dir == 'T' {
                        settings.half_icon
                    } else {
                        -settings.half_icon
                    };
                }
            }

            ArchitectureEdgePoints {
                start_x,
                start_y,
                mid_x,
                mid_y,
                end_x,
                end_y,
            }
        };

    // Edges (including conservative label bounds).
    if model.edges_len() != 0 {
        let arrow_size = settings.icon_size_px / 6.0;
        for (edge_idx, edge) in model.edges().enumerate() {
            let points = edge_points(edge_idx, edge);

            extend_bounds(
                content_bounds,
                Bounds::from_points(vec![
                    (points.start_x, points.start_y),
                    (points.mid_x, points.mid_y),
                    (points.end_x, points.end_y),
                ])
                .unwrap_or(Bounds {
                    min_x: points.start_x,
                    min_y: points.start_y,
                    max_x: points.end_x,
                    max_y: points.end_y,
                }),
            );

            if edge.lhs_into == Some(true) {
                let arrow = architecture_arrow_geometry(
                    edge.lhs_dir,
                    points.start_x,
                    points.start_y,
                    points.mid_x,
                    points.mid_y,
                    arrow_size,
                );
                extend_bounds(content_bounds, arrow.bounds);
            }

            if edge.rhs_into == Some(true) {
                let arrow = architecture_arrow_geometry(
                    edge.rhs_dir,
                    points.end_x,
                    points.end_y,
                    points.mid_x,
                    points.mid_y,
                    arrow_size,
                );
                extend_bounds(content_bounds, arrow.bounds);
            }

            let label_plan = architecture_edge_label_plan(edge, points, settings, text_measurer);
            if let Some(label_plan) = label_plan.as_ref() {
                extend_bounds(content_bounds, label_plan.bounds.clone());
            }

            out.push_str("<g>");
            let id = format!("{diagram_id}-{}", edge_id("L", edge.lhs_id, edge.rhs_id, 0));
            let _ = write!(
                out,
                r#"<path d="M {sx},{sy} L {mx},{my} L{ex},{ey} " class="edge" id="{id}"/>"#,
                sx = fmt(points.start_x),
                sy = fmt(points.start_y),
                mx = fmt(points.mid_x),
                my = fmt(points.mid_y),
                ex = fmt(points.end_x),
                ey = fmt(points.end_y),
                id = escape_xml(&id)
            );

            if edge.lhs_into == Some(true) {
                let arrow = architecture_arrow_geometry(
                    edge.lhs_dir,
                    points.start_x,
                    points.start_y,
                    points.mid_x,
                    points.mid_y,
                    arrow_size,
                );
                let _ = write!(
                    out,
                    r#"<polygon points="{pts}" transform="{transform}" class="arrow"/>"#,
                    pts = arrow.points,
                    transform = arrow.transform
                );
            }

            if edge.rhs_into == Some(true) {
                let arrow = architecture_arrow_geometry(
                    edge.rhs_dir,
                    points.end_x,
                    points.end_y,
                    points.mid_x,
                    points.mid_y,
                    arrow_size,
                );
                let _ = write!(
                    out,
                    r#"<polygon points="{pts}" transform="{transform}" class="arrow"/>"#,
                    pts = arrow.points,
                    transform = arrow.transform
                );
            }

            if let Some(label_plan) = label_plan {
                let _ = write!(
                    out,
                    r#"<g dy="1em" alignment-baseline="middle" dominant-baseline="{baseline}" text-anchor="middle" transform="{transform}">"#,
                    baseline = label_plan.dominant_baseline,
                    transform = label_plan.transform.as_str()
                );
                out.push_str(r#"<g><rect class="background" style="stroke: none"/>"#);
                write_svg_text_lines(out, &label_plan.lines);
                out.push_str("</g></g>");
            }

            out.push_str("</g>");
        }
    }
}
