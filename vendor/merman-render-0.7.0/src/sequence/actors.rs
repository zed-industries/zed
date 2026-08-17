use super::constants::{
    SEQUENCE_WRAPPED_MESSAGE_WIDTH_EPS_PX, sequence_actor_lifeline_start_y,
    sequence_actor_visual_height, sequence_text_dimensions_height_px,
};
use super::metrics::{SequenceMathHeightMode, measure_sequence_label_for_layout};
use crate::math::MathRenderer;
use crate::model::{LayoutEdge, LayoutNode, LayoutPoint};
use crate::text::{TextMeasurer, TextStyle, split_html_br_lines, wrap_label_like_mermaid_lines};
use crate::{Error, Result};
use merman_core::MermaidConfig;
use merman_core::diagrams::sequence::SequenceActor;
use merman_core::diagrams::sequence::SequenceDiagramRenderModel;
use std::collections::{BTreeMap, HashMap};

use super::metrics::measure_svg_like_with_html_br;

pub(super) struct SequenceActorLayoutPlanContext<'a> {
    pub(super) model: &'a SequenceDiagramRenderModel,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) actor_text_style: &'a TextStyle,
    pub(super) note_text_style: &'a TextStyle,
    pub(super) msg_text_style: &'a TextStyle,
    pub(super) math_config: &'a MermaidConfig,
    pub(super) math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
    pub(super) actor_width_min: f64,
    pub(super) actor_height: f64,
    pub(super) actor_margin: f64,
    pub(super) actor_font_size: f64,
    pub(super) box_margin: f64,
    pub(super) box_text_margin: f64,
    pub(super) wrap_padding: f64,
    pub(super) message_width_scale: f64,
    pub(super) message_font_size: f64,
}

pub(super) struct SequenceActorLayoutPlan<'a> {
    pub(super) actor_index: HashMap<&'a str, usize>,
    pub(super) actor_widths: Vec<f64>,
    pub(super) actor_base_heights: Vec<f64>,
    pub(super) actor_box: Vec<Option<usize>>,
    pub(super) actor_left_x: Vec<f64>,
    pub(super) actor_centers_x: Vec<f64>,
    pub(super) box_margins: Vec<f64>,
    pub(super) actor_top_offset_y: f64,
    pub(super) max_actor_layout_height: f64,
    pub(super) has_boxes: bool,
}

pub(super) struct SequenceActorLifecycleContext<'a> {
    pub(super) actor_index: &'a HashMap<&'a str, usize>,
    pub(super) actor_base_heights: &'a [f64],
    pub(super) created_actors: &'a BTreeMap<String, usize>,
    pub(super) destroyed_actors: &'a BTreeMap<String, usize>,
    pub(super) actor_height: f64,
}

pub(super) fn plan_sequence_actors<'a>(
    ctx: SequenceActorLayoutPlanContext<'a>,
) -> Result<SequenceActorLayoutPlan<'a>> {
    let has_boxes = !ctx.model.boxes.is_empty();
    let has_box_titles = ctx
        .model
        .boxes
        .iter()
        .any(|b| b.name.as_deref().is_some_and(|s| !s.trim().is_empty()));

    if ctx.model.actor_order.is_empty() {
        return Err(Error::InvalidModel {
            message: "sequence model has no actorOrder".to_string(),
        });
    }

    let max_box_title_height = max_box_title_height(&ctx, has_box_titles);
    let (actor_widths, actor_base_heights) = measure_actor_boxes(&ctx)?;
    let actor_index = actor_index(ctx.model);
    let actor_to_message_width = actor_message_widths(&ctx, &actor_index);
    let actor_margins = actor_margins(&actor_widths, &actor_to_message_width, ctx.actor_margin);
    let box_margins = box_margins(
        &ctx,
        &actor_index,
        &actor_widths,
        &actor_margins,
        &actor_to_message_width,
    );
    let actor_top_offset_y =
        actor_top_offset_y(&ctx, has_boxes, has_box_titles, max_box_title_height);
    let actor_box = actor_box(ctx.model, &actor_index);
    let actor_left_x = actor_left_x(
        &ctx,
        &actor_widths,
        &actor_margins,
        &actor_box,
        &box_margins,
    );
    let actor_centers_x = actor_centers_x(&actor_left_x, &actor_widths);
    let max_actor_layout_height = actor_base_heights
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        .max(1.0);

    Ok(SequenceActorLayoutPlan {
        actor_index,
        actor_widths,
        actor_base_heights,
        actor_box,
        actor_left_x,
        actor_centers_x,
        box_margins,
        actor_top_offset_y,
        max_actor_layout_height,
        has_boxes,
    })
}

fn max_box_title_height(ctx: &SequenceActorLayoutPlanContext<'_>, has_box_titles: bool) -> f64 {
    if !has_box_titles {
        return 0.0;
    }

    // Mermaid uses `utils.calculateTextDimensions(...).height` for box titles and stores the max
    // across boxes in `box.textMaxHeight` (used for bumping actor `starty` when any title exists).
    //
    // In Mermaid 11.12.2 with 16px fonts, this height comes out as 17px (not the larger SVG
    // `getBBox()` height used elsewhere). Keep this model-level constant to match upstream DOM.
    let line_h = sequence_text_dimensions_height_px(ctx.message_font_size);
    ctx.model
        .boxes
        .iter()
        .filter_map(|b| b.name.as_deref())
        .map(|s| split_html_br_lines(s).len().max(1) as f64 * line_h)
        .fold(0.0, f64::max)
}

fn measure_actor_boxes(ctx: &SequenceActorLayoutPlanContext<'_>) -> Result<(Vec<f64>, Vec<f64>)> {
    // Measure participant boxes.
    let mut actor_widths: Vec<f64> = Vec::with_capacity(ctx.model.actor_order.len());
    let mut actor_base_heights: Vec<f64> = Vec::with_capacity(ctx.model.actor_order.len());
    for id in &ctx.model.actor_order {
        let a = ctx
            .model
            .actors
            .get(id)
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing actor {id}"),
            })?;
        if a.wrap {
            // Upstream wraps actor descriptions to `conf.width - 2*wrapPadding` and clamps the
            // actor box width to `conf.width`.
            let wrap_w = (ctx.actor_width_min - 2.0 * ctx.wrap_padding).max(1.0);
            let wrapped_lines = wrap_label_like_mermaid_lines(
                &a.description,
                ctx.measurer,
                ctx.actor_text_style,
                wrap_w,
            );
            let line_count = wrapped_lines.len().max(1) as f64;
            let text_h = sequence_text_dimensions_height_px(ctx.actor_font_size) * line_count;
            actor_base_heights.push(ctx.actor_height.max(text_h).max(1.0));
            actor_widths.push(ctx.actor_width_min.max(1.0));
        } else {
            let (w0, _h0) = measure_sequence_label_for_layout(
                ctx.measurer,
                &a.description,
                ctx.actor_text_style,
                ctx.math_config,
                ctx.math_renderer,
                SequenceMathHeightMode::Actor,
            );
            let w = (w0 + 2.0 * ctx.wrap_padding).max(ctx.actor_width_min);
            actor_base_heights.push(ctx.actor_height.max(1.0));
            actor_widths.push(w.max(1.0));
        }
    }
    Ok((actor_widths, actor_base_heights))
}

fn actor_index(model: &SequenceDiagramRenderModel) -> HashMap<&str, usize> {
    let mut actor_index: HashMap<&str, usize> = HashMap::new();
    for (i, id) in model.actor_order.iter().enumerate() {
        actor_index.insert(id.as_str(), i);
    }
    actor_index
}

fn actor_message_widths(
    ctx: &SequenceActorLayoutPlanContext<'_>,
    actor_index: &HashMap<&str, usize>,
) -> Vec<f64> {
    let mut actor_to_message_width: Vec<f64> = vec![0.0; ctx.model.actor_order.len()];
    for msg in &ctx.model.messages {
        let (Some(from), Some(to)) = (msg.from.as_deref(), msg.to.as_deref()) else {
            continue;
        };
        let Some(&from_idx) = actor_index.get(from) else {
            continue;
        };
        let Some(&to_idx) = actor_index.get(to) else {
            continue;
        };

        let placement = msg.placement;
        // If this is the first actor, and the note is left of it, no need to calculate the margin.
        if placement == Some(0) && to_idx == 0 {
            continue;
        }
        // If this is the last actor, and the note is right of it, no need to calculate the margin.
        if placement == Some(1) && to_idx + 1 == ctx.model.actor_order.len() {
            continue;
        }

        let is_note = placement.is_some();
        let is_message = !is_note;
        let style = if is_note {
            ctx.note_text_style
        } else {
            ctx.msg_text_style
        };
        let text = msg.message_text();
        if text.is_empty() {
            continue;
        }

        let (w0, _h0) = if text.contains("$$") {
            measure_sequence_label_for_layout(
                ctx.measurer,
                text,
                style,
                ctx.math_config,
                ctx.math_renderer,
                SequenceMathHeightMode::Bound,
            )
        } else {
            let measured_text = if msg.wrap {
                // Upstream uses `wrapLabel(message, conf.width - 2*wrapPadding, ...)` when
                // computing max per-actor message widths for spacing.
                let wrap_w = (ctx.actor_width_min - 2.0 * ctx.wrap_padding).max(1.0);
                let lines = wrap_label_like_mermaid_lines(text, ctx.measurer, style, wrap_w);
                lines.join("<br>")
            } else {
                text.to_string()
            };
            measure_svg_like_with_html_br(ctx.measurer, &measured_text, style)
        };
        let w0 = w0 * ctx.message_width_scale;
        let mut message_w = (w0 + 2.0 * ctx.wrap_padding).max(0.0);
        if msg.wrap
            && message_w > ctx.actor_width_min
            && message_w <= ctx.actor_width_min + SEQUENCE_WRAPPED_MESSAGE_WIDTH_EPS_PX
        {
            message_w = ctx.actor_width_min;
        }

        let prev_idx = if to_idx > 0 { Some(to_idx - 1) } else { None };
        let next_idx = if to_idx + 1 < ctx.model.actor_order.len() {
            Some(to_idx + 1)
        } else {
            None
        };

        if is_message && next_idx.is_some_and(|n| n == from_idx) {
            actor_to_message_width[to_idx] = actor_to_message_width[to_idx].max(message_w);
        } else if is_message && prev_idx.is_some_and(|p| p == from_idx) {
            actor_to_message_width[from_idx] = actor_to_message_width[from_idx].max(message_w);
        } else if is_message && from_idx == to_idx {
            let half = message_w / 2.0;
            actor_to_message_width[from_idx] = actor_to_message_width[from_idx].max(half);
            actor_to_message_width[to_idx] = actor_to_message_width[to_idx].max(half);
        } else if placement == Some(1) {
            // RIGHTOF
            actor_to_message_width[from_idx] = actor_to_message_width[from_idx].max(message_w);
        } else if placement == Some(0) {
            // LEFTOF
            if let Some(p) = prev_idx {
                actor_to_message_width[p] = actor_to_message_width[p].max(message_w);
            }
        } else if placement == Some(2) {
            // OVER
            if let Some(p) = prev_idx {
                actor_to_message_width[p] = actor_to_message_width[p].max(message_w / 2.0);
            }
            if next_idx.is_some() {
                actor_to_message_width[from_idx] =
                    actor_to_message_width[from_idx].max(message_w / 2.0);
            }
        }
    }
    actor_to_message_width
}

fn actor_margins(
    actor_widths: &[f64],
    actor_to_message_width: &[f64],
    actor_margin: f64,
) -> Vec<f64> {
    let mut actor_margins: Vec<f64> = vec![actor_margin; actor_to_message_width.len()];
    for i in 0..actor_to_message_width.len() {
        let msg_w = actor_to_message_width[i];
        if msg_w <= 0.0 {
            continue;
        }
        let w0 = actor_widths[i];
        let actor_w = if i + 1 < actor_to_message_width.len() {
            let w1 = actor_widths[i + 1];
            msg_w + actor_margin - (w0 / 2.0) - (w1 / 2.0)
        } else {
            msg_w + actor_margin - (w0 / 2.0)
        };
        actor_margins[i] = actor_w.max(actor_margin);
    }
    actor_margins
}

fn box_margins(
    ctx: &SequenceActorLayoutPlanContext<'_>,
    actor_index: &HashMap<&str, usize>,
    actor_widths: &[f64],
    actor_margins: &[f64],
    actor_to_message_width: &[f64],
) -> Vec<f64> {
    // Mermaid's `calculateActorMargins(...)` computes per-box `box.margin` based on total actor
    // widths/margins and the box title width. For totalWidth, Mermaid only counts `actor.margin`
    // if it was set (actors without messages have `margin === undefined` until render-time).
    let mut box_margins: Vec<f64> = vec![ctx.box_text_margin; ctx.model.boxes.len()];
    for (box_idx, b) in ctx.model.boxes.iter().enumerate() {
        let mut total_width = 0.0;
        for actor_key in &b.actor_keys {
            let Some(&i) = actor_index.get(actor_key.as_str()) else {
                continue;
            };
            let actor_margin_for_box = if actor_to_message_width[i] > 0.0 {
                actor_margins[i]
            } else {
                0.0
            };
            total_width += actor_widths[i] + actor_margin_for_box;
        }

        total_width += ctx.box_margin * 8.0;
        total_width -= 2.0 * ctx.box_text_margin;

        let Some(name) = b.name.as_deref().filter(|s| !s.trim().is_empty()) else {
            continue;
        };

        let (text_w, _text_h) = measure_sequence_label_for_layout(
            ctx.measurer,
            name,
            ctx.msg_text_style,
            ctx.math_config,
            ctx.math_renderer,
            SequenceMathHeightMode::Bound,
        );
        let min_width = total_width.max(text_w + 2.0 * ctx.wrap_padding);
        if total_width < min_width {
            box_margins[box_idx] += (min_width - total_width) / 2.0;
        }
    }
    box_margins
}

fn actor_top_offset_y(
    ctx: &SequenceActorLayoutPlanContext<'_>,
    has_boxes: bool,
    has_box_titles: bool,
    max_box_title_height: f64,
) -> f64 {
    // Actors start lower when boxes exist, to make room for box headers.
    let mut actor_top_offset_y = 0.0;
    if has_boxes {
        actor_top_offset_y += ctx.box_margin;
        if has_box_titles {
            actor_top_offset_y += max_box_title_height;
        }
    }
    actor_top_offset_y
}

fn actor_box(
    model: &SequenceDiagramRenderModel,
    actor_index: &HashMap<&str, usize>,
) -> Vec<Option<usize>> {
    // Assign each actor to at most one box (Mermaid's db assigns a single `actor.box` reference).
    let mut actor_box: Vec<Option<usize>> = vec![None; model.actor_order.len()];
    for (box_idx, b) in model.boxes.iter().enumerate() {
        for actor_key in &b.actor_keys {
            let Some(&i) = actor_index.get(actor_key.as_str()) else {
                continue;
            };
            actor_box[i] = Some(box_idx);
        }
    }
    actor_box
}

fn actor_left_x(
    ctx: &SequenceActorLayoutPlanContext<'_>,
    actor_widths: &[f64],
    actor_margins: &[f64],
    actor_box: &[Option<usize>],
    box_margins: &[f64],
) -> Vec<f64> {
    let mut actor_left_x: Vec<f64> = Vec::with_capacity(ctx.model.actor_order.len());
    let mut prev_width = 0.0;
    let mut prev_margin = 0.0;
    let mut prev_box: Option<usize> = None;
    for i in 0..ctx.model.actor_order.len() {
        let w = actor_widths[i];
        let cur_box = actor_box[i];

        // end of box
        if prev_box.is_some() && prev_box != cur_box {
            if let Some(prev) = prev_box {
                prev_margin += ctx.box_margin + box_margins[prev];
            }
        }

        // new box
        if cur_box.is_some() && cur_box != prev_box {
            if let Some(bi) = cur_box {
                prev_margin += box_margins[bi];
            }
        }

        // Mermaid widens the margin before a created actor by `actor.width / 2`.
        if ctx
            .model
            .created_actors
            .contains_key(&ctx.model.actor_order[i])
        {
            prev_margin += w / 2.0;
        }
        let x = prev_width + prev_margin;
        actor_left_x.push(x);
        prev_width += w + prev_margin;
        prev_margin = actor_margins[i];
        prev_box = cur_box;
    }
    actor_left_x
}

fn actor_centers_x(actor_left_x: &[f64], actor_widths: &[f64]) -> Vec<f64> {
    let mut actor_centers_x: Vec<f64> = Vec::with_capacity(actor_left_x.len());
    for i in 0..actor_left_x.len() {
        actor_centers_x.push(actor_left_x[i] + actor_widths[i] / 2.0);
    }
    actor_centers_x
}

pub(super) struct SequenceActorLifecycle<'a> {
    ctx: SequenceActorLifecycleContext<'a>,
    created_top_center_y: BTreeMap<String, f64>,
    destroyed_bottom_top_y: BTreeMap<String, f64>,
}

pub(super) struct SequenceFooterActorContext<'a, 'b> {
    pub(super) actor_order: &'a [String],
    pub(super) actors: &'a BTreeMap<String, SequenceActor>,
    pub(super) actor_widths: &'a [f64],
    pub(super) actor_centers_x: &'a [f64],
    pub(super) actor_base_heights: &'a [f64],
    pub(super) actor_lifecycle: &'b SequenceActorLifecycle<'a>,
    pub(super) actor_top_offset_y: f64,
    pub(super) bottom_box_top_y: f64,
    pub(super) mirror_actors: bool,
    pub(super) label_box_height: f64,
    pub(super) box_text_margin: f64,
}

pub(super) struct SequenceTopActorContext<'a> {
    pub(super) actor_order: &'a [String],
    pub(super) actors: &'a BTreeMap<String, SequenceActor>,
    pub(super) actor_widths: &'a [f64],
    pub(super) actor_centers_x: &'a [f64],
    pub(super) actor_base_heights: &'a [f64],
    pub(super) actor_top_offset_y: f64,
    pub(super) label_box_height: f64,
}

impl<'a> SequenceActorLifecycle<'a> {
    pub(super) fn new(ctx: SequenceActorLifecycleContext<'a>) -> Self {
        Self {
            ctx,
            created_top_center_y: BTreeMap::new(),
            destroyed_bottom_top_y: BTreeMap::new(),
        }
    }

    pub(super) fn created_actor_index(&self, actor_id: &str) -> Option<usize> {
        self.ctx.created_actors.get(actor_id).copied()
    }

    pub(super) fn destroyed_actor_index(&self, actor_id: &str) -> Option<usize> {
        self.ctx.destroyed_actors.get(actor_id).copied()
    }

    pub(super) fn created_top_center_y(&self, actor_id: &str) -> Option<f64> {
        self.created_top_center_y.get(actor_id).copied()
    }

    pub(super) fn destroyed_bottom_top_y(&self, actor_id: &str) -> Option<f64> {
        self.destroyed_bottom_top_y.get(actor_id).copied()
    }

    pub(super) fn apply_message_y_adjustment(
        &mut self,
        msg_idx: usize,
        from: &str,
        to: &str,
        line_y: f64,
    ) -> f64 {
        // Mermaid updates created/destroyed actor vertical anchors while processing messages and
        // advances the cursor by half of the actor's pre-render layout height. Type-specific SVG
        // glyph drawing may later mutate the visual height, but that does not feed back into this
        // lifecycle cursor adjustment.
        if self.created_actor_index(to) == Some(msg_idx) {
            let h = self.actor_lifecycle_height(to);
            self.created_top_center_y.insert(to.to_string(), line_y);
            h / 2.0
        } else if self.destroyed_actor_index(from) == Some(msg_idx) {
            let h = self.actor_lifecycle_height(from);
            self.destroyed_bottom_top_y
                .insert(from.to_string(), line_y - h / 2.0);
            h / 2.0
        } else if self.destroyed_actor_index(to) == Some(msg_idx) {
            let h = self.actor_lifecycle_height(to);
            self.destroyed_bottom_top_y
                .insert(to.to_string(), line_y - h / 2.0);
            h / 2.0
        } else {
            0.0
        }
    }

    pub(super) fn apply_created_top_actor_positions(&self, nodes: &mut [LayoutNode]) {
        // Created actors render from `lineStartY - actor.height / 2` in Mermaid's
        // `adjustCreatedDestroyedData(...)`. Type-specific drawing can use a taller visual node,
        // but that visual height does not move the creation anchor.
        for node in nodes {
            let Some(actor_id) = node.id.strip_prefix("actor-top-") else {
                continue;
            };
            if let Some(y) = self.created_top_center_y(actor_id) {
                let h = self.actor_lifecycle_height(actor_id);
                node.y = y - h / 2.0 + node.height / 2.0;
            }
        }
    }

    fn actor_lifecycle_height(&self, actor_id: &str) -> f64 {
        let Some(idx) = self.ctx.actor_index.get(actor_id).copied() else {
            return self.ctx.actor_height.max(1.0);
        };
        self.ctx
            .actor_base_heights
            .get(idx)
            .copied()
            .unwrap_or(self.ctx.actor_height)
            .max(1.0)
    }
}

pub(super) fn sequence_actor_is_type_width_limited(
    actors: &BTreeMap<String, SequenceActor>,
    actor_id: &str,
) -> bool {
    actors
        .get(actor_id)
        .map(|a| {
            matches!(
                a.actor_type.as_str(),
                "actor" | "control" | "entity" | "database"
            )
        })
        .unwrap_or(false)
}

pub(super) fn append_sequence_top_actors(
    nodes: &mut Vec<LayoutNode>,
    ctx: SequenceTopActorContext<'_>,
) {
    for (idx, id) in ctx.actor_order.iter().enumerate() {
        let w = ctx.actor_widths[idx];
        let cx = ctx.actor_centers_x[idx];
        let base_h = ctx.actor_base_heights[idx];
        let actor_type = ctx
            .actors
            .get(id)
            .map(|a| a.actor_type.as_str())
            .unwrap_or("participant");
        let visual_h = sequence_actor_visual_height(actor_type, w, base_h, ctx.label_box_height);
        let top_y = ctx.actor_top_offset_y + visual_h / 2.0;
        nodes.push(LayoutNode {
            id: format!("actor-top-{id}"),
            x: cx,
            y: top_y,
            width: w,
            height: visual_h,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });
    }
}

pub(super) fn append_sequence_footer_actors(
    nodes: &mut Vec<LayoutNode>,
    edges: &mut Vec<LayoutEdge>,
    ctx: SequenceFooterActorContext<'_, '_>,
) {
    for (idx, id) in ctx.actor_order.iter().enumerate() {
        let w = ctx.actor_widths[idx];
        let cx = ctx.actor_centers_x[idx];
        let base_h = ctx.actor_base_heights[idx];
        let actor_type = ctx
            .actors
            .get(id)
            .map(|a| a.actor_type.as_str())
            .unwrap_or("participant");
        let visual_h = sequence_actor_visual_height(actor_type, w, base_h, ctx.label_box_height);
        let bottom_top_y = ctx
            .actor_lifecycle
            .destroyed_bottom_top_y(id)
            .unwrap_or(ctx.bottom_box_top_y);
        let bottom_visual_h = if ctx.mirror_actors { visual_h } else { 0.0 };
        nodes.push(LayoutNode {
            id: format!("actor-bottom-{id}"),
            x: cx,
            y: bottom_top_y + bottom_visual_h / 2.0,
            width: w,
            height: bottom_visual_h,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });

        let top_center_y = ctx
            .actor_lifecycle
            .created_top_center_y(id)
            .unwrap_or(ctx.actor_top_offset_y + visual_h / 2.0);
        let top_left_y = top_center_y - visual_h / 2.0;
        let lifeline_start_y =
            top_left_y + sequence_actor_lifeline_start_y(actor_type, base_h, ctx.box_text_margin);

        edges.push(LayoutEdge {
            id: format!("lifeline-{id}"),
            from: format!("actor-top-{id}"),
            to: format!("actor-bottom-{id}"),
            from_cluster: None,
            to_cluster: None,
            points: vec![
                LayoutPoint {
                    x: cx,
                    y: lifeline_start_y,
                },
                LayoutPoint {
                    x: cx,
                    y: bottom_top_y,
                },
            ],
            label: None,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: None,
            end_marker: None,
            stroke_dasharray: None,
        });
    }
}
