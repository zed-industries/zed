// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::str::FromStr;
use std::sync::Arc;

use strict_num::PositiveF32;
use svgtypes::{Length, LengthUnit as Unit};

use super::OptionLog;
use super::converter::{self, Cache, SvgColorExt};
use super::svgtree::{AId, EId, SvgNode};
use crate::*;

pub(crate) enum ServerOrColor {
    Server(Paint),
    Color { color: Color, opacity: Opacity },
}

pub(crate) fn convert(
    node: SvgNode,
    state: &converter::State,
    cache: &mut converter::Cache,
) -> Option<ServerOrColor> {
    // Check for existing.
    if let Some(paint) = cache.paint.get(node.element_id()) {
        return Some(ServerOrColor::Server(paint.clone()));
    }

    // Unwrap is safe, because we already checked for is_paint_server().
    let paint = match node.tag_name().unwrap() {
        EId::LinearGradient => convert_linear(node, state),
        EId::RadialGradient => convert_radial(node, state),
        EId::Pattern => convert_pattern(node, state, cache),
        _ => unreachable!(),
    };

    if let Some(ServerOrColor::Server(paint)) = &paint {
        cache
            .paint
            .insert(node.element_id().to_string(), paint.clone());
    }

    paint
}

#[inline(never)]
fn convert_linear(node: SvgNode, state: &converter::State) -> Option<ServerOrColor> {
    let id = NonEmptyString::new(node.element_id().to_string())?;

    let stops = convert_stops(find_gradient_with_stops(node)?);
    if stops.len() < 2 {
        return stops_to_color(&stops);
    }

    let units = convert_units(node, AId::GradientUnits, Units::ObjectBoundingBox);
    let transform = node.resolve_transform(AId::GradientTransform, state);

    let gradient = LinearGradient {
        x1: resolve_number(node, AId::X1, units, state, Length::zero()),
        y1: resolve_number(node, AId::Y1, units, state, Length::zero()),
        x2: resolve_number(
            node,
            AId::X2,
            units,
            state,
            Length::new(100.0, Unit::Percent),
        ),
        y2: resolve_number(node, AId::Y2, units, state, Length::zero()),
        base: BaseGradient {
            id,
            units,
            transform,
            spread_method: convert_spread_method(node),
            stops,
        },
    };

    Some(ServerOrColor::Server(Paint::LinearGradient(Arc::new(
        gradient,
    ))))
}

#[inline(never)]
fn convert_radial(node: SvgNode, state: &converter::State) -> Option<ServerOrColor> {
    let id = NonEmptyString::new(node.element_id().to_string())?;

    let stops = convert_stops(find_gradient_with_stops(node)?);
    if stops.len() < 2 {
        return stops_to_color(&stops);
    }

    let units = convert_units(node, AId::GradientUnits, Units::ObjectBoundingBox);
    let r = resolve_number(node, AId::R, units, state, Length::new(50.0, Unit::Percent));

    // 'A value of zero will cause the area to be painted as a single color
    // using the color and opacity of the last gradient stop.'
    //
    // https://www.w3.org/TR/SVG11/pservers.html#RadialGradientElementRAttribute
    if !r.is_valid_length() {
        let stop = stops.last().unwrap();
        return Some(ServerOrColor::Color {
            color: stop.color,
            opacity: stop.opacity,
        });
    }

    let spread_method = convert_spread_method(node);
    let cx = resolve_number(
        node,
        AId::Cx,
        units,
        state,
        Length::new(50.0, Unit::Percent),
    );
    let cy = resolve_number(
        node,
        AId::Cy,
        units,
        state,
        Length::new(50.0, Unit::Percent),
    );
    let fx = resolve_number(node, AId::Fx, units, state, Length::new_number(cx as f64));
    let fy = resolve_number(node, AId::Fy, units, state, Length::new_number(cy as f64));
    let transform = node.resolve_transform(AId::GradientTransform, state);

    let gradient = RadialGradient {
        cx,
        cy,
        r: PositiveF32::new(r).unwrap(),
        fx,
        fy,
        base: BaseGradient {
            id,
            units,
            transform,
            spread_method,
            stops,
        },
    };

    Some(ServerOrColor::Server(Paint::RadialGradient(Arc::new(
        gradient,
    ))))
}

#[inline(never)]
fn convert_pattern(
    node: SvgNode,
    state: &converter::State,
    cache: &mut converter::Cache,
) -> Option<ServerOrColor> {
    let node_with_children = find_pattern_with_children(node)?;

    let id = NonEmptyString::new(node.element_id().to_string())?;

    let view_box = {
        let n1 = resolve_attr(node, AId::ViewBox);
        let n2 = resolve_attr(node, AId::PreserveAspectRatio);
        n1.parse_viewbox().map(|vb| ViewBox {
            rect: vb,
            aspect: n2.attribute(AId::PreserveAspectRatio).unwrap_or_default(),
        })
    };

    let units = convert_units(node, AId::PatternUnits, Units::ObjectBoundingBox);
    let content_units = convert_units(node, AId::PatternContentUnits, Units::UserSpaceOnUse);

    let transform = node.resolve_transform(AId::PatternTransform, state);

    let rect = NonZeroRect::from_xywh(
        resolve_number(node, AId::X, units, state, Length::zero()),
        resolve_number(node, AId::Y, units, state, Length::zero()),
        resolve_number(node, AId::Width, units, state, Length::zero()),
        resolve_number(node, AId::Height, units, state, Length::zero()),
    );
    let rect = rect.log_none(|| {
        log::warn!(
            "Pattern '{}' has an invalid size. Skipped.",
            node.element_id()
        );
    })?;

    let mut patt = Pattern {
        id,
        units,
        content_units,
        transform,
        rect,
        view_box,
        root: Group::empty(),
    };

    // We can apply viewbox transform only for user space coordinates.
    // Otherwise we need a bounding box, which is unknown at this point.
    if patt.view_box.is_some()
        && patt.units == Units::UserSpaceOnUse
        && patt.content_units == Units::UserSpaceOnUse
    {
        let mut g = Group::empty();
        g.transform = view_box.unwrap().to_transform(rect.size());
        g.abs_transform = g.transform;

        converter::convert_children(node_with_children, state, cache, &mut g);
        if !g.has_children() {
            return None;
        }

        g.calculate_bounding_boxes();
        patt.root.children.push(Node::Group(Box::new(g)));
    } else {
        converter::convert_children(node_with_children, state, cache, &mut patt.root);
        if !patt.root.has_children() {
            return None;
        }
    }

    patt.root.calculate_bounding_boxes();

    Some(ServerOrColor::Server(Paint::Pattern(Arc::new(patt))))
}

fn convert_spread_method(node: SvgNode) -> SpreadMethod {
    let node = resolve_attr(node, AId::SpreadMethod);
    node.attribute(AId::SpreadMethod).unwrap_or_default()
}

pub(crate) fn convert_units(node: SvgNode, name: AId, def: Units) -> Units {
    let node = resolve_attr(node, name);
    node.attribute(name).unwrap_or(def)
}

fn find_gradient_with_stops<'a, 'input: 'a>(
    node: SvgNode<'a, 'input>,
) -> Option<SvgNode<'a, 'input>> {
    for link in node.href_iter() {
        if !link.tag_name().unwrap().is_gradient() {
            log::warn!(
                "Gradient '{}' cannot reference '{}' via 'xlink:href'.",
                node.element_id(),
                link.tag_name().unwrap()
            );
            return None;
        }

        if link.children().any(|n| n.tag_name() == Some(EId::Stop)) {
            return Some(link);
        }
    }

    None
}

fn find_pattern_with_children<'a, 'input: 'a>(
    node: SvgNode<'a, 'input>,
) -> Option<SvgNode<'a, 'input>> {
    for link in node.href_iter() {
        if link.tag_name() != Some(EId::Pattern) {
            log::warn!(
                "Pattern '{}' cannot reference '{}' via 'xlink:href'.",
                node.element_id(),
                link.tag_name().unwrap()
            );
            return None;
        }

        if link.has_children() {
            return Some(link);
        }
    }

    None
}

fn convert_stops(grad: SvgNode) -> Vec<Stop> {
    let mut stops = Vec::new();

    {
        let mut prev_offset = Length::zero();
        for stop in grad.children() {
            if stop.tag_name() != Some(EId::Stop) {
                log::warn!("Invalid gradient child: '{:?}'.", stop.tag_name().unwrap());
                continue;
            }

            // `number` can be either a number or a percentage.
            let offset = stop.attribute(AId::Offset).unwrap_or(prev_offset);
            let offset = match offset.unit {
                Unit::None => offset.number,
                Unit::Percent => offset.number / 100.0,
                _ => prev_offset.number,
            };
            prev_offset = Length::new_number(offset);
            let offset = crate::f32_bound(0.0, offset as f32, 1.0);

            let (color, opacity) = match stop.attribute(AId::StopColor) {
                Some("currentColor") => stop
                    .find_attribute(AId::Color)
                    .unwrap_or_else(svgtypes::Color::black),
                Some(value) => {
                    if let Ok(c) = svgtypes::Color::from_str(value) {
                        c
                    } else {
                        log::warn!("Failed to parse stop-color value: '{}'.", value);
                        svgtypes::Color::black()
                    }
                }
                _ => svgtypes::Color::black(),
            }
            .split_alpha();

            let stop_opacity = stop
                .attribute::<Opacity>(AId::StopOpacity)
                .unwrap_or(Opacity::ONE);
            stops.push(Stop {
                offset: StopOffset::new_clamped(offset),
                color,
                opacity: opacity * stop_opacity,
            });
        }
    }

    // Remove stops with equal offset.
    //
    // Example:
    // offset="0.5"
    // offset="0.7"
    // offset="0.7" <-- this one should be removed
    // offset="0.7"
    // offset="0.9"
    if stops.len() >= 3 {
        let mut i = 0;
        while i < stops.len() - 2 {
            let offset1 = stops[i + 0].offset.get();
            let offset2 = stops[i + 1].offset.get();
            let offset3 = stops[i + 2].offset.get();

            if offset1.approx_eq_ulps(&offset2, 4) && offset2.approx_eq_ulps(&offset3, 4) {
                // Remove offset in the middle.
                stops.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    // Remove zeros.
    //
    // From:
    // offset="0.0"
    // offset="0.0"
    // offset="0.7"
    //
    // To:
    // offset="0.0"
    // offset="0.00000001"
    // offset="0.7"
    if stops.len() >= 2 {
        let mut i = 0;
        while i < stops.len() - 1 {
            let offset1 = stops[i + 0].offset.get();
            let offset2 = stops[i + 1].offset.get();

            if offset1.approx_eq_ulps(&0.0, 4) && offset2.approx_eq_ulps(&0.0, 4) {
                stops[i + 1].offset = StopOffset::new_clamped(offset1 + f32::EPSILON);
            }

            i += 1;
        }
    }

    // Shift equal offsets.
    //
    // From:
    // offset="0.5"
    // offset="0.7"
    // offset="0.7"
    //
    // To:
    // offset="0.5"
    // offset="0.699999999"
    // offset="0.7"
    {
        let mut i = 1;
        while i < stops.len() {
            let offset1 = stops[i - 1].offset.get();
            let offset2 = stops[i - 0].offset.get();

            // Next offset must be smaller then previous.
            if offset1 > offset2 || offset1.approx_eq_ulps(&offset2, 4) {
                // Make previous offset a bit smaller.
                let new_offset = offset1 - f32::EPSILON;
                stops[i - 1].offset = StopOffset::new_clamped(new_offset);
                stops[i - 0].offset = StopOffset::new_clamped(offset1);
            }

            i += 1;
        }
    }

    stops
}

#[inline(never)]
pub(crate) fn resolve_number(
    node: SvgNode,
    name: AId,
    units: Units,
    state: &converter::State,
    def: Length,
) -> f32 {
    resolve_attr(node, name).convert_length(name, units, state, def)
}

fn resolve_attr<'a, 'input: 'a>(node: SvgNode<'a, 'input>, name: AId) -> SvgNode<'a, 'input> {
    if node.has_attribute(name) {
        return node;
    }

    match node.tag_name().unwrap() {
        EId::LinearGradient => resolve_lg_attr(node, name),
        EId::RadialGradient => resolve_rg_attr(node, name),
        EId::Pattern => resolve_pattern_attr(node, name),
        EId::Filter => resolve_filter_attr(node, name),
        _ => node,
    }
}

fn resolve_lg_attr<'a, 'input: 'a>(node: SvgNode<'a, 'input>, name: AId) -> SvgNode<'a, 'input> {
    for link in node.href_iter() {
        let tag_name = match link.tag_name() {
            Some(v) => v,
            None => return node,
        };

        match (name, tag_name) {
            // Coordinates can be resolved only from
            // ref element with the same type.
              (AId::X1, EId::LinearGradient)
            | (AId::Y1, EId::LinearGradient)
            | (AId::X2, EId::LinearGradient)
            | (AId::Y2, EId::LinearGradient)
            // Other attributes can be resolved
            // from any kind of gradient.
            | (AId::GradientUnits, EId::LinearGradient)
            | (AId::GradientUnits, EId::RadialGradient)
            | (AId::SpreadMethod, EId::LinearGradient)
            | (AId::SpreadMethod, EId::RadialGradient)
            | (AId::GradientTransform, EId::LinearGradient)
            | (AId::GradientTransform, EId::RadialGradient) => {
                if link.has_attribute(name) {
                    return link;
                }
            }
            _ => break,
        }
    }

    node
}

fn resolve_rg_attr<'a, 'input>(node: SvgNode<'a, 'input>, name: AId) -> SvgNode<'a, 'input> {
    for link in node.href_iter() {
        let tag_name = match link.tag_name() {
            Some(v) => v,
            None => return node,
        };

        match (name, tag_name) {
            // Coordinates can be resolved only from
            // ref element with the same type.
              (AId::Cx, EId::RadialGradient)
            | (AId::Cy, EId::RadialGradient)
            | (AId::R,  EId::RadialGradient)
            | (AId::Fx, EId::RadialGradient)
            | (AId::Fy, EId::RadialGradient)
            // Other attributes can be resolved
            // from any kind of gradient.
            | (AId::GradientUnits, EId::LinearGradient)
            | (AId::GradientUnits, EId::RadialGradient)
            | (AId::SpreadMethod, EId::LinearGradient)
            | (AId::SpreadMethod, EId::RadialGradient)
            | (AId::GradientTransform, EId::LinearGradient)
            | (AId::GradientTransform, EId::RadialGradient) => {
                if link.has_attribute(name) {
                    return link;
                }
            }
            _ => break,
        }
    }

    node
}

fn resolve_pattern_attr<'a, 'input: 'a>(
    node: SvgNode<'a, 'input>,
    name: AId,
) -> SvgNode<'a, 'input> {
    for link in node.href_iter() {
        let tag_name = match link.tag_name() {
            Some(v) => v,
            None => return node,
        };

        if tag_name != EId::Pattern {
            break;
        }

        if link.has_attribute(name) {
            return link;
        }
    }

    node
}

fn resolve_filter_attr<'a, 'input: 'a>(node: SvgNode<'a, 'input>, aid: AId) -> SvgNode<'a, 'input> {
    for link in node.href_iter() {
        let tag_name = match link.tag_name() {
            Some(v) => v,
            None => return node,
        };

        if tag_name != EId::Filter {
            break;
        }

        if link.has_attribute(aid) {
            return link;
        }
    }

    node
}

fn stops_to_color(stops: &[Stop]) -> Option<ServerOrColor> {
    if stops.is_empty() {
        None
    } else {
        Some(ServerOrColor::Color {
            color: stops[0].color,
            opacity: stops[0].opacity,
        })
    }
}

// Update paints servers by doing the following:
// 1. Replace context fills/strokes that are linked to
// a use node with their actual values.
// 2. Convert all object units to UserSpaceOnUse
pub fn update_paint_servers(
    group: &mut Group,
    context_transform: Transform,
    context_bbox: Option<Rect>,
    text_bbox: Option<Rect>,
    cache: &mut Cache,
) {
    for child in &mut group.children {
        // Set context transform and bbox if applicable if the
        // current group is a use node.
        let (context_transform, context_bbox) = if group.is_context_element {
            (group.abs_transform, Some(group.bounding_box))
        } else {
            (context_transform, context_bbox)
        };

        node_to_user_coordinates(child, context_transform, context_bbox, text_bbox, cache);
    }
}

// When parsing clipPaths, masks and filters we already know group's bounding box.
// But with gradients and patterns we don't, because we have to know text bounding box
// before we even parsed it. Which is impossible.
// Therefore our only choice is to parse gradients and patterns preserving their units
// and then replace them with `userSpaceOnUse` after the whole tree parsing is finished.
// So while gradients and patterns do still store their units,
// they are not exposed in the public API and for the caller they are always `userSpaceOnUse`.
fn node_to_user_coordinates(
    node: &mut Node,
    context_transform: Transform,
    context_bbox: Option<Rect>,
    text_bbox: Option<Rect>,
    cache: &mut Cache,
) {
    match node {
        Node::Group(g) => {
            // No need to check clip paths, because they cannot have paint servers.
            if let Some(mask) = &mut g.mask {
                if let Some(mask) = Arc::get_mut(mask) {
                    update_paint_servers(
                        &mut mask.root,
                        context_transform,
                        context_bbox,
                        None,
                        cache,
                    );

                    if let Some(sub_mask) = &mut mask.mask {
                        if let Some(sub_mask) = Arc::get_mut(sub_mask) {
                            update_paint_servers(
                                &mut sub_mask.root,
                                context_transform,
                                context_bbox,
                                None,
                                cache,
                            );
                        }
                    }
                }
            }

            for filter in &mut g.filters {
                if let Some(filter) = Arc::get_mut(filter) {
                    for primitive in &mut filter.primitives {
                        if let filter::Kind::Image(image) = &mut primitive.kind {
                            update_paint_servers(
                                &mut image.root,
                                context_transform,
                                context_bbox,
                                None,
                                cache,
                            );
                        }
                    }
                }
            }

            update_paint_servers(g, context_transform, context_bbox, text_bbox, cache);
        }
        Node::Path(path) => {
            // Paths inside `Text::flattened` are special and must use text's bounding box
            // instead of their own.
            let bbox = text_bbox.unwrap_or(path.bounding_box);

            process_fill(
                &mut path.fill,
                path.abs_transform,
                context_transform,
                context_bbox,
                bbox,
                cache,
            );
            process_stroke(
                &mut path.stroke,
                path.abs_transform,
                context_transform,
                context_bbox,
                bbox,
                cache,
            );
        }
        Node::Image(image) => {
            if let ImageKind::SVG(tree) = &mut image.kind {
                update_paint_servers(&mut tree.root, context_transform, context_bbox, None, cache);
            }
        }
        Node::Text(text) => {
            // By the SVG spec, `tspan` doesn't have a bbox and uses the parent `text` bbox.
            // Therefore we have to use text's bbox when converting tspan and flatted text
            // paint servers.
            let bbox = text.bounding_box;

            // We need to update three things:
            // 1. The fills/strokes of the original elements in the usvg tree.
            // 2. The fills/strokes of the layouted elements of the text.
            // 3. The fills/strokes of the outlined text.

            // 1.
            for chunk in &mut text.chunks {
                for span in &mut chunk.spans {
                    process_fill(
                        &mut span.fill,
                        text.abs_transform,
                        context_transform,
                        context_bbox,
                        bbox,
                        cache,
                    );
                    process_stroke(
                        &mut span.stroke,
                        text.abs_transform,
                        context_transform,
                        context_bbox,
                        bbox,
                        cache,
                    );
                    process_text_decoration(&mut span.decoration.underline, bbox, cache);
                    process_text_decoration(&mut span.decoration.overline, bbox, cache);
                    process_text_decoration(&mut span.decoration.line_through, bbox, cache);
                }
            }

            // 2.
            #[cfg(feature = "text")]
            for span in &mut text.layouted {
                process_fill(
                    &mut span.fill,
                    text.abs_transform,
                    context_transform,
                    context_bbox,
                    bbox,
                    cache,
                );
                process_stroke(
                    &mut span.stroke,
                    text.abs_transform,
                    context_transform,
                    context_bbox,
                    bbox,
                    cache,
                );

                let mut process_decoration = |path: &mut Path| {
                    process_fill(
                        &mut path.fill,
                        text.abs_transform,
                        context_transform,
                        context_bbox,
                        bbox,
                        cache,
                    );
                    process_stroke(
                        &mut path.stroke,
                        text.abs_transform,
                        context_transform,
                        context_bbox,
                        bbox,
                        cache,
                    );
                };

                if let Some(path) = &mut span.overline {
                    process_decoration(path);
                }

                if let Some(path) = &mut span.underline {
                    process_decoration(path);
                }

                if let Some(path) = &mut span.line_through {
                    process_decoration(path);
                }
            }

            // 3.
            update_paint_servers(
                &mut text.flattened,
                context_transform,
                context_bbox,
                Some(bbox),
                cache,
            );
        }
    }
}

fn process_fill(
    fill: &mut Option<Fill>,
    path_transform: Transform,
    context_transform: Transform,
    context_bbox: Option<Rect>,
    bbox: Rect,
    cache: &mut Cache,
) {
    let mut ok = false;
    if let Some(fill) = fill.as_mut() {
        // Path context elements (i.e. for  markers) have already been resolved,
        // so we only care about use nodes.
        ok = process_paint(
            &mut fill.paint,
            matches!(fill.context_element, Some(ContextElement::UseNode)),
            context_transform,
            context_bbox,
            path_transform,
            bbox,
            cache,
        );
    }
    if !ok {
        *fill = None;
    }
}

fn process_stroke(
    stroke: &mut Option<Stroke>,
    path_transform: Transform,
    context_transform: Transform,
    context_bbox: Option<Rect>,
    bbox: Rect,
    cache: &mut Cache,
) {
    let mut ok = false;
    if let Some(stroke) = stroke.as_mut() {
        // Path context elements (i.e. for  markers) have already been resolved,
        // so we only care about use nodes.
        ok = process_paint(
            &mut stroke.paint,
            matches!(stroke.context_element, Some(ContextElement::UseNode)),
            context_transform,
            context_bbox,
            path_transform,
            bbox,
            cache,
        );
    }
    if !ok {
        *stroke = None;
    }
}

fn process_context_paint(
    paint: &mut Paint,
    context_transform: Transform,
    path_transform: Transform,
    cache: &mut Cache,
) -> Option<()> {
    // The idea is the following: We have a certain context element that has
    // a transform A, and further below in the tree we have for example a path
    // whose paint has a transform C. In order to get from A to C, there is some
    // transformation matrix B such that A x B = C. We now need to figure out
    // a way to get from C back to A, so that the transformation of the paint
    // matches the one from the context element, even if B was applied. How
    // do we do that? We calculate CxB^(-1), which will overall then have
    // the same effect as A. How do we calculate B^(-1)?
    // --> (A^(-1)xC)^(-1)
    let rev_transform = context_transform
        .invert()?
        .pre_concat(path_transform)
        .invert()?;

    match paint {
        Paint::Color(_) => {}
        Paint::LinearGradient(lg) => {
            let transform = lg.transform.post_concat(rev_transform);
            *paint = Paint::LinearGradient(Arc::new(LinearGradient {
                x1: lg.x1,
                y1: lg.y1,
                x2: lg.x2,
                y2: lg.y2,
                base: BaseGradient {
                    id: cache.gen_linear_gradient_id(),
                    units: lg.units,
                    transform,
                    spread_method: lg.spread_method,
                    stops: lg.stops.clone(),
                },
            }));
        }
        Paint::RadialGradient(rg) => {
            let transform = rg.transform.post_concat(rev_transform);
            *paint = Paint::RadialGradient(Arc::new(RadialGradient {
                cx: rg.cx,
                cy: rg.cy,
                r: rg.r,
                fx: rg.fx,
                fy: rg.fy,
                base: BaseGradient {
                    id: cache.gen_radial_gradient_id(),
                    units: rg.units,
                    transform,
                    spread_method: rg.spread_method,
                    stops: rg.stops.clone(),
                },
            }));
        }
        Paint::Pattern(pat) => {
            let transform = pat.transform.post_concat(rev_transform);
            *paint = Paint::Pattern(Arc::new(Pattern {
                id: cache.gen_pattern_id(),
                units: pat.units,
                content_units: pat.content_units,
                transform,
                rect: pat.rect,
                view_box: pat.view_box,
                root: pat.root.clone(),
            }));
        }
    }

    Some(())
}

pub(crate) fn process_paint(
    paint: &mut Paint,
    has_context: bool,
    context_transform: Transform,
    context_bbox: Option<Rect>,
    path_transform: Transform,
    bbox: Rect,
    cache: &mut Cache,
) -> bool {
    if paint.units() == Units::ObjectBoundingBox
        || paint.content_units() == Units::ObjectBoundingBox
    {
        let bbox = if has_context {
            let Some(bbox) = context_bbox else {
                return false;
            };
            bbox
        } else {
            bbox
        };

        if paint.to_user_coordinates(bbox, cache).is_none() {
            return false;
        }
    }

    if let Paint::Pattern(patt) = paint {
        if let Some(patt) = Arc::get_mut(patt) {
            update_paint_servers(&mut patt.root, Transform::default(), None, None, cache);
        }
    }

    if has_context {
        process_context_paint(paint, context_transform, path_transform, cache);
    }

    true
}

fn process_text_decoration(style: &mut Option<TextDecorationStyle>, bbox: Rect, cache: &mut Cache) {
    if let Some(style) = style.as_mut() {
        process_fill(
            &mut style.fill,
            Transform::default(),
            Transform::default(),
            None,
            bbox,
            cache,
        );
        process_stroke(
            &mut style.stroke,
            Transform::default(),
            Transform::default(),
            None,
            bbox,
            cache,
        );
    }
}

impl Paint {
    fn to_user_coordinates(&mut self, bbox: Rect, cache: &mut Cache) -> Option<()> {
        let name = if matches!(self, Paint::Pattern(_)) {
            "Pattern"
        } else {
            "Gradient"
        };
        let bbox = bbox
            .to_non_zero_rect()
            .log_none(|| log::warn!("{} on zero-sized shapes is not allowed.", name))?;

        // `Arc::get_mut()` allow us to modify some paint servers in-place.
        // This reduces the amount of cloning and preserves the original ID as well.
        match self {
            Paint::Color(_) => {} // unreachable
            Paint::LinearGradient(lg) => {
                let transform = lg.transform.post_concat(Transform::from_bbox(bbox));
                if let Some(lg) = Arc::get_mut(lg) {
                    lg.base.transform = transform;
                    lg.base.units = Units::UserSpaceOnUse;
                } else {
                    *lg = Arc::new(LinearGradient {
                        x1: lg.x1,
                        y1: lg.y1,
                        x2: lg.x2,
                        y2: lg.y2,
                        base: BaseGradient {
                            id: cache.gen_linear_gradient_id(),
                            units: Units::UserSpaceOnUse,
                            transform,
                            spread_method: lg.spread_method,
                            stops: lg.stops.clone(),
                        },
                    });
                }
            }
            Paint::RadialGradient(rg) => {
                let transform = rg.transform.post_concat(Transform::from_bbox(bbox));
                if let Some(rg) = Arc::get_mut(rg) {
                    rg.base.transform = transform;
                    rg.base.units = Units::UserSpaceOnUse;
                } else {
                    *rg = Arc::new(RadialGradient {
                        cx: rg.cx,
                        cy: rg.cy,
                        r: rg.r,
                        fx: rg.fx,
                        fy: rg.fy,
                        base: BaseGradient {
                            id: cache.gen_radial_gradient_id(),
                            units: Units::UserSpaceOnUse,
                            transform,
                            spread_method: rg.spread_method,
                            stops: rg.stops.clone(),
                        },
                    });
                }
            }
            Paint::Pattern(patt) => {
                let rect = if patt.units == Units::ObjectBoundingBox {
                    patt.rect.bbox_transform(bbox)
                } else {
                    patt.rect
                };

                if let Some(patt) = Arc::get_mut(patt) {
                    patt.rect = rect;
                    patt.units = Units::UserSpaceOnUse;

                    if patt.content_units == Units::ObjectBoundingBox && patt.view_box.is_none() {
                        // No need to shift patterns.
                        let transform = Transform::from_scale(bbox.width(), bbox.height());
                        push_pattern_transform(&mut patt.root, transform);
                    }

                    if let Some(view_box) = patt.view_box {
                        push_pattern_transform(&mut patt.root, view_box.to_transform(rect.size()));
                    }

                    patt.content_units = Units::UserSpaceOnUse;
                } else {
                    let mut root = if patt.content_units == Units::ObjectBoundingBox
                        && patt.view_box.is_none()
                    {
                        // No need to shift patterns.
                        let transform = Transform::from_scale(bbox.width(), bbox.height());

                        let mut g = patt.root.clone();
                        push_pattern_transform(&mut g, transform);
                        g
                    } else {
                        patt.root.clone()
                    };

                    if let Some(view_box) = patt.view_box {
                        push_pattern_transform(&mut root, view_box.to_transform(rect.size()));
                    }

                    *patt = Arc::new(Pattern {
                        id: cache.gen_pattern_id(),
                        units: Units::UserSpaceOnUse,
                        content_units: Units::UserSpaceOnUse,
                        transform: patt.transform,
                        rect,
                        view_box: patt.view_box,
                        root,
                    });
                }
            }
        }

        Some(())
    }
}

fn push_pattern_transform(root: &mut Group, transform: Transform) {
    // TODO: we should update abs_transform in all descendants as well
    let mut g = std::mem::replace(root, Group::empty());
    g.transform = transform;
    g.abs_transform = transform;

    root.children.push(Node::Group(Box::new(g)));
    root.calculate_bounding_boxes();
}

impl Paint {
    #[inline]
    pub(crate) fn units(&self) -> Units {
        match self {
            Self::Color(_) => Units::UserSpaceOnUse,
            Self::LinearGradient(lg) => lg.units,
            Self::RadialGradient(rg) => rg.units,
            Self::Pattern(patt) => patt.units,
        }
    }

    #[inline]
    pub(crate) fn content_units(&self) -> Units {
        match self {
            Self::Pattern(patt) => patt.content_units,
            _ => Units::UserSpaceOnUse,
        }
    }
}
