// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::converter::{self, SvgColorExt};
use super::paint_server;
use super::svgtree::{AId, FromValue, SvgNode};
use crate::tree::ContextElement;
use crate::{
    ApproxEqUlps, Color, Fill, FillRule, LineCap, LineJoin, Opacity, Paint, Stroke,
    StrokeMiterlimit, Units,
};

impl<'a, 'input: 'a> FromValue<'a, 'input> for LineCap {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "butt" => Some(LineCap::Butt),
            "round" => Some(LineCap::Round),
            "square" => Some(LineCap::Square),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for LineJoin {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "miter" => Some(LineJoin::Miter),
            "miter-clip" => Some(LineJoin::MiterClip),
            "round" => Some(LineJoin::Round),
            "bevel" => Some(LineJoin::Bevel),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for FillRule {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "nonzero" => Some(FillRule::NonZero),
            "evenodd" => Some(FillRule::EvenOdd),
            _ => None,
        }
    }
}

pub(crate) fn resolve_fill(
    node: SvgNode,
    has_bbox: bool,
    state: &converter::State,
    cache: &mut converter::Cache,
) -> Option<Fill> {
    if state.parent_clip_path.is_some() {
        // A `clipPath` child can be filled only with a black color.
        return Some(Fill {
            paint: Paint::Color(Color::black()),
            opacity: Opacity::ONE,
            rule: node.find_attribute(AId::ClipRule).unwrap_or_default(),
            context_element: None,
        });
    }

    let mut sub_opacity = Opacity::ONE;
    let (paint, context_element) =
        if let Some(n) = node.ancestors().find(|n| n.has_attribute(AId::Fill)) {
            let value: &str = n.attribute(AId::Fill)?;
            convert_paint(
                node,
                value,
                AId::Fill,
                has_bbox,
                state,
                &mut sub_opacity,
                cache,
            )?
        } else {
            (Paint::Color(Color::black()), None)
        };

    let fill_opacity = node
        .find_attribute::<Opacity>(AId::FillOpacity)
        .unwrap_or(Opacity::ONE);

    Some(Fill {
        paint,
        opacity: sub_opacity * fill_opacity,
        rule: node.find_attribute(AId::FillRule).unwrap_or_default(),
        context_element,
    })
}

pub(crate) fn resolve_stroke(
    node: SvgNode,
    has_bbox: bool,
    state: &converter::State,
    cache: &mut converter::Cache,
) -> Option<Stroke> {
    if state.parent_clip_path.is_some() {
        // A `clipPath` child cannot be stroked.
        return None;
    }

    let mut sub_opacity = Opacity::ONE;
    let (paint, context_element) =
        if let Some(n) = node.ancestors().find(|n| n.has_attribute(AId::Stroke)) {
            let value: &str = n.attribute(AId::Stroke)?;

            convert_paint(
                node,
                value,
                AId::Stroke,
                has_bbox,
                state,
                &mut sub_opacity,
                cache,
            )?
        } else {
            return None;
        };

    let width = node.resolve_valid_length(AId::StrokeWidth, state, 1.0)?;

    // Must be bigger than 1.
    let miterlimit = node.find_attribute(AId::StrokeMiterlimit).unwrap_or(4.0);
    let miterlimit = if miterlimit < 1.0 { 1.0 } else { miterlimit };
    let miterlimit = StrokeMiterlimit::new(miterlimit);

    let stroke_opacity = node
        .find_attribute::<Opacity>(AId::StrokeOpacity)
        .unwrap_or(Opacity::ONE);

    let stroke = Stroke {
        paint,
        dasharray: conv_dasharray(node, state),
        dashoffset: node.resolve_length(AId::StrokeDashoffset, state, 0.0),
        miterlimit,
        opacity: sub_opacity * stroke_opacity,
        width,
        linecap: node.find_attribute(AId::StrokeLinecap).unwrap_or_default(),
        linejoin: node.find_attribute(AId::StrokeLinejoin).unwrap_or_default(),
        context_element,
    };

    Some(stroke)
}

fn convert_paint(
    node: SvgNode,
    value: &str,
    aid: AId,
    has_bbox: bool,
    state: &converter::State,
    opacity: &mut Opacity,
    cache: &mut converter::Cache,
) -> Option<(Paint, Option<ContextElement>)> {
    let paint = match svgtypes::Paint::from_str(value) {
        Ok(v) => v,
        Err(_) => {
            if aid == AId::Fill {
                log::warn!(
                    "Failed to parse fill value: '{}'. Fallback to black.",
                    value
                );
                svgtypes::Paint::Color(svgtypes::Color::black())
            } else if aid == AId::Stroke {
                log::warn!(
                    "Failed to parse stroke value: '{}'. Fallback to no stroke.",
                    value
                );
                return None;
            } else {
                return None;
            }
        }
    };

    match paint {
        svgtypes::Paint::None => None,
        svgtypes::Paint::Inherit => None, // already resolved by svgtree
        svgtypes::Paint::ContextFill => state
            .context_element
            .clone()
            .and_then(|(f, _)| f)
            .map(|f| (f.paint, f.context_element)),
        svgtypes::Paint::ContextStroke => state
            .context_element
            .clone()
            .and_then(|(_, s)| s)
            .map(|s| (s.paint, s.context_element)),
        svgtypes::Paint::CurrentColor => {
            let svg_color: svgtypes::Color = node
                .find_attribute(AId::Color)
                .unwrap_or_else(svgtypes::Color::black);
            let (color, alpha) = svg_color.split_alpha();
            *opacity = alpha;
            Some((Paint::Color(color), None))
        }
        svgtypes::Paint::Color(svg_color) => {
            let (color, alpha) = svg_color.split_alpha();
            *opacity = alpha;
            Some((Paint::Color(color), None))
        }
        svgtypes::Paint::FuncIRI(func_iri, fallback) => {
            if let Some(link) = node.document().element_by_id(func_iri) {
                let tag_name = link.tag_name().unwrap();
                if tag_name.is_paint_server() {
                    match paint_server::convert(link, state, cache) {
                        Some(paint_server::ServerOrColor::Server(paint)) => {
                            // We can use a paint server node with ObjectBoundingBox units
                            // for painting only when the shape itself has a bbox.
                            //
                            // See SVG spec 7.11 for details.

                            if !has_bbox && paint.units() == Units::ObjectBoundingBox {
                                from_fallback(node, fallback, opacity).map(|p| (p, None))
                            } else {
                                Some((paint, None))
                            }
                        }
                        Some(paint_server::ServerOrColor::Color { color, opacity: so }) => {
                            *opacity = so;
                            Some((Paint::Color(color), None))
                        }
                        None => from_fallback(node, fallback, opacity).map(|p| (p, None)),
                    }
                } else {
                    log::warn!("'{}' cannot be used to {} a shape.", tag_name, aid);
                    None
                }
            } else {
                from_fallback(node, fallback, opacity).map(|p| (p, None))
            }
        }
    }
}

fn from_fallback(
    node: SvgNode,
    fallback: Option<svgtypes::PaintFallback>,
    opacity: &mut Opacity,
) -> Option<Paint> {
    match fallback? {
        svgtypes::PaintFallback::None => None,
        svgtypes::PaintFallback::CurrentColor => {
            let svg_color: svgtypes::Color = node
                .find_attribute(AId::Color)
                .unwrap_or_else(svgtypes::Color::black);
            let (color, alpha) = svg_color.split_alpha();
            *opacity = alpha;
            Some(Paint::Color(color))
        }
        svgtypes::PaintFallback::Color(svg_color) => {
            let (color, alpha) = svg_color.split_alpha();
            *opacity = alpha;
            Some(Paint::Color(color))
        }
    }
}

// Prepare the 'stroke-dasharray' according to:
// https://www.w3.org/TR/SVG11/painting.html#StrokeDasharrayProperty
fn conv_dasharray(node: SvgNode, state: &converter::State) -> Option<Vec<f32>> {
    let node = node
        .ancestors()
        .find(|n| n.has_attribute(AId::StrokeDasharray))?;
    let list = super::units::convert_list(node, AId::StrokeDasharray, state)?;

    // `A negative value is an error`
    if list.iter().any(|n| n.is_sign_negative()) {
        return None;
    }

    // `If the sum of the values is zero, then the stroke is rendered
    // as if a value of none were specified.`
    {
        // no Iter::sum(), because of f64

        let mut sum: f32 = 0.0;
        for n in list.iter() {
            sum += *n;
        }

        if sum.approx_eq_ulps(&0.0, 4) {
            return None;
        }
    }

    // `If an odd number of values is provided, then the list of values
    // is repeated to yield an even number of values.`
    if list.len() % 2 != 0 {
        let mut tmp_list = list.clone();
        tmp_list.extend_from_slice(&list);
        return Some(tmp_list);
    }

    Some(list)
}
