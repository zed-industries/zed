// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::sync::Arc;

use strict_num::NonZeroPositiveF32;
use svgtypes::Length;
use tiny_skia_path::Point;

use super::converter;
use super::svgtree::{AId, EId, SvgNode};
use crate::{
    ApproxEqUlps, ApproxZeroUlps, ClipPath, Fill, Group, Node, NonZeroRect, Path, Size, Transform,
    ViewBox,
};

// Similar to `tiny_skia_path::PathSegment`, but without the `QuadTo`.
#[derive(Copy, Clone, Debug)]
enum Segment {
    MoveTo(Point),
    LineTo(Point),
    CubicTo(Point, Point, Point),
    Close,
}

pub(crate) fn is_valid(node: SvgNode) -> bool {
    // `marker-*` attributes cannot be set on shapes inside a `clipPath`.
    if node
        .ancestors()
        .any(|n| n.tag_name() == Some(EId::ClipPath))
    {
        return false;
    }

    let start = node.find_attribute::<SvgNode>(AId::MarkerStart);
    let mid = node.find_attribute::<SvgNode>(AId::MarkerMid);
    let end = node.find_attribute::<SvgNode>(AId::MarkerEnd);
    start.is_some() || mid.is_some() || end.is_some()
}

pub(crate) fn convert(
    node: SvgNode,
    path: &tiny_skia_path::Path,
    state: &converter::State,
    cache: &mut converter::Cache,
    parent: &mut Group,
) {
    let list = [
        (AId::MarkerStart, MarkerKind::Start),
        (AId::MarkerMid, MarkerKind::Middle),
        (AId::MarkerEnd, MarkerKind::End),
    ];

    for (aid, kind) in &list {
        let mut marker = None;
        if let Some(link) = node.find_attribute::<SvgNode>(*aid) {
            if link.tag_name() == Some(EId::Marker) {
                marker = Some(link);
            }
        }

        if let Some(marker) = marker {
            // TODO: move to svgtree
            // Check for recursive marker.
            if state.parent_markers.contains(&marker) {
                log::warn!("Recursive marker detected: {}", marker.element_id());
                continue;
            }

            resolve(node, path, marker, *kind, state, cache, parent);
        }
    }
}

#[derive(Clone, Copy)]
enum MarkerKind {
    Start,
    Middle,
    End,
}

enum MarkerOrientation {
    Auto,
    AutoStartReverse,
    Angle(f32),
}

fn resolve(
    shape_node: SvgNode,
    path: &tiny_skia_path::Path,
    marker_node: SvgNode,
    marker_kind: MarkerKind,
    state: &converter::State,
    cache: &mut converter::Cache,
    parent: &mut Group,
) -> Option<()> {
    let stroke_scale = stroke_scale(shape_node, marker_node, state)?.get();

    let r = convert_rect(marker_node, state)?;

    let view_box = marker_node.parse_viewbox().map(|vb| ViewBox {
        rect: vb,
        aspect: marker_node
            .attribute(AId::PreserveAspectRatio)
            .unwrap_or_default(),
    });

    let has_overflow = {
        let overflow = marker_node.attribute(AId::Overflow);
        // `overflow` is `hidden` by default.
        overflow.is_none() || overflow == Some("hidden") || overflow == Some("scroll")
    };

    let clip_path = if has_overflow {
        let clip_rect = if let Some(vbox) = view_box {
            vbox.rect
        } else {
            r.size().to_non_zero_rect(0.0, 0.0)
        };

        let mut clip_path = ClipPath::empty(cache.gen_clip_path_id());

        let mut path = Path::new_simple(Arc::new(tiny_skia_path::PathBuilder::from_rect(
            clip_rect.to_rect(),
        )))?;
        path.fill = Some(Fill::default());

        clip_path.root.children.push(Node::Path(Box::new(path)));

        Some(Arc::new(clip_path))
    } else {
        None
    };

    // TODO: avoid allocation
    let mut segments: Vec<Segment> = Vec::with_capacity(path.len());
    let mut prev = Point::zero();
    let mut prev_move = Point::zero();
    for seg in path.segments() {
        match seg {
            tiny_skia_path::PathSegment::MoveTo(p) => {
                segments.push(Segment::MoveTo(p));
                prev = p;
                prev_move = p;
            }
            tiny_skia_path::PathSegment::LineTo(p) => {
                segments.push(Segment::LineTo(p));
                prev = p;
            }
            tiny_skia_path::PathSegment::QuadTo(p1, p) => {
                let (p1, p2, p) = quad_to_curve(prev, p1, p);
                segments.push(Segment::CubicTo(p1, p2, p));
                prev = p;
            }
            tiny_skia_path::PathSegment::CubicTo(p1, p2, p) => {
                segments.push(Segment::CubicTo(p1, p2, p));
                prev = p;
            }
            tiny_skia_path::PathSegment::Close => {
                segments.push(Segment::Close);
                prev = prev_move;
            }
        }
    }

    let draw_marker = |p: tiny_skia_path::Point, idx: usize| {
        let mut ts = Transform::from_translate(p.x, p.y);

        let angle = match convert_orientation(marker_node) {
            MarkerOrientation::AutoStartReverse if idx == 0 => {
                (calc_vertex_angle(&segments, idx) + 180.0) % 360.0
            }
            MarkerOrientation::Auto | MarkerOrientation::AutoStartReverse => {
                calc_vertex_angle(&segments, idx)
            }
            MarkerOrientation::Angle(angle) => angle,
        };

        if !angle.approx_zero_ulps(4) {
            ts = ts.pre_rotate(angle);
        }

        if let Some(vbox) = view_box {
            let size = Size::from_wh(r.width() * stroke_scale, r.height() * stroke_scale).unwrap();
            let vbox_ts = vbox.to_transform(size);
            let (sx, sy) = vbox_ts.get_scale();
            ts = ts.pre_scale(sx, sy);
        } else {
            ts = ts.pre_scale(stroke_scale, stroke_scale);
        }

        ts = ts.pre_translate(-r.x(), -r.y());

        // TODO: do not create a group when no clipPath
        let mut g = Group {
            transform: ts,
            abs_transform: parent.abs_transform.pre_concat(ts),
            clip_path: clip_path.clone(),
            ..Group::empty()
        };

        let mut marker_state = state.clone();
        marker_state.parent_markers.push(marker_node);
        converter::convert_children(marker_node, &marker_state, cache, &mut g);
        g.calculate_bounding_boxes();

        if g.has_children() {
            parent.children.push(Node::Group(Box::new(g)));
        }
    };

    draw_markers(&segments, marker_kind, draw_marker);

    Some(())
}

fn stroke_scale(
    path_node: SvgNode,
    marker_node: SvgNode,
    state: &converter::State,
) -> Option<NonZeroPositiveF32> {
    match marker_node.attribute(AId::MarkerUnits) {
        Some("userSpaceOnUse") => NonZeroPositiveF32::new(1.0),
        _ => path_node.resolve_valid_length(AId::StrokeWidth, state, 1.0),
    }
}

fn draw_markers<P>(path: &[Segment], kind: MarkerKind, mut draw_marker: P)
where
    P: FnMut(tiny_skia_path::Point, usize),
{
    match kind {
        MarkerKind::Start => {
            if let Some(Segment::MoveTo(p)) = path.first().cloned() {
                draw_marker(p, 0);
            }
        }
        MarkerKind::Middle => {
            let total = path.len() - 1;
            let mut i = 1;
            while i < total {
                let p = match path[i] {
                    Segment::MoveTo(p) => p,
                    Segment::LineTo(p) => p,
                    Segment::CubicTo(_, _, p) => p,
                    _ => {
                        i += 1;
                        continue;
                    }
                };

                draw_marker(p, i);

                i += 1;
            }
        }
        MarkerKind::End => {
            let idx = path.len() - 1;
            match path.last().cloned() {
                Some(Segment::LineTo(p)) => {
                    draw_marker(p, idx);
                }
                Some(Segment::CubicTo(_, _, p)) => {
                    draw_marker(p, idx);
                }
                Some(Segment::Close) => {
                    let p = get_subpath_start(path, idx);
                    draw_marker(p, idx);
                }
                _ => {}
            }
        }
    }
}

fn calc_vertex_angle(path: &[Segment], idx: usize) -> f32 {
    if idx == 0 {
        // First segment.

        debug_assert!(path.len() > 1);

        let seg1 = path[0];
        let seg2 = path[1];

        match (seg1, seg2) {
            (Segment::MoveTo(pm), Segment::LineTo(p)) => calc_line_angle(pm.x, pm.y, p.x, p.y),
            (Segment::MoveTo(pm), Segment::CubicTo(p1, _, p)) => {
                if pm.x.approx_eq_ulps(&p1.x, 4) && pm.y.approx_eq_ulps(&p1.y, 4) {
                    calc_line_angle(pm.x, pm.y, p.x, p.y)
                } else {
                    calc_line_angle(pm.x, pm.y, p1.x, p1.y)
                }
            }
            _ => 0.0,
        }
    } else if idx == path.len() - 1 {
        // Last segment.

        let seg1 = path[idx - 1];
        let seg2 = path[idx];

        match (seg1, seg2) {
            (_, Segment::MoveTo(_)) => 0.0, // unreachable
            (_, Segment::LineTo(p)) => {
                let prev = get_prev_vertex(path, idx);
                calc_line_angle(prev.x, prev.y, p.x, p.y)
            }
            (_, Segment::CubicTo(p1, p2, p)) => {
                if p2.x.approx_eq_ulps(&p.x, 4) && p2.y.approx_eq_ulps(&p.y, 4) {
                    calc_line_angle(p1.x, p1.y, p.x, p.y)
                } else {
                    calc_line_angle(p2.x, p2.y, p.x, p.y)
                }
            }
            (Segment::LineTo(p), Segment::Close) => {
                let next = get_subpath_start(path, idx);
                calc_line_angle(p.x, p.y, next.x, next.y)
            }
            (Segment::CubicTo(_, p2, p), Segment::Close) => {
                let prev = get_prev_vertex(path, idx);
                let next = get_subpath_start(path, idx);
                calc_curves_angle(
                    prev.x, prev.y, p2.x, p2.y, p.x, p.y, next.x, next.y, next.x, next.y,
                )
            }
            (_, Segment::Close) => 0.0,
        }
    } else {
        // Middle segments.

        let seg1 = path[idx];
        let seg2 = path[idx + 1];

        // TODO: Not sure if there is a better way.
        match (seg1, seg2) {
            (Segment::MoveTo(pm), Segment::LineTo(p)) => calc_line_angle(pm.x, pm.y, p.x, p.y),
            (Segment::MoveTo(pm), Segment::CubicTo(p1, _, _)) => {
                calc_line_angle(pm.x, pm.y, p1.x, p1.y)
            }
            (Segment::LineTo(p1), Segment::LineTo(p2)) => {
                let prev = get_prev_vertex(path, idx);
                calc_angle(prev.x, prev.y, p1.x, p1.y, p1.x, p1.y, p2.x, p2.y)
            }
            (Segment::CubicTo(_, c1_p2, c1_p), Segment::CubicTo(c2_p1, _, c2_p)) => {
                let prev = get_prev_vertex(path, idx);
                calc_curves_angle(
                    prev.x, prev.y, c1_p2.x, c1_p2.y, c1_p.x, c1_p.y, c2_p1.x, c2_p1.y, c2_p.x,
                    c2_p.y,
                )
            }
            (Segment::LineTo(pl), Segment::CubicTo(p1, _, p)) => {
                let prev = get_prev_vertex(path, idx);
                calc_curves_angle(
                    prev.x, prev.y, prev.x, prev.y, pl.x, pl.y, p1.x, p1.y, p.x, p.y,
                )
            }
            (Segment::CubicTo(_, p2, p), Segment::LineTo(pl)) => {
                let prev = get_prev_vertex(path, idx);
                calc_curves_angle(prev.x, prev.y, p2.x, p2.y, p.x, p.y, pl.x, pl.y, pl.x, pl.y)
            }
            (Segment::LineTo(p), Segment::MoveTo(_)) => {
                let prev = get_prev_vertex(path, idx);
                calc_line_angle(prev.x, prev.y, p.x, p.y)
            }
            (Segment::CubicTo(_, p2, p), Segment::MoveTo(_)) => {
                if p.x.approx_eq_ulps(&p2.x, 4) && p.y.approx_eq_ulps(&p2.y, 4) {
                    let prev = get_prev_vertex(path, idx);
                    calc_line_angle(prev.x, prev.y, p.x, p.y)
                } else {
                    calc_line_angle(p2.x, p2.y, p.x, p.y)
                }
            }
            (Segment::LineTo(p), Segment::Close) => {
                let prev = get_prev_vertex(path, idx);
                let next = get_subpath_start(path, idx);
                calc_angle(prev.x, prev.y, p.x, p.y, p.x, p.y, next.x, next.y)
            }
            (_, Segment::Close) => {
                let prev = get_prev_vertex(path, idx);
                let next = get_subpath_start(path, idx);
                calc_line_angle(prev.x, prev.y, next.x, next.y)
            }
            (_, Segment::MoveTo(_)) | (Segment::Close, _) => 0.0,
        }
    }
}

fn calc_line_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    calc_angle(x1, y1, x2, y2, x1, y1, x2, y2)
}

fn calc_curves_angle(
    px: f32,
    py: f32, // previous vertex
    cx1: f32,
    cy1: f32, // previous control point
    x: f32,
    y: f32, // current vertex
    cx2: f32,
    cy2: f32, // next control point
    nx: f32,
    ny: f32, // next vertex
) -> f32 {
    if cx1.approx_eq_ulps(&x, 4) && cy1.approx_eq_ulps(&y, 4) {
        calc_angle(px, py, x, y, x, y, cx2, cy2)
    } else if x.approx_eq_ulps(&cx2, 4) && y.approx_eq_ulps(&cy2, 4) {
        calc_angle(cx1, cy1, x, y, x, y, nx, ny)
    } else {
        calc_angle(cx1, cy1, x, y, x, y, cx2, cy2)
    }
}

fn calc_angle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> f32 {
    use std::f32::consts::*;

    fn normalize(rad: f32) -> f32 {
        let v = rad % (PI * 2.0);
        if v < 0.0 { v + PI * 2.0 } else { v }
    }

    fn vector_angle(vx: f32, vy: f32) -> f32 {
        let rad = vy.atan2(vx);
        if rad.is_nan() { 0.0 } else { normalize(rad) }
    }

    let in_a = vector_angle(x2 - x1, y2 - y1);
    let out_a = vector_angle(x4 - x3, y4 - y3);
    let d = (out_a - in_a) * 0.5;

    let mut angle = in_a + d;
    if FRAC_PI_2 < d.abs() {
        angle -= PI;
    }

    normalize(angle).to_degrees()
}

fn get_subpath_start(segments: &[Segment], idx: usize) -> tiny_skia_path::Point {
    let offset = segments.len() - idx;
    for seg in segments.iter().rev().skip(offset) {
        if let Segment::MoveTo(p) = *seg {
            return p;
        }
    }

    tiny_skia_path::Point::zero()
}

fn get_prev_vertex(segments: &[Segment], idx: usize) -> tiny_skia_path::Point {
    match segments[idx - 1] {
        Segment::MoveTo(p) => p,
        Segment::LineTo(p) => p,
        Segment::CubicTo(_, _, p) => p,
        Segment::Close => get_subpath_start(segments, idx),
    }
}

fn convert_rect(node: SvgNode, state: &converter::State) -> Option<NonZeroRect> {
    NonZeroRect::from_xywh(
        node.convert_user_length(AId::RefX, state, Length::zero()),
        node.convert_user_length(AId::RefY, state, Length::zero()),
        node.convert_user_length(AId::MarkerWidth, state, Length::new_number(3.0)),
        node.convert_user_length(AId::MarkerHeight, state, Length::new_number(3.0)),
    )
}

fn convert_orientation(node: SvgNode) -> MarkerOrientation {
    match node.attribute(AId::Orient) {
        Some("auto") => MarkerOrientation::Auto,
        Some("auto-start-reverse") => MarkerOrientation::AutoStartReverse,
        _ => match node.attribute::<svgtypes::Angle>(AId::Orient) {
            Some(angle) => MarkerOrientation::Angle(angle.to_degrees() as f32),
            None => MarkerOrientation::Angle(0.0),
        },
    }
}

fn quad_to_curve(prev: Point, p1: Point, p: Point) -> (Point, Point, Point) {
    #[inline]
    fn calc(n1: f32, n2: f32) -> f32 {
        (n1 + n2 * 2.0) / 3.0
    }

    (
        Point::from_xy(calc(prev.x, p1.x), calc(prev.y, p1.y)),
        Point::from_xy(calc(p.x, p1.x), calc(p.y, p1.y)),
        p,
    )
}
