#![allow(missing_docs)]
use crate::taffy::LayoutId;
use crate::taffy::layout_context::LayoutContext;
use crate::text_system::{
    INLINE_BOX_PLACEHOLDER, InlineBoxMetrics, InlineFlowItem, InlineFlowLayout,
};
use crate::{
    Bounds, Edges, Pixels, Point, SharedString, Size, TextOverflow, TextStyle, WhiteSpace, point,
    px,
};
use parking_lot::Mutex;
use std::{ops::Range, sync::Arc};
use taffy::prelude::TaffyMaxContent;
use taffy::{
    LayoutPartialTree, MaybeMath, MaybeResolve, ResolveOrZero, RoundTree,
    geometry::{Rect as TaffyRect, Size as TaffySize},
    style::AvailableSpace as TaffyAvailableSpace,
    tree::{Layout, LayoutInput, LayoutOutput, NodeId, RunMode, SizingMode},
};

pub struct InlineMeasureContext {
    pub(crate) items: Vec<InlineFlowItem>,
    pub(crate) text_style: TextStyle,
    pub(crate) result: Arc<Mutex<Option<Arc<InlineFlowLayout>>>>,
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Clone)]
pub struct InlineLayoutView {
    pub(crate) layout: Arc<InlineFlowLayout>,
    pub(crate) origin: Point<Pixels>,
}

impl InlineLayoutView {
    pub fn text(&self) -> SharedString {
        self.layout.logical_text.clone()
    }

    pub fn plain_text(&self) -> SharedString {
        let text = self.layout.logical_text.as_ref();
        if !text.contains(INLINE_BOX_PLACEHOLDER) {
            return self.layout.logical_text.clone();
        }
        strip_inline_box_placeholders(text).into()
    }

    pub fn plain_text_range(&self, range: Range<usize>) -> SharedString {
        let text = self.layout.logical_text.as_ref();
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= text.len());
        debug_assert!(text.is_char_boundary(range.start));
        debug_assert!(text.is_char_boundary(range.end));

        let slice = &text[range];
        if !slice.contains(INLINE_BOX_PLACEHOLDER) {
            return slice.to_string().into();
        }
        strip_inline_box_placeholders(slice).into()
    }

    pub fn len(&self) -> usize {
        self.layout.logical_len
    }

    pub fn content_size(&self) -> Size<Pixels> {
        self.layout.content_size
    }

    pub fn bounds(&self) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin,
            size: self.layout.content_size,
        }
    }

    pub fn surrounding_word_range(&self, logical_index: usize) -> Range<usize> {
        let text = self.layout.logical_text.as_ref();
        let bytes = text.as_bytes();
        let clamp_index = logical_index.min(bytes.len());

        let mut previous_space = 0;
        for idx in (0..clamp_index).rev() {
            if bytes[idx] == b' ' || bytes[idx] == INLINE_BOX_PLACEHOLDER as u8 {
                previous_space = idx + 1;
                break;
            }
        }

        let mut next_space = bytes.len();
        for idx in clamp_index..bytes.len() {
            if bytes[idx] == b' ' || bytes[idx] == INLINE_BOX_PLACEHOLDER as u8 {
                next_space = idx;
                break;
            }
        }

        previous_space..next_space
    }
}

fn strip_inline_box_placeholders(text: &str) -> String {
    text.chars()
        .filter(|ch| *ch != INLINE_BOX_PLACEHOLDER)
        .collect()
}

// ============================================================================
// Internal Computation Logic
// ============================================================================

pub struct InlineSizing {
    pub padding: TaffyRect<f32>,
    pub border: TaffyRect<f32>,
    pub padding_border_sum: TaffySize<f32>,
    pub padding_border_minimum: TaffySize<Option<f32>>,
    pub node_size: TaffySize<Option<f32>>,
    pub node_min_size: TaffySize<Option<f32>>,
    pub node_max_size: TaffySize<Option<f32>>,
    pub aspect_ratio: Option<f32>,
    pub known_dimensions: TaffySize<Option<f32>>,
}

pub fn resolve_inline_sizing(
    style: &taffy::Style,
    parent_size: TaffySize<Option<f32>>,
    known_dimensions: TaffySize<Option<f32>>,
    sizing_mode: SizingMode,
) -> InlineSizing {
    let padding = style
        .padding
        .resolve_or_zero(parent_size.width, |_c, _b| 0.0);
    let border = style
        .border
        .resolve_or_zero(parent_size.width, |_c, _b| 0.0);
    let padding_border = padding + border;
    let padding_border_sum = TaffySize {
        width: padding_border.left + padding_border.right,
        height: padding_border.top + padding_border.bottom,
    };
    let box_sizing_adjustment = if style.box_sizing == taffy::style::BoxSizing::ContentBox {
        padding_border_sum
    } else {
        TaffySize::ZERO
    };

    let (node_size, node_min_size, node_max_size, aspect_ratio) = match sizing_mode {
        SizingMode::ContentSize => (known_dimensions, TaffySize::NONE, TaffySize::NONE, None),
        SizingMode::InherentSize => {
            let aspect_ratio = style.aspect_ratio;
            let style_size = style
                .size
                .maybe_resolve(parent_size, |_c, _b| 0.0)
                .maybe_apply_aspect_ratio(aspect_ratio)
                .maybe_add(box_sizing_adjustment);
            let style_min_size = style
                .min_size
                .maybe_resolve(parent_size, |_c, _b| 0.0)
                .maybe_apply_aspect_ratio(aspect_ratio)
                .maybe_add(box_sizing_adjustment);
            let style_max_size = style
                .max_size
                .maybe_resolve(parent_size, |_c, _b| 0.0)
                .maybe_add(box_sizing_adjustment);
            let node_size =
                known_dimensions.or(style_size.maybe_clamp(style_min_size, style_max_size));
            (node_size, style_min_size, style_max_size, aspect_ratio)
        }
    };

    let node_size = node_size.map(|opt| opt);
    let node_min_size = node_min_size.map(|opt| opt);
    let node_max_size = node_max_size.map(|opt| opt);
    let known_dimensions = known_dimensions.map(|opt| opt);

    let padding_border_minimum = padding_border_sum.map(Some);
    let known_dimensions = known_dimensions
        .or(node_size)
        .maybe_max(padding_border_minimum);

    InlineSizing {
        padding,
        border,
        padding_border_sum,
        padding_border_minimum,
        known_dimensions,
        node_size,
        node_min_size,
        node_max_size,
        aspect_ratio,
    }
}

#[inline]
fn phys_to_logical(px: f32, inv_scale: f32) -> Pixels {
    Pixels(px * inv_scale)
}

#[inline]
fn logical_to_phys(px: Pixels, scale: f32) -> f32 {
    px.0 * scale
}

#[inline]
fn taffy_size_to_logical(size: taffy::geometry::Size<f32>, inv_scale: f32) -> Size<Pixels> {
    Size {
        width: phys_to_logical(size.width, inv_scale),
        height: phys_to_logical(size.height, inv_scale),
    }
}

#[inline]
fn taffy_rect_to_edges(rect: taffy::geometry::Rect<f32>, inv_scale: f32) -> Edges<Pixels> {
    Edges {
        top: phys_to_logical(rect.top, inv_scale),
        right: phys_to_logical(rect.right, inv_scale),
        bottom: phys_to_logical(rect.bottom, inv_scale),
        left: phys_to_logical(rect.left, inv_scale),
    }
}

#[inline]
fn logical_point_to_taffy(point: Point<Pixels>, scale: f32) -> taffy::geometry::Point<f32> {
    taffy::geometry::Point {
        x: logical_to_phys(point.x, scale),
        y: logical_to_phys(point.y, scale),
    }
}

#[inline]
fn logical_size_to_taffy(size: Size<Pixels>, scale: f32) -> taffy::geometry::Size<f32> {
    taffy::geometry::Size {
        width: logical_to_phys(size.width, scale),
        height: logical_to_phys(size.height, scale),
    }
}

fn measure_inline_boxes(
    measure: &mut InlineMeasureContext,
    context: &mut LayoutContext,
    inputs: &LayoutInput,
    available_space: TaffySize<TaffyAvailableSpace>,
    inv_scale: f32,
    box_layout_ids: &mut Vec<LayoutId>,
    force: bool,
) {
    box_layout_ids.clear();

    for item in measure.items.iter_mut() {
        let InlineFlowItem::InlineBox {
            layout_id, metrics, ..
        } = item
        else {
            continue;
        };

        box_layout_ids.push(*layout_id);
        let needs_measure = force || metrics.is_none();
        if !needs_measure {
            continue;
        }

        let child_node_id: NodeId = (*layout_id).into();
        let child_style = context.get_style(child_node_id);

        let margin = child_style
            .margin
            .resolve_or_zero(inputs.parent_size.width, |_c, _b| 0.0);
        let child_available_width = match available_space.width {
            TaffyAvailableSpace::Definite(px) => TaffyAvailableSpace::Definite(px),
            _ => TaffyAvailableSpace::MaxContent,
        };
        let child_available_height = match available_space.height {
            TaffyAvailableSpace::Definite(px) => TaffyAvailableSpace::Definite(px),
            _ => TaffyAvailableSpace::MaxContent,
        };

        let child_inputs = LayoutInput {
            known_dimensions: TaffySize::NONE,
            available_space: TaffySize {
                width: child_available_width,
                height: child_available_height,
            },
            parent_size: inputs.parent_size,
            run_mode: inputs.run_mode,
            sizing_mode: inputs.sizing_mode,
            axis: inputs.axis,
            vertical_margins_are_collapsible: inputs.vertical_margins_are_collapsible,
        };

        let child_output = context.compute_child_layout(child_node_id, child_inputs);
        let size = taffy_size_to_logical(child_output.size, inv_scale);
        let mut baseline = child_output
            .first_baselines
            .y
            .map(|b| phys_to_logical(b, inv_scale))
            .unwrap_or(size.height);
        baseline = px(baseline.0.clamp(0.0, size.height.0));

        let metrics_value = InlineBoxMetrics {
            width: size.width,
            height: size.height,
            margin: taffy_rect_to_edges(margin, inv_scale),
            baseline,
        };
        *metrics = Some(metrics_value.clone());
    }
}

pub(crate) fn compute_inline_layout_impl(
    measure: &mut InlineMeasureContext,
    style: &taffy::Style,
    inputs: LayoutInput,
    context: &mut LayoutContext,
) -> (LayoutOutput, Option<Arc<InlineFlowLayout>>) {
    let scale_factor = context.get_scale_factor();
    let inv_scale = 1.0 / scale_factor;
    let text_system = context.text_system();

    let sizing = resolve_inline_sizing(
        &style,
        inputs.parent_size,
        inputs.known_dimensions,
        inputs.sizing_mode,
    );

    if inputs.run_mode == RunMode::ComputeSize {
        if let TaffySize {
            width: Some(width),
            height: Some(height),
        } = sizing.known_dimensions
        {
            return (
                LayoutOutput::from_outer_size(TaffySize { width, height }),
                None,
            );
        }
    }

    let known_dimensions = inputs
        .known_dimensions
        .map(|opt| opt.map(|v| v * inv_scale));
    let padding = sizing.padding.map(|v| v * inv_scale);
    let border = sizing.border.map(|v| v * inv_scale);
    let padding_border_sum = sizing.padding_border_sum.map(|v| v * inv_scale);
    let padding_border_minimum = sizing
        .padding_border_minimum
        .map(|opt| opt.map(|v| v * inv_scale));
    let node_min_size = sizing.node_min_size.map(|opt| opt.map(|v| v * inv_scale));
    let node_max_size = sizing.node_max_size.map(|opt| opt.map(|v| v * inv_scale));
    let node_size = sizing.node_size.map(|opt| opt.map(|v| v * inv_scale));
    let _aspect_ratio = sizing.aspect_ratio;

    let padding_border_offset = point(px(padding.left + border.left), px(padding.top + border.top));

    let rem_size = context.get_rem_size();
    let font_size = measure.text_style.font_size.to_pixels(rem_size);
    let line_height = measure
        .text_style
        .line_height
        .to_pixels(measure.text_style.font_size, rem_size);

    let mut box_layout_ids = Vec::new();

    if inputs.run_mode == RunMode::ComputeSize && sizing.known_dimensions.width.is_none() {
        let is_nowrap = measure.text_style.white_space == WhiteSpace::Nowrap;
        let is_min_content = inputs.available_space.width == TaffyAvailableSpace::MinContent;
        let is_max_content = inputs.available_space.width == TaffyAvailableSpace::MaxContent;

        if is_nowrap || is_min_content || is_max_content {
            let needs_measure = measure
                .items
                .iter()
                .any(|item| matches!(item, InlineFlowItem::InlineBox { metrics: None, .. }));
            if needs_measure {
                let intrinsic_available = TaffySize::MAX_CONTENT;
                measure_inline_boxes(
                    measure,
                    context,
                    &inputs,
                    intrinsic_available,
                    inv_scale,
                    &mut box_layout_ids,
                    false,
                );
            }

            let layout = text_system.shape_inline_flow(
                &measure.items,
                font_size,
                line_height,
                None,
                None,
                measure.text_style.white_space,
                None,
                None,
            );

            let intrinsic_width = if is_min_content {
                layout.intrinsic_min_width
            } else {
                layout.intrinsic_max_width
            };

            let outer_width = intrinsic_width.0 + padding_border_sum.width;
            let outer_height = line_height.0 + padding_border_sum.height;

            return (
                LayoutOutput::from_outer_size(TaffySize {
                    width: outer_width * scale_factor,
                    height: outer_height * scale_factor,
                }),
                None,
            );
        }
    }

    let content_available = TaffySize {
        width: match (inputs.known_dimensions.width, inputs.available_space.width) {
            (Some(known_width), _) => (known_width * inv_scale - padding_border_sum.width).max(0.0),
            (_, TaffyAvailableSpace::Definite(px)) => {
                (px * inv_scale - padding_border_sum.width).max(0.0)
            }
            (_, TaffyAvailableSpace::MinContent | TaffyAvailableSpace::MaxContent) => f32::INFINITY,
        },
        height: match (
            inputs.known_dimensions.height,
            inputs.available_space.height,
        ) {
            (Some(known_height), _) => {
                (known_height * inv_scale - padding_border_sum.height).max(0.0)
            }
            (_, TaffyAvailableSpace::Definite(px)) => {
                (px * inv_scale - padding_border_sum.height).max(0.0)
            }
            (_, TaffyAvailableSpace::MinContent | TaffyAvailableSpace::MaxContent) => f32::INFINITY,
        },
    };

    let content_available_space = context.to_taffy_available_space(content_available);

    let force_measure = inputs.run_mode != RunMode::ComputeSize;
    measure_inline_boxes(
        measure,
        context,
        &inputs,
        content_available_space,
        inv_scale,
        &mut box_layout_ids,
        force_measure,
    );

    let wrap_width = if measure.text_style.white_space == WhiteSpace::Nowrap {
        None
    } else if content_available.width.is_finite() {
        Some(px(content_available.width))
    } else {
        None
    };

    let text_overflow = match &measure.text_style.text_overflow {
        Some(TextOverflow::Truncate(suffix)) => Some(suffix.clone()),
        _ => None,
    };

    let truncate_width = if text_overflow.is_some() && content_available.width.is_finite() {
        Some(px(content_available.width))
    } else {
        None
    };

    let layout = text_system.shape_inline_flow(
        &measure.items,
        font_size,
        line_height,
        wrap_width,
        truncate_width,
        measure.text_style.white_space,
        measure.text_style.line_clamp,
        text_overflow,
    );

    for placement in layout.boxes.iter() {
        let Some(layout_id) = box_layout_ids.get(placement.index) else {
            continue;
        };
        let child_id: NodeId = (*layout_id).into();
        let origin = Point {
            x: placement.relative_bounds.origin.x + padding_border_offset.x,
            y: placement.relative_bounds.origin.y + padding_border_offset.y,
        };
        let child_layout = Layout {
            order: 0,
            size: logical_size_to_taffy(placement.relative_bounds.size, scale_factor),
            location: logical_point_to_taffy(origin, scale_factor),
            ..Default::default()
        };
        context.set_unrounded_layout(child_id, &child_layout);
        context.set_final_layout(child_id, &child_layout);
    }

    let measured_size = TaffySize {
        width: layout.content_size.width.0 + padding_border_sum.width,
        height: layout.content_size.height.0 + padding_border_sum.height,
    };
    let mut final_size = known_dimensions.or(node_size).unwrap_or(measured_size);
    final_size = final_size.maybe_clamp(node_min_size, node_max_size);
    final_size.width = final_size
        .width
        .max(measured_size.width)
        .max(padding_border_sum.width);
    final_size.height = final_size
        .height
        .max(measured_size.height)
        .max(padding_border_sum.height);
    let final_size = final_size.maybe_max(padding_border_minimum);

    (
        LayoutOutput::from_outer_size(TaffySize {
            width: final_size.width * scale_factor,
            height: final_size.height * scale_factor,
        }),
        Some(layout),
    )
}
