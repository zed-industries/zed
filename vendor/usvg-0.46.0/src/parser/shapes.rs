// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::sync::Arc;

use svgtypes::Length;
use tiny_skia_path::Path;

use super::svgtree::{AId, EId, SvgNode};
use super::{converter, units};
use crate::{ApproxEqUlps, IsValidLength, Rect};

pub(crate) fn convert(node: SvgNode, state: &converter::State) -> Option<Arc<Path>> {
    match node.tag_name()? {
        EId::Rect => convert_rect(node, state),
        EId::Circle => convert_circle(node, state),
        EId::Ellipse => convert_ellipse(node, state),
        EId::Line => convert_line(node, state),
        EId::Polyline => convert_polyline(node),
        EId::Polygon => convert_polygon(node),
        EId::Path => convert_path(node),
        _ => None,
    }
}

pub(crate) fn convert_path(node: SvgNode) -> Option<Arc<Path>> {
    let value: &str = node.attribute(AId::D)?;
    let mut builder = tiny_skia_path::PathBuilder::new();
    for segment in svgtypes::SimplifyingPathParser::from(value) {
        let segment = match segment {
            Ok(v) => v,
            Err(_) => break,
        };

        match segment {
            svgtypes::SimplePathSegment::MoveTo { x, y } => {
                builder.move_to(x as f32, y as f32);
            }
            svgtypes::SimplePathSegment::LineTo { x, y } => {
                builder.line_to(x as f32, y as f32);
            }
            svgtypes::SimplePathSegment::Quadratic { x1, y1, x, y } => {
                builder.quad_to(x1 as f32, y1 as f32, x as f32, y as f32);
            }
            svgtypes::SimplePathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                builder.cubic_to(
                    x1 as f32, y1 as f32, x2 as f32, y2 as f32, x as f32, y as f32,
                );
            }
            svgtypes::SimplePathSegment::ClosePath => {
                builder.close();
            }
        }
    }

    builder.finish().map(Arc::new)
}

fn convert_rect(node: SvgNode, state: &converter::State) -> Option<Arc<Path>> {
    // 'width' and 'height' attributes must be positive and non-zero.
    let width = node.convert_user_length(AId::Width, state, Length::zero());
    let height = node.convert_user_length(AId::Height, state, Length::zero());
    if !width.is_valid_length() {
        log::warn!(
            "Rect '{}' has an invalid 'width' value. Skipped.",
            node.element_id()
        );
        return None;
    }
    if !height.is_valid_length() {
        log::warn!(
            "Rect '{}' has an invalid 'height' value. Skipped.",
            node.element_id()
        );
        return None;
    }

    let x = node.convert_user_length(AId::X, state, Length::zero());
    let y = node.convert_user_length(AId::Y, state, Length::zero());

    let (mut rx, mut ry) = resolve_rx_ry(node, state);

    // Clamp rx/ry to the half of the width/height.
    //
    // Should be done only after resolving.
    if rx > width / 2.0 {
        rx = width / 2.0;
    }
    if ry > height / 2.0 {
        ry = height / 2.0;
    }

    // Conversion according to https://www.w3.org/TR/SVG11/shapes.html#RectElement
    let path = if rx.approx_eq_ulps(&0.0, 4) {
        tiny_skia_path::PathBuilder::from_rect(Rect::from_xywh(x, y, width, height)?)
    } else {
        let mut builder = tiny_skia_path::PathBuilder::new();
        builder.move_to(x + rx, y);

        builder.line_to(x + width - rx, y);
        builder.arc_to(rx, ry, 0.0, false, true, x + width, y + ry);

        builder.line_to(x + width, y + height - ry);
        builder.arc_to(rx, ry, 0.0, false, true, x + width - rx, y + height);

        builder.line_to(x + rx, y + height);
        builder.arc_to(rx, ry, 0.0, false, true, x, y + height - ry);

        builder.line_to(x, y + ry);
        builder.arc_to(rx, ry, 0.0, false, true, x + rx, y);

        builder.close();

        builder.finish()?
    };

    Some(Arc::new(path))
}

fn resolve_rx_ry(node: SvgNode, state: &converter::State) -> (f32, f32) {
    let mut rx_opt = node.attribute::<Length>(AId::Rx);
    let mut ry_opt = node.attribute::<Length>(AId::Ry);

    // Remove negative values first.
    if let Some(v) = rx_opt {
        if v.number.is_sign_negative() {
            rx_opt = None;
        }
    }
    if let Some(v) = ry_opt {
        if v.number.is_sign_negative() {
            ry_opt = None;
        }
    }

    // Resolve.
    match (rx_opt, ry_opt) {
        (None, None) => (0.0, 0.0),
        (Some(rx), None) => {
            let rx = units::convert_user_length(rx, node, AId::Rx, state);
            (rx, rx)
        }
        (None, Some(ry)) => {
            let ry = units::convert_user_length(ry, node, AId::Ry, state);
            (ry, ry)
        }
        (Some(rx), Some(ry)) => {
            let rx = units::convert_user_length(rx, node, AId::Rx, state);
            let ry = units::convert_user_length(ry, node, AId::Ry, state);
            (rx, ry)
        }
    }
}

fn convert_line(node: SvgNode, state: &converter::State) -> Option<Arc<Path>> {
    let x1 = node.convert_user_length(AId::X1, state, Length::zero());
    let y1 = node.convert_user_length(AId::Y1, state, Length::zero());
    let x2 = node.convert_user_length(AId::X2, state, Length::zero());
    let y2 = node.convert_user_length(AId::Y2, state, Length::zero());

    let mut builder = tiny_skia_path::PathBuilder::new();
    builder.move_to(x1, y1);
    builder.line_to(x2, y2);
    builder.finish().map(Arc::new)
}

fn convert_polyline(node: SvgNode) -> Option<Arc<Path>> {
    let builder = points_to_path(node, "Polyline")?;
    builder.finish().map(Arc::new)
}

fn convert_polygon(node: SvgNode) -> Option<Arc<Path>> {
    let mut builder = points_to_path(node, "Polygon")?;
    builder.close();
    builder.finish().map(Arc::new)
}

fn points_to_path(node: SvgNode, eid: &str) -> Option<tiny_skia_path::PathBuilder> {
    use svgtypes::PointsParser;

    let mut builder = tiny_skia_path::PathBuilder::new();
    match node.attribute::<&str>(AId::Points) {
        Some(text) => {
            for (x, y) in PointsParser::from(text) {
                if builder.is_empty() {
                    builder.move_to(x as f32, y as f32);
                } else {
                    builder.line_to(x as f32, y as f32);
                }
            }
        }
        _ => {
            log::warn!(
                "{} '{}' has an invalid 'points' value. Skipped.",
                eid,
                node.element_id()
            );
            return None;
        }
    };

    // 'polyline' and 'polygon' elements must contain at least 2 points.
    if builder.len() < 2 {
        log::warn!(
            "{} '{}' has less than 2 points. Skipped.",
            eid,
            node.element_id()
        );
        return None;
    }

    Some(builder)
}

fn convert_circle(node: SvgNode, state: &converter::State) -> Option<Arc<Path>> {
    let cx = node.convert_user_length(AId::Cx, state, Length::zero());
    let cy = node.convert_user_length(AId::Cy, state, Length::zero());
    let r = node.convert_user_length(AId::R, state, Length::zero());

    if !r.is_valid_length() {
        log::warn!(
            "Circle '{}' has an invalid 'r' value. Skipped.",
            node.element_id()
        );
        return None;
    }

    ellipse_to_path(cx, cy, r, r)
}

fn convert_ellipse(node: SvgNode, state: &converter::State) -> Option<Arc<Path>> {
    let cx = node.convert_user_length(AId::Cx, state, Length::zero());
    let cy = node.convert_user_length(AId::Cy, state, Length::zero());
    let (rx, ry) = resolve_rx_ry(node, state);

    if !rx.is_valid_length() {
        log::warn!(
            "Ellipse '{}' has an invalid 'rx' value. Skipped.",
            node.element_id()
        );
        return None;
    }

    if !ry.is_valid_length() {
        log::warn!(
            "Ellipse '{}' has an invalid 'ry' value. Skipped.",
            node.element_id()
        );
        return None;
    }

    ellipse_to_path(cx, cy, rx, ry)
}

fn ellipse_to_path(cx: f32, cy: f32, rx: f32, ry: f32) -> Option<Arc<Path>> {
    let mut builder = tiny_skia_path::PathBuilder::new();
    builder.move_to(cx + rx, cy);
    builder.arc_to(rx, ry, 0.0, false, true, cx, cy + ry);
    builder.arc_to(rx, ry, 0.0, false, true, cx - rx, cy);
    builder.arc_to(rx, ry, 0.0, false, true, cx, cy - ry);
    builder.arc_to(rx, ry, 0.0, false, true, cx + rx, cy);
    builder.close();
    builder.finish().map(Arc::new)
}

trait PathBuilderExt {
    fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32,
        y: f32,
    );
}

impl PathBuilderExt for tiny_skia_path::PathBuilder {
    fn arc_to(
        &mut self,
        rx: f32,
        ry: f32,
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32,
        y: f32,
    ) {
        let prev = match self.last_point() {
            Some(v) => v,
            None => return,
        };

        let svg_arc = kurbo::SvgArc {
            from: kurbo::Point::new(prev.x as f64, prev.y as f64),
            to: kurbo::Point::new(x as f64, y as f64),
            radii: kurbo::Vec2::new(rx as f64, ry as f64),
            x_rotation: (x_axis_rotation as f64).to_radians(),
            large_arc,
            sweep,
        };

        match kurbo::Arc::from_svg_arc(&svg_arc) {
            Some(arc) => {
                arc.to_cubic_beziers(0.1, |p1, p2, p| {
                    self.cubic_to(
                        p1.x as f32,
                        p1.y as f32,
                        p2.x as f32,
                        p2.y as f32,
                        p.x as f32,
                        p.y as f32,
                    );
                });
            }
            None => {
                self.line_to(x, y);
            }
        }
    }
}
